pub mod cloudflare_tunnels;
use crate::config::types as config;

#[derive(Debug, Clone)]
pub enum Hostname {
    String(String),
}

impl From<config::ingress::http::Hostname> for Hostname {
    fn from(hostname: config::ingress::http::Hostname) -> Self {
        match hostname {
            config::ingress::http::Hostname::String(hostname) => Self::String(hostname),
        }
    }
}
