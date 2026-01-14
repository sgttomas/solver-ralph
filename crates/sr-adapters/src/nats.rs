//! NATS JetStream Message Bus Adapter (D-21)
//!
//! Implements the MessageBus port using NATS JetStream for reliable
//! event-driven orchestration messaging.
//!
//! Per SR-SPEC §4.6 and SR-PLAN D-21:
//! - Message subjects and payload schemas are defined and versioned
//! - Publisher and consumer handle redelivery deterministically
//! - At-least-once semantics handled without duplicating logical state transitions
//!
//! Key features:
//! - JetStream for durability and replay
//! - Stream per domain (sr-events, sr-commands, sr-queries)
//! - Idempotent message processing via message hash
//! - Redelivery handling with ack/nak/term

use async_nats::{
    jetstream::{
        self,
        consumer::{pull::Config as PullConsumerConfig, AckPolicy, DeliverPolicy, ReplayPolicy},
        stream::Config as StreamConfig,
        Context as JetStreamContext,
    },
    Client as NatsClient,
};
use serde::{Deserialize, Serialize};
use sr_ports::{MessageBus, MessageBusError, MessageSubscription};
use std::{collections::HashSet, future::Future, pin::Pin, sync::Arc};
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

/// NATS JetStream configuration
#[derive(Debug, Clone)]
pub struct NatsConfig {
    /// NATS server URL (e.g., "nats://localhost:4222")
    pub url: String,
    /// Stream prefix for SOLVER-Ralph events
    pub stream_prefix: String,
    /// Default consumer name prefix
    pub consumer_prefix: String,
    /// Message TTL in seconds (default: 7 days)
    pub message_ttl_secs: i64,
    /// Max messages per subject (default: unlimited = -1)
    pub max_msgs_per_subject: i64,
    /// Enable duplicate detection window (default: 2 minutes)
    pub duplicate_window_secs: i64,
}

impl Default for NatsConfig {
    fn default() -> Self {
        Self {
            url: "nats://localhost:4222".to_string(),
            stream_prefix: "sr".to_string(),
            consumer_prefix: "sr-consumer".to_string(),
            message_ttl_secs: 7 * 24 * 60 * 60, // 7 days
            max_msgs_per_subject: -1,           // Unlimited
            duplicate_window_secs: 120,         // 2 minutes
        }
    }
}

impl NatsConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string()),
            stream_prefix: std::env::var("NATS_STREAM_PREFIX").unwrap_or_else(|_| "sr".to_string()),
            consumer_prefix: std::env::var("NATS_CONSUMER_PREFIX")
                .unwrap_or_else(|_| "sr-consumer".to_string()),
            message_ttl_secs: std::env::var("NATS_MESSAGE_TTL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(7 * 24 * 60 * 60),
            max_msgs_per_subject: std::env::var("NATS_MAX_MSGS_PER_SUBJECT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(-1),
            duplicate_window_secs: std::env::var("NATS_DUPLICATE_WINDOW_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(120),
        }
    }
}

// ============================================================================
// Message Contracts
// ============================================================================

/// Message envelope for all SOLVER-Ralph messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    /// Schema version for forward compatibility
    pub schema_version: String,
    /// Message type (matches event_type from domain)
    pub message_type: String,
    /// Unique message ID (from event_id)
    pub message_id: String,
    /// Correlation ID for tracing
    pub correlation_id: Option<String>,
    /// Causation ID for event chains
    pub causation_id: Option<String>,
    /// Timestamp of message creation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Actor who triggered this message
    pub actor_id: String,
    /// Actor type (HUMAN, AGENT, SYSTEM)
    pub actor_kind: String,
    /// The actual payload
    pub payload: serde_json::Value,
    /// Idempotency key (message hash from outbox)
    pub idempotency_key: String,
}

/// Current schema version
pub const SCHEMA_VERSION: &str = "1.0";

/// Stream names for different domains
pub mod streams {
    /// Events stream - domain events for projections and reactions
    pub const EVENTS: &str = "events";
    /// Commands stream - orchestration commands
    pub const COMMANDS: &str = "commands";
    /// Queries stream - request/reply queries (rarely used)
    pub const QUERIES: &str = "queries";
}

