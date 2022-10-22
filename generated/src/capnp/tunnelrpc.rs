#![allow(dead_code)]

// This file was not generated and I wrote it.
// However we should likely invest time to make a generator for this file.
// This file is not required to handle capnp messages, however the raw capnp messages are so annoying to work with
// that I wrote this file to make it easier to work with.
// This file is essentially wrappers for capnp messages to convert them into rust structs.
// Also for capnp rpc client/server to provide async functions to send and receive messages, from the struct wrappers (mentioned above).
// The unfortunate reality is the Capnp Rust Library is not very good, and thus each server/client connection must be run on a single thread.
// However this isnt that bad, because we can run a thread per connection. Since capnp is a binary protocol, the way we get the data doesnt matter.

use super::raw::tunnelrpc_capnp;
pub mod primitives {
    #![allow(dead_code)]

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
    #![allow(dead_code)]

    use super::primitives;
    use anyhow::Result;

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    impl Default for RegistrationOptions {
        fn default() -> Self {
            Self {
                client_id: String::new(),
                version: String::new(),
                os: String::new(),
                existing_tunnel_policy: primitives::ExistingTunnelPolicy::Ignore,
                pool_name: String::new(),
                tags: Vec::new(),
                connection_id: 0,
                origin_local_ip: String::new(),
                is_autoupdated: false,
                run_from_terminal: false,
                compression_quality: 0,
                uuid: String::new(),
                num_previous_attempts: 0,
                features: Vec::new(),
            }
        }
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
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
            let builder = message.init_root::<super::primitives::Authentication::Builder>();
            auth.to_primitive(builder);
            let reader = message
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
            let builder = message.init_root::<super::primitives::TunnelRegistration::Builder>();
            tunnel_registration.to_primitive(builder);
            let reader = message
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
            let builder =
                message.init_root::<super::primitives::RegistrationOptions::Builder>();
            registration_options.to_primitive(builder);
            let reader = message
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
            let builder = message.init_root::<super::primitives::Tag::Builder>();
            tag.to_primitive(builder);
            let reader = message
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
            let builder = message.init_root::<super::primitives::ServerInfo::Builder>();
            server_info.to_primitive(builder);
            let reader = message
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
            let builder =
                message.init_root::<super::primitives::AuthenticationResponse::Builder>();
            authentication_response.to_primitive(builder);
            let reader = message
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
            let builder = message.init_root::<super::primitives::ClientInfo::Builder>();
            client_info.to_primitive(builder);
            let reader = message
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
            let builder = message.init_root::<super::primitives::ConnectionOptions::Builder>();
            connection_options.to_primitive(builder);
            let reader  = message
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
            let builder = message.init_root::<super::primitives::ConnectionResponse::Builder>();
            connection_response.to_primitive(builder);
            let reader  = message
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
            let builder = message.init_root::<super::primitives::ConnectionResponse::Builder>();
            connection_response.to_primitive(builder);
            let reader  = message
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
            let builder = message.init_root::<super::primitives::ConnectionDetails::Builder>();
            connection_details.to_primitive(builder);
            let reader  = message
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
            let builder = message.init_root::<super::primitives::ConnectionError::Builder>();
            connection_error.to_primitive(builder);
            let reader  = message
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
            let builder = message.init_root::<super::primitives::TunnelAuth::Builder>();
            tunnel_auth.to_primitive(builder);
            let reader  = message
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
            let builder =
                message.init_root::<super::primitives::RegisterUdpSessionResponse::Builder>();
            register_udp_session_response.to_primitive(builder);
            let reader  = message
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
            let builder =
                message.init_root::<super::primitives::UpdateConfigurationResponse::Builder>();
            update_tunnel_response.to_primitive(builder);
            let reader  = message
                .get_root_as_reader::<super::primitives::UpdateConfigurationResponse::Reader>()
                .unwrap();
            let update_tunnel_response2 =
                super::UpdateConfigurationResponse::from_primitive(reader).unwrap();
            assert_eq!(update_tunnel_response, update_tunnel_response2);
        }
    }
}

pub mod interfaces {
    #![allow(dead_code)]

    use super::primitives;
    use super::structs;

    pub mod registration_server {
        #![allow(dead_code)]

        use crate::capnp::raw::tunnelrpc_capnp;
        use async_trait::async_trait;
        use capnp::capability::Promise;
        use capnp_rpc::{twoparty::VatId, RpcSystem};

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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
        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterConnectionParams {}

        impl UnregisterConnectionParams {
            pub fn to_primitive(&self, builder: primitives::UnregisterConnectionParams::Builder) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::UnregisterConnectionParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterConnectionResults {}

        impl UnregisterConnectionResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterConnectionResults::Builder) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::UnregisterConnectionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct UpdateLocalConfigurationResults {}

