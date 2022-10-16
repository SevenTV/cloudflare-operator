use anyhow::{anyhow, Result};
use log::debug;
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

pub mod common;
pub mod tunnels;

static BASE_URL: &str = "https://api.cloudflare.com/client/v4";

pub enum Auth {
    ApiKey { key: String, email: String },
    ApiToken(String),
}

pub struct Client {
    auth: Auth,
    account_id: String,
}

impl Client {
    pub fn new(account_id: String, auth: Auth) -> Self {
        Self { account_id, auth }
    }

    fn add_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth {
            Auth::ApiKey { key, email } => request
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key),
            Auth::ApiToken(token) => request.header("Authorization", format!("Bearer {}", token)),
        }
    }

    async fn do_request<R: DeserializeOwned, T: Serialize + ?Sized>(
        &self,
        url: &str,
        method: Method,
        body: Option<&T>,
    ) -> Result<(R, StatusCode)> {
        debug!("{} {}", method, url);

        let client = reqwest::Client::new();

        let mut request = self.add_auth(client.request(method.clone(), url));

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| anyhow!("failed to send request {} {}: {:#}", method, url, e))?;

        let status = response.status();

        let response_text = response.text().await.map_err(|e| {
            anyhow!(
                "Failed to read response (status code {}) {} {}: {:#}",
                status,
                method,
                url,
                e
            )
        })?;

        let data = serde_json::from_str::<R>(&response_text).map_err(|e| {
            anyhow!(
                "Failed to parse response (status code {}) {} {}: {:#}",
                status,
                method,
                url,
                e
            )
        })?;

        Ok((data, status))
    }
}

macro_rules! handle_cloudflare_response {
    ($response:expr) => {
        let (data, status) = $response;
        if !data.success {
            return Err(anyhow!(
                "Cloudflare API returned an error ({}): {:#?} {:#?}",
                status,
                data.errors,
                data.messages
            ));
        }

        return Ok(data.result.unwrap_or(Default::default()));
    };
}

pub(self) use handle_cloudflare_response;
