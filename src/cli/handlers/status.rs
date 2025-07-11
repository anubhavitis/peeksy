use crate::{
    daemon::{daemon, pid},
    launchd::launchd,
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
    let launchd = launchd::LaunchD::new();
    if launchd.is_loaded().await && launchd.is_running().await {
        println!("Peeksy daemon is already running\n{}", NOTE);
    } else {
        println!("Peeksy daemon is not running");
    }
}

pub async fn restart_daemon() {
    let launchd = launchd::LaunchD::new();
    launchd.unload().await;
    launchd.load().await;
    println!("✅ Peeksy daemon restarted successfully");
}

pub async fn stop_daemon() {
    let launchd = launchd::LaunchD::new();
    launchd.unload().await;
    println!("✅ Peeksy daemon stopped successfully");
}

pub async fn start_daemon() {
    let launchd = launchd::LaunchD::new();
    if launchd.is_loaded().await && launchd.is_running().await {
        println!("Peeksy daemon is already running");
        return;
    }
    launchd.load().await;
    println!("✅ Peeksy daemon started successfully");
}

pub async fn daemon() {
    let (is_running, pid) = is_daemon_running().await;
    if is_running {
        println!("Peeksy daemon is already running with PID {}", pid.unwrap());
        return;
    }
    daemon::run().await;
}
