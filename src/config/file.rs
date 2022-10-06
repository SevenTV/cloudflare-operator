use super::Config;
use std::{error::Error, fs::File, result::Result};

pub fn parse(file: String) -> Result<Config, Box<dyn Error>> {
    let file = File::open(file);
    if let Ok(file) = file {
        let config: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(file);
        if let Ok(config) = config {
            return Ok(config);
        }

        Err(Box::new(config.err().unwrap()))
    } else {
        Err(Box::new(file.err().unwrap()))
    }
}
