use serde::Deserialize;

use super::upstream;

#[derive(Debug, Deserialize, Clone)]
pub struct Container<R> {
    #[serde(flatten)]
    pub rule: R,

    #[serde(flatten)]
    pub hostname: Option<HostnameUnion>,

    #[serde(flatten)]
    pub path: Option<PathUnion>,

    pub upstreams: Option<Vec<upstream::Container<upstream::http::Upstreams>>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum HostnameUnion {
    Single { hostname: Hostname },
    Multi { hostnames: Vec<Hostname> },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Hostname {
    // todo support regex and wildcard (glob) hostnames
    String(String),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PathUnion {
    Single { path: Path },
    Multi { paths: Vec<Path> },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Path {
    String(String),
    Struct(PathStruct),
}

#[derive(Debug, Deserialize, Clone)]
pub struct PathStruct {
    pub kind: Option<PathKind>,
    pub path: String,
    pub methods: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PathKind {
    Exact,
    Prefix,
    Regex,
}

pub mod cloudflare {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Default, Clone)]
    pub struct Ingress {
        pub tunnel_id: uuid::Uuid,
    }
}
