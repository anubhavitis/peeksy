use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, RecvTimeoutError},
    Arc,
};

use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    config,
    daemon::pid,
    manager::{ai::OpenAI, image::SSManager},
    utils::ss::get_screenshot_dir,
};

use tokio::signal;

async fn daemon(shutdown: Arc<AtomicBool>) {
    let screenshot_dir = get_screenshot_dir();
    info!("Peeksy is running on {}", screenshot_dir.display());

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).expect("Failed to create watcher");
    watcher
        .watch(&screenshot_dir, RecursiveMode::NonRecursive)
        .expect("Failed to watch directory");

    let config = config::config::Config::fetch().expect("Failed to fetch config");
    let ai = OpenAI::new(
        config.openai_api_key.unwrap(),
        config.openai_prompt_file_path.unwrap(),
        config.openai_model.unwrap(),
    );
    let ss_controller = SSManager::new(ai);

    info!("Setup complete, Peeksy is ready!");
    while !shutdown.load(Ordering::Relaxed) {
        match rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(event) => {
                if let Ok(Event {
                    kind: EventKind::Create(_),
                    paths,
                    ..
                }) = event
                {
                    for path in paths {
                        info!("Detected new file: {:?}", path);
                        let resp = ss_controller.process_new_ss(&path).await;
                        if let Err(e) = resp {
                            error!("Error processing file: {:?}", e);
                        }
                    }
                }
            }
            Err(RecvTimeoutError::Timeout) => continue,
            Err(e) => error!("Watch error: {:?}", e),
        }
    }

    info!("Shutting down Peeksy thread...");
    watcher.unwatch(&screenshot_dir).ok();
}

pub async fn run() {
    let config = config::config::Config::fetch().expect("Failed to fetch config");
    if !config.ready() {
        error!("[Peeksy Ready] Please update Peeksy config to use Peeksy. Use `peeksy edit-config` to update Peeksy config");
        return;
    }

    let new_pid = std::process::id();
    info!("Starting Peeksy daemon with PID {}", new_pid);

    // save the pid
    pid::save_pid(new_pid);

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    let peeksy_thread_handler = tokio::spawn(async move {
        info!("Starting Peeksy thread...");
        daemon(shutdown_clone).await;
    });

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal. Shutting down...");
            shutdown.store(true, Ordering::Relaxed);
        }
        _ = peeksy_thread_handler => {
            error!("Peeksy thread exited unexpectedly");
        }
    }

    // Give the thread a moment to clean up
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    info!("Peeksy: Shutting down");
}
