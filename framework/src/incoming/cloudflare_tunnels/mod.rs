use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use tokio::{select, task::JoinHandle, time::Instant};
use tracing::{error, info, warn};
use utils::{common::handle_errors, context::wait::Context};
use uuid::Uuid;

mod dns;
mod edge;
mod tls;
mod tunnel;

pub mod types;

use self::{
    edge::{EdgeTracker, IpPortHost},
    tunnel::EdgeTunnelClient,
    types::{ControlStreamError, EdgeRegion, Protocol, ProtocolCerts, RootCert, TunnelAuth},
};

use super::types::HandleHttp;

pub use dns::resolve_edge_addr;

pub struct Supervisor {
    id: Uuid,
    tracker: EdgeTracker,
    tls: Arc<ProtocolCerts>,
    auth: TunnelAuth,
    handle: HandleHttp,
}

impl Supervisor {
    pub async fn new(
        id: Uuid,
        edges: Vec<EdgeRegion>,
        auth: TunnelAuth,
        tls: ProtocolCerts,
        handle: HandleHttp,
    ) -> Result<Self> {
        let mut ips = Vec::new();

        for edge in edges {
            for ip in edge.addrs {
                ips.push(IpPortHost {
                    ip,
                    port: edge.port,
                });
            }
        }

        Ok(Supervisor {
            id,
            auth,
            handle,
            tls: Arc::new(tls),
            tracker: EdgeTracker::new(ips),
        })
    }

    async fn start_helper(&self, ctx: Context, idx: u8, tls: RootCert) -> JoinHandle<Result<()>> {
        let mut ctx = ctx;
        let id = self.id;
        let mut tracker = self.tracker.clone();
        let auth = self.auth.clone();
        let handle = self.handle.clone();
        let protocol = Protocol::Quic;

        tokio::spawn(async move {
            let mut big_attempt: u8 = 0;

            loop {
                let ip = tracker.get(&idx).await?;
                let mut attempt: u8 = 0;
                let mut last_connect: Instant;
                let mut done = false;

                loop {
                    select! {
                        _ = ctx.done() => {
                            info!("shutting down tunnel {}", id);
                            done = true;
                            break;
                        }
                        _ = tokio::time::sleep(Duration::from_secs(1)) => {
                            let client = EdgeTunnelClient::new(id, idx, auth.clone(), handle.clone());
                            info!("Starting tunnel server {}", id);

                            last_connect = tokio::time::Instant::now();

                            let result = client
                                .serve(ctx.clone(), protocol.clone(), ip.clone(), tls.clone(), attempt)
                                .await;

                            // 5min reset on attempt
                            if last_connect + Duration::from_secs(5 * 300) < tokio::time::Instant::now() {
                                attempt = 0;
                                big_attempt = 0
                            }

                            attempt += 1;
                            if let Err(e) = result {
                                error!("tunnel server {} failed: {}", id, e);
                            } else if let Ok(Some(e)) = result {
                                match e {
                                    ControlStreamError::Connection(e) => {
                                        if e.should_retry {
                                            warn!("tunnel server {} failed: {:?}", id, e);
                                            select!{
                                                _ = ctx.done() => {
                                                    info!("shutting down tunnel {}", id);
                                                    done = true;
                                                    break;
                                                }
                                                _ = tokio::time::sleep(Duration::from_nanos(e.retry_after as u64)) => {
                                                    continue;
                                                }
                                            }
                                        } else {
                                            error!("tunnel server {} fatally failed: {:?}", id, e);
                                            return Err(anyhow!("tunnel server {} failed: {:?}", id, e));
                                        }
                                    },
                                    ControlStreamError::Timeout(t) => {
                                        warn!("tunnel server {} timed out: {:?}", id, t);
                                    },
                                    ControlStreamError::Other(e) => {
                                        error!("tunnel server {} failed: {}", id, e);
                                    },
                                }
                            } else {
                                info!("tunnel server {} exited", id);
                            }

                            if attempt > 5 {
                                // We think the edge is down, so we'll get a new ip and try again...
                                warn!("tunnel server {} failed too many times, getting new edge", id);
                                break;
                            }
                        }
                    }
                }

                big_attempt += 1;
                tracker.release(&idx).await;
                if big_attempt > 5 {
                    error!("tunnel server {} failed too many times, shutting down", id);
                    return Err(anyhow!("tunnel server {} failed too many times", id));
                } else if done {
                    break;
                }
            }

            Ok(())
        })
    }

    pub async fn start(self, ctx: Context, limit: u8) -> Result<()> {
        let tls = self.tls.quic.clone(); // TODO: support other protocols

        let mut handles = Vec::new();

        for idx in 0..limit {
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
