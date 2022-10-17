use std::net::IpAddr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    Quic,
}

pub(super) struct TLSSettings {
    pub server_name: String,
    pub next_protos: Vec<String>,
}

impl Protocol {
    pub(super) fn tls_settings(&self) -> TLSSettings {
        match self {
            Protocol::Quic => TLSSettings {
                server_name: "quic.cftunnel.com".to_string(),
                next_protos: vec!["argotunnel".to_string()],
            },
        }
    }
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
