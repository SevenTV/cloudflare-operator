use anyhow::{anyhow, Context, Result};
use log::debug;
use trust_dns_resolver::{config::ResolverConfig, TokioAsyncResolver};

use super::types::{EdgeRegion, EdgeRegionLocation};

const SRV_SERVICE: &str = "v2-origintunneld";
const SRV_PROTO: &str = "tcp";
const SRV_NAME: &str = "argotunnel.com";

impl EdgeRegionLocation {
    fn to_srv_service(&self) -> String {
        match self {
            EdgeRegionLocation::AUTO => SRV_SERVICE.to_string(),
            EdgeRegionLocation::US => format!("us-{}", SRV_SERVICE),
        }
    }
}

pub async fn resolve_edge_addr(location: EdgeRegionLocation) -> Result<Vec<EdgeRegion>> {
    let cloudflare_resolver =
        TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), Default::default())
            .context("failed to create cloudflare_resolver")?;

    let domain = format!("_{}._{}.{}", location.to_srv_service(), SRV_PROTO, SRV_NAME);

    debug!("domain {} looking up SRV record.", domain);

    let records = cloudflare_resolver
        .srv_lookup(&domain)
        .await
        .with_context(|| format!("failed to look up DNS SRV record for {}", domain))?;

    // These are 2 records, one primary and one backup
    // We now need to resolve the A record for each of these as well as the AAAA record

    let mut edge_regions = Vec::new();

    for record in records.iter() {
        let hostname = record
            .target()
            .to_string()
            .trim_end_matches('.')
            .to_string();
        let port = record.port();

        let mut addrs = Vec::new();

        if utils::ip::support_ipv4().await? {
            let v4_addrs = cloudflare_resolver
                .ipv4_lookup(&hostname)
                .await
                .with_context(|| format!("failed to look up DNS A record for {}", hostname))?;

            v4_addrs
                .iter()
                .for_each(|a| addrs.push(a.to_owned().into()));
        }

        if utils::ip::support_ipv6().await? {
            let v6_addrs = cloudflare_resolver
                .ipv6_lookup(&hostname)
                .await
                .with_context(|| format!("failed to look up DNS AAAA record for {}", hostname))?;

            v6_addrs
                .iter()
                .for_each(|a| addrs.push(a.to_owned().into()));
        }

        if addrs.is_empty() {
            return Err(anyhow!("No addresses found for endpoint"));
        }

        edge_regions.push(EdgeRegion {
            addrs,
            hostname,
            port,
        });
    }

    Ok(edge_regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_edge_addr_auto() {
        let edge_regions = resolve_edge_addr(EdgeRegionLocation::AUTO).await;
        assert!(edge_regions.is_ok());
        assert!(!edge_regions.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_resolve_edge_addr_us() {
        let edge_regions = resolve_edge_addr(EdgeRegionLocation::US).await;
        assert!(edge_regions.is_ok());
        assert!(!edge_regions.unwrap().is_empty());
    }
}
