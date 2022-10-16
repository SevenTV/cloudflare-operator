pub mod quic_metadata_protocol;
pub mod tunnelrpc;

pub mod raw {
    use procmacros::generated_mod;

    generated_mod!(pub tunnelrpc_capnp "tunnelrpc_capnp.rs");
    generated_mod!(pub quic_metadata_protocol_capnp "quic_metadata_protocol_capnp.rs");
}
