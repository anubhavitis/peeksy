use clap::Parser;
use peeksy::{cli, config, logger::logger};

#[tokio::main]
async fn main() {
    // initial setups
    logger::setup_logger();
    config::setup::initial_setup().expect("Failed to setup config");

    // check_permissions();
    let args = cli::cli::Args::parse();
    args.execute().await;
}
