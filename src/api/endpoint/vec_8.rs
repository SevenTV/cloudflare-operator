
use hyper::Method;
use serde::Serialize;

use crate::{api::types::RequestBody};

use super::{Endpoint, macros::endpoint};

pub trait Vec8Endpoint<QueryType, BodyType>: Sync + Send
where
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

    fn to_endpoint(self) -> Box<dyn Endpoint<Vec<u8>, QueryType, BodyType>>
    where
        Self: Sized + 'static,
        QueryType: Serialize + 'static,
        BodyType: RequestBody + 'static,
    {
        Box::new(Box::new(self) as Box<dyn Vec8Endpoint<QueryType, BodyType>>)
    }
}

endpoint!(P Vec8Endpoint, Vec<u8>, { [ resp, self ]
    Ok(
        hyper::body::to_bytes(resp.into_body()).await?.to_vec()
    )
});
