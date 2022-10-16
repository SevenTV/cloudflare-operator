use std::time::Duration;

use crate::config;
use anyhow::Result;
use anyhow::{anyhow, Context};

use tokio::{select, time};
use tracing::info;
use utils::context::wait::Handle;

mod ingress;

pub async fn start(cfg: config::Config) -> Result<()> {
    let mut handle = Handle::new();

    let ingress = ingress::IngressController::new();

    let mut future = {
        let ctx = handle.spawn();
        let cfg = cfg.clone();

        tokio::spawn(async move { ingress.run(ctx, cfg).await })
    };

    select! {
        r = &mut future => {
            r??;

            return Err(anyhow!("Ingress controller exited unexpectedly"));
        }
        r = tokio::signal::ctrl_c() => {
            if let Err(err) = r {
                return Err(anyhow!("Failed to listen for ctrl-c: {}", err));
            }

            info!("shutting down");
        }
    }

    let shutdown = async {
        if let Some(timeout) = cfg.shutdown_timeout {
            time::timeout(Duration::from_secs(timeout), async {
                handle.cancel().await;
            })
            .await
            .context(format!(
                "Shutdown timedout after {}s - force shutting down",
                timeout,
            ))?;
        } else {
            handle.cancel().await;
        }

        Ok::<(), anyhow::Error>(())
    };

    select! {
        r = shutdown => r?,
        r = tokio::signal::ctrl_c() => {
            r.map_err(|err| anyhow!("Failed to listen for ctrl-c: {}", err))?;
            info!("force shutting down");
        }
    }

    info!("controller stopped");

    Ok(())
}
