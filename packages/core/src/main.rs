mod config;

use dotenvy::dotenv;
use config::Config;

fn main() {
    dotenv().ok();

    let config = Config::from_env()
        .expect("❌ Failed to load environment configuration");

    println!("🚀 Stellar Fee Tracker starting up");
    println!("🔧 Loaded config: {:#?}", config);
}
