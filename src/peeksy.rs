use dotenv::dotenv;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, RecvTimeoutError},
        Arc,
    },
};

use chrono::Local;
use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::file::SSController;

fn get_controller() -> Result<SSController, String> {
    dotenv().ok();

    let api_key = match std::env::var("PEEKSY_OPENAI_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            return Err(format!("Error getting api key: {:?}", e));
        }
    };

    let prompt_file_path = match std::env::var("PEEKSY_OPENAI_PROMPT_FILE") {
        Ok(path) => path,
        Err(e) => {
            return Err(format!("Error getting prompt file path: {:?}", e));
        }
    };

    let prompt = std::fs::read_to_string(&prompt_file_path).expect("Failed to read prompt file");

    Ok(SSController::new(api_key, prompt))
}

pub async fn controller(screenshot_dir: PathBuf, shutdown: Arc<AtomicBool>) {
    info!(
        "Starting Peeksy on {} at {}",
        screenshot_dir.display(),
        Local::now()
    );
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).expect("Failed to create watcher");
    watcher
        .watch(&screenshot_dir, RecursiveMode::NonRecursive)
        .expect("Failed to watch directory");

    let ss_controller = get_controller().expect("Failed to setup controller");

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
                        let resp = ss_controller.process_file(&path).await;
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
