use std::fmt::Debug;

use async_trait::async_trait;
use hyper::{Body, Client, Request};
use serde::Serialize;

use crate::types::Result;

use self::{endpoint::Endpoint, types::RequestBody};

pub mod endpoint;
pub mod types;

#[async_trait]
pub trait ApiClient: Send + Sync {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: Box<dyn Endpoint<ResultType, QueryType, BodyType>>,
    ) -> Result<ResultType>
    where
        ResultType: Debug + Default,
        QueryType: Serialize,
        BodyType: RequestBody,
    {
        let mut builder = Request::builder().method(endpoint._method());

        if let Some(headers) = self.headers() {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        if let Some(headers) = endpoint._headers() {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        if let Some(query) = endpoint._query() {
            builder = builder.uri(format!(
                "{}{}?{}",
                self.base_url(),
                endpoint._path(),
                serde_qs::to_string(&query)?
            ));
        } else {
            builder = builder.uri(format!("{}{}", self.base_url(), endpoint._path()));
        }

        let req = builder.body(match endpoint._body() {
            Some(body) => Body::from(body.to_vec()?),
            None => Body::empty(),
        })?;

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client: Client<_, Body> = Client::builder().build(https);

        Ok(endpoint._result(client.request(req).await?).await?)
    }

    fn base_url(&self) -> String {
        "".to_string()
    }

    fn headers(&self) -> Option<Vec<(String, String)>> {
        None
    }
}
