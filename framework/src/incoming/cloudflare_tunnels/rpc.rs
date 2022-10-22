use capnp_rpc::{twoparty::VatId, RpcSystem, VatNetwork};
use generated::capnp::tunnelrpc::interfaces::tunnel_server;

pub(crate) struct ControlStreamManager {
    tunnel_server: tunnel_server::client::Client,
}

impl ControlStreamManager {
    pub fn new(network: Box<dyn VatNetwork<VatId>>) -> (Self, RpcSystem<VatId>) {
        let mut rpc_system = RpcSystem::new(network, None);

        let tclient = tunnel_server::client::Client::new_from_system(&mut rpc_system);

        (
            Self {
                tunnel_server: tclient,
            },
            rpc_system,
        )
    }

    pub fn get_tunnel_client(&self) -> &tunnel_server::client::Client {
        &self.tunnel_server
    }
}
