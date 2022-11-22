use anyhow::{anyhow, Result};
use trust_dns_resolver::{config::ResolverConfig, TokioAsyncResolver};

const DOMAIN: &str = "clients3.google.com";

pub async fn support_ipv6() -> Result<bool> {
    let cloudflare_resolver =
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), Default::default())?;

    let records = cloudflare_resolver.ipv6_lookup(DOMAIN).await?;

    let records = records.iter().collect::<Vec<_>>();
    if !records.is_empty() {
        let conn = tokio::net::TcpSocket::new_v6()?;
        Ok(conn
            .connect(std::net::SocketAddr::V6(std::net::SocketAddrV6::new(
                records[0].to_owned(),
                80,
                0,
                0,
            )))
            .await
            .is_ok())
    } else {
        Err(anyhow!("no ipv6 ips for domain"))
    }
}

pub async fn support_ipv4() -> Result<bool> {
    let cloudflare_resolver =
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), Default::default())?;
    let records = cloudflare_resolver.ipv4_lookup(DOMAIN).await?;

    let records = records.iter().collect::<Vec<_>>();
    if !records.is_empty() {
        let conn = tokio::net::TcpSocket::new_v4()?;
        Ok(conn
            .connect(std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
                records[0].to_owned(),
                80,
            )))
            .await
            .is_ok())
    } else {
        Err(anyhow!("no ipv4 ips for domain"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_support_ipv6() {
        assert!(support_ipv6().await.is_ok());
    }

    #[tokio::test]
    async fn test_support_ipv4() {
        assert!(support_ipv4().await.is_ok());
    }
}
