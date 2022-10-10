use capnp::{capability::Promise, Result};
use capnp_rpc::{twoparty::VatId, Connection, VatNetwork};
use futures::FutureExt;
use log::debug;

pub(super) struct LoggedVatNetwork {
    network: Box<dyn VatNetwork<VatId>>,
}

impl LoggedVatNetwork {
    pub fn new(network: Box<dyn VatNetwork<VatId>>) -> Self {
        Self { network }
    }

    pub fn boxed(self) -> Box<dyn VatNetwork<VatId>> {
        Box::new(self)
    }
}

async fn wrap_connection(
    result: Result<Box<dyn Connection<VatId>>>,
) -> Result<Box<dyn Connection<VatId>>> {
    match result {
        Ok(connection) => Ok(LoggedConnection::new(connection).boxed()),
        Err(err) => Err(err),
    }
}

impl VatNetwork<VatId> for LoggedVatNetwork {
    fn accept(&mut self) -> Promise<Box<dyn Connection<VatId>>, ::capnp::Error> {
        debug!("accepting connection");

        Promise::from_future(self.network.accept().then(|result| wrap_connection(result)))
    }

    fn connect(&mut self, id: VatId) -> Option<Box<dyn Connection<VatId>>> {
        debug!("connect to connection");

        match self.network.connect(id) {
            Some(connection) => Some(LoggedConnection::new(connection).boxed()),
            None => None,
        }
    }

    fn drive_until_shutdown(&mut self) -> Promise<(), capnp::Error> {
        self.network.drive_until_shutdown()
    }
}

struct LoggedConnection {
    connection: Box<dyn Connection<VatId>>,
}

impl LoggedConnection {
    fn new(connection: Box<dyn Connection<VatId>>) -> Self {
        Self { connection }
    }

    fn boxed(self) -> Box<dyn Connection<VatId>> {
        Box::new(self)
    }
}

impl Connection<VatId> for LoggedConnection {
    fn get_peer_vat_id(&self) -> VatId {
        self.connection.get_peer_vat_id()
    }

    fn new_outgoing_message(
        &mut self,
        first_segment_word_size: u32,
    ) -> Box<dyn capnp_rpc::OutgoingMessage> {
        debug!("rpcconnection: new outgoing message");
        self.connection
            .new_outgoing_message(first_segment_word_size)
    }

    fn receive_incoming_message(
        &mut self,
    ) -> Promise<Option<Box<dyn capnp_rpc::IncomingMessage>>, capnp::Error> {
        debug!("rpcconnection: receive incoming message");
        self.connection.receive_incoming_message()
    }

    fn shutdown(&mut self, result: capnp::Result<()>) -> Promise<(), capnp::Error> {
        debug!("rpcconnection: shutdown");
        self.connection.shutdown(result)
    }
}
