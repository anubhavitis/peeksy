use crate::{
    daemon::{daemon, pid},
    launchd::launchd,
    utils,
};

const NOTE: &str = "⚠️ Note: If you have updated the screenshot directory, you need to restart the daemon.\n Use `peeksy restart` to restart the daemon.";

pub async fn is_daemon_running() -> (bool, Option<u32>) {
    let pid = pid::get_pid();
    match pid {
        Ok(pid) => {
            // check if process is running at saved pid
            let process = std::process::Command::new("ps")
                .args(["-p", &pid.to_string()])
                .output()
                .unwrap();
            (process.status.success(), Some(pid))
        }
        Err(_) => (false, None),
    }
}

pub async fn status_daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!(
            "Peeksy daemon is running with PID {}\n\n{}",
            pid.unwrap(),
            NOTE
        );
    } else {
        println!("Peeksy daemon is not running");
    }
}

pub async fn restart_daemon() {
    stop_daemon().await;
    start_daemon().await;
}

pub async fn stop_daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if !is_running {
        println!("Peeksy daemon is not running");
        return;
    }

    println!("Stopping Peeksy daemon with PID {}", pid.unwrap());

    if let Err(e) = std::process::Command::new("kill")
        .arg(pid.unwrap().to_string())
        .output()
    {
        println!("Failed to stop daemon: {}", e);
        return;
    }

    // Give process time to terminate
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Verify daemon was stopped
    let (still_running, _) = is_daemon_running().await;
    if still_running {
        println!("❌ Failed to stop Peeksy daemon with PID {}", pid.unwrap());
    } else {
        println!("✅ Peeksy daemon stopped successfully");
    }

    let launchd = launchd::LaunchD::new();
    launchd.unload().await;
}

pub async fn start_daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!(
            "Peeksy daemon is already running with PID {}\n\n{}",
            pid.unwrap(),
            NOTE
        );
        return;
    }

    // Gets the current executable binary: which in this case is peeksy
    let current_exe = std::env::current_exe().expect("Failed to get current executable path");
    let screenshot_dir = utils::ss::get_screenshot_dir();

    // Spawn daemon as separate process. Command `<exec> daemon`
    match std::process::Command::new(&current_exe)
        .arg("daemon")
        .spawn()
    {
        Ok(_) => {
            // Give it a moment to start
            std::thread::sleep(std::time::Duration::from_millis(1000));

            // check if the daemon is successfully running
            let (is_running, pid) = is_daemon_running().await;
            if is_running {
                println!(
                    "✅ Daemon started successfully with PID {} at screenshot directory: {}",
                    pid.unwrap(),
                    screenshot_dir.display()
                );
            } else {
                println!("❌ Failed to start daemon");
            }
        }
        Err(e) => {
            println!("❌ Failed to spawn daemon: {}", e);
        }
    }

    let launchd = launchd::LaunchD::new();
    launchd.load().await;
}

pub async fn daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!("Peeksy daemon is already running with PID {}", pid.unwrap());
        return;
    }
    daemon::run().await;
}
