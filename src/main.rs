use dotenv::dotenv;
use log::{error, info};
use simplelog::*;
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::signal;

use peeksy::peeksy;

fn get_screenshot_dir() -> PathBuf {
    use std::process::Command;
    let output = Command::new("defaults")
        .args(["read", "com.apple.screencapture", "location"])
        .output()
        .ok();

    if let Some(out) = output {
        if out.status.success() {
            let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
            return PathBuf::from(raw);
        }
    }

    // picks desktop dir if default is not found
    dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("/Users/Shared"))
}

fn setup_logger() {
    // todo: fix this path to have apt location
    let log_path = dirs::home_dir().unwrap();
    let info_path = log_path.join("Desktop/info.txt");
    let error_path = log_path.join("Desktop/error.txt");
    let debug_path = log_path.join("Desktop/debug.txt");

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(&info_path).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            File::create(&error_path).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(&debug_path).unwrap(),
        ),
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])
    .unwrap();
}

fn check_permissions() {
    // check if user has permission to read/write to screenshot directory
    info!("Peeksy: Checking permissions");
    info!("Peeksy: Permission check complete");
}

fn setup_required_env_vars() -> Result<(), Box<dyn std::error::Error>> {
    // Create .env file if it doesn't exist
    if !Path::new(".env").exists() {
        File::create(".env")?;
        info!("Created .env file");
    }
    dotenv().ok();

    // Handle OpenAI API key
    let api_key = match std::env::var("PEEKSY_OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            info!("OpenAI API key not found in environment variables");
            println!("Please enter your OpenAI API key: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();

            let mut file = OpenOptions::new().write(true).append(true).open(".env")?;
            writeln!(file, "PEEKSY_OPENAI_API_KEY={}", input)?;
            input
        }
    };

    // Handle prompt file path
    let prompt_path = match std::env::var("PEEKSY_OPENAI_PROMPT_FILE") {
        Ok(path) => path,
        Err(_) => {
            info!("OpenAI prompt file not found in environment variables");
            println!("Please enter the path to your prompt file (default: prompt.txt): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = if input.trim().is_empty() {
                "prompt.txt".to_string()
            } else {
                input.trim().to_string()
            };

            let mut file = OpenOptions::new().write(true).append(true).open(".env")?;
            writeln!(file, "PEEKSY_OPENAI_PROMPT_FILE={}", input)?;
            input
        }
    };

    // Verify the values were set correctly
    if api_key.is_empty() || prompt_path.is_empty() {
        return Err("Required environment variables are empty".into());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    setup_logger();
    setup_required_env_vars().expect("Failed to setup required environment variables");
    check_permissions();
    let screenshot_dir = get_screenshot_dir();
    info!(
        "Peeksy: ScreenShot Monitoring directory: {:?}",
        screenshot_dir
    );

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    let peeksy_thread_handler = tokio::spawn(async move {
        info!("Starting Peeksy thread");
        peeksy::controller(screenshot_dir, shutdown_clone).await;
    });

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
            shutdown.store(true, Ordering::Relaxed);
        }
        _ = peeksy_thread_handler => {
            info!("Peeksy thread exited unexpectedly");
        }
    }

    // Give the thread a moment to clean up
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    info!("Peeksy: Shutting down");
}
