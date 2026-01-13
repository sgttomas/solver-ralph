//! SOLVER-Ralph Port Traits
//!
//! This crate defines the port interfaces (traits) that adapters must implement.
//! Per SR-SPEC §4.1 and D-07, these ports include:
//! - EventStore
//! - EvidenceStore
//! - OracleRunner
//! - MessageBus
//! - IdentityProvider
//! - Clock

use std::future::Future;

use chrono::{DateTime, Utc};
use sr_domain::{DomainError, EventEnvelope, TypedRef};

/// Event store port per SR-SPEC §1.5.1
///
/// The append-only event log is the sole source of truth for governance-relevant state.
pub trait EventStore: Send + Sync {
    /// Append events to a stream with optimistic concurrency control
    fn append(
        &self,
        stream_id: &str,
        expected_version: u64,
        events: Vec<EventEnvelope>,
    ) -> impl Future<Output = Result<u64, EventStoreError>> + Send;

    /// Read events from a stream starting at a given sequence
    fn read_stream(
        &self,
        stream_id: &str,
        from_seq: u64,
        limit: usize,
    ) -> impl Future<Output = Result<Vec<EventEnvelope>, EventStoreError>> + Send;

    /// Replay all events in global order
    fn replay_all(
        &self,
        from_global_seq: u64,
        limit: usize,
    ) -> impl Future<Output = Result<Vec<EventEnvelope>, EventStoreError>> + Send;
}

/// Event store errors
#[derive(Debug, thiserror::Error)]
pub enum EventStoreError {
    #[error("Concurrency conflict: expected version {expected}, got {actual}")]
    ConcurrencyConflict { expected: u64, actual: u64 },

    #[error("Stream not found: {stream_id}")]
    StreamNotFound { stream_id: String },

    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },
}

/// Evidence store port per SR-SPEC §1.9
///
/// Evidence bundles are stored in content-addressed storage.
pub trait EvidenceStore: Send + Sync {
    /// Store an evidence bundle, returning its content hash
    fn store(
        &self,
        manifest: &[u8],
        blobs: Vec<(&str, &[u8])>,
    ) -> impl Future<Output = Result<String, EvidenceStoreError>> + Send;

    /// Retrieve an evidence bundle by content hash
    fn retrieve(
        &self,
        content_hash: &str,
    ) -> impl Future<Output = Result<Vec<u8>, EvidenceStoreError>> + Send;

    /// Check if evidence exists
    fn exists(
        &self,
        content_hash: &str,
    ) -> impl Future<Output = Result<bool, EvidenceStoreError>> + Send;
}

/// Evidence store errors
#[derive(Debug, thiserror::Error)]
pub enum EvidenceStoreError {
    #[error("Evidence not found: {content_hash}")]
    NotFound { content_hash: String },

    #[error("Storage error: {message}")]
    StorageError { message: String },

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
}

/// Oracle runner port per SR-SPEC §4.5
///
/// Executes oracle suites in sandboxed containers.
pub trait OracleRunner: Send + Sync {
    /// Execute an oracle suite against a candidate
    fn run(
        &self,
        candidate_id: &str,
        oracle_suite_id: &str,
        oracle_suite_hash: &str,
    ) -> impl Future<Output = Result<OracleRunResult, OracleRunnerError>> + Send;
}

/// Oracle run result
#[derive(Debug, Clone)]
pub struct OracleRunResult {
    pub run_id: String,
    pub evidence_bundle_hash: String,
    pub status: OracleStatus,
    pub environment_fingerprint: serde_json::Value,
}

/// Oracle status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleStatus {
    Pass,
    Fail,
    Error,
}

/// Oracle runner errors
#[derive(Debug, thiserror::Error)]
pub enum OracleRunnerError {
    #[error("Oracle suite not found: {suite_id}")]
    SuiteNotFound { suite_id: String },

    #[error("Execution error: {message}")]
    ExecutionError { message: String },

    #[error("Environment mismatch: {details}")]
    EnvironmentMismatch { details: String },
}

/// Message bus port per SR-SPEC §4.6
///
/// Publishes domain events to NATS subjects.
pub trait MessageBus: Send + Sync {
    /// Publish an event to the message bus
    fn publish(
        &self,
        subject: &str,
        payload: &[u8],
    ) -> impl Future<Output = Result<(), MessageBusError>> + Send;

    /// Subscribe to a subject
    fn subscribe(
        &self,
        subject: &str,
    ) -> impl Future<Output = Result<Box<dyn MessageSubscription>, MessageBusError>> + Send;
}

/// Message subscription trait
pub trait MessageSubscription: Send + Sync {
    /// Receive the next message
    fn next(&mut self) -> impl Future<Output = Option<Vec<u8>>> + Send;
}

/// Message bus errors
#[derive(Debug, thiserror::Error)]
pub enum MessageBusError {
    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    #[error("Publish error: {message}")]
    PublishError { message: String },
}

/// Identity provider port per SR-SPEC §2.1
///
/// Validates OIDC JWTs and derives actor identity.
pub trait IdentityProvider: Send + Sync {
    /// Validate a token and return the actor identity
    fn validate(&self, token: &str) -> impl Future<Output = Result<ActorIdentity, IdentityError>> + Send;
}

/// Actor identity from validated token
#[derive(Debug, Clone)]
pub struct ActorIdentity {
    pub actor_kind: sr_domain::ActorKind,
    pub actor_id: String,
    pub claims: serde_json::Value,
}

/// Identity provider errors
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Invalid token: {reason}")]
    InvalidToken { reason: String },

    #[error("Token expired")]
    TokenExpired,

    #[error("Provider error: {message}")]
    ProviderError { message: String },
}

/// Clock port for deterministic time handling
pub trait Clock: Send + Sync {
    /// Get the current UTC time
    fn now(&self) -> DateTime<Utc>;
}

/// System clock implementation
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
