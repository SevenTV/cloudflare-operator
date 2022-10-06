use serde::Deserialize;
use serde_env::from_env;

use super::Config;

#[derive(Debug, Deserialize, Default, Clone)]
struct Env {
    #[serde(default)]
    cfi: Config,
}

pub fn parse() -> Config {
    from_env::<Env>().unwrap().cfi
}
