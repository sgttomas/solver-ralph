//! SOLVER-Ralph Domain Core
//!
//! This crate contains the pure domain logic for SOLVER-Ralph, including:
//! - Domain entities (Loop, Iteration, Candidate, Run, Evidence, etc.)
//! - State machines and transitions
//! - Invariants and validation rules
//! - Event definitions
//!
//! Per SR-SPEC §4.1 (Hexagonal Architecture), this crate MUST NOT import
//! DB clients, HTTP frameworks, container runtimes, or LLM SDKs.

pub mod commands;
pub mod entities;
pub mod errors;
pub mod events;
pub mod state_machines;

pub use commands::*;
pub use entities::*;
pub use errors::*;
pub use events::*;
pub use state_machines::*;
