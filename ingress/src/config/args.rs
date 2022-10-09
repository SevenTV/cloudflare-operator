use clap::{Arg, ArgAction, Command};

use super::{Config, ConfigCloudflare, ConfigKubernetes};

macro_rules! bool_optional {
    ($count:expr) => {
        if ($count) {
            Some(true)
        } else {
            None
        }
    };
}

pub fn parse() -> Config {
    let matches = Command::new("cloudflared-ingress")
        .version("1.0")
        .author("Troy Benson <troy@7tv.app>")
        .about("Manages Cloudflare tunnels for Kubernetes Ingress")
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Debug mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("kubernetes.kubeconfig")
                .long("k8s.kubeconfig")
                .help("Kubernetes config file"),
        )
        .arg(
            Arg::new("kubernetes.namespace")
                .long("k8s.namespace")
                .help("Kubernetes namespace"),
        )
        .arg(
            Arg::new("kubernetes.pod_name")
                .long("k8s.pod-name")
                .help("Kubernetes pod name"),
        )
        .arg(
            Arg::new("kubernetes.lock_name")
                .long("k8s.lock-name")
                .help("Kubernetes lock name"),
        )
        .arg(
            Arg::new("kubernetes.ingress_class")
                .long("k8s.ingress-class")
                .help("Kubernetes ingress class"),
        )
        .arg(
            Arg::new("kubernetes.watch_ingresses_without_class")
                .long("k8s.watch-ingress-without-class")
                .help("Watch ingresses without an ingress class")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("cloudflare.account_id")
                .long("cf.account-id")
                .help("Cloudflare account ID"),
        )
        .arg(
            Arg::new("cloudflare.tunnel_id")
                .long("cf.tunnel-id")
                .help("Cloudflare tunnel ID"),
        )
        .arg(
            Arg::new("cloudflare.api_token")
                .long("cf.api-token")
                .help("Cloudflare API token"),
        )
        .arg(
            Arg::new("config_file")
                .long("config")
                .help("Config file path"),
        )
        .arg(
            Arg::new("shutdown_timeout")
                .long("shutdown-timeout")
                .help("Graceful shutdown timeout"),
        )
        .get_matches();

    return Config {
        debug: bool_optional!(matches.get_one::<bool>("debug").unwrap().to_owned()),
        shutdown_timeout: matches.get_one::<u64>("shutdown_timeout").cloned(),
        kubernetes: ConfigKubernetes {
            kubeconfig: matches.get_one::<String>("kubernetes.kubeconfig").cloned(),
            namespace: matches.get_one::<String>("kubernetes.namespace").cloned(),
            pod_name: matches.get_one::<String>("kubernetes.pod_name").cloned(),
            lock_name: matches.get_one::<String>("kubernetes.lock_name").cloned(),
            ingress_class: matches
                .get_one::<String>("kubernetes.ingress_class")
                .cloned(),
            watch_ingresses_without_class: bool_optional!(matches
                .get_one::<bool>("kubernetes.watch_ingresses_without_class")
                .unwrap()
                .to_owned()),
        },
        cloudflare: ConfigCloudflare {
            account_id: matches.get_one::<String>("cloudflare.account_id").cloned(),
            tunnel_id: matches.get_one::<String>("cloudflare.tunnel_id").cloned(),
            api_token: matches.get_one::<String>("cloudflare.api_token").cloned(),
        },
        config_file: matches.get_one::<String>("config_file").cloned(),
    };
}
