use std::time::Duration;

use crate::config;
use anyhow::Result;
use anyhow::{anyhow, Context};
use framework::cloudflare::{
    self,
    api::Auth,
    supervisor::{types::EdgeRegionLocation, Supervisor, TunnelAuth},
};
use log::info;
use tokio::{select, time};
use utils::context::wait::SuperContext;

pub async fn start(cfg: config::Config) -> Result<()> {
    let cf_api = cloudflare::api::Client::new(
        cfg.cloudflare.get_account_id(),
        Auth::ApiToken(cfg.cloudflare.get_api_token()),
    );

    let token = cf_api
        .get_tunnel_token(cfg.cloudflare.get_tunnel_id().as_str())
        .await?;

    let tkn = TunnelAuth::new(token.as_str())?;

    let (context, handle) = SuperContext::new(None);
    
    let supervisor = Supervisor::new(&EdgeRegionLocation::AUTO, tkn).await?;

    info!("Starting supervisor");

    let supervisor_handle = tokio::spawn(async move {
        supervisor.start(context).await
    });


    select! {
        r = supervisor_handle => {
            info!("Supervisor exited");
            r??; // this is a double question mark, it unwraps the result from the promise and then unwraps the result from the return.
        }
        r = tokio::signal::ctrl_c() => {
            if let Err(err) = r {
                return Err(anyhow!("Failed to listen for ctrl-c: {}", err));
            }

            info!("shutting down");
        }
    }

    let shutdown = async {
        if let Some(timeout) = cfg.get_shutdown_timeout() {
            time::timeout(Duration::from_secs(timeout), async {
                let _ = handle.cancel().await;
            })
            .await
            .context(format!(
                "Shutdown timedout after {}s - force shutting down",
                timeout,
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

    info!("supervisor stopped");

    Ok(())
}
