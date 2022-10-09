use capnp::{capability::Promise, Result};
use capnp_rpc::{Connection, VatNetwork};
use futures::{AsyncRead, FutureExt};
use log::debug;

pub(super) struct LoggedVatNetwork<T>
where
    T: AsyncRead + 'static + Unpin,
{
    network: Box<dyn VatNetwork<T>>,
}

impl<T> LoggedVatNetwork<T>
where
    T: AsyncRead + Unpin,
{
    pub fn new(network: Box<dyn VatNetwork<T>>) -> Self {
        Self { network }
    }

    pub fn boxed(self) -> Box<dyn VatNetwork<T>> {
        Box::new(self)
    }
}

async fn wrap_connection<T>(
    result: Result<Box<dyn Connection<T>>>,
) -> Result<Box<dyn Connection<T>>>
where
    T: AsyncRead + Unpin + 'static,
{
    match result {
        Ok(connection) => Ok(LoggedConnection::new(connection).boxed()),
        Err(err) => Err(err),
    }
}

impl<T> VatNetwork<T> for LoggedVatNetwork<T>
where
    T: AsyncRead + Unpin,
{
    fn accept(&mut self) -> Promise<Box<dyn Connection<T>>, ::capnp::Error> {
        debug!("accepting connection");

        Promise::from_future(
            self.network
                .accept()
                .then(|result| wrap_connection::<T>(result)),
        )
    }

    fn connect(&mut self, id: T) -> Option<Box<dyn Connection<T>>> {
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

struct LoggedConnection<T>
where
    T: AsyncRead + 'static + Unpin,
{
    connection: Box<dyn Connection<T>>,
}

impl<T> LoggedConnection<T>
where
    T: AsyncRead + Unpin,
{
    fn new(connection: Box<dyn Connection<T>>) -> Self {
        Self { connection }
    }

    fn boxed(self) -> Box<dyn Connection<T>> {
        Box::new(self)
    }
}

impl<T> Connection<T> for LoggedConnection<T>
where
    T: AsyncRead + Unpin,
{
    fn get_peer_vat_id(&self) -> T {
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
