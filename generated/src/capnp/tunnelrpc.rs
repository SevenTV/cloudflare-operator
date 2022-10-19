#![allow(dead_code, unused_variables, unused_mut)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.

use super::raw::tunnelrpc_capnp;
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

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::Authentication::Reader) -> Result<Self> {
            Ok(Self {
                key: primitive.get_key()?.to_string(),
                email: primitive.get_email()?.to_string(),
                origin_c_a_key: primitive.get_origin_c_a_key()?.to_string(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::RegistrationOptions::Reader) -> Result<Self> {
            let t = primitive.get_tags()?;
            let tags = utils::vec_type_from_primitive!(t, Tag);

            Ok(Self {
                client_id: primitive.get_client_id()?.to_string(),
                version: primitive.get_version()?.to_string(),
                os: primitive.get_os()?.to_string(),
                existing_tunnel_policy: primitive.get_existing_tunnel_policy()?,
                pool_name: primitive.get_pool_name()?.to_string(),
                tags,
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

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::Tag::Reader) -> Result<Self> {
            Ok(Self {
                name: primitive.get_name()?.to_string(),
                value: primitive.get_value()?.to_string(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ServerInfo {
        pub location_name: String,
    }

    impl ServerInfo {
        pub fn to_primitive(&self, builder: primitives::ServerInfo::Builder) {
            let mut builder = builder;

            builder.set_location_name(&self.location_name);
        }

        pub fn from_primitive(primitive: primitives::ServerInfo::Reader) -> Result<Self> {
            Ok(Self {
                location_name: primitive.get_location_name()?.to_string(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::ClientInfo::Reader) -> Result<Self> {
            Ok(Self {
                client_id: primitive.get_client_id()?.to_vec(),
                features: vec_string_from_primitive(primitive.get_features()?)?,
                version: primitive.get_version()?.to_string(),
                arch: primitive.get_arch()?.to_string(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ConnectionResponse {
        pub result: ConnectionResponseResult,
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ConnectionResponseResult {
        ConnectionDetails(ConnectionDetails),
        Error(ConnectionError),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::ConnectionDetails::Reader) -> Result<Self> {
            Ok(Self {
                uuid: primitive.get_uuid()?.to_vec(),
                location_name: primitive.get_location_name()?.to_string(),
                tunnel_is_remotely_managed: primitive.get_tunnel_is_remotely_managed(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::ConnectionError::Reader) -> Result<Self> {
            Ok(Self {
                cause: primitive.get_cause()?.to_string(),
                retry_after: primitive.get_retry_after(),
                should_retry: primitive.get_should_retry(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(primitive: primitives::TunnelAuth::Reader) -> Result<Self> {
            Ok(Self {
                account_tag: primitive.get_account_tag()?.to_string(),
                tunnel_secret: primitive.get_tunnel_secret()?.to_vec(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(
            primitive: primitives::RegisterUdpSessionResponse::Reader,
        ) -> Result<Self> {
            Ok(Self {
                err: primitive.get_err()?.to_string(),
                spans: primitive.get_spans()?.to_vec(),
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
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

        pub fn from_primitive(
            primitive: primitives::UpdateConfigurationResponse::Reader,
        ) -> Result<Self> {
            Ok(Self {
                latest_applied_version: primitive.get_latest_applied_version(),
                err: primitive.get_err()?.to_string(),
            })
        }
    }

    pub fn vec_string_to_primitive(vec: &[String], builder: capnp::text_list::Builder) {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_authentication() {
            let auth = Authentication {
                email: "some@email".to_string(),
                key: "key123".to_string(),
                origin_c_a_key: "secret_key".to_string(),
            };
            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::Authentication::Builder>();
            auth.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::Authentication::Reader>()
                .unwrap();
            let auth2 = super::Authentication::from_primitive(reader).unwrap();
            assert_eq!(auth, auth2);
        }

        #[test]
        fn test_tunnel_registration() {
            let tunnel_registration = TunnelRegistration {
                conn_digest: "conn_digest".to_string().into_bytes(),
                err: "err".to_string(),
                event_digest: "event_digest".to_string().into_bytes(),
                log_lines: vec!["log_line1".to_string(), "log_line2".to_string()],
                permanent_failure: true,
                retry_after_seconds: 123,
                tunnel_i_d: "tunnel_id".to_string(),
                url: "url".to_string(),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::TunnelRegistration::Builder>();
            tunnel_registration.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::TunnelRegistration::Reader>()
                .unwrap();
            let tunnel_registration2 = super::TunnelRegistration::from_primitive(reader).unwrap();
            assert_eq!(tunnel_registration, tunnel_registration2);
        }

        #[test]
        fn test_registration_options() {
            let registration_options = RegistrationOptions {
                client_id: "client_id".to_string(),
                version: "version".to_string(),
                os: "os".to_string(),
                existing_tunnel_policy: super::primitives::ExistingTunnelPolicy::Ignore,
                pool_name: "pool_name".to_string(),
                tags: vec![Tag {
                    name: "tag1".to_string(),
                    value: "value1".to_string(),
                }],
                connection_id: 5,
                origin_local_ip: "origin_local_ip".to_string(),
                is_autoupdated: true,
                run_from_terminal: true,
                compression_quality: 5,
                uuid: "uuid".to_string(),
                num_previous_attempts: 5,
                features: vec!["feature1".to_string(), "feature2".to_string()],
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder =
                message.init_root::<super::primitives::RegistrationOptions::Builder>();
            registration_options.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::RegistrationOptions::Reader>()
                .unwrap();
            let registration_options2 = super::RegistrationOptions::from_primitive(reader).unwrap();
            assert_eq!(registration_options, registration_options2);
        }

        #[test]
        fn test_tag() {
            let tag = Tag {
                name: "name".to_string(),
                value: "value".to_string(),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::Tag::Builder>();
            tag.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::Tag::Reader>()
                .unwrap();
            let tag2 = super::Tag::from_primitive(reader).unwrap();
            assert_eq!(tag, tag2);
        }

        #[test]
        fn test_server_info() {
            let server_info = ServerInfo {
                location_name: "location_name".to_string(),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ServerInfo::Builder>();
            server_info.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ServerInfo::Reader>()
                .unwrap();
            let server_info2 = super::ServerInfo::from_primitive(reader).unwrap();
            assert_eq!(server_info, server_info2);
        }

        #[test]
        fn test_authentication_response() {
            let authentication_response = AuthenticationResponse {
                hours_until_refresh: 5,
                jwt: "jwt".to_string().into_bytes(),
                permanent_err: "permanent_err".to_string(),
                retryable_err: "retryable_err".to_string(),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder =
                message.init_root::<super::primitives::AuthenticationResponse::Builder>();
            authentication_response.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::AuthenticationResponse::Reader>()
                .unwrap();
            let authentication_response2 =
                super::AuthenticationResponse::from_primitive(reader).unwrap();
            assert_eq!(authentication_response, authentication_response2);
        }

        #[test]
        fn test_client_info() {
            let client_info = ClientInfo {
                client_id: "client_id".to_string().into_bytes(),
                features: vec!["feature1".to_string(), "feature2".to_string()],
                version: "version".to_string(),
                arch: "arch".to_string(),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ClientInfo::Builder>();
            client_info.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ClientInfo::Reader>()
                .unwrap();
            let client_info2 = super::ClientInfo::from_primitive(reader).unwrap();
            assert_eq!(client_info, client_info2);
        }

        #[test]
        fn test_connection_options() {
            let connection_options = ConnectionOptions {
                client: ClientInfo {
                    client_id: "client_id".to_string().into_bytes(),
                    features: vec!["feature1".to_string(), "feature2".to_string()],
                    version: "version".to_string(),
                    arch: "arch".to_string(),
                },
                compression_quality: 5,
                num_previous_attempts: 5,
                origin_local_ip: "origin_local_ip".to_string().into_bytes(),
                replace_existing: true,
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ConnectionOptions::Builder>();
            connection_options.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ConnectionOptions::Reader>()
                .unwrap();
            let connection_options2 = super::ConnectionOptions::from_primitive(reader).unwrap();
            assert_eq!(connection_options, connection_options2);
        }

        #[test]
        fn test_connection_response() {
            let connection_response = ConnectionResponse {
                result: ConnectionResponseResult::ConnectionDetails(ConnectionDetails {
                    uuid: "uuid".to_string().into_bytes(),
                    location_name: "YYZ".to_string(),
                    tunnel_is_remotely_managed: true,
                }),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ConnectionResponse::Builder>();
            connection_response.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ConnectionResponse::Reader>()
                .unwrap();
            let connection_response2 = super::ConnectionResponse::from_primitive(reader).unwrap();
            assert_eq!(connection_response, connection_response2);

            let connection_response = ConnectionResponse {
                result: ConnectionResponseResult::Error(ConnectionError {
                    cause: "cause".to_string(),
                    retry_after: 50,
                    should_retry: false,
                }),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ConnectionResponse::Builder>();
            connection_response.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ConnectionResponse::Reader>()
                .unwrap();
            let connection_response2 = super::ConnectionResponse::from_primitive(reader).unwrap();
            assert_eq!(connection_response, connection_response2);
        }

        #[test]
        fn test_connection_details() {
            let connection_details = ConnectionDetails {
                uuid: "uuid".to_string().into_bytes(),
                location_name: "YYZ".to_string(),
                tunnel_is_remotely_managed: true,
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ConnectionDetails::Builder>();
            connection_details.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ConnectionDetails::Reader>()
                .unwrap();
            let connection_details2 = super::ConnectionDetails::from_primitive(reader).unwrap();
            assert_eq!(connection_details, connection_details2);
        }

        #[test]
        fn test_connection_error() {
            let connection_error = ConnectionError {
                cause: "cause".to_string(),
                retry_after: 50,
                should_retry: false,
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::ConnectionError::Builder>();
            connection_error.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::ConnectionError::Reader>()
                .unwrap();
            let connection_error2 = super::ConnectionError::from_primitive(reader).unwrap();
            assert_eq!(connection_error, connection_error2);
        }

        #[test]
        fn test_tunnel_auth() {
            let tunnel_auth = TunnelAuth {
                account_tag: "account_tag".to_string(),
                tunnel_secret: "tunnel_secret".to_string().into_bytes(),
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder = message.init_root::<super::primitives::TunnelAuth::Builder>();
            tunnel_auth.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::TunnelAuth::Reader>()
                .unwrap();
            let tunnel_auth2 = super::TunnelAuth::from_primitive(reader).unwrap();
            assert_eq!(tunnel_auth, tunnel_auth2);
        }

        #[test]
        fn test_register_udp_session_response() {
            let register_udp_session_response = RegisterUdpSessionResponse {
                err: "err".to_string(),
                spans: vec![1, 2, 3],
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder =
                message.init_root::<super::primitives::RegisterUdpSessionResponse::Builder>();
            register_udp_session_response.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::RegisterUdpSessionResponse::Reader>()
                .unwrap();
            let register_udp_session_response2 =
                super::RegisterUdpSessionResponse::from_primitive(reader).unwrap();
            assert_eq!(
                register_udp_session_response,
                register_udp_session_response2
            );
        }

        #[test]
        fn test_update_configuration_response() {
            let update_tunnel_response = UpdateConfigurationResponse {
                err: "err".to_string(),
                latest_applied_version: 5,
            };

            let mut message = capnp::message::Builder::new_default();
            let mut builder =
                message.init_root::<super::primitives::UpdateConfigurationResponse::Builder>();
            update_tunnel_response.to_primitive(builder);
            let mut reader = message
                .get_root_as_reader::<super::primitives::UpdateConfigurationResponse::Reader>()
                .unwrap();
            let update_tunnel_response2 =
                super::UpdateConfigurationResponse::from_primitive(reader).unwrap();
            assert_eq!(update_tunnel_response, update_tunnel_response2);
        }
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

        #[derive(Clone, Debug, PartialEq, Eq)]
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

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct RegisterConnectionResults {
            pub result: ConnectionResponse,
        }

        impl RegisterConnectionResults {
            pub fn to_primitive(&self, builder: primitives::RegisterConnectionResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::RegisterConnectionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: ConnectionResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterConnectionParams {}

        impl UnregisterConnectionParams {
            pub fn to_primitive(&self, builder: primitives::UnregisterConnectionParams::Builder) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::UnregisterConnectionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterConnectionResults {}

        impl UnregisterConnectionResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterConnectionResults::Builder) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::UnregisterConnectionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
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

            pub fn from_primitive(
                primitive: primitives::UpdateLocalConfigurationParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    config: primitive.get_config()?.to_vec(),
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UpdateLocalConfigurationResults {}

        impl UpdateLocalConfigurationResults {
            pub fn to_primitive(
                &self,
                builder: primitives::UpdateLocalConfigurationResults::Builder,
            ) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::UpdateLocalConfigurationResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_register_connection_params() {
                let register_connection_params = RegisterConnectionParams {
                    auth: TunnelAuth {
                        account_tag: "account_tag".to_string(),
                        tunnel_secret: "tunnel_secret".to_string().into_bytes(),
                    },
                    tunnel_id: vec![1, 2, 3],
                    conn_index: 5,
                    options: ConnectionOptions {
                        client: ClientInfo {
                            client_id: "".to_string().into_bytes(),
                            features: vec!["abc".to_string()],
                            version: "version".to_string(),
                            arch: "arch".to_string(),
                        },
                        origin_local_ip: "origin_local_ip".to_string().into_bytes(),
                        replace_existing: true,
                        compression_quality: 5,
                        num_previous_attempts: 5,
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<super::primitives::RegisterConnectionParams::Builder>();
                register_connection_params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::RegisterConnectionParams::Reader>()
                    .unwrap();
                let register_connection_params2 =
                    super::RegisterConnectionParams::from_primitive(reader).unwrap();
                assert_eq!(register_connection_params, register_connection_params2);
            }

            #[test]
            fn test_register_connection_request() {
                let register_connection_request = RegisterConnectionResults {
                    result: ConnectionResponse {
                        result: ConnectionResponseResult::ConnectionDetails(ConnectionDetails {
                            uuid: "uuid".to_string().into_bytes(),
                            location_name: "YYZ".to_string(),
                            tunnel_is_remotely_managed: true,
                        }),
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<super::primitives::RegisterConnectionResults::Builder>();
                register_connection_request.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::RegisterConnectionResults::Reader>()
                    .unwrap();
                let register_connection_request2 =
                    super::RegisterConnectionResults::from_primitive(reader).unwrap();
                assert_eq!(register_connection_request, register_connection_request2);
            }

            #[test]
            fn test_unregister_connection_params() {
                let unregister_connection_params = UnregisterConnectionParams {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<super::primitives::UnregisterConnectionParams::Builder>();
                unregister_connection_params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UnregisterConnectionParams::Reader>()
                    .unwrap();
                let unregister_connection_params2 =
                    super::UnregisterConnectionParams::from_primitive(reader).unwrap();
                assert_eq!(unregister_connection_params, unregister_connection_params2);
            }

            #[test]
            fn test_unregister_connection_results() {
                let unregister_connection_results = UnregisterConnectionResults {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<super::primitives::UnregisterConnectionResults::Builder>();
                unregister_connection_results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UnregisterConnectionResults::Reader>()
                    .unwrap();
                let unregister_connection_results2 =
                    super::UnregisterConnectionResults::from_primitive(reader).unwrap();
                assert_eq!(
                    unregister_connection_results,
                    unregister_connection_results2
                );
            }

            #[test]
            fn test_update_local_configuration_params() {
                let update_local_configuration_params = UpdateLocalConfigurationParams {
                    config: vec![1, 2, 3],
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message
                    .init_root::<super::primitives::UpdateLocalConfigurationParams::Builder>();
                update_local_configuration_params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UpdateLocalConfigurationParams::Reader>()
                    .unwrap();
                let update_local_configuration_params2 =
                    super::UpdateLocalConfigurationParams::from_primitive(reader).unwrap();
                assert_eq!(
                    update_local_configuration_params,
                    update_local_configuration_params2
                );
            }

            #[test]
            fn test_update_local_configuration_results() {
                let update_local_configuration_results = UpdateLocalConfigurationResults {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message
                    .init_root::<super::primitives::UpdateLocalConfigurationResults::Builder>(
                );
                update_local_configuration_results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UpdateLocalConfigurationResults::Reader>()
                    .unwrap();
                let update_local_configuration_results2 =
                    super::UpdateLocalConfigurationResults::from_primitive(reader).unwrap();
                assert_eq!(
                    update_local_configuration_results,
                    update_local_configuration_results2
                );
            }
        }
    }

    pub mod tunnel_server {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Clone, Debug, PartialEq, Eq)]
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

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct RegisterTunnelResults {
            pub result: TunnelRegistration,
        }

        impl RegisterTunnelResults {
            pub fn to_primitive(&self, builder: primitives::RegisterTunnelResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::RegisterTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: TunnelRegistration::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct GetServerInfoParams {}

        impl GetServerInfoParams {
            pub fn to_primitive(&self, builder: primitives::GetServerInfoParams::Builder) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::GetServerInfoParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct GetServerInfoResults {
            pub result: ServerInfo,
        }

        impl GetServerInfoResults {
            pub fn to_primitive(&self, builder: primitives::GetServerInfoResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::GetServerInfoResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: ServerInfo::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterTunnelParams {
            pub grace_period_nano_sec: i64,
        }

        impl UnregisterTunnelParams {
            pub fn to_primitive(&self, builder: primitives::UnregisterTunnelParams::Builder) {
                let mut builder = builder;

                builder.set_grace_period_nano_sec(self.grace_period_nano_sec);
            }

            pub fn from_primitive(
                primitive: primitives::UnregisterTunnelParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    grace_period_nano_sec: primitive.get_grace_period_nano_sec(),
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterTunnelResults {}

        impl UnregisterTunnelResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterTunnelResults::Builder) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::UnregisterTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct ObsoleteDeclarativeTunnelConnectParams {}

        impl ObsoleteDeclarativeTunnelConnectParams {
            pub fn to_primitive(
                &self,
                builder: primitives::ObsoleteDeclarativeTunnelConnectParams::Builder,
            ) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::ObsoleteDeclarativeTunnelConnectParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]

        pub struct ObsoleteDeclarativeTunnelConnectResults {}

        impl ObsoleteDeclarativeTunnelConnectResults {
            pub fn to_primitive(
                &self,
                builder: primitives::ObsoleteDeclarativeTunnelConnectResults::Builder,
            ) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::ObsoleteDeclarativeTunnelConnectResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
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

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct AuthenticateResults {
            pub result: AuthenticationResponse,
        }

        impl AuthenticateResults {
            pub fn to_primitive(&self, builder: primitives::AuthenticateResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::AuthenticateResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: AuthenticationResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
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

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct ReconnectTunnelResults {
            pub result: TunnelRegistration,
        }

        impl ReconnectTunnelResults {
            pub fn to_primitive(&self, builder: primitives::ReconnectTunnelResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::ReconnectTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: TunnelRegistration::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        mod tests {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn test_register_tunnel_params() {
                let params = RegisterTunnelParams {
                    hostname: "test".to_string(),
                    options: RegistrationOptions {
                        existing_tunnel_policy: super::primitives::ExistingTunnelPolicy::Ignore,
                        client_id: "client_id".to_string(),
                        compression_quality: 1,
                        connection_id: 7,
                        features: vec!["123".to_string()],
                        is_autoupdated: true,
                        num_previous_attempts: 2,
                        origin_local_ip: "".to_string(),
                        os: "os".to_string(),
                        pool_name: "pool_name".to_string(),
                        run_from_terminal: true,
                        tags: vec![Tag {
                            name: "tag".to_string(),
                            value: "value".to_string(),
                        }],
                        uuid: "uuid".to_string(),
                        version: "version".to_string(),
                    },
                    origin_cert: vec![1, 2, 3],
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::RegisterTunnelParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::RegisterTunnelParams::Reader>()
                    .unwrap();
                let params2 = super::RegisterTunnelParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_register_tunnel_results() {
                let results = RegisterTunnelResults {
                    result: TunnelRegistration {
                        conn_digest: vec![1, 2, 3],
                        event_digest: vec![4, 5, 6],
                        err: "Error".to_string(),
                        log_lines: vec!["log".to_string()],
                        permanent_failure: true,
                        retry_after_seconds: 1,
                        tunnel_i_d: "tunnel_id".to_string(),
                        url: "url".to_string(),
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::RegisterTunnelResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::RegisterTunnelResults::Reader>()
                    .unwrap();
                let results2 = super::RegisterTunnelResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_get_server_info_params() {
                let params = GetServerInfoParams {};
                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::GetServerInfoParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::GetServerInfoParams::Reader>()
                    .unwrap();
                let params2 = super::GetServerInfoParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_get_server_info_results() {
                let results = GetServerInfoResults {
                    result: ServerInfo {
                        location_name: "location_name".to_string(),
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::GetServerInfoResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::GetServerInfoResults::Reader>()
                    .unwrap();
                let results2 = super::GetServerInfoResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_unregister_tunnel_params() {
                let params = UnregisterTunnelParams {
                    grace_period_nano_sec: 1,
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::UnregisterTunnelParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UnregisterTunnelParams::Reader>()
                    .unwrap();
                let params2 = super::UnregisterTunnelParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_unregister_tunnel_results() {
                let results = UnregisterTunnelResults {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::UnregisterTunnelResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UnregisterTunnelResults::Reader>()
                    .unwrap();
                let results2 = super::UnregisterTunnelResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_obsolete_declarative_tunnel_connect_params() {
                let params = ObsoleteDeclarativeTunnelConnectParams {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message
                    .init_root::<primitives::ObsoleteDeclarativeTunnelConnectParams::Builder>(
                );
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::ObsoleteDeclarativeTunnelConnectParams::Reader>()
                    .unwrap();
                let params2 =
                    super::ObsoleteDeclarativeTunnelConnectParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_obsolete_declarative_tunnel_connect_results() {
                let results = ObsoleteDeclarativeTunnelConnectResults {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message
                    .init_root::<primitives::ObsoleteDeclarativeTunnelConnectResults::Builder>(
                );
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::ObsoleteDeclarativeTunnelConnectResults::Reader>()
                    .unwrap();
                let results2 =
                    super::ObsoleteDeclarativeTunnelConnectResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_authenticate_params() {
                let params = AuthenticateParams {
                    origin_cert: "origin_cert".to_string().into_bytes(),
                    hostname: "hostname".to_string(),
                    options: RegistrationOptions {
                        client_id: "client_id".to_string(),
                        compression_quality: 1,
                        connection_id: 8,
                        existing_tunnel_policy: primitives::ExistingTunnelPolicy::Ignore,
                        features: vec!["feature".to_string()],
                        is_autoupdated: true,
                        num_previous_attempts: 1,
                        origin_local_ip: "origin_local_ip".to_string(),
                        os: "os".to_string(),
                        pool_name: "pool_name".to_string(),
                        run_from_terminal: true,
                        tags: vec![Tag {
                            name: "name".to_string(),
                            value: "value".to_string(),
                        }],
                        uuid: "uuid".to_string(),
                        version: "version".to_string(),
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::AuthenticateParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::AuthenticateParams::Reader>()
                    .unwrap();
                let params2 = super::AuthenticateParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_authenticate_results() {
                let results = AuthenticateResults {
                    result: AuthenticationResponse {
                        permanent_err: "error".to_string(),
                        retryable_err: "retry".to_string(),
                        jwt: "jwt".to_string().into_bytes(),
                        hours_until_refresh: 5,
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::AuthenticateResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::AuthenticateResults::Reader>()
                    .unwrap();
                let results2 = super::AuthenticateResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_reconnect_tunnel_params() {
                let params = ReconnectTunnelParams {
                    jwt: "jwt".to_string().into_bytes(),
                    event_digest: "event_digest".to_string().into_bytes(),
                    conn_digest: "conn_digest".to_string().into_bytes(),
                    hostname: "hostname".to_string(),
                    options: RegistrationOptions {
                        existing_tunnel_policy: super::primitives::ExistingTunnelPolicy::Ignore,
                        client_id: "client_id".to_string(),
                        compression_quality: 1,
                        connection_id: 7,
                        features: vec!["123".to_string()],
                        is_autoupdated: true,
                        num_previous_attempts: 2,
                        origin_local_ip: "".to_string(),
                        os: "os".to_string(),
                        pool_name: "pool_name".to_string(),
                        run_from_terminal: true,
                        tags: vec![Tag {
                            name: "tag".to_string(),
                            value: "value".to_string(),
                        }],
                        uuid: "uuid".to_string(),
                        version: "version".to_string(),
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder = message.init_root::<primitives::ReconnectTunnelParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::ReconnectTunnelParams::Reader>()
                    .unwrap();
                let params2 = super::ReconnectTunnelParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_reconnect_tunnel_results() {
                let results = ReconnectTunnelResults {
                    result: TunnelRegistration {
                        conn_digest: vec![1, 2, 3],
                        event_digest: vec![4, 5, 6],
                        err: "Error".to_string(),
                        log_lines: vec!["log".to_string()],
                        permanent_failure: true,
                        retry_after_seconds: 1,
                        tunnel_i_d: "tunnel_id".to_string(),
                        url: "url".to_string(),
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::ReconnectTunnelResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::ReconnectTunnelResults::Reader>()
                    .unwrap();
                let results2 = super::ReconnectTunnelResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }
        }
    }

    pub mod session_manager {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Clone, Debug, PartialEq, Eq)]
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

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct RegisterUdpSessionResults {
            pub result: RegisterUdpSessionResponse,
        }

        impl RegisterUdpSessionResults {
            pub fn to_primitive(&self, builder: primitives::RegisterUdpSessionResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::RegisterUdpSessionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: RegisterUdpSessionResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
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

            pub fn from_primitive(
                primitive: primitives::UnregisterUdpSessionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    session_id: primitive.get_session_id()?.to_vec(),
                    message: primitive.get_message()?.to_string(),
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterUdpSessionResults {}

        impl UnregisterUdpSessionResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterUdpSessionResults::Builder) {
                let mut builder = builder;
            }

            pub fn from_primitive(
                primitive: primitives::UnregisterUdpSessionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_register_udp_session_params() {
                let params = RegisterUdpSessionParams {
                    session_id: vec![1, 2, 3],
                    dst_ip: vec![4, 5, 6],
                    close_after_idle_hint: 1,
                    trace_context: "trace_context".to_string(),
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::RegisterUdpSessionParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::RegisterUdpSessionParams::Reader>()
                    .unwrap();
                let params2 = super::RegisterUdpSessionParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_register_udp_session_results() {
                let results = RegisterUdpSessionResults {
                    result: RegisterUdpSessionResponse {
                        err: "Error".to_string(),
                        spans: vec![1, 2, 4, 5],
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::RegisterUdpSessionResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::RegisterUdpSessionResults::Reader>()
                    .unwrap();
                let results2 = super::RegisterUdpSessionResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_unregister_udp_session_params() {
                let params = UnregisterUdpSessionParams {
                    session_id: vec![1, 2, 3],
                    message: "message".to_string(),
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::UnregisterUdpSessionParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UnregisterUdpSessionParams::Reader>()
                    .unwrap();
                let params2 = super::UnregisterUdpSessionParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_unregister_udp_session_results() {
                let results = UnregisterUdpSessionResults {};

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::UnregisterUdpSessionResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UnregisterUdpSessionResults::Reader>()
                    .unwrap();
                let results2 = super::UnregisterUdpSessionResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }
        }
    }

    pub mod configuration_manager {
        #![allow(dead_code, unused_variables, unused_mut)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Clone, Debug, PartialEq, Eq)]
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

            pub fn from_primitive(
                primitive: primitives::UpdateConfigurationParams::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    version: primitive.get_version(),
                    config: primitive.get_config()?.to_vec(),
                })
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct UpdateConfigurationResults {
            pub result: UpdateConfigurationResponse,
        }

        impl UpdateConfigurationResults {
            pub fn to_primitive(&self, builder: primitives::UpdateConfigurationResults::Builder) {
                let mut builder = builder;

                self.result.to_primitive(builder.reborrow().init_result());
            }

            pub fn from_primitive(
                primitive: primitives::UpdateConfigurationResults::Reader,
            ) -> Result<Self> {
                Ok(Self {
                    result: UpdateConfigurationResponse::from_primitive(primitive.get_result()?)?,
                })
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_update_configuration_params() {
                let params = UpdateConfigurationParams {
                    version: 1,
                    config: vec![1, 2, 3],
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::UpdateConfigurationParams::Builder>();
                params.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UpdateConfigurationParams::Reader>()
                    .unwrap();
                let params2 = super::UpdateConfigurationParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_update_configuration_results() {
                let results = UpdateConfigurationResults {
                    result: UpdateConfigurationResponse {
                        err: "Error".to_string(),
                        latest_applied_version: 1,
                    },
                };

                let mut message = capnp::message::Builder::new_default();
                let mut builder =
                    message.init_root::<primitives::UpdateConfigurationResults::Builder>();
                results.to_primitive(builder);
                let mut reader = message
                    .get_root_as_reader::<super::primitives::UpdateConfigurationResults::Reader>()
                    .unwrap();
                let results2 = super::UpdateConfigurationResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }
        }
    }
}
