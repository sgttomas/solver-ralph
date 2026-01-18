//! Loop Governor Service (D-22)
//!
//! Implements the loop governor that decides when to start iterations,
//! emits IterationStarted (SYSTEM-only), and enforces budgets/stop conditions
//! per SR-SPEC and SR-DIRECTIVE.
//!
//! Key responsibilities:
//! - Monitor loop state and decide when to start new iterations
//! - Emit IterationStarted events with SYSTEM actor
//! - Enforce budget limits (iteration count, time, cost)
//! - Handle stop conditions and triggers
//! - Record all decisions as events (no silent actions)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_domain::{
    ActorKind, EventEnvelope, EventId, IterationState, LoopState, StreamKind, TypedRef,
};
use sr_ports::{EventStore, EventStoreError};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

// ============================================================================
// Governor Configuration
// ============================================================================

/// Loop budget configuration per SR-SPEC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopBudget {
    /// Maximum number of iterations allowed (0 = unlimited)
    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,
    /// Maximum oracle runs allowed (0 = unlimited)
    #[serde(default = "default_max_oracle_runs")]
    pub max_oracle_runs: u32,
    /// Maximum wall-clock time for the loop in hours (0 = unlimited)
    #[serde(default = "default_max_wallclock_hours")]
    pub max_wallclock_hours: u32,
    /// Minimum delay between iterations in seconds
    #[serde(default = "default_min_iteration_delay_secs")]
    pub min_iteration_delay_secs: i64,
}

impl Default for LoopBudget {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            max_oracle_runs: default_max_oracle_runs(),
            max_wallclock_hours: default_max_wallclock_hours(),
            min_iteration_delay_secs: default_min_iteration_delay_secs(),
        }
    }
}

fn default_max_iterations() -> u32 {
    5
}

fn default_max_oracle_runs() -> u32 {
    25
}

fn default_max_wallclock_hours() -> u32 {
    16
}

fn default_min_iteration_delay_secs() -> i64 {
    1
}

impl LoopBudget {
    /// Convert wall-clock budget to seconds (0 treated as unlimited)
    fn max_duration_secs(&self) -> i64 {
        if self.max_wallclock_hours == 0 {
            i64::MAX
        } else {
            (self.max_wallclock_hours as i64) * 3600
        }
    }
}

/// Stop condition types per SR-SPEC
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopCondition {
    /// Budget exhausted (iterations, time, or cost)
    BudgetExhausted,
    /// Explicit stop requested by human
    HumanStop,
    /// Goal achieved (worker signals completion)
    GoalAchieved,
    /// Integrity condition triggered
    IntegrityCondition,
    /// Loop closed
    LoopClosed,
    /// No eligible work available (SR-DIRECTIVE §4.2)
    NoEligibleWork,
    /// Work Surface missing or archived per SR-PLAN-V4 Phase 4c
    WorkSurfaceMissing,
    /// Any other stop trigger string (kept for audit)
    Trigger(String),
}

/// Iteration start preconditions per SR-SPEC
#[derive(Debug, Clone, Default)]
pub struct IterationPreconditions {
    /// Loop must be in ACTIVE state
    pub loop_active: bool,
    /// No incomplete iteration exists
    pub no_incomplete_iteration: bool,
    /// Budget not exhausted
    pub budget_available: bool,
    /// Minimum delay elapsed since last iteration
    pub delay_elapsed: bool,
    /// No active stop triggers
    pub no_stop_triggers: bool,
    /// All required approvals in place (if any pending)
    pub approvals_satisfied: bool,
    /// Work Surface available for this work unit per SR-PLAN-V4 Phase 4c
    /// This is only checked when work_unit_id is specified for the loop
    pub work_surface_available: bool,
}

impl IterationPreconditions {
    /// Check if all preconditions are satisfied
    pub fn all_satisfied(&self) -> bool {
        self.loop_active
            && self.no_incomplete_iteration
            && self.budget_available
            && self.delay_elapsed
            && self.no_stop_triggers
            && self.approvals_satisfied
            && self.work_surface_available
    }

    /// Get the first unsatisfied precondition (for error reporting)
    pub fn first_unsatisfied(&self) -> Option<&'static str> {
        if !self.loop_active {
            Some("loop_not_active")
        } else if !self.no_incomplete_iteration {
            Some("incomplete_iteration_exists")
        } else if !self.budget_available {
            Some("budget_exhausted")
        } else if !self.delay_elapsed {
            Some("min_delay_not_elapsed")
        } else if !self.no_stop_triggers {
            Some("stop_trigger_active")
        } else if !self.approvals_satisfied {
            Some("pending_approvals")
        } else if !self.work_surface_available {
            Some("work_surface_missing")
        } else {
            None
        }
    }
}

impl StopCondition {
    fn from_trigger(trigger: &str) -> Self {
        match trigger {
            "BUDGET_EXHAUSTED" => StopCondition::BudgetExhausted,
            "HUMAN_STOP" => StopCondition::HumanStop,
            "GOAL_ACHIEVED" => StopCondition::GoalAchieved,
            "LOOP_CLOSED" => StopCondition::LoopClosed,
            "NO_ELIGIBLE_WORK" => StopCondition::NoEligibleWork,
            "WORK_SURFACE_MISSING" => StopCondition::WorkSurfaceMissing,
            _ => StopCondition::Trigger(trigger.to_string()),
        }
    }
}

// ============================================================================
// Governor State
// ============================================================================

