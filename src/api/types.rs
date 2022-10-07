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

trait_alias!(pub ResultBase = Debug + Default);
