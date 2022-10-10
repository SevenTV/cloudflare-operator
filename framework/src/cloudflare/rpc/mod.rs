use capnp_rpc::{
    twoparty::{self, VatId},
    RpcSystem, VatNetwork,
};
use procmacros::generated_mod;
use quinn::{RecvStream, SendStream};

use self::clients::{RegistrationServerClient, TunnelServerClient};

generated_mod!(pub tunnelrpc_capnp "tunnelrpc_capnp.rs");

pub mod alias;
pub mod clients;
mod logged;
pub mod types;

pub(crate) struct ControlStreamManager {
    rpc_system: RpcSystem<VatId>,

    tunnel_server: TunnelServerClient,
}

impl ControlStreamManager {
    pub fn new(network: Box<dyn VatNetwork<VatId>>) -> Self {
        let mut rpc_system = RpcSystem::new(network, None);

        let rclient = RegistrationServerClient::new_from_system(&mut rpc_system);
        let tclient = TunnelServerClient::new_from_system(&mut rpc_system, rclient);

        Self {
            rpc_system,
            tunnel_server: tclient,
        }
    }

    pub fn get_tunnel_client(&self) -> TunnelServerClient {
        self.tunnel_server.clone()
    }
}

pub(crate) fn new_network(send: SendStream, recv: RecvStream) -> Box<dyn VatNetwork<VatId>> {
    log_network(Box::new(twoparty::VatNetwork::new(
        recv,
        send,
        VatId::Client,
        Default::default(),
    )))
}

pub(crate) fn log_network(network: Box<dyn VatNetwork<VatId>>) -> Box<dyn VatNetwork<VatId>> {
    logged::LoggedVatNetwork::new(network).boxed()
}
