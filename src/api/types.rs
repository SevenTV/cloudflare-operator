use serde::Serialize;

use crate::{types::Result, utils::macros::trait_alias};
use std::fmt::Debug;

pub trait RequestBody: Send + Sync + Debug {
    fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl RequestBody for () {}

impl RequestBody for Vec<u8> {
    fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(self.to_owned())
    }
}

impl RequestBody for String {
    fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}

pub trait QueryParams: Send + Sync + Debug {
    fn to_string(&self) -> Result<String> {
        Ok(String::new())
    }
}

impl QueryParams for () {}

impl QueryParams for String {
    fn to_string(&self) -> Result<String> {
        Ok(self.to_owned())
    }
}

impl QueryParams for Vec<(String, String)> {
    fn to_string(&self) -> Result<String> {
        Ok(self
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&"))
    }
}

trait_alias!(pub ResultBase = Debug + Default + Sync + Send);
pub trait QueryParamsSerialize: Serialize + ResultBase {}

impl<T: QueryParamsSerialize> QueryParams for T {
    fn to_string(&self) -> Result<String> {
        Ok(serde_qs::to_string(self)?)
    }
}
