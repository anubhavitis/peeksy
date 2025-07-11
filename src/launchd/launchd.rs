use std::{path::PathBuf, process::Command};

use log::info;

const PLIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>Label</key>
            <string>com.anubhavitis.peeksy</string>
            <key>ProgramArguments</key>
            <array>
                <string>/opt/homebrew/bin/peeksy</string>
                <string>daemon</string>
            </array>
            <key>RunAtLoad</key>
            <true/>
            <key>KeepAlive</key>
            <false/>
            <key>StandardOutPath</key>
            <string>/tmp/peeksy.out</string>
            <key>StandardErrorPath</key>
            <string>/tmp/peeksy.err</string>
        </dict>
        </plist>"#;

pub struct LaunchD {
    plist_path: PathBuf,
}

impl LaunchD {
    pub fn new() -> Self {
        let user_home_dir_path = dirs::home_dir().unwrap();
        let plist_path =
            user_home_dir_path.join("Library/LaunchAgents/com.anubhavitis.peeksy.plist");

        let launchd = Self { plist_path };
        launchd.setup();
        launchd.write_plist();
        launchd
    }

    fn setup(&self) {
        let plist_dir = self.plist_path.parent().unwrap();
        if !plist_dir.exists() {
            std::fs::create_dir_all(plist_dir).unwrap();
        }
    }

    fn write_plist(&self) {
        std::fs::write(self.plist_path.clone(), PLIST).unwrap();
    }

    pub async fn load(&self) {
        info!("loading LaunchD plist path: {}", self.plist_path.display());
        let output = Command::new("launchctl")
            .args(["load", self.plist_path.to_str().unwrap()])
            .output()
            .unwrap();
        info!("LaunchD plist loaded successfully: {}", output.status);
    }

    pub async fn unload(&self) {
        info!(
            "unloading LaunchD plist path: {}",
            self.plist_path.display()
        );
        let output = Command::new("launchctl")
            .args(["unload", self.plist_path.to_str().unwrap()])
            .output()
            .unwrap();
        info!("LaunchD plist unloaded successfully: {}", output.status);
    }
}
