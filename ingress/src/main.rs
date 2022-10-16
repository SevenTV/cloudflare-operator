use std::process::exit;

mod app;
mod config;
mod helpers;
mod setup;

use tracing::{error, info};

#[tokio::main]
async fn main() {
    let config = match setup::setup() {
        Ok(config) => config,
        Err(err) => {
            error!("failed to setup: {:#}", err);
            exit(1);
        }
    };

    info!("Starting cloudflared-ingress");

    let result = app::start(config).await;
    if let Err(err) = result {
        error!("app failed with: {:#}", err);
        exit(1);
    }

    info!("cloudflared-ingress stopped");
}
