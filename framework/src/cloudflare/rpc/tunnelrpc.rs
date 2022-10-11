#![allow(dead_code, unused_variables, unused_mut)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.

use super::tunnelrpc_capnp;
pub mod primitives {
    #![allow(dead_code, unused_variables, unused_mut)]

    // These are all the types from the capnp schema.
    pub use super::tunnelrpc_capnp::authenticate_response as AuthenticationResponse;
    pub use super::tunnelrpc_capnp::authentication as Authentication;
    pub use super::tunnelrpc_capnp::client_info as ClientInfo;
    pub use super::tunnelrpc_capnp::connection_details as ConnectionDetails;
    pub use super::tunnelrpc_capnp::connection_error as ConnectionError;
    pub use super::tunnelrpc_capnp::connection_options as ConnectionOptions;
    pub use super::tunnelrpc_capnp::connection_response as ConnectionResponse;
    pub use super::tunnelrpc_capnp::connection_response::result as ConnectionResponseResult;
    pub use super::tunnelrpc_capnp::register_udp_session_response as RegisterUdpSessionResponse;
    pub use super::tunnelrpc_capnp::registration_options as RegistrationOptions;
    pub use super::tunnelrpc_capnp::server_info as ServerInfo;
    pub use super::tunnelrpc_capnp::tag as Tag;
    pub use super::tunnelrpc_capnp::tunnel_auth as TunnelAuth;
    pub use super::tunnelrpc_capnp::tunnel_registration as TunnelRegistration;
    pub use super::tunnelrpc_capnp::update_configuration_response as UpdateConfigurationResponse;
    pub use super::tunnelrpc_capnp::ExistingTunnelPolicy;

    // These are the types used in the RPC functions
    pub use super::tunnelrpc_capnp::registration_server::register_connection_params as RegisterConnectionParams;
    pub use super::tunnelrpc_capnp::registration_server::register_connection_results as RegisterConnectionResults;
    pub use super::tunnelrpc_capnp::registration_server::unregister_connection_params as UnregisterConnectionParams;
    pub use super::tunnelrpc_capnp::registration_server::unregister_connection_results as UnregisterConnectionResults;
    pub use super::tunnelrpc_capnp::registration_server::update_local_configuration_params as UpdateLocalConfigurationParams;
    pub use super::tunnelrpc_capnp::registration_server::update_local_configuration_results as UpdateLocalConfigurationResults;

    pub use super::tunnelrpc_capnp::tunnel_server::authenticate_params as AuthenticateParams;
    pub use super::tunnelrpc_capnp::tunnel_server::authenticate_results as AuthenticateResults;
    pub use super::tunnelrpc_capnp::tunnel_server::get_server_info_params as GetServerInfoParams;
    pub use super::tunnelrpc_capnp::tunnel_server::get_server_info_results as GetServerInfoResults;
    pub use super::tunnelrpc_capnp::tunnel_server::obsolete_declarative_tunnel_connect_params as ObsoleteDeclarativeTunnelConnectParams;
    pub use super::tunnelrpc_capnp::tunnel_server::obsolete_declarative_tunnel_connect_results as ObsoleteDeclarativeTunnelConnectResults;
    pub use super::tunnelrpc_capnp::tunnel_server::reconnect_tunnel_params as ReconnectTunnelParams;
    pub use super::tunnelrpc_capnp::tunnel_server::reconnect_tunnel_results as ReconnectTunnelResults;
    pub use super::tunnelrpc_capnp::tunnel_server::register_tunnel_params as RegisterTunnelParams;
    pub use super::tunnelrpc_capnp::tunnel_server::register_tunnel_results as RegisterTunnelResults;
    pub use super::tunnelrpc_capnp::tunnel_server::unregister_tunnel_params as UnregisterTunnelParams;
    pub use super::tunnelrpc_capnp::tunnel_server::unregister_tunnel_results as UnregisterTunnelResults;

    pub use super::tunnelrpc_capnp::session_manager::register_udp_session_params as RegisterUdpSessionParams;
    pub use super::tunnelrpc_capnp::session_manager::register_udp_session_results as RegisterUdpSessionResults;
    pub use super::tunnelrpc_capnp::session_manager::unregister_udp_session_params as UnregisterUdpSessionParams;
    pub use super::tunnelrpc_capnp::session_manager::unregister_udp_session_results as UnregisterUdpSessionResults;

