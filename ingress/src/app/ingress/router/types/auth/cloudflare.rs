use crate::config::types as config;

#[derive(Debug, Clone)]
pub enum Auth {
    ApiKey { api_key: String, email: String },
    ApiToken { api_token: String },
}

impl From<config::auth::Cloudflare> for Auth {
    fn from(auth: config::auth::Cloudflare) -> Self {
        match auth {
            config::auth::Cloudflare::ApiKey { api_key, email } => Self::ApiKey { api_key, email },
            config::auth::Cloudflare::ApiToken { api_token } => Self::ApiToken { api_token },
        }
    }
}
