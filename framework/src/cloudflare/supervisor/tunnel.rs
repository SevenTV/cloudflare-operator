use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::StreamExt;
use log::info;
use tokio::select;
use tokio::sync::mpsc;
use tokio_util::task::LocalPoolHandle;
use utils::context::wait::SuperContext;

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
        match protocol {
            Protocol::QUIC => Ok(self.serve_quic(ctx, addr, tls_config).await?),
            Protocol::HTTP2 => Ok(self.serve_http2(ctx, addr, tls_config).await?),
            _ => Err(anyhow!("Protocol not supported")),
        }
    }

    pub async fn serve_quic(
        self,
        ctx: SuperContext,
        addr: IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        let client_crypto = tls_config.config;

        let mut endpoint = quinn::Endpoint::client("[::]:0".parse()?)?; // not sure why we use the base ipv6 address here however it works.
        endpoint.set_default_client_config(quinn::ClientConfig::new(Arc::new(client_crypto))); // set the client config to the tls config we created earlier.

        let conn = endpoint
            .connect(addr.to_socket_addr(), tls_config.server_name.as_str())?
            .await
            .map_err(|e| anyhow!("failed to connect: {}", e))?;

        let control_fut = {
            let auth = self.auth.clone();
            let id = self.id;
            let (send, recv) = conn.connection.open_bi().await?;
            let mut ctx = ctx.clone();

            // We have to run the control stream on a single thread because capnp-rpc-rust was poorly designed.
            // This is a workaround for that.
            // We create a local pool handle with size 1 and run the control stream on that.
            // When this handle is dropped, the control stream will be dropped as well.
            LocalPoolHandle::new(1).spawn_pinned(move || async move {
                let (control_stream, system) = ControlStreamManager::new(new_network(send, recv));

                let system_fut = {
                    tokio::task::spawn_local(async move {
                        select! {
                            r = system => {
                                info!("system exited with {:?}", r);
                                return Ok(r?);
                            }
                            _ = ctx.done() => {
                                info!("Context closed");
                            }
                        }
                        return Ok::<(), anyhow::Error>(());
                    })
                };

                let tunnel_client = control_stream.get_tunnel_client();

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

                system_fut.await??;

                Ok::<(), anyhow::Error>(())
            })
        };

        let (dm, dm_req) = datagram::Manager::new(conn.connection, conn.datagrams);

        let datagrams_fut = {
            let mut ctx = ctx.clone();

            tokio::spawn(async move {
                select! {
                    r = dm.serve() => {
                        info!("datagram manager exited with {:?}", r);
                        Ok(r?)
                    }
                    _ = ctx.done() => {
                        info!("Context closed");
                        Ok::<(), anyhow::Error>(())
                    }
                }
            })
        };

        drop(ctx);

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
