use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CloudflareError {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CloudflareResponse<T> {
    pub success: bool,
    pub errors: Vec<CloudflareError>,
    pub messages: Vec<String>,
    pub result: Option<T>,
}
