//! API Handlers for D-18, D-19, D-20: Core, Governance, and Evidence API Endpoints
//!
//! Per SR-SPEC §2: HTTP endpoints for loops, iterations, candidates, runs,
//! approvals, exceptions, decisions, freeze records, evidence, and oracles.
//! Enforces domain transition rules and HUMAN-only actions where required.
//!
//! Per SR-PLAN-V3: Intake management endpoints for draft, activate, archive, fork.

pub mod approvals;
pub mod candidates;
pub mod decisions;
pub mod error;
pub mod evidence;
pub mod exceptions;
pub mod freeze;
pub mod intakes;
pub mod iterations;
pub mod loops;
pub mod oracles;
pub mod prompt_loop;
pub mod references;
pub mod runs;
pub mod templates;
pub mod work_surfaces;

pub use error::{ApiError, ApiResult};
