use crate::app::ingress::router::types::*;
use crate::config::types as config;

#[derive(Debug, Clone)]
pub struct Container<T> {
    pub paths: Vec<(http::path::Type, http::path::Method)>,
    pub hostnames: Vec<ingress::Hostname>,
    pub ingress: T,
    pub upstreams: Vec<upstream::container::Container<upstream::upstreams::Http>>,
}

impl From<config::ingress::http::PathUnion> for Vec<(http::path::Type, http::path::Method)> {
    fn from(path: config::ingress::http::PathUnion) -> Self {
        match path {
            config::ingress::http::PathUnion::Multi { paths } => {
                paths.into_iter().map(|path| path.into()).collect()
            }
            config::ingress::http::PathUnion::Single { path } => vec![path.into()],
        }
    }
}

impl From<config::ingress::http::HostnameUnion> for Vec<ingress::Hostname> {
    fn from(hostname: config::ingress::http::HostnameUnion) -> Self {
        match hostname {
            config::ingress::http::HostnameUnion::Multi { hostnames } => hostnames
                .into_iter()
                .map(|hostname| hostname.into())
                .collect(),
            config::ingress::http::HostnameUnion::Single { hostname } => vec![hostname.into()],
        }
    }
}
