use crate::config;
use anyhow::Result;
use framework::cloudflare::supervisor::{types::EdgeRegionLocation, Supervisor};
use log::info;
use tokio::time;
use utils::context::wait::SuperContext;

pub async fn start(cfg: config::Config) -> Result<()> {
    let (context, handle) = SuperContext::new(None);
    let mut supervisor = Supervisor::new(&EdgeRegionLocation::AUTO).await?;

    let main = tokio::spawn(async move {
        supervisor.start(context).await.unwrap();

        info!("free");
    });

    time::sleep(time::Duration::from_secs(2)).await;
    let timeout = handle.cancel();
    info!("cancelled context");

    time::sleep(time::Duration::from_secs(2)).await;

    let _ = timeout.await;

    main.await?;

    Ok(())
}
