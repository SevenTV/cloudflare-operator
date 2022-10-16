use std::{net::IpAddr, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
use utils::context::wait::Context;
use uuid::Uuid;

use crate::incoming::types::HttpMethod;

#[derive(Debug, Clone)]
pub(super) struct EdgeRegion {
    pub addrs: Vec<IpAddr>,
    pub hostname: String,
    pub port: u16,
}
pub enum EdgeRegionLocation {
    AUTO,
    US,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(super) enum Protocol {
    #[allow(dead_code)]
    None,
    Quic,
    HTTP2,
}

pub(super) struct TLSSettings {
    pub server_name: String,
    pub next_protos: Vec<String>,
}

impl Protocol {
    pub(super) fn fallback(&self) -> Self {
        match self {
            Protocol::Quic => Protocol::HTTP2,
            Protocol::HTTP2 => Protocol::None,
            Protocol::None => Protocol::None,
        }
    }

    pub(super) fn tls_settings(&self) -> TLSSettings {
        match self {
            Protocol::Quic => TLSSettings {
                server_name: "quic.cftunnel.com".to_string(),
                next_protos: vec!["argotunnel".to_string()],
            },
            Protocol::HTTP2 => TLSSettings {
                server_name: "h2.cftunnel.com".to_string(),
                next_protos: vec![],
            },
            Protocol::None => panic!("no tls settings for protocol None"),
        }
    }
}

pub type HandleHttp = Box<Arc<dyn HandleHttpTrait>>;

#[async_trait]
pub trait HandleHttpTrait: Send + Sync {
    async fn handle(
        &self,
        ctx: Context,
        req: HttpRequest,
        stream: Box<dyn HttpStream>,
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub is_websocket: bool,
    pub headers: Vec<(String, String)>,
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

#[async_trait]
pub trait HttpStream: Send + Sync {
    async fn decompose<'a>(
        &mut self,
        resp: Result<HttpResponse>,
    ) -> Result<(
        &mut (dyn AsyncRead + Send + Sync + Unpin),
        &mut (dyn AsyncWrite + Send + Sync + Unpin),
    )>;
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TunnelAuth {
    #[serde(rename = "a")]
    pub account_tag: String,
    #[serde(rename = "s")]
    pub tunnel_secret: String, // in base64
    #[serde(rename = "t")]
    pub tunnel_id: Uuid,
}

impl TunnelAuth {
    pub fn new(token: &str) -> Result<Self> {
        Ok(serde_json::from_slice(&base64::decode(token)?)?)
    }

    pub fn encode(&self) -> Result<String> {
        Ok(base64::encode_config(
            serde_json::to_string(self)?,
            base64::URL_SAFE_NO_PAD,
        ))
    }

    pub fn tunnel_secret_decode(&self) -> Result<Vec<u8>> {
        Ok(base64::decode_config(
            &self.tunnel_secret,
            base64::URL_SAFE_NO_PAD,
        )?)
    }
}
