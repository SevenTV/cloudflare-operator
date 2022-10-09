use bitflags::bitflags;
use capnp_rpc::{rpc_twoparty_capnp, twoparty::VatId, RpcSystem, VatNetwork};
use procmacros::generated_mod;

generated_mod!(pub tunnelrpc_capnp "tunnelrpc_capnp.rs");

pub struct TunnelServerClient {
    client: tunnelrpc_capnp::tunnel_server::Client,
    rpc_system: RpcSystem<VatId>,
}

impl TunnelServerClient {
    pub fn new(network: Box<dyn VatNetwork<VatId>>) -> Self {
        let mut rpc_system = RpcSystem::new(network, None);

        let client: tunnelrpc_capnp::tunnel_server::Client =
            rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        TunnelServerClient { client, rpc_system }
    }
}
