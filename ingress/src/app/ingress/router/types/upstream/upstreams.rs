use crate::app::ingress::router::types::*;
use crate::config::types as config;

#[derive(Debug, Clone)]
pub enum Http {
    Http(upstream::http::Upstream),
}

impl From<config::ingress::upstream::Container<config::ingress::upstream::http::Upstreams>>
    for upstream::container::Container<Http>
{
    fn from(
        upstream: config::ingress::upstream::Container<config::ingress::upstream::http::Upstreams>,
    ) -> Self {
        Self {
            weight: upstream.weight.unwrap_or(1),
            upstream: match upstream.upstream {
                config::ingress::upstream::http::Upstreams::Http(http) => Http::Http(http.into()),
            },
        }
    }
}
