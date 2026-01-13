//! API Handlers for D-18 and D-19: Core and Governance API Endpoints
//!
//! Per SR-SPEC §2: HTTP endpoints for loops, iterations, candidates, runs,
//! approvals, exceptions, decisions, and freeze records.
//! Enforces domain transition rules and HUMAN-only actions where required.

pub mod approvals;
pub mod candidates;
pub mod decisions;
pub mod error;
pub mod exceptions;
pub mod freeze;
pub mod iterations;
pub mod loops;
pub mod runs;

pub use error::{ApiError, ApiResult};
