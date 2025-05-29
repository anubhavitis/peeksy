use clap::Parser;
use peeksy::{cli, config, logger::logger};

#[tokio::main]
async fn main() {
    logger::setup_logger();
    config::setup::setup_config().expect("Failed to setup config");
    // check_permissions();
    let args = cli::cli::Args::parse();
    args.execute().await;
}
