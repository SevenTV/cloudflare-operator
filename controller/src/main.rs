use std::process::exit;

mod app;
mod config;
mod helpers;
mod setup;

use log::{error, info};

#[tokio::main]
async fn main() {
    let config = setup::setup();

    info!("Starting cloudflared-ingress");

    let result = app::start(config).await;
    if let Err(err) = result {
        error!("Failed to start: {:#}", err);
        exit(1);
    }

    info!("cloudflared-ingress stopped");
}
