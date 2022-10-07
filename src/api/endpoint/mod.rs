use async_trait::async_trait;
use hyper::{Body, Method, Response};

use crate::types::Result;

use super::types::{QueryParams, RequestBody, ResultBase};

mod macros;

mod json;
mod string;
mod vec_8;

pub use json::*;
pub use string::*;
pub use vec_8::*;

#[async_trait]
pub trait Endpoint<ResultType = (), QueryType = (), BodyType = ()>: Sync + Send
where
    ResultType: ResultBase,
    QueryType: QueryParams,
    BodyType: RequestBody,
{
    fn _method(&self) -> Method {
        panic!("method not implemented")
    }

    fn _path(&self) -> String {
        panic!("method not implemented")
    }

    fn _query(&self) -> Option<QueryType> {
        panic!("method not implemented")
    }

    fn _body(&self) -> Option<BodyType> {
        panic!("method not implemented")
    }

    fn _headers(&self) -> Option<Vec<(String, String)>> {
        panic!("method not implemented")
    }

    async fn _result(&self, _resp: Response<Body>) -> Result<ResultType> {
        panic!("method not implemented")
    }
}
