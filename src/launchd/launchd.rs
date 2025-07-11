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

    async fn status(&self) -> Result<i32, anyhow::Error> {
        info!("checking LaunchD plist status");
        let output = Command::new("launchctl").args(["list"]).output().unwrap();

        // here output will be of form ``` <PID> <status> <plist>``` or null
        // if null, then the plist is not loaded
        // if not null, then plist is loaded.
        // if PID > 0 the process is running, else not.

        let output_str = String::from_utf8(output.stdout).unwrap();
        let mut lines = output_str
            .lines()
            .filter(|line| line.contains("com.anubhavitis.peeksy"));

        // if no lines
        let line = lines.next();
        match line {
            Some(line) => {
                let pid = line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(0);
                Ok(pid)
            }
            None => Err(anyhow::anyhow!("LaunchD plist is not running")),
        }
    }

    pub async fn is_loaded(&self) -> bool {
        match self.status().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn is_running(&self) -> bool {
        match self.status().await {
            Ok(pid) => pid > 0,
            Err(_) => false,
        }
    }

    pub async fn load(&self) {
        if self.is_loaded().await {
            // if loaded, but not running, then unload first.
            if self.is_running().await {
                info!("LaunchD plist is already running");
                return;
            }

            info!("LaunchD plist is already loaded, but not running. Reloading...");
            self.unload().await;
            return;
        }

        info!("loading LaunchD plist path: {}", self.plist_path.display());
        let output = Command::new("launchctl")
            .args(["load", self.plist_path.to_str().unwrap()])
            .output()
            .unwrap();
        info!("LaunchD plist loaded successfully: {}", output.status);
    }

    pub async fn unload(&self) {
        if !self.is_loaded().await {
            info!("LaunchD plist is not running");
            return;
        }

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
