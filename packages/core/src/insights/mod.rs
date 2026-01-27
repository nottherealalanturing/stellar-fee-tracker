//! Fee Insights Module
//! 
//! This module provides analytical insights from raw blockchain fee data,
//! including rolling averages, extremes tracking, and congestion detection.

pub mod engine;
pub mod calculator;
pub mod tracker;
pub mod detector;
pub mod types;
pub mod error;
pub mod config;
pub mod provider;
pub mod horizon_adapter;

pub use engine::FeeInsightsEngine;
pub use types::*;
pub use error::InsightsError;
pub use config::InsightsConfig;
pub use provider::{FeeDataProvider, ProviderMetadata};
pub use horizon_adapter::HorizonFeeDataProvider;