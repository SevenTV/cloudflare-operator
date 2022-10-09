use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::info;
use utils::context::wait::SuperContext;

use super::types::Protocol;

use super::{edge::IpPortHost, tls::RootCert};

pub(super) struct EdgeTunnelServer {
    id: u32,
}

impl EdgeTunnelServer {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub async fn serve(
        &self,
        ctx: SuperContext,
        protocol: Protocol,
        addr: &IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        match protocol {
            Protocol::QUIC => self.serve_quic(ctx, addr, tls_config).await,
            Protocol::HTTP2 => self.serve_http2(ctx, addr, tls_config).await,
            _ => Err(anyhow!("Protocol not supported")),
        }
    }

    pub async fn serve_quic(
        &self,
        ctx: SuperContext,
        addr: &IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        let client_crypto = tls_config.config;

        let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())?; // not sure why we use the base ipv6 address here however it works.
        endpoint.set_default_client_config(quinn::ClientConfig::new(Arc::new(client_crypto))); // set the client config to the tls config we created earlier.

        let mut ctx = ctx;

        tokio::select! {
            _ = ctx.done() => {
                info!("Context cancelled");
                return Ok(())
            }
            conn = async {
                endpoint
                    .connect(addr.to_socket_addr(), tls_config.server_name.as_str())?
                    .await
                    .map_err(|e| anyhow!("failed to connect: {}", e))
            } => {
                let conn = conn?;

                info!("Connection established {:?}", conn.connection.stats());

                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        }

        // let (mut send, recv) = conn
        //     .connection
        //     .open_bi()
        //     .await
        //     .map_err(|e| anyhow!("failed to open bi: {}", e))?;

        Ok(())
    }

    pub async fn serve_http2(
        &self,
        _ctx: SuperContext,
        _addr: &IpPortHost,
        _tls_config: RootCert,
    ) -> Result<()> {
        panic!("not implemented")
    }
}
