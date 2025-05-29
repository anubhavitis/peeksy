use std::{fs::File, path::PathBuf};

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

        // write an empty json object to the file
        let json = serde_json::json!({});
        serde_json::to_writer_pretty(file, &json).expect("Failed to write to config file");
        println!("Wrote empty json object to config file: {:?}", path);
    }

    path
}
