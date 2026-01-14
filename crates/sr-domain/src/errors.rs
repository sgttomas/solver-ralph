//! Domain errors

use thiserror::Error;

/// Domain errors for SOLVER-Ralph
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid state transition: cannot transition from {current_state} via {attempted_transition}")]
    InvalidTransition {
        current_state: String,
        attempted_transition: String,
    },

    #[error("Invalid actor: {reason}")]
    InvalidActor { reason: String },

    #[error("Invariant violation: {invariant}")]
    InvariantViolation { invariant: String },

    #[error("Missing required reference: {ref_kind}")]
    MissingReference { ref_kind: String },

    #[error("Budget exceeded: {budget_type}")]
    BudgetExceeded { budget_type: String },

    #[error("Integrity condition: {condition}")]
    IntegrityCondition { condition: String },
}