    pub use super::tunnelrpc_capnp::configuration_manager::update_configuration_params as UpdateConfigurationParams;
    pub use super::tunnelrpc_capnp::configuration_manager::update_configuration_results as UpdateConfigurationResults;
}

pub mod structs {
    #![allow(dead_code, unused_variables, unused_mut)]

    use super::primitives;
    use anyhow::Result;

    #[derive(Debug)]
    pub struct Authentication {
        pub key: String,
        pub email: String,
        pub origin_c_a_key: String,
    }

    impl Authentication {
        pub fn to_primitive(&self, builder: primitives::Authentication::Builder) {
            let mut builder = builder;

            builder.set_key(&self.key);
            builder.set_email(&self.email);
            builder.set_origin_c_a_key(&self.origin_c_a_key);
        }
    }

    impl Authentication {
        pub fn from_primitive(primitive: primitives::Authentication::Reader) -> Result<Self> {
            Ok(Self {
                key: primitive.get_key()?.to_string(),
                email: primitive.get_email()?.to_string(),
                origin_c_a_key: primitive.get_origin_c_a_key()?.to_string(),
            })
        }
    }

    #[derive(Debug)]
    pub struct TunnelRegistration {
        pub err: String,
        pub url: String,
        pub log_lines: Vec<String>,
        pub permanent_failure: bool,
        pub tunnel_i_d: String,
        pub retry_after_seconds: u16,
        pub event_digest: Vec<u8>,
        pub conn_digest: Vec<u8>,
    }

    impl TunnelRegistration {
        pub fn to_primitive(&self, builder: primitives::TunnelRegistration::Builder) {
            let mut builder = builder;

            builder.set_err(&self.err);
            builder.set_url(&self.url);

            vec_string_to_primitive(
                &self.log_lines,
                builder
                    .reborrow()
                    .init_log_lines(self.log_lines.len() as u32),
            );

            builder.set_permanent_failure(self.permanent_failure);
            builder.set_tunnel_i_d(&self.tunnel_i_d);
            builder.set_retry_after_seconds(self.retry_after_seconds);
            builder.set_event_digest(&self.event_digest);
            builder.set_conn_digest(&self.conn_digest);
        }
    }

    impl TunnelRegistration {
        pub fn from_primitive(primitive: primitives::TunnelRegistration::Reader) -> Result<Self> {
            Ok(Self {
                err: primitive.get_err()?.to_string(),
                url: primitive.get_url()?.to_string(),
                log_lines: vec_string_from_primitive(primitive.get_log_lines()?)?,
                permanent_failure: primitive.get_permanent_failure(),
                tunnel_i_d: primitive.get_tunnel_i_d()?.to_string(),
                retry_after_seconds: primitive.get_retry_after_seconds(),
                event_digest: primitive.get_event_digest()?.to_vec(),
                conn_digest: primitive.get_conn_digest()?.to_vec(),
            })
        }
    }

    #[derive(Debug)]
    pub struct RegistrationOptions {
        pub client_id: String,
        pub version: String,
        pub os: String,
        pub existing_tunnel_policy: primitives::ExistingTunnelPolicy,
        pub pool_name: String,
        pub tags: Vec<Tag>,
        pub connection_id: u8,
        pub origin_local_ip: String,
        pub is_autoupdated: bool,
        pub run_from_terminal: bool,
        pub compression_quality: u64,
        pub uuid: String,
        pub num_previous_attempts: u8,
        pub features: Vec<String>,
    }

