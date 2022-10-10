use anyhow::Result;

use super::Config;
use std::fs::File;

pub fn parse(file: String) -> Result<Config> {
    let file = File::open(file)?;
    Ok(serde_yaml::from_reader(file)?)
}