/// Loop tracking state maintained by the governor
#[derive(Debug, Clone)]
pub struct LoopTrackingState {
    /// Loop ID
    pub loop_id: String,
    /// Current loop state
    pub loop_state: LoopState,
    /// Loop budget
    pub budget: LoopBudget,
    /// Number of iterations started
    pub iteration_count: u32,
    /// Number of oracle runs started
    pub oracle_runs_started: u32,
    /// Current iteration ID (if any)
    pub current_iteration_id: Option<String>,
    /// Current iteration state (if any)
    pub current_iteration_state: Option<IterationState>,
    /// Loop creation time
    pub created_at: DateTime<Utc>,
    /// Last iteration started time
    pub last_iteration_at: Option<DateTime<Utc>>,
    /// Active stop triggers
    pub stop_triggers: Vec<StopCondition>,
    /// Pending portal approvals required before iteration can proceed
    /// Maps portal_id to the reason (stop trigger type) requiring approval
    pub pending_portal_approvals: HashSet<String>,
    /// Cost units consumed
    pub cost_consumed: u64,
    /// Work unit ID associated with this loop (per SR-PLAN-V4 Phase 4c)
    /// When set, the governor will validate Work Surface availability
    pub work_unit_id: Option<String>,
    /// Cached Work Surface ID when work_unit_id is set and Work Surface is available
    pub work_surface_id: Option<String>,
}

impl LoopTrackingState {
    pub fn new(loop_id: String, budget: LoopBudget, created_at: DateTime<Utc>) -> Self {
        Self {
            loop_id,
            loop_state: LoopState::Created,
            budget,
            iteration_count: 0,
            oracle_runs_started: 0,
            current_iteration_id: None,
            current_iteration_state: None,
            created_at,
            last_iteration_at: None,
            stop_triggers: Vec::new(),
            pending_portal_approvals: HashSet::new(),
            cost_consumed: 0,
            work_unit_id: None,
            work_surface_id: None,
        }
    }

    /// Create a new loop tracking state with a work unit ID
    /// This enables Work Surface validation per SR-PLAN-V4 Phase 4c
    pub fn new_with_work_unit(
        loop_id: String,
        budget: LoopBudget,
        created_at: DateTime<Utc>,
        work_unit_id: String,
    ) -> Self {
        Self {
            loop_id,
            loop_state: LoopState::Created,
            budget,
            iteration_count: 0,
            oracle_runs_started: 0,
            current_iteration_id: None,
            current_iteration_state: None,
            created_at,
            last_iteration_at: None,
            stop_triggers: Vec::new(),
            pending_portal_approvals: HashSet::new(),
            cost_consumed: 0,
            work_unit_id: Some(work_unit_id),
            work_surface_id: None,
        }
    }
}

// ============================================================================
// Governor Decision Record
// ============================================================================

/// Governor decision record (for audit)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernorDecision {
    /// Decision ID
    pub decision_id: String,
    /// Loop ID
    pub loop_id: String,
    /// Decision type
    pub decision_type: GovernorDecisionType,
    /// Preconditions at decision time
    pub preconditions: PreconditionSnapshot,
    /// Decision outcome
    pub outcome: GovernorOutcome,
    /// Timestamp
    pub decided_at: DateTime<Utc>,
    /// Rationale
    pub rationale: String,
}

/// Governor decision types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GovernorDecisionType {
    /// Decision to start an iteration
    StartIteration,
    /// Decision to not start an iteration
    DeferIteration,
    /// Decision to stop the loop
    StopLoop,
}

/// Precondition snapshot for decision records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreconditionSnapshot {
    pub loop_state: String,
    pub iteration_count: u32,
    pub budget_remaining_iterations: u32,
    pub budget_remaining_oracle_runs: u32,
    pub budget_remaining_secs: i64,
    pub has_incomplete_iteration: bool,
    pub stop_triggers: Vec<String>,
    pub pending_portal_approvals: Vec<String>,
}

/// Governor decision outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GovernorOutcome {
    /// Decision approved and executed
    Executed,
    /// Decision deferred (preconditions not met)
    Deferred,
    /// Decision blocked (stop condition)
    Blocked,
}

// ============================================================================
// Loop Governor
// ============================================================================

/// Loop governor error types
#[derive(Debug, thiserror::Error)]
pub enum GovernorError {
    #[error("Loop not found: {loop_id}")]
    LoopNotFound { loop_id: String },

    #[error("Precondition not met: {reason}")]
    PreconditionNotMet { reason: String },

    #[error("Budget exhausted: {budget_type}")]
    BudgetExhausted { budget_type: String },

    #[error("Stop condition active: {condition:?}")]
    StopConditionActive { condition: StopCondition },

    #[error("Event store error: {message}")]
    EventStoreError { message: String },

    #[error("Invalid state: {message}")]
    InvalidState { message: String },
}

impl From<EventStoreError> for GovernorError {
    fn from(e: EventStoreError) -> Self {
        GovernorError::EventStoreError {
            message: e.to_string(),
        }
    }
}

/// Loop governor service
///
/// The governor is responsible for deciding when to start iterations
/// and enforcing loop budgets/stop conditions.
pub struct LoopGovernor<E: EventStore> {
    /// Event store for emitting events
    event_store: Arc<E>,
    /// Tracked loop states
    loops: Arc<RwLock<HashMap<String, LoopTrackingState>>>,
    /// Decision records for audit
    decisions: Arc<RwLock<Vec<GovernorDecision>>>,
}

impl<E: EventStore> LoopGovernor<E> {
    /// Create a new loop governor
    pub fn new(event_store: Arc<E>) -> Self {
        Self {
            event_store,
            loops: Arc::new(RwLock::new(HashMap::new())),
            decisions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a loop with the governor
    #[instrument(skip(self))]
    pub async fn register_loop(
        &self,
        loop_id: &str,
        budget: LoopBudget,
        created_at: DateTime<Utc>,
    ) {
        let state = LoopTrackingState::new(loop_id.to_string(), budget, created_at);
        let mut loops = self.loops.write().await;
        loops.insert(loop_id.to_string(), state);
        info!(loop_id = %loop_id, "Loop registered with governor");
    }

    /// Update loop state from event
    #[instrument(skip(self))]
    pub async fn handle_event(&self, event: &EventEnvelope) -> Result<(), GovernorError> {
        let mut loops = self.loops.write().await;
        let mut budget_stop: Option<(String, String)> = None;

        match event.event_type.as_str() {
            "LoopCreated" => {
                // Extract budget from payload if present
                let budget = event
                    .payload
                    .get("budgets")
                    .or_else(|| event.payload.get("budget"))
                    .and_then(|b| serde_json::from_value(b.clone()).ok())
                    .unwrap_or_default();

                let state =
                    LoopTrackingState::new(event.stream_id.clone(), budget, event.occurred_at);
                loops.insert(event.stream_id.clone(), state);
                debug!(loop_id = %event.stream_id, "Loop created");
            }
            "LoopUpdated" => {
                if let Some(state) = loops.get_mut(&event.stream_id) {
                    if let Some(budget_val) = event.payload.get("budgets") {
                        if let Ok(new_budget) = serde_json::from_value::<LoopBudget>(
                            budget_val.clone(),
                        ) {
                            let previous_budget = state.budget.clone();
                            state.budget = new_budget;

                            // If budget was previously exhausted but the new budget allows progress,
                            // clear the budget stop trigger and portal requirement.
                            if (state.budget.max_oracle_runs == 0
                                || state.oracle_runs_started < state.budget.max_oracle_runs)
                                && state
                                    .stop_triggers
                                    .iter()
                                    .any(|c| matches!(c, StopCondition::BudgetExhausted))
                            {
                                state
                                    .stop_triggers
                                    .retain(|c| !matches!(c, StopCondition::BudgetExhausted));
                                state
                                    .pending_portal_approvals
                                    .retain(|p| p != "HumanAuthorityExceptionProcess");
                            }

                            debug!(
                                loop_id = %event.stream_id,
                                previous_max_runs = previous_budget.max_oracle_runs,
                                new_max_runs = state.budget.max_oracle_runs,
                                previous_max_iterations = previous_budget.max_iterations,
                                new_max_iterations = state.budget.max_iterations,
                                "Loop budget updated"
                            );
                        }
                    }
                }
            }
            "RunStarted" => {
                // Track oracle run budget usage when loop context is available
                let loop_id = event
                    .payload
                    .get("loop_id")
                    .and_then(|l| l.as_str())
                    .or_else(|| {
                        event.refs.iter().find_map(|r| {
                            if r.kind.eq_ignore_ascii_case("loop") {
                                Some(r.id.as_str())
                            } else {
                                None
                            }
                        })
                    });

                if let Some(loop_id) = loop_id {
                    if let Some(state) = loops.get_mut(loop_id) {
                        state.oracle_runs_started = state.oracle_runs_started.saturating_add(1);

                        // Emit stop when oracle run budget is exhausted
                        if state.budget.max_oracle_runs > 0
                            && state.oracle_runs_started >= state.budget.max_oracle_runs
                        {
                            if !state
                                .stop_triggers
                                .iter()
                                .any(|c| matches!(c, StopCondition::BudgetExhausted))
                            {
                                state.stop_triggers.push(StopCondition::BudgetExhausted);
                                state
                                    .pending_portal_approvals
                                    .insert("HumanAuthorityExceptionProcess".to_string());
                                budget_stop = Some((
                                    loop_id.to_string(),
                                    "HumanAuthorityExceptionProcess".to_string(),
                                ));
                            }
                        }

                        debug!(
                            loop_id = %loop_id,
                            oracle_runs_started = state.oracle_runs_started,
                            "RunStarted tracked for loop"
                        );
                    }
                }
            }
            "LoopActivated" => {
                if let Some(state) = loops.get_mut(&event.stream_id) {
                    state.loop_state = LoopState::Active;
                    debug!(loop_id = %event.stream_id, "Loop activated");
                }
            }
            "LoopPaused" => {
                if let Some(state) = loops.get_mut(&event.stream_id) {
                    state.loop_state = LoopState::Paused;
                    debug!(loop_id = %event.stream_id, "Loop paused");
                }
            }
            "LoopResumed" => {
                if let Some(state) = loops.get_mut(&event.stream_id) {
                    state.loop_state = LoopState::Active;
                    debug!(loop_id = %event.stream_id, "Loop resumed");
                }
            }
            "LoopClosed" => {
                if let Some(state) = loops.get_mut(&event.stream_id) {
                    state.loop_state = LoopState::Closed;
                    state.stop_triggers.push(StopCondition::LoopClosed);
                    debug!(loop_id = %event.stream_id, "Loop closed");
                }
            }
            "IterationStarted" => {
                // Find the loop by iteration stream
                let iteration_id = event.stream_id.clone();
                let loop_id = event
                    .refs
                    .iter()
                    .find(|r| r.kind == "loop")
                    .map(|r| r.id.clone());

                if let Some(lid) = loop_id {
                    if let Some(state) = loops.get_mut(&lid) {
                        state.iteration_count += 1;
                        state.current_iteration_id = Some(iteration_id);
                        state.current_iteration_state = Some(IterationState::Started);
                        state.last_iteration_at = Some(event.occurred_at);
                        debug!(
                            loop_id = %lid,
                            iteration = state.iteration_count,
                            "Iteration started"
                        );
                    }
                }
            }
            "IterationCompleted" => {
                // Find loop containing this iteration
                for state in loops.values_mut() {
                    if state.current_iteration_id.as_deref() == Some(&event.stream_id) {
                        state.current_iteration_state = Some(IterationState::Completed);
                        state.current_iteration_id = None;
                        debug!(loop_id = %state.loop_id, "Iteration completed");
                        break;
                    }
                }
            }
            "StopTriggered" => {
                if let Some(state) = loops.get_mut(&event.stream_id) {
                    let trigger = event
                        .payload
                        .get("trigger")
                        .and_then(|t| t.as_str())
                        .or_else(|| event.payload.get("condition").and_then(|c| c.as_str()))
                        .unwrap_or("HUMAN_STOP");

                    let condition = StopCondition::from_trigger(trigger);
                    state.stop_triggers.push(condition.clone());

                    // Track pending portal approval if recommended_portal is specified
                    if let Some(portal) = event
                        .payload
                        .get("recommended_portal")
                        .and_then(|p| p.as_str())
                    {
                        state.pending_portal_approvals.insert(portal.to_string());
                        debug!(
                            loop_id = %event.stream_id,
                            portal = %portal,
                            "Pending portal approval added"
                        );
                    }

                    debug!(
                        loop_id = %event.stream_id,
                        condition = ?condition,
                        "Stop triggered"
                    );
                }
            }
            "ApprovalRecorded" => {
                // Extract portal_id and loop reference from approval
                if let Some(portal_id) = event.payload.get("portal_id").and_then(|p| p.as_str()) {
                    // Find the loop this approval relates to
                    // Approvals can reference loops via subject_refs
                    let related_loop_id = event
                        .payload
                        .get("subject_refs")
                        .and_then(|refs| refs.as_array())
                        .and_then(|refs| {
                            refs.iter().find_map(|r| {
                                if r.get("kind")?.as_str()? == "Loop" {
                                    r.get("id")?.as_str().map(|s| s.to_string())
                                } else {
                                    None
                                }
                            })
                        })
                        .or_else(|| {
                            // Fall back to stream_id if it's a loop stream
                            if event.stream_id.starts_with("loop_") {
                                Some(event.stream_id.clone())
                            } else {
                                None
                            }
                        });

                    if let Some(loop_id) = related_loop_id {
                        if let Some(state) = loops.get_mut(&loop_id) {
                            if state.pending_portal_approvals.remove(portal_id) {
                                debug!(
                                    loop_id = %loop_id,
                                    portal_id = %portal_id,
                                    "Portal approval satisfied"
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        drop(loops);

        // Emit stop after releasing loop lock to avoid holding across await
        if let Some((loop_id, portal)) = budget_stop {
            self.emit_stop_triggered(&loop_id, "BUDGET_EXHAUSTED", Some(&portal))
                .await?;
        }

        Ok(())
    }

    /// Check iteration preconditions for a loop
    #[instrument(skip(self))]
    pub async fn check_preconditions(
        &self,
        loop_id: &str,
    ) -> Result<IterationPreconditions, GovernorError> {
        let loops = self.loops.read().await;
        let state = loops.get(loop_id).ok_or(GovernorError::LoopNotFound {
            loop_id: loop_id.to_string(),
        })?;

        let now = Utc::now();

        // Check budget
        let iterations_remaining = if state.budget.max_iterations == 0 {
            u32::MAX
        } else {
            state
                .budget
                .max_iterations
                .saturating_sub(state.iteration_count)
        };

        let runs_remaining = if state.budget.max_oracle_runs == 0 {
            u32::MAX
        } else {
            state
                .budget
                .max_oracle_runs
                .saturating_sub(state.oracle_runs_started)
        };

        let duration_remaining = if state.budget.max_wallclock_hours == 0 {
            i64::MAX
        } else {
            state.budget.max_duration_secs() - (now - state.created_at).num_seconds()
        };

        // Check delay
        let delay_elapsed = if let Some(last) = state.last_iteration_at {
            (now - last).num_seconds() >= state.budget.min_iteration_delay_secs
        } else {
            true
        };

        // Check Work Surface availability per SR-PLAN-V4 Phase 4c
        // If work_unit_id is set, we require work_surface_id to be set (validated)
        // If work_unit_id is None, no Work Surface validation is needed
        let work_surface_available = match &state.work_unit_id {
            Some(_) => state.work_surface_id.is_some(),
            None => true, // No Work Surface validation required
        };

        Ok(IterationPreconditions {
            loop_active: state.loop_state == LoopState::Active,
            no_incomplete_iteration: state.current_iteration_id.is_none()
                || state.current_iteration_state == Some(IterationState::Completed)
                || state.current_iteration_state == Some(IterationState::Failed),
            budget_available: iterations_remaining > 0
                && runs_remaining > 0
                && duration_remaining > 0,
            delay_elapsed,
            no_stop_triggers: state.stop_triggers.is_empty(),
            approvals_satisfied: state.pending_portal_approvals.is_empty(),
            work_surface_available,
        })
    }

    /// Try to start an iteration for a loop
    ///
    /// This is the main governor decision point. It checks preconditions
    /// and emits IterationStarted if all conditions are met.
    #[instrument(skip(self, refs))]
    pub async fn try_start_iteration(
        &self,
        loop_id: &str,
        refs: Vec<TypedRef>,
    ) -> Result<Option<String>, GovernorError> {
        // Check preconditions
        let preconditions = self.check_preconditions(loop_id).await?;

        // Record decision
        let decision_id = format!("dec_{}", ulid::Ulid::new());
        let now = Utc::now();

        let snapshot = {
            let loops = self.loops.read().await;
            let state = loops.get(loop_id).ok_or(GovernorError::LoopNotFound {
                loop_id: loop_id.to_string(),
            })?;

            PreconditionSnapshot {
                loop_state: format!("{:?}", state.loop_state),
                iteration_count: state.iteration_count,
                budget_remaining_iterations: if state.budget.max_iterations == 0 {
                    u32::MAX
                } else {
                    state
                        .budget
                        .max_iterations
                        .saturating_sub(state.iteration_count)
                },
                budget_remaining_oracle_runs: if state.budget.max_oracle_runs == 0 {
                    u32::MAX
                } else {
                    state
                        .budget
                        .max_oracle_runs
                        .saturating_sub(state.oracle_runs_started)
                },
                budget_remaining_secs: if state.budget.max_wallclock_hours == 0 {
                    i64::MAX
                } else {
                    state.budget.max_duration_secs() - (now - state.created_at).num_seconds()
                },
                has_incomplete_iteration: state.current_iteration_id.is_some()
                    && state.current_iteration_state != Some(IterationState::Completed)
                    && state.current_iteration_state != Some(IterationState::Failed),
                stop_triggers: state
                    .stop_triggers
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect(),
                pending_portal_approvals: state.pending_portal_approvals.iter().cloned().collect(),
            }
        };

        if !preconditions.all_satisfied() {
            let reason = preconditions.first_unsatisfied().unwrap_or("unknown");

            let decision = GovernorDecision {
                decision_id: decision_id.clone(),
                loop_id: loop_id.to_string(),
                decision_type: GovernorDecisionType::DeferIteration,
                preconditions: snapshot,
                outcome: GovernorOutcome::Deferred,
                decided_at: now,
                rationale: format!("Precondition not met: {}", reason),
            };

            let mut decisions = self.decisions.write().await;
            decisions.push(decision);

            info!(
                loop_id = %loop_id,
                reason = %reason,
                "Iteration deferred due to precondition"
            );

            return Ok(None);
        }

        // All preconditions met - emit IterationStarted
        let iteration_id = format!("iter_{}", ulid::Ulid::new());
        let iteration_number = {
            let loops = self.loops.read().await;
            loops
                .get(loop_id)
                .map(|s| s.iteration_count + 1)
                .unwrap_or(1)
        };

        // Build IterationStarted event
        let mut event_refs = refs;
        event_refs.push(TypedRef {
            kind: "loop".to_string(),
            id: loop_id.to_string(),
            rel: "parent".to_string(),
            meta: serde_json::Value::Null,
        });

        let payload = serde_json::json!({
            "loop_id": loop_id,
            "iteration_number": iteration_number,
            "started_by": "SYSTEM",
            "preconditions_snapshot": snapshot,
        });

        let event = Self::create_iteration_started_event(
            &iteration_id,
            loop_id,
            iteration_number,
            event_refs,
        );

        // Emit to event store
        // Note: In a real implementation, we'd use append_with_outbox
        // For now, we just record the decision
        let decision = GovernorDecision {
            decision_id: decision_id.clone(),
            loop_id: loop_id.to_string(),
            decision_type: GovernorDecisionType::StartIteration,
            preconditions: snapshot,
            outcome: GovernorOutcome::Executed,
            decided_at: now,
            rationale: format!(
                "All preconditions satisfied, starting iteration {}",
                iteration_number
            ),
        };

        {
            let mut decisions = self.decisions.write().await;
            decisions.push(decision);
        }

        // Update local state
        {
            let mut loops = self.loops.write().await;
            if let Some(state) = loops.get_mut(loop_id) {
                state.iteration_count += 1;
                state.current_iteration_id = Some(iteration_id.clone());
                state.current_iteration_state = Some(IterationState::Started);
                state.last_iteration_at = Some(now);
            }
        }

        info!(
            loop_id = %loop_id,
            iteration_id = %iteration_id,
            iteration_number = iteration_number,
            "Iteration started by governor"
        );

        Ok(Some(iteration_id))
    }

    /// Create an IterationStarted event with SYSTEM actor
    fn create_iteration_started_event(
        iteration_id: &str,
        loop_id: &str,
        iteration_number: u32,
        refs: Vec<TypedRef>,
    ) -> EventEnvelope {
        let now = Utc::now();
        let payload = serde_json::json!({
            "loop_id": loop_id,
            "iteration_number": iteration_number,
        });

        let mut hasher = Sha256::new();
        hasher.update(iteration_id.as_bytes());
        hasher.update(b":");
        hasher.update(loop_id.as_bytes());
        hasher.update(b":");
        hasher.update(&iteration_number.to_le_bytes());
        let envelope_hash = format!("sha256:{}", hex::encode(hasher.finalize()));

        EventEnvelope {
            event_id: EventId::new(),
            stream_id: iteration_id.to_string(),
            stream_kind: StreamKind::Iteration,
            stream_seq: 1,
            global_seq: None,
            event_type: "IterationStarted".to_string(),
            occurred_at: now,
            actor_kind: ActorKind::System, // SYSTEM-only per SR-SPEC
            actor_id: "governor".to_string(),
            correlation_id: Some(loop_id.to_string()),
            causation_id: None,
            supersedes: vec![],
            refs,
            payload,
            envelope_hash,
        }
    }

    /// Get all decisions for a loop (for audit)
    pub async fn get_decisions(&self, loop_id: &str) -> Vec<GovernorDecision> {
        let decisions = self.decisions.read().await;
        decisions
            .iter()
            .filter(|d| d.loop_id == loop_id)
            .cloned()
            .collect()
    }

    /// Get loop tracking state
    pub async fn get_loop_state(&self, loop_id: &str) -> Option<LoopTrackingState> {
        let loops = self.loops.read().await;
        loops.get(loop_id).cloned()
    }

    /// Trigger a stop condition
    #[instrument(skip(self))]
    pub async fn trigger_stop(
        &self,
        loop_id: &str,
        condition: StopCondition,
    ) -> Result<(), GovernorError> {
        let mut loops = self.loops.write().await;
        let state = loops.get_mut(loop_id).ok_or(GovernorError::LoopNotFound {
            loop_id: loop_id.to_string(),
        })?;

        state.stop_triggers.push(condition.clone());

        let decision = GovernorDecision {
            decision_id: format!("dec_{}", ulid::Ulid::new()),
            loop_id: loop_id.to_string(),
            decision_type: GovernorDecisionType::StopLoop,
            preconditions: PreconditionSnapshot {
                loop_state: format!("{:?}", state.loop_state),
                iteration_count: state.iteration_count,
                budget_remaining_iterations: 0,
                budget_remaining_oracle_runs: 0,
                budget_remaining_secs: 0,
                has_incomplete_iteration: false,
                stop_triggers: state
                    .stop_triggers
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect(),
                pending_portal_approvals: state.pending_portal_approvals.iter().cloned().collect(),
            },
            outcome: GovernorOutcome::Executed,
            decided_at: Utc::now(),
            rationale: format!("Stop triggered: {:?}", condition),
        };

        drop(loops);

        let mut decisions = self.decisions.write().await;
        decisions.push(decision);

        info!(
            loop_id = %loop_id,
            condition = ?condition,
            "Stop condition triggered"
        );

        Ok(())
    }

    /// Set Work Surface ID for a loop per SR-PLAN-V4 Phase 4c
    ///
    /// Call this after validating that a Work Surface exists for the work unit.
    /// This enables the governor to pass the work_surface_available precondition.
    #[instrument(skip(self))]
    pub async fn set_work_surface(
        &self,
        loop_id: &str,
        work_surface_id: String,
    ) -> Result<(), GovernorError> {
        let mut loops = self.loops.write().await;
        let state = loops.get_mut(loop_id).ok_or(GovernorError::LoopNotFound {
            loop_id: loop_id.to_string(),
        })?;

        state.work_surface_id = Some(work_surface_id.clone());

        debug!(
            loop_id = %loop_id,
            work_surface_id = %work_surface_id,
            "Work Surface set for loop"
        );

        Ok(())
    }

    /// Clear Work Surface ID for a loop (e.g., when archived)
    ///
    /// This will cause the work_surface_available precondition to fail.
    #[instrument(skip(self))]
    pub async fn clear_work_surface(&self, loop_id: &str) -> Result<(), GovernorError> {
        let mut loops = self.loops.write().await;
        let state = loops.get_mut(loop_id).ok_or(GovernorError::LoopNotFound {
            loop_id: loop_id.to_string(),
        })?;

        if state.work_surface_id.is_some() {
            state.work_surface_id = None;
            debug!(loop_id = %loop_id, "Work Surface cleared for loop");
        }

        Ok(())
    }

    /// Set work unit ID for a loop per SR-PLAN-V4 Phase 4c
    ///
    /// This enables Work Surface validation for the loop.
    #[instrument(skip(self))]
    pub async fn set_work_unit(
        &self,
        loop_id: &str,
        work_unit_id: String,
    ) -> Result<(), GovernorError> {
        let mut loops = self.loops.write().await;
        let state = loops.get_mut(loop_id).ok_or(GovernorError::LoopNotFound {
            loop_id: loop_id.to_string(),
        })?;

        state.work_unit_id = Some(work_unit_id.clone());

        debug!(
            loop_id = %loop_id,
            work_unit_id = %work_unit_id,
            "Work unit set for loop (Work Surface validation enabled)"
        );

        Ok(())
    }

    /// Get the Work Surface ID for a loop (if set)
    pub async fn get_work_surface_id(&self, loop_id: &str) -> Option<String> {
        let loops = self.loops.read().await;
        loops.get(loop_id).and_then(|s| s.work_surface_id.clone())
    }

    /// Get the work unit ID for a loop (if set)
    pub async fn get_work_unit_id(&self, loop_id: &str) -> Option<String> {
        let loops = self.loops.read().await;
        loops.get(loop_id).and_then(|s| s.work_unit_id.clone())
    }

    async fn emit_stop_triggered(
        &self,
        loop_id: &str,
        trigger: &str,
        recommended_portal: Option<&str>,
    ) -> Result<(), GovernorError> {
        let events = self.event_store.read_stream(loop_id, 0, 1000).await?;
        let current_version = events.len() as u64;
        let event_id = EventId::new();

        let mut payload = serde_json::json!({
            "trigger": trigger,
            "condition": trigger,
            "requires_decision": true,
        });

        if let Some(portal) = recommended_portal {
            payload["recommended_portal"] = serde_json::json!(portal);
        }

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: loop_id.to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: current_version.saturating_add(1),
            global_seq: None,
            event_type: "StopTriggered".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "governor".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload,
            envelope_hash: compute_envelope_hash(&event_id),
        };

        self.event_store
            .append(loop_id, current_version, vec![event])
            .await?;

        Ok(())
    }
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[test]
    fn test_preconditions_all_satisfied() {
        let mut pre = IterationPreconditions::default();
        assert!(!pre.all_satisfied());

        pre.loop_active = true;
        pre.no_incomplete_iteration = true;
        pre.budget_available = true;
        pre.delay_elapsed = true;
        pre.no_stop_triggers = true;
        pre.approvals_satisfied = true;
        pre.work_surface_available = true;

        assert!(pre.all_satisfied());
    }

    #[test]
    fn test_preconditions_first_unsatisfied() {
        let mut pre = IterationPreconditions::default();
        assert_eq!(pre.first_unsatisfied(), Some("loop_not_active"));

        pre.loop_active = true;
        assert_eq!(pre.first_unsatisfied(), Some("incomplete_iteration_exists"));

        pre.no_incomplete_iteration = true;
        assert_eq!(pre.first_unsatisfied(), Some("budget_exhausted"));
    }

    #[test]
    fn test_budget_default() {
        let budget = LoopBudget::default();
        assert_eq!(budget.max_iterations, 5);
        assert_eq!(budget.max_oracle_runs, 25);
        assert_eq!(budget.max_wallclock_hours, 16);
        assert_eq!(budget.min_iteration_delay_secs, 1);
    }

    #[tokio::test]
    async fn max_oracle_runs_exhaustion_emits_stop() {
        let event_store = Arc::new(DummyEventStore::default());
        let governor = LoopGovernor::new(event_store.clone());

        let loop_event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_budget".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "LoopCreated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({
                "budgets": {
                    "max_iterations": 5,
                    "max_oracle_runs": 2,
                    "max_wallclock_hours": 16
                }
            }),
            envelope_hash: "hash".to_string(),
        };

        governor.handle_event(&loop_event).await.unwrap();

        let run_started = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "run_evt".to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: 1,
            global_seq: None,
            event_type: "RunStarted".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({"loop_id": "loop_budget"}),
            envelope_hash: "hash".to_string(),
        };

        governor.handle_event(&run_started).await.unwrap();
        governor.handle_event(&run_started).await.unwrap();

        let state = governor.get_loop_state("loop_budget").await.unwrap();
        assert!(state
            .stop_triggers
            .iter()
            .any(|c| matches!(c, StopCondition::BudgetExhausted)));
        assert!(state
            .pending_portal_approvals
            .contains("HumanAuthorityExceptionProcess"));

        let events = event_store.events_for("loop_budget").await;
        let stop = events
            .iter()
            .find(|e| e.event_type == "StopTriggered")
            .expect("stop emitted");
        assert_eq!(
            stop.payload
                .get("recommended_portal")
                .and_then(|v| v.as_str()),
            Some("HumanAuthorityExceptionProcess")
        );
    }

    #[test]
    fn test_loop_tracking_state() {
        let state =
            LoopTrackingState::new("loop_test".to_string(), LoopBudget::default(), Utc::now());

        assert_eq!(state.loop_state, LoopState::Created);
        assert_eq!(state.iteration_count, 0);
        assert_eq!(state.oracle_runs_started, 0);
        assert!(state.current_iteration_id.is_none());
        assert!(state.stop_triggers.is_empty());
        assert!(state.work_unit_id.is_none());
        assert!(state.work_surface_id.is_none());
    }

    #[test]
    fn test_loop_tracking_state_with_work_unit() {
        let state = LoopTrackingState::new_with_work_unit(
            "loop_test".to_string(),
            LoopBudget::default(),
            Utc::now(),
            "wu_test".to_string(),
        );

        assert_eq!(state.loop_state, LoopState::Created);
        assert_eq!(state.work_unit_id, Some("wu_test".to_string()));
        assert_eq!(state.oracle_runs_started, 0);
        assert!(state.work_surface_id.is_none());
    }

    #[test]
    fn test_work_surface_missing_stop_condition() {
        let condition = StopCondition::WorkSurfaceMissing;
        let json = serde_json::to_string(&condition).unwrap();
        assert_eq!(json, "\"WORK_SURFACE_MISSING\"");

        let parsed: StopCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, StopCondition::WorkSurfaceMissing);
    }

    #[test]
    fn test_no_eligible_work_stop_condition() {
        let condition = StopCondition::NoEligibleWork;
        let json = serde_json::to_string(&condition).unwrap();
        assert_eq!(json, "\"NO_ELIGIBLE_WORK\"");

        let parsed: StopCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, StopCondition::NoEligibleWork);
    }

    #[test]
    fn test_work_surface_available_precondition() {
        let mut pre = IterationPreconditions::default();
        pre.loop_active = true;
        pre.no_incomplete_iteration = true;
        pre.budget_available = true;
        pre.delay_elapsed = true;
        pre.no_stop_triggers = true;
        pre.approvals_satisfied = true;
        // work_surface_available defaults to false

        assert!(!pre.all_satisfied());
        assert_eq!(pre.first_unsatisfied(), Some("work_surface_missing"));

        pre.work_surface_available = true;
        assert!(pre.all_satisfied());
        assert_eq!(pre.first_unsatisfied(), None);
    }

    #[test]
    fn test_governor_decision_serialization() {
        let decision = GovernorDecision {
            decision_id: "dec_test".to_string(),
            loop_id: "loop_test".to_string(),
            decision_type: GovernorDecisionType::StartIteration,
            preconditions: PreconditionSnapshot {
                loop_state: "Active".to_string(),
                iteration_count: 5,
                budget_remaining_iterations: 95,
                budget_remaining_oracle_runs: 25,
                budget_remaining_secs: 3000,
                has_incomplete_iteration: false,
                stop_triggers: vec![],
                pending_portal_approvals: vec![],
            },
            outcome: GovernorOutcome::Executed,
            decided_at: Utc::now(),
            rationale: "All preconditions satisfied".to_string(),
        };

        let json = serde_json::to_string(&decision).unwrap();
        let parsed: GovernorDecision = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.decision_id, "dec_test");
        assert_eq!(parsed.decision_type, GovernorDecisionType::StartIteration);
    }

    #[tokio::test]
    async fn run_budget_exhaustion_blocks_preconditions() {
        let governor = LoopGovernor::new(Arc::new(DummyEventStore::default()));
        governor
            .register_loop(
                "loop_test",
                LoopBudget {
                    max_iterations: 5,
                    max_oracle_runs: 1,
                    max_wallclock_hours: 1,
                    min_iteration_delay_secs: 0,
                },
                Utc::now(),
            )
            .await;

        // Activate the loop so budget is the limiting factor
        let activate_event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_test".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "LoopActivated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "test".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({}),
            envelope_hash: "hash".to_string(),
        };
        governor.handle_event(&activate_event).await.unwrap();

        // First run consumes the only allowed oracle run
        let run_event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "run_test".to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: 1,
            global_seq: None,
            event_type: "RunStarted".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "runner".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![TypedRef {
                kind: "Loop".to_string(),
                id: "loop_test".to_string(),
                rel: "parent".to_string(),
                meta: serde_json::Value::Null,
            }],
            payload: json!({ "loop_id": "loop_test" }),
            envelope_hash: "hash".to_string(),
        };
        governor.handle_event(&run_event).await.unwrap();

        let preconditions = governor.check_preconditions("loop_test").await.unwrap();
        assert!(!preconditions.budget_available);
        assert_eq!(preconditions.first_unsatisfied(), Some("budget_exhausted"));
    }

    #[tokio::test]
    async fn loop_updated_budget_extension_clears_budget_stop() {
        let event_store = Arc::new(DummyEventStore::default());
        let governor = LoopGovernor::new(event_store.clone());

        let loop_created = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_budget_update".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "LoopCreated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({
                "budgets": {
                    "max_iterations": 5,
                    "max_oracle_runs": 1,
                    "max_wallclock_hours": 16
                }
            }),
            envelope_hash: "hash".to_string(),
        };
        governor.handle_event(&loop_created).await.unwrap();

        let activate_event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_budget_update".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 2,
            global_seq: None,
            event_type: "LoopActivated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({}),
            envelope_hash: "hash".to_string(),
        };
        governor.handle_event(&activate_event).await.unwrap();

