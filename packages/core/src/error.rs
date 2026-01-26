use std::fmt;
use std::error::Error;

/// Unified application error.
///
/// This ensures all layers (config, network, parsing)
/// fail in a predictable and debuggable way.
#[derive(Debug)]
pub enum AppError {
    Config(String),
    Network(String),
    Parse(String),
    Unknown(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Config error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AppError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl Error for AppError {}