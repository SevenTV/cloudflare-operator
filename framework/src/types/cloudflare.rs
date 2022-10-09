use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Token {
    #[serde(rename = "a")]
    pub account_tag: String,
    #[serde(rename = "s")]
    pub tunnel_secret: String,
    #[serde(rename = "t")]
    pub tunnel_id: Uuid,
}

impl Token {
    pub fn encode(&self) -> Result<String> {
        Ok(base64::encode_config(
            serde_json::to_string(self)?,
            base64::URL_SAFE_NO_PAD,
        ))
    }
}

pub fn token_from_str(token: &str) -> Result<Token> {
    Ok(serde_json::from_slice(&base64::decode(token)?)?)
}