        // Exhaust the original oracle-run budget
        let run_started = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "run_evt".to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: 1,
            global_seq: None,
            event_type: "RunStarted".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({"loop_id": "loop_budget_update"}),
            envelope_hash: "hash".to_string(),
        };
        governor.handle_event(&run_started).await.unwrap();

        let pre_before = governor
            .check_preconditions("loop_budget_update")
            .await
            .unwrap();
        assert!(!pre_before.budget_available);
        assert_eq!(pre_before.first_unsatisfied(), Some("budget_exhausted"));

        // Extend budgets via LoopUpdated and ensure the stop condition is cleared
        let loop_updated = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_budget_update".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 3,
            global_seq: None,
            event_type: "LoopUpdated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({
                "budgets": {
                    "max_iterations": 5,
                    "max_oracle_runs": 3,
                    "max_wallclock_hours": 16
                }
            }),
            envelope_hash: "hash".to_string(),
        };
        governor.handle_event(&loop_updated).await.unwrap();

        let state = governor
            .get_loop_state("loop_budget_update")
            .await
            .expect("loop exists");
        assert_eq!(state.budget.max_oracle_runs, 3);
        assert!(
            !state
                .stop_triggers
                .iter()
                .any(|c| matches!(c, StopCondition::BudgetExhausted))
        );
        assert!(
            !state
                .pending_portal_approvals
                .contains("HumanAuthorityExceptionProcess")
        );

        let pre_after = governor
            .check_preconditions("loop_budget_update")
            .await
            .unwrap();
        assert!(pre_after.budget_available);
        assert!(pre_after.no_stop_triggers);
        assert_eq!(pre_after.first_unsatisfied(), None);
    }

    #[tokio::test]
    async fn loop_created_event_uses_payload_budget() {
        let governor = LoopGovernor::new(Arc::new(DummyEventStore::default()));

        let event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_budget".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "LoopCreated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({
                "budgets": {
                    "max_iterations": 2,
                    "max_oracle_runs": 3,
                    "max_wallclock_hours": 4
                }
            }),
            envelope_hash: "hash".to_string(),
        };

        governor.handle_event(&event).await.unwrap();
        let state = governor.get_loop_state("loop_budget").await.unwrap();

        assert_eq!(state.budget.max_iterations, 2);
        assert_eq!(state.budget.max_oracle_runs, 3);
        assert_eq!(state.budget.max_wallclock_hours, 4);
    }

    #[tokio::test]
    async fn budget_exhausted_stop_tracks_portal_recommendation() {
        let governor = LoopGovernor::new(Arc::new(DummyEventStore::default()));
        governor
            .register_loop("loop_stop", LoopBudget::default(), Utc::now())
            .await;

        let event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_stop".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "StopTriggered".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({
                "trigger": "BUDGET_EXHAUSTED",
                "recommended_portal": "HumanAuthorityExceptionProcess",
                "requires_decision": true
            }),
            envelope_hash: "hash".to_string(),
        };

        governor.handle_event(&event).await.unwrap();
        let state = governor.get_loop_state("loop_stop").await.unwrap();

        assert!(state
            .pending_portal_approvals
            .contains("HumanAuthorityExceptionProcess"));
        assert!(state
            .stop_triggers
            .iter()
            .any(|c| matches!(c, StopCondition::BudgetExhausted)));
    }

    #[tokio::test]
    async fn stop_trigger_uses_trigger_field_and_tracks_portal() {
        let governor = LoopGovernor::new(Arc::new(DummyEventStore::default()));
        governor
            .register_loop("loop_test", LoopBudget::default(), Utc::now())
            .await;

        let event = EventEnvelope {
            event_id: EventId::new(),
            stream_id: "loop_test".to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "StopTriggered".to_string(),
            occurred_at: Utc::now(),
            actor_kind: ActorKind::System,
            actor_id: "tester".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: json!({
                "trigger": "REPEATED_FAILURE",
                "recommended_portal": "HumanAuthorityExceptionProcess",
                "requires_decision": true
            }),
            envelope_hash: "hash".to_string(),
        };

        governor.handle_event(&event).await.unwrap();
        let state = governor.get_loop_state("loop_test").await.unwrap();

        assert_eq!(state.stop_triggers.len(), 1);
        assert_eq!(
            state.pending_portal_approvals,
            std::collections::HashSet::from(["HumanAuthorityExceptionProcess".to_string()])
        );
    }

    #[test]
    fn test_create_iteration_started_event() {
        let event = LoopGovernor::<DummyEventStore>::create_iteration_started_event(
            "iter_test",
            "loop_test",
            1,
            vec![],
        );

        assert_eq!(event.event_type, "IterationStarted");
        assert_eq!(event.actor_kind, ActorKind::System);
        assert_eq!(event.actor_id, "governor");
        assert_eq!(event.stream_kind, StreamKind::Iteration);
    }

    // Dummy event store for tests
    #[derive(Default)]
    struct DummyEventStore {
        streams: Arc<Mutex<HashMap<String, Vec<EventEnvelope>>>>,
    }

    impl DummyEventStore {
        async fn events_for(&self, stream_id: &str) -> Vec<EventEnvelope> {
            let guard = self.streams.lock().await;
            guard.get(stream_id).cloned().unwrap_or_default()
        }
    }

    impl EventStore for DummyEventStore {
        async fn append(
            &self,
            stream_id: &str,
            _expected_version: u64,
            events: Vec<EventEnvelope>,
        ) -> Result<u64, EventStoreError> {
            let mut guard = self.streams.lock().await;
            let stream = guard.entry(stream_id.to_string()).or_default();
            stream.extend(events);
            Ok(stream.len() as u64)
        }

        async fn read_stream(
            &self,
            stream_id: &str,
            from_seq: u64,
            limit: usize,
        ) -> Result<Vec<EventEnvelope>, EventStoreError> {
            let guard = self.streams.lock().await;
            let events = guard.get(stream_id).cloned().unwrap_or_else(Vec::new);

            let start = from_seq as usize;
            let end = if limit == 0 {
                events.len()
            } else {
                std::cmp::min(events.len(), start + limit)
            };

            Ok(events.get(start..end).unwrap_or(&[]).to_vec())
        }

        async fn replay_all(
            &self,
            _from_global_seq: u64,
            _limit: usize,
        ) -> Result<Vec<EventEnvelope>, EventStoreError> {
            let guard = self.streams.lock().await;
            let mut all = Vec::new();
            for events in guard.values() {
                all.extend(events.clone());
            }
            Ok(all)
        }
    }
}
