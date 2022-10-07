use hyper::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    api::types::{RequestBody, ResultBase},
    utils::macros::trait_alias,
};

use super::{macros::endpoint, Endpoint};

trait_alias!(pub ResultJson = DeserializeOwned + ResultBase);

pub trait JsonEndpoint<ResultType, QueryType, BodyType>: Sync + Send
where
    ResultType: ResultJson,
    QueryType: Serialize,
    BodyType: RequestBody,
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

    fn to_endpoint(self) -> Box<dyn Endpoint<ResultType, QueryType, BodyType>>
    where
        Self: Sized + 'static,
        ResultType: ResultJson + 'static,
        QueryType: Serialize + 'static,
        BodyType: RequestBody + 'static,
    {
        Box::new(Box::new(self) as Box<dyn JsonEndpoint<ResultType, QueryType, BodyType>>)
    }
}

endpoint!(T JsonEndpoint, ResultJson, { [ resp, self ]
    Ok(serde_json::from_slice(
        &hyper::body::to_bytes(resp.into_body()).await?.to_vec(),
    )?)
});

