use std::sync::Arc;
use std::time::Duration;

use ::utils::context::wait::Context;
use anyhow::{anyhow, Ok, Result};
use futures::StreamExt;
use log::info;
use quinn::{IdleTimeout, TransportConfig, VarInt};
use tokio::select;
use uuid::Uuid;

use crate::incoming::types::HandleHttp;

use self::utils::ControlStreamInfo;

use super::types::{ControlStreamError, Protocol, TunnelAuth};

use super::{edge::IpPortHost, types::RootCert};

pub(super) struct EdgeTunnelClient {
    id: Uuid,
    idx: u8,
    auth: TunnelAuth,

    handle: HandleHttp,
}

mod utils;

impl EdgeTunnelClient {
    pub fn new(id: Uuid, idx: u8, auth: TunnelAuth, handle: HandleHttp) -> Self {
        Self {
            id,
            idx,
            auth,
            handle,
        }
    }

    pub async fn serve(
        self,
        ctx: Context,
        protocol: Protocol,
        addr: IpPortHost,
        tls_config: RootCert,
        attempt: u8,
    ) -> Result<Option<ControlStreamError>> {
        let id = self.id;

        let resp = match protocol {
            Protocol::Quic => Ok(self
                .serve_quic(ctx.clone(), addr, tls_config, attempt)
                .await?),
        };

        info!("Tunnel client {} is shutting down", id);

        resp
    }

    async fn serve_quic(
        self,
        ctx: Context,
        addr: IpPortHost,
        tls_config: RootCert,
        attempt: u8,
    ) -> Result<Option<ControlStreamError>> {
        let client_crypto = tls_config.config;

        // not sure why we use the base ipv6 address here however it works.
        let mut endpoint = quinn::Endpoint::client("[::]:0".parse()?)?;
        // set the client config to the tls config we created earlier.
        let mut config = quinn::ClientConfig::new(Arc::new(client_crypto));

        let mut transport = TransportConfig::default();
        transport.max_concurrent_uni_streams(VarInt::from(0_u32));
        transport.max_concurrent_bidi_streams(VarInt::from(100_000_u32));
        transport.keep_alive_interval(Some(std::time::Duration::from_secs(30)));
        transport.max_idle_timeout(Some(IdleTimeout::from(VarInt::from(45 * 1000_u32))));
        transport.datagram_receive_buffer_size(None);

        config.transport = Arc::new(transport);

        endpoint.set_default_client_config(config);

        // create a new quinn connection to the edge server.
        let conn = endpoint
            .connect(addr.to_socket_addr(), tls_config.server_name.as_str())?
            .await
            .map_err(|e| anyhow!("failed to connect: {}", e))?;

        let (send, recv) = conn.connection.open_bi().await?;

        let (control_fut, local_ctx) = utils::serve_control_stream(
            ctx.clone(),
            ControlStreamInfo {
                id: self.id,
                auth: self.auth.clone(),
                idx: self.idx,
                version: "0.0.1".to_string(),
                arch: std::env::consts::ARCH.to_string(),
                features: Vec::new(),
                compression_quality: 0,
                num_previous_attempts: attempt,
                origin_local_ip: Vec::new(),
                timeout: Duration::from_secs(5),
            },
            send,
            recv,
        );

        let new_streams_fut = {
            let mut local_ctx = local_ctx;
            let new_streams = conn.bi_streams;
            let handle = self.handle;

            tokio::spawn(async move {
                let mut iter = new_streams.enumerate();
                loop {
                    select! {
                        stream = iter.next() => {
                            if let Some((_, stream)) = stream {
                                tokio::spawn(utils::serve_stream(local_ctx.clone(), stream?, handle.clone()));
                            } else {
                                return Err(anyhow!("stream iterator ended prematurely"));
                            }
                        },
                        _ = local_ctx.done() => break,
                    }
                }

                Ok(())
            })
        };

        select! {
            r = control_fut => {
                let r = r?;
                if let Err(e) = r {
                    return Ok(Some(e));
                }
            },
            r = new_streams_fut => {
                r??;
            },
        }

        Ok(None)
    }
}
