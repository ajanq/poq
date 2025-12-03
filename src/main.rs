mod cli;
mod config;
mod core;
mod error;
mod languages;
mod templating;
mod utils;

use log::{debug, error, info};

#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Initializing poq");
    info!("Starting poq");
    match cli::commands::run().await {
        Ok(_) => info!("poq completed successfully"),
        Err(e) => {
            error!("poq encountered an error: {}", e);
            std::process::exit(1);
        }
    }
}
