use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthContainer {
    pub name: Option<String>,

    #[serde(flatten)]
    pub auth: Auth,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Auth {
    Cloudflare(Cloudflare),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Cloudflare {
    ApiKey { api_key: String, email: String },
    ApiToken { api_token: String },
}
