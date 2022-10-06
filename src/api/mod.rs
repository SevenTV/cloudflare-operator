use std::fmt::Debug;

use async_trait::async_trait;
use hyper::{Body, Client, Request};
use serde::Serialize;

use crate::types::Result;

use self::endpoint::Endpoint;

pub mod endpoint;
pub mod types;

#[async_trait]
pub trait ApiClient: Send + Sync {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType>),
    ) -> Result<ResultType>
    where
        ResultType: Debug + Default,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        let mut builder = Request::builder().method(endpoint.method());

        if let Some(headers) = self.headers() {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        if let Some(headers) = endpoint.headers() {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        if let Some(query) = endpoint.query() {
            builder = builder.uri(format!(
                "{}{}?{}",
                self.base_url(),
                endpoint.path(),
                serde_qs::to_string(&query)?
            ));
        } else {
            builder = builder.uri(format!("{}{}", self.base_url(), endpoint.path()));
        }

        let req = builder.body(match endpoint.body() {
            Some(body) => Body::from(serde_json::to_vec(&body)?),
            None => Body::empty(),
        })?;

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client: Client<_, Body> = Client::builder().build(https);

        Ok(endpoint.result(client.request(req).await?).await?)
    }

    fn base_url(&self) -> String {
        "".to_string()
    }

    fn headers(&self) -> Option<Vec<(String, String)>> {
        None
    }
}
