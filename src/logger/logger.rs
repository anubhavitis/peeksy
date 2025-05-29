use std::fs::OpenOptions;

use simplelog::*;

pub fn setup_logger() {
    let log_path = dirs::config_dir().unwrap().join("peeksy");
    if !log_path.exists() {
        std::fs::create_dir_all(&log_path).unwrap();
    }

    let info_path = log_path.join("info.log");

    // create file only if it does not exist
    let info_writable = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&info_path)
        .unwrap();

    let error_path = log_path.join("error.log");
    let error_writable = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&error_path)
        .unwrap();

    let debug_path = log_path.join("debug.log");
    let debug_writable = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&debug_path)
        .unwrap();

    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Info, Config::default(), info_writable),
        WriteLogger::new(LevelFilter::Error, Config::default(), error_writable),
        WriteLogger::new(LevelFilter::Debug, Config::default(), debug_writable),
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])
    .unwrap();
}
