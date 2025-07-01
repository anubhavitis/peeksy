use std::{fs::File, path::PathBuf};

// sets the prompt at prompt.txt
pub fn initial_setup() -> Result<(), Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = dirs::config_dir().unwrap().join("peeksy");
    {
        // saving default prompt file to config directory
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

        std::fs::write(path.join("prompt.txt"), default_prompt)
            .expect("Failed to write prompt file");
    }

    Ok(())
}

pub fn get_config_path() -> PathBuf {
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

    let path = parent.join("peeksy_config.json");

    if !path.exists() {
        // create an empty json file
        let file = File::create(path.clone()).expect("Failed to create config file");
        println!("Created empty config file: {:?}", path);

        let default_prompt_file_path = parent.join("prompt.txt");

        // write an empty json object to the file
        let json = serde_json::json!({
            "openai_api_key": "",
            "openai_prompt_file_path": default_prompt_file_path.to_str().unwrap()
        });
        serde_json::to_writer_pretty(file, &json).expect("Failed to write to config file");
        println!("Wrote empty json object to config file: {:?}", path);
    }

    path
}
