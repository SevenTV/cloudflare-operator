use hyper::Method;
use serde::Deserialize;

use crate::{
    api::{
        endpoint::{JsonEndpoint, StringEndpoint, Vec8Endpoint},
        ApiClient,
    },
    types::Result,
};

use super::Context;

#[derive(Clone)]
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

impl StringEndpoint<(), ()> for Ep {}
impl Vec8Endpoint<(), ()> for Ep {}
impl JsonEndpoint<IpResp, (), ()> for Ep {}


pub async fn run(_ctx: &Context) -> Result<()> {
    let api = Api {};
    let ep = Ep {};

    let resp = api.request(StringEndpoint::to_endpoint(ep.clone())).await?;
    println!("{:?}", resp);

    let resp2 = api.request(JsonEndpoint::to_endpoint(ep.clone())).await?;
    println!("{:?}", resp2);

    let resp3 = api.request(Vec8Endpoint::to_endpoint(ep.clone())).await?;
    println!("{:?}", resp3);

    Ok(())
}
