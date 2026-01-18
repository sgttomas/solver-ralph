//! Flake control mechanisms for oracle tests
//!
//! Per SR-PLAN D-26 acceptance criteria:
//! - Flake controls exist (timeouts, retries policy) and are recorded
//!
//! This module provides:
//! - Configurable retry policies with exponential backoff
//! - Timeout policies per service type
//! - Recording of all retry attempts for evidence bundles

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

/// Flake control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeControl {
    /// Retry policy configuration
    pub retry_policy: RetryPolicy,
    /// Timeout policy per service
    pub timeout_policy: TimeoutPolicy,
    /// Recorded retry attempts
    #[serde(default)]
    pub retry_attempts: Vec<RetryAttempt>,
    /// Overall started timestamp
    pub started_at: Option<DateTime<Utc>>,
    /// Overall completed timestamp
    pub completed_at: Option<DateTime<Utc>>,
}

impl Default for FlakeControl {
    fn default() -> Self {
        Self {
            retry_policy: RetryPolicy::default(),
            timeout_policy: TimeoutPolicy::default(),
            retry_attempts: Vec::new(),
            started_at: None,
            completed_at: None,
        }
    }
}

impl FlakeControl {
    /// Create a new flake control with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom retry policy
    pub fn with_retry_policy(retry_policy: RetryPolicy) -> Self {
        Self {
            retry_policy,
            ..Default::default()
        }
    }

    /// Mark started
    pub fn start(&mut self) {
        self.started_at = Some(Utc::now());
    }

    /// Mark completed
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }

    /// Record a retry attempt
    pub fn record_attempt(&mut self, attempt: RetryAttempt) {
        self.retry_attempts.push(attempt);
    }

    /// Execute an operation with retry logic
    pub async fn execute_with_retry<F, Fut, T, E>(
        &mut self,
        operation_name: &str,
        operation: F,
    ) -> Result<T, FlakeError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut attempts = 0;
        let mut current_delay = self.retry_policy.initial_delay;

        loop {
            attempts += 1;
            let attempt_start = Utc::now();

            debug!(
                operation = operation_name,
                attempt = attempts,
                "Executing operation"
            );

            match operation().await {
                Ok(result) => {
                    self.record_attempt(RetryAttempt {
                        operation: operation_name.to_string(),
                        attempt_number: attempts,
                        started_at: attempt_start,
                        completed_at: Utc::now(),
                        success: true,
                        error: None,
                        delay_ms: if attempts > 1 {
                            Some(current_delay.as_millis() as u64)
                        } else {
                            None
                        },
                    });

                    if attempts > 1 {
                        info!(
                            operation = operation_name,
                            attempts = attempts,
                            "Operation succeeded after retries"
                        );
                    }

                    return Ok(result);
                }
                Err(e) => {
                    let error_msg = e.to_string();

                    self.record_attempt(RetryAttempt {
                        operation: operation_name.to_string(),
                        attempt_number: attempts,
                        started_at: attempt_start,
                        completed_at: Utc::now(),
                        success: false,
                        error: Some(error_msg.clone()),
                        delay_ms: if attempts > 1 {
                            Some(current_delay.as_millis() as u64)
                        } else {
                            None
                        },
                    });

                    if attempts >= self.retry_policy.max_retries {
                        warn!(
                            operation = operation_name,
                            attempts = attempts,
                            error = %error_msg,
                            "Operation failed after max retries"
                        );

                        return Err(FlakeError::MaxRetriesExceeded {
                            operation: operation_name.to_string(),
                            attempts,
                            last_error: error_msg,
                        });
                    }

                    // Apply jitter
                    let jitter = if self.retry_policy.jitter_factor > 0.0 {
                        let jitter_range = (current_delay.as_millis() as f64
                            * self.retry_policy.jitter_factor)
                            as u64;
                        Duration::from_millis(rand::random::<u64>() % jitter_range.max(1))
                    } else {
                        Duration::ZERO
                    };

                    let delay_with_jitter = current_delay + jitter;

                    debug!(
                        operation = operation_name,
                        attempt = attempts,
                        delay_ms = delay_with_jitter.as_millis(),
                        "Retrying after delay"
                    );

                    sleep(delay_with_jitter).await;

                    // Calculate next delay with exponential backoff
                    current_delay = Duration::from_millis(
                        (current_delay.as_millis() as f64 * self.retry_policy.backoff_multiplier)
                            as u64,
                    )
                    .min(self.retry_policy.max_delay);
                }
            }
        }
    }

    /// Get total retry count
    pub fn total_retries(&self) -> usize {
        self.retry_attempts
            .iter()
            .filter(|a| a.attempt_number > 1)
            .count()
    }

    /// Get failed attempt count
    pub fn failed_attempts(&self) -> usize {
        self.retry_attempts.iter().filter(|a| !a.success).count()
    }
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay before first retry
    #[serde(with = "duration_millis")]
    pub initial_delay: Duration,
    /// Maximum delay between retries
    #[serde(with = "duration_millis")]
    pub max_delay: Duration,
    /// Backoff multiplier (e.g., 2.0 for doubling)
    pub backoff_multiplier: f64,
    /// Jitter factor (0.0 to 1.0)
    pub jitter_factor: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

impl RetryPolicy {
    /// Create a strict policy (no retries)
    pub fn strict() -> Self {
        Self {
            max_retries: 1,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            backoff_multiplier: 1.0,
            jitter_factor: 0.0,
        }
    }

