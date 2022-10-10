use anyhow::anyhow;
use anyhow::Result;
use log::{debug, error};

use crate::config::{self, Config};
use utils::logger;

pub fn setup() -> Result<Config> {
    let logger = logger::init();
    let mut config = config::args::parse()?;

    config::env::parse()?.merge(&mut config);

    let config_file = config.get_config_file();
    let was_set = config_file.is_some();
    let path = config_file.unwrap_or_else(|| "config.yaml".to_string());
    if !path.is_empty() {
        let result = config::file::parse(path).map_err(|e| {
            error!("Failed to parse config file: {}", e);
            e
        });
        if let Ok(result) = result {
            result.merge(&mut config);
        } else if was_set {
            return Err(anyhow!(
                "Failed to parse config file: {:#}",
                result.unwrap_err()
            ));
        }
    }

    if config.get_debug() {
        logger.set_level(log::LevelFilter::Debug);
        debug!("debug mode enabled");
    } else {
        logger.set_level(log::LevelFilter::Info);
    }

    Ok(config)
}
