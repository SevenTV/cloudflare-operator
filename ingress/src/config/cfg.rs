use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub mode: Option<Mode>,

    // All arguments relating to kubernetes
    #[serde(default)]
    pub kubernetes: ConfigKubernetes,

    // Debug mode
    pub log_level: Option<String>,

    // Graceful shutdown timeout
    pub shutdown_timeout: Option<u64>,

    // All ingress rules if ingress_mode is set to "standalone"
    pub rules: Option<Vec<rules::IngressRule>>,

    // All auth credentials if ingress_mode is set to "standalone"
    pub auth: Option<Vec<auth::AuthContainer>>,

    pub cloudflare_tunnels: Option<Vec<cloudflare_tunnels::Tunnel>>,

    #[serde(skip)]
    pub config_file: Option<String>,
}

pub mod cloudflare_tunnels {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Tunnel {
        pub account_id: String,
        pub tunnel_id: uuid::Uuid,
        pub auth: Option<String>,
    }
}

pub mod auth {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct AuthContainer {
        pub name: Option<String>,

        #[serde(flatten)]
        pub auth: Auth,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
    pub enum Auth {
        Cloudflare(AuthCloudflare),
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum AuthCloudflare {
        ApiKey { api_key: String, email: String },
        ApiToken { api_token: String },
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigKubernetes {
    // The path to the kubeconfig file
    pub kubeconfig: Option<String>,

    // The namespace of this controller
    pub namespace: Option<String>,

    // The pod name of this controller
    pub pod_name: Option<String>,

    // The lease name to use for leader election
    pub lock_name: Option<String>,

    // The ingress class name to use for ingress selection
    pub ingress_class: Option<String>,

    // Watch ingresses without an ingress class
    pub watch_ingresses_without_class: Option<bool>,
}

pub mod rules {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
    pub enum IngressRule {
        CloudflareTunnel(http::Container<http::cloudflare::Ingress>),
    }

    pub mod http {
        use serde::Deserialize;

        use super::upstream;

        #[derive(Debug, Deserialize, Clone)]
        pub struct Container<R> {
            #[serde(flatten)]
            pub rule: R,

            #[serde(flatten)]
            pub hostname: Option<HostnameUnion>,

            #[serde(flatten)]
            pub path: PathUnion,

            pub upstreams: Option<Vec<upstream::Container<upstream::http::Upstreams>>>,
        }

        #[derive(Debug, Deserialize, Clone)]
        #[serde(untagged)]
        pub enum HostnameUnion {
            Single { hostname: Hostname },
            Multi { hostnames: Vec<Hostname> },
        }

        #[derive(Debug, Deserialize, Clone)]
        #[serde(untagged)]
        pub enum Hostname {
            // todo support regex and wildcard (glob) hostnames
            String(String),
        }

        #[derive(Debug, Deserialize, Clone)]
        #[serde(untagged)]
        pub enum PathUnion {
            Single { path: Path },
            Multi { paths: Vec<Path> },
        }

        #[derive(Debug, Deserialize, Clone)]
        #[serde(untagged)]
        pub enum Path {
            String(String),
            Struct(PathStruct),
        }

        #[derive(Debug, Deserialize, Clone)]
        pub struct PathStruct {
            pub kind: Option<PathKind>,
            pub path: String,
            pub methods: Option<Vec<String>>,
        }

        #[derive(Debug, Deserialize, Clone)]
        pub enum PathKind {
            Exact,
            Prefix,
            Regex,
        }

        pub mod cloudflare {
            use serde::Deserialize;

            #[derive(Debug, Deserialize, Default, Clone)]
            pub struct Ingress {
                pub tunnel_id: uuid::Uuid,
            }
        }
    }

    pub mod upstream {
        use serde::Deserialize;

        #[derive(Debug, Deserialize, Clone)]
        pub struct Container<T> {
            pub weight: Option<u32>,

            #[serde(flatten)]
            pub upstream: T,
        }

        pub mod http {
            use serde::Deserialize;

            #[derive(Debug, Deserialize, Clone)]
            #[serde(tag = "kind", rename_all = "kebab-case")]
            pub enum Upstreams {
                #[serde(rename = "http")]
                Http(HttpUpstream),
            }

            #[derive(Debug, Deserialize, Default, Clone)]
            pub struct HttpUpstream {
                pub host: String,
                pub port: u16,
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Mode {
    #[serde(rename = "k8s")]
    K8S,
    #[serde(rename = "standalone")]
    Standalone,
}

impl Config {
    pub fn merge(self, other: Config) -> Config {
        Config {
            cloudflare_tunnels: other.cloudflare_tunnels.or(self.cloudflare_tunnels),
            kubernetes: other.kubernetes.merge(self.kubernetes),
            mode: other.mode.or(self.mode),
            log_level: other.log_level.or(self.log_level),
            shutdown_timeout: other.shutdown_timeout.or(self.shutdown_timeout),
            rules: other.rules.or(self.rules),
            auth: other.auth.or(self.auth),
            config_file: other.config_file.or_else(|| self.config_file.clone()),
        }
    }
}

impl ConfigKubernetes {
    fn merge(self, other: ConfigKubernetes) -> Self {
        ConfigKubernetes {
            kubeconfig: other
                .kubeconfig
                .to_owned()
                .or_else(|| self.kubeconfig.to_owned()),
            namespace: other
                .namespace
                .to_owned()
                .or_else(|| self.namespace.to_owned()),
            pod_name: other
                .pod_name
                .to_owned()
                .or_else(|| self.pod_name.to_owned()),
            lock_name: other
                .lock_name
                .to_owned()
                .or_else(|| self.lock_name.to_owned()),
            ingress_class: other
                .ingress_class
                .to_owned()
                .or_else(|| self.ingress_class.to_owned()),
            watch_ingresses_without_class: other
                .watch_ingresses_without_class
                .or_else(|| self.watch_ingresses_without_class.to_owned()),
        }
    }
}
