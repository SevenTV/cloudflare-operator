use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};
use utils::context::wait::Context;

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

pub type HandleHttp = Box<Arc<dyn HandleHttpTrait>>;

#[async_trait]
pub trait HandleHttpTrait: Send + Sync {
    async fn handle(
        &self,
        ctx: Context,
        req: HttpRequest,
        stream: Box<dyn HttpStream>,
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub hostname: String,
    pub path: String,
    pub is_websocket: bool,
    pub headers: Vec<(String, String)>,
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

#[async_trait]
pub trait HttpStream: Send + Sync {
    async fn decompose<'a>(
        &mut self,
        resp: Result<HttpResponse>,
    ) -> Result<(
        &mut (dyn AsyncRead + Send + Sync + Unpin),
        &mut (dyn AsyncWrite + Send + Sync + Unpin),
    )>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_safe() {
        // this function basically just exists to make sure the code compiles and that all these types are thread-safe.
        fn _assert_send_sync<T: Send + Sync>() {}

        _assert_send_sync::<HttpMethod>();
        _assert_send_sync::<HttpRequest>();
        _assert_send_sync::<HttpResponse>();
        _assert_send_sync::<HandleHttp>();
        _assert_send_sync::<Box<dyn HttpStream>>();
    }
}