    impl RegistrationOptions {
        pub fn to_primitive(&self, builder: primitives::RegistrationOptions::Builder) {
            let mut builder = builder;

            builder.set_client_id(&self.client_id);
            builder.set_version(&self.version);
            builder.set_os(&self.os);
            builder.set_existing_tunnel_policy(self.existing_tunnel_policy);
            builder.set_pool_name(&self.pool_name);

            {
                let b = builder.reborrow().init_tags(self.tags.len() as u32);
                let t = &self.tags;
                utils::vec_type_to_primitive!(t, b);
            }

            builder.set_connection_id(self.connection_id);
            builder.set_origin_local_ip(&self.origin_local_ip);
            builder.set_is_autoupdated(self.is_autoupdated);
            builder.set_run_from_terminal(self.run_from_terminal);
            builder.set_compression_quality(self.compression_quality);
            builder.set_uuid(&self.uuid);
            builder.set_num_previous_attempts(self.num_previous_attempts);

            vec_string_to_primitive(
                &self.features,
                builder.reborrow().init_features(self.features.len() as u32),
            );
        }
    }

    impl RegistrationOptions {
        pub fn from_primitive(primitive: primitives::RegistrationOptions::Reader) -> Result<Self> {
            let t = primitive.get_tags()?;
            let tags = utils::vec_type_from_primitive!(t, Tag);

            Ok(Self {
                client_id: primitive.get_client_id()?.to_string(),
                version: primitive.get_version()?.to_string(),
                os: primitive.get_os()?.to_string(),
                existing_tunnel_policy: primitive.get_existing_tunnel_policy()?,
                pool_name: primitive.get_pool_name()?.to_string(),
                tags: tags,
                connection_id: primitive.get_connection_id(),
                origin_local_ip: primitive.get_origin_local_ip()?.to_string(),
                is_autoupdated: primitive.get_is_autoupdated(),
                run_from_terminal: primitive.get_run_from_terminal(),
                compression_quality: primitive.get_compression_quality(),
                uuid: primitive.get_uuid()?.to_string(),
                num_previous_attempts: primitive.get_num_previous_attempts(),
                features: vec_string_from_primitive(primitive.get_features()?)?,
            })
        }
    }

    #[derive(Debug)]
    pub struct Tag {
        pub name: String,
        pub value: String,
    }

    impl Tag {
        pub fn to_primitive(&self, builder: primitives::Tag::Builder) {
            let mut builder = builder;

            builder.set_name(&self.name);
            builder.set_value(&self.value);
        }
    }

    impl Tag {
        pub fn from_primitive(primitive: primitives::Tag::Reader) -> Result<Self> {
            Ok(Self {
                name: primitive.get_name()?.to_string(),
                value: primitive.get_value()?.to_string(),
            })
        }
    }

    #[derive(Debug)]
    pub struct ServerInfo {
        pub location_name: String,
    }

    impl ServerInfo {
        pub fn to_primitive(&self, builder: primitives::ServerInfo::Builder) {
            let mut builder = builder;

            builder.set_location_name(&self.location_name);
        }
    }

    impl ServerInfo {
        pub fn from_primitive(primitive: primitives::ServerInfo::Reader) -> Result<Self> {
            Ok(Self {
                location_name: primitive.get_location_name()?.to_string(),
            })
        }
    }

    #[derive(Debug)]
    pub struct AuthenticationResponse {
        pub permanent_err: String,
        pub retryable_err: String,
        pub jwt: Vec<u8>,
        pub hours_until_refresh: u8,
    }

    impl AuthenticationResponse {
        pub fn to_primitive(&self, builder: primitives::AuthenticationResponse::Builder) {
            let mut builder = builder;

            builder.set_permanent_err(&self.permanent_err);
            builder.set_retryable_err(&self.retryable_err);
            builder.set_jwt(&self.jwt);
            builder.set_hours_until_refresh(self.hours_until_refresh);
        }
    }

    impl AuthenticationResponse {
        pub fn from_primitive(
            primitive: primitives::AuthenticationResponse::Reader,
        ) -> Result<Self> {
            Ok(Self {
                permanent_err: primitive.get_permanent_err()?.to_string(),
                retryable_err: primitive.get_retryable_err()?.to_string(),
                jwt: primitive.get_jwt()?.to_vec(),
                hours_until_refresh: primitive.get_hours_until_refresh(),
            })
        }
    }

    #[derive(Debug)]
    pub struct ClientInfo {
        pub client_id: Vec<u8>,
        pub features: Vec<String>,
        pub version: String,
        pub arch: String,
    }

