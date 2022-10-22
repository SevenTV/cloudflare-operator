#![allow(dead_code, unused_variables, unused_mut)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.

use anyhow::Result;
use capnp_rpc::{twoparty::VatId, RpcSystem};

use generated::capnp::{
    raw::tunnelrpc_capnp,
    tunnelrpc::interfaces::{configuration_manager, session_manager, tunnel_server},
};


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
