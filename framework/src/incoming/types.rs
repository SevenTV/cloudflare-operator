use std::{str::FromStr, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};
use utils::context::wait::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl FromStr for HttpMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        match s.as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(anyhow!("invalid http method")),
        }
    }
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
    pub is_tls: bool,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

#[async_trait]
pub trait HttpStream: Send + Sync {
    async fn decompose<'a>(
        &mut self,
        timeout: Duration,
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

    #[test]
    fn test_method_from_string() {
        assert_eq!(HttpMethod::from_str("GET").unwrap(), HttpMethod::GET);
        assert_eq!(HttpMethod::from_str("POST").unwrap(), HttpMethod::POST);
        assert_eq!(HttpMethod::from_str("PUT").unwrap(), HttpMethod::PUT);
        assert_eq!(HttpMethod::from_str("DELETE").unwrap(), HttpMethod::DELETE);
        assert_eq!(HttpMethod::from_str("HEAD").unwrap(), HttpMethod::HEAD);
        assert_eq!(
            HttpMethod::from_str("OPTIONS").unwrap(),
            HttpMethod::OPTIONS
        );
        assert_eq!(HttpMethod::from_str("TRACE").unwrap(), HttpMethod::TRACE);
        assert_eq!(
            HttpMethod::from_str("CONNECT").unwrap(),
            HttpMethod::CONNECT
        );
        assert_eq!(HttpMethod::from_str("PATCH").unwrap(), HttpMethod::PATCH);
        assert!(HttpMethod::from_str("FOO").is_err());

        assert_eq!(HttpMethod::from_str("get").unwrap(), HttpMethod::GET);
        assert_eq!(HttpMethod::from_str("post").unwrap(), HttpMethod::POST);
        assert_eq!(HttpMethod::from_str("put").unwrap(), HttpMethod::PUT);
        assert_eq!(HttpMethod::from_str("delete").unwrap(), HttpMethod::DELETE);
        assert_eq!(HttpMethod::from_str("head").unwrap(), HttpMethod::HEAD);
        assert_eq!(
            HttpMethod::from_str("options").unwrap(),
            HttpMethod::OPTIONS
        );
        assert_eq!(HttpMethod::from_str("trace").unwrap(), HttpMethod::TRACE);
        assert_eq!(
            HttpMethod::from_str("connect").unwrap(),
            HttpMethod::CONNECT
        );
        assert_eq!(HttpMethod::from_str("patch").unwrap(), HttpMethod::PATCH);
    }
}
