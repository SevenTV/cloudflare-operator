use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CloudflareError {
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct CloudflareResponse<T> {
    pub success: bool,
    pub errors: Vec<CloudflareError>,
    pub messages: Vec<String>,
    pub result: Option<T>,
}
