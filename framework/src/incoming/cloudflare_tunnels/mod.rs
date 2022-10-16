use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{select, sync::Mutex};
use tracing::{error, info};
use utils::context::wait::Context;
use uuid::Uuid;

mod dns;
mod edge;
mod rpc;
mod tls;
mod tunnel;

pub mod types;

use self::{
    dns::resolve_edge_addr,
    edge::{EdgeTracker, IpPortHost},
    types::{EdgeRegionLocation, Protocol, TunnelAuth},
};

use self::tunnel::EdgeTunnelClient;

pub struct Supervisor {
    id: Uuid,
    tracker: EdgeTracker,
    tls: Arc<Mutex<HashMap<Protocol, tls::RootCert>>>,
    auth: TunnelAuth,
    handle: types::HandleHttp,
}

impl Supervisor {
    pub async fn new(
        id: Uuid,
        location: &EdgeRegionLocation,
        auth: TunnelAuth,
        handle: types::HandleHttp,
    ) -> Result<Self> {
        let edges = resolve_edge_addr(location).await?;
        let mut ips = Vec::new();

        for edge in edges {
            for ip in edge.addrs {
                ips.push(IpPortHost {
                    ip,
                    hostname: edge.hostname.clone(),
                    port: edge.port,
                    version: match ip.is_ipv6() {
                        false => edge::IpVersion::Ipv4,
                        true => edge::IpVersion::Ipv6,
                    },
                });
            }
        }

        Ok(Supervisor {
            id,
            auth,
            handle,
            tls: Arc::new(Mutex::new(tls::get_proto_edge_tls_map().await?)),
            tracker: EdgeTracker::new(ips),
        })
    }

    pub async fn start(mut self, ctx: Context) -> Result<()> {
        let tls = self.tls.lock().await.get(&Protocol::Quic).unwrap().clone(); // todo

        let mut handles = Vec::new();

        for i in 0..4 {
            let ip = self.tracker.get(&i).await?;
            let ctx = ctx.clone();
            let auth = self.auth.clone();
            let tls = tls.clone();
            let id = self.id;
            let handle = self.handle.clone();

            handles.push(tokio::spawn(async move {
                Self::start_helper(ctx, Protocol::Quic, id, i, ip, tls, auth, handle).await
                // todo handle errors here to get new IP ect...
            }));
        }

        let handles = {
            let mut h = Vec::new();
            for handle in handles {
                h.push(handle.await);
            }

            h
        };

        utils::handle_errors(handles)?;

        Ok(())
    }

    async fn start_helper(
        ctx: Context,
        protocol: Protocol,
        id: Uuid,
        idx: u32,
        ip: IpPortHost,
        tls: tls::RootCert,
        auth: TunnelAuth,
        handle: types::HandleHttp,
    ) -> Result<()> {
        let mut ctx = ctx;
        loop {
            select! {
                _ = ctx.done() => {
                    info!("shutting down tunnel {}", id);
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    let server = EdgeTunnelClient::new(id, idx, auth.clone(), handle.clone());
                    info!("Starting tunnel server {}", id);

                    let result = server
                        .serve(ctx.clone(), protocol.clone(), ip.clone(), tls.clone())
                        .await;
                    if let Err(e) = result {
                        error!("tunnel server {} failed: {}", id, e);
                    } else {
                        info!("tunnel server {} exited", id);
                    }
                }
            }
        }

        Ok(())
    }
}
