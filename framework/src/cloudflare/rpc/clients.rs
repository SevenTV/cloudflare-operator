#![allow(dead_code, unused_variables, unused_mut)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.

use anyhow::Result;
use capnp_rpc::{twoparty::VatId, RpcSystem};
use log::info;

use super::{
    tunnelrpc::interfaces::{
        configuration_manager, registration_server, session_manager, tunnel_server,
    },
    tunnelrpc_capnp,
};

#[derive(Clone)]
pub struct RegistrationServerClient {
    inner: tunnelrpc_capnp::registration_server::Client,
}

impl RegistrationServerClient {
    pub fn new(inner: tunnelrpc_capnp::registration_server::Client) -> Self {
        Self { inner }
    }

    pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
        Self::new(system.bootstrap(VatId::Server))
    }

    pub async fn register_connection(
        &self,
        request: registration_server::RegisterConnectionParams,
    ) -> Result<registration_server::RegisterConnectionResults> {
        let mut req = self.inner.register_connection_request();
        request.to_primitive(req.get());

        info!("Sending register_connection request");

        let response = req.send().promise.await?;

        info!("Received register_connection response");

        let response = response.get()?;

        registration_server::RegisterConnectionResults::from_primitive(response)
    }

    pub async fn unregister_connection(
        &self,
        request: registration_server::UnregisterConnectionParams,
    ) -> Result<registration_server::UnregisterConnectionResults> {
        let mut req = self.inner.unregister_connection_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        registration_server::UnregisterConnectionResults::from_primitive(response)
    }

    pub async fn update_local_configuration(
        &self,
        request: registration_server::UpdateLocalConfigurationParams,
    ) -> Result<registration_server::UpdateLocalConfigurationResults> {
        let mut req = self.inner.update_local_configuration_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        registration_server::UpdateLocalConfigurationResults::from_primitive(response)
    }
}

#[derive(Clone)]
pub struct TunnelServerClient {
    inner: tunnelrpc_capnp::tunnel_server::Client,
    registration: RegistrationServerClient,
}

impl TunnelServerClient {
    pub fn new(
        inner: tunnelrpc_capnp::tunnel_server::Client,
        registration: RegistrationServerClient,
    ) -> Self {
        Self {
            inner,
            registration,
        }
    }

    pub fn new_from_system(
        system: &mut RpcSystem<VatId>,
        registration: RegistrationServerClient,
    ) -> Self {
        Self::new(system.bootstrap(VatId::Server), registration)
    }

    pub fn get_registration_client(&self) -> &RegistrationServerClient {
        &self.registration
    }

    pub async fn register_tunnel(
        &self,
        request: tunnel_server::RegisterTunnelParams,
    ) -> Result<tunnel_server::RegisterTunnelResults> {
        let mut req = self.inner.register_tunnel_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        tunnel_server::RegisterTunnelResults::from_primitive(response)
    }

    pub async fn get_server_info(
        &self,
        request: tunnel_server::GetServerInfoParams,
    ) -> Result<tunnel_server::GetServerInfoResults> {
        let mut req = self.inner.get_server_info_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        tunnel_server::GetServerInfoResults::from_primitive(response)
    }

    pub async fn unregister_tunnel(
        &self,
        request: tunnel_server::UnregisterTunnelParams,
    ) -> Result<tunnel_server::UnregisterTunnelResults> {
        let mut req = self.inner.unregister_tunnel_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        tunnel_server::UnregisterTunnelResults::from_primitive(response)
    }

    pub async fn obsolete_declarative_tunnel_connect(
        &self,
        request: tunnel_server::ObsoleteDeclarativeTunnelConnectParams,
    ) -> Result<tunnel_server::ObsoleteDeclarativeTunnelConnectResults> {
        let mut req = self.inner.obsolete_declarative_tunnel_connect_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        tunnel_server::ObsoleteDeclarativeTunnelConnectResults::from_primitive(response)
    }

    pub async fn authenticate(
        &self,
        request: tunnel_server::AuthenticateParams,
    ) -> Result<tunnel_server::AuthenticateResults> {
        let mut req = self.inner.authenticate_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        tunnel_server::AuthenticateResults::from_primitive(response)
    }

    pub async fn reconnect_tunnel(
        &self,
        request: tunnel_server::ReconnectTunnelParams,
    ) -> Result<tunnel_server::ReconnectTunnelResults> {
        let mut req = self.inner.reconnect_tunnel_request();
        request.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        tunnel_server::ReconnectTunnelResults::from_primitive(response)
    }
}

#[derive(Clone)]
pub struct SessionManagerClient {
    inner: tunnelrpc_capnp::session_manager::Client,
}

impl SessionManagerClient {
    pub fn new(inner: tunnelrpc_capnp::session_manager::Client) -> Self {
        Self { inner }
    }

    pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
        Self::new(system.bootstrap(VatId::Server))
    }

    pub async fn register_udp_session(
        &self,
        params: session_manager::RegisterUdpSessionParams,
    ) -> Result<session_manager::RegisterUdpSessionResults> {
        let mut req = self.inner.register_udp_session_request();
        params.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        session_manager::RegisterUdpSessionResults::from_primitive(response)
    }
    pub async fn unregister_udp_session(
        &self,
        params: session_manager::UnregisterUdpSessionParams,
    ) -> Result<session_manager::UnregisterUdpSessionResults> {
        let mut req = self.inner.unregister_udp_session_request();
        params.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        session_manager::UnregisterUdpSessionResults::from_primitive(response)
    }
}

#[derive(Clone)]
pub struct ConfigurationManagerClient {
    inner: tunnelrpc_capnp::configuration_manager::Client,
}

impl ConfigurationManagerClient {
    pub fn new(inner: tunnelrpc_capnp::configuration_manager::Client) -> Self {
        Self { inner }
    }

    pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
        Self::new(system.bootstrap(VatId::Server))
    }

    pub async fn update_configuration(
        &self,
        params: configuration_manager::UpdateConfigurationParams,
    ) -> Result<configuration_manager::UpdateConfigurationResults> {
        let mut req = self.inner.update_configuration_request();
        params.to_primitive(req.get());

        let response = req.send().promise.await?;
        let response = response.get()?;

        configuration_manager::UpdateConfigurationResults::from_primitive(response)
    }
}

#[derive(Clone)]
pub struct CloudflaredServerClient {
    session_manager: SessionManagerClient,
    configuration_manager: ConfigurationManagerClient,
}

impl CloudflaredServerClient {
    pub fn new(
        session_manager: SessionManagerClient,
        configuration_manager: ConfigurationManagerClient,
    ) -> Self {
        Self {
            session_manager,
            configuration_manager,
        }
    }

    pub fn get_session_manager_client(&self) -> &SessionManagerClient {
        &self.session_manager
    }

    pub fn get_configuration_manager_client(&self) -> &ConfigurationManagerClient {
        &self.configuration_manager
    }
}