/// Subject patterns for message routing
pub mod subjects {
    // Event subjects (matching outbox topic_for_event)
    pub const LOOP_EVENTS: &str = "sr.events.loop";
    pub const ITERATION_EVENTS: &str = "sr.events.iteration";
    pub const CANDIDATE_EVENTS: &str = "sr.events.candidate";
    pub const RUN_EVENTS: &str = "sr.events.run";
    pub const ORACLE_EVENTS: &str = "sr.events.oracle";
    pub const GOVERNANCE_EVENTS: &str = "sr.events.governance";
    pub const FREEZE_EVENTS: &str = "sr.events.freeze";
    pub const STALENESS_EVENTS: &str = "sr.events.staleness";
    pub const APPROVAL_EVENTS: &str = "sr.events.approval";
    pub const EXCEPTION_EVENTS: &str = "sr.events.exception";
    pub const DECISION_EVENTS: &str = "sr.events.decision";
    pub const WORKSURFACE_EVENTS: &str = "sr.events.worksurface";
    pub const OTHER_EVENTS: &str = "sr.events.other";

    // Command subjects
    pub const START_ITERATION: &str = "sr.commands.iteration.start";
    pub const COMPLETE_ITERATION: &str = "sr.commands.iteration.complete";
    pub const REGISTER_CANDIDATE: &str = "sr.commands.candidate.register";
    pub const RUN_ORACLE: &str = "sr.commands.oracle.run";

    /// Get all event subjects for stream binding
    pub fn all_event_subjects() -> Vec<String> {
        vec![
            LOOP_EVENTS.to_string(),
            ITERATION_EVENTS.to_string(),
            CANDIDATE_EVENTS.to_string(),
            RUN_EVENTS.to_string(),
            ORACLE_EVENTS.to_string(),
            GOVERNANCE_EVENTS.to_string(),
            FREEZE_EVENTS.to_string(),
            STALENESS_EVENTS.to_string(),
            APPROVAL_EVENTS.to_string(),
            EXCEPTION_EVENTS.to_string(),
            DECISION_EVENTS.to_string(),
            WORKSURFACE_EVENTS.to_string(),
            OTHER_EVENTS.to_string(),
        ]
    }

    /// Get all command subjects for stream binding
    pub fn all_command_subjects() -> Vec<String> {
        vec![
            START_ITERATION.to_string(),
            COMPLETE_ITERATION.to_string(),
            REGISTER_CANDIDATE.to_string(),
            RUN_ORACLE.to_string(),
        ]
    }
}

// ============================================================================
// NATS Message Bus
// ============================================================================

/// NATS JetStream message bus implementation
pub struct NatsMessageBus {
    client: NatsClient,
    jetstream: JetStreamContext,
    config: NatsConfig,
    /// Track processed idempotency keys (in-memory, can be replaced with Redis)
    processed_keys: Arc<RwLock<HashSet<String>>>,
}

impl NatsMessageBus {
    /// Connect to NATS and initialize JetStream
    pub async fn connect(config: NatsConfig) -> Result<Self, MessageBusError> {
        info!(url = %config.url, "Connecting to NATS");

        let client = async_nats::connect(&config.url).await.map_err(|e| {
            MessageBusError::ConnectionError {
                message: format!("Failed to connect to NATS: {}", e),
            }
        })?;

        let jetstream = jetstream::new(client.clone());

        let bus = Self {
            client,
            jetstream,
            config,
            processed_keys: Arc::new(RwLock::new(HashSet::new())),
        };

        // Initialize streams
        bus.ensure_streams().await?;

        info!("NATS JetStream message bus initialized");
        Ok(bus)
    }

    /// Ensure required JetStream streams exist
    async fn ensure_streams(&self) -> Result<(), MessageBusError> {
        // Events stream
        self.ensure_stream(streams::EVENTS, subjects::all_event_subjects())
            .await?;

        // Commands stream
        self.ensure_stream(streams::COMMANDS, subjects::all_command_subjects())
            .await?;

        Ok(())
    }

