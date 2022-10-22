use std::{collections::HashMap, net::IpAddr, sync::Arc};

use anyhow::anyhow;
use anyhow::Result;
use rand::seq::SliceRandom;
use tokio::sync::Mutex;

use rand::thread_rng;

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

#[derive(Clone)]
pub struct EdgeTracker {
    mp: Arc<Mutex<HashMap<u32, IpPortHost>>>,
    pool: Arc<Mutex<Vec<IpPortHost>>>,
}

impl EdgeTracker {
    pub fn new(pool: Vec<IpPortHost>) -> Self {
        let mut pool = pool;
        let mut rand = thread_rng();

        // Randomize the ip pool
        pool.shuffle(&mut rand);

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
            Ok(ip.clone())
        } else {
            let mut pool = self.pool.lock().await;
            if let Some(ip) = pool.pop() {
                hash.insert(*id, ip.clone());
                Ok(ip)
            } else {
                Err(anyhow!("no avaliable addresses"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[tokio::test]
    async fn test_edge_tracker() {
        let addr = "1.1.1.1".parse().unwrap();
        let pool = vec![IpPortHost {
            ip: addr,
            port: 1,
            version: IpVersion::Ipv4,
            hostname: "test".to_string(),
        }];

        let mut tracker = EdgeTracker::new(pool);

        let ip = tracker.get(&1).await;
        assert!(ip.is_ok());
        assert_eq!(ip.unwrap().ip, addr);

        let ip = tracker.get(&1).await;
        assert_eq!(ip.unwrap().ip, addr);

        // try get a new one
        let ip = tracker.get(&2).await;
        assert!(ip.is_err());

        tracker.release(&1).await;

        let ip = tracker.get(&2).await;
        assert!(ip.is_ok());
        assert_eq!(ip.unwrap().ip, addr);
    }

    #[test]
    fn test_ip_port_host_to_sock_addr() {
        let host = IpPortHost {
            ip: "1.1.1.1".parse().unwrap(),
            port: 1,
            version: IpVersion::Ipv4,
            hostname: "test".to_string(),
        };

        let socket = host.to_socket_addr();
        assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
        assert_eq!(socket.port(), 1);
    }
}
