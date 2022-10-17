pub mod cloudflare;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Cloudflare = 1,
}

#[derive(Debug, Clone)]
pub enum Auth {
    Cloudflare(cloudflare::Auth),
}

// Type conversions from config types to router types

use crate::config::types as config;

impl From<config::auth::Auth> for Auth {
    fn from(auth: config::auth::Auth) -> Self {
        match auth {
            config::auth::Auth::Cloudflare(auth) => Self::Cloudflare(cloudflare::Auth::from(auth)),
        }
    }
}
