pub mod common;
pub mod tunnels;

static BASE_URL: &str = "https://api.cloudflare.com/client/v4";

pub enum Auth {
    ApiKey { key: String, email: String },
    ApiToken(String),
}

pub struct Client {
    client: reqwest::Client,
    auth: Auth,
    account_id: String,
}

impl Client {
    pub fn new(account_id: String, auth: Auth) -> Self {
        Self {
            client: reqwest::Client::new(),
            account_id,
            auth,
        }
    }

    fn add_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth {
            Auth::ApiKey { key, email } => request
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key),
            Auth::ApiToken(token) => request.header("Authorization", format!("Bearer {}", token)),
        }
    }
}
