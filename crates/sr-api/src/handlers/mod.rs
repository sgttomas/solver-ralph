//! API Handlers for D-18, D-19, D-20: Core, Governance, and Evidence API Endpoints
//!
//! Per SR-SPEC §2: HTTP endpoints for loops, iterations, candidates, runs,
//! approvals, exceptions, decisions, freeze records, and evidence.
//! Enforces domain transition rules and HUMAN-only actions where required.

pub mod approvals;
pub mod candidates;
pub mod decisions;
pub mod error;
pub mod evidence;
pub mod exceptions;
pub mod freeze;
pub mod iterations;
pub mod loops;
pub mod prompt_loop;
pub mod runs;

pub use error::{ApiError, ApiResult};
