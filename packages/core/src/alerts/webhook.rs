//! Webhook alert delivery.
//!
//! Dispatches HTTP POST notifications to registered webhook targets when a
//! fee spike is detected. Every delivery attempt — successful or not — is
//! persisted in the `alert_events` table via [`FeeRepository::log_alert_event`].
//!
//! This module is the integration point for Issue #31 (webhook delivery) and
//! Issue #32 (alert history). The `dispatch` function is called by the
//! scheduler / insights engine whenever a spike crosses an alert threshold.

use std::sync::Arc;

use chrono::Utc;

use crate::repository::{AlertEvent, FeeRepository};

/// Payload describing a triggered fee-spike alert.
#[derive(Debug, Clone)]
pub struct AlertPayload {
    /// The alert config row id that triggered this dispatch (if known).
    pub config_id: Option<i64>,
    /// Severity label, e.g. "Minor", "Major", "Critical".
    pub severity: String,
    /// Highest fee observed during the spike window (in stroops).
    pub peak_fee: i64,
    /// Rolling baseline fee used for comparison.
    pub baseline_fee: f64,
    /// `peak_fee / baseline_fee`.
    pub spike_ratio: f64,
    /// Destination webhook URL.
    pub webhook_url: String,
}

/// Dispatch a webhook notification and log the outcome to the database.
///
/// The HTTP client (`reqwest`) is not yet wired up in this stub — the
/// `delivered` flag defaults to `false` until Issue #31 lands and the full
/// HTTP POST is implemented.  The repository logging is fully functional.
pub async fn dispatch(payload: AlertPayload, repository: Arc<FeeRepository>) {
    // TODO (Issue #31): perform the actual HTTP POST here and capture success.
    let delivered = false;

    let event = AlertEvent {
        id: None,
        config_id: payload.config_id,
        severity: payload.severity.clone(),
        peak_fee: payload.peak_fee,
        baseline_fee: payload.baseline_fee,
        spike_ratio: payload.spike_ratio,
        webhook_url: payload.webhook_url.clone(),
        delivered,
        triggered_at: Utc::now().to_rfc3339(),
    };

    if let Err(err) = repository.log_alert_event(&event).await {
        tracing::error!(
            "Failed to log alert event for webhook {}: {}",
            payload.webhook_url,
            err
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn dispatch_logs_event_to_database() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = Arc::new(FeeRepository::new(pool));

        let payload = AlertPayload {
            config_id: None,
            severity: "Major".to_string(),
            peak_fee: 8000,
            baseline_fee: 130.5,
            spike_ratio: 61.3,
            webhook_url: "https://hooks.example.com/test".to_string(),
        };

        dispatch(payload, repo.clone()).await;

        let events = repo.query_alert_history(10, None, None).await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].severity, "Major");
        assert_eq!(events[0].peak_fee, 8000);
        // delivered = false until Issue #31 implements the HTTP POST
        assert!(!events[0].delivered);
    }
}
