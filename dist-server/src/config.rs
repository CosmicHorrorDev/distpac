use anyhow::Result;
use serde::Deserialize;

use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub announce_url: String,
}

impl Config {
    pub fn try_new() -> Result<Self> {
        let config_path = dist_utils::path::server_config_file();
        let config_file = File::open(&config_path)?;
        let config = serde_yaml::from_reader(config_file)?;
        Ok(config)
    }
}
