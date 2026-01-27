//! Rolling Average Calculator

use crate::insights::{
    types::*,
    error::InsightsError,
    config::AverageConfig,
};

/// Calculator for rolling averages across multiple time windows
pub struct RollingAverageCalculator {
    config: AverageConfig,
    // Implementation will be added in later tasks
}

impl RollingAverageCalculator {
    /// Create a new rolling average calculator
    pub fn new(config: AverageConfig) -> Self {
        Self {
            config,
        }
    }
    
    /// Add a new data point
    pub fn add_data_point(&mut self, _point: FeeDataPoint) {
        // Implementation will be added in later tasks
        todo!("Implementation pending")
    }
    
    /// Calculate averages for all time windows
    pub fn calculate_averages(&self) -> Result<RollingAverages, InsightsError> {
        // Implementation will be added in later tasks
        todo!("Implementation pending")
    }
}