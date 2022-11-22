use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigKubernetes {
    pub enabled: Option<bool>,

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

impl ConfigKubernetes {
    pub fn merge(self, other: ConfigKubernetes) -> Self {
        ConfigKubernetes {
            enabled: other.enabled.or(self.enabled),
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
