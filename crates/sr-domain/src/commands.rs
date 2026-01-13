//! Domain commands per SR-SPEC §4.2

use serde::{Deserialize, Serialize};

use crate::entities::{ActorId, TypedRef};

/// Command to create a new Ralph Loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoopCommand {
    pub goal: String,
    pub work_unit: String,
    pub budgets: LoopBudgets,
    pub directive_ref: TypedRef,
    pub actor: ActorId,
}

/// Budget configuration per SR-DIRECTIVE §4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopBudgets {
    pub max_iterations: u32,
    pub max_oracle_runs: u32,
    pub max_wallclock_hours: u32,
}

impl Default for LoopBudgets {
    fn default() -> Self {
        Self {
            max_iterations: 5,
            max_oracle_runs: 25,
            max_wallclock_hours: 16,
        }
    }
}

/// Command to start an iteration (SYSTEM-only per SR-SPEC §2.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartIterationCommand {
    pub loop_id: String,
    pub refs: Vec<TypedRef>,
}

/// Command to complete an iteration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteIterationCommand {
    pub loop_id: String,
    pub iteration_id: String,
    pub summary: IterationSummary,
}

/// Iteration summary per SR-SPEC §3.2.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationSummary {
    pub iteration_id: String,
    pub loop_id: String,
    pub intent: String,
    pub actions: Vec<IterationAction>,
    pub artifacts_touched: Vec<String>,
    pub candidates_produced: Vec<String>,
    pub runs_executed: Vec<String>,
    pub outcomes: IterationOutcomes,
    pub next_steps: Vec<NextStep>,
    pub open_risks: Vec<OpenRisk>,
}

/// Action performed during an iteration per SR-SPEC §3.2.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationAction {
    pub kind: String,
    pub summary: String,
    pub artifacts: Vec<String>,
}

/// Outcomes from an iteration per SR-SPEC §3.2.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationOutcomes {
    pub oracle_results: Vec<OracleResult>,
    pub stop_triggers_fired: Vec<String>,
}

/// Oracle result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResult {
    pub run_id: String,
    pub oracle_suite_id: String,
    pub status: String,
    pub evidence_refs: Vec<String>,
}

/// Next step from an iteration per SR-SPEC §3.2.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStep {
    pub kind: String,
    pub description: String,
    pub blocking: bool,
}

/// Open risk from an iteration per SR-SPEC §3.2.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRisk {
    pub severity: String,
    pub description: String,
    pub mitigation: String,
}
