use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::signal;

use crate::daemon::peeksy::controller;
use log::info;

pub async fn run() {
    info!("Starting Peeksy daemon");
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    let peeksy_thread_handler = tokio::spawn(async move {
        info!("Starting Peeksy thread");
        controller(shutdown_clone).await;
    });

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
            shutdown.store(true, Ordering::Relaxed);
        }
        _ = peeksy_thread_handler => {
            info!("Peeksy thread exited unexpectedly");
        }
    }

    // Give the thread a moment to clean up
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    info!("Peeksy: Shutting down");
}
