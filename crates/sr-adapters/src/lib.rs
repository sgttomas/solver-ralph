//! SOLVER-Ralph Adapter Implementations
//!
//! This crate provides adapter implementations for the ports defined in sr-ports.
//! Per SR-SPEC, these include:
//! - PostgreSQL event store adapter (D-10)
//! - Projection builder (D-11)
//! - MinIO evidence store adapter (D-14)
//! - NATS message bus adapter (D-21)
//! - Zitadel identity provider adapter (D-17)

pub mod config;
pub mod postgres;
pub mod projections;

pub use config::*;
pub use postgres::PostgresEventStore;
pub use projections::{
    CandidateProjection, IterationProjection, LoopProjection, ProjectionBuilder, ProjectionConfig,
    ProjectionError, RunProjection,
};
