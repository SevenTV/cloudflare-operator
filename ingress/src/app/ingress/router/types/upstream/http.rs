use crate::config::types as config;

#[derive(Debug, Clone)]
pub struct Upstream {
    pub host: String,
    pub port: u16,
}

impl From<config::ingress::upstream::http::HttpUpstream> for Upstream {
    fn from(upstream: config::ingress::upstream::http::HttpUpstream) -> Self {
        Self {
            host: upstream.host,
            port: upstream.port,
        }
    }
}
