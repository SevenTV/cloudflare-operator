use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct RawBody {
    pub body: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct StringBody {
    pub body: String,
}
