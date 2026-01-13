//! State machines per SR-SPEC §3

use crate::entities::LoopState;
use crate::errors::DomainError;

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
}

/// Loop transition events
#[derive(Debug, Clone, Copy)]
pub enum LoopTransition {
    Activate,
    Stop,
    Resume,
    Close,
}
