use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{select, sync::Mutex, task::JoinHandle};
use tracing::{error, info};
use utils::{common::handle_errors, context::wait::Context};
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
    tls::RootCert,
    tunnel::EdgeTunnelClient,
    types::{EdgeRegionLocation, Protocol, TunnelAuth},
};

use super::types::HandleHttp;

pub struct Supervisor {
    id: Uuid,
    tracker: EdgeTracker,
    tls: Arc<Mutex<HashMap<Protocol, tls::RootCert>>>,
    auth: TunnelAuth,
    handle: HandleHttp,
}

impl Supervisor {
    pub async fn new(
        id: Uuid,
        location: &EdgeRegionLocation,
        auth: TunnelAuth,
        handle: HandleHttp,
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

    async fn start_helper(&self, ctx: Context, idx: u32, tls: RootCert) -> JoinHandle<Result<()>> {
        let mut ctx = ctx;
        let id = self.id;
        let mut tracker = self.tracker.clone();
        let auth = self.auth.clone();
        let handle = self.handle.clone();
        let protocol = Protocol::Quic;

        tokio::spawn(async move {
            let ip = tracker.get(&idx).await?;

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

            tracker.release(&idx).await;

            Ok(())
        })
    }

    pub async fn start(self, ctx: Context) -> Result<()> {
        let tls = self.tls.lock().await.get(&Protocol::Quic).unwrap().clone(); // todo

        let mut handles = Vec::new();

        for idx in 0..4 {
            handles.push(self.start_helper(ctx.clone(), idx, tls.clone()).await);
        }

        let handles = {
            let mut h = Vec::new();
            for handle in handles {
                h.push(handle.await);
            }

            h
        };

        handle_errors(handles)?;

        Ok(())
    }
}
