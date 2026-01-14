//! State machines per SR-SPEC §3

use crate::entities::{
    ExceptionKind, ExceptionStatus, IterationState, LoopState, RunState, VerificationStatus,
};
use crate::errors::DomainError;

// ============================================================================
// Loop State Machine
// ============================================================================

/// Loop state machine per SR-SPEC §3.1.2
pub struct LoopStateMachine;

impl LoopStateMachine {
    /// Validate and compute the next state for a loop transition
    pub fn transition(current: LoopState, event: LoopTransition) -> Result<LoopState, DomainError> {
        match (current, event) {
            // CREATED -> ACTIVE via LoopActivated
            (LoopState::Created, LoopTransition::Activate) => Ok(LoopState::Active),

            // ACTIVE -> PAUSED via StopTriggered
            (LoopState::Active, LoopTransition::Stop) => Ok(LoopState::Paused),

            // PAUSED -> ACTIVE via DecisionRecorded + LoopResumed
            (LoopState::Paused, LoopTransition::Resume) => Ok(LoopState::Active),

            // Any state -> CLOSED via LoopClosed
            (_, LoopTransition::Close) => Ok(LoopState::Closed),

            // Invalid transitions
            (state, transition) => Err(DomainError::InvalidTransition {
                current_state: format!("{state:?}"),
                attempted_transition: format!("{transition:?}"),
            }),
        }
    }

    /// Check if a transition is allowed from the current state
    pub fn can_transition(current: LoopState, event: LoopTransition) -> bool {
        Self::transition(current, event).is_ok()
    }
}

/// Loop transition events
#[derive(Debug, Clone, Copy)]
pub enum LoopTransition {
    Activate,
    Stop,
    Resume,
    Close,
}

// ============================================================================
// Iteration State Machine
// ============================================================================

/// Iteration state machine per SR-SPEC §3.2
pub struct IterationStateMachine;

impl IterationStateMachine {
    /// Validate and compute the next state for an iteration transition
    pub fn transition(
        current: IterationState,
        event: IterationTransition,
    ) -> Result<IterationState, DomainError> {
        match (current, event) {
            // STARTED -> RUNNING via work beginning
            (IterationState::Started, IterationTransition::BeginWork) => {
                Ok(IterationState::Running)
            }

            // RUNNING -> COMPLETED via successful completion
            (IterationState::Running, IterationTransition::Complete) => {
                Ok(IterationState::Completed)
            }

            // RUNNING -> FAILED via failure
            (IterationState::Running, IterationTransition::Fail) => Ok(IterationState::Failed),

            // STARTED -> FAILED via early failure (e.g., context compilation failure)
            (IterationState::Started, IterationTransition::Fail) => Ok(IterationState::Failed),

            // Invalid transitions
            (state, transition) => Err(DomainError::InvalidTransition {
                current_state: format!("{state:?}"),
                attempted_transition: format!("{transition:?}"),
            }),
        }
    }

    /// Check if a transition is allowed from the current state
    pub fn can_transition(current: IterationState, event: IterationTransition) -> bool {
        Self::transition(current, event).is_ok()
    }
}

/// Iteration transition events
#[derive(Debug, Clone, Copy)]
pub enum IterationTransition {
    BeginWork,
    Complete,
    Fail,
}

// ============================================================================
// Run State Machine
// ============================================================================

/// Run state machine per SR-SPEC §3
pub struct RunStateMachine;

impl RunStateMachine {
    /// Validate and compute the next state for a run transition
    pub fn transition(current: RunState, event: RunTransition) -> Result<RunState, DomainError> {
        match (current, event) {
            // STARTED -> RUNNING via execution beginning
            (RunState::Started, RunTransition::BeginExecution) => Ok(RunState::Running),

            // RUNNING -> COMPLETED via successful completion
            (RunState::Running, RunTransition::Complete) => Ok(RunState::Completed),

            // RUNNING -> FAILED via failure
            (RunState::Running, RunTransition::Fail) => Ok(RunState::Failed),

            // STARTED -> FAILED via early failure (e.g., oracle not found)
            (RunState::Started, RunTransition::Fail) => Ok(RunState::Failed),

            // Invalid transitions
            (state, transition) => Err(DomainError::InvalidTransition {
                current_state: format!("{state:?}"),
                attempted_transition: format!("{transition:?}"),
            }),
        }
    }

    /// Check if a transition is allowed from the current state
    pub fn can_transition(current: RunState, event: RunTransition) -> bool {
        Self::transition(current, event).is_ok()
    }
}

/// Run transition events
#[derive(Debug, Clone, Copy)]
pub enum RunTransition {
    BeginExecution,
    Complete,
    Fail,
}

// ============================================================================
// Exception State Machine
// ============================================================================

/// Exception state machine per SR-CONTRACT
pub struct ExceptionStateMachine;

