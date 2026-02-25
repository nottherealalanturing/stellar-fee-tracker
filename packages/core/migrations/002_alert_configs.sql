-- Migration 002: Alert configurations
-- Stores webhook targets for outbound fee-spike notifications.

CREATE TABLE IF NOT EXISTS alert_configs (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    webhook_url TEXT    NOT NULL,
    threshold   TEXT    NOT NULL DEFAULT 'Major',
    enabled     INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);
