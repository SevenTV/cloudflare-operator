use uuid::Uuid;

use crate::app::ingress::router::types::*;
use crate::config::types as config;

#[derive(Debug, Clone)]
pub struct Ingress {
    pub tunnel_id: Uuid,
    pub account_id: String,
    pub auth: auth::cloudflare::Auth,
    pub ingress: Vec<Container>,
}

pub type Container = http::container::Container<Parameters>;

#[derive(Debug, Clone)]
pub struct Parameters {}

type ConfigContainer = config::ingress::http::Container<config::ingress::http::cloudflare::Ingress>;

impl From<ConfigContainer> for Container {
    fn from(container: ConfigContainer) -> Self {
        Self {
            paths: container.path.map_or_else(Vec::new, |p| p.into()),
            hostnames: container.hostname.map_or_else(Vec::new, |h| h.into()),
            ingress: Parameters {},
            upstreams: container.upstreams.map_or_else(Vec::new, |u| {
                u.iter().map(|u| u.to_owned().into()).collect()
            }),
        }
    }
}
