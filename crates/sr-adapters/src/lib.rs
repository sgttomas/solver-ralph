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
//! - NATS message bus adapter (D-21)
//! - Zitadel identity provider adapter (D-17)

pub mod config;
pub mod evidence;
pub mod graph;
pub mod minio;
pub mod outbox;
pub mod postgres;
pub mod projections;

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
    ApprovalProjection, CandidateProjection, DecisionProjection, ExceptionProjection,
    FreezeRecordProjection, IterationProjection, LoopProjection, ProjectionBuilder,
    ProjectionConfig, ProjectionError, RunProjection,
};
