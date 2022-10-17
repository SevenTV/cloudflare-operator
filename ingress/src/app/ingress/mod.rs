use std::path::Path;

use anyhow::{anyhow, Ok, Result};
use notify::Watcher;
use tokio::{join, select, sync::mpsc::Sender};
use tracing::error;
use tuple_conv::RepeatedTuple;
use utils::context::wait::{Context, Handle};

use crate::config::Config;

mod router;

pub struct IngressController {}

impl IngressController {
    pub fn new() -> Self {
        Self {}
    }

    fn watch_config_reload(
        ctx: Context,
        tx: Sender<()>,
        path: String,
    ) -> Result<impl std::future::Future<Output = Result<()>>> {
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Err(err) = res {
                error!("Failed to watch config file: {:#?}", err);
            } else if let Err(err) = tx.try_send(()) {
                error!("Failed to send config reload event: {:#}", err);
            }
        })?;

        watcher.watch(Path::new(&path), notify::RecursiveMode::NonRecursive)?;

        let mut ctx = ctx;

        Ok(async move {
            ctx.done().await;

            watcher.unwatch(Path::new(&path))?;

            Ok(())
        })
    }

    async fn watch_k8s(_ctx: Context, _tx: Sender<()>, _cfg: Config) -> Result<()> {
        panic!("unimplemented");
    }

    pub async fn run(self, ctx: Context, cfg: Config) -> Result<()> {
        let mut handle = Handle::new_from_parent(&ctx, None);
        let (drop_sender, mut drop_receiver) = tokio::sync::mpsc::channel(1);
        let (rebuild_ingress_sender, rebuild_ingress_receiver) = tokio::sync::mpsc::channel(1);

        let watch_config = {
            let pth = cfg.config_file.clone();
            let mut ctx = handle.spawn();
            let tx = rebuild_ingress_sender.clone();

            tokio::spawn(utils::detect_finish(drop_sender.clone(), async move {
                if let Some(pth) = pth {
                    IngressController::watch_config_reload(ctx, tx, pth)?.await?;
                } else {
                    ctx.done().await;
                }

                Ok(())
            }))
        };

        let watch_k8s = {
            let cfg = cfg.clone();
            let mut ctx = handle.spawn();
            let tx = rebuild_ingress_sender.clone();

            tokio::spawn(utils::detect_finish(drop_sender.clone(), async move {
                if cfg.kubernetes.enabled.unwrap_or_default() {
                    IngressController::watch_k8s(ctx, tx, cfg).await?;
                } else {
                    ctx.done().await;
                }

                Ok(())
            }))
        };

        let route_controller = {
            let ctx = handle.spawn();
            let cfg = cfg.clone();

            tokio::spawn(utils::detect_finish(drop_sender.clone(), async move {
                router::RouteController::new(cfg, rebuild_ingress_receiver)
                    .serve(ctx)
                    .await
            }))
        };

        // Wait for one of the contexts to be dropped...
        {
            let mut ctx = ctx;
            select! {
                _ = drop_receiver.recv() => {
                    error!("ingress controller dropped a context, unexpectedly");
                    handle.cancel().await;
                }
                _ = ctx.done() => {
                    handle.cancel().await;
                }
            }
        }

        utils::handle_errors(join!(watch_config, watch_k8s, route_controller).to_vec())
            .map_err(|err| anyhow!("ingress controller failed: {:?}", err))?;

        Ok(())
    }
}
