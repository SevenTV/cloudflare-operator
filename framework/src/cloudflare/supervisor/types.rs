use std::net::IpAddr;

#[derive(Debug, Clone)]
pub(super) struct EdgeRegion {
    pub addrs: Vec<IpAddr>,
    pub hostname: String,
    pub port: u16,
}
pub enum EdgeRegionLocation {
    AUTO,
    US,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) enum Protocol {
    NONE,
    QUIC,
    HTTP2,
}

pub(super) struct TLSSettings {
    pub server_name: String,
    pub next_protos: Vec<String>,
}

impl Protocol {
    pub(super) fn fallback(&self) -> Self {
        match self {
            Protocol::QUIC => Protocol::HTTP2,
            Protocol::HTTP2 => Protocol::NONE,
            Protocol::NONE => Protocol::NONE,
        }
    }

    pub(super) fn tls_settings(&self) -> TLSSettings {
        match self {
            Protocol::QUIC => TLSSettings {
                server_name: "quic.cftunnel.com".to_string(),
                next_protos: vec!["argotunnel".to_string()],
            },
            Protocol::HTTP2 => TLSSettings {
                server_name: "h2.cftunnel.com".to_string(),
                next_protos: vec![],
            },
            Protocol::NONE => panic!("no tls settings for protocol NONE"),
        }
    }
}
