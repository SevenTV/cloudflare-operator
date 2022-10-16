use anyhow::{anyhow, Result};
use async_trait::async_trait;
use capnp::message::TypedBuilder;
use log::{error, info};
use quinn::{RecvStream, SendStream};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    select,
};

use utils::context::wait::{Context, Handle as ContextHandle};

use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;
use uuid::Uuid;

use crate::incoming::{
    cloudflare_tunnels::types::{self, HandleHttp, TunnelAuth},
    types::HttpMethod,
};

use super::super::rpc::{new_network, ControlStreamManager};

use generated::capnp::{
    quic_metadata_protocol::primitives,
    quic_metadata_protocol::structs::{ConnectRequest, ConnectResponse, Metadata},
    tunnelrpc::{
        interfaces::registration_server::{RegisterConnectionParams, UnregisterConnectionParams},
        structs,
    },
};

pub fn serve_control_stream(
    ctx: Context,
    id: Uuid,
    idx: u8,
    auth: TunnelAuth,
    send: SendStream,
    recv: RecvStream,
) -> (JoinHandle<Result<()>>, Context) {
    // We have to run the control stream on a single thread because capnp-rpc-rust was poorly designed.
    // This is a workaround for that.
    // We create a local pool handle with size 1 and run the control stream on that.
    // When this handle is dropped, the control stream will be dropped as well.
    let mut handle = ContextHandle::new();
    let local_ctx = handle.spawn();

    let fut = LocalPoolHandle::new(1).spawn_pinned(move || async move {
        serve_control_stream_single_thread(ServeControlStreamSingleThread {
            ctx,
            id,
            idx,
            handle,
            auth,
            send,
            recv,
        })
        .await
    });

    (fut, local_ctx)
}

struct ServeControlStreamSingleThread {
    ctx: Context,
    id: Uuid,
    idx: u8,
    handle: ContextHandle,
    auth: TunnelAuth,
    send: SendStream,
    recv: RecvStream,
}

async fn serve_control_stream_single_thread(args: ServeControlStreamSingleThread) -> Result<()> {
    let ServeControlStreamSingleThread {
        mut ctx,
        mut handle,
        auth,
        idx,
        send,
        recv,
        id,
    } = args;

    let (control_stream, system) = ControlStreamManager::new(new_network(send, recv));

    let mut system_fut = {
        let mut ctx = handle.spawn();

        tokio::task::spawn_local(async move {
            select! {
                _ = system => {}
                _ = ctx.done() => {}
            }

            Ok::<(), anyhow::Error>(())
        })
    };

    let tunnel_client = control_stream.get_tunnel_client();

    {
        info!("Registering tunnel with edge");

        // TODO: we need to set real values here this was just for testing...
        let resp = tunnel_client
            .get_registration_client()
            .register_connection(RegisterConnectionParams {
                auth: structs::TunnelAuth {
                    account_tag: auth.account_tag.clone(),
                    tunnel_secret: auth.tunnel_secret_decode()?,
                },
                tunnel_id: auth.tunnel_id.into_bytes().to_vec(),
                conn_index: idx,
                options: structs::ConnectionOptions {
                    client: structs::ClientInfo {
                        client_id: id.into_bytes().to_vec(),
                        version: "0.0.1".to_string(),
                        arch: std::env::consts::ARCH.to_string(),
                        features: vec![],
                    },
                    origin_local_ip: Vec::new(),
                    replace_existing: true,
                    compression_quality: 0,
                    num_previous_attempts: 0,
                },
            })
            .await
            .map_err(|e| anyhow!("failed to register connection: {:#}", e))?;

        info!("Registered connection: {:?}", resp);
    }

    select! {
        _ = &mut system_fut => {
            Err(anyhow!("rpc system handler exited prematurely"))
        },
        _ = ctx.done() => {
            // This will always return an erorr because we dont get a response as cloudflare seems to rudely close the connection on us.
            // Therefore we can just ignore the response from here.
            let _ = tunnel_client.get_registration_client().unregister_connection(UnregisterConnectionParams{}).await;

            // We now need to clean up this thread.
            handle.cancel().await;

            Ok::<(), anyhow::Error>(())
        }
    }
}

const DATA_STREAM_SIGNATURE: [u8; 6] = [0x0A, 0x36, 0xCD, 0x12, 0xA1, 0x3E];
const RPC_STREAM_SIGNATURE: [u8; 6] = [0x52, 0xBB, 0x82, 0x5C, 0xDB, 0x65];

enum Protocol {
    RPCStream,
    DataStream,
}

