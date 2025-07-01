use std::{fs::File, path::PathBuf};

use super::config;

pub fn get_config_path() -> PathBuf {
    let parent = dirs::config_dir().unwrap().join("peeksy");
    let path: PathBuf = parent.join("peeksy_config.json");
    path
}

pub fn initial_setup() -> Result<(), Box<anyhow::Error>> {
    initial_prompt_setup()?;
    initial_path_setup()?;
    default_config_setup()?;
    Ok(())
}

// sets the prompt at prompt.txt
pub fn initial_prompt_setup() -> Result<(), Box<anyhow::Error>> {
    let path: std::path::PathBuf = dirs::config_dir().unwrap().join("peeksy");
    let prompt_path = path.join("prompt.txt");
    let default_prompt = r#"
    Analyze the attached image and generate a short, descriptive filename that clearly reflects its subject, context, and content.
    Rules:
        1. Use lowercase letters only. Separate words with hyphens. No spaces or underscores.
        2. Keep the filename between 3 to 8 words. Be concise but meaningful.
        3. Apply intelligent context recognition:
            - If it is an album cover, include the album title and band or artist name.
            - If it is artwork, mention the style (e.g., oil-painting, digital-art, 3d-render).
            - If it's a poster, include the movie/show/event name.
        4. Avoid generic terms like "image", "picture", "photo", or "screenshot".
        5. Do not include the file extension (e.g., .jpg or .png) in the output.
    
    Return only the final filename string, with no extra explanation or punctuation."#;

    if !prompt_path.exists() {
        std::fs::write(&prompt_path, default_prompt).expect("Failed to write prompt file");
    }

    Ok(())
}

pub fn initial_path_setup() -> Result<(), Box<anyhow::Error>> {
    let parent = dirs::config_dir().unwrap().join("peeksy");
    if !parent.exists() {
        match std::fs::create_dir_all(parent.clone()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to create config directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    let path: PathBuf = parent.join("peeksy_config.json");

    if !path.exists() {
        // create an empty json file
        let file = File::create(path.clone()).expect("Failed to create config file");
        println!("Created empty config file: {:?}", path);

        // write an empty json object to the file
        let json = serde_json::json!({
            "openai_api_key": "",

        });

        // write the json to the file
        serde_json::to_writer_pretty(file, &json).expect("Failed to write to config file");
        println!("Wrote empty json object to config file: {:?}", path);
    }

    Ok(())
}

fn default_config_setup() -> Result<(), Box<anyhow::Error>> {
    let mut config = config::Config::fetch()?;

    if !config.openai_prompt_file_path_exists() {
        let parent = dirs::config_dir().unwrap().join("peeksy");
        let default_prompt_file_path = parent.join("prompt.txt");

        config.openai_prompt_file_path =
            Some(default_prompt_file_path.to_str().unwrap().to_string());
        config.save()?;
    }

    if !config.openai_model_exists() {
        config.openai_model = Some("gpt-4o".to_string());
        config.save()?;
    }

    Ok(())
}
