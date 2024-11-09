// Responsible for handling the config. Does not do any actions on its own, just config parsing

use std::{fs, path::PathBuf};
use serde::Deserialize;
use anyhow;


#[derive(Debug, Deserialize)]
pub struct Config {
    // items to load per chunk
    pub load_chunk: usize,
    // load file size from remote automatically, worth disabling by default in case you are in a
    // country which does not like certain domains
    pub auto_load_file_size: bool
}

impl Config {
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let file_contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&file_contents)?;
        return Ok(config)
    }

    pub fn default_config() -> String {
        return r"load_chunk = 100
auto_load_file_size = false".to_string()
    }
}