    /// Create or update a stream
    async fn ensure_stream(
        &self,
        name: &str,
        subjects: Vec<String>,
    ) -> Result<(), MessageBusError> {
        let stream_name = format!("{}-{}", self.config.stream_prefix, name);

        let config = StreamConfig {
            name: stream_name.clone(),
            subjects,
            max_age: std::time::Duration::from_secs(self.config.message_ttl_secs as u64),
            duplicate_window: std::time::Duration::from_secs(
                self.config.duplicate_window_secs as u64,
            ),
            ..Default::default()
        };

        match self.jetstream.get_stream(&stream_name).await {
            Ok(stream) => {
                debug!(stream = %stream_name, "Stream exists, updating config");
                self.jetstream
                    .update_stream(config.clone())
                    .await
                    .map_err(|e| MessageBusError::ConnectionError {
                        message: format!("Failed to update stream: {}", e),
                    })?;
            }
            Err(_) => {
                info!(stream = %stream_name, "Creating new stream");
                self.jetstream.create_stream(config).await.map_err(|e| {
                    MessageBusError::ConnectionError {
                        message: format!("Failed to create stream: {}", e),
                    }
                })?;
            }
        }

        Ok(())
    }

    /// Create a pull consumer for a subject pattern
    pub async fn create_consumer(
        &self,
        stream_name: &str,
        consumer_name: &str,
        filter_subject: Option<&str>,
    ) -> Result<NatsConsumer, MessageBusError> {
        let full_stream_name = format!("{}-{}", self.config.stream_prefix, stream_name);
        let full_consumer_name = format!("{}-{}", self.config.consumer_prefix, consumer_name);

        let stream = self
            .jetstream
            .get_stream(&full_stream_name)
            .await
            .map_err(|e| MessageBusError::ConnectionError {
                message: format!("Failed to get stream: {}", e),
            })?;

        let consumer_config = PullConsumerConfig {
            durable_name: Some(full_consumer_name.clone()),
            ack_policy: AckPolicy::Explicit,
            deliver_policy: DeliverPolicy::All,
            replay_policy: ReplayPolicy::Instant,
            filter_subject: filter_subject.map(|s| s.to_string()).unwrap_or_default(),
            ..Default::default()
        };

        let consumer = stream
            .get_or_create_consumer(&full_consumer_name, consumer_config)
            .await
            .map_err(|e| MessageBusError::ConnectionError {
                message: format!("Failed to create consumer: {}", e),
            })?;

        info!(
            stream = %full_stream_name,
            consumer = %full_consumer_name,
            "Consumer created"
        );

        Ok(NatsConsumer {
            consumer,
            processed_keys: self.processed_keys.clone(),
        })
    }

    /// Check if a message was already processed (idempotency check)
    pub async fn is_processed(&self, idempotency_key: &str) -> bool {
        let keys = self.processed_keys.read().await;
        keys.contains(idempotency_key)
    }

    /// Mark a message as processed
    pub async fn mark_processed(&self, idempotency_key: &str) {
        let mut keys = self.processed_keys.write().await;
        keys.insert(idempotency_key.to_string());
    }

    /// Publish with idempotency key for duplicate detection
    #[instrument(skip(self, payload), fields(subject = %subject))]
    pub async fn publish_with_id(
        &self,
        subject: &str,
        payload: &[u8],
        idempotency_key: &str,
    ) -> Result<(), MessageBusError> {
        let ack = self
            .jetstream
            .publish_with_headers(
                subject.to_string(),
                {
                    let mut headers = async_nats::HeaderMap::new();
                    headers.insert("Nats-Msg-Id", idempotency_key.to_string());
                    headers
                },
                payload.to_vec().into(),
            )
            .await
            .map_err(|e| MessageBusError::PublishError {
                message: format!("Failed to publish: {}", e),
            })?;

        // Wait for ack from JetStream
        ack.await.map_err(|e| MessageBusError::PublishError {
            message: format!("Failed to get publish ack: {}", e),
        })?;

        debug!(subject = %subject, "Message published to JetStream");
        Ok(())
    }
}

