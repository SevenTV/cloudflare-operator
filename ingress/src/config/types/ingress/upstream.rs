use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Container<T> {
    pub weight: Option<u32>,

    #[serde(flatten)]
    pub upstream: T,
}

pub mod http {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(tag = "kind", rename_all = "kebab-case")]
    pub enum Upstreams {
        #[serde(rename = "http")]
        Http(HttpUpstream),
    }

    #[derive(Debug, Deserialize, Default, Clone)]
    pub struct HttpUpstream {
        pub host: String,
        pub port: u16,
    }
}
