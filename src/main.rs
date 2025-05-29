use chrono::Local;
use dotenv::dotenv;
use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use simplelog::*;
use std::{fs::File, io, path::PathBuf, sync::mpsc::channel};

use peeksy::file::SSController;

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

fn setup() -> SSController {
    setup_logger();
    dotenv().ok();

    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            error!("Error getting api key: {:?}", e);
            println!("Please enter your OpenAI API key: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        }
    };

    let prompt_file_path = match std::env::var("OPENAI_PROMPT_FILE") {
        Ok(path) => path,
        Err(e) => {
            error!("Error getting prompt file path: {:?}", e);
            println!("Please enter the path to your prompt file (default: prompt.txt):");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let prompt_path = input.trim().to_string();
                    if prompt_path.is_empty() {
                        "prompt.txt".to_string()
                    } else {
                        prompt_path
                    }
                }
                Err(_) => "prompt.txt".to_string(),
            }
        }
    };
    info!("Using prompt file: {:?}", prompt_file_path);

    let prompt = std::fs::read_to_string(&prompt_file_path).expect("Failed to read prompt file");

    SSController::new(api_key, prompt)
}

#[tokio::main]
async fn main() {
    info!("Starting Peeksy at {}", Local::now());

    let screenshot_dir = get_screenshot_dir();
    info!(
        "Peeksy: ScreenShot Monitoring directory: {:?}",
        screenshot_dir
    );

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).expect("Failed to create watcher");
    watcher
        .watch(&screenshot_dir, RecursiveMode::NonRecursive)
        .expect("Failed to watch directory");

    let ss_controller = setup();

    info!("Setup complete, Peeksy is ready!");
    loop {
        match rx.recv() {
            Ok(event) => {
                // println!("Received event {:?}", event);
                if let Ok(Event {
                    kind: EventKind::Create(_),
                    paths,
                    ..
                }) = event
                {
                    for path in paths {
                        info!("Detected new file: {:?}", path);
                        let resp = ss_controller.process_file(&path).await;
                        if let Err(e) = resp {
                            error!("Error processing file: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}
