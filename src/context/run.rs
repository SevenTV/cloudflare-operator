use log::info;
use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::{
    api::{
        endpoint::{JsonEndpoint, StringEndpoint, Vec8Endpoint},
        types::QueryParamsSerialize,
        ApiClient,
    },
};

use super::Context;

#[derive(Clone)]
struct Ep {}

#[derive(Clone)]
struct Api {}

impl ApiClient for Api {
    fn base_url(&self) -> String {
        "https://api.ipify.org".to_string()
    }
}

#[derive(Default, Debug, Deserialize)]
struct IpResp {
    ip: String,
}

#[derive(Default, Debug, Serialize)]
struct Query {
    format: String,
}

impl QueryParamsSerialize for Query {}

impl StringEndpoint<(), ()> for Ep {}
impl Vec8Endpoint<(), ()> for Ep {}
impl JsonEndpoint<IpResp, Query, ()> for Ep {
    fn query(&self) -> Option<Query> {
        Some(Query {
            format: "json".to_string(),
        })
    }
}

pub async fn run(_ctx: &Context) -> Result<()> {
    let api = Api {};
    let ep = Ep {};

    let ep_cloned = ep.clone();
    let api_clone = api.clone();
    let resp = tokio::spawn(async move {
        api_clone
            .request(StringEndpoint::to_endpoint(ep_cloned.clone()))
            .await
            .unwrap()
    });

    let ep_cloned = ep.clone();
    let api_clone = api.clone();
    let resp2 = tokio::spawn(async move {
        api_clone
            .request(JsonEndpoint::to_endpoint(ep_cloned.clone()))
            .await
            .unwrap()
    });

    let ep_cloned = ep.clone();
    let api_clone = api.clone();
    let resp3 = tokio::spawn(async move {
        api_clone
            .request(Vec8Endpoint::to_endpoint(ep_cloned.clone()))
            .await
            .unwrap()
    });

    info!("Waiting on responses");

    let (resp, resp2, resp3) = tokio::join!(resp, resp2, resp3);

    info!("resp1: {:?}", resp);
    info!("resp2: {:?}", resp2);
    info!("resp3: {:?}", resp3);

    Ok(())
}