impl ExceptionStateMachine {
    /// Validate and compute the next state for an exception transition
    pub fn transition(
        current: ExceptionStatus,
        event: ExceptionTransition,
    ) -> Result<ExceptionStatus, DomainError> {
        match (current, event) {
            // CREATED -> ACTIVE via activation
            (ExceptionStatus::Created, ExceptionTransition::Activate) => {
                Ok(ExceptionStatus::Active)
            }

            // ACTIVE -> RESOLVED via resolution
            (ExceptionStatus::Active, ExceptionTransition::Resolve) => {
                Ok(ExceptionStatus::Resolved)
            }

            // ACTIVE -> EXPIRED via expiration
            (ExceptionStatus::Active, ExceptionTransition::Expire) => Ok(ExceptionStatus::Expired),

            // CREATED -> RESOLVED via resolution before activation
            (ExceptionStatus::Created, ExceptionTransition::Resolve) => {
                Ok(ExceptionStatus::Resolved)
            }

            // Invalid transitions
            (state, transition) => Err(DomainError::InvalidTransition {
                current_state: format!("{state:?}"),
                attempted_transition: format!("{transition:?}"),
            }),
        }
    }

    /// Check if a transition is allowed from the current state
    pub fn can_transition(current: ExceptionStatus, event: ExceptionTransition) -> bool {
        Self::transition(current, event).is_ok()
    }
}

/// Exception transition events
#[derive(Debug, Clone, Copy)]
pub enum ExceptionTransition {
    Activate,
    Resolve,
    Expire,
}

// ============================================================================
// Verification Status Computation
// ============================================================================

/// Candidate verification status computation per SR-SPEC §3.3
pub struct VerificationComputer;

impl VerificationComputer {
    /// Compute verification status based on oracle results and waivers
    ///
    /// Per SR-SPEC §3.3:
    /// - Verified (Strict): all required oracles PASS, no unresolved integrity conditions
    /// - Verified-with-Exceptions: some required oracles FAIL but covered by active waivers
    /// - Unverified: otherwise
    ///
    /// Per SR-SPEC §1.14: Waivers MUST NOT bypass integrity conditions
    pub fn compute(
        required_oracle_results: &[(String, bool)], // (oracle_id, passed)
        active_waivers: &[String],                  // oracle_ids covered by waivers
        has_integrity_conditions: bool,
    ) -> Result<VerificationStatus, DomainError> {
        // Integrity conditions block all verification per SR-SPEC §1.14
        if has_integrity_conditions {
            return Ok(VerificationStatus::Unverified);
        }

        let failed_oracles: Vec<&String> = required_oracle_results
            .iter()
            .filter(|(_, passed)| !passed)
            .map(|(id, _)| id)
            .collect();

        if failed_oracles.is_empty() {
            // All required oracles passed
            return Ok(VerificationStatus::VerifiedStrict);
        }

        // Check if all failures are covered by waivers
        let uncovered_failures: Vec<&&String> = failed_oracles
            .iter()
            .filter(|id| !active_waivers.contains(id))
            .collect();

        if uncovered_failures.is_empty() {
            Ok(VerificationStatus::VerifiedWithExceptions)
        } else {
            Ok(VerificationStatus::Unverified)
        }
    }
}

// ============================================================================
// Invariant Validators
// ============================================================================

/// Domain invariant validators per SR-CONTRACT
pub struct InvariantValidator;

impl InvariantValidator {
    /// Validate that an actor is HUMAN for portal/approval actions
    /// Per SR-CONTRACT C-TB-3: Portal crossings require human authority
    pub fn require_human_actor(
        actor_kind: crate::entities::ActorKind,
        action: &str,
    ) -> Result<(), DomainError> {
        if actor_kind != crate::entities::ActorKind::Human {
            return Err(DomainError::InvalidActor {
                reason: format!(
                    "{action} requires HUMAN actor, got {:?}",
                    actor_kind
                ),
            });
        }
        Ok(())
    }

    /// Validate that a waiver does not target an integrity condition
    /// Per SR-SPEC §1.14: Waivers cannot bypass integrity conditions
    pub fn validate_waiver_target(target: &str) -> Result<(), DomainError> {
        const INTEGRITY_CONDITIONS: &[&str] = &[
            "ORACLE_TAMPER",
            "ORACLE_GAP",
            "ORACLE_ENV_MISMATCH",
            "ORACLE_FLAKE",
            "EVIDENCE_MISSING",
        ];

        if INTEGRITY_CONDITIONS.contains(&target) {
            return Err(DomainError::InvariantViolation {
                invariant: format!(
                    "Waivers cannot target integrity conditions. '{}' is non-waivable per SR-SPEC §1.14",
                    target
                ),
            });
        }
        Ok(())
    }

