use anyhow::Result;

use super::Config;
use std::fs::File;

pub fn parse(pth: &String) -> Result<Option<Config>> {
    let file = File::open(&pth);
    if let Ok(file) = file {
        let mut cfg: Config = serde_yaml::from_reader(file)?;

        cfg.config_file = Some(pth.clone());

        Ok(Some(cfg))
    } else {
        Ok(None)
    }
}
