use clap::{Parser, Subcommand};

use crate::{
    config::{self},
    daemon::{pid, runner},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start,
    Stop,
    Restart,
    Status,
    CurrentConfig,
    ViewPromptFile,
    UpdateApiKey {
        value: String,
    },
    UpdatePromptFilePath {
        value: String,
    },

    #[command(name = "daemon")]
    Daemon,
    InfoLogs,
    ErrorLogs,
}

impl Args {
    pub async fn execute(&self) {
        match &self.command {
            Commands::Start => start_daemon().await,
            Commands::Stop => stop_daemon().await,
            Commands::Restart => restart_daemon().await,
            Commands::Status => status_daemon().await,
            Commands::CurrentConfig => current_config().await,
            Commands::ViewPromptFile => view_prompt_file().await,
            Commands::UpdateApiKey { value } => update_api_key(value).await,
            Commands::UpdatePromptFilePath { value } => update_prompt_file(value).await,
            Commands::Daemon => daemon().await,
            Commands::InfoLogs => info_logs().await,
            Commands::ErrorLogs => error_logs().await,
        }
    }
}

async fn is_daemon_running() -> (bool, u32) {
    let pid = pid::get_pid();
    match pid {
        Ok(pid) => {
            let process = std::process::Command::new("ps")
                .args(["-p", &pid.to_string()])
                .output()
                .unwrap();
            (process.status.success(), pid)
        }
        Err(_) => (false, 0),
    }
}

async fn status_daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!("Peeksy daemon is running with PID {}", pid);
    } else {
        println!("Peeksy daemon is not running");
    }
}

async fn restart_daemon() {
    stop_daemon().await;
    start_daemon().await;
}

async fn stop_daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if !is_running {
        println!("Peeksy daemon is not running");
        return;
    }

    println!("Stopping Peeksy daemon with PID {}", pid);

    if let Err(e) = std::process::Command::new("kill")
        .arg(pid.to_string())
        .output()
    {
        println!("Failed to stop daemon: {}", e);
        return;
    }

    // Give process time to terminate
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Verify daemon was stopped
    let (still_running, _) = is_daemon_running().await;
    if still_running {
        println!("❌ Failed to stop Peeksy daemon with PID {}", pid);
    } else {
        println!("✅ Peeksy daemon stopped successfully");
    }
}

async fn start_daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!("Peeksy daemon is already running with PID {}", pid);
        return;
    }

    let current_exe = std::env::current_exe().expect("Failed to get current executable path");

    // Spawn daemon as separate process
    match std::process::Command::new(&current_exe)
        .arg("daemon")
        .spawn()
    {
        Ok(_) => {
            // Give it a moment to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let (is_running, pid) = is_daemon_running().await;
            if is_running {
                println!("✅ Daemon started successfully with PID {}", pid);
            } else {
                println!("❌ Failed to start daemon");
            }
        }
        Err(e) => {
            println!("❌ Failed to spawn daemon: {}", e);
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

async fn update_api_key(value: &str) {
    let mut config = config::config::Config::fetch().expect("Failed to fetch config");
    config.openai_api_key = Some(value.to_string());
    config.save().expect("Failed to save config");
}

async fn update_prompt_file(value: &str) {
    let mut config = config::config::Config::fetch().expect("Failed to fetch config");
    config.openai_prompt_file_path = Some(value.to_string());
    config.save().expect("Failed to save config");
}

async fn current_config() {
    let config = config::config::Config::fetch().expect("Failed to fetch config");
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
}

async fn view_prompt_file() {
    let config = config::config::Config::fetch().expect("Failed to fetch config");
    let prompt_file = config.openai_prompt_file_path.unwrap();
    let prompt = std::fs::read_to_string(prompt_file).expect("Failed to read prompt file");
    println!("----------\n{}\n---------", prompt);
}

async fn info_logs() {
    let path = dirs::config_dir().unwrap().join("peeksy");
    let logs = std::fs::read_to_string(path.join("info.log")).expect("Failed to read info.log");
    println!("---------\n{}", logs);
}

async fn error_logs() {
    let path = dirs::config_dir().unwrap().join("peeksy");
    let logs = std::fs::read_to_string(path.join("error.log")).expect("Failed to read error.log");
    println!("---------\n{}", logs);
}
