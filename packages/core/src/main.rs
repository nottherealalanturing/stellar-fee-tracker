mod config;
mod error;
mod logging;

use dotenvy::dotenv;
use crate::config::Config;
use crate::error::AppError;
use crate::logging::init_logging;

fn main() {
    dotenv().ok();
    init_logging();

    let config = Config::from_env()
        .map_err(AppError::Config)
        .unwrap_or_else(|err| {
            tracing::error!("{}", err);
            std::process::exit(1);
        });

    tracing::info!("Service started with config: {:?}", config);
}