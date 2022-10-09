use anyhow::Result;

use super::Config;
use std::fs::File;

pub fn parse(file: String) -> Result<Config> {
    let file = File::open(file);
    if let Ok(file) = file {
        let config: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(file);
        if let Ok(config) = config {
            return Ok(config);
        }

        Err(config.err().unwrap().into())
    } else {
        Err(file.err().unwrap().into())
    }
}
