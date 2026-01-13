//! SOLVER-Ralph Adapter Implementations
//!
//! This crate provides adapter implementations for the ports defined in sr-ports.
//! Per SR-SPEC, these include:
//! - PostgreSQL event store adapter (D-10)
//! - Projection builder (D-11)
//! - Dependency graph projection (D-12)
//! - Outbox publisher (D-13)
//! - MinIO evidence store adapter (D-14)
//! - Evidence manifest library (D-15)
//! - Restricted evidence handling (D-16)
//! - NATS message bus adapter (D-21)
//! - Loop governor service (D-22)
//! - Zitadel identity provider adapter (D-17)

pub mod config;
pub mod evidence;
pub mod governor;
pub mod graph;
pub mod infisical;
pub mod minio;
pub mod nats;
pub mod outbox;
pub mod postgres;
pub mod projections;
pub mod restricted;

pub use config::*;
pub use evidence::{
    EvidenceArtifact, EvidenceManifest, EvidenceManifestBuilder, ManifestValidationError,
    OracleResult, OracleResultStatus,
};
pub use graph::{
    DependencyEdge, EdgeType, GraphError, GraphNode, GraphProjection, StalenessMarker,
    StalenessReason,
};
pub use minio::{MinioConfig, MinioEvidenceStore};
pub use outbox::{
    OutboxEntry, OutboxError, OutboxPublisher, OutboxPublisherConfig, OutboxWriter,
};
pub use postgres::PostgresEventStore;
pub use projections::{
    ApprovalProjection, CandidateProjection, DecisionProjection, EvidenceProjection,
    ExceptionProjection, FreezeRecordProjection, IterationProjection, LoopProjection,
    ProjectionBuilder, ProjectionConfig, ProjectionError, RunProjection,
};
pub use restricted::{
    standard_redaction_rules, EncryptionMetadata, EvidenceClassification, EvidenceRedactor,
    InMemorySecretProvider, RedactedContentType, RedactionEntry, RedactionManifest, RedactionRule,
    RestrictedEvidenceConfig, RestrictedEvidenceStore,
};
pub use infisical::{InfisicalConfig, InfisicalSecretProvider};
pub use nats::{
    create_envelope, serialize_envelope, streams, subjects, MessageEnvelope, NatsConfig,
    NatsConsumer, NatsMessage, NatsMessageBus, SCHEMA_VERSION,
};
pub use governor::{
    GovernorDecision, GovernorDecisionType, GovernorError, GovernorOutcome,
    IterationPreconditions, LoopBudget, LoopGovernor, LoopTrackingState, PreconditionSnapshot,
    StopCondition,
};
