use chrono::Local;
use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use simplelog::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};

fn modify_path(path: &Path) -> PathBuf {
    let filename = path.file_name().unwrap().to_str().unwrap()[1..].to_string();
    let parent = path.parent().unwrap_or(Path::new("."));
    parent.join(filename)
}

fn is_screenshot_file(path: &Path) -> bool {
    if let Some(mut filename) = path.file_name().and_then(|n| n.to_str()) {
        filename = &filename[1..];
        let lowercase = filename.to_lowercase();
        println!("lowercase filename: {:?}", lowercase);
        (lowercase.starts_with("screenshot") || lowercase.contains("screen shot"))
            && !filename.starts_with("ss-")
            && path.extension().map_or(false, |ext| ext == "png")
    } else {
        false
    }
}

fn file_is_recent(path: &Path, max_age: Duration) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(created) = metadata.created().or_else(|_| metadata.modified()) {
            return SystemTime::now()
                .duration_since(created)
                .unwrap_or(Duration::MAX)
                < max_age;
        }
    }
    false
}

fn process_file(weird_path: &Path) {
    println!("Processing file: {:?}", weird_path);
    if !is_screenshot_file(weird_path) {
        return;
    }

    let path = modify_path(weird_path);

    if !file_is_recent(&path, Duration::from_secs(10)) {
        info!("Skipping old file: {:?}", path);
        return;
    }

    println!("file is screenshot and recent: {:?}", path);

    let parent = path.parent().unwrap_or(Path::new("."));
    let filename = path.file_name().unwrap().to_str().unwrap();
    let new_filename = format!("ss-{}", filename);
    let new_path = parent.join(new_filename);

    if let Err(e) = fs::copy(path.clone(), &new_path) {
        error!(
            "Failed to copy file: {:?} -> {:?}, Error: {}",
            path, new_path, e
        );
    } else {
        info!("Created duplicate: {:?}", new_path);
    }
}

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

    dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("/Users/Shared"))
}

fn main() -> Result<()> {
    let log_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Desktop/screenshot-monitor.txt");

    let _ = fs::create_dir_all(log_path.parent().unwrap());

    println!("Log path: {:?}", log_path);

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        fs::File::create(&log_path).unwrap(),
    )])
    .unwrap();

    info!("Starting screenshot monitor at {}", Local::now());
    println!("Starting screenshot monitor at {}", Local::now());

    let screenshot_dir = get_screenshot_dir();
    info!("Monitoring directory: {:?}", screenshot_dir);
    println!("Monitoring directory: {:?}", screenshot_dir);
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default())?;
    watcher.watch(&screenshot_dir, RecursiveMode::NonRecursive)?;

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
                    println!("received event for paths: {:?}", paths);
                    for path in paths {
                        dbg!(path.is_file());
                        info!("Detected new file: {:?}", path);
                        process_file(&path);
                    }
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}
