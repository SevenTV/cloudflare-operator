use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use log::info;
use tokio::sync::Mutex;
use utils::context::wait::SuperContext;
use uuid::Uuid;

use crate::cloudflare::supervisor::tunnel::EdgeTunnelClient;

use self::{
    dns::resolve_edge_addr,
    edge::{EdgeTracker, IpPortHost},
    types::{EdgeRegionLocation, Protocol},
};

pub use super::rpc::types::TunnelAuth;

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

        let mut handles = Vec::new();

        for i in 0..5 {
            let ip = self.tracker.get(&i).await?;
            let ctx = ctx.clone();
            let auth = self.auth.clone();
            let tls = tls.clone();

            handles.push(tokio::spawn(async move {
                Self::start_helper(ctx, Protocol::QUIC, i, ip, tls, auth).await
            }));
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }

    async fn start_helper(
        ctx: SuperContext,
        protocol: Protocol,
        id: u32,
        ip: IpPortHost,
        tls: tls::RootCert,
        auth: TunnelAuth,
    ) -> Result<()> {
        let server = EdgeTunnelClient::new(id, auth);
        info!("Starting tunnel server {}", id);

        server
            .serve(ctx.clone(), protocol, ip.clone(), tls.clone())
            .await
    }
}
