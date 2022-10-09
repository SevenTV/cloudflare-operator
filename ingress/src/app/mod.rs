use std::{future::Future, time::Duration};

use crate::config;
use anyhow::{anyhow, Context, Result};
use framework::cloudflare::supervisor::{types::EdgeRegionLocation, Supervisor};
use log::info;
use tokio::{select, time};
use utils::context::wait::SuperContext;

pub async fn start(cfg: config::Config) -> Result<()> {
    let (context, handle) = SuperContext::new(None);
    let mut supervisor = Supervisor::new(&EdgeRegionLocation::AUTO).await?;

    let main = tokio::spawn(async move {
        supervisor.start(context.clone()).await.unwrap();
        time::sleep(Duration::from_secs(10)).await;
        drop(context);
    });

    select! {
        r = main => {
            info!("Supervisor exited");
            r?;
        }
        r = tokio::signal::ctrl_c() => {
            if let Err(err) = r {
                return Err(anyhow!("Failed to listen for ctrl-c: {}", err));
            }

            info!("shutting down");
        }
    }

    let shutdown = async {
        if cfg.get_shutdown_timeout().is_some() {
            time::timeout(
                Duration::from_secs(cfg.get_shutdown_timeout().unwrap()),
                async {
                    let _ = handle.cancel().await;
                },
            )
            .await
            .context(format!(
                "Shutdown timedout after {}s - force shutting down",
                cfg.get_shutdown_timeout().unwrap()
            ))?;
        } else {
            let _ = handle.cancel().await;
        }

        Ok::<(), anyhow::Error>(())
    };

    info!("waiting for supervisor to stop");

    select! {
        r = shutdown => r?,
        r = tokio::signal::ctrl_c() => {
            r.map_err(|err| anyhow!("Failed to listen for ctrl-c: {}", err))?;
            info!("force shutting down");
        }
    }

    Ok(())
}
