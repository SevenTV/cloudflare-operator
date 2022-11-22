use crate::config::types as config;
use bitflags::bitflags;

bitflags! {
    pub struct Method: u16 {
        const NONE = 0;
        const GET = 1 << 0;
        const POST = 1 << 1;
        const PUT =  1 << 2;
        const DELETE = 1 << 3;
        const PATCH = 1 << 4;
        const HEAD = 1 << 5;
        const OPTIONS = 1 << 6;
        const CONNECT = 1 << 7;
        const TRACE = 1 << 8;
        const ALL = (1 << 8) - 1;
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Exact(String),
    Prefix(String),
    Regex(String),
}

// Type conversions from config types to router types

impl From<Vec<String>> for Method {
    fn from(methods: Vec<String>) -> Self {
        let mut method = Method::NONE;
        for m in methods {
            match m.as_str() {
                "GET" => method |= Method::GET,
                "POST" => method |= Method::POST,
                "PUT" => method |= Method::PUT,
                "DELETE" => method |= Method::DELETE,
                "PATCH" => method |= Method::PATCH,
                "HEAD" => method |= Method::HEAD,
                "OPTIONS" => method |= Method::OPTIONS,
                "CONNECT" => method |= Method::CONNECT,
                "TRACE" => method |= Method::TRACE,
                _ => {}
            }
        }
        method
    }
}

impl From<config::ingress::http::Path> for (Type, Method) {
    fn from(path: config::ingress::http::Path) -> Self {
        match path {
            config::ingress::http::Path::String(path) => (Type::Prefix(path), Method::ALL),
            config::ingress::http::Path::Struct(path) => {
                let method = path.methods.map_or(Method::ALL, |m| m.into());

                match path.kind {
                    Some(config::ingress::http::PathKind::Exact) => {
                        (Type::Exact(path.path), method)
                    }
                    Some(config::ingress::http::PathKind::Prefix) => {
                        (Type::Prefix(path.path), method)
                    }
                    Some(config::ingress::http::PathKind::Regex) => {
                        (Type::Regex(path.path), method)
                    }
                    None => (Type::Prefix(path.path), method),
                }
            }
        }
    }
}
