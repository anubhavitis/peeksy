use std::path::PathBuf;

fn get_pid_path() -> PathBuf {
    let parent_path = dirs::config_dir().unwrap().join("peeksy");
    if !parent_path.exists() {
        std::fs::create_dir_all(parent_path.clone()).unwrap();
    }
    parent_path.join("peeksy.pid")
}

pub fn save_pid(pid: u32) {
    let pid_path = get_pid_path();
    std::fs::write(pid_path, pid.to_string()).unwrap();
}

pub fn get_pid() -> Result<u32, String> {
    let pid_path = get_pid_path();
    if pid_path.exists() {
        let pid = std::fs::read_to_string(pid_path).unwrap();
        Ok(pid.parse::<u32>().unwrap())
    } else {
        Err("Peeksy daemon is not running".to_string())
    }
}
