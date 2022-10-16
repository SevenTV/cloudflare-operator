use std::net::IpAddr;

use uuid::Uuid;

pub enum Ingest {
    CloudflareTunnel(CloudflareTunnelIngest),
}

pub struct CloudflareTunnelIngest {
    pub tunnel_id: Uuid,
    pub client_ip: IpAddr,
}

pub enum RequestType {
    Http(HttpRequest),
}

pub struct Request {
    pub id: Uuid,
    pub ingest: Ingest,
    pub request: RequestType,
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    PATCH,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub hostname: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub is_websocket: bool,
}

#[test]
fn thread_safe() {
    // this function basically just exists to make sure the code compiles and that all these types are thread-safe.
    fn _assert_send_sync<T: Send + Sync>() {}

    _assert_send_sync::<Ingest>();
    _assert_send_sync::<CloudflareTunnelIngest>();
    _assert_send_sync::<RequestType>();
    _assert_send_sync::<Request>();
    _assert_send_sync::<HttpMethod>();
    _assert_send_sync::<HttpRequest>();
}
