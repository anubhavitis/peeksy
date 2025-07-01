use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::File, io};

use crate::config::setup;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub openai_prompt_file_path: Option<String>,
}

impl Config {
    pub fn fetch() -> Result<Self, anyhow::Error> {
        let config_path = setup::get_config_path();
        let config_file = File::open(config_path.clone()).expect("Failed to open config file");
        let config: Config = serde_json::from_reader(config_file)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let config_path = setup::get_config_path();
        let config_file = File::create(config_path.clone()).expect("Failed to create config file");
        serde_json::to_writer_pretty(config_file, self)?;
        Ok(())
    }

    fn openai_api_key_exists(&self) -> bool {
        if let Some(key) = self.openai_api_key.as_ref() {
            !key.is_empty()
        } else {
            false
        }
    }

    fn openai_prompt_file_path_exists(&self) -> bool {
        if let Some(path) = self.openai_prompt_file_path.as_ref() {
            !path.is_empty()
        } else {
            false
        }
    }

    pub fn get_openai_api_key(&self) -> Option<String> {
        self.openai_api_key.clone()
    }

    pub fn get_openai_prompt_file_path(&self) -> Option<String> {
        self.openai_prompt_file_path.clone()
    }

    // sets the api key and prompt file path
    pub fn edit_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut updated = false;

        // Handle OpenAI API key if empty or not set
        let new_key = self.get_openai_api_key_from_user()?;
        if new_key.is_some() {
            self.openai_api_key = new_key;
            updated = true;
        }

        // Handle prompt file path if empty or not set
        let new_path = self.get_openai_prompt_file_path_from_user()?;
        if new_path.is_some() {
            self.openai_prompt_file_path = new_path;
            updated = true;
        }

        if updated {
            let config_clone = self.clone();
            self.save().expect("Failed to save config");
            info!("[setup_config] Config saved: {:?}", config_clone);
        }

        Ok(())
    }

    fn get_openai_api_key_from_user(&self) -> Result<Option<String>, anyhow::Error> {
        if self.openai_api_key_exists() {
            println!("Please enter your OpenAI API key (press enter to skip and keep using the existing key): ");
        } else {
            println!("Please enter your OpenAI API key: ");
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| anyhow::anyhow!("Error reading input: {}", e))?;

        let input = input.trim().to_string();
        if input.is_empty() {
            return Ok(None);
        }
        Ok(Some(input))
    }

    fn get_openai_prompt_file_path_from_user(&self) -> Result<Option<String>, anyhow::Error> {
        if self.openai_prompt_file_path_exists() {
            let path = self.get_openai_prompt_file_path().unwrap();
            println!(
                "Please enter your OpenAI prompt file path (press enter to skip and keep using the existing path: {}): ",
                path
            );
        } else {
            println!("Please enter your OpenAI prompt file path: ");
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| anyhow::anyhow!("Error reading input: {}", e))?;

        let input = input.trim().to_string();
        if input.is_empty() {
            return Ok(None);
        }
        Ok(Some(input))
    }
}
