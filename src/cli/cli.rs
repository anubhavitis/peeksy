use std::{collections::HashSet, fs, io, path::PathBuf};

use clap::{Parser, Subcommand};
use log::error;

use crate::{
    cli::handlers::{
        config::{current_config, edit_config, view_prompt_file},
        log::{error_logs, info_logs},
        status::{daemon, restart_daemon, start_daemon, status_daemon, stop_daemon},
    },
    config::config::Config,
    manager::{ai::OpenAI, image::SSManager},
    utils::ss::get_screenshot_dir,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // status handlers
    Start,
    Stop,
    Restart,
    Status,

    // config handlers
    CurrentConfig,
    ViewPromptFile,
    EditConfig,

    // utils
    ProcessExistingScreenshots,
    Rename {
        file_path: String,
    },
    #[command(name = "daemon")]
    Daemon,

    // log handlers
    InfoLogs,
    ErrorLogs,
}

impl Args {
    pub async fn execute(&self) {
        // execute command
        match &self.command {
            // log handlers
            Commands::InfoLogs => info_logs().await,
            Commands::ErrorLogs => error_logs().await,

            // config handlers
            Commands::CurrentConfig => current_config().await,
            Commands::ViewPromptFile => view_prompt_file().await,
            Commands::EditConfig => edit_config().await,

            // daemon handlers
            Commands::Start => start_daemon().await,
            Commands::Stop => stop_daemon().await,
            Commands::Restart => restart_daemon().await,
            Commands::Status => status_daemon().await,
            Commands::Daemon => daemon().await,

            // utils handlers
            Commands::Rename { file_path } => rename_file(file_path).await,
            Commands::ProcessExistingScreenshots => process_existing_screenshots().await,
        }
    }
}

async fn rename_file(file_name: &str) {
    let config = Config::fetch().expect("Failed to fetch config");
    if !config.ready() {
        error!("Config is not ready. Please run `peeksy edit-config` to set it.");
        return;
    }

    let file_name = PathBuf::from(file_name);

    if !is_image(&file_name) {
        error!(
            "File is not an image: {:?}\n Raise an issue on github https://github.com/anubhavitis/peeksy/issues for support.",
            file_name
        );
        return;
    }

    let config = Config::fetch().expect("Failed to fetch config");
    let ai = OpenAI::new(
        config.openai_api_key.unwrap(),
        config.openai_prompt_file_path.unwrap(),
        config.openai_model.unwrap(),
    );

    let ss_manager = SSManager::new(ai);
    let resp = ss_manager.process_random_image(&file_name).await;
    if let Err(e) = resp {
        error!("Error processing file: {:?}", e);
    }
}

async fn process_existing_screenshots() {
    let ss_dir = get_screenshot_dir();

    let files = fs::read_dir(ss_dir).unwrap();

    let config = Config::fetch().unwrap();
    let ai = OpenAI::new(
        config.openai_api_key.unwrap(),
        config.openai_prompt_file_path.unwrap(),
        config.openai_model.unwrap(),
    );
    let ss_manager = SSManager::new(ai);

    let mut screenshot = vec![];
    for file in files {
        let file = file.unwrap();
        let file_path = file.path();

        if ss_manager.is_screenshot_file(&file_path) {
            screenshot.push(file_path);
        }
    }

    println!("Found {} screenshots", screenshot.len());
    println!("Do you want to continue? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() != "y" {
        println!("Exiting...");
        return;
    }

    for file in screenshot {
        let resp = ss_manager.process_random_image(&file).await;
        if let Err(e) = resp {
            error!("Error processing file: {:?}", e);
        }
    }
}

fn is_image(file_name: &PathBuf) -> bool {
    // Set of allowed image extensions (lowercase)
    let image_exts: HashSet<&'static str> = [
        "png", "jpg", "jpeg", "gif", "webp", "heic", "heif", "bmp", "tiff", "tif",
    ]
    .iter()
    .cloned()
    .collect();

    file_name
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| image_exts.contains(&ext.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}