    impl ClientInfo {
        pub fn to_primitive(&self, builder: primitives::ClientInfo::Builder) {
            let mut builder = builder;

            builder.set_client_id(&self.client_id);
            vec_string_to_primitive(
                &self.features,
                builder.reborrow().init_features(self.features.len() as u32),
            );
            builder.set_version(&self.version);
            builder.set_arch(&self.arch);
        }
    }

    impl ClientInfo {
        pub fn from_primitive(primitive: primitives::ClientInfo::Reader) -> Result<Self> {
            Ok(Self {
                client_id: primitive.get_client_id()?.to_vec(),
                features: vec_string_from_primitive(primitive.get_features()?)?,
                version: primitive.get_version()?.to_string(),
                arch: primitive.get_arch()?.to_string(),
            })
        }
    }

    #[derive(Debug)]
    pub struct ConnectionOptions {
        pub client: ClientInfo,
        pub origin_local_ip: Vec<u8>,
        pub replace_existing: bool,
        pub compression_quality: u8,
        pub num_previous_attempts: u8,
    }

    impl ConnectionOptions {
        pub fn to_primitive(&self, builder: primitives::ConnectionOptions::Builder) {
            let mut builder = builder;

            self.client.to_primitive(builder.reborrow().init_client());
            builder.set_origin_local_ip(&self.origin_local_ip);
            builder.set_replace_existing(self.replace_existing);
            builder.set_compression_quality(self.compression_quality);
            builder.set_num_previous_attempts(self.num_previous_attempts);
        }
    }

    impl ConnectionOptions {
        pub fn from_primitive(primitive: primitives::ConnectionOptions::Reader) -> Result<Self> {
            Ok(Self {
                client: ClientInfo::from_primitive(primitive.get_client()?)?,
                origin_local_ip: primitive.get_origin_local_ip()?.to_vec(),
                replace_existing: primitive.get_replace_existing(),
                compression_quality: primitive.get_compression_quality(),
                num_previous_attempts: primitive.get_num_previous_attempts(),
            })
        }
    }

    #[derive(Debug)]
    pub struct ConnectionResponse {
        result: ConnectionResponseResult,
    }

    impl ConnectionResponse {
        pub fn to_primitive(&self, builder: primitives::ConnectionResponse::Builder) {
            let mut builder = builder.init_result();

            match &self.result {
                ConnectionResponseResult::ConnectionDetails(details) => {
                    let builder = builder.reborrow().init_connection_details();
                    details.to_primitive(builder);
                }
                ConnectionResponseResult::Error(err) => {
                    let builder = builder.reborrow().init_error();
                    err.to_primitive(builder);
                }
            }
        }
    }

    impl ConnectionResponse {
        pub fn from_primitive(primitive: primitives::ConnectionResponse::Reader) -> Result<Self> {
            Ok(Self {
                result: {
                    match primitive.get_result().which()? {
                        primitives::ConnectionResponseResult::ConnectionDetails(details) => {
                            ConnectionResponseResult::ConnectionDetails(
                                ConnectionDetails::from_primitive(details?)?,
                            )
                        }
                        primitives::ConnectionResponseResult::Error(error) => {
                            ConnectionResponseResult::Error(ConnectionError::from_primitive(
                                error?,
                            )?)
                        }
                    }
                },
            })
        }
    }

    #[derive(Debug)]
    pub enum ConnectionResponseResult {
        ConnectionDetails(ConnectionDetails),
        Error(ConnectionError),
    }

    #[derive(Debug)]
    pub struct ConnectionDetails {
        pub uuid: Vec<u8>,
        pub location_name: String,
        pub tunnel_is_remotely_managed: bool,
    }

    impl ConnectionDetails {
        pub fn to_primitive(&self, builder: primitives::ConnectionDetails::Builder) {
            let mut builder = builder;

            builder.set_uuid(&self.uuid);
            builder.set_location_name(&self.location_name);
            builder.set_tunnel_is_remotely_managed(self.tunnel_is_remotely_managed);
        }
    }

    impl ConnectionDetails {
        pub fn from_primitive(primitive: primitives::ConnectionDetails::Reader) -> Result<Self> {
            Ok(Self {
                uuid: primitive.get_uuid()?.to_vec(),
                location_name: primitive.get_location_name()?.to_string(),
                tunnel_is_remotely_managed: primitive.get_tunnel_is_remotely_managed(),
            })
        }
    }

