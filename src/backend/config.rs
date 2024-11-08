// Responsible for handling the config. Does not do any actions on its own, just config parsing

use std::{fs, path::PathBuf};
use serde::Deserialize;
use anyhow;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub repositories: Vec<String>
}

impl Config {
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let file_contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&file_contents)?;
        return Ok(config)
    }

    pub fn default_config() -> String {
        return r"repositories = []
".to_string()
    }
}