async fn determine_protocol(recv: &mut RecvStream) -> Result<Protocol> {
    let mut buf = [0u8; 6];

    recv.read_exact(&mut buf).await?;

    if buf == DATA_STREAM_SIGNATURE {
        Ok(Protocol::DataStream)
    } else if buf == RPC_STREAM_SIGNATURE {
        Ok(Protocol::RPCStream)
    } else {
        Err(anyhow!("invalid protocol signature"))
    }
}

pub async fn serve_stream(ctx: Context, stream: (SendStream, RecvStream), handle: HandleHttp) {
    let (send, mut recv) = stream;

    let result: Result<()> = async move {
        match determine_protocol(&mut recv).await? {
            Protocol::RPCStream => Err(anyhow!("unimplemented")),
            Protocol::DataStream => {
                let mut buf = [0u8; 2];
                recv.read_exact(&mut buf).await?;

                let request = ConnectRequest::from_primitive(
                    capnp_futures::serialize::read_message(
                        &mut recv,
                        capnp::message::ReaderOptions::new(),
                    )
                    .await?
                    .get_root()?,
                )?;

                // https://example.com/abc
                // "https:" "" "example.com" "abc"
                let path = request.dest.splitn(4, '/').nth(3).unwrap();
                let mut req = types::HttpRequest {
                    method: HttpMethod::GET,
                    path: format!("/{}", path),
                    headers: Vec::new(),
                    is_websocket: request.connection_type == primitives::ConnectionType::Websocket,
                };

                for metadata in request.metadata {
                    if metadata.key.starts_with("HttpHeader:") {
                        let key = metadata.key.trim_start_matches("HttpHeader:");
                        let value = metadata.val;

                        req.headers.push((key.to_string(), value));
                    } else if metadata.key == "HttpHost" {
                        req.headers.push(("Host".to_string(), metadata.val));
                    } else if metadata.key == "HttpMethod" {
                        req.method = match metadata.val.as_str() {
                            "GET" => HttpMethod::GET,
                            "POST" => HttpMethod::POST,
                            "PUT" => HttpMethod::PUT,
                            "DELETE" => HttpMethod::DELETE,
                            "HEAD" => HttpMethod::HEAD,
                            "OPTIONS" => HttpMethod::OPTIONS,
                            "PATCH" => HttpMethod::PATCH,
                            "TRACE" => HttpMethod::TRACE,
                            "CONNECT" => HttpMethod::CONNECT,
                            _ => return Err(anyhow!("invalid http method")),
                        }
                    }
                }

                handle
                    .handle(
                        ctx,
                        req,
                        Box::new(HttpStream {
                            send,
                            recv,
                            version: buf,
                        }),
                    )
                    .await
            }
        }
    }
    .await;

    if let Err(e) = result {
        error!("error processing stream: {:#}", e);
    } else {
        info!("Stream ended");
    }
}

struct HttpStream {
    send: SendStream,
    recv: RecvStream,
    version: [u8; 2],
}

#[async_trait]
impl types::HttpStream for HttpStream {
    async fn decompose<'a>(
        &mut self,
        resp: Result<types::HttpResponse>,
    ) -> Result<(
        &mut (dyn AsyncRead + Send + Sync + Unpin),
        &mut (dyn AsyncWrite + Send + Sync + Unpin),
    )> {
        let mut send = &mut self.send;
        let recv = &mut self.recv;

        send.write_chunks(&mut [
            DATA_STREAM_SIGNATURE.as_slice().into(),
            self.version.as_slice().to_owned().into(),
        ])
        .await?;

        let mut is_err = false;

        let response = match resp {
            Ok(resp) => {
                let mut metadata = Vec::new();

                for (key, val) in resp.headers {
                    metadata.push(Metadata {
                        key: format!("HttpHeader:{}", key),
                        val,
                    });
                }

                metadata.push(Metadata {
                    key: "HttpStatus".to_string(),
                    val: format!("{}", resp.status),
                });

                ConnectResponse {
                    error: None,
                    metadata,
                }
            }
            Err(e) => {
                is_err = true;

                ConnectResponse {
                    error: Some(format!("{:#}", e)),
                    metadata: Vec::new(),
                }
            }
        };

        let mut builder = TypedBuilder::<primitives::ConnectResponse::Owned>::new_default();
        response.to_primitive(builder.init_root());

        capnp_futures::serialize::write_message(&mut send, builder.into_inner()).await?;

        if is_err {
            Err(anyhow!("error in response"))
        } else {
            Ok((recv, send))
        }
    }
}
