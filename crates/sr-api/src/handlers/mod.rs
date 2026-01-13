//! API Handlers for D-18: Core API Endpoints
//!
//! Per SR-SPEC §2: HTTP endpoints for loops, iterations, candidates, runs.
//! Enforces domain transition rules and SYSTEM-only actions where required.

pub mod candidates;
pub mod error;
pub mod iterations;
pub mod loops;
pub mod runs;

pub use error::{ApiError, ApiResult};
