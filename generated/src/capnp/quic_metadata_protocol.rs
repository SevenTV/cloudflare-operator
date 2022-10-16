#![allow(dead_code, unused_variables, unused_mut)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.

use super::raw::quic_metadata_protocol_capnp;

pub mod primitives {
    #![allow(dead_code, unused_variables, unused_mut)]

    pub use super::quic_metadata_protocol_capnp::connect_request as ConnectRequest;
    pub use super::quic_metadata_protocol_capnp::connect_response as ConnectResponse;
    pub use super::quic_metadata_protocol_capnp::metadata as Metadata;
    pub use super::quic_metadata_protocol_capnp::ConnectionType;
}

pub mod structs {
    use anyhow::Result;

    use super::primitives;

    #[derive(Clone, Debug)]
    pub struct ConnectRequest {
        pub dest: String,
        pub connection_type: primitives::ConnectionType,
        pub metadata: Vec<Metadata>,
    }

    impl ConnectRequest {
        pub fn to_primitive(&self, builder: primitives::ConnectRequest::Builder) {
            let mut builder = builder;

            builder.set_dest(&self.dest);
            builder.set_type(self.connection_type);

            {
                let b = builder.reborrow().init_metadata(self.metadata.len() as u32);
                let m = &self.metadata;
                utils::vec_type_to_primitive!(m, b);
            }
        }

        pub fn from_primitive(primitive: primitives::ConnectRequest::Reader) -> Result<Self> {
            let m = primitive.get_metadata()?;
            let metadata = utils::vec_type_from_primitive!(m, Metadata);

            Ok(Self {
                dest: primitive.get_dest()?.to_string(),
                connection_type: primitive.get_type()?,
                metadata,
            })
        }
    }

    #[derive(Clone, Debug)]
    pub struct Metadata {
        pub key: String,
        pub val: String,
    }

    impl Metadata {
        pub fn to_primitive(&self, builder: primitives::Metadata::Builder) {
            let mut builder = builder;

            builder.set_key(&self.key);
            builder.set_val(&self.val);
        }

        pub fn from_primitive(primitive: primitives::Metadata::Reader) -> Result<Self> {
            Ok(Self {
                key: primitive.get_key()?.to_string(),
                val: primitive.get_val()?.to_string(),
            })
        }
    }

    #[derive(Clone, Debug)]
    pub struct ConnectResponse {
        pub error: Option<String>,
        pub metadata: Vec<Metadata>,
    }

    impl ConnectResponse {
        pub fn to_primitive(&self, builder: primitives::ConnectResponse::Builder) {
            let mut builder = builder;

            if let Some(error) = &self.error {
                builder.set_error(error);
            }

            {
                let b = builder.reborrow().init_metadata(self.metadata.len() as u32);
                let m = &self.metadata;
                utils::vec_type_to_primitive!(m, b);
            }
        }

        pub fn from_primitive(primitive: primitives::ConnectResponse::Reader) -> Result<Self> {
            let m = primitive.get_metadata()?;
            let metadata = utils::vec_type_from_primitive!(m, Metadata);

            Ok(Self {
                error: Some(primitive.get_error()?.to_string()),
                metadata,
            })
        }
    }
}
