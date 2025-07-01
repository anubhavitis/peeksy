use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, RecvTimeoutError},
        Arc,
    },
};

use anyhow::Error;
use chrono::Local;
use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{ai::ai::OpenAI, config, daemon::file::SSController};

fn get_controller() -> Result<SSController, String> {
    let config = config::config::Config::fetch().expect("Failed to fetch config");
    let openai_api_key = config.openai_api_key.unwrap();
    let openai_prompt_file = config.openai_prompt_file_path.unwrap();

    if openai_api_key.is_empty() || openai_prompt_file.is_empty() {
        return Err("OpenAI API key or prompt file is empty".to_string());
    }

    let prompt = std::fs::read_to_string(&openai_prompt_file).expect("Failed to read prompt file");
    let ai = OpenAI::new(openai_api_key, prompt);
    Ok(SSController::new(ai))
}

fn get_clean_path(raw: String, home: PathBuf) -> Result<PathBuf, Error> {
    if raw.starts_with("~/") {
        return Ok(home.join(raw[2..].to_string()));
    }

    let raw_path = PathBuf::from(raw.clone());
    // if raw_path has home path, then return raw_path
    if raw_path.starts_with(&home) {
        return Ok(raw_path);
    } else {
        return Err(anyhow::anyhow!(
            "Raw path {} is not understanding",
            raw_path.display()
        ));
    }
}

pub fn get_screenshot_dir() -> PathBuf {
    use std::process::Command;

    // picks desktop dir if default is not found
    let desktop_ss_dir = dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("/Users/Shared"));

    let default_dir = Command::new("defaults")
        .args(["read", "com.apple.screencapture", "location"])
        .output()
        .ok();

    dbg!(&default_dir);

    let out = match default_dir {
        None => return desktop_ss_dir,
        Some(out) => out,
    };

    if !out.status.success() {
        return desktop_ss_dir;
    }

    let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let parent = dirs::home_dir().unwrap();
    let clean_path = get_clean_path(raw, parent);

    match clean_path {
        Ok(path) => path,
        Err(e) => {
            error!("Error getting clean path: {:?}", e);
            desktop_ss_dir
        }
    }
}

pub async fn controller(shutdown: Arc<AtomicBool>) {
    let screenshot_dir = get_screenshot_dir();
    let pid = std::process::id();
    println!("🚀 Daemon started with PID: {}", pid);

    // Save PID to file (like my example)
    let parent_path = dirs::config_dir().unwrap().join("peeksy");
    let pid_path = parent_path.join("peeksy.pid");
    std::fs::write(pid_path, pid.to_string()).unwrap();

    info!(
        "Starting Peeksy on {} at {} with PID {}",
        screenshot_dir.display(),
        Local::now(),
        pid
    );

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).expect("Failed to create watcher");
    watcher
        .watch(&screenshot_dir, RecursiveMode::NonRecursive)
        .expect("Failed to watch directory");

    let ss_controller = get_controller().expect("Failed to setup controller");

    info!("Setup complete, Peeksy is ready!");
    while !shutdown.load(Ordering::Relaxed) {
        match rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(event) => {
                if let Ok(Event {
                    kind: EventKind::Create(_),
                    paths,
                    ..
                }) = event
                {
                    for path in paths {
                        info!("Detected new file: {:?}", path);
                        let resp = ss_controller.process_ss(&path).await;
                        if let Err(e) = resp {
                            error!("Error processing file: {:?}", e);
                        }
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => continue,
            Err(e) => error!("Watch error: {:?}", e),
        }
    }

    info!("Shutting down Peeksy thread...");
    watcher.unwatch(&screenshot_dir).ok();
}