impl MessageBus for NatsMessageBus {
    #[instrument(skip(self, payload), fields(subject = %subject))]
    async fn publish(&self, subject: &str, payload: &[u8]) -> Result<(), MessageBusError> {
        // For non-idempotent publish, generate a unique ID
        let msg_id = ulid::Ulid::new().to_string();
        self.publish_with_id(subject, payload, &msg_id).await
    }

    async fn subscribe(
        &self,
        subject: &str,
    ) -> Result<Box<dyn MessageSubscription>, MessageBusError> {
        // Determine which stream the subject belongs to
        let stream_name = if subject.starts_with("sr.events.") {
            streams::EVENTS
        } else if subject.starts_with("sr.commands.") {
            streams::COMMANDS
        } else {
            streams::EVENTS // Default to events
        };

        // Create a unique consumer for this subscription
        let consumer_name = format!("sub-{}", ulid::Ulid::new());
        let consumer = self
            .create_consumer(stream_name, &consumer_name, Some(subject))
            .await?;

        Ok(Box::new(consumer))
    }
}

// ============================================================================
// NATS Consumer
// ============================================================================

/// NATS JetStream consumer for message subscription
pub struct NatsConsumer {
    consumer: jetstream::consumer::Consumer<jetstream::consumer::pull::Config>,
    processed_keys: Arc<RwLock<HashSet<String>>>,
}

impl NatsConsumer {
    /// Fetch a batch of messages
    pub async fn fetch(&self, batch_size: usize) -> Result<Vec<NatsMessage>, MessageBusError> {
        let messages = self
            .consumer
            .fetch()
            .max_messages(batch_size)
            .messages()
            .await
            .map_err(|e| MessageBusError::ConnectionError {
                message: format!("Failed to fetch messages: {}", e),
            })?;

        // Collect messages
        use futures::StreamExt;
        let collected: Vec<_> = messages.take(batch_size).collect().await;

        let mut result = Vec::new();
        for msg_result in collected {
            match msg_result {
                Ok(msg) => {
                    result.push(NatsMessage {
                        inner: msg,
                        processed_keys: self.processed_keys.clone(),
                    });
                }
                Err(e) => {
                    warn!(error = %e, "Failed to receive message");
                }
            }
        }

        Ok(result)
    }

    /// Process messages with a handler, handling redelivery
    pub async fn process<F, Fut>(
        &self,
        batch_size: usize,
        handler: F,
    ) -> Result<usize, MessageBusError>
    where
        F: Fn(MessageEnvelope) -> Fut,
        Fut: std::future::Future<Output = Result<(), MessageBusError>>,
    {
        let messages = self.fetch(batch_size).await?;
        let mut processed_count = 0;

        for msg in messages {
            // Parse envelope
            let envelope: MessageEnvelope = match serde_json::from_slice(&msg.inner.payload) {
                Ok(e) => e,
                Err(e) => {
                    error!(error = %e, "Failed to parse message envelope");
                    msg.term().await?;
                    continue;
                }
            };

            // Idempotency check
            {
                let keys = self.processed_keys.read().await;
                if keys.contains(&envelope.idempotency_key) {
                    debug!(
                        key = %envelope.idempotency_key,
                        "Skipping already processed message"
                    );
                    msg.ack().await?;
                    continue;
                }
            }

            // Process the message
            match handler(envelope.clone()).await {
                Ok(()) => {
                    // Mark as processed and ack
                    {
                        let mut keys = self.processed_keys.write().await;
                        keys.insert(envelope.idempotency_key.clone());
                    }
                    msg.ack().await?;
                    processed_count += 1;
                }
                Err(e) => {
                    error!(
                        error = %e,
                        message_id = %envelope.message_id,
                        "Failed to process message"
                    );
                    // NAK for redelivery
                    msg.nak().await?;
                }
            }
        }

        Ok(processed_count)
    }
}

impl MessageSubscription for NatsConsumer {
    fn next(&mut self) -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send + '_>> {
        Box::pin(async move {
            let messages = self.fetch(1).await.ok()?;
            if let Some(msg) = messages.into_iter().next() {
                let payload = msg.inner.payload.to_vec();
                // Auto-ack for simple subscription interface
                let _ = msg.ack().await;
                Some(payload)
            } else {
                None
            }
        })
    }
}

