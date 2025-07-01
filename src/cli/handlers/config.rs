use crate::config;

pub async fn edit_config(config: &mut config::config::Config) {
    config.edit_config().expect("Failed to edit config");
    println!("Config edited successfully");
}

pub async fn current_config(config: &config::config::Config) {
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
}

pub async fn view_prompt_file(config: &config::config::Config) {
    let prompt_file = config.get_openai_prompt_file_path();
    match prompt_file {
        Some(prompt_file) => {
            let prompt = std::fs::read_to_string(prompt_file).expect("Failed to read prompt file");
            println!("----------\n{}\n---------", prompt);
        }
        None => {
            println!("Prompt file not found in config. Please run `peeksy edit-config` to set it.");
        }
    }
}
