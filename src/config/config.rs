use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

use crate::config::utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub openai_prompt_file_path: Option<String>,
}

impl Config {
    pub fn new(api_key: String, prompt_file: String) -> Self {
        Self {
            openai_api_key: Some(api_key),
            openai_prompt_file_path: Some(prompt_file),
        }
    }

    pub fn fetch() -> Result<Self, anyhow::Error> {
        let config_path = utils::get_config_path();
        let config_file = File::open(config_path.clone()).expect("Failed to open config file");
        let config: Config = serde_json::from_reader(config_file)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let config_path = utils::get_config_path();
        let config_file = File::create(config_path.clone()).expect("Failed to create config file");
        serde_json::to_writer_pretty(config_file, self)?;
        Ok(())
    }
}
