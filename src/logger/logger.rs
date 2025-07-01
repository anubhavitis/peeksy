use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use simplelog::*;

fn get_log_path() -> PathBuf {
    let log_path = dirs::config_dir().unwrap().join("peeksy");
    if !log_path.exists() {
        std::fs::create_dir_all(&log_path).unwrap();
    }
    log_path
}

fn get_info_writable(log_path: PathBuf) -> File {
    let info_path = log_path.join("info.log");
    let info_writable = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&info_path)
        .unwrap();
    info_writable
}

fn get_error_writable(log_path: PathBuf) -> File {
    let error_path = log_path.join("error.log");
    let error_writable = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&error_path)
        .unwrap();
    error_writable
}

fn get_debug_writable(log_path: PathBuf) -> File {
    let debug_path = log_path.join("debug.log");
    let debug_writable = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&debug_path)
        .unwrap();
    debug_writable
}

pub fn setup_logger() {
    let log_path = get_log_path();
    let info_writable = get_info_writable(log_path.clone());
    let error_writable = get_error_writable(log_path.clone());
    let debug_writable = get_debug_writable(log_path.clone());

    // Check if we're in release mode (debug_assertions is false in release builds)
    let is_debug = cfg!(debug_assertions);
    if !is_debug {
        // Release mode: only error logs
        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            error_writable,
        )])
        .unwrap();
    } else {
        // Debug mode: all logs (info, error, debug)
        CombinedLogger::init(vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), info_writable),
            WriteLogger::new(LevelFilter::Error, Config::default(), error_writable),
            WriteLogger::new(LevelFilter::Debug, Config::default(), debug_writable),
        ])
        .unwrap();
    }
}
