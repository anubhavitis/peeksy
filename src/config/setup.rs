use std::io;

use crate::config::config;

use log::info;

pub fn setup_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::Config::fetch().expect("Failed to fetch config");

    let path: std::path::PathBuf = dirs::config_dir().unwrap().join("peeksy");
    {
        // saving default prompt file to config directory
        let prompt_file =
            std::fs::read_to_string("prompt.txt").expect("Failed to read prompt file");
        std::fs::write(path.join("prompt.txt"), prompt_file).expect("Failed to write prompt file");
    }

    let mut updated = false;
    // Handle OpenAI API key if empty or not set
    if config
        .openai_api_key
        .as_ref()
        .map_or(true, |k| k.is_empty())
    {
        info!("OpenAI API key not found in environment variables");
        println!("Please enter your OpenAI API key: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        config.openai_api_key = Some(input.trim().to_string());
        updated = true;
    }

    // Handle prompt file path if empty or not set
    if config
        .openai_prompt_file_path
        .as_ref()
        .map_or(true, |f| f.is_empty())
    {
        info!("OpenAI prompt file not found in environment variables");
        let prompt_file = path.join("prompt.txt");
        println!(
            "Please enter your prompt file name(press enter for default: {}): ",
            prompt_file.to_str().unwrap()
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = if input.trim().is_empty() {
            prompt_file.to_str().unwrap().to_string()
        } else {
            input.trim().to_string()
        };
        config.openai_prompt_file_path = Some(input);
    }

    if updated {
        let config_clone = config.clone();
        config.save().expect("Failed to save config");
        info!("Config saved: {:?}", config_clone);
    }

    Ok(())
}
