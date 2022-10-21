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
    tunnel_secret: String, // in base64
    #[serde(rename = "t")]
    pub tunnel_id: Uuid,
}

impl TunnelAuth {
    pub fn new(token: &str) -> Result<Self> {
        Ok(serde_json::from_slice(&base64::decode_config(token, base64::URL_SAFE_NO_PAD)?)?)
    }

    pub fn set_tunnel_secret(&mut self, secret: &str) {
        self.tunnel_secret = base64::encode_config(secret, base64::URL_SAFE_NO_PAD);
    }

    pub fn tunnel_secret(&self) -> Result<Vec<u8>> {
        Ok(base64::decode_config(&self.tunnel_secret, base64::URL_SAFE_NO_PAD)?)
    }

    pub fn encode(&self) -> Result<String> {
        Ok(base64::encode_config(serde_json::to_string(self)?, base64::URL_SAFE_NO_PAD))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tunnel_auth() {
        let token = "eyJhIjoiY2ZfYWNjb3VudF90YWciLCJzIjoiWTJaZmRIVnVibVZzWDNObFkzSmxkQT09IiwidCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCJ9";
        let auth = super::TunnelAuth::new(token);
        assert!(auth.is_ok());

        let auth = auth.unwrap();
        assert_eq!(auth.account_tag, "cf_account_tag");
        assert_eq!(auth.tunnel_id, uuid::Uuid::nil());

        let secret = auth.tunnel_secret();
        assert!(secret.is_ok());
        assert_eq!(secret.unwrap(), b"cf_tunnel_secret");
    }

    #[test]
    fn test_tunnel_auth_encode() {
        let token = "eyJhIjoiY2ZfYWNjb3VudF90YWciLCJzIjoiWTJaZmRIVnVibVZzWDNObFkzSmxkQT09IiwidCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCJ9";
        let auth = super::TunnelAuth::new(token);
        assert!(auth.is_ok());
        let encoded = auth.unwrap().encode();
        assert!(encoded.is_ok());
        assert_eq!(encoded.unwrap(), token);
    }

    #[test]
    fn test_tunnel_auth_set_secret() {
        let token = "eyJhIjoiY2ZfYWNjb3VudF90YWciLCJzIjoiWTJaZmRIVnVibVZzWDNObFkzSmxkQT09IiwidCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCJ9";
        let auth = super::TunnelAuth::new(token);
        assert!(auth.is_ok());

        let mut auth = auth.unwrap();
        auth.set_tunnel_secret("new_secret");
        let secret = auth.tunnel_secret();
        assert!(secret.is_ok());
        assert_eq!(secret.unwrap(), b"new_secret");
    }

    #[test]
    fn test_tunnel_auth_set_secret_encode() {
        let token = "eyJhIjoiY2ZfYWNjb3VudF90YWciLCJzIjoiWTJaZmRIVnVibVZzWDNObFkzSmxkQT09IiwidCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCJ9";
        let mut auth = super::TunnelAuth::new(token).unwrap();
        auth.set_tunnel_secret("new_secret");
        let encoded = auth.encode();
        assert!(encoded.is_ok());
        assert_eq!(encoded.unwrap(), "eyJhIjoiY2ZfYWNjb3VudF90YWciLCJzIjoiYm1WM1gzTmxZM0psZEEiLCJ0IjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAwIn0");
    }
}