        impl UpdateLocalConfigurationResults {
            pub fn to_primitive(
                &self,
                builder: primitives::UpdateLocalConfigurationResults::Builder,
            ) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::UpdateLocalConfigurationResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        pub mod client {
            use super::*;

            #[derive(Clone)]
            pub struct Client {
                inner: tunnelrpc_capnp::registration_server::Client,
            }

            impl Client {
                pub fn new(inner: tunnelrpc_capnp::registration_server::Client) -> Self {
                    Self { inner }
                }

                pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
                    Self::new(system.bootstrap(VatId::Server))
                }

                pub async fn register_connection(
                    &self,
                    request: RegisterConnectionParams,
                ) -> Result<RegisterConnectionResults> {
                    let mut req = self.inner.register_connection_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;

                    let response = response.get()?;

                    RegisterConnectionResults::from_primitive(response)
                }

                pub async fn unregister_connection(
                    &self,
                    request: UnregisterConnectionParams,
                ) -> Result<UnregisterConnectionResults> {
                    let mut req = self.inner.unregister_connection_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    UnregisterConnectionResults::from_primitive(response)
                }

                pub async fn update_local_configuration(
                    &self,
                    request: UpdateLocalConfigurationParams,
                ) -> Result<UpdateLocalConfigurationResults> {
                    let mut req = self.inner.update_local_configuration_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    UpdateLocalConfigurationResults::from_primitive(response)
                }
            }
        }

        pub mod server {
            use crate::capnp::rpc::{server_async_wrapper, ServerFactory};

            use super::*;

            #[async_trait]
            pub trait Client: Send + Sync {
                async fn register_connection(
                    &self,
                    _request: RegisterConnectionParams,
                ) -> Result<RegisterConnectionResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn unregister_connection(
                    &self,
                    _request: UnregisterConnectionParams,
                ) -> Result<UnregisterConnectionResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn update_local_configuration(
                    &self,
                    _request: UpdateLocalConfigurationParams,
                ) -> Result<UpdateLocalConfigurationResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                fn build(self) -> tunnelrpc_capnp::registration_server::Client
                where
                    Self: Sized + Clone + 'static,
                {
                    capnp_rpc::new_client(Box::<dyn ServerFactory<Self>>::from(self))
                }
            }

            impl<T: Client + Clone + 'static> tunnelrpc_capnp::registration_server::Server
                for Box<dyn ServerFactory<T>>
            {
                fn register_connection(
                    &mut self,
                    params: tunnelrpc_capnp::registration_server::RegisterConnectionParams,
                    mut results: tunnelrpc_capnp::registration_server::RegisterConnectionResults,
                ) -> Promise<(), ::capnp::Error> {
                    server_async_wrapper!(RegisterConnectionParams, register_connection [self, params, results]);
                }

                fn unregister_connection(
                    &mut self,
                    params: tunnelrpc_capnp::registration_server::UnregisterConnectionParams,
                    mut results: tunnelrpc_capnp::registration_server::UnregisterConnectionResults,
                ) -> Promise<(), ::capnp::Error> {
                    server_async_wrapper!(UnregisterConnectionParams, unregister_connection [self, params, results]);
                }

                fn update_local_configuration(
                    &mut self,
                    params: tunnelrpc_capnp::registration_server::UpdateLocalConfigurationParams,
                    mut results: tunnelrpc_capnp::registration_server::UpdateLocalConfigurationResults,
                ) -> Promise<(), ::capnp::Error> {
                    server_async_wrapper!(UpdateLocalConfigurationParams, update_local_configuration [self, params, results]);
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use std::sync::Arc;

            use tokio::sync::Mutex;

            use crate::capnp::rpc::tests::setup_mock_networks;
            use crate::capnp::tunnelrpc::structs;

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
                let builder =
                    message.init_root::<super::primitives::RegisterConnectionParams::Builder>();
                register_connection_params.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<super::primitives::RegisterConnectionResults::Builder>();
                register_connection_request.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<super::primitives::UnregisterConnectionParams::Builder>();
                unregister_connection_params.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<super::primitives::UnregisterConnectionResults::Builder>();
                unregister_connection_results.to_primitive(builder);
                let reader  = message
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
                let builder = message
                    .init_root::<super::primitives::UpdateLocalConfigurationParams::Builder>();
                update_local_configuration_params.to_primitive(builder);
                let reader  = message
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
                let builder = message
                    .init_root::<super::primitives::UpdateLocalConfigurationResults::Builder>(
                );
                update_local_configuration_results.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::UpdateLocalConfigurationResults::Reader>()
                    .unwrap();
                let update_local_configuration_results2 =
                    super::UpdateLocalConfigurationResults::from_primitive(reader).unwrap();
                assert_eq!(
                    update_local_configuration_results,
                    update_local_configuration_results2
                );
            }

            #[derive(Clone)]
            struct RegistrationServer<P, R> {
                send: tokio::sync::mpsc::Sender<P>,
                recv: Arc<Mutex<tokio::sync::mpsc::Receiver<Result<R, capnp::Error>>>>,
            }

            #[tokio::test]
            async fn test_client_server_register_connection() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory =
                            RegistrationServer<RegisterConnectionParams, RegisterConnectionResults>;

                        use crate::capnp::tunnelrpc::interfaces::registration_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn register_connection(
                                &self,
                                params: RegisterConnectionParams,
                            ) -> Result<RegisterConnectionResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = RegisterConnectionParams {
                            auth: structs::TunnelAuth {
                                account_tag: "account_tag".to_string(),
                                tunnel_secret: "tunnel_secret".to_string().into_bytes(),
                            },
                            conn_index: 0,
                            tunnel_id: "tunnel_id".to_string().into_bytes(),
                            options: structs::ConnectionOptions {
                                replace_existing: true,
                                compression_quality: 3,
                                num_previous_attempts: 5,
                                client: structs::ClientInfo {
                                    arch: "arch".to_string(),
                                    version: "version".to_string(),
                                    client_id: "client_id".to_string().into_bytes(),
                                    features: vec!["feature".to_string()],
                                },
                                origin_local_ip: "origin_local_ip".to_string().into_bytes(),
                            },
                        };

                        let result = RegisterConnectionResults {
                            result: structs::ConnectionResponse {
                                result: structs::ConnectionResponseResult::ConnectionDetails(
                                    structs::ConnectionDetails {
                                        location_name: "location_name".to_string(),
                                        tunnel_is_remotely_managed: true,
                                        uuid: uuid::Uuid::new_v4().into_bytes().to_vec(),
                                    },
                                ),
                            },
                        };

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.register_connection(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.register_connection(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unregister_connection() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = RegistrationServer<
                            UnregisterConnectionParams,
                            UnregisterConnectionResults,
                        >;

                        use crate::capnp::tunnelrpc::interfaces::registration_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn unregister_connection(
                                &self,
                                params: UnregisterConnectionParams,
                            ) -> Result<UnregisterConnectionResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Ok(UnregisterConnectionResults::default()))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.unregister_connection(Default::default()).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), Default::default());
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.unregister_connection(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_update_local_configuration() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = RegistrationServer<
                            UpdateLocalConfigurationParams,
                            UpdateLocalConfigurationResults,
                        >;

                        use crate::capnp::tunnelrpc::interfaces::registration_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn update_local_configuration(
                                &self,
                                params: UpdateLocalConfigurationParams,
                            ) -> Result<UpdateLocalConfigurationResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = UpdateLocalConfigurationParams {
                            config: "config".to_string().into_bytes(),
                        };

                        let h = {
                            let send_results = send_results.clone();
                            let params = params.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params.clone());
                                send_results.send(Ok(Default::default())).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.update_local_configuration(params.clone()).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), Default::default());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            let params = params.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params.clone());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.update_local_configuration(params).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unimplemented() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        #[derive(Clone)]
                        struct Server {}

