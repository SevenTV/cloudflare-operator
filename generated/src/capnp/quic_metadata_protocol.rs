#![allow(dead_code)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.

use super::raw::quic_metadata_protocol_capnp;

pub mod primitives {
    #![allow(dead_code)]

    pub use super::quic_metadata_protocol_capnp::connect_request as ConnectRequest;
    pub use super::quic_metadata_protocol_capnp::connect_response as ConnectResponse;
    pub use super::quic_metadata_protocol_capnp::metadata as Metadata;
    pub use super::quic_metadata_protocol_capnp::ConnectionType;
}

pub mod structs {
    use anyhow::Result;

    use super::primitives;

    #[derive(Clone, Debug, PartialEq, Eq)]
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

    #[derive(Clone, Debug, PartialEq, Eq)]
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ConnectResponse {
        pub error: String,
        pub metadata: Vec<Metadata>,
    }

    impl ConnectResponse {
        pub fn to_primitive(&self, builder: primitives::ConnectResponse::Builder) {
            let mut builder = builder;

            builder.set_error(&self.error);

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
                error: primitive.get_error()?.to_string(),
                metadata,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_connect_request() {
            let m = ConnectRequest {
                dest: "test".to_string(),
                connection_type: primitives::ConnectionType::Http,
                metadata: vec![Metadata {
                    key: "key".to_string(),
                    val: "val".to_string(),
                }],
            };

            let mut buf = Vec::new();
            {
                let mut message = capnp::message::Builder::new_default();
                {
                    let builder = message.init_root::<primitives::ConnectRequest::Builder>();
                    m.to_primitive(builder);
                }
                capnp::serialize::write_message(&mut buf, &message).unwrap();
            }

            let message_reader =
                capnp::serialize::read_message(&mut &buf[..], capnp::message::ReaderOptions::new())
                    .unwrap();
            let reader = message_reader
                .get_root::<primitives::ConnectRequest::Reader>()
                .unwrap();

            let m2 = ConnectRequest::from_primitive(reader).unwrap();

            assert_eq!(m, m2);
        }

        #[test]
        fn test_metadata() {
            let m = Metadata {
                key: "key".to_string(),
                val: "val".to_string(),
            };

            let mut buf = Vec::new();
            {
                let mut message = capnp::message::Builder::new_default();
                {
                    let builder = message.init_root::<primitives::Metadata::Builder>();
                    m.to_primitive(builder);
                }
                capnp::serialize::write_message(&mut buf, &message).unwrap();
            }

            let message_reader =
                capnp::serialize::read_message(&mut &buf[..], capnp::message::ReaderOptions::new())
                    .unwrap();
            let reader = message_reader
                .get_root::<primitives::Metadata::Reader>()
                .unwrap();

            let m2 = Metadata::from_primitive(reader).unwrap();

            assert_eq!(m, m2);
        }

        #[test]
        fn test_connect_response() {
            let m = ConnectResponse {
                error: "test".to_string(),
                metadata: vec![Metadata {
                    key: "key".to_string(),
                    val: "val".to_string(),
                }],
            };

            let mut buf = Vec::new();
            {
                let mut message = capnp::message::Builder::new_default();
                {
                    let builder = message.init_root::<primitives::ConnectResponse::Builder>();
                    m.to_primitive(builder);
                }
                capnp::serialize::write_message(&mut buf, &message).unwrap();
            }

            let message_reader =
                capnp::serialize::read_message(&mut &buf[..], capnp::message::ReaderOptions::new())
                    .unwrap();
            let reader = message_reader
                .get_root::<primitives::ConnectResponse::Reader>()
                .unwrap();

            let m2 = ConnectResponse::from_primitive(reader).unwrap();

            assert_eq!(m, m2);
        }
    }
}