    #[derive(Debug)]
    pub struct ConnectionError {
        pub cause: String,
        pub retry_after: i64, // in nanoseconds
        pub should_retry: bool,
    }

    impl ConnectionError {
        pub fn to_primitive(&self, builder: primitives::ConnectionError::Builder) {
            let mut builder = builder;

            builder.set_cause(&self.cause);
            builder.set_retry_after(self.retry_after);
            builder.set_should_retry(self.should_retry);
        }
    }

    impl ConnectionError {
        pub fn from_primitive(primitive: primitives::ConnectionError::Reader) -> Result<Self> {
            Ok(Self {
                cause: primitive.get_cause()?.to_string(),
                retry_after: primitive.get_retry_after(),
                should_retry: primitive.get_should_retry(),
            })
        }
    }

    #[derive(Debug)]
    pub struct TunnelAuth {
        pub account_tag: String,
        pub tunnel_secret: Vec<u8>,
    }

    impl TunnelAuth {
        pub fn to_primitive(&self, builder: primitives::TunnelAuth::Builder) {
            let mut builder = builder;

            builder.set_account_tag(&self.account_tag);
            builder.set_tunnel_secret(&self.tunnel_secret);
        }
    }

    impl TunnelAuth {
        pub fn from_primitive(primitive: primitives::TunnelAuth::Reader) -> Result<Self> {
            Ok(Self {
                account_tag: primitive.get_account_tag()?.to_string(),
                tunnel_secret: primitive.get_tunnel_secret()?.to_vec(),
            })
        }
    }

    #[derive(Debug)]
    pub struct RegisterUdpSessionResponse {
        pub err: String,
        pub spans: Vec<u8>,
    }

    impl RegisterUdpSessionResponse {
        pub fn to_primitive(&self, builder: primitives::RegisterUdpSessionResponse::Builder) {
            let mut builder = builder;

            builder.set_err(&self.err);
            builder.set_spans(&self.spans);
        }
    }

    impl RegisterUdpSessionResponse {
        pub fn from_primitive(
            primitive: primitives::RegisterUdpSessionResponse::Reader,
        ) -> Result<Self> {
            Ok(Self {
                err: primitive.get_err()?.to_string(),
                spans: primitive.get_spans()?.to_vec(),
            })
        }
    }

    #[derive(Debug)]
    pub struct UpdateConfigurationResponse {
        pub latest_applied_version: i32,
        pub err: String,
    }

    impl UpdateConfigurationResponse {
        pub fn to_primitive(&self, builder: primitives::UpdateConfigurationResponse::Builder) {
            let mut builder = builder;

            builder.set_latest_applied_version(self.latest_applied_version);
            builder.set_err(&self.err);
        }
    }

    impl UpdateConfigurationResponse {
        pub fn from_primitive(
            primitive: primitives::UpdateConfigurationResponse::Reader,
        ) -> Result<Self> {
            Ok(Self {
                latest_applied_version: primitive.get_latest_applied_version(),
                err: primitive.get_err()?.to_string(),
            })
        }
    }

    pub fn vec_string_to_primitive(vec: &Vec<String>, builder: capnp::text_list::Builder) {
        let mut builder = builder;
        for (i, s) in vec.iter().enumerate() {
            builder.set(i as u32, s);
        }
    }

    pub fn vec_string_from_primitive(reader: capnp::text_list::Reader) -> Result<Vec<String>> {
        let mut vec = Vec::new();
        for i in 0..reader.len() {
            vec.push(reader.get(i)?.to_string());
        }
        Ok(vec)
    }
}

pub mod interfaces {
    #![allow(dead_code, unused_variables, unused_mut)]

    use super::primitives;
    use super::structs;

