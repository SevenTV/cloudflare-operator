use super::{common::CloudflareResponse, handle_cloudflare_response, Client, BASE_URL};
use anyhow::{anyhow, Result};
use reqwest::Method;

mod types;

pub use types::*;

impl Client {
    pub async fn list_tunnels(&self) -> Result<Vec<Tunnel>> {
        let url = format!("{}/accounts/{}/cfd_tunnel", BASE_URL, self.account_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Vec<Tunnel>>, ()>(&url, Method::GET, None)
                .await?
        );
    }

    pub async fn create_tunnel(&self, req: TunnelRequest) -> Result<Tunnel> {
        let url = format!("{}/accounts/{}/cfd_tunnel", BASE_URL, self.account_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, TunnelRequest>(
                &url,
                Method::POST,
                Some(&req)
            )
            .await?
        );
    }

    pub async fn update_tunnel(&self, tunnel_id: &str, req: TunnelRequest) -> Result<Tunnel> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}",
            BASE_URL, self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, TunnelRequest>(
                &url,
                Method::PATCH,
                Some(&req)
            )
            .await?
        );
    }

    pub async fn get_tunnel(&self, tunnel_id: &str) -> Result<Tunnel> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}",
            BASE_URL, self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, ()>(&url, Method::GET, None)
                .await?
        );
    }

    pub async fn delete_tunnel(&self, tunnel_id: &str) -> Result<Tunnel> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}",
            BASE_URL, self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, ()>(&url, Method::DELETE, None)
                .await?
        );
    }

    pub async fn list_tunnel_connections(&self, tunnel_id: &str) -> Result<Vec<TunnelConnection>> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}/connections",
            BASE_URL, self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Vec<TunnelConnection>>, ()>(
                &url,
                Method::GET,
                None
            )
            .await?
        );
    }

    pub async fn clean_up_tunnel_connections(&self, tunnel_id: &str) -> Result<()> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}/connections",
            BASE_URL, self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<()>, ()>(&url, Method::DELETE, None)
                .await?
        );
    }

    pub async fn get_tunnel_token(&self, tunnel_id: &str) -> Result<String> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}/token",
            BASE_URL, self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<String>, ()>(&url, Method::GET, None)
                .await?
        );
    }
}