    /// Validate exception can be created (human actor required)
    pub fn validate_exception_creation(
        actor_kind: crate::entities::ActorKind,
        exception_kind: ExceptionKind,
        target: Option<&str>,
    ) -> Result<(), DomainError> {
        Self::require_human_actor(actor_kind, "Exception creation")?;

        // Additional validation for waivers
        if exception_kind == ExceptionKind::Waiver {
            if let Some(t) = target {
                Self::validate_waiver_target(t)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_state_machine_valid_transitions() {
        // CREATED -> ACTIVE
        assert_eq!(
            LoopStateMachine::transition(LoopState::Created, LoopTransition::Activate).unwrap(),
            LoopState::Active
        );

        // ACTIVE -> PAUSED
        assert_eq!(
            LoopStateMachine::transition(LoopState::Active, LoopTransition::Stop).unwrap(),
            LoopState::Paused
        );

        // PAUSED -> ACTIVE
        assert_eq!(
            LoopStateMachine::transition(LoopState::Paused, LoopTransition::Resume).unwrap(),
            LoopState::Active
        );

        // Any -> CLOSED
        assert_eq!(
            LoopStateMachine::transition(LoopState::Created, LoopTransition::Close).unwrap(),
            LoopState::Closed
        );
        assert_eq!(
            LoopStateMachine::transition(LoopState::Active, LoopTransition::Close).unwrap(),
            LoopState::Closed
        );
    }

    #[test]
    fn test_loop_state_machine_invalid_transitions() {
        // Cannot activate already active loop
        assert!(LoopStateMachine::transition(LoopState::Active, LoopTransition::Activate).is_err());

        // Cannot resume active loop
        assert!(LoopStateMachine::transition(LoopState::Active, LoopTransition::Resume).is_err());

        // Cannot stop paused loop
        assert!(LoopStateMachine::transition(LoopState::Paused, LoopTransition::Stop).is_err());
    }

    #[test]
    fn test_iteration_state_machine() {
        // STARTED -> RUNNING
        assert_eq!(
            IterationStateMachine::transition(IterationState::Started, IterationTransition::BeginWork)
                .unwrap(),
            IterationState::Running
        );

        // RUNNING -> COMPLETED
        assert_eq!(
            IterationStateMachine::transition(IterationState::Running, IterationTransition::Complete)
                .unwrap(),
            IterationState::Completed
        );

        // RUNNING -> FAILED
        assert_eq!(
            IterationStateMachine::transition(IterationState::Running, IterationTransition::Fail)
                .unwrap(),
            IterationState::Failed
        );
    }

    #[test]
    fn test_verification_computation() {
        // All pass -> VerifiedStrict
        let results = vec![("test1".to_string(), true), ("test2".to_string(), true)];
        assert_eq!(
            VerificationComputer::compute(&results, &[], false).unwrap(),
            VerificationStatus::VerifiedStrict
        );

        // Some fail, no waivers -> Unverified
        let results = vec![("test1".to_string(), true), ("test2".to_string(), false)];
        assert_eq!(
            VerificationComputer::compute(&results, &[], false).unwrap(),
            VerificationStatus::Unverified
        );

        // Some fail, covered by waivers -> VerifiedWithExceptions
        let results = vec![("test1".to_string(), true), ("test2".to_string(), false)];
        let waivers = vec!["test2".to_string()];
        assert_eq!(
            VerificationComputer::compute(&results, &waivers, false).unwrap(),
            VerificationStatus::VerifiedWithExceptions
        );

        // Integrity conditions -> always Unverified
        let results = vec![("test1".to_string(), true)];
        assert_eq!(
            VerificationComputer::compute(&results, &[], true).unwrap(),
            VerificationStatus::Unverified
        );
    }

    #[test]
    fn test_waiver_target_validation() {
        // Valid waiver targets
        assert!(InvariantValidator::validate_waiver_target("test_failure").is_ok());
        assert!(InvariantValidator::validate_waiver_target("lint_check").is_ok());

        // Invalid waiver targets (integrity conditions)
        assert!(InvariantValidator::validate_waiver_target("ORACLE_TAMPER").is_err());
        assert!(InvariantValidator::validate_waiver_target("ORACLE_GAP").is_err());
        assert!(InvariantValidator::validate_waiver_target("EVIDENCE_MISSING").is_err());
    }

    #[test]
    fn test_human_actor_requirement() {
        use crate::entities::ActorKind;

        // Human actor allowed
        assert!(InvariantValidator::require_human_actor(ActorKind::Human, "Approval").is_ok());

        // Non-human actors rejected
        assert!(InvariantValidator::require_human_actor(ActorKind::Agent, "Approval").is_err());
        assert!(InvariantValidator::require_human_actor(ActorKind::System, "Approval").is_err());
    }
}
