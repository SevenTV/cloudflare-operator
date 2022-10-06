use serde::Deserialize;

use crate::{
    api::{endpoint::JsonEndpoint, ApiClient},
    types::Result,
};

use super::Context;

struct Ep {}

struct Api {}

impl ApiClient for Api {
    fn base_url(&self) -> String {
        "https://api.ipify.org?format=json".to_string()
    }
}

#[derive(Default, Debug, Deserialize)]
struct IpResp {
    ip: String,
}

impl JsonEndpoint<IpResp, (), ()> for Ep {}

pub async fn run(_ctx: &Context) -> Result<()> {
    let api = Api {};
    let ep = Ep {};

    let resp = api.request(&ep).await?;

    println!("{}", resp.ip);

    Ok(())
}
