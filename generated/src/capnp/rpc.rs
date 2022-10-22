use anyhow::Result;
use capnp_rpc::{
    twoparty::{self, VatId},
    VatNetwork,
};
use futures::{AsyncRead, AsyncWrite};

pub struct ServerFactoryDefault<T: Sized>(pub T);

impl<T: Clone + Send + Sync> ServerFactoryDefault<T> {
    pub fn new(client: T) -> Self {
        Self(client)
    }
}

pub trait ServerFactory<T: Clone + Send + Sync> {
    fn new_client(&self) -> Result<T>;
}

impl<T: Clone + Send + Sync> ServerFactory<T> for ServerFactoryDefault<T> {
    fn new_client(&self) -> Result<T> {
        Ok(self.0.clone())
    }
}

impl<T: Clone + 'static + Sync + Send> From<T> for Box<dyn ServerFactory<T>> {
    fn from(client: T) -> Self {
        Box::new(ServerFactoryDefault::new(client))
    }
}

pub fn new_network_client<R, W>(send: W, recv: R) -> Box<dyn VatNetwork<VatId>>
where
    R: AsyncRead + Unpin + Send + Sync + 'static,
    W: AsyncWrite + Unpin + Send + Sync + 'static,
{
    Box::new(twoparty::VatNetwork::new(
        recv,
        send,
        VatId::Client,
        Default::default(),
    ))
}

pub fn new_network_server<R, W>(send: W, recv: R) -> Box<dyn VatNetwork<VatId>>
where
    R: AsyncRead + Unpin + Send + Sync + 'static,
    W: AsyncWrite + Unpin + Send + Sync + 'static,
{
    Box::new(twoparty::VatNetwork::new(
        recv,
        send,
        VatId::Server,
        Default::default(),
    ))
}

pub(crate) mod tests {
    use futures::AsyncReadExt;

    use super::*;

    #[allow(dead_code)]
    pub async fn setup_mock_networks() -> (Box<dyn VatNetwork<VatId>>, Box<dyn VatNetwork<VatId>>) {
        // Create two networks, one for the server and one for the client
        let (client, server) = tokio::io::duplex(64);

        let client_network = {
            let (recv, send) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(client).split();

            new_network_client(send, recv)
        };

        let server_network = {
            let (recv, send) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(server).split();

            new_network_server(send, recv)
        };

        (client_network, server_network)
    }
}

macro_rules! server_async_wrapper {
    ($t:ident, $f:ident [$self:ident, $params:ident, $results:ident]) => {
        let client = match $self.new_client() {
            Ok(client) => client,
            Err(err) => {
                return Promise::err(capnp::Error::failed(format!(
                    "Failed to create client: {:#}",
                    err
                )));
            }
        };

        return Promise::from_future(async move {
            let p = $t::from_primitive($params.get()?)
                .map_err(|e| capnp::Error::failed(e.to_string()))?;
            let r = client.$f(p).await?;
            r.to_primitive($results.get());
            Ok(())
        });
    };
}

pub(crate) use server_async_wrapper;
