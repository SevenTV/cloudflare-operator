use super::{common::CloudflareResponse, Client, BASE_URL};
use anyhow::{anyhow, Context, Result};
use log::{debug, error};
use std::collections::HashMap;

mod types;

pub use types::*;

impl Client {
    pub async fn list_tunnels(&self) -> Result<Vec<Tunnel>> {
        let url = format!("{}/accounts/{}/cfd_tunnel", BASE_URL, self.account_id);

        debug!("GET {}", url);

        let response = self
            .add_auth(self.client.get(&url))
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response = serde_json::from_str::<CloudflareResponse<Vec<Tunnel>>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn create_tunnel(&self, req: TunnelRequest) -> Result<Tunnel> {
        let url = format!("{}/accounts/{}/cfd_tunnel", BASE_URL, self.account_id);

        debug!("POST {}", url);

        let response = self
            .add_auth(self.client.post(&url))
            .json(&req)
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response = serde_json::from_str::<CloudflareResponse<Tunnel>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn update_tunnel(&self, tunnel_id: &str, req: TunnelRequest) -> Result<Tunnel> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}",
            BASE_URL, self.account_id, tunnel_id
        );

        debug!("PATCH {}", url);

        let response = self
            .add_auth(self.client.patch(&url))
            .json(&req)
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response = serde_json::from_str::<CloudflareResponse<Tunnel>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn get_tunnel(&self, tunnel_id: &str) -> Result<Tunnel> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}",
            BASE_URL, self.account_id, tunnel_id
        );

        debug!("GET {}", url);

        let response = self
            .add_auth(self.client.get(&url))
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response = serde_json::from_str::<CloudflareResponse<Tunnel>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn delete_tunnel(&self, tunnel_id: &str) -> Result<Tunnel> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}",
            BASE_URL, self.account_id, tunnel_id
        );

        debug!("DELETE {}", url);

        let response = self
            .add_auth(self.client.delete(&url))
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response = serde_json::from_str::<CloudflareResponse<Tunnel>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn list_tunnel_connections(&self, tunnel_id: &str) -> Result<Vec<TunnelConnection>> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}/connections",
            BASE_URL, self.account_id, tunnel_id
        );

        debug!("GET {}", url);

        let response = self
            .add_auth(self.client.get(&url))
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response =
            serde_json::from_str::<CloudflareResponse<Vec<TunnelConnection>>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn clean_up_tunnel_connections(&self, tunnel_id: &str) -> Result<()> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}/connections",
            BASE_URL, self.account_id, tunnel_id
        );

        debug!("DELETE {}", url);

        let response = self
            .add_auth(self.client.delete(&url))
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response =
            serde_json::from_str::<CloudflareResponse<HashMap<String, String>>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }

    pub async fn get_tunnel_token(&self, tunnel_id: &str) -> Result<String> {
        let url = format!(
            "{}/accounts/{}/cfd_tunnel/{}/token",
            BASE_URL, self.account_id, tunnel_id
        );

        debug!("GET {}", url);

        let response = self
            .add_auth(self.client.get(&url))
            .send()
            .await
            .with_context(|| format!("Failed to send request {}", url))?;

        let response_text = response
            .text()
            .await
            .with_context(|| format!("Failed to read response {}", url))?;

        let response = serde_json::from_str::<CloudflareResponse<String>>(&response_text);

        if let Ok(response) = response {
            if response.success {
                return Ok(response.result.unwrap());
            }

            Err(anyhow!(
                "errors: {} message: {}",
                response
                    .errors
                    .iter()
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                response.messages.join(", ")
            ))
        } else {
            error!("Failed to parse response json {}", response_text);
            Err(anyhow!(
                "{} returned invalid response. {}",
                url,
                response.unwrap_err()
            ))
        }
    }
}
