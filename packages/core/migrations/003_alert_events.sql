-- Migration 003: Alert events log
-- Records every triggered alert dispatch for auditing and debugging.

CREATE TABLE IF NOT EXISTS alert_events (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    config_id    INTEGER REFERENCES alert_configs(id),
    severity     TEXT    NOT NULL,
    peak_fee     INTEGER NOT NULL,
    baseline_fee REAL    NOT NULL,
    spike_ratio  REAL    NOT NULL,
    webhook_url  TEXT    NOT NULL,
    delivered    INTEGER NOT NULL DEFAULT 0,  -- 1 = success, 0 = failed
    triggered_at TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_alert_events_triggered_at
    ON alert_events (triggered_at);
