use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use tracing::metadata::LevelFilter;

use crate::config::{self, Config};
use utils::logger;

pub fn setup() -> Result<Config> {
    let handle = logger::init(tracing::Level::INFO);

    let mut config = config::args::parse()?;

    config = config::env::parse()?.merge(config);

    let config_file = &config.config_file;
    let was_set = config_file.is_some();
    let path = config_file
        .clone()
        .unwrap_or_else(|| "config.yaml".to_string());
    if !path.is_empty() {
        let result = config::file::parse(&path).map_err(|e| {
            log::error!("Failed to parse config file: {}", e);
            e
        });
        if let Ok(result) = result {
            if let Some(file_config) = result {
                config = file_config.merge(config);
            } else if was_set {
                return Err(anyhow!("Config file was set but not found"));
            }
        } else {
            return Err(anyhow!(
                "Failed to parse config file: {:#}",
                result.unwrap_err()
            ));
        }
    }

    if let Some(level) = config.log_level.clone() {
        handle.set_level(LevelFilter::from_str(&level)?.into_level().unwrap());
    }

    Ok(config)
}