                        use crate::capnp::tunnelrpc::interfaces::registration_server::server::Client;

                        #[async_trait]
                        impl Client for Server {}

                        let server_factory = Server {}.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let resp = client.register_connection(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.unregister_connection(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.update_local_configuration(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );
                    })
                    .await;
            }
        }
    }

    pub mod tunnel_server {
        #![allow(dead_code)]

        use super::primitives;
        use super::structs::*;
        use anyhow::Result;

        use crate::capnp::{raw::tunnelrpc_capnp, tunnelrpc::interfaces::registration_server};
        use async_trait::async_trait;
        use capnp::capability::Promise;
        use capnp_rpc::{twoparty::VatId, RpcSystem};

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct GetServerInfoParams {}

        impl GetServerInfoParams {
            pub fn to_primitive(&self, builder: primitives::GetServerInfoParams::Builder) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::GetServerInfoParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterTunnelResults {}

        impl UnregisterTunnelResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterTunnelResults::Builder) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::UnregisterTunnelResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct ObsoleteDeclarativeTunnelConnectParams {}

        impl ObsoleteDeclarativeTunnelConnectParams {
            pub fn to_primitive(
                &self,
                builder: primitives::ObsoleteDeclarativeTunnelConnectParams::Builder,
            ) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::ObsoleteDeclarativeTunnelConnectParams::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct ObsoleteDeclarativeTunnelConnectResults {}

        impl ObsoleteDeclarativeTunnelConnectResults {
            pub fn to_primitive(
                &self,
                builder: primitives::ObsoleteDeclarativeTunnelConnectResults::Builder,
            ) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::ObsoleteDeclarativeTunnelConnectResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        pub mod client {
            use super::*;

            #[derive(Clone)]
            pub struct Client {
                inner: tunnelrpc_capnp::tunnel_server::Client,
                registration: registration_server::client::Client,
            }

            impl Client {
                pub fn new(
                    inner: tunnelrpc_capnp::tunnel_server::Client,
                    registration: registration_server::client::Client,
                ) -> Self {
                    Self {
                        inner,
                        registration,
                    }
                }

                pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
                    Self::new(
                        system.bootstrap(VatId::Server),
                        registration_server::client::Client::new(system.bootstrap(VatId::Server)),
                    )
                }

                pub fn get_registration_client(&self) -> &registration_server::client::Client {
                    &self.registration
                }

                pub async fn register_tunnel(
                    &self,
                    request: RegisterTunnelParams,
                ) -> Result<RegisterTunnelResults> {
                    let mut req = self.inner.register_tunnel_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    RegisterTunnelResults::from_primitive(response)
                }

                pub async fn get_server_info(
                    &self,
                    request: GetServerInfoParams,
                ) -> Result<GetServerInfoResults> {
                    let mut req = self.inner.get_server_info_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    GetServerInfoResults::from_primitive(response)
                }

                pub async fn unregister_tunnel(
                    &self,
                    request: UnregisterTunnelParams,
                ) -> Result<UnregisterTunnelResults> {
                    let mut req = self.inner.unregister_tunnel_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    UnregisterTunnelResults::from_primitive(response)
                }

                pub async fn obsolete_declarative_tunnel_connect(
                    &self,
                    request: ObsoleteDeclarativeTunnelConnectParams,
                ) -> Result<ObsoleteDeclarativeTunnelConnectResults> {
                    let mut req = self.inner.obsolete_declarative_tunnel_connect_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    ObsoleteDeclarativeTunnelConnectResults::from_primitive(response)
                }

                pub async fn authenticate(
                    &self,
                    request: AuthenticateParams,
                ) -> Result<AuthenticateResults> {
                    let mut req = self.inner.authenticate_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    AuthenticateResults::from_primitive(response)
                }

                pub async fn reconnect_tunnel(
                    &self,
                    request: ReconnectTunnelParams,
                ) -> Result<ReconnectTunnelResults> {
                    let mut req = self.inner.reconnect_tunnel_request();
                    request.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    ReconnectTunnelResults::from_primitive(response)
                }
            }
        }

        pub mod server {
            use crate::capnp::rpc::{server_async_wrapper, ServerFactory};

            use super::*;

            #[async_trait]
            pub trait Client: Send + Sync {
                async fn register_tunnel(
                    &self,
                    _request: RegisterTunnelParams,
                ) -> Result<RegisterTunnelResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn get_server_info(
                    &self,
                    _request: GetServerInfoParams,
                ) -> Result<GetServerInfoResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn unregister_tunnel(
                    &self,
                    _request: UnregisterTunnelParams,
                ) -> Result<UnregisterTunnelResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn obsolete_declarative_tunnel_connect(
                    &self,
                    _request: ObsoleteDeclarativeTunnelConnectParams,
                ) -> Result<ObsoleteDeclarativeTunnelConnectResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn authenticate(
                    &self,
                    _request: AuthenticateParams,
                ) -> Result<AuthenticateResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn reconnect_tunnel(
                    &self,
                    _request: ReconnectTunnelParams,
                ) -> Result<ReconnectTunnelResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                fn build(self) -> tunnelrpc_capnp::tunnel_server::Client
                where
                    Self: Sized + Clone + registration_server::server::Client + 'static,
                {
                    capnp_rpc::new_client(Box::<dyn ServerFactory<Self>>::from(self))
                }
            }

            impl<T: Client + Clone + registration_server::server::Client + 'static>
                tunnelrpc_capnp::tunnel_server::Server for Box<dyn ServerFactory<T>>
            {
                fn register_tunnel(
                    &mut self,
                    params: tunnelrpc_capnp::tunnel_server::RegisterTunnelParams,
                    mut results: tunnelrpc_capnp::tunnel_server::RegisterTunnelResults,
                ) -> Promise<(), capnp::Error> {
                    server_async_wrapper!(RegisterTunnelParams, register_tunnel [self, params, results]);
                }
                fn get_server_info(
                    &mut self,
                    params: tunnelrpc_capnp::tunnel_server::GetServerInfoParams,
                    mut results: tunnelrpc_capnp::tunnel_server::GetServerInfoResults,
                ) -> Promise<(), capnp::Error> {
                    server_async_wrapper!(GetServerInfoParams, get_server_info [self, params, results]);
                }
                fn unregister_tunnel(
                    &mut self,
                    params: tunnelrpc_capnp::tunnel_server::UnregisterTunnelParams,
                    mut results: tunnelrpc_capnp::tunnel_server::UnregisterTunnelResults,
                ) -> Promise<(), capnp::Error> {
                    server_async_wrapper!(UnregisterTunnelParams, unregister_tunnel [self, params, results]);
                }
                fn obsolete_declarative_tunnel_connect(
                    &mut self,
                    params: tunnelrpc_capnp::tunnel_server::ObsoleteDeclarativeTunnelConnectParams,
                    mut results: tunnelrpc_capnp::tunnel_server::ObsoleteDeclarativeTunnelConnectResults<>,
                ) -> Promise<(), capnp::Error> {
                    server_async_wrapper!(ObsoleteDeclarativeTunnelConnectParams, obsolete_declarative_tunnel_connect [self, params, results]);
                }
                fn authenticate(
                    &mut self,
                    params: tunnelrpc_capnp::tunnel_server::AuthenticateParams,
                    mut results: tunnelrpc_capnp::tunnel_server::AuthenticateResults,
                ) -> Promise<(), capnp::Error> {
                    server_async_wrapper!(AuthenticateParams, authenticate [self, params, results]);
                }
                fn reconnect_tunnel(
                    &mut self,
                    params: tunnelrpc_capnp::tunnel_server::ReconnectTunnelParams,
                    mut results: tunnelrpc_capnp::tunnel_server::ReconnectTunnelResults,
                ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
                    server_async_wrapper!(ReconnectTunnelParams, reconnect_tunnel [self, params, results]);
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use std::sync::Arc;

            use tokio::sync::Mutex;

            use crate::capnp::rpc::tests::setup_mock_networks;
            use crate::capnp::tunnelrpc::structs;

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
                let builder = message.init_root::<primitives::RegisterTunnelParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
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
                let builder = message.init_root::<primitives::RegisterTunnelResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::RegisterTunnelResults::Reader>()
                    .unwrap();
                let results2 = super::RegisterTunnelResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_get_server_info_params() {
                let params = GetServerInfoParams {};
                let mut message = capnp::message::Builder::new_default();
                let builder = message.init_root::<primitives::GetServerInfoParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
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
                let builder = message.init_root::<primitives::GetServerInfoResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<primitives::UnregisterTunnelParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::UnregisterTunnelParams::Reader>()
                    .unwrap();
                let params2 = super::UnregisterTunnelParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_unregister_tunnel_results() {
                let results = UnregisterTunnelResults {};

                let mut message = capnp::message::Builder::new_default();
                let builder =
                    message.init_root::<primitives::UnregisterTunnelResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::UnregisterTunnelResults::Reader>()
                    .unwrap();
                let results2 = super::UnregisterTunnelResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[test]
            fn test_obsolete_declarative_tunnel_connect_params() {
                let params = ObsoleteDeclarativeTunnelConnectParams {};

                let mut message = capnp::message::Builder::new_default();
                let builder = message
                    .init_root::<primitives::ObsoleteDeclarativeTunnelConnectParams::Builder>(
                );
                params.to_primitive(builder);
                let reader  = message
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
                let builder = message
                    .init_root::<primitives::ObsoleteDeclarativeTunnelConnectResults::Builder>(
                );
                results.to_primitive(builder);
                let reader  = message
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
                let builder = message.init_root::<primitives::AuthenticateParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
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
                let builder = message.init_root::<primitives::AuthenticateResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
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
                let builder = message.init_root::<primitives::ReconnectTunnelParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<primitives::ReconnectTunnelResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::ReconnectTunnelResults::Reader>()
                    .unwrap();
                let results2 = super::ReconnectTunnelResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[derive(Clone)]
            struct TunnelServer<P, R> {
                send: tokio::sync::mpsc::Sender<P>,
                recv: Arc<Mutex<tokio::sync::mpsc::Receiver<Result<R, capnp::Error>>>>,
            }

            #[tokio::test]
            async fn test_client_server_register_tunnel() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = TunnelServer<RegisterTunnelParams, RegisterTunnelResults>;

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn register_tunnel(
                                &self,
                                params: RegisterTunnelParams,
                            ) -> Result<RegisterTunnelResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = RegisterTunnelParams {
                            hostname: "hostname".to_string(),
                            options: RegistrationOptions {
                                existing_tunnel_policy:
                                    super::primitives::ExistingTunnelPolicy::Ignore,
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
                            origin_cert: "origin_cert".to_string().into_bytes(),
                        };

                        let result = RegisterTunnelResults {
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

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.register_tunnel(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.register_tunnel(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_get_server_info() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = TunnelServer<GetServerInfoParams, GetServerInfoResults>;

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn get_server_info(
                                &self,
                                params: GetServerInfoParams,
                            ) -> Result<GetServerInfoResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = GetServerInfoParams {};

                        let result = GetServerInfoResults {
                            result: structs::ServerInfo {
                                location_name: "location_name".to_string(),
                            },
                        };

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.get_server_info(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.get_server_info(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unregister_tunnel() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory =
                            TunnelServer<UnregisterTunnelParams, UnregisterTunnelResults>;

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn unregister_tunnel(
                                &self,
                                params: UnregisterTunnelParams,
                            ) -> Result<UnregisterTunnelResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = UnregisterTunnelParams {
                            grace_period_nano_sec: 123,
                        };

                        let result = UnregisterTunnelResults {};

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.unregister_tunnel(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.unregister_tunnel(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_obsolete_declarative_tunnel_connect() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = TunnelServer<
                            ObsoleteDeclarativeTunnelConnectParams,
                            ObsoleteDeclarativeTunnelConnectResults,
                        >;

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn obsolete_declarative_tunnel_connect(
                                &self,
                                params: ObsoleteDeclarativeTunnelConnectParams,
                            ) -> Result<ObsoleteDeclarativeTunnelConnectResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = ObsoleteDeclarativeTunnelConnectParams {};
                        let result = ObsoleteDeclarativeTunnelConnectResults {};

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.obsolete_declarative_tunnel_connect(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client
                            .obsolete_declarative_tunnel_connect(Default::default())
                            .await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_authenticate() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = TunnelServer<AuthenticateParams, AuthenticateResults>;

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn authenticate(
                                &self,
                                params: AuthenticateParams,
                            ) -> Result<AuthenticateResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = AuthenticateParams {
                            hostname: "hostname".to_string(),
                            origin_cert: "origin_cert".to_string().into_bytes(),
                            options: structs::RegistrationOptions {
                                client_id: "client_id".to_string(),
                                origin_local_ip: "origin_local_ip".to_string(),
                                os: "os".to_string(),
                                pool_name: "pool_name".to_string(),
                                uuid: "uuid".to_string(),
                                version: "version".to_string(),
                                compression_quality: 1,
                                connection_id: 50,
                                num_previous_attempts: 10,
                                is_autoupdated: true,
                                run_from_terminal: true,
                                tags: vec![structs::Tag {
                                    name: "name".to_string(),
                                    value: "value".to_string(),
                                }],
                                features: vec!["feature1".to_string(), "feature2".to_string()],
                                existing_tunnel_policy: primitives::ExistingTunnelPolicy::Ignore,
                            },
                        };

                        let result = AuthenticateResults {
                            result: structs::AuthenticationResponse {
                                hours_until_refresh: 1,
                                jwt: "jwt".to_string().into_bytes(),
                                permanent_err: "permanent_err".to_string(),
                                retryable_err: "retryable_err".to_string(),
                            },
                        };

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.authenticate(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.authenticate(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_reconnect_tunnel() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = TunnelServer<ReconnectTunnelParams, ReconnectTunnelResults>;

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn reconnect_tunnel(
                                &self,
                                params: ReconnectTunnelParams,
                            ) -> Result<ReconnectTunnelResults, capnp::Error>
                            {
                                self.send.send(params).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = ReconnectTunnelParams {
                            conn_digest: "conn_digest".to_string().into_bytes(),
                            event_digest: "event_digest".to_string().into_bytes(),
                            jwt: "jwt".to_string().into_bytes(),
                            hostname: "hostname".to_string(),
                            options: structs::RegistrationOptions {
                                client_id: "client_id".to_string(),
                                origin_local_ip: "origin_local_ip".to_string(),
                                os: "os".to_string(),
                                pool_name: "pool_name".to_string(),
                                uuid: "uuid".to_string(),
                                version: "version".to_string(),
                                compression_quality: 1,
                                connection_id: 50,
                                num_previous_attempts: 10,
                                is_autoupdated: true,
                                run_from_terminal: true,
                                tags: vec![structs::Tag {
                                    name: "name".to_string(),
                                    value: "value".to_string(),
                                }],
                                features: vec!["feature1".to_string(), "feature2".to_string()],
                                existing_tunnel_policy: primitives::ExistingTunnelPolicy::Ignore,
                            },
                        };

                        let result = ReconnectTunnelResults {
                            result: structs::TunnelRegistration {
                                conn_digest: "conn_digest".to_string().into_bytes(),
                                event_digest: "event_digest".to_string().into_bytes(),
                                err: "err".to_string(),
                                tunnel_i_d: "tunnel_i_d".to_string(),
                                url: "url".to_string(),
                                log_lines: vec!["log_line1".to_string(), "log_line2".to_string()],
                                permanent_failure: true,
                                retry_after_seconds: 20,
                            },
                        };

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.reconnect_tunnel(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.reconnect_tunnel(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unimplemented() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        #[derive(Clone)]
                        struct Factory {}

                        use crate::capnp::tunnelrpc::interfaces::tunnel_server::server::Client;

                        #[async_trait]
                        impl Client for Factory {}

                        #[async_trait]
                        impl registration_server::server::Client for Factory {}

                        let server_factory = Factory {}.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let resp = client.register_tunnel(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.get_server_info(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.unregister_tunnel(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client
                            .obsolete_declarative_tunnel_connect(Default::default())
                            .await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.authenticate(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.reconnect_tunnel(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );
                    })
                    .await;
            }
        }
    }

    pub mod session_manager {
        #![allow(dead_code)]

        use super::primitives;
        use super::structs::*;
        use crate::capnp::raw::tunnelrpc_capnp;
        use anyhow::Result;
        use async_trait::async_trait;
        use capnp::capability::Promise;
        use capnp_rpc::{twoparty::VatId, RpcSystem};

        use crate::capnp::rpc::{server_async_wrapper, ServerFactory};

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
        pub struct UnregisterUdpSessionResults {}

        impl UnregisterUdpSessionResults {
            pub fn to_primitive(&self, builder: primitives::UnregisterUdpSessionResults::Builder) {
                let mut _builder = builder;
            }

            pub fn from_primitive(
                _primitive: primitives::UnregisterUdpSessionResults::Reader,
            ) -> Result<Self> {
                Ok(Self {})
            }
        }

        pub mod client {
            use super::*;

            #[derive(Clone)]
            pub struct Client {
                inner: tunnelrpc_capnp::session_manager::Client,
            }

            impl Client {
                pub fn new(inner: tunnelrpc_capnp::session_manager::Client) -> Self {
                    Self { inner }
                }

                pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
                    Self::new(system.bootstrap(VatId::Server))
                }

                pub async fn register_udp_session(
                    &self,
                    params: RegisterUdpSessionParams,
                ) -> Result<RegisterUdpSessionResults> {
                    let mut req = self.inner.register_udp_session_request();
                    params.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    RegisterUdpSessionResults::from_primitive(response)
                }
                pub async fn unregister_udp_session(
                    &self,
                    params: UnregisterUdpSessionParams,
                ) -> Result<UnregisterUdpSessionResults> {
                    let mut req = self.inner.unregister_udp_session_request();
                    params.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    UnregisterUdpSessionResults::from_primitive(response)
                }
            }
        }

        pub mod server {
            use super::*;

            #[async_trait]
            pub trait Client: Send + Sync {
                async fn register_udp_session(
                    &self,
                    _request: RegisterUdpSessionParams,
                ) -> Result<RegisterUdpSessionResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                async fn unregister_udp_session(
                    &self,
                    _request: UnregisterUdpSessionParams,
                ) -> Result<UnregisterUdpSessionResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                fn build(self) -> tunnelrpc_capnp::session_manager::Client
                where
                    Self: Sized + Clone + 'static,
                {
                    capnp_rpc::new_client(Box::<dyn ServerFactory<Self>>::from(self))
                }
            }

            impl<T: Client + Clone + 'static> tunnelrpc_capnp::session_manager::Server
                for Box<dyn ServerFactory<T>>
            {
                fn register_udp_session(
                    &mut self,
                    params: tunnelrpc_capnp::session_manager::RegisterUdpSessionParams,
                    mut results: tunnelrpc_capnp::session_manager::RegisterUdpSessionResults,
                ) -> Promise<(), ::capnp::Error> {
                    server_async_wrapper!(RegisterUdpSessionParams, register_udp_session [self, params, results]);
                }

                fn unregister_udp_session(
                    &mut self,
                    params: tunnelrpc_capnp::session_manager::UnregisterUdpSessionParams,
                    mut results: tunnelrpc_capnp::session_manager::UnregisterUdpSessionResults,
                ) -> Promise<(), ::capnp::Error> {
                    server_async_wrapper!(UnregisterUdpSessionParams, unregister_udp_session [self, params, results]);
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use std::sync::Arc;

            use tokio::sync::Mutex;

            use crate::capnp::rpc::tests::setup_mock_networks;

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
                let builder =
                    message.init_root::<primitives::RegisterUdpSessionParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<primitives::RegisterUdpSessionResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<primitives::UnregisterUdpSessionParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::UnregisterUdpSessionParams::Reader>()
                    .unwrap();
                let params2 = super::UnregisterUdpSessionParams::from_primitive(reader).unwrap();
                assert_eq!(params, params2);
            }

            #[test]
            fn test_unregister_udp_session_results() {
                let results = UnregisterUdpSessionResults {};

                let mut message = capnp::message::Builder::new_default();
                let builder =
                    message.init_root::<primitives::UnregisterUdpSessionResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::UnregisterUdpSessionResults::Reader>()
                    .unwrap();
                let results2 = super::UnregisterUdpSessionResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[derive(Clone)]
            struct SessionManager<P, R> {
                send: tokio::sync::mpsc::Sender<P>,
                recv: Arc<Mutex<tokio::sync::mpsc::Receiver<Result<R, capnp::Error>>>>,
            }

            #[tokio::test]
            async fn test_client_server_register_udp_session() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory =
                            SessionManager<RegisterUdpSessionParams, RegisterUdpSessionResults>;

                        use crate::capnp::tunnelrpc::interfaces::session_manager::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn register_udp_session(
                                &self,
                                request: RegisterUdpSessionParams,
                            ) -> Result<RegisterUdpSessionResults, capnp::Error>
                            {
                                self.send.send(request).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = RegisterUdpSessionParams {
                            dst_ip: vec![1, 2, 3],
                            session_id: vec![4, 5, 6],
                            close_after_idle_hint: 1,
                            trace_context: "trace_context".to_string(),
                        };

                        let result = RegisterUdpSessionResults {
                            result: RegisterUdpSessionResponse {
                                err: "error".to_string(),
                                spans: vec![1, 2, 3, 4],
                            },
                        };

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.register_udp_session(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.register_udp_session(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unregister_udp_session() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory =
                            SessionManager<UnregisterUdpSessionParams, UnregisterUdpSessionResults>;

                        use crate::capnp::tunnelrpc::interfaces::session_manager::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn unregister_udp_session(
                                &self,
                                request: UnregisterUdpSessionParams,
                            ) -> Result<UnregisterUdpSessionResults, capnp::Error>
                            {
                                self.send.send(request).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }
                        .build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = UnregisterUdpSessionParams {
                            session_id: vec![1, 2, 3],
                            message: "message".to_string(),
                        };

                        let result = UnregisterUdpSessionResults {};

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.unregister_udp_session(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.unregister_udp_session(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unimplemented() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        use crate::capnp::tunnelrpc::interfaces::session_manager::server::Client;

                        #[derive(Clone)]
                        struct Server {}

                        #[async_trait]
                        impl Client for Server {}

                        let server_factory = Server {}.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let resp = client.register_udp_session(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );

                        let resp = client.unregister_udp_session(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );
                    })
                    .await;
            }
        }
    }

    pub mod configuration_manager {
        #![allow(dead_code)]

        use super::primitives;
        use super::structs::*;
        use crate::capnp::raw::tunnelrpc_capnp;
        use anyhow::Result;
        use async_trait::async_trait;
        use capnp::capability::Promise;
        use capnp_rpc::{twoparty::VatId, RpcSystem};

        use crate::capnp::rpc::{server_async_wrapper, ServerFactory};

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        #[derive(Default, Clone, Debug, PartialEq, Eq)]
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

        pub mod client {
            use super::*;

            #[derive(Clone)]
            pub struct Client {
                inner: tunnelrpc_capnp::configuration_manager::Client,
            }

            impl Client {
                pub fn new(inner: tunnelrpc_capnp::configuration_manager::Client) -> Self {
                    Self { inner }
                }

                pub fn new_from_system(system: &mut RpcSystem<VatId>) -> Self {
                    Self::new(system.bootstrap(VatId::Server))
                }

                pub async fn update_configuration(
                    &self,
                    params: UpdateConfigurationParams,
                ) -> Result<UpdateConfigurationResults> {
                    let mut req = self.inner.update_configuration_request();
                    params.to_primitive(req.get());

                    let response = req.send().promise.await?;
                    let response = response.get()?;

                    UpdateConfigurationResults::from_primitive(response)
                }
            }
        }

        pub mod server {
            use super::*;

            #[async_trait]
            pub trait Client: Send + Sync {
                async fn update_configuration(
                    &self,
                    _request: UpdateConfigurationParams,
                ) -> Result<UpdateConfigurationResults, capnp::Error> {
                    Err(capnp::Error::unimplemented("unimplemented".to_string()))
                }

                fn build(self) -> tunnelrpc_capnp::configuration_manager::Client
                where
                    Self: Sized + Clone + 'static,
                {
                    capnp_rpc::new_client(Box::<dyn ServerFactory<Self>>::from(self))
                }
            }

            impl<T: Client + Clone + 'static> tunnelrpc_capnp::configuration_manager::Server
                for Box<dyn ServerFactory<T>>
            {
                fn update_configuration(
                    &mut self,
                    params: tunnelrpc_capnp::configuration_manager::UpdateConfigurationParams,
                    mut results: tunnelrpc_capnp::configuration_manager::UpdateConfigurationResults,
                ) -> Promise<(), ::capnp::Error> {
                    server_async_wrapper!(UpdateConfigurationParams, update_configuration [self, params, results]);
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use std::sync::Arc;

            use tokio::sync::Mutex;

            use crate::capnp::{rpc::tests::setup_mock_networks, tunnelrpc::structs};

            use super::*;

            #[test]
            fn test_update_configuration_params() {
                let params = UpdateConfigurationParams {
                    version: 1,
                    config: vec![1, 2, 3],
                };

                let mut message = capnp::message::Builder::new_default();
                let builder =
                    message.init_root::<primitives::UpdateConfigurationParams::Builder>();
                params.to_primitive(builder);
                let reader  = message
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
                let builder =
                    message.init_root::<primitives::UpdateConfigurationResults::Builder>();
                results.to_primitive(builder);
                let reader  = message
                    .get_root_as_reader::<super::primitives::UpdateConfigurationResults::Reader>()
                    .unwrap();
                let results2 = super::UpdateConfigurationResults::from_primitive(reader).unwrap();
                assert_eq!(results, results2);
            }

            #[derive(Clone)]
            struct ConfigurationManager<P, R> {
                send: tokio::sync::mpsc::Sender<P>,
                recv: Arc<Mutex<tokio::sync::mpsc::Receiver<Result<R, capnp::Error>>>>,
            }

            #[tokio::test]
            async fn test_client_server_update_configuration() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        type Factory = ConfigurationManager<
                            UpdateConfigurationParams,
                            UpdateConfigurationResults,
                        >;

                        use crate::capnp::tunnelrpc::interfaces::configuration_manager::server::Client;

                        #[async_trait]
                        impl Client for Factory {
                            async fn update_configuration(
                                &self,
                                request: UpdateConfigurationParams,
                            ) -> Result<UpdateConfigurationResults, capnp::Error>
                            {
                                self.send.send(request).await.unwrap();
                                self.recv.lock().await.recv().await.unwrap()
                            }
                        }

                        let (send_params, mut recv_params) = tokio::sync::mpsc::channel(1);
                        let (send_results, recv_results) = tokio::sync::mpsc::channel(1);
                        let server_factory = Factory {
                            send: send_params,
                            recv: Arc::new(Mutex::new(recv_results)),
                        }.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let params = UpdateConfigurationParams {
                            config: "config".to_string().into_bytes(),
                            version: 1,
                        };

                        let result = UpdateConfigurationResults {
                            result: structs::UpdateConfigurationResponse {
                                err: "error".to_string(),
                                latest_applied_version: 1,
                            },
                        };

                        let h = {
                            let params = params.clone();
                            let result = result.clone();
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), params);
                                send_results.send(Ok(result)).await.unwrap();
                                recv_params
                            })
                        };

                        let resp = client.update_configuration(params).await;
                        assert!(resp.is_ok());
                        assert_eq!(resp.unwrap(), result);
                        assert!(h.is_finished());

                        let h = h.await;
                        assert!(h.is_ok());

                        let mut recv_params = h.unwrap();

                        let h = {
                            let send_results = send_results.clone();
                            tokio::spawn(async move {
                                assert_eq!(recv_params.recv().await.unwrap(), Default::default());
                                send_results
                                    .send(Err(capnp::Error::failed(
                                        "failure is tested for".to_string(),
                                    )))
                                    .await
                                    .unwrap();
                                recv_params
                            })
                        };

                        let resp = client.update_configuration(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Failed: remote exception: failure is tested for"
                        );
                        assert!(h.is_finished());
                    })
                    .await;
            }

            #[tokio::test]
            async fn test_client_server_unimplemented() {
                tokio::task::LocalSet::new()
                    .run_until(async {
                        let (client_network, server_network) = setup_mock_networks().await;
                        let mut system = RpcSystem::new(client_network, None);
                        let client = client::Client::new_from_system(&mut system);
                        let _client_runner = tokio::task::spawn_local(system);

                        #[derive(Clone)]
                        struct Server {}

                        use crate::capnp::tunnelrpc::interfaces::configuration_manager::server::Client;

                        #[async_trait]
                        impl Client for Server {}

                        let server_factory = Server {}.build();
                        let system =
                            RpcSystem::new(server_network, Some(server_factory.client));
                        let _server_runner = tokio::task::spawn_local(system);

                        let resp = client.update_configuration(Default::default()).await;
                        assert!(resp.is_err());
                        assert_eq!(
                            resp.unwrap_err().to_string(),
                            "Unimplemented: remote exception: unimplemented"
                        );
                    })
                    .await;
            }
        }
    }
}
