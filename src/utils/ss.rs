use std::path::PathBuf;

use anyhow::Error;
use log::error;

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
