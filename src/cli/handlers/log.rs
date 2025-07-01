pub async fn info_logs() {
    let path = dirs::config_dir().unwrap().join("peeksy");
    let logs = std::fs::read_to_string(path.join("info.log")).expect("Failed to read info.log");
    println!("---------\n{}", logs);
}

pub async fn error_logs() {
    let path = dirs::config_dir().unwrap().join("peeksy");
    let logs = std::fs::read_to_string(path.join("error.log")).expect("Failed to read error.log");
    println!("---------\n{}", logs);
}