    pub mod registration_server {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Debug)]
        pub struct RegisterConnectionParams {
            pub auth: TunnelAuth,
            pub tunnel_id: Vec<u8>,
            pub conn_index: u8,
            pub options: ConnectionOptions,
        }

        impl RegisterConnectionParams {
            pub fn to_primitive(&self, builder: primitives::RegisterConnectionParams::Builder) {
                let mut builder = builder;

                self.auth.to_primitive(builder.reborrow().init_auth());
                builder.set_tunnel_id(&self.tunnel_id);
                builder.set_conn_index(self.conn_index);
                self.options.to_primitive(builder.reborrow().init_options());
            }
        }

        impl RegisterConnectionParams {
            pub fn from_primitive(
                primitive: primitives::RegisterConnectionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    auth: TunnelAuth::from_primitive(primitive.get_auth()?)?,
                    tunnel_id: primitive.get_tunnel_id()?.to_vec(),
                    conn_index: primitive.get_conn_index(),
                    options: ConnectionOptions::from_primitive(primitive.get_options()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct RegisterConnectionResults {
            pub result: ConnectionResponse,
        }

        impl RegisterConnectionResults {
            pub fn to_primitive(&self, builder: primitives::RegisterConnectionResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl RegisterConnectionResults {
            pub fn from_primitive(
                primitive: primitives::RegisterConnectionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: ConnectionResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct UnregisterConnectionParams {}

        impl UnregisterConnectionParams {
            pub fn to_primitive(&self, builder: primitives::UnregisterConnectionParams::Builder) {
                let mut builder = builder;
            }
        }

        impl UnregisterConnectionParams {
            pub fn from_primitive(
                primitive: primitives::UnregisterConnectionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Debug)]
        pub struct UnregisterConnectionResults {}

        impl UnregisterConnectionResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterConnectionResults::Builder) {
                let mut builder = builder;
            }
        }

        impl UnregisterConnectionResults {
            pub fn from_primitive(
                primitive: primitives::UnregisterConnectionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Debug)]
        pub struct UpdateLocalConfigurationParams {
            pub config: Vec<u8>,
        }

        impl UpdateLocalConfigurationParams {
            pub fn to_primitive(
                &self,
                builder: primitives::UpdateLocalConfigurationParams::Builder,
            ) {
                let mut builder = builder;

                builder.set_config(&self.config);
            }
        }

        impl UpdateLocalConfigurationParams {
            pub fn from_primitive(
                primitive: primitives::UpdateLocalConfigurationParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    config: primitive.get_config()?.to_vec(),
                })
            }
        }

        #[derive(Debug)]
        pub struct UpdateLocalConfigurationResults {}

        impl UpdateLocalConfigurationResults {
            pub fn to_primitive(
                &self,
                builder: primitives::UpdateLocalConfigurationResults::Builder,
            ) {
                let mut builder = builder;
            }
        }

        impl UpdateLocalConfigurationResults {
            pub fn from_primitive(
                primitive: primitives::UpdateLocalConfigurationResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }
    }

    pub mod tunnel_server {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Debug)]
        pub struct RegisterTunnelParams {
            pub origin_cert: Vec<u8>,
            pub hostname: String,
            pub options: RegistrationOptions,
        }

        impl RegisterTunnelParams {
            pub fn to_primitive(&self, builder: primitives::RegisterTunnelParams::Builder) {
                let mut builder = builder;

                builder.set_origin_cert(&self.origin_cert);
                builder.set_hostname(&self.hostname);
                self.options.to_primitive(builder.reborrow().init_options());
            }
        }

        impl RegisterTunnelParams {
            pub fn from_primitive(
                primitive: primitives::RegisterTunnelParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    origin_cert: primitive.get_origin_cert()?.to_vec(),
                    hostname: primitive.get_hostname()?.to_string(),
                    options: RegistrationOptions::from_primitive(primitive.get_options()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct RegisterTunnelResults {
            pub result: TunnelRegistration,
        }

        impl RegisterTunnelResults {
            pub fn to_primitive(&self, builder: primitives::RegisterTunnelResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl RegisterTunnelResults {
            pub fn from_primitive(
                primitive: primitives::RegisterTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: TunnelRegistration::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct GetServerInfoParams {}

        impl GetServerInfoParams {
            pub fn to_primitive(&self, builder: primitives::GetServerInfoParams::Builder) {
                let mut builder = builder;
            }
        }

        impl GetServerInfoParams {
            pub fn from_primitive(
                primitive: primitives::GetServerInfoParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Debug)]
        pub struct GetServerInfoResults {
            pub result: ServerInfo,
        }

        impl GetServerInfoResults {
            pub fn to_primitive(&self, builder: primitives::GetServerInfoResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl GetServerInfoResults {
            pub fn from_primitive(
                primitive: primitives::GetServerInfoResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: ServerInfo::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct UnregisterTunnelParams {
            pub grace_period_nano_sec: i64,
        }

        impl UnregisterTunnelParams {
            pub fn to_primitive(&self, builder: primitives::UnregisterTunnelParams::Builder) {
                let mut builder = builder;

                builder.set_grace_period_nano_sec(self.grace_period_nano_sec);
            }
        }

        impl UnregisterTunnelParams {
            pub fn from_primitive(
                primitive: primitives::UnregisterTunnelParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    grace_period_nano_sec: primitive.get_grace_period_nano_sec(),
                })
            }
        }

        #[derive(Debug)]
        pub struct UnregisterTunnelResults {}

        impl UnregisterTunnelResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterTunnelResults::Builder) {
                let mut builder = builder;
            }
        }

        impl UnregisterTunnelResults {
            pub fn from_primitive(
                primitive: primitives::UnregisterTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Debug)]
        pub struct ObsoleteDeclarativeTunnelConnectParams {}

        impl ObsoleteDeclarativeTunnelConnectParams {
            pub fn to_primitive(
                &self,
                builder: primitives::ObsoleteDeclarativeTunnelConnectParams::Builder,
            ) {
                let mut builder = builder;
            }
        }

        impl ObsoleteDeclarativeTunnelConnectParams {
            pub fn from_primitive(
                primitive: primitives::ObsoleteDeclarativeTunnelConnectParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Debug)]
        pub struct ObsoleteDeclarativeTunnelConnectResults {}

        impl ObsoleteDeclarativeTunnelConnectResults {
            pub fn to_primitive(
                &self,
                builder: primitives::ObsoleteDeclarativeTunnelConnectResults::Builder,
            ) {
                let mut builder = builder;
            }
        }

        impl ObsoleteDeclarativeTunnelConnectResults {
            pub fn from_primitive(
                primitive: primitives::ObsoleteDeclarativeTunnelConnectResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Debug)]
        pub struct AuthenticateParams {
            pub origin_cert: Vec<u8>,
            pub hostname: String,
            pub options: RegistrationOptions,
        }

        impl AuthenticateParams {
            pub fn to_primitive(&self, builder: primitives::AuthenticateParams::Builder) {
                let mut builder = builder;

                builder.set_origin_cert(&self.origin_cert);
                builder.set_hostname(&self.hostname);
                self.options.to_primitive(builder.reborrow().init_options());
            }
        }

        impl AuthenticateParams {
            pub fn from_primitive(
                primitive: primitives::AuthenticateParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    origin_cert: primitive.get_origin_cert()?.to_vec(),
                    hostname: primitive.get_hostname()?.to_string(),
                    options: RegistrationOptions::from_primitive(primitive.get_options()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct AuthenticateResults {
            pub result: AuthenticationResponse,
        }

        impl AuthenticateResults {
            pub fn to_primitive(&self, builder: primitives::AuthenticateResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl AuthenticateResults {
            pub fn from_primitive(
                primitive: primitives::AuthenticateResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: AuthenticationResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct ReconnectTunnelParams {
            pub jwt: Vec<u8>,
            pub event_digest: Vec<u8>,
            pub conn_digest: Vec<u8>,
            pub hostname: String,
            pub options: RegistrationOptions,
        }

        impl ReconnectTunnelParams {
            pub fn to_primitive(&self, builder: primitives::ReconnectTunnelParams::Builder) {
                let mut builder = builder;

                builder.set_jwt(&self.jwt);
                builder.set_event_digest(&self.event_digest);
                builder.set_conn_digest(&self.conn_digest);
                builder.set_hostname(&self.hostname);
                self.options.to_primitive(builder.reborrow().init_options());
            }
        }

        impl ReconnectTunnelParams {
            pub fn from_primitive(
                primitive: primitives::ReconnectTunnelParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    jwt: primitive.get_jwt()?.to_vec(),
                    event_digest: primitive.get_event_digest()?.to_vec(),
                    conn_digest: primitive.get_conn_digest()?.to_vec(),
                    hostname: primitive.get_hostname()?.to_string(),
                    options: RegistrationOptions::from_primitive(primitive.get_options()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct ReconnectTunnelResults {
            pub result: TunnelRegistration,
        }

        impl ReconnectTunnelResults {
            pub fn to_primitive(&self, builder: primitives::ReconnectTunnelResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl ReconnectTunnelResults {
            pub fn from_primitive(
                primitive: primitives::ReconnectTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: TunnelRegistration::from_primitive(primitive.get_result()?)?,
                })
            }
        }
    }

    pub mod session_manager {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Debug)]
        pub struct RegisterUdpSessionParams {
            pub session_id: Vec<u8>,
            pub dst_ip: Vec<u8>,
            pub close_after_idle_hint: i64,
            pub trace_context: String,
        }

        impl RegisterUdpSessionParams {
            pub fn to_primitive(&self, builder: primitives::RegisterUdpSessionParams::Builder) {
                let mut builder = builder;

                builder.set_session_id(&self.session_id);
                builder.set_dst_ip(&self.dst_ip);
                builder.set_close_after_idle_hint(self.close_after_idle_hint);
                builder.set_trace_context(&self.trace_context);
            }
        }

        impl RegisterUdpSessionParams {
            pub fn from_primitive(
                primitive: primitives::RegisterUdpSessionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    session_id: primitive.get_session_id()?.to_vec(),
                    dst_ip: primitive.get_dst_ip()?.to_vec(),
                    close_after_idle_hint: primitive.get_close_after_idle_hint(),
                    trace_context: primitive.get_trace_context()?.to_string(),
                })
            }
        }

        #[derive(Debug)]
        pub struct RegisterUdpSessionResults {
            pub result: RegisterUdpSessionResponse,
        }

        impl RegisterUdpSessionResults {
            pub fn to_primitive(&self, builder: primitives::RegisterUdpSessionResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl RegisterUdpSessionResults {
            pub fn from_primitive(
                primitive: primitives::RegisterUdpSessionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: RegisterUdpSessionResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Debug)]
        pub struct UnregisterUdpSessionParams {
            pub session_id: Vec<u8>,
            pub message: String,
        }

        impl UnregisterUdpSessionParams {
            pub fn to_primitive(&self, builder: primitives::UnregisterUdpSessionParams::Builder) {
                let mut builder = builder;

                builder.set_session_id(&self.session_id);
                builder.set_message(&self.message);
            }
        }

        impl UnregisterUdpSessionParams {
            pub fn from_primitive(
                primitive: primitives::UnregisterUdpSessionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    session_id: primitive.get_session_id()?.to_vec(),
                    message: primitive.get_message()?.to_string(),
                })
            }
        }

        #[derive(Debug)]
        pub struct UnregisterUdpSessionResults {}

        impl UnregisterUdpSessionResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterUdpSessionResults::Builder) {
                let mut builder = builder;
            }
        }

        impl UnregisterUdpSessionResults {
            pub fn from_primitive(
                primitive: primitives::UnregisterUdpSessionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }
    }

    pub mod configuration_manager {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Debug)]
        pub struct UpdateConfigurationParams {
            pub version: i32,
            pub config: Vec<u8>,
        }

        impl UpdateConfigurationParams {
            pub fn to_primitive(&self, builder: primitives::UpdateConfigurationParams::Builder) {
                let mut builder = builder;

                builder.set_version(self.version);
                builder.set_config(&self.config);
            }
        }

        impl UpdateConfigurationParams {
            pub fn from_primitive(
                primitive: primitives::UpdateConfigurationParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    version: primitive.get_version(),
                    config: primitive.get_config()?.to_vec(),
                })
            }
        }

        #[derive(Debug)]
        pub struct UpdateConfigurationResults {
            pub result: UpdateConfigurationResponse,
        }

        impl UpdateConfigurationResults {
            pub fn to_primitive(&self, builder: primitives::UpdateConfigurationResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }
        }

        impl UpdateConfigurationResults {
            pub fn from_primitive(
                primitive: primitives::UpdateConfigurationResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: UpdateConfigurationResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }
    }
}
