use capnp_rpc::{rpc_twoparty_capnp, twoparty::VatId, RpcSystem, VatNetwork};
use futures::AsyncRead;
use procmacros::generated_mod;

generated_mod!(pub tunnelrpc_capnp "tunnelrpc_capnp.rs");

mod logged;

pub(crate) struct TunnelServerClient {
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

pub(crate) fn log_network<T>(network: Box<dyn VatNetwork<T>>) -> Box<dyn VatNetwork<T>>
where
    T: AsyncRead + 'static + Unpin,
{
    logged::LoggedVatNetwork::<T>::new(network).boxed()
}
