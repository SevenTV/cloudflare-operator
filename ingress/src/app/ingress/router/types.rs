pub enum IngressType {
    CloudflareTunnel(CloudflareTunnel),
}

pub mod auth {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Type {
        Cloudflare = 1,
    }

    #[derive(Debug, Clone)]
    pub enum Auth {
        Cloudflare(Cloudflare),

        #[allow(dead_code)]
        Unknown,
    }

    #[derive(Debug, Clone)]
    pub enum Cloudflare {
        ApiKey { key: String, email: String },
        ApiToken { token: String },
    }
}

#[derive(Debug, Clone)]
pub struct CloudflareTunnel {
    pub id: uuid::Uuid,
    pub account_id: String,
    pub auth: auth::Cloudflare,
    pub ingress: Vec<http::Container<http::cloudflare_tunnel::Ingress>>,
}

#[derive(Debug, Clone)]
pub enum Hostname {
    String(String),
}

pub mod http {
    use anyhow::Result;
    use bitflags::bitflags;

    use crate::config::cfg;

    #[derive(Debug, Clone)]
    pub struct Container<T> {
        pub paths: Vec<(PathType, PathMethod)>,
        pub hostnames: Vec<super::Hostname>,
        pub ingress: T,
        pub upstreams: Vec<upstream::HttpUpstreamContainer>,
    }

    pub fn cloudflare_tunnel_ingress_from_cfg(
        rule: &cfg::rules::http::Container<cfg::rules::http::cloudflare::Ingress>,
    ) -> Result<Container<cloudflare_tunnel::Ingress>> {
        let mut upstreams = Vec::new();

        if let Some(ups) = &rule.upstreams {
            for upstream in ups {
                upstreams.push(upstream::HttpUpstreamContainer::from_cfg(upstream)?);
            }
        }

        let mut hostnames = Vec::new();
        if let Some(hostname) = &rule.hostname {
            let tmp = match hostname {
                cfg::rules::http::HostnameUnion::Multi { hostnames } => hostnames.clone(),
                cfg::rules::http::HostnameUnion::Single { hostname } => {
                    vec![hostname.clone()]
                }
            };

            for hostname in tmp {
                match hostname {
                    cfg::rules::http::Hostname::String(hostname) => {
                        hostnames.push(super::Hostname::String(hostname));
                    }
                };
            }
        }

        Ok(Container::<cloudflare_tunnel::Ingress> {
            paths: path_from_cfg(&rule.path)?,
            hostnames,
            ingress: cloudflare_tunnel::Ingress::from_cfg(&rule.rule)?,
            upstreams,
        })
    }

    pub mod upstream {
        use anyhow::Result;

        pub type HttpUpstreamContainer = Container<Upstream>;

        #[derive(Debug, Clone)]
        pub enum Upstream {
            Http(http::Upstream),
        }

        impl HttpUpstreamContainer {
            pub fn from_cfg(
                upstream: &crate::config::cfg::rules::upstream::Container<
                    crate::config::cfg::rules::upstream::http::Upstreams,
                >,
            ) -> Result<Self> {
                Ok(Self {
                    weight: upstream.weight.unwrap_or(1),
                    upstream: match upstream.upstream {
                        crate::config::cfg::rules::upstream::http::Upstreams::Http(ref http) => {
                            Upstream::Http(http::Upstream::from_cfg(http)?)
                        }
                    },
                })
            }
        }

        #[derive(Debug, Clone)]
        pub struct Container<T> {
            pub weight: u32,
            pub upstream: T,
        }

        pub mod http {
            use anyhow::Result;

            #[derive(Debug, Clone)]
            pub struct Upstream {
                pub host: String,
                pub port: u16,
            }

            impl Upstream {
                pub fn from_cfg(
                    upstream: &crate::config::cfg::rules::upstream::http::HttpUpstream,
                ) -> Result<Self> {
                    Ok(Self {
                        host: upstream.host.clone(),
                        port: upstream.port,
                    })
                }
            }
        }
    }

    bitflags! {
        pub struct PathMethod: u16 {
            const NONE = 0;
            const GET = 1 << 0;
            const POST = 1 << 1;
            const PUT =  1 << 2;
            const DELETE = 1 << 3;
            const PATCH = 1 << 4;
            const HEAD = 1 << 5;
            const OPTIONS = 1 << 6;
            const CONNECT = 1 << 7;
            const TRACE = 1 << 8;
            const ALL = (1 << 8) - 1;
        }
    }

    #[derive(Debug, Clone)]
    pub enum PathType {
        Exact(String),
        Prefix(String),
        Regex(String),
    }

    pub fn path_from_cfg(
        path: &cfg::rules::http::PathUnion,
    ) -> Result<Vec<(PathType, PathMethod)>> {
        let paths = match path {
            cfg::rules::http::PathUnion::Single { path } => {
                vec![path.clone()]
            }
            cfg::rules::http::PathUnion::Multi { paths } => paths.clone(),
        };

        let mut result = Vec::new();

        for path in &paths {
            match path {
                cfg::rules::http::Path::String(path) => {
                    result.push((PathType::Prefix(path.clone()), PathMethod::ALL));
                }
                cfg::rules::http::Path::Struct(path) => {
                    let path_type = match &path.kind {
                        Some(cfg::rules::http::PathKind::Exact) => {
                            PathType::Exact(path.path.clone())
                        }
                        Some(cfg::rules::http::PathKind::Prefix) => {
                            PathType::Prefix(path.path.clone())
                        }
                        Some(cfg::rules::http::PathKind::Regex) => {
                            PathType::Regex(path.path.clone())
                        }
                        None => PathType::Prefix(path.path.clone()),
                    };

                    let mut path_method = PathMethod::NONE;

                    if let Some(methods) = &path.methods {
                        for method in methods {
                            match method.as_str() {
                                "GET" => path_method |= PathMethod::GET,
                                "POST" => path_method |= PathMethod::POST,
                                "PUT" => path_method |= PathMethod::PUT,
                                "DELETE" => path_method |= PathMethod::DELETE,
                                "PATCH" => path_method |= PathMethod::PATCH,
                                "HEAD" => path_method |= PathMethod::HEAD,
                                "OPTIONS" => path_method |= PathMethod::OPTIONS,
                                "CONNECT" => path_method |= PathMethod::CONNECT,
                                "TRACE" => path_method |= PathMethod::TRACE,
                                _ => {}
                            }
                        }
                    }

                    result.push((path_type, path_method));
                }
            };
        }

        Ok(result)
    }

    pub mod cloudflare_tunnel {
        use anyhow::Result;
        use uuid::Uuid;

        #[derive(Debug, Clone)]
        pub struct Ingress {
            pub tunnel_id: Uuid,
        }

        impl Ingress {
            pub fn from_cfg(
                rule: &crate::config::cfg::rules::http::cloudflare::Ingress,
            ) -> Result<Self> {
                Ok(Self {
                    tunnel_id: rule.tunnel_id,
                })
            }
        }
    }
}
