use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub stellar_network: StellarNetwork,
    pub horizon_url: String,
    pub poll_interval_seconds: u64,
}

#[derive(Debug, Clone)]
pub enum StellarNetwork {
    Testnet,
    Mainnet,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let stellar_network = match env::var("STELLAR_NETWORK")
            .map_err(|_| "STELLAR_NETWORK is required")?
            .as_str()
        {
            "testnet" => StellarNetwork::Testnet,
            "mainnet" => StellarNetwork::Mainnet,
            other => return Err(format!("Invalid STELLAR_NETWORK: {}", other)),
        };

        let horizon_url =
            env::var("HORIZON_URL").map_err(|_| "HORIZON_URL is required")?;

        let poll_interval_seconds = env::var("POLL_INTERVAL_SECONDS")
            .map_err(|_| "POLL_INTERVAL_SECONDS is required")?
            .parse::<u64>()
            .map_err(|_| "POLL_INTERVAL_SECONDS must be a valid number")?;

        Ok(Self {
            stellar_network,
            horizon_url,
            poll_interval_seconds,
        })
    }
}
