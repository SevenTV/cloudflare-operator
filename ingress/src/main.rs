use std::process::exit;

mod app;
mod config;
mod helpers;
mod setup;

use log::{error, info};

#[tokio::main]
async fn main() {
    console_subscriber::init();

    let config = setup::setup();

    info!("Starting cloudflared-ingress");

    let result = app::start(config).await;
    if let Err(err) = result {
        error!("{:#}", err);
        exit(1);
    }

    info!("cloudflared-ingress stopped");
}