    /// Create a lenient policy (more retries)
    pub fn lenient() -> Self {
        Self {
            max_retries: 5,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_factor: 0.2,
        }
    }

    /// Create policy for network operations
    pub fn network() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
            jitter_factor: 0.1,
        }
    }
}

/// Timeout policy per service type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutPolicy {
    /// Database connection timeout
    #[serde(with = "duration_millis")]
    pub database_connect: Duration,
    /// Database query timeout
    #[serde(with = "duration_millis")]
    pub database_query: Duration,
    /// Object storage timeout
    #[serde(with = "duration_millis")]
    pub object_storage: Duration,
    /// Message bus timeout
    #[serde(with = "duration_millis")]
    pub message_bus: Duration,
    /// API request timeout
    #[serde(with = "duration_millis")]
    pub api_request: Duration,
    /// E2E test timeout
    #[serde(with = "duration_millis")]
    pub e2e_test: Duration,
}

impl Default for TimeoutPolicy {
    fn default() -> Self {
        Self {
            database_connect: Duration::from_secs(10),
            database_query: Duration::from_secs(30),
            object_storage: Duration::from_secs(30),
            message_bus: Duration::from_secs(10),
            api_request: Duration::from_secs(30),
            e2e_test: Duration::from_secs(300),
        }
    }
}

/// Record of a retry attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryAttempt {
    /// Operation name
    pub operation: String,
    /// Attempt number (1-indexed)
    pub attempt_number: u32,
    /// Attempt start time
    pub started_at: DateTime<Utc>,
    /// Attempt completion time
    pub completed_at: DateTime<Utc>,
    /// Whether the attempt succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Delay before this attempt in milliseconds
    pub delay_ms: Option<u64>,
}

/// Flake control errors
#[derive(Debug, thiserror::Error)]
pub enum FlakeError {
    #[error("Max retries exceeded for '{operation}' after {attempts} attempts: {last_error}")]
    MaxRetriesExceeded {
        operation: String,
        attempts: u32,
        last_error: String,
    },

    #[error("Operation timeout: {0}")]
    Timeout(String),
}

/// Serde helper for Duration in milliseconds
mod duration_millis {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (duration.as_millis() as u64).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_flake_control() {
        let control = FlakeControl::default();
        assert_eq!(control.retry_policy.max_retries, 3);
        assert!(control.retry_attempts.is_empty());
    }

    #[test]
    fn test_retry_policy_strict() {
        let policy = RetryPolicy::strict();
        assert_eq!(policy.max_retries, 1);
        assert_eq!(policy.initial_delay, Duration::ZERO);
    }

    #[test]
    fn test_retry_policy_lenient() {
        let policy = RetryPolicy::lenient();
        assert_eq!(policy.max_retries, 5);
    }

    #[test]
    fn test_timeout_policy_default() {
        let policy = TimeoutPolicy::default();
        assert_eq!(policy.database_connect, Duration::from_secs(10));
        assert_eq!(policy.e2e_test, Duration::from_secs(300));
    }

    #[test]
    fn test_retry_attempt_serialization() {
        let attempt = RetryAttempt {
            operation: "test_op".to_string(),
            attempt_number: 1,
            started_at: Utc::now(),
            completed_at: Utc::now(),
            success: true,
            error: None,
            delay_ms: None,
        };

        let json = serde_json::to_string(&attempt).unwrap();
        let parsed: RetryAttempt = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.operation, "test_op");
        assert!(parsed.success);
    }

    #[test]
    fn test_flake_control_record_attempt() {
        let mut control = FlakeControl::new();
        control.start();

        control.record_attempt(RetryAttempt {
            operation: "test".to_string(),
            attempt_number: 1,
            started_at: Utc::now(),
            completed_at: Utc::now(),
            success: false,
            error: Some("test error".to_string()),
            delay_ms: None,
        });

        control.record_attempt(RetryAttempt {
            operation: "test".to_string(),
            attempt_number: 2,
            started_at: Utc::now(),
            completed_at: Utc::now(),
            success: true,
            error: None,
            delay_ms: Some(500),
        });

        control.complete();

        assert_eq!(control.retry_attempts.len(), 2);
        assert_eq!(control.failed_attempts(), 1);
        assert_eq!(control.total_retries(), 1);
    }

    #[tokio::test]
    async fn test_execute_with_retry_success() {
        let mut control = FlakeControl::new();
        control.retry_policy.max_retries = 3;

        let result = control
            .execute_with_retry::<_, _, i32, String>("test", || async { Ok(42) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(control.retry_attempts.len(), 1);
        assert!(control.retry_attempts[0].success);
    }

    #[tokio::test]
    async fn test_execute_with_retry_failure() {
        let mut control = FlakeControl::new();
        control.retry_policy.max_retries = 2;
        control.retry_policy.initial_delay = Duration::from_millis(10);

        let attempt_count = std::sync::atomic::AtomicU32::new(0);

        let result = control
            .execute_with_retry::<_, _, i32, &str>("test", || {
                let _ = attempt_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async move { Err::<i32, _>("always fails") }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(control.retry_attempts.len(), 2);
    }

    #[test]
    fn test_flake_control_serialization() {
        let control = FlakeControl::default();
        let json = serde_json::to_string_pretty(&control).unwrap();
        let parsed: FlakeControl = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.retry_policy.max_retries,
            control.retry_policy.max_retries
        );
    }
}