// ============================================================================
// NATS Message Wrapper
// ============================================================================

/// Wrapper around NATS message with ack/nak/term methods
pub struct NatsMessage {
    inner: jetstream::Message,
    processed_keys: Arc<RwLock<HashSet<String>>>,
}

impl NatsMessage {
    /// Acknowledge successful processing
    pub async fn ack(&self) -> Result<(), MessageBusError> {
        self.inner
            .ack()
            .await
            .map_err(|e| MessageBusError::ConnectionError {
                message: format!("Failed to ack: {}", e),
            })
    }

    /// Negative acknowledge for redelivery
    pub async fn nak(&self) -> Result<(), MessageBusError> {
        self.inner
            .ack_with(jetstream::AckKind::Nak(None))
            .await
            .map_err(|e| MessageBusError::ConnectionError {
                message: format!("Failed to nak: {}", e),
            })
    }

    /// Terminate message (don't redeliver)
    pub async fn term(&self) -> Result<(), MessageBusError> {
        self.inner
            .ack_with(jetstream::AckKind::Term)
            .await
            .map_err(|e| MessageBusError::ConnectionError {
                message: format!("Failed to term: {}", e),
            })
    }

    /// Get the message payload
    pub fn payload(&self) -> &[u8] {
        &self.inner.payload
    }

    /// Get the subject
    pub fn subject(&self) -> &str {
        self.inner.subject.as_str()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a message envelope from an outbox entry
pub fn create_envelope(
    message_type: &str,
    message_id: &str,
    actor_id: &str,
    actor_kind: &str,
    payload: serde_json::Value,
    idempotency_key: &str,
    correlation_id: Option<String>,
    causation_id: Option<String>,
) -> MessageEnvelope {
    MessageEnvelope {
        schema_version: SCHEMA_VERSION.to_string(),
        message_type: message_type.to_string(),
        message_id: message_id.to_string(),
        correlation_id,
        causation_id,
        timestamp: chrono::Utc::now(),
        actor_id: actor_id.to_string(),
        actor_kind: actor_kind.to_string(),
        payload,
        idempotency_key: idempotency_key.to_string(),
    }
}

/// Serialize a message envelope for publishing
pub fn serialize_envelope(envelope: &MessageEnvelope) -> Result<Vec<u8>, MessageBusError> {
    serde_json::to_vec(envelope).map_err(|e| MessageBusError::PublishError {
        message: format!("Failed to serialize envelope: {}", e),
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = NatsConfig::default();
        assert_eq!(config.url, "nats://localhost:4222");
        assert_eq!(config.stream_prefix, "sr");
        assert_eq!(config.message_ttl_secs, 7 * 24 * 60 * 60);
    }

    #[test]
    fn test_all_event_subjects() {
        let subjects = subjects::all_event_subjects();
        assert!(subjects.contains(&subjects::LOOP_EVENTS.to_string()));
        assert!(subjects.contains(&subjects::ITERATION_EVENTS.to_string()));
        assert!(subjects.contains(&subjects::CANDIDATE_EVENTS.to_string()));
    }

    #[test]
    fn test_create_envelope() {
        let envelope = create_envelope(
            "LoopCreated",
            "evt_123",
            "system",
            "SYSTEM",
            serde_json::json!({"loop_id": "loop_abc"}),
            "sha256:abc123",
            None,
            None,
        );

        assert_eq!(envelope.schema_version, SCHEMA_VERSION);
        assert_eq!(envelope.message_type, "LoopCreated");
        assert_eq!(envelope.idempotency_key, "sha256:abc123");
    }

    #[test]
    fn test_serialize_envelope() {
        let envelope = create_envelope(
            "TestEvent",
            "evt_test",
            "test",
            "SYSTEM",
            serde_json::json!({}),
            "key123",
            None,
            None,
        );

        let bytes = serialize_envelope(&envelope).unwrap();
        let parsed: MessageEnvelope = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(parsed.message_type, "TestEvent");
    }

    #[test]
    fn test_subject_patterns() {
        assert!(subjects::LOOP_EVENTS.starts_with("sr.events."));
        assert!(subjects::START_ITERATION.starts_with("sr.commands."));
    }
}
