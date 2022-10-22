use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use capnp::message::TypedBuilder;
use capnp_rpc::RpcSystem;
use log::{error, info};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    select,
};

use tracing::warn;
use utils::context::wait::{Context, Handle as ContextHandle};

use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;
use uuid::Uuid;

use crate::incoming::{
    cloudflare_tunnels::types::{ControlStreamError, TunnelAuth},
    types::{HandleHttp, HttpMethod, HttpRequest, HttpResponse, HttpStream},
};

use generated::capnp::{
    quic_metadata_protocol::primitives,
    quic_metadata_protocol::structs::{ConnectRequest, ConnectResponse, Metadata},
    rpc::new_network_client,
    tunnelrpc::{
        interfaces::{
            registration_server::{RegisterConnectionParams, UnregisterConnectionParams},
            tunnel_server,
        },
        structs,
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ControlStreamInfo {
    pub auth: TunnelAuth,
    pub id: Uuid,
    pub idx: u8,
    pub version: String,
    pub arch: String,
    pub features: Vec<String>,
    pub compression_quality: u8,
    pub num_previous_attempts: u8,
    pub origin_local_ip: Vec<u8>,
    pub timeout: Duration,
}

pub(super) fn serve_control_stream<
    R: AsyncRead + Unpin + Send + Sync,
    W: AsyncWrite + Unpin + Send + Sync,
>(
    ctx: Context,
    info: ControlStreamInfo,
    send: W,
    recv: R,
) -> (JoinHandle<Result<(), ControlStreamError>>, Context)
where
    R: Sized + 'static,
    W: Sized + 'static,
{
    // We have to run the control stream on a single thread because capnp-rpc-rust was poorly designed.
    // This is a workaround for that.
    // We create a local pool handle with size 1 and run the control stream on that.
    // When this handle is dropped, the control stream will be dropped as well.
    let mut handle = ContextHandle::new();
    let local_ctx = handle.spawn();

    let fut = LocalPoolHandle::new(1).spawn_pinned(|| async move {
        serve_control_stream_single_thread(ServeControlStreamSingleThread::<R, W> {
            ctx,
            info,
            handle,
            recv,
            send,
        })
        .await
    });

    (fut, local_ctx)
}

struct ServeControlStreamSingleThread<R, W> {
    ctx: Context,
    info: ControlStreamInfo,
    handle: ContextHandle,
    recv: R,
    send: W,
}

async fn serve_control_stream_single_thread<
    R: AsyncRead + Unpin + Send + Sync + 'static,
    W: AsyncWrite + Unpin + Send + Sync + 'static,
>(
    args: ServeControlStreamSingleThread<R, W>,
) -> Result<(), ControlStreamError> {
    let ServeControlStreamSingleThread {
        mut ctx,
        mut handle,
        info,
        send,
        recv,
    } = args;

    let send = tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(send);
    let recv = tokio_util::compat::TokioAsyncReadCompatExt::compat(recv);

    let mut system = RpcSystem::new(new_network_client(send, recv), None);
    let tunnel_client = tunnel_server::client::Client::new_from_system(&mut system);

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

    {
        info!("Registering tunnel with edge");
        let ControlStreamInfo {
            auth,
            idx,
            id,
            version,
            features,
            arch,
            compression_quality,
            num_previous_attempts,
            origin_local_ip,
            timeout,
        } = info;

        let resp =
            tunnel_client
                .get_registration_client()
                .register_connection(RegisterConnectionParams {
                    auth: structs::TunnelAuth {
                        account_tag: auth.account_tag.clone(),
                        tunnel_secret: auth
                            .tunnel_secret()
                            .map_err(ControlStreamError::Other)?,
                    },
                    tunnel_id: auth.tunnel_id.into_bytes().to_vec(),
                    conn_index: idx,
                    options: structs::ConnectionOptions {
                        client: structs::ClientInfo {
                            client_id: id.into_bytes().to_vec(),
                            arch,
                            features,
                            version,
                        },
                        origin_local_ip,
                        compression_quality,
                        num_previous_attempts,
                        replace_existing: true,
                    },
                });

        select! {
            r = resp => {
                let r = r.map_err(|e| anyhow!("failed to register connection: {:#}", e)).map_err(ControlStreamError::Other)?;
                match r.result.result {
                    structs::ConnectionResponseResult::ConnectionDetails(details) => {
                        info!("Registered connection: {:?}", details);
                    }
                    structs::ConnectionResponseResult::Error(e) => {
                        // TODO we should look at the error and decide if we should retry...
                        return Err(ControlStreamError::Connection(e));
                    }
                }
            }
            _ = tokio::time::sleep(timeout) => {
                error!("Registering tunnel with edge timed out");
                return Err(ControlStreamError::Timeout(timeout));
            }
            _ = ctx.done() => {
                return Ok(());
            }
        }
    }

    select! {
        _ = &mut system_fut => {
            Err(ControlStreamError::Other(anyhow!("rpc system handler exited prematurely")))
        },
        _ = ctx.done() => {
            // This will always return an erorr because we dont get a response as cloudflare seems to rudely close the connection on us.
            // Therefore we can just ignore the response from here.
            let _ = tunnel_client.get_registration_client().unregister_connection(UnregisterConnectionParams{}).await;

            // We now need to clean up this thread.
            handle.cancel().await;

            Ok::<(), ControlStreamError>(())
        }
    }
}

const STREAM_SIGNATURE_LENGTH: usize = 6;
const DATA_STREAM_SIGNATURE: [u8; STREAM_SIGNATURE_LENGTH] = [0x0A, 0x36, 0xCD, 0x12, 0xA1, 0x3E];
const RPC_STREAM_SIGNATURE: [u8; STREAM_SIGNATURE_LENGTH] = [0x52, 0xBB, 0x82, 0x5C, 0xDB, 0x65];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Protocol {
    RPCStream,
    DataStream,
}

impl Protocol {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes == DATA_STREAM_SIGNATURE {
            Ok(Protocol::DataStream)
        } else if bytes == RPC_STREAM_SIGNATURE {
            Ok(Protocol::RPCStream)
        } else {
            Err(anyhow!("invalid protocol signature"))
        }
    }
}

#[inline]
async fn determine_protocol(
    recv: &mut (dyn AsyncRead + Unpin + Send + Sync + 'static),
) -> Result<Protocol> {
    let mut buf = [0u8; STREAM_SIGNATURE_LENGTH];

    recv.read_exact(&mut buf).await?;

    Protocol::from_bytes(&buf)
}

pub async fn serve_stream<
    R: AsyncRead + Unpin + Send + Sync + 'static,
    W: AsyncWrite + Unpin + Send + Sync + 'static,
>(
    ctx: Context,
    stream: (W, R),
    handle: HandleHttp,
) {
    let (send, mut recv) = stream;

    // tokio::time::timeout(duration, f).await
    
    
    let result: Result<()> = async move {
        match tokio::time::timeout(
            Duration::from_secs(2),
            determine_protocol(&mut recv),
        ).await?? {
            Protocol::RPCStream => Err(anyhow!("unimplemented")),
            Protocol::DataStream => {
                let mut buf = [0u8; 2];
                recv.read_exact(&mut buf).await?;

                let mut recv = tokio_util::compat::TokioAsyncReadCompatExt::compat(recv);

                let request = ConnectRequest::from_primitive(
                    tokio::time::timeout(Duration::from_secs(2), capnp_futures::serialize::read_message(
                        &mut recv,
                        capnp::message::ReaderOptions::new(),
                    ))
                    .await??
                    .get_root()?,
                )?;

                let recv = recv.into_inner();

                // https://example.com/abc
                // "https:" "" "example.com" "abc"
                let mut req = HttpRequest {
                    method: HttpMethod::GET,
                    path: "".to_string(),
                    headers: Vec::new(),
                    hostname: "".to_string(),
                    is_websocket: request.connection_type == primitives::ConnectionType::Websocket,
                    is_tls: false,
                };

                let mut valid = 0;
                for metadata in request.metadata {
                    if metadata.key.starts_with("HttpHeader:") {
                        let key = metadata.key.trim_start_matches("HttpHeader:");
                        let value = metadata.val;

                        req.headers.push((key.to_string(), value));
                    } else if metadata.key == "HttpHost" {
                        req.headers.push(("Host".to_string(), metadata.val.clone()));
                        req.hostname = metadata.val;
                        if valid & 1 != 0 {
                            return Err(anyhow!("duplicate HttpHost"));
                        }

                        valid |= 1;
                    } else if metadata.key == "HttpMethod" {
                        req.method = metadata.val.parse()?;
                        if valid & 2 != 0 {
                            return Err(anyhow!("duplicate HttpMethod"));
                        }

                        valid |= 2;
                    } else {
                        warn!("unhandled metadata: {:?}", metadata);
                    }
                }

                if request.dest.starts_with("https://") {
                    req.path = request.dest[8..].trim_start_matches(&req.hostname).to_string();
                    req.is_tls = true;
                } else if request.dest.starts_with("http://") {
                    req.path = request.dest[7..].trim_start_matches(&req.hostname).to_string();
                } else {
                    return Err(anyhow!("invalid dest: {}", request.dest));
                }

                if valid != 3 {
                    return Err(anyhow!("missing metadata"));
                }

                handle
                    .handle(
                        ctx,
                        req,
                        Box::new(HttpStreamImpl {
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

struct HttpStreamImpl<
    R: AsyncRead + Unpin + Send + Sync + 'static,
    W: AsyncWrite + Unpin + Send + Sync + 'static,
> {
    send: W,
    recv: R,
    version: [u8; 2],
}

#[async_trait]
impl<
        R: AsyncRead + Unpin + Send + Sync + 'static,
        W: AsyncWrite + Unpin + Send + Sync + 'static,
    > HttpStream for HttpStreamImpl<R, W>
{
    async fn decompose<'a>(
        &mut self,
        timeout: Duration,
        resp: Result<HttpResponse>,
    ) -> Result<(
        &mut (dyn AsyncRead + Send + Sync + Unpin),
        &mut (dyn AsyncWrite + Send + Sync + Unpin),
    )> {
        let (recv, send) = tokio::time::timeout(timeout, async {
            let send = &mut self.send;
            let recv = &mut self.recv;
            send.write_all(&DATA_STREAM_SIGNATURE).await?;
            send.write_all(&self.version).await?;
    
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
                        error: "".to_string(),
                        metadata,
                    }
                }
                Err(e) => {
                    is_err = true;
    
                    ConnectResponse {
                        error: format!("{:#}", e),
                        metadata: Vec::new(),
                    }
                }
            };
    
            let mut builder = TypedBuilder::<primitives::ConnectResponse::Owned>::new_default();
            response.to_primitive(builder.init_root());
    
            let mut send = tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(send);
    
            capnp_futures::serialize::write_message(&mut send, builder.into_inner()).await?;
    
            let send = send.into_inner();
    
            if is_err {
                Err(anyhow!("error in response"))
            } else {
                Ok((recv, send))
            }
        }).await.map_err(|_| anyhow!("decompose timeout"))??;

        Ok((recv, send))
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use generated::capnp::{
        rpc::new_network_server,
        tunnelrpc::{
            interfaces::registration_server::{
                self, server::Client, RegisterConnectionResults, UnregisterConnectionResults,
            },
            structs::{
                ConnectionDetails, ConnectionError, ConnectionResponse, ConnectionResponseResult,
            },
        },
    };
    use tokio::sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    };
    use utils::context::wait::Handle;

    use crate::incoming::types::HandleHttpTrait;

    use super::*;

    #[test]
    fn test_protocol() {
        assert_eq!(
            Protocol::from_bytes(&RPC_STREAM_SIGNATURE).unwrap(),
            Protocol::RPCStream
        );
        assert_eq!(
            Protocol::from_bytes(&DATA_STREAM_SIGNATURE).unwrap(),
            Protocol::DataStream
        );
        assert!(Protocol::from_bytes(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]).is_err());
    }

    fn setup_control_stream<T: AsyncRead + AsyncWrite + Sized + Sync + Send + 'static>(
        client: T,
        server: T,
        info: ControlStreamInfo,
    ) -> (
        Handle,
        (
            JoinHandle<Result<(), ControlStreamError>>,
            JoinHandle<Result<()>>,
        ),
        (
            Receiver<RegisterConnectionParams>,
            Sender<Result<RegisterConnectionResults, capnp::Error>>,
        ),
        (
            Receiver<UnregisterConnectionParams>,
            Sender<Result<UnregisterConnectionResults, capnp::Error>>,
        ),
    ) {
        let mut handle = Handle::default();

        #[derive(Clone)]
        struct Server {
            send_register_connection_params: tokio::sync::mpsc::Sender<RegisterConnectionParams>,
            recv_register_connection_results: Arc<
                Mutex<tokio::sync::mpsc::Receiver<Result<RegisterConnectionResults, capnp::Error>>>,
            >,
            send_unregister_connection_params:
                tokio::sync::mpsc::Sender<UnregisterConnectionParams>,
            recv_unregister_connection_results: Arc<
                Mutex<
                    tokio::sync::mpsc::Receiver<Result<UnregisterConnectionResults, capnp::Error>>,
                >,
            >,
        }

        #[async_trait]
        impl tunnel_server::server::Client for Server {}

        #[async_trait]
        impl registration_server::server::Client for Server {
            async fn register_connection(
                &self,
                request: RegisterConnectionParams,
            ) -> Result<RegisterConnectionResults, capnp::Error> {
                self.send_register_connection_params
                    .send(request)
                    .await
                    .unwrap();
                self.recv_register_connection_results
                    .lock()
                    .await
                    .recv()
                    .await
                    .unwrap()
            }

            async fn unregister_connection(
                &self,
                request: UnregisterConnectionParams,
            ) -> Result<UnregisterConnectionResults, capnp::Error> {
                self.send_unregister_connection_params
                    .send(request)
                    .await
                    .unwrap();
                self.recv_unregister_connection_results
                    .lock()
                    .await
                    .recv()
                    .await
                    .unwrap()
            }
        }

        let (send_register_connection_params, recv_register_connection_params) =
            tokio::sync::mpsc::channel(1);
        let (send_register_connection_results, recv_register_connection_results) =
            tokio::sync::mpsc::channel(1);

        let (send_unregister_connection_params, recv_unregister_connection_params) =
            tokio::sync::mpsc::channel(1);
        let (send_unregister_connection_results, recv_unregister_connection_results) =
            tokio::sync::mpsc::channel(1);

        let server = LocalPoolHandle::new(1).spawn_pinned(|| async move {
            let server_factory = Server {
                send_register_connection_params,
                recv_register_connection_results: Arc::new(Mutex::new(
                    recv_register_connection_results,
                )),
                send_unregister_connection_params,
                recv_unregister_connection_results: Arc::new(Mutex::new(
                    recv_unregister_connection_results,
                )),
            }
            .build();

            let (recv, send) = tokio::io::split(server);

            let send = tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(send);
            let recv = tokio_util::compat::TokioAsyncReadCompatExt::compat(recv);

            let server_network = new_network_server(send, recv);

            RpcSystem::new(server_network, Some(server_factory.client))
                .await
                .map_err(|e| anyhow!(e))
        });

        let (h, _) = {
            let (recv, send) = tokio::io::split(client);
            serve_control_stream(handle.spawn(), info, send, recv)
        };

        (
            handle,
            (h, server),
            (
                recv_register_connection_params,
                send_register_connection_results,
            ),
            (
                recv_unregister_connection_params,
                send_unregister_connection_results,
            ),
        )
    }

    #[tokio::test]
    async fn test_serve_control_stream() {
        let (client, server) = tokio::io::duplex(64);

        let info = ControlStreamInfo {
            version: "1.0.0".to_string(),
            arch: "x86_64".to_string(),
            auth: TunnelAuth {
                account_tag: "test".to_string(),
                tunnel_id: Uuid::new_v4(),
                tunnel_secret: "test".to_string(),
            },
            compression_quality: 1,
            idx: 1,
            num_previous_attempts: 12,
            features: vec!["test".to_string()],
            id: Uuid::new_v4(),
            origin_local_ip: "origin_local_ip".to_string().into_bytes(),
            timeout: Duration::from_secs(3),
        };

        let (
            handle,
            (h, server),
            (mut recv_register_connection_params, send_register_connection_results),
            (mut recv_unregister_connection_params, send_unregister_connection_results),
        ) = setup_control_stream(client, server, info.clone());

        let req = recv_register_connection_params.recv().await;
        assert!(req.is_some());
        let req = req.unwrap();
        assert_eq!(req.auth.tunnel_secret, info.auth.tunnel_secret().unwrap());
        assert_eq!(req.auth.account_tag, info.auth.account_tag);
        assert_eq!(req.tunnel_id, info.auth.tunnel_id.as_bytes());
        assert_eq!(req.conn_index, info.idx);
        assert_eq!(req.options.compression_quality, info.compression_quality);
        assert_eq!(
            req.options.num_previous_attempts,
            info.num_previous_attempts
        );
        assert_eq!(req.options.origin_local_ip, info.origin_local_ip);
        assert_eq!(req.options.replace_existing, true);
        assert_eq!(req.options.client.arch, info.arch);
        assert_eq!(req.options.client.version, info.version);
        assert_eq!(req.options.client.client_id, info.id.as_bytes());
        assert_eq!(req.options.client.features, info.features);

        send_register_connection_results
            .send(Ok(RegisterConnectionResults {
                result: ConnectionResponse {
                    result: ConnectionResponseResult::ConnectionDetails(ConnectionDetails {
                        uuid: Uuid::new_v4().as_bytes().to_vec(),
                        location_name: "test".to_string(),
                        tunnel_is_remotely_managed: true,
                    }),
                },
            }))
            .await
            .unwrap();

        // We sleep here to allow the client to process the request.
        tokio::time::sleep(Duration::from_millis(100)).await;

        // We should have received a connection.
        let shutdown = handle.cancel();

        let req = recv_unregister_connection_params.recv().await;
        assert!(req.is_some());
        send_unregister_connection_results
            .send(Ok(UnregisterConnectionResults {}))
            .await
            .unwrap();

        select! {
            _ = async move {
                let h = h.await;
                assert!(h.is_ok());
                assert!(h.unwrap().is_ok());
                let s = server.await;
                assert!(s.is_ok());
                assert!(s.unwrap().is_ok());
            } => {}
            _ = tokio::time::sleep(Duration::from_millis(100)) => panic!("timed out"),
        }

        select! {
            _ = shutdown => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                panic!("timed out");
            }
        };
    }

    #[tokio::test]
    async fn test_serve_control_stream_error() {
        let (client, server) = tokio::io::duplex(64);

        let mut info = ControlStreamInfo::default();
        info.timeout = Duration::from_secs(3);

        let (
            handle,
            (h, server),
            (mut recv_register_connection_params, send_register_connection_results),
            (_, _),
        ) = setup_control_stream(client, server, info.clone());

        let req = recv_register_connection_params.recv().await;
        assert!(req.is_some());

        let error = ConnectionError {
            cause: "test".to_string(),
            retry_after: 0,
            should_retry: true,
        };

        send_register_connection_results
            .send(Ok(RegisterConnectionResults {
                result: ConnectionResponse {
                    result: ConnectionResponseResult::Error(error.clone()),
                },
            }))
            .await
            .unwrap();

        // We sleep here to allow the client to process the request.
        tokio::time::sleep(Duration::from_millis(100)).await;

        assert!(h.is_finished());
        let h = h.await;
        assert!(h.is_ok());

        // we should check this error...
        let h = h.unwrap();
        assert!(h.is_err());
        match h.unwrap_err() {
            ControlStreamError::Connection(e) => assert_eq!(e, error),
            _ => panic!("unexpected error"),
        }

        assert!(server.is_finished());
        let server = server.await;
        assert!(server.is_ok());
        assert!(server.unwrap().is_ok());

        // We should not have received a connection.

        let shutdown = handle.cancel();

        select! {
            _ = shutdown => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                panic!("timed out");
            }
        };
    }

    #[tokio::test]
    async fn test_serve_control_stream_timeout() {
        let (client, server) = tokio::io::duplex(64);

        let mut info = ControlStreamInfo::default();
        let timeout = Duration::from_millis(500);
        info.timeout = timeout;

        let (
            handle,
            (h, server),
            (mut recv_register_connection_params, _send_register_connection_results),
            (_, _),
        ) = setup_control_stream(client, server, info.clone());

        let req = recv_register_connection_params.recv().await;
        assert!(req.is_some());

        // We sleep here to allow the client to process the request.
        tokio::time::sleep(Duration::from_millis(600)).await;

        assert!(h.is_finished());
        let h = h.await;
        assert!(h.is_ok());

        // we should check this error...
        let h = h.unwrap();
        assert!(h.is_err());

        match h.unwrap_err() {
            ControlStreamError::Timeout(t) => assert_eq!(t, timeout),
            e => panic!("unexpected error {:?}", e),
        }

        assert!(server.is_finished());
        let server = server.await;
        assert!(server.is_ok());

        // We should not have received a connection.

        let shutdown = handle.cancel();

        select! {
            _ = shutdown => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                panic!("timed out");
            }
        };
    }

    #[tokio::test]
    async fn test_serve_control_stream_shutdown() {
        let (client, server) = tokio::io::duplex(64);

        let mut info = ControlStreamInfo::default();
        let timeout = Duration::from_millis(500);
        info.timeout = timeout;

        let (
            handle,
            (h, server),
            (mut recv_register_connection_params, _send_register_connection_results),
            (_, _),
        ) = setup_control_stream(client, server, info.clone());

        let req = recv_register_connection_params.recv().await;
        assert!(req.is_some());

        // We sleep here to allow the client to process the request.

        let shutdown = handle.cancel();

        tokio::time::sleep(Duration::from_millis(100)).await;

        assert!(h.is_finished());
        let h = h.await;
        assert!(h.is_ok());

        // we should check this error...
        let h = h.unwrap();
        assert!(h.is_ok());

        assert!(server.is_finished());
        let server = server.await;
        assert!(server.is_ok());

        select! {
            _ = shutdown => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                panic!("timed out");
            }
        };
    }

    fn setup_serve_stream<T: AsyncRead + AsyncWrite + Sized + Sync + Send + 'static>(client: T) -> (Handle, JoinHandle<()>, (Receiver<(Context, HttpRequest, Box<dyn HttpStream>)>, Sender<(Context, Result<()>)>)) {
        let mut handle = Handle::new();

        let (send_req, recv_req) = tokio::sync::mpsc::channel(1);
        let (send_resp, recv_resp) = tokio::sync::mpsc::channel(1);

        struct HttpHandle{
            send_req: tokio::sync::mpsc::Sender<(Context, HttpRequest, Box<dyn HttpStream>)>,
            recv_resp: Arc<Mutex<tokio::sync::mpsc::Receiver<(Context, Result<()>)>>>,
        }

        #[async_trait]
        impl HandleHttpTrait for HttpHandle {
            async fn handle(
                &self,
                ctx: Context,
                req: HttpRequest,
                stream: Box<dyn HttpStream>,
            ) -> Result<()> {
                self.send_req.send((ctx, req, stream)).await.map_err(|_| anyhow!("send failed"))?;
                self.recv_resp.lock().await.recv().await.unwrap().1
            }
        }

        let http_handle = Arc::new(HttpHandle {
            send_req,
            recv_resp: Arc::new(Mutex::new(recv_resp)),
        });

        let client_fut = {
            let ctx = handle.spawn();
            let (r, w) = tokio::io::split(client);

            tokio::spawn(serve_stream(ctx, (w, r), Box::new(http_handle)))
        };

        (handle, client_fut, (recv_req, send_resp))
    }

    #[tokio::test]
    async fn test_serve_stream() {
        let (client, mut server) = tokio::io::duplex(64);

        let (handle, client_fut, (mut recv_req, send_resp)) = setup_serve_stream(client);

        let version = b"01".to_vec();

        assert!(server.write_all(&DATA_STREAM_SIGNATURE).await.is_ok()); // write signature
        assert!(server.write_all(&version).await.is_ok()); // version

        // now we should be able to write the request
        let mut msg = capnp::message::Builder::new_default();
        let body = msg.init_root::<primitives::ConnectRequest::Builder>();
        let req = ConnectRequest{
            connection_type: primitives::ConnectionType::Http,
            dest: "https://test.com/abc".to_string(),
            metadata: vec![Metadata{
                key: "HttpHost".to_string(),
                val: "test.com".to_string(),
            }, Metadata{
                key: "HttpMethod".to_string(),
                val: "GET".to_string(),
            }],
        };

        req.to_primitive(body);

        let mut server = tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(server);

        assert!(capnp_futures::serialize::write_message(&mut server, &msg).await.is_ok());

        let (ctx, req, mut stream) = recv_req.recv().await.unwrap();
        assert!(!req.is_websocket);
        assert_eq!(req.method, HttpMethod::GET);
        assert_eq!(req.hostname, "test.com");
        assert_eq!(req.path, "/abc");
        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].0, "Host");
        assert_eq!(req.headers[0].1, "test.com");
        assert!(req.is_tls);

        let resp = HttpResponse{
            status: 200,
            headers: Vec::new(),
        };

        let server = tokio::spawn(async move {
             // we should be able to read the response
            let mut server = server.into_inner();
            let protocol = determine_protocol(&mut server).await;
            assert!(protocol.is_ok());
            assert_eq!(protocol.unwrap(), Protocol::DataStream);
            
            let mut buf = [0u8; 2];
            assert!(server.read_exact(&mut buf).await.is_ok());
            assert_eq!(buf.to_vec(), version);
            
            let mut server = tokio_util::compat::TokioAsyncReadCompatExt::compat(server);
            let msg = capnp_futures::serialize::read_message(&mut server, Default::default()).await;
            assert!(msg.is_ok());
            let msg = msg.unwrap();
            let root = msg.get_root::<primitives::ConnectResponse::Reader>();
            assert!(root.is_ok());
            let root = root.unwrap();
            let resp = ConnectResponse::from_primitive(root);
            assert!(resp.is_ok());
            let resp = resp.unwrap();
            assert_eq!(resp.error, "");
            assert_eq!(resp.metadata.len(), 1);
            assert_eq!(resp.metadata[0].key, "HttpStatus");
            assert_eq!(resp.metadata[0].val, "200");
        });

        let streams = stream.decompose(Duration::from_secs(1), Ok(resp.clone())).await;
        assert!(streams.is_ok());

        tokio::time::sleep(Duration::from_millis(10)).await;

        assert!(server.is_finished());
        let server = server.await;
        assert!(server.is_ok());

        send_resp.send((ctx, Ok(()))).await.unwrap();

        tokio::time::sleep(Duration::from_millis(10)).await;

        assert!(client_fut.is_finished());
        assert!(client_fut.await.is_ok());

        handle.cancel().await;
    }
}
