use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Result;
use futures::select;
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

pub use super::rpc::types::TunnelAuth;
use tokio_util::task::LocalPoolHandle;

mod dns;
mod edge;
mod tls;
mod tunnel;
pub mod types;

pub struct Supervisor {
    id: Uuid,
    tracker: EdgeTracker,
    tls: Arc<Mutex<HashMap<Protocol, tls::RootCert>>>,
    auth: TunnelAuth,
}

impl Supervisor {
    pub async fn new(location: &EdgeRegionLocation, auth: TunnelAuth) -> Result<Self> {
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
            auth,
            tls: Arc::new(Mutex::new(tls::get_proto_edge_tls_map().await?)),
            tracker: EdgeTracker::new(ips),
        })
    }

    pub async fn start(mut self, ctx: SuperContext) -> Result<()> {
        let tls = self.tls.lock().await.get(&Protocol::QUIC).unwrap().clone(); // todo
        let ip = self.tracker.get(&0).await?;

        // this is important because we should be able to cancel the tunnel server if we want to.
        // this select statement allows us to continue if the ctx gets cancelled.
        // which will cancel the tunnel server.

        let fut = {
            let server = EdgeTunnelServer::new(0, self.auth.clone());
            info!("Starting tunnel server");
            server.serve(ctx.clone(), Protocol::QUIC, ip.clone(), tls.clone())
        };

        fut.await?;

        self.tracker.release(&0).await;

        Ok(())
    }
}
