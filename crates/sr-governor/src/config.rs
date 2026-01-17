//! Governor Service Configuration
//!
//! Configuration for the standalone loop governor service.
//! All values can be set via environment variables.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Governor service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernorConfig {
    /// Database URL for event store connection
    pub database_url: String,

    /// NATS URL for message bus connection
    pub nats_url: String,

    /// Poll interval in milliseconds (default: 1000ms)
    pub poll_interval_ms: u64,

    /// Maximum concurrent loops to process per poll cycle (default: 10)
    pub max_concurrent_loops: usize,

    /// Enable dry-run mode (decisions are logged but not executed)
    pub dry_run: bool,

    /// Health check HTTP port (default: 8081)
    pub health_port: u16,

    /// Log level (default: "info")
    pub log_level: String,

    /// Service name for tracing
    pub service_name: String,
}

impl Default for GovernorConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost:5432/solver_ralph".to_string(),
            nats_url: "nats://localhost:4222".to_string(),
            poll_interval_ms: 1000,
            max_concurrent_loops: 10,
            dry_run: false,
            health_port: 8081,
            log_level: "info".to_string(),
            service_name: "sr-governor".to_string(),
        }
    }
}

impl GovernorConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("SR_DATABASE_URL")
                .unwrap_or_else(|_| Self::default().database_url),
            nats_url: std::env::var("SR_NATS_URL").unwrap_or_else(|_| Self::default().nats_url),
            poll_interval_ms: std::env::var("SR_GOVERNOR_POLL_INTERVAL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            max_concurrent_loops: std::env::var("SR_GOVERNOR_MAX_CONCURRENT_LOOPS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            dry_run: std::env::var("SR_GOVERNOR_DRY_RUN")
                .map(|s| s == "true" || s == "1")
                .unwrap_or(false),
            health_port: std::env::var("SR_GOVERNOR_HEALTH_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8081),
            log_level: std::env::var("SR_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            service_name: std::env::var("SR_SERVICE_NAME")
                .unwrap_or_else(|_| "sr-governor".to_string()),
        }
    }

    /// Get poll interval as Duration
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.poll_interval_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GovernorConfig::default();
        assert_eq!(config.poll_interval_ms, 1000);
        assert_eq!(config.max_concurrent_loops, 10);
        assert!(!config.dry_run);
        assert_eq!(config.health_port, 8081);
    }

    #[test]
    fn test_poll_interval_duration() {
        let config = GovernorConfig::default();
        assert_eq!(config.poll_interval(), Duration::from_millis(1000));
    }
}
