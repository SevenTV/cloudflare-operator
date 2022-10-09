#![allow(dead_code)] // todo remove

use serde::Deserialize;

pub mod args;
pub mod env;
pub mod file;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    // All arguments relating to kubernetes
    #[serde(default)]
    pub kubernetes: ConfigKubernetes,

    // All arguments relating to cloudflare
    #[serde(default)]
    pub cloudflare: ConfigCloudflare,

    // Debug mode
    debug: Option<bool>,

    // Config file path
    config_file: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigKubernetes {
    // The path to the kubeconfig file
    kubeconfig: Option<String>,

    // The namespace of this controller
    namespace: Option<String>,

    // The pod name of this controller
    pod_name: Option<String>,

    // The lease name to use for leader election
    lock_name: Option<String>,

    // The ingress class name to use for ingress selection
    ingress_class: Option<String>,

    // Watch ingresses without an ingress class
    watch_ingresses_without_class: Option<bool>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigCloudflare {
    // The Account ID of the Cloudflare account
    account_id: Option<String>,

    // The Tunnel ID of the Cloudflared tunnel
    tunnel_id: Option<String>,

    // The API token to use for Cloudflare
    api_token: Option<String>,
}

impl Config {
    pub fn get_config_file(&self) -> Option<String> {
        self.config_file.clone()
    }

    pub fn get_debug(&self) -> bool {
        self.debug.unwrap_or(false)
    }

    pub fn merge(&self, other: &mut Config) {
        *other = Config {
            cloudflare: other.cloudflare.merge(&self.cloudflare),
            kubernetes: other.kubernetes.merge(&self.kubernetes),
            debug: other.debug.or(self.debug),
            config_file: other
                .config_file
                .clone()
                .or_else(|| self.config_file.clone()),
        };
    }
}

impl ConfigKubernetes {
    pub fn get_kubeconfig(&self) -> String {
        self.kubeconfig.clone().unwrap_or_else(|| "".to_string())
    }

    pub fn get_namespace(&self) -> String {
        self.namespace
            .clone()
            .unwrap_or_else(|| "default".to_string())
    }

    pub fn get_pod_name(&self) -> String {
        self.pod_name.clone().unwrap_or_else(|| "".to_string())
    }

    pub fn get_lock_name(&self) -> String {
        self.lock_name.clone().unwrap_or_else(|| "".to_string())
    }

    pub fn get_ingress_class(&self) -> String {
        self.ingress_class
            .clone()
            .unwrap_or_else(|| "cloudflared".to_string())
    }

    pub fn get_watch_ingresses_without_class(&self) -> bool {
        self.watch_ingresses_without_class.unwrap_or(false)
    }

    fn merge(&self, other: &ConfigKubernetes) -> Self {
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

impl ConfigCloudflare {
    pub fn get_account_id(&self) -> String {
        self.account_id.clone().unwrap_or_else(|| "".to_string())
    }

    pub fn get_tunnel_id(&self) -> String {
        self.tunnel_id.clone().unwrap_or_else(|| "".to_string())
    }

    pub fn get_api_token(&self) -> String {
        self.api_token.clone().unwrap_or_else(|| "".to_string())
    }

    fn merge(&self, other: &ConfigCloudflare) -> Self {
        ConfigCloudflare {
            account_id: other
                .account_id
                .to_owned()
                .or_else(|| self.account_id.to_owned()),
            tunnel_id: other
                .tunnel_id
                .to_owned()
                .or_else(|| self.tunnel_id.to_owned()),
            api_token: other
                .api_token
                .to_owned()
                .or_else(|| self.api_token.to_owned()),
        }
    }
}
