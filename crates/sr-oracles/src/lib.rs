//! Oracle command implementations for SOLVER-Ralph (D-26)
//!
//! This crate provides the oracle command implementations referenced by SR-SUITE-FULL:
//! - integration: Tests DB/MinIO/NATS/API connectivity
//! - e2e: Full end-to-end stack verification
//!
//! Per SR-PLAN D-26:
//! - Suite can stand up the full stack and run e2e flows deterministically within tolerance
//! - Flake controls exist (timeouts, retries policy) and are recorded
//!
//! Expected evidence:
//! - Evidence Bundle with an e2e run transcript and artifacts

pub mod e2e;
pub mod flake_control;
pub mod integration;
pub mod report;

pub use e2e::E2ERunner;
pub use flake_control::{FlakeControl, RetryPolicy};
pub use integration::IntegrationRunner;
pub use report::{E2EReport, IntegrationReport, ServiceTestResult};
