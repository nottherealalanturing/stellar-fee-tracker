//! Extremes Tracker for min/max fee values

use crate::insights::{
    types::*,
    error::InsightsError,
    config::ExtremesConfig,
};

/// Tracker for minimum and maximum fee values
pub struct ExtremesTracker {
    config: ExtremesConfig,
    // Implementation will be added in later tasks
}

impl ExtremesTracker {
    /// Create a new extremes tracker
    pub fn new(config: ExtremesConfig) -> Self {
        Self {
            config,
        }
    }
    
    /// Update with new fee data
    pub fn update_with_fees(&mut self, _fees: &[FeeDataPoint]) -> Result<(), InsightsError> {
        // Implementation will be added in later tasks
        todo!("Implementation pending")
    }
    
    /// Get current extremes
    pub fn get_current_extremes(&self) -> Result<FeeExtremes, InsightsError> {
        // Implementation will be added in later tasks
        todo!("Implementation pending")
    }
}