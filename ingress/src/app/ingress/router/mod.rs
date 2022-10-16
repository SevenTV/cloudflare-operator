use std::{collections::HashMap, time::Duration};

use anyhow::{anyhow, Result};
use tokio::{select, sync::mpsc::Receiver};
use tracing::{error, info};
use utils::context::wait::Context;
use uuid::Uuid;

use crate::config::{cfg, cfg::rules::IngressRule::CloudflareTunnel, Config};

mod manager;
mod types;

pub struct RouteController {
    cfg: Config,
    rebuild_notify: Receiver<()>,
}

#[derive(Debug, Clone, Default)]
struct RebuildConfig {
    auth_map: HashMap<(String, types::auth::Type), types::auth::Auth>,
    cloudflare_tunnels: HashMap<Uuid, types::CloudflareTunnel>,
}

impl RouteController {
    pub fn new(cfg: Config, rebuild_notify: Receiver<()>) -> Self {
        Self {
            cfg,
            rebuild_notify,
        }
    }

    fn fetch_config(path: &String) -> Option<Config> {
        crate::config::file::parse(path).ok().unwrap_or(None)
    }

    fn rebuild_auth_from_config(cfg: &Config, rebuild: &mut RebuildConfig) {
        if let Some(auth) = &cfg.auth {
            for auth in auth.iter() {
                let (t, a) = match &auth.auth {
                    cfg::auth::Auth::Cloudflare(a) => (
                        (
                            auth.name.clone().unwrap_or_else(|| "default".to_string()),
                            types::auth::Type::Cloudflare,
                        ),
                        types::auth::Auth::Cloudflare(match a {
                            cfg::auth::AuthCloudflare::ApiToken { api_token } => {
                                types::auth::Cloudflare::ApiToken {
                                    token: api_token.clone(),
                                }
                            }
                            cfg::auth::AuthCloudflare::ApiKey { api_key, email } => {
                                types::auth::Cloudflare::ApiKey {
                                    key: api_key.clone(),
                                    email: email.clone(),
                                }
                            }
                        }),
                    ),
                };
                let name = t.0.clone();
                if let std::collections::hash_map::Entry::Vacant(e) = rebuild.auth_map.entry(t) {
                    e.insert(a);
                } else {
                    error!("duplicate auth name: {}", name);
                }
            }
        }
    }

    fn rebuild_cloudflare_tunnels_from_config(cfg: &Config, rebuild: &mut RebuildConfig) {
        if let Some(tunnels) = &cfg.cloudflare_tunnels {
            for tunnel in tunnels {
                let auth = rebuild.auth_map.get(&(
                    tunnel.auth.clone().unwrap_or_else(|| "default".to_string()),
                    types::auth::Type::Cloudflare,
                ));
                if let Some(auth) = auth {
                    if let types::auth::Auth::Cloudflare(auth) = auth {
                        rebuild.cloudflare_tunnels.insert(
                            tunnel.tunnel_id,
                            types::CloudflareTunnel {
                                account_id: tunnel.account_id.clone(),
                                id: tunnel.tunnel_id,
                                auth: auth.clone(),
                                ingress: vec![],
                            },
                        );
                    } else {
                        panic!("auth type mismatch");
                    }
                }
            }
        }
    }

    fn rebuild_ingress_from_config(cfg: &Config, rebuild: &mut RebuildConfig) {
        if let Some(rules) = &cfg.rules {
            for rule in rules {
                let add_result = || -> Result<()> {
                    match rule {
                        CloudflareTunnel(rule) => {
                            if let Some(entry) =
                                rebuild.cloudflare_tunnels.get_mut(&rule.rule.tunnel_id)
                            {
                                let tunnel = types::http::cloudflare_tunnel_ingress_from_cfg(rule)?;
                                entry.ingress.push(tunnel);
                                return Ok(());
                            }

                            Err(anyhow!("tunnel id not found: {}", rule.rule.tunnel_id))
                        }
                    }
                }();

                if let Err(e) = add_result {
                    error!("failed to add rule: {}", e);
                }
            }
        }
    }

    pub async fn serve(self, ctx: Context) -> Result<()> {
        let (debounce_sender, mut debounce_reciever) = tokio::sync::mpsc::channel(1);

        let _ = debounce_sender.try_send(()); // trigger initial load

        let debouncer = {
            let mut ctx = ctx.clone();
            let recv = self.rebuild_notify;

            tokio::spawn(async move {
                select! {
                    _ = ctx.done() => {
                        Ok(())
                    }
                    _ = utils::debounce_notify(debounce_sender, recv, Duration::from_secs(3)) => {
                        Err(anyhow!("debounce exited early"))
                    }
                }
            })
        };

        let updater = {
            let mut ctx = ctx.clone();
            let mut cfg = self.cfg.clone();

            tokio::spawn(async move {
                let mut manager = manager::Manager::new();
                manager.graceful(ctx.clone()).await;

                loop {
                    select! {
                        r = debounce_reciever.recv() => {
                            if r.is_none() {
                                return Err(anyhow!("channel closed unexpectedly"));
                            }

                            info!("rebuilding ingress");

                            cfg = {
                                if let Some(path) = cfg.config_file.clone() {
                                    Self::fetch_config(&path).unwrap_or(cfg)
                                } else {
                                    cfg
                                }
                            };

                            let mut rebuild = RebuildConfig::default();

                            // first we must get all the auths
                            Self::rebuild_auth_from_config(&cfg, &mut rebuild);

                            // then we can get all the tunnels
                            Self::rebuild_cloudflare_tunnels_from_config(&cfg, &mut rebuild);

                            // then we can get all the ingress
                            Self::rebuild_ingress_from_config(&cfg, &mut rebuild);

                            // remove tunnels with no ingress rules
                            rebuild.cloudflare_tunnels.retain(|_, v| !v.ingress.is_empty());

                            // We now have a full rebuild config, we can now reconsolidate the ingress
                            manager.update(rebuild).await;

                            info!("rebuild complete");
                        }
                        _ = ctx.done() => {
                            break;
                        }
                    }
                }

                Ok(())
            })
        };

        select! {
            r = debouncer => {r??;},
            r = updater => {r??;},
        };

        Ok(())
    }
}
