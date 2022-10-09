use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use log::{error, info};
use tokio::sync::Mutex;
use utils::context::wait::SuperContext;
use uuid::Uuid;

use self::{
    dns::resolve_edge_addr,
    edge::{EdgeTracker, IpPortHost},
    tunnel::EdgeTunnelServer,
    types::{EdgeRegionLocation, Protocol},
};

mod dns;
mod edge;
mod tls;
mod tunnel;
pub mod types;

pub struct Supervisor {
    id: Uuid,
    tracker: EdgeTracker,
    tls: Arc<Mutex<HashMap<Protocol, tls::RootCert>>>,
}

impl Supervisor {
    pub async fn new(location: &EdgeRegionLocation) -> Result<Self> {
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
            id: Uuid::new_v4(),
            tls: Arc::new(Mutex::new(tls::get_proto_edge_tls_map().await?)),
            tracker: EdgeTracker::new(ips),
        })
    }

    pub async fn start(&mut self, ctx: SuperContext) -> Result<()> {
        let tls = self.tls.lock().await.get(&Protocol::QUIC).unwrap().clone();
        let ip = self.tracker.get(&0).await?;

        let mut ctx = ctx;
        let sub_ctx = ctx.clone();

        let edge = EdgeTunnelServer::new(0);

        tokio::select! {
            _ = ctx.done() => {
                info!("Context cancelled");
            }
            _ = edge.serve(sub_ctx, Protocol::QUIC, &ip, tls) => {
                error!("Tunnel server exited");
            }
        }

        self.tracker.release(&0).await;

        Ok(())
    }
}
