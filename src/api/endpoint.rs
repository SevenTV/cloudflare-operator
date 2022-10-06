use std::fmt::Debug;

use async_trait::async_trait;
use hyper::{Body, Method, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::types::Result;

use super::types::{RawBody, StringBody};

#[async_trait]
pub trait Endpoint<ResultType = (), QueryType = (), BodyType = ()>: Sync + Send
where
    ResultType: Debug + Default,
    QueryType: Serialize,
    BodyType: Serialize,
{
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "".to_string()
    }

    fn query(&self) -> Option<QueryType> {
        None
    }

    fn body(&self) -> Option<BodyType> {
        None
    }

    fn headers(&self) -> Option<Vec<(String, String)>> {
        None
    }

    async fn result(&self, _resp: Response<Body>) -> Result<ResultType> {
        Ok(ResultType::default())
    }
}

pub trait RawEndpoint<QueryType, BodyType>: Endpoint<Vec<u8>, QueryType, BodyType>
where
    QueryType: Serialize,
    BodyType: Serialize,
{
}

#[async_trait]
impl<QueryType: Serialize, BodyType: Serialize, T: RawEndpoint<QueryType, BodyType>>
    Endpoint<RawBody, QueryType, BodyType> for T
{
    async fn result(&self, resp: Response<Body>) -> Result<RawBody> {
        Ok(RawBody {
            body: hyper::body::to_bytes(resp.into_body()).await?.to_vec(),
        })
    }
}

pub trait StringEndpoint<QueryType, BodyType>: Endpoint<String, QueryType, BodyType>
where
    QueryType: Serialize,
    BodyType: Serialize,
{
}

#[async_trait]
impl<QueryType: Serialize, BodyType: Serialize, T: StringEndpoint<QueryType, BodyType>>
    Endpoint<StringBody, QueryType, BodyType> for T
{
    async fn result(&self, resp: Response<Body>) -> Result<StringBody> {
        Ok(StringBody {
            body: String::from_utf8_lossy(&hyper::body::to_bytes(resp.into_body()).await?.to_vec())
                .to_string(),
        })
    }
}

pub trait JsonEndpoint<ResultType, QueryType, BodyType>:
    Endpoint<ResultType, QueryType, BodyType>
where
    ResultType: DeserializeOwned + Debug + Default,
    QueryType: Serialize,
    BodyType: Serialize,
{
}

#[async_trait]
impl<
        ResultType: DeserializeOwned + Debug + Default,
        QueryType: Serialize,
        BodyType: Serialize,
        T: JsonEndpoint<ResultType, QueryType, BodyType>,
    > Endpoint<ResultType, QueryType, BodyType> for T
{
    async fn result(&self, resp: Response<Body>) -> Result<ResultType> {
        Ok(serde_json::from_slice(
            &hyper::body::to_bytes(resp.into_body()).await?.to_vec(),
        )?)
    }
}
