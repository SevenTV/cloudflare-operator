use serde::de::DeserializeOwned;

use crate::{api::types::ResultBase, utils::macros::trait_alias};

use super::macros::endpoint;

trait_alias!(pub ResultJson = DeserializeOwned + ResultBase);

endpoint!(T JsonEndpoint, ResultJson, { [ resp, self ]
    Ok(serde_json::from_slice(
        &hyper::body::to_bytes(resp.into_body()).await?.to_vec(),
    )?)
});

pub use internal::JsonEndpoint;
