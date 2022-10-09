use crate::config;
use anyhow::Result;
use framework::{api::cloudflare as cloudflare_api, rpc::cloudflare as cloudflare_rpc};
use log::info;
use tokio_context::context::Context;

pub async fn start(cfg: config::Config) -> Result<()> {
    // let client = cloudflare::Client::new(
    //     cfg.cloudflare.get_account_id(),
    //     cloudflare::Auth::ApiToken(cfg.cloudflare.get_api_token()),
    // );

    // let token = client
    //     .get_tunnel_token(&cfg.cloudflare.get_tunnel_id())
    //     .await
    //     .context("failed to fetch token")?;

    // let token = framework::types::cloudflare::token_from_str(token.as_str())
    //     .context("failed to parse token")?;

    // println!("{:#?}", token);

    // let client_id = uuid::Uuid::new_v4();

    // let client_info = framework::rpc::cloudflare::ClientInfo {
    //     client_id,
    //     features: framework::rpc::cloudflare::TunnelFeatures::DEFAULT,
    //     version: "0.1.0".to_string(),
    //     arch: "x86_64".to_string(),
    // };

    // println!(
    //     "{:#?}",
    //     cloudflare_rpc::dns::resolve_edge_addr(cloudflare_rpc::dns::EdgeRegionLocation::AUTO)
    //         .await?
    // );

    // info!("ipv4: {} ipv6: {}", utils::ip::support_ipv4().await, utils::ip::support_ipv6().await);

    let (contxt, handle) = Context::new();

    let mut supervisor = cloudflare_rpc::supervisor::Supervisor::new(
        &cloudflare_rpc::types::EdgeRegionLocation::AUTO,
    )
    .await?;

    supervisor.start(&contxt).await
}
