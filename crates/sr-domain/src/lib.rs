//! SOLVER-Ralph Domain Core
//!
//! This crate contains the pure domain logic for SOLVER-Ralph, including:
//! - Domain entities (Loop, Iteration, Candidate, Run, Evidence, etc.)
//! - State machines and transitions
//! - Invariants and validation rules
//! - Event definitions
//! - Context compilation for deterministic context bundles
//! - Work surface schemas (Intake, ProcedureTemplate, WorkSurfaceInstance)
//! - Plan instance decomposition (PlanInstance, WorkUnitPlan, DecompositionRationale)
//! - Semantic oracle schemas (SemanticSet, SemanticEvalResult) per SR-SEMANTIC-ORACLE-SPEC
//! - Strongly-typed references (StrongTypedRef) per SR-SPEC §1.5.3
//! - Intake lifecycle (IntakeStatus, events) per SR-WORK-SURFACE §3
//!
//! Per SR-SPEC §4.1 (Hexagonal Architecture), this crate MUST NOT import
//! DB clients, HTTP frameworks, container runtimes, or LLM SDKs.

pub mod commands;
pub mod context;
pub mod entities;
pub mod errors;
pub mod events;
pub mod intake;
pub mod plan_instance;
pub mod procedure_templates;
pub mod refs;
pub mod semantic_oracle;
pub mod state_machines;
pub mod work_surface;

pub use commands::*;
pub use context::*;
pub use entities::*;
pub use errors::*;
pub use events::*;
pub use intake::*;
pub use plan_instance::*;
pub use procedure_templates::*;
pub use refs::*;
pub use semantic_oracle::*;
pub use state_machines::*;
pub use work_surface::*;
