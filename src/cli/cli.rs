use std::sync::{Arc, Mutex};

use clap::{Parser, Subcommand};

use crate::{
    cli::handlers::{
        config::{current_config, edit_config, view_prompt_file},
        log::{error_logs, info_logs},
        status::{is_daemon_running, restart_daemon, start_daemon, status_daemon, stop_daemon},
    },
    config,
    daemon::runner,
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
        // init requirements
        let config = config::config::Config::fetch().expect("Failed to fetch config");

        // execute command
        match &self.command {
            // daemon handlers
            Commands::Start => start_daemon().await,
            Commands::Stop => stop_daemon().await,
            Commands::Restart => restart_daemon().await,
            Commands::Status => status_daemon().await,

            // config handlers
            Commands::CurrentConfig => current_config(&config).await,
            Commands::ViewPromptFile => view_prompt_file(&config).await,
            Commands::EditConfig => edit_config(&mut config.clone()).await,

            // utils handlers
            Commands::Rename { file_path } => rename_file(file_path).await,
            Commands::ProcessExistingScreenshots => process_existing_screenshots().await,
            Commands::Daemon => daemon().await,

            // log handlers
            Commands::InfoLogs => info_logs().await,
            Commands::ErrorLogs => error_logs().await,
        }
    }
}

async fn daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!("Peeksy daemon is already running with PID {}", pid);
        return;
    }
    runner::run().await;
}

async fn rename_file(file_path: &str) {
    /*
        1. Convert the file_path to PathBuf
        2. Check if the file exists at the path, and is an image. If no, print error.
        3.
    */
}

async fn process_existing_screenshots() {
    /*
    1. Get the Screenshot Dir
    2. Fetch all the screenshots in the folder
    3. count the total screenshots and print the total count
    4. ask confirmation from user if they want to continue
    5. if Yes, start a seprate process and handle all screenshot one by one.
    */
}
