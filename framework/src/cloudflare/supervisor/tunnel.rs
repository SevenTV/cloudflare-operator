use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use futures::StreamExt;
use log::info;
use tokio::select;
use tokio::sync::mpsc;
use tokio_util::task::LocalPoolHandle;
use utils::context::wait::SuperContext;

use crate::cloudflare::rpc::alias::interfaces::registration_server::UnregisterConnectionParams;
use crate::cloudflare::rpc::alias::{
    interfaces::registration_server::RegisterConnectionParams, structs,
};
use crate::cloudflare::rpc::types::TunnelAuth;
use crate::cloudflare::rpc::{new_network, ControlStreamManager};
use crate::cloudflare::supervisor::datagram;

use super::types::Protocol;

use super::{edge::IpPortHost, tls::RootCert};

pub(super) struct EdgeTunnelClient {
    id: u32,
    auth: TunnelAuth,
}

impl EdgeTunnelClient {
    pub fn new(id: u32, auth: TunnelAuth) -> Self {
        Self { id, auth }
    }

    pub async fn serve(
        self,
        ctx: SuperContext,
        protocol: Protocol,
        addr: IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        let id = self.id.clone();

        let resp = match protocol {
            Protocol::QUIC => Ok(self.serve_quic(ctx.clone(), addr, tls_config).await?),
            Protocol::HTTP2 => Ok(self.serve_http2(ctx.clone(), addr, tls_config).await?),
            _ => Err(anyhow!("Protocol not supported")),
        };

        info!("Tunnel client {} is shutting down", id);

        resp
    }

    pub async fn serve_quic(
        self,
        ctx: SuperContext,
        addr: IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        let client_crypto = tls_config.config;

        // not sure why we use the base ipv6 address here however it works.
        let mut endpoint = quinn::Endpoint::client("[::]:0".parse()?)?;
        // set the client config to the tls config we created earlier.
        endpoint.set_default_client_config(quinn::ClientConfig::new(Arc::new(client_crypto)));

        // create a new quinn connection to the edge server.
        let conn = endpoint
            .connect(addr.to_socket_addr(), tls_config.server_name.as_str())?
            .await
            .map_err(|e| anyhow!("failed to connect: {}", e))?;

        // This context is used to gracefully shutdown the connection. When the supervisor is shutting down.

        let (send, recv) = conn.connection.open_bi().await?;

        let (control_fut, local_ctx) = {
            let (_lctx, local_handle) = utils::context::wait::SuperContext::new(None);
            let id = self.id.clone();
            let auth = self.auth.clone();
            let mut local_ctx = _lctx.clone();
            let mut ctx = ctx.clone();

            // We have to run the control stream on a single thread because capnp-rpc-rust was poorly designed.
            // This is a workaround for that.
            // We create a local pool handle with size 1 and run the control stream on that.
            // When this handle is dropped, the control stream will be dropped as well.
            (LocalPoolHandle::new(1).spawn_pinned(move || async move {
                let (control_stream, system) = ControlStreamManager::new(new_network(send, recv));

                let mut system_fut = {
                    tokio::task::spawn_local(async move {
                        select! {
                            r = system => {
                                info!("system exited with {:?}", r);
                            }
                            _ = local_ctx.done() => {}
                        }

                        return Ok::<(), anyhow::Error>(());
                    })
                };

                let tunnel_client = control_stream.get_tunnel_client();

                {
                    info!("Registering tunnel with edge");

                    let resp = tunnel_client
                        .get_registration_client()
                        .register_connection(RegisterConnectionParams {
                            auth: structs::TunnelAuth {
                                account_tag: auth.account_tag.clone(),
                                tunnel_secret: auth.tunnel_secret_decode()?,
                            },
                            tunnel_id: auth.tunnel_id.into_bytes().to_vec(),
                            conn_index: (id as u8),
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
                    r = &mut system_fut => {
                        info!("System exited prematurely");
                        r??;
                    },
                    _ = ctx.done() => {
                        // This will always return an erorr because we dont get a response as cloudflare seems to rudely close the connection on us.
                        // Therefore we can just ignore the response from here.
                        let _ = tunnel_client.get_registration_client().unregister_connection(UnregisterConnectionParams{}).await;

                        // We now need to clean up this thread.
                        let _ = local_handle.cancel().await;

                        // This should be resolved due to the above await.
                        system_fut.await??;
                    }
                };

                info!("finished control stream");

                Ok::<(), anyhow::Error>(())
            }), _lctx)
        };

        let (dm, dm_req) = datagram::Manager::new(conn.connection, conn.datagrams);

        let datagrams_fut = {
            let mut local_ctx = local_ctx;
            tokio::spawn(async move {
                select! {
                    r = dm.serve() => {
                        r?;
                    }
                    _ = local_ctx.done() => {}
                }

                info!("finished datagram manager");

                Ok::<(), anyhow::Error>(())
            })
        };

        info!("{:?}", tokio::join!(control_fut, datagrams_fut));

        Ok(())
    }

    pub async fn serve_http2(
        self,
        _ctx: SuperContext,
        _addr: IpPortHost,
        _tls_config: RootCert,
    ) -> Result<()> {
        panic!("not implemented")
    }
}
