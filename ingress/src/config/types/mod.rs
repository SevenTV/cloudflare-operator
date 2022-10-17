use serde::Deserialize;

pub mod auth;
pub mod cloudflare;
pub mod ingress;
pub mod k8s;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    // All arguments relating to kubernetes
    #[serde(default)]
    pub kubernetes: k8s::ConfigKubernetes,

    // Debug mode
    pub log_level: Option<String>,

    // Graceful shutdown timeout
    pub shutdown_timeout: Option<u64>,

    // All ingress rules if ingress_mode is set to "standalone"
    pub rules: Option<Vec<ingress::IngressRule>>,

    // All auth credentials if ingress_mode is set to "standalone"
    pub auth: Option<Vec<auth::AuthContainer>>,

    pub cloudflare_tunnels: Option<Vec<cloudflare::Tunnel>>,

    #[serde(skip)]
    pub config_file: Option<String>,
}

impl Config {
    pub fn merge(self, other: Config) -> Config {
        Config {
            cloudflare_tunnels: other.cloudflare_tunnels.or(self.cloudflare_tunnels),
            kubernetes: other.kubernetes.merge(self.kubernetes),
            log_level: other.log_level.or(self.log_level),
            shutdown_timeout: other.shutdown_timeout.or(self.shutdown_timeout),
            rules: other.rules.or(self.rules),
            auth: other.auth.or(self.auth),
            config_file: other.config_file.or_else(|| self.config_file.clone()),
        }
    }
}
