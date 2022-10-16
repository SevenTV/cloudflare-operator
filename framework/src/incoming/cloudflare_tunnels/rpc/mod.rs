use capnp_rpc::{
    twoparty::{self, VatId},
    RpcSystem, VatNetwork,
};
use quinn::{RecvStream, SendStream};

use self::clients::{RegistrationServerClient, TunnelServerClient};

pub mod clients;
pub mod types;

mod logged;

pub(crate) struct ControlStreamManager {
    tunnel_server: TunnelServerClient,
}

impl ControlStreamManager {
    pub fn new(network: Box<dyn VatNetwork<VatId>>) -> (Self, RpcSystem<VatId>) {
        let mut rpc_system = RpcSystem::new(network, None);

        let rclient = RegistrationServerClient::new_from_system(&mut rpc_system);
        let tclient = TunnelServerClient::new_from_system(&mut rpc_system, rclient);

        (
            Self {
                tunnel_server: tclient,
            },
            rpc_system,
        )
    }

    pub fn get_tunnel_client(&self) -> &TunnelServerClient {
        &self.tunnel_server
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
