mod config;
mod services;

use crate::services::horizon::HorizonClient;
use dotenvy::dotenv;
use config::Config;

fn main() {
    dotenv().ok();

    let config = Config::from_env()
        .expect("❌ Failed to load environment configuration");

    println!("🚀 Stellar Fee Tracker starting up");
    println!("🔧 Loaded config: {:#?}", config);
   

    let horizon_client = HorizonClient::new(config.horizon_url.clone());
    tracing::info!(
        "Horizon client initialized with base URL: {}",
        horizon_client.base_url()
    );
}


