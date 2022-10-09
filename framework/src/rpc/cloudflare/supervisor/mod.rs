use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use tokio::sync::Mutex;
use tokio_context::context::Context;
use uuid::Uuid;

use self::{
    edge::{EdgeTracker, IpPortHost},
    tunnel::EdgeTunnelServer,
};

use super::{
    dns::resolve_edge_addr,
    types::{EdgeRegion, EdgeRegionLocation},
    Protocol,
};

mod edge;
mod tls;
mod tunnel;

pub struct Supervisor {
    id: Uuid,
    tracker: EdgeTracker,
    tls: Arc<Mutex<HashMap<Protocol, tls::RootCert>>>,
}

impl Supervisor {
    pub async fn new(location: &EdgeRegionLocation) -> Result<Self> {
        Ok(Supervisor {
            id: Uuid::new_v4(),
            tls: Arc::new(Mutex::new(tls::get_proto_edge_tls_map().await?)),
            tracker: EdgeTracker::new(
                resolve_edge_addr(location)
                    .await?
                    .iter()
                    .map(|region| {
                        region
                            .addrs
                            .iter()
                            .map(|ip| IpPortHost {
                                ip: ip.to_owned(),
                                port: region.port.clone(),
                                version: match ip.is_ipv6() {
                                    true => edge::IpVersion::Ipv6,
                                    false => edge::IpVersion::Ipv4,
                                },
                                hostname: region.hostname.clone(),
                            })
                            .collect::<Vec<IpPortHost>>()
                    })
                    .collect::<Vec<Vec<IpPortHost>>>()
                    .iter()
                    .flatten()
                    .map(|v| v.to_owned())
                    .collect::<Vec<IpPortHost>>(),
            ),
        })
    }

    pub async fn start(&mut self, ctx: &Context) -> Result<()> {
        let tls = self.tls.lock().await.get(&Protocol::QUIC).unwrap().clone();
        let ip = self.tracker.get(&0).await?;
        let result = EdgeTunnelServer::new(0)
            .serve(ctx, super::Protocol::QUIC, &ip, tls)
            .await;
        self.tracker.release(&0).await;
        result
    }
}
