use anyhow::{anyhow, Result};
use capnp::message::TypedBuilder;
use log::{error, info};
use quinn::{RecvStream, SendStream};
use tokio::select;

use utils::context::wait::{SuperContext, SuperHandle};

use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;

use crate::cloudflare::rpc::quic_metadata_protocol::primitives;
use crate::cloudflare::rpc::quic_metadata_protocol::structs::{
    ConnectRequest, ConnectResponse, Metadata,
};
use crate::cloudflare::rpc::tunnelrpc::interfaces::registration_server::UnregisterConnectionParams;
use crate::cloudflare::rpc::tunnelrpc::{
    interfaces::registration_server::RegisterConnectionParams, structs,
};
use crate::cloudflare::rpc::types::TunnelAuth;
use crate::cloudflare::rpc::{new_network, ControlStreamManager};

pub fn serve_control_stream(
    id: u8,
    auth: TunnelAuth,
    ctx: SuperContext,
    send: SendStream,
    recv: RecvStream,
) -> (JoinHandle<Result<()>>, SuperContext) {
    // We have to run the control stream on a single thread because capnp-rpc-rust was poorly designed.
    // This is a workaround for that.
    // We create a local pool handle with size 1 and run the control stream on that.
    // When this handle is dropped, the control stream will be dropped as well.
    let (local_ctx, local_handle) = SuperContext::new(None);

    let local_ctx_clone = local_ctx.clone();

    let fut = LocalPoolHandle::new(1).spawn_pinned(move || async move {
        serve_control_stream_single_thread(ctx, local_ctx_clone, local_handle, auth, id, send, recv)
            .await
    });

    (fut, local_ctx)
}

async fn serve_control_stream_single_thread(
    ctx: SuperContext,
    local_ctx: SuperContext,
    local_handle: SuperHandle,
    auth: TunnelAuth,
    id: u8,
    send: SendStream,
    recv: RecvStream,
) -> Result<()> {
    let mut ctx = ctx;
    let mut local_ctx = local_ctx;

    let (control_stream, system) = ControlStreamManager::new(new_network(send, recv));

    let mut system_fut = {
        tokio::task::spawn_local(async move {
            select! {
                _ = system => {}
                _ = local_ctx.done() => {}
            }

            return Ok::<(), anyhow::Error>(());
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
                conn_index: id.into(),
                options: structs::ConnectionOptions {
                    client: structs::ClientInfo {
                        client_id: auth.tunnel_id.into_bytes().to_vec(),
                        version: "test".to_string(),
                        arch: "test".to_string(),
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
            let _ = local_handle.cancel().await;

            // This should be resolved due to the above await.
            system_fut.await??;

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

pub async fn serve_stream(ctx: SuperContext, stream: (SendStream, RecvStream)) {
    let (send, recv) = stream;

    let mut ctx = ctx;

    let mut recv = recv;

    let mut send = send;

    let result: Result<()> = async move {
        match determine_protocol(&mut recv).await? {
            Protocol::RPCStream => Err(anyhow!("unimplemented")),
            Protocol::DataStream => {
                let mut buf = [0u8; 2];
                recv.read_exact(&mut buf).await?;

                let version = String::from_utf8(buf.to_vec())?;

                // should be 01
                println!("version: {}", version);

                let request = ConnectRequest::from_primitive(
                    capnp_futures::serialize::read_message(
                        &mut recv,
                        capnp::message::ReaderOptions::new(),
                    )
                    .await?
                    .get_root()?,
                )?;

                // at this point we know all the information we need to connect to the upstream server.
                // we can also reject the connection if we want to.
                // we have the following information:
                // - hostname
                // - path
                // - headers
                // - method
                // - ip address

                // lets respond with a 200 OK
                // and send back the headers we received.

                // set up a response
                send.write_chunks(&mut [
                    DATA_STREAM_SIGNATURE.as_slice().into(),
                    buf.as_slice().to_owned().into(),
                ])
                .await?;

                let response = ConnectResponse {
                    error: None,
                    metadata: vec![
                        Metadata {
                            key: "HttpStatus".to_string(),
                            val: "200".to_string(),
                        },
                        Metadata {
                            key: "HttpHeader:X-Test123".to_string(),
                            val: "kappa123".to_string(),
                        },
                    ],
                };

                let mut builder = TypedBuilder::<primitives::ConnectResponse::Owned>::new_default();
                response.to_primitive(builder.init_root());

                capnp_futures::serialize::write_message(&mut send, builder.into_inner()).await?;

                send.write(format!("{:#?}", request).as_bytes()).await?;

                Ok::<(), anyhow::Error>(())
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
