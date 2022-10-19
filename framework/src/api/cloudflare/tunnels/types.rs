use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Tunnel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub account_tag: String,
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub connections: Vec<TunnelConn>,
    #[serde(default)]
    pub conns_active_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub conns_inactive_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tun_type: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub remote_config: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct TunnelConn {
    #[serde(default)]
    pub colo_name: String,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub is_pending_reconnect: bool,
    #[serde(default)]
    pub origin_ip: String,
    #[serde(default)]
    pub opened_at: DateTime<Utc>,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub client_version: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct TunnelRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tunnel_secret: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct TunnelConnection {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub arch: String,
    #[serde(default)]
    pub conns: Vec<TunnelConn>,
    #[serde(default)]
    pub run_at: DateTime<Utc>,
}
