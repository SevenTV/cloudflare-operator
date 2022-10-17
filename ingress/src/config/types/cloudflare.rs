use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Tunnel {
    pub account_id: String,
    pub tunnel_id: uuid::Uuid,
    pub auth: Option<String>,
}
