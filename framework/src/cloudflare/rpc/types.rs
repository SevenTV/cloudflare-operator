use std::net::IpAddr;

use anyhow::Result;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) struct TunnelOptions {
    pub client_info: ClientInfo,
    pub origin_local_ip: IpAddr,
    pub replace_existing: bool,
    pub compression_quality: u8,
    pub num_previous_attempts: u8,
}

bitflags! {
    pub(crate) struct TunnelFeatures: u32 {
        const SERIALIZED_HEADERS = 1 << 0;
        const QUICK_RECONNECT = 1 << 1;
        const ALLOW_REMOTE_CONFIG = 1 << 2;
        const DATAGRAM_V2 = 1 << 3;
        const POST_QUANTUM = 1 << 4;

        const DEFAULT = Self::ALLOW_REMOTE_CONFIG.bits | Self::SERIALIZED_HEADERS.bits | Self::DATAGRAM_V2.bits;
    }
}

impl TunnelFeatures {
    pub fn to_vec(&self) -> Vec<String> {
        let mut features = Vec::new();

        if self.contains(Self::SERIALIZED_HEADERS) {
            features.push("serialized_headers".to_string());
        }

        if self.contains(Self::QUICK_RECONNECT) {
            features.push("quick_reconnect".to_string());
        }

        if self.contains(Self::ALLOW_REMOTE_CONFIG) {
            features.push("allow_remote_config".to_string());
        }

        if self.contains(Self::DATAGRAM_V2) {
            features.push("datagram_v2".to_string());
        }

        if self.contains(Self::POST_QUANTUM) {
            features.push("post_quantum".to_string());
        }

        features
    }
}

pub(crate) struct ClientInfo {
    pub client_id: uuid::Uuid, // this is required for capnp
    pub features: TunnelFeatures,
    pub version: String,
    pub arch: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TunnelAuth {
    #[serde(rename = "a")]
    pub account_tag: String,
    #[serde(rename = "s")]
    pub tunnel_secret: String,
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
}
