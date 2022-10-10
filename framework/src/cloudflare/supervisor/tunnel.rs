use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::StreamExt;
use log::info;
use tokio::select;
use tokio_util::task::LocalPoolHandle;
use utils::context::wait::SuperContext;

use crate::cloudflare::rpc::alias::{
    interfaces::registration_server::RegisterConnectionParams, structs,
};
use crate::cloudflare::rpc::types::TunnelAuth;
use crate::cloudflare::rpc::{clients, new_network, ControlStreamManager};

use super::types::Protocol;

use super::{edge::IpPortHost, tls::RootCert};

pub(super) struct EdgeTunnelServer {
    id: u32,
    auth: TunnelAuth,
}

impl EdgeTunnelServer {
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
        LocalPoolHandle::new(1).spawn_pinned(|| {
            async move {
                match protocol {
                    Protocol::QUIC => Ok(self.serve_quic(ctx, addr, tls_config).await?),
                    Protocol::HTTP2 => Ok(self.serve_http2(ctx, addr, tls_config).await?),
                    _ => Err(anyhow!("Protocol not supported")),
                }
            }
        }).await?
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
            let (send, recv) = conn.connection.open_bi().await?;
    
            let (control_stream, system) = ControlStreamManager::new(new_network(send, recv));
    
            let system_fut = {
                let mut ctx = ctx.clone();
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
    
            let auth = self.auth.clone();
    
            info!("Registering tunnel with edge");
    
            let resp = tunnel_client
                .get_registration_client()
                .register_connection(RegisterConnectionParams {
                    auth: structs::TunnelAuth {
                        account_tag: auth.account_tag.clone(),
                        tunnel_secret: auth.tunnel_secret_decode()?,
                    },
                    tunnel_id: self.auth.tunnel_id.into_bytes().to_vec(),
                    conn_index: (self.id as u8),
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

            system_fut
        };

        let datagrams_fut = {
            tokio::task::spawn_local(async move {
                let mut stream = conn.datagrams.enumerate();
                while let Some((size, datagram)) = stream.next().await {
                    info!("Got datagram: {:?}", datagram.map_err(|e| anyhow!("failed to read datagram: {}", e))?);
                }

                return Ok::<(), anyhow::Error>(());
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
