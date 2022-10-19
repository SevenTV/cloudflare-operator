use trust_dns_resolver::{config::ResolverConfig, TokioAsyncResolver};

const DOMAIN: &str = "clients3.google.com";

pub async fn support_ipv6() -> bool {
    let cloudflare_resolver =
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), Default::default());

    if let Ok(cloudflare_resolver) = cloudflare_resolver {
        let records = cloudflare_resolver.ipv6_lookup(DOMAIN).await;

        if let Ok(records) = records {
            let records = records.iter().collect::<Vec<_>>();
            if !records.is_empty() {
                let conn = tokio::net::TcpSocket::new_v6();
                if let Ok(conn) = conn {
                    return conn
                        .connect(std::net::SocketAddr::V6(std::net::SocketAddrV6::new(
                            records[0].to_owned(),
                            80,
                            0,
                            0,
                        )))
                        .await
                        .is_ok();
                }
            }
        }
    }

    false
}

pub async fn support_ipv4() -> bool {
    let cloudflare_resolver =
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), Default::default());

    if let Ok(cloudflare_resolver) = cloudflare_resolver {
        let records = cloudflare_resolver.ipv4_lookup(DOMAIN).await;

        if let Ok(records) = records {
            let records = records.iter().collect::<Vec<_>>();
            if !records.is_empty() {
                let conn = tokio::net::TcpSocket::new_v4();
                if let Ok(conn) = conn {
                    return conn
                        .connect(std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
                            records[0].to_owned(),
                            80,
                        )))
                        .await
                        .is_ok();
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_support_ipv6() {
        // This test will fail if the host doesn't support IPv6
        assert!(support_ipv6().await);
    }

    #[tokio::test]
    async fn test_support_ipv4() {
        // This test will fail if the host doesn't support IPv4
        assert!(support_ipv4().await);
    }
}
