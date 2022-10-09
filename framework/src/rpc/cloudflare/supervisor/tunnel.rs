use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio_context::context::Context;

use crate::rpc::cloudflare::Protocol;

use super::{edge::IpPortHost, tls::RootCert};

pub struct EdgeTunnelServer {
    id: u32,
}

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

impl EdgeTunnelServer {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub async fn serve(
        &self,
        ctx: &Context,
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
        ctx: &Context,
        addr: &IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        let client_crypto = tls_config.config;

        let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())?;
        endpoint.set_default_client_config(quinn::ClientConfig::new(Arc::new(client_crypto)));

        let conn = endpoint
            .connect(addr.to_socket_addr(), tls_config.server_name.as_str())?
            .await
            .map_err(|e| anyhow!("failed to connect: {}", e))?;

        let (mut send, recv) = conn
            .connection
            .open_bi()
            .await
            .map_err(|e| anyhow!("failed to open bi: {}", e))?;

        time::delay_for(time::Duration::from_secs(5)).await;

        Ok(())
    }

    pub async fn serve_http2(
        &self,
        ctx: &Context,
        addr: &IpPortHost,
        tls_config: RootCert,
    ) -> Result<()> {
        panic!("not implemented")
    }
}
