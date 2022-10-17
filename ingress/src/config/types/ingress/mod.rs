use serde::Deserialize;

pub mod http;
pub mod upstream;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum IngressRule {
    CloudflareTunnel(http::Container<http::cloudflare::Ingress>),
}
