use std::{collections::HashMap, net::IpAddr, sync::Arc};

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

#[derive(Clone)]
pub enum IpVersion {
    Ipv4,
    Ipv6,
}

#[derive(Clone)]
pub struct IpPortHost {
    pub ip: IpAddr,
    pub port: u16,
    pub version: IpVersion,
    pub hostname: String,
}

impl IpPortHost {
    pub fn to_socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(self.ip, self.port)
    }
}

pub struct EdgeTracker {
    mp: Arc<Mutex<HashMap<u32, IpPortHost>>>,
    pool: Arc<Mutex<Vec<IpPortHost>>>,
}

impl EdgeTracker {
    pub fn new(pool: Vec<IpPortHost>) -> Self {
        Self {
            mp: Arc::new(Mutex::new(HashMap::new())),
            pool: Arc::new(Mutex::new(pool)),
        }
    }

    pub async fn release(&mut self, id: &u32) {
        let mut hash = self.mp.lock().await;

        let ip = hash.remove(id);
        if let Some(ip) = ip {
            let mut pool = self.pool.lock().await;
            pool.push(ip.to_owned());
        }
    }

    pub async fn get(&mut self, id: &u32) -> Result<IpPortHost> {
        let mut hash = self.mp.lock().await;

        let ip = hash.get(id);
        if let Some(ip) = ip {
            Ok(ip.clone().to_owned())
        } else {
            let mut pool = self.pool.lock().await;
            if let Some(ip) = pool.pop() {
                hash.insert(id.clone(), ip.clone());
                Ok(ip)
            } else {
                Err(anyhow!("no avaliable addresses"))
            }
        }
    }
}
