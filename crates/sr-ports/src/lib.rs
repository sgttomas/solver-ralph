//! SOLVER-Ralph Port Traits
//! This crate defines the port interfaces (traits) that adapters must implement.
//! Per SR-SPEC §4.1 and D-07, these ports include:
//! - EventStore
//! - EvidenceStore
//! - OracleRunner
//! - MessageBus
//! - IdentityProvider
//! - Clock

use std::future::Future;
use std::pin::Pin;

use chrono::{DateTime, Utc};
use sr_domain::EventEnvelope;

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

    #[error("Suite hash mismatch: expected {expected}, got {actual}")]
    SuiteHashMismatch { expected: String, actual: String },

    #[error("Container creation failed: {reason}")]
    ContainerCreationFailed { reason: String },

    #[error("Container execution failed: {reason}")]
    ContainerExecutionFailed { reason: String },

    #[error("Execution timeout for oracle {oracle_id}: exceeded {timeout_secs} seconds")]
    ExecutionTimeout {
        oracle_id: String,
        timeout_secs: u64,
    },

    #[error("Output capture failed for {artifact}: {reason}")]
    OutputCaptureFailed { artifact: String, reason: String },

    #[error("Manifest creation failed: {reason}")]
    ManifestCreationFailed { reason: String },

    #[error("Evidence store failed: {reason}")]
    EvidenceStoreFailed { reason: String },

    #[error("Environment mismatch: {details}")]
    EnvironmentMismatch { details: String },

    #[error("Restricted evidence access denied: {reason}")]
    RestrictedEvidenceAccessDenied { reason: String },

    #[error("Execution error: {message}")]
    ExecutionError { message: String },
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
    fn next(&mut self) -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send + '_>>;
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
    fn validate(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<ActorIdentity, IdentityError>> + Send;
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

// ============================================================================
// Secret Provider Port (D-16: Restricted Evidence Handling)
// ============================================================================

/// Secret provider port for envelope key management per SR-SPEC
///
/// This port abstracts secret/key management for restricted evidence handling.
/// The primary implementation is Infisical, but test implementations may use
/// in-memory stores.
///
/// Envelope encryption pattern:
/// 1. Generate a data encryption key (DEK) for the evidence bundle
/// 2. Encrypt the DEK with a key encryption key (KEK) from the secret provider
/// 3. Store the encrypted DEK alongside the evidence
/// 4. Retrieve and decrypt the DEK when accessing restricted evidence
pub trait SecretProvider: Send + Sync {
    /// Retrieve a secret by path
    ///
    /// Path format: "project/environment/path/to/secret"
    fn get_secret(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<SecretValue, SecretProviderError>> + Send;

    /// Store a secret (for DEK envelope storage)
    ///
    /// Returns the secret ID for later retrieval
    fn store_secret(
        &self,
        path: &str,
        value: &[u8],
        metadata: SecretMetadata,
    ) -> impl Future<Output = Result<String, SecretProviderError>> + Send;

    /// Delete a secret
    fn delete_secret(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<(), SecretProviderError>> + Send;

    /// Check if a secret exists
    fn secret_exists(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<bool, SecretProviderError>> + Send;

    /// Get the key encryption key (KEK) for envelope encryption
    ///
    /// This returns the KEK that will be used to encrypt data encryption keys.
    /// The KEK itself should never leave the secret provider in plaintext
    /// for production use.
    fn get_envelope_key(
        &self,
        key_id: &str,
    ) -> impl Future<Output = Result<EnvelopeKey, SecretProviderError>> + Send;
}

/// A secret value retrieved from the provider
#[derive(Debug, Clone)]
pub struct SecretValue {
    /// The secret data
    pub value: Vec<u8>,
    /// Version of the secret
    pub version: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Associated metadata
    pub metadata: SecretMetadata,
}

/// Metadata for a stored secret
#[derive(Debug, Clone, Default)]
pub struct SecretMetadata {
    /// Human-readable description
    pub description: Option<String>,
    /// Associated evidence bundle hash (for DEKs)
    pub evidence_hash: Option<String>,
    /// Key rotation policy
    pub rotation_policy: Option<String>,
    /// Additional tags
    pub tags: std::collections::HashMap<String, String>,
}

/// An envelope key for encrypt/decrypt operations
#[derive(Debug, Clone)]
pub struct EnvelopeKey {
    /// Key identifier
    pub key_id: String,
    /// Key material (32 bytes for AES-256)
    pub key_material: Vec<u8>,
    /// Algorithm (e.g., "AES-256-GCM")
    pub algorithm: String,
    /// Key version
    pub version: u64,
}

/// Secret provider errors
#[derive(Debug, thiserror::Error)]
pub enum SecretProviderError {
    #[error("Secret not found: {path}")]
    NotFound { path: String },

    #[error("Access denied: {reason}")]
    AccessDenied { reason: String },

    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    #[error("Invalid secret path: {path}")]
    InvalidPath { path: String },

    #[error("Encryption error: {message}")]
    EncryptionError { message: String },

    #[error("Provider error: {message}")]
    ProviderError { message: String },
}
