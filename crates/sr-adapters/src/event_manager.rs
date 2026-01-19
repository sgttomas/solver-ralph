//! Event Manager per SR-EVENT-MANAGER
//!
//! D-40: Implements work-unit + stage state projection + eligible-set computation.
//!
//! The Event Manager is a pure projection builder that consumes the event stream
//! and produces derived state for the Control-Plane. All outputs are rebuildable
//! from the event stream alone.
//!
//! Key outputs:
//! - status_by_work_unit: Work unit status projections
//! - eligible_set: Work units eligible for scheduling
//! - dependency_graph_snapshot: Nodes, edges, satisfaction annotations
//! - runlist: Human-friendly view grouped by status

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sr_domain::{
    plan_instance::{DependencyEdgeType, PlanInstance, WorkUnitPlanStatus},
    work_surface::{StageId, WorkUnitId},
    EventEnvelope,
};
use std::collections::HashMap;
use tracing::{debug, info, instrument, warn};

/// Event Manager error types
#[derive(Debug, thiserror::Error)]
pub enum EventManagerError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Work unit not found: {work_unit_id}")]
    WorkUnitNotFound { work_unit_id: String },

    #[error("Plan instance not loaded")]
    PlanInstanceNotLoaded,

    #[error("Invalid state transition: {message}")]
    InvalidStateTransition { message: String },
}

impl From<sqlx::Error> for EventManagerError {
    fn from(e: sqlx::Error) -> Self {
        EventManagerError::DatabaseError {
            message: e.to_string(),
        }
    }
}

// ============================================================================
// Coarse Status per SR-EVENT-MANAGER §2.1
// ============================================================================

/// Coarse work unit status per SR-EVENT-MANAGER §2.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CoarseStatus {
    /// Work unit is planned but not yet started
    Todo,
    /// Work unit is eligible for scheduling (deps satisfied, not blocked)
    Eligible,
    /// Work unit is currently being executed
    InProgress,
    /// Work unit is blocked (stop trigger, portal, or missing deps)
    Blocked,
    /// Work unit completed successfully (terminal stage passed)
    Complete,
}

impl Default for CoarseStatus {
    fn default() -> Self {
        Self::Todo
    }
}

impl From<WorkUnitPlanStatus> for CoarseStatus {
    fn from(status: WorkUnitPlanStatus) -> Self {
        match status {
            WorkUnitPlanStatus::Pending => CoarseStatus::Todo,
            WorkUnitPlanStatus::Active => CoarseStatus::InProgress,
            WorkUnitPlanStatus::Completed => CoarseStatus::Complete,
            WorkUnitPlanStatus::Blocked => CoarseStatus::Blocked,
            WorkUnitPlanStatus::Skipped => CoarseStatus::Complete,
        }
    }
}

// ============================================================================
// Stage Status per SR-EVENT-MANAGER §2.2
// ============================================================================

/// Stage status within a work unit per SR-EVENT-MANAGER §2.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StageStatus {
    /// Stage not yet entered
    Pending,
    /// Stage entered but not completed
    Active,
    /// Stage completed (gate passed)
    Passed,
    /// Stage failed (gate failed)
    Failed,
    /// Stage blocked (requires portal or has stop trigger)
    Blocked,
}

impl Default for StageStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Stage status entry with evidence reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageStatusEntry {
    pub status: StageStatus,
    pub last_evidence_bundle_ref: Option<String>,
    pub entered_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub iteration_count: u32,
}

impl Default for StageStatusEntry {
    fn default() -> Self {
        Self {
            status: StageStatus::Pending,
            last_evidence_bundle_ref: None,
            entered_at: None,
            completed_at: None,
            iteration_count: 0,
        }
    }
}

// ============================================================================
// Block Reason per SR-EVENT-MANAGER §2.3
// ============================================================================

/// Block reason per SR-EVENT-MANAGER §2.3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockReason {
    pub reason_type: BlockReasonType,
    pub source_event_id: String,
    pub description: String,
    pub requires_portal: bool,
    pub portal_id: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

/// Block reason type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockReasonType {
    StopTrigger,
    PortalRequired,
    DependencyUnsatisfied,
    IntegrityCondition,
    MissingWorkSurface,
}

// ============================================================================
// Staleness Marker per SR-EVENT-MANAGER §2.4
// ============================================================================

/// Staleness marker per SR-EVENT-MANAGER §2.4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StalenessMarkerEntry {
    pub stale_id: String,
    pub root_kind: String,
    pub root_id: String,
    pub reason_code: String,
    pub marked_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// ============================================================================
// Work Unit Status per SR-EVENT-MANAGER §2
// ============================================================================

/// Work unit status projection per SR-EVENT-MANAGER §2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnitStatus {
    /// Work unit identifier
    pub work_unit_id: WorkUnitId,

    /// Coarse status (TODO, ELIGIBLE, IN_PROGRESS, BLOCKED, COMPLETE)
    pub coarse_status: CoarseStatus,

    /// Current stage ID (if stage-gated)
    pub current_stage_id: Option<StageId>,

    /// Stage status map {stage_id -> status entry}
    pub stage_status: HashMap<String, StageStatusEntry>,

    /// Whether all dependencies are satisfied
    pub deps_satisfied: bool,

    /// Active block reasons
    pub block_reasons: Vec<BlockReason>,

    /// Active staleness markers
    pub staleness_markers: Vec<StalenessMarkerEntry>,

    /// Last iteration ID
    pub last_iteration_id: Option<String>,

    /// Last candidate ID
    pub last_candidate_id: Option<String>,

    /// Last evidence bundle ID
    pub last_evidence_bundle_id: Option<String>,

    /// Work surface recorded
    pub has_work_surface: bool,

    /// Last updated timestamp
    pub last_updated_at: DateTime<Utc>,

    /// Last event ID processed
    pub last_event_id: String,
}

impl WorkUnitStatus {
    /// Create a new work unit status (initial state)
    pub fn new(work_unit_id: WorkUnitId) -> Self {
        Self {
            work_unit_id,
            coarse_status: CoarseStatus::Todo,
            current_stage_id: None,
            stage_status: HashMap::new(),
            deps_satisfied: false,
            block_reasons: Vec::new(),
            staleness_markers: Vec::new(),
            last_iteration_id: None,
            last_candidate_id: None,
            last_evidence_bundle_id: None,
            has_work_surface: false,
            last_updated_at: Utc::now(),
            last_event_id: String::new(),
        }
    }

    /// Check if the work unit is complete
    pub fn is_complete(&self) -> bool {
        self.coarse_status == CoarseStatus::Complete
    }

    /// Check if the work unit is blocked
    pub fn is_blocked(&self) -> bool {
        self.coarse_status == CoarseStatus::Blocked || !self.block_reasons.is_empty()
    }

    /// Check if the work unit is eligible for scheduling
    pub fn is_eligible(&self) -> bool {
        self.coarse_status == CoarseStatus::Eligible
            || (self.coarse_status == CoarseStatus::Todo
                && self.deps_satisfied
                && self.block_reasons.is_empty()
                && self.has_work_surface)
    }

    /// Add a block reason
    pub fn add_block_reason(&mut self, reason: BlockReason) {
        self.block_reasons.push(reason);
        if self.coarse_status != CoarseStatus::Complete {
            self.coarse_status = CoarseStatus::Blocked;
        }
    }

    /// Remove block reasons of a specific type
    pub fn remove_block_reasons(&mut self, reason_type: BlockReasonType) {
        self.block_reasons.retain(|r| r.reason_type != reason_type);
        self.recompute_coarse_status();
    }

    /// Recompute coarse status based on current state
    fn recompute_coarse_status(&mut self) {
        if self.coarse_status == CoarseStatus::Complete {
            return; // Complete is terminal
        }

        if !self.block_reasons.is_empty() {
            self.coarse_status = CoarseStatus::Blocked;
        } else if self.last_iteration_id.is_some() {
            self.coarse_status = CoarseStatus::InProgress;
        } else if self.deps_satisfied && self.has_work_surface {
            self.coarse_status = CoarseStatus::Eligible;
        } else {
            self.coarse_status = CoarseStatus::Todo;
        }
    }
}

// ============================================================================
// Eligible Set per SR-EVENT-MANAGER §3
// ============================================================================

/// Eligible set entry per SR-EVENT-MANAGER §3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibleSetEntry {
    pub work_unit_id: WorkUnitId,
    pub priority: i32,
    pub ready_since: DateTime<Utc>,
    pub current_stage_id: Option<StageId>,
}

/// Eligible set per SR-EVENT-MANAGER §3
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EligibleSet {
    pub entries: Vec<EligibleSetEntry>,
    pub computed_at: DateTime<Utc>,
}

impl EligibleSet {
    /// Create a new eligible set from work unit statuses
    pub fn from_statuses(statuses: &HashMap<String, WorkUnitStatus>) -> Self {
        let mut entries: Vec<EligibleSetEntry> = statuses
            .values()
            .filter(|s| s.is_eligible())
            .map(|s| EligibleSetEntry {
                work_unit_id: s.work_unit_id.clone(),
                priority: 0, // Priority can be computed from plan instance
                ready_since: s.last_updated_at,
                current_stage_id: s.current_stage_id.clone(),
            })
            .collect();

        // Sort by priority (descending) then by ready_since (ascending)
        entries.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.ready_since.cmp(&b.ready_since))
        });

        Self {
            entries,
            computed_at: Utc::now(),
        }
    }

    /// Get work unit IDs in the eligible set
    pub fn work_unit_ids(&self) -> Vec<&WorkUnitId> {
        self.entries.iter().map(|e| &e.work_unit_id).collect()
    }

    /// Check if a work unit is in the eligible set
    pub fn contains(&self, work_unit_id: &WorkUnitId) -> bool {
        self.entries.iter().any(|e| &e.work_unit_id == work_unit_id)
    }

    /// Get the size of the eligible set
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the eligible set is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ============================================================================
// Dependency Graph Snapshot per SR-EVENT-MANAGER §4
// ============================================================================

/// Dependency graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraphNode {
    pub work_unit_id: WorkUnitId,
    pub coarse_status: CoarseStatus,
    pub is_satisfied: bool,
}

/// Dependency graph edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraphEdge {
    pub from_work_unit_id: WorkUnitId,
    pub to_work_unit_id: WorkUnitId,
    pub edge_type: DependencyEdgeType,
    pub is_satisfied: bool,
}

/// Dependency graph snapshot per SR-EVENT-MANAGER §4
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraphSnapshot {
    pub nodes: Vec<DependencyGraphNode>,
    pub edges: Vec<DependencyGraphEdge>,
    pub computed_at: DateTime<Utc>,
}

impl DependencyGraphSnapshot {
    /// Build from plan instance and work unit statuses
    pub fn from_plan_and_statuses(
        plan: &PlanInstance,
        statuses: &HashMap<String, WorkUnitStatus>,
    ) -> Self {
        let nodes: Vec<DependencyGraphNode> = plan
            .work_units
            .iter()
            .map(|wu| {
                let status = statuses.get(wu.work_unit_id.as_str());
                DependencyGraphNode {
                    work_unit_id: wu.work_unit_id.clone(),
                    coarse_status: status
                        .map(|s| s.coarse_status)
                        .unwrap_or(CoarseStatus::Todo),
                    is_satisfied: status.map(|s| s.is_complete()).unwrap_or(false),
                }
            })
            .collect();

        let edges: Vec<DependencyGraphEdge> = plan
            .dependency_edges
            .iter()
            .map(|edge| {
                let to_status = statuses.get(edge.to_work_unit_id.as_str());
                DependencyGraphEdge {
                    from_work_unit_id: edge.from_work_unit_id.clone(),
                    to_work_unit_id: edge.to_work_unit_id.clone(),
                    edge_type: edge.edge_type,
                    is_satisfied: to_status.map(|s| s.is_complete()).unwrap_or(false),
                }
            })
            .collect();

        Self {
            nodes,
            edges,
            computed_at: Utc::now(),
        }
    }

    /// Get unsatisfied dependencies for a work unit
    pub fn get_unsatisfied_deps(&self, work_unit_id: &WorkUnitId) -> Vec<&WorkUnitId> {
        self.edges
            .iter()
            .filter(|e| &e.from_work_unit_id == work_unit_id && !e.is_satisfied)
            .map(|e| &e.to_work_unit_id)
            .collect()
    }
}

// ============================================================================
// Run List per SR-EVENT-MANAGER §5
// ============================================================================

/// Run list entry per SR-EVENT-MANAGER §5
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunListEntry {
    pub work_unit_id: WorkUnitId,
    pub title: String,
    pub current_stage: Option<String>,
    pub iteration_count: u32,
    pub last_activity: Option<DateTime<Utc>>,
}

/// Run list grouped by status per SR-EVENT-MANAGER §5
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunList {
    pub todo: Vec<RunListEntry>,
    pub eligible: Vec<RunListEntry>,
    pub in_progress: Vec<RunListEntry>,
    pub blocked: Vec<RunListEntry>,
    pub complete: Vec<RunListEntry>,
    pub computed_at: DateTime<Utc>,
}

impl RunList {
    /// Build from plan instance and work unit statuses
    pub fn from_plan_and_statuses(
        plan: &PlanInstance,
        statuses: &HashMap<String, WorkUnitStatus>,
    ) -> Self {
        let mut run_list = Self {
            computed_at: Utc::now(),
            ..Default::default()
        };

        for wu in &plan.work_units {
            let status = statuses.get(wu.work_unit_id.as_str());
            let coarse_status = status
                .map(|s| s.coarse_status)
                .unwrap_or(CoarseStatus::Todo);

            let iteration_count = status
                .and_then(|s| {
                    s.current_stage_id.as_ref().and_then(|stage_id| {
                        s.stage_status
                            .get(stage_id.as_str())
                            .map(|ss| ss.iteration_count)
                    })
                })
                .unwrap_or(0);

            let entry = RunListEntry {
                work_unit_id: wu.work_unit_id.clone(),
                title: wu.title.clone(),
                current_stage: status.and_then(|s| {
                    s.current_stage_id
                        .as_ref()
                        .map(|id| id.as_str().to_string())
                }),
                iteration_count,
                last_activity: status.map(|s| s.last_updated_at),
            };

            match coarse_status {
                CoarseStatus::Todo => run_list.todo.push(entry),
                CoarseStatus::Eligible => run_list.eligible.push(entry),
                CoarseStatus::InProgress => run_list.in_progress.push(entry),
                CoarseStatus::Blocked => run_list.blocked.push(entry),
                CoarseStatus::Complete => run_list.complete.push(entry),
            }
        }

        run_list
    }

    /// Get total work units count
    pub fn total_count(&self) -> usize {
        self.todo.len()
            + self.eligible.len()
            + self.in_progress.len()
            + self.blocked.len()
            + self.complete.len()
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        let total = self.total_count();
        if total == 0 {
            return 0.0;
        }
        (self.complete.len() as f64 / total as f64) * 100.0
    }
}

// ============================================================================
// Event Manager per SR-EVENT-MANAGER §1
// ============================================================================

/// Event Manager per SR-EVENT-MANAGER §1
///
/// Consumes the event stream and produces derived state for the Control-Plane.
/// All outputs are rebuildable from the event stream alone (deterministic).
pub struct EventManager {
    pool: Option<PgPool>,
    /// Work unit statuses keyed by work_unit_id
    statuses: HashMap<String, WorkUnitStatus>,
    /// Loaded plan instance
    plan_instance: Option<PlanInstance>,
    /// Last processed global sequence
    last_global_seq: u64,
    /// Last processed event ID
    last_event_id: String,
}

impl EventManager {
    /// Create a new event manager
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Some(pool),
            statuses: HashMap::new(),
            plan_instance: None,
            last_global_seq: 0,
            last_event_id: String::new(),
        }
    }

    /// Create an in-memory event manager (for testing and replay verification)
    pub fn new_in_memory() -> Self {
        Self {
            pool: None,
            statuses: HashMap::new(),
            plan_instance: None,
            last_global_seq: 0,
            last_event_id: String::new(),
        }
    }

    /// Get the pool reference
    pub fn pool(&self) -> Option<&PgPool> {
        self.pool.as_ref()
    }

    /// Load a plan instance and initialize work unit statuses
    #[instrument(skip(self, plan))]
    pub fn load_plan_instance(&mut self, plan: PlanInstance) {
        info!(
            plan_id = plan.plan_instance_id.as_str(),
            work_units = plan.work_units.len(),
            "Loading plan instance into Event Manager"
        );

        // Initialize status for each work unit
        for wu in &plan.work_units {
            let mut status = WorkUnitStatus::new(wu.work_unit_id.clone());

            // Check if dependencies are satisfied (initially only true if no deps)
            status.deps_satisfied = wu.depends_on.is_empty();

            // Add block reason for missing work surface
            status.add_block_reason(BlockReason {
                reason_type: BlockReasonType::MissingWorkSurface,
                source_event_id: String::new(),
                description: "Work surface not yet recorded".to_string(),
                requires_portal: false,
                portal_id: None,
                recorded_at: Utc::now(),
            });

            // Add block reasons for unsatisfied dependencies
            for dep_id in &wu.depends_on {
                status.add_block_reason(BlockReason {
                    reason_type: BlockReasonType::DependencyUnsatisfied,
                    source_event_id: String::new(),
                    description: format!("Waiting for dependency: {}", dep_id.as_str()),
                    requires_portal: false,
                    portal_id: None,
                    recorded_at: Utc::now(),
                });
            }

            self.statuses
                .insert(wu.work_unit_id.as_str().to_string(), status);
        }

        self.plan_instance = Some(plan);
    }

    /// Get the current plan instance
    pub fn plan_instance(&self) -> Option<&PlanInstance> {
        self.plan_instance.as_ref()
    }

    /// Get work unit status
    pub fn get_status(&self, work_unit_id: &str) -> Option<&WorkUnitStatus> {
        self.statuses.get(work_unit_id)
    }

    /// Get mutable work unit status
    fn get_status_mut(&mut self, work_unit_id: &str) -> Option<&mut WorkUnitStatus> {
        self.statuses.get_mut(work_unit_id)
    }

    /// Get all work unit statuses
    pub fn all_statuses(&self) -> &HashMap<String, WorkUnitStatus> {
        &self.statuses
    }

    /// Compute the eligible set
    pub fn compute_eligible_set(&self) -> EligibleSet {
        EligibleSet::from_statuses(&self.statuses)
    }

    /// Compute the dependency graph snapshot
    pub fn compute_dependency_graph(&self) -> Result<DependencyGraphSnapshot, EventManagerError> {
        let plan = self
            .plan_instance
            .as_ref()
            .ok_or(EventManagerError::PlanInstanceNotLoaded)?;

        Ok(DependencyGraphSnapshot::from_plan_and_statuses(
            plan,
            &self.statuses,
        ))
    }

    /// Compute the run list
    pub fn compute_run_list(&self) -> Result<RunList, EventManagerError> {
        let plan = self
            .plan_instance
            .as_ref()
            .ok_or(EventManagerError::PlanInstanceNotLoaded)?;

        Ok(RunList::from_plan_and_statuses(plan, &self.statuses))
    }

    /// Apply an event to update projections
    #[instrument(skip(self, event), fields(event_id = %event.event_id.as_str(), event_type = %event.event_type))]
    pub fn apply_event(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        match event.event_type.as_str() {
            // Work surface recorded - enables eligibility
            "WorkSurfaceRecorded" => self.handle_work_surface_recorded(event),

            // Stage events
            "StageEntered" => self.handle_stage_entered(event),
            "StageCompleted" => self.handle_stage_completed(event),

            // Iteration events
            "IterationStarted" => self.handle_iteration_started(event),
            "IterationCompleted" => self.handle_iteration_completed(event),

            // Candidate events
            "CandidateMaterialized" => self.handle_candidate_materialized(event),

            // Evidence events
            "EvidenceBundleRecorded" => self.handle_evidence_bundle_recorded(event),

            // Stop trigger events
            "StopTriggered" => self.handle_stop_triggered(event),

            // Approval events
            "ApprovalRecorded" => self.handle_approval_recorded(event),

            // Oracle events
            "SemanticOracleEvaluated" => self.handle_semantic_oracle_evaluated(event),

            // Staleness events
            "NodeMarkedStale" => self.handle_node_marked_stale(event),
            "StalenessResolved" => self.handle_staleness_resolved(event),

            _ => {
                debug!(event_type = %event.event_type, "Event type not handled by Event Manager");
                Ok(())
            }
        }?;

        // Update last processed
        if let Some(seq) = event.global_seq {
            self.last_global_seq = seq;
        }
        self.last_event_id = event.event_id.as_str().to_string();

        // Recompute dependency satisfaction after any event
        self.recompute_dependency_satisfaction();

        Ok(())
    }

    // ========================================================================
    // Event Handlers
    // ========================================================================

    fn handle_work_surface_recorded(
        &mut self,
        event: &EventEnvelope,
    ) -> Result<(), EventManagerError> {
        let work_unit_id = event.payload["work_unit_id"]
            .as_str()
            .unwrap_or(&event.stream_id);

        if let Some(status) = self.get_status_mut(work_unit_id) {
            status.has_work_surface = true;
            status.remove_block_reasons(BlockReasonType::MissingWorkSurface);

            // Set initial stage if provided
            if let Some(stage_id) = event.payload["stage_id"].as_str() {
                status.current_stage_id = Some(StageId::from_string(stage_id.to_string()));

                // Initialize stage status
                status
                    .stage_status
                    .entry(stage_id.to_string())
                    .or_insert_with(|| StageStatusEntry {
                        status: StageStatus::Pending,
                        ..Default::default()
                    });
            }

            status.last_updated_at = event.occurred_at;
            status.last_event_id = event.event_id.as_str().to_string();

            info!(
                work_unit_id = work_unit_id,
                "Work surface recorded for work unit"
            );
        }

        Ok(())
    }

    fn handle_stage_entered(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        let work_unit_id = event.payload["work_unit_id"]
            .as_str()
            .unwrap_or(&event.stream_id);
        let stage_id = event.payload["stage_id"].as_str().unwrap_or("");

        if let Some(status) = self.get_status_mut(work_unit_id) {
            status.current_stage_id = Some(StageId::from_string(stage_id.to_string()));

            let stage_entry = status.stage_status.entry(stage_id.to_string()).or_default();

            stage_entry.status = StageStatus::Active;
            stage_entry.entered_at = Some(event.occurred_at);
            stage_entry.iteration_count += 1;

            status.last_updated_at = event.occurred_at;
            status.last_event_id = event.event_id.as_str().to_string();

            debug!(
                work_unit_id = work_unit_id,
                stage_id = stage_id,
                "Stage entered"
            );
        }

        Ok(())
    }

    fn handle_stage_completed(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        let work_unit_id = event.payload["work_unit_id"]
            .as_str()
            .unwrap_or(&event.stream_id);
        let stage_id = event.payload["stage_id"].as_str().unwrap_or("");
        let passed = event.payload["passed"].as_bool().unwrap_or(true);
        let is_terminal = event.payload["is_terminal"].as_bool().unwrap_or(false);

        if let Some(status) = self.get_status_mut(work_unit_id) {
            let stage_entry = status.stage_status.entry(stage_id.to_string()).or_default();

            stage_entry.status = if passed {
                StageStatus::Passed
            } else {
                StageStatus::Failed
            };
            stage_entry.completed_at = Some(event.occurred_at);

            // If terminal stage passed, mark work unit complete
            if is_terminal && passed {
                status.coarse_status = CoarseStatus::Complete;
                status.block_reasons.clear();
                info!(work_unit_id = work_unit_id, "Work unit completed");
            }

            status.last_updated_at = event.occurred_at;
            status.last_event_id = event.event_id.as_str().to_string();

            debug!(
                work_unit_id = work_unit_id,
                stage_id = stage_id,
                passed = passed,
                is_terminal = is_terminal,
                "Stage completed"
            );
        }

        Ok(())
    }

    fn handle_iteration_started(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        let iteration_id = &event.stream_id;

        // Extract work_unit_id from refs or payload
        let work_unit_id = event.payload["work_unit_id"].as_str().or_else(|| {
            event.refs.iter().find_map(|r| {
                if r.kind == "WorkUnit" {
                    Some(r.id.as_str())
                } else {
                    None
                }
            })
        });

        if let Some(work_unit_id) = work_unit_id {
            if let Some(status) = self.get_status_mut(work_unit_id) {
                status.last_iteration_id = Some(iteration_id.clone());
                status.coarse_status = CoarseStatus::InProgress;
                status.last_updated_at = event.occurred_at;
                status.last_event_id = event.event_id.as_str().to_string();

                debug!(
                    work_unit_id = work_unit_id,
                    iteration_id = iteration_id,
                    "Iteration started"
                );
            }
        }

        Ok(())
    }

    fn handle_iteration_completed(
        &mut self,
        event: &EventEnvelope,
    ) -> Result<(), EventManagerError> {
        let iteration_id = &event.stream_id;

        // Extract work_unit_id from refs or payload
        let work_unit_id = event.payload["work_unit_id"].as_str().or_else(|| {
            event.refs.iter().find_map(|r| {
                if r.kind == "WorkUnit" {
                    Some(r.id.as_str())
                } else {
                    None
                }
            })
        });

        if let Some(work_unit_id) = work_unit_id {
            if let Some(status) = self.get_status_mut(work_unit_id) {
                status.last_updated_at = event.occurred_at;
                status.last_event_id = event.event_id.as_str().to_string();

                // Recompute status (might go back to eligible if not complete)
                if status.coarse_status != CoarseStatus::Complete {
                    status.recompute_coarse_status();
                }

                debug!(
                    work_unit_id = work_unit_id,
                    iteration_id = iteration_id,
                    "Iteration completed"
                );
            }
        }

        Ok(())
    }

    fn handle_candidate_materialized(
        &mut self,
        event: &EventEnvelope,
    ) -> Result<(), EventManagerError> {
        let candidate_id = &event.stream_id;

        // Extract work_unit_id from refs or payload
        let work_unit_id = event.payload["work_unit_id"].as_str().or_else(|| {
            event.refs.iter().find_map(|r| {
                if r.kind == "WorkUnit" {
                    Some(r.id.as_str())
                } else {
                    None
                }
            })
        });

        if let Some(work_unit_id) = work_unit_id {
            if let Some(status) = self.get_status_mut(work_unit_id) {
                status.last_candidate_id = Some(candidate_id.clone());
                status.last_updated_at = event.occurred_at;
                status.last_event_id = event.event_id.as_str().to_string();

                debug!(
                    work_unit_id = work_unit_id,
                    candidate_id = candidate_id,
                    "Candidate materialized"
                );
            }
        }

        Ok(())
    }

    fn handle_evidence_bundle_recorded(
        &mut self,
        event: &EventEnvelope,
    ) -> Result<(), EventManagerError> {
        let bundle_id = &event.stream_id;

        // Extract work_unit_id and stage_id from refs or payload
        let work_unit_id = event.payload["work_unit_id"].as_str().or_else(|| {
            event.refs.iter().find_map(|r| {
                if r.kind == "WorkUnit" {
                    Some(r.id.as_str())
                } else {
                    None
                }
            })
        });

        let stage_id = event.payload["stage_id"].as_str();

        if let Some(work_unit_id) = work_unit_id {
            if let Some(status) = self.get_status_mut(work_unit_id) {
                status.last_evidence_bundle_id = Some(bundle_id.clone());

                // Update stage status if stage_id is known
                if let Some(stage_id) = stage_id {
                    let stage_entry = status.stage_status.entry(stage_id.to_string()).or_default();
                    stage_entry.last_evidence_bundle_ref = Some(bundle_id.clone());
                }

                status.last_updated_at = event.occurred_at;
                status.last_event_id = event.event_id.as_str().to_string();

                debug!(
                    work_unit_id = work_unit_id,
                    bundle_id = bundle_id,
                    "Evidence bundle recorded"
                );
            }
        }

        Ok(())
    }

    fn handle_stop_triggered(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        let work_unit_id = event.payload["work_unit_id"]
            .as_str()
            .unwrap_or(&event.stream_id);
        let stop_reason = event.payload["reason"].as_str().unwrap_or("Stop triggered");
        let requires_portal = event.payload["requires_portal"].as_bool().unwrap_or(false);
        let portal_id = event.payload["portal_id"].as_str().map(String::from);

        if let Some(status) = self.get_status_mut(work_unit_id) {
            status.add_block_reason(BlockReason {
                reason_type: BlockReasonType::StopTrigger,
                source_event_id: event.event_id.as_str().to_string(),
                description: stop_reason.to_string(),
                requires_portal,
                portal_id,
                recorded_at: event.occurred_at,
            });

            status.last_updated_at = event.occurred_at;
            status.last_event_id = event.event_id.as_str().to_string();

            info!(
                work_unit_id = work_unit_id,
                reason = stop_reason,
                "Stop triggered for work unit"
            );
        }

        Ok(())
    }

    fn handle_approval_recorded(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        // Extract work_unit_id from refs or payload
        let work_unit_id = event.payload["work_unit_id"].as_str().or_else(|| {
            event.refs.iter().find_map(|r| {
                if r.kind == "WorkUnit" {
                    Some(r.id.as_str())
                } else {
                    None
                }
            })
        });

        if let Some(work_unit_id) = work_unit_id {
            if let Some(status) = self.get_status_mut(work_unit_id) {
                // Remove portal-related block reasons
                status.remove_block_reasons(BlockReasonType::PortalRequired);

                // Check if this approval also resolves a stop trigger
                let approval_type = event.payload["approval_type"].as_str();
                if approval_type == Some("stop_trigger_relief") {
                    status.remove_block_reasons(BlockReasonType::StopTrigger);
                }

                status.last_updated_at = event.occurred_at;
                status.last_event_id = event.event_id.as_str().to_string();

                info!(
                    work_unit_id = work_unit_id,
                    "Approval recorded for work unit"
                );
            }
        }

        Ok(())
    }

    fn handle_semantic_oracle_evaluated(
        &mut self,
        event: &EventEnvelope,
    ) -> Result<(), EventManagerError> {
        // Extract work_unit_id and update relevant status
        let work_unit_id = event.payload["work_unit_id"].as_str().or_else(|| {
            event.refs.iter().find_map(|r| {
                if r.kind == "WorkUnit" {
                    Some(r.id.as_str())
                } else {
                    None
                }
            })
        });

        if let Some(work_unit_id) = work_unit_id {
            if let Some(status) = self.get_status_mut(work_unit_id) {
                status.last_updated_at = event.occurred_at;
                status.last_event_id = event.event_id.as_str().to_string();

                debug!(work_unit_id = work_unit_id, "Semantic oracle evaluated");
            }
        }

        Ok(())
    }

    fn handle_node_marked_stale(&mut self, event: &EventEnvelope) -> Result<(), EventManagerError> {
        let dependent_id = event.payload["dependent_id"]
            .as_str()
            .unwrap_or(&event.stream_id);

        if let Some(status) = self.get_status_mut(dependent_id) {
            let stale_id = event.payload["stale_id"].as_str().unwrap_or("").to_string();
            let root_kind = event.payload["root_kind"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let root_id = event.payload["root_id"].as_str().unwrap_or("").to_string();
            let reason_code = event.payload["reason_code"]
                .as_str()
                .unwrap_or("MANUAL_MARK")
                .to_string();

            status.staleness_markers.push(StalenessMarkerEntry {
                stale_id,
                root_kind,
                root_id,
                reason_code,
                marked_at: event.occurred_at,
                resolved_at: None,
            });

            status.last_updated_at = event.occurred_at;
            status.last_event_id = event.event_id.as_str().to_string();

            warn!(work_unit_id = dependent_id, "Work unit marked as stale");
        }

        Ok(())
    }

    fn handle_staleness_resolved(
        &mut self,
        event: &EventEnvelope,
    ) -> Result<(), EventManagerError> {
        let stale_id = event.payload["stale_id"].as_str().unwrap_or("");

        // Find and mark the staleness as resolved in all affected work units
        for status in self.statuses.values_mut() {
            for marker in &mut status.staleness_markers {
                if marker.stale_id == stale_id {
                    marker.resolved_at = Some(event.occurred_at);
                }
            }

            // Remove resolved markers
            status.staleness_markers.retain(|m| m.resolved_at.is_none());
        }

        Ok(())
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// Recompute dependency satisfaction for all work units
    fn recompute_dependency_satisfaction(&mut self) {
        let plan = match &self.plan_instance {
            Some(p) => p,
            None => return,
        };

        // Collect completion status for all work units
        let completion_status: HashMap<String, bool> = self
            .statuses
            .iter()
            .map(|(id, s)| (id.clone(), s.is_complete()))
            .collect();

        // Update dependency satisfaction
        for wu in &plan.work_units {
            let all_deps_satisfied = wu.depends_on.iter().all(|dep_id| {
                completion_status
                    .get(dep_id.as_str())
                    .copied()
                    .unwrap_or(false)
            });

            if let Some(status) = self.statuses.get_mut(wu.work_unit_id.as_str()) {
                status.deps_satisfied = all_deps_satisfied;

                // Update block reasons for dependencies
                if all_deps_satisfied {
                    status.remove_block_reasons(BlockReasonType::DependencyUnsatisfied);
                } else {
                    // Ensure dependency block reasons exist for unsatisfied deps
                    for dep_id in &wu.depends_on {
                        let dep_complete = completion_status
                            .get(dep_id.as_str())
                            .copied()
                            .unwrap_or(false);

                        if !dep_complete {
                            let has_block = status.block_reasons.iter().any(|r| {
                                r.reason_type == BlockReasonType::DependencyUnsatisfied
                                    && r.description.contains(dep_id.as_str())
                            });

                            if !has_block {
                                status.add_block_reason(BlockReason {
                                    reason_type: BlockReasonType::DependencyUnsatisfied,
                                    source_event_id: String::new(),
                                    description: format!(
                                        "Waiting for dependency: {}",
                                        dep_id.as_str()
                                    ),
                                    requires_portal: false,
                                    portal_id: None,
                                    recorded_at: Utc::now(),
                                });
                            }
                        }
                    }
                }

                // Recompute coarse status
                status.recompute_coarse_status();
            }
        }
    }

    /// Rebuild the event manager state from events
    #[instrument(skip(self, events))]
    pub fn rebuild(&mut self, events: &[EventEnvelope]) -> Result<(), EventManagerError> {
        info!(event_count = events.len(), "Rebuilding Event Manager state");

        // Clear existing state
        for status in self.statuses.values_mut() {
            *status = WorkUnitStatus::new(status.work_unit_id.clone());
        }

        // Replay all events
        for event in events {
            self.apply_event(event)?;
        }

        info!("Event Manager rebuild complete");
        Ok(())
    }

    // ========================================================================
    // Replay Proof Methods (D-36)
    // ========================================================================

    /// Compute a deterministic state hash for replay verification
    ///
    /// Per SR-CONTRACT C-EVT-7, projections must be rebuildable from the event log.
    /// This method produces a deterministic hash of all projection state that can
    /// be compared before and after replay to verify determinism.
    ///
    /// The hash incorporates:
    /// - All work unit statuses in sorted key order
    /// - coarse_status, deps_satisfied, current_stage_id, has_work_surface
    /// - Stage status entries in sorted order
    /// - Block reasons (sorted by description for determinism)
    ///
    /// Returns a `sha256:...` formatted hash string.
    pub fn compute_state_hash(&self) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();

        // Hash statuses in deterministic order (sorted by key)
        let mut status_keys: Vec<&String> = self.statuses.keys().collect();
        status_keys.sort();

        for key in status_keys {
            let status = self.statuses.get(key).unwrap();

            // Hash work unit id
            hasher.update(key.as_bytes());
            hasher.update(b":");

            // Hash coarse_status
            hasher.update(format!("{:?}", status.coarse_status).as_bytes());
            hasher.update(b":");

            // Hash deps_satisfied
            hasher.update(if status.deps_satisfied { b"1" } else { b"0" });
            hasher.update(b":");

            // Hash has_work_surface
            hasher.update(if status.has_work_surface { b"1" } else { b"0" });
            hasher.update(b":");

            // Hash current_stage_id
            if let Some(ref stage_id) = status.current_stage_id {
                hasher.update(stage_id.as_str().as_bytes());
            }
            hasher.update(b":");

            // Hash stage_status entries in sorted order
            let mut stage_keys: Vec<&String> = status.stage_status.keys().collect();
            stage_keys.sort();
            for stage_key in stage_keys {
                let stage_entry = status.stage_status.get(stage_key).unwrap();
                hasher.update(stage_key.as_bytes());
                hasher.update(b"=");
                hasher.update(format!("{:?}", stage_entry.status).as_bytes());
                hasher.update(b",");
            }
            hasher.update(b":");

            // Hash block_reasons (sort by description for determinism)
            let mut block_descs: Vec<&str> = status
                .block_reasons
                .iter()
                .map(|r| r.description.as_str())
                .collect();
            block_descs.sort();
            for desc in block_descs {
                hasher.update(desc.as_bytes());
                hasher.update(b";");
            }
            hasher.update(b"|");
        }

        // Include last processed sequence
        hasher.update(self.last_global_seq.to_le_bytes());

        format!("sha256:{}", hex::encode(hasher.finalize()))
    }

    /// Verify replay determinism by replaying events and comparing state hashes
    ///
    /// This method proves that `apply_event()` is deterministic:
    /// 1. Computes the current state hash
    /// 2. Creates a fresh EventManager with the same plan instance
    /// 3. Replays all provided events
    /// 4. Computes the replayed state hash
    /// 5. Compares and reports any discrepancies
    ///
    /// Per SR-CONTRACT C-EVT-7 and C-CTX-2 (no ghost inputs), the replay
    /// must produce identical state since `apply_event()` uses only event data.
    pub fn verify_replay(&self, events: &[EventEnvelope]) -> crate::replay::ReplayProof {
        use crate::replay::ReplayProof;

        // 1. Compute original state hash
        let original_hash = self.compute_state_hash();

        // 2. Create fresh EventManager and load same plan instance
        let mut replayed = EventManager::new_in_memory();
        if let Some(ref plan) = self.plan_instance {
            replayed.load_plan_instance(plan.clone());
        }

        // 3. Replay all events
        for event in events {
            let _ = replayed.apply_event(event);
        }

        // 4. Compute replayed state hash
        let replayed_hash = replayed.compute_state_hash();

        // 5. Find any discrepancies
        let discrepancies = self.find_discrepancies(&replayed);

        // 6. Build and return proof
        ReplayProof::new(
            events,
            original_hash,
            replayed_hash,
            discrepancies,
            self.plan_instance
                .as_ref()
                .map(|p| p.plan_instance_id.as_str().to_string()),
            self.statuses.len(),
        )
    }

    /// Find field-level discrepancies between this EventManager and another
    ///
    /// Compares all work unit statuses field-by-field and reports differences.
    /// This is used by `verify_replay()` to provide detailed diagnostics when
    /// replay produces different state.
    pub fn find_discrepancies(
        &self,
        other: &EventManager,
    ) -> Vec<crate::replay::ReplayDiscrepancy> {
        use crate::replay::ReplayDiscrepancy;

        let mut discrepancies = Vec::new();

        // Check all work units in self
        for (id, status) in &self.statuses {
            match other.statuses.get(id) {
                None => {
                    discrepancies.push(ReplayDiscrepancy::new(
                        id.clone(),
                        "existence",
                        serde_json::json!(true),
                        serde_json::json!(false),
                    ));
                }
                Some(other_status) => {
                    // Compare coarse_status
                    if status.coarse_status != other_status.coarse_status {
                        discrepancies.push(ReplayDiscrepancy::new(
                            id.clone(),
                            "coarse_status",
                            serde_json::json!(format!("{:?}", status.coarse_status)),
                            serde_json::json!(format!("{:?}", other_status.coarse_status)),
                        ));
                    }

                    // Compare deps_satisfied
                    if status.deps_satisfied != other_status.deps_satisfied {
                        discrepancies.push(ReplayDiscrepancy::new(
                            id.clone(),
                            "deps_satisfied",
                            serde_json::json!(status.deps_satisfied),
                            serde_json::json!(other_status.deps_satisfied),
                        ));
                    }

                    // Compare has_work_surface
                    if status.has_work_surface != other_status.has_work_surface {
                        discrepancies.push(ReplayDiscrepancy::new(
                            id.clone(),
                            "has_work_surface",
                            serde_json::json!(status.has_work_surface),
                            serde_json::json!(other_status.has_work_surface),
                        ));
                    }

                    // Compare current_stage_id
                    if status.current_stage_id != other_status.current_stage_id {
                        discrepancies.push(ReplayDiscrepancy::new(
                            id.clone(),
                            "current_stage_id",
                            serde_json::json!(status.current_stage_id.as_ref().map(|s| s.as_str())),
                            serde_json::json!(other_status
                                .current_stage_id
                                .as_ref()
                                .map(|s| s.as_str())),
                        ));
                    }

                    // Compare stage_status map
                    for (stage_id, stage_entry) in &status.stage_status {
                        match other_status.stage_status.get(stage_id) {
                            None => {
                                discrepancies.push(ReplayDiscrepancy::new(
                                    id.clone(),
                                    format!("stage_status.{}", stage_id),
                                    serde_json::json!(format!("{:?}", stage_entry.status)),
                                    serde_json::json!(null),
                                ));
                            }
                            Some(other_entry) => {
                                if stage_entry.status != other_entry.status {
                                    discrepancies.push(ReplayDiscrepancy::new(
                                        id.clone(),
                                        format!("stage_status.{}.status", stage_id),
                                        serde_json::json!(format!("{:?}", stage_entry.status)),
                                        serde_json::json!(format!("{:?}", other_entry.status)),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check for work units in other that aren't in self
        for id in other.statuses.keys() {
            if !self.statuses.contains_key(id) {
                discrepancies.push(ReplayDiscrepancy::new(
                    id.clone(),
                    "existence",
                    serde_json::json!(false),
                    serde_json::json!(true),
                ));
            }
        }

        discrepancies
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use sr_domain::entities::ContentHash;
    use sr_domain::plan_instance::{PlanInstance, SourceRef, SourceRefId, WorkUnitPlan};
    use sr_domain::work_surface::{ContentAddressedRef, TemplateId, WorkKind};
    use sr_domain::{ActorKind, EventId, StreamKind, TypedRef};

    fn create_test_plan() -> PlanInstance {
        let source_ref = SourceRef {
            id: SourceRefId::new("test"),
            content_hash: ContentHash::new("abc123"),
            source_type: "test".to_string(),
            title: Some("Test Plan".to_string()),
        };

        let wu1 = WorkUnitPlan::new(
            WorkUnitId::new("001"),
            "First Work Unit".to_string(),
            WorkKind::ResearchMemo,
            ContentAddressedRef {
                id: "intake:001".to_string(),
                content_hash: ContentHash::new("def456"),
            },
            TemplateId::new("GENERIC"),
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("ghi789"),
            },
            StageId::new("FRAME"),
        );

        let mut wu2 = WorkUnitPlan::new(
            WorkUnitId::new("002"),
            "Second Work Unit".to_string(),
            WorkKind::DecisionRecord,
            ContentAddressedRef {
                id: "intake:002".to_string(),
                content_hash: ContentHash::new("jkl012"),
            },
            TemplateId::new("GENERIC"),
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("ghi789"),
            },
            StageId::new("FRAME"),
        );

        // wu2 depends on wu1
        wu2.add_dependency(wu1.work_unit_id.clone());

        PlanInstance::new(source_ref, vec![wu1, wu2])
    }

    #[test]
    fn test_coarse_status_default() {
        assert_eq!(CoarseStatus::default(), CoarseStatus::Todo);
    }

    #[test]
    fn test_work_unit_status_initial() {
        let status = WorkUnitStatus::new(WorkUnitId::new("test"));
        assert_eq!(status.coarse_status, CoarseStatus::Todo);
        assert!(!status.deps_satisfied);
        assert!(!status.has_work_surface);
        assert!(status.block_reasons.is_empty());
    }

    #[test]
    fn test_eligible_set_empty() {
        let statuses: HashMap<String, WorkUnitStatus> = HashMap::new();
        let eligible_set = EligibleSet::from_statuses(&statuses);
        assert!(eligible_set.is_empty());
    }

    #[test]
    fn test_eligible_set_with_eligible_work_unit() {
        let mut statuses = HashMap::new();
        let mut status = WorkUnitStatus::new(WorkUnitId::new("test"));
        status.deps_satisfied = true;
        status.has_work_surface = true;
        status.block_reasons.clear();
        statuses.insert("WU-test".to_string(), status);

        let eligible_set = EligibleSet::from_statuses(&statuses);
        assert_eq!(eligible_set.len(), 1);
        assert!(eligible_set.contains(&WorkUnitId::new("test")));
    }

    #[test]
    fn test_run_list_completion_percentage() {
        let mut run_list = RunList::default();
        run_list.todo.push(RunListEntry {
            work_unit_id: WorkUnitId::new("001"),
            title: "Todo".to_string(),
            current_stage: None,
            iteration_count: 0,
            last_activity: None,
        });
        run_list.complete.push(RunListEntry {
            work_unit_id: WorkUnitId::new("002"),
            title: "Complete".to_string(),
            current_stage: None,
            iteration_count: 0,
            last_activity: None,
        });

        assert_eq!(run_list.total_count(), 2);
        assert_eq!(run_list.completion_percentage(), 50.0);
    }

    #[test]
    fn test_block_reason_management() {
        let mut status = WorkUnitStatus::new(WorkUnitId::new("test"));

        // Add block reason
        status.add_block_reason(BlockReason {
            reason_type: BlockReasonType::StopTrigger,
            source_event_id: "evt_123".to_string(),
            description: "Test stop".to_string(),
            requires_portal: false,
            portal_id: None,
            recorded_at: Utc::now(),
        });

        assert!(status.is_blocked());
        assert_eq!(status.block_reasons.len(), 1);

        // Remove block reason
        status.remove_block_reasons(BlockReasonType::StopTrigger);
        assert_eq!(status.block_reasons.len(), 0);
    }

    #[test]
    fn test_stage_status_transitions() {
        let mut status = WorkUnitStatus::new(WorkUnitId::new("test"));

        // Initial stage status
        status
            .stage_status
            .insert("stage:FRAME".to_string(), StageStatusEntry::default());

        let stage_entry = status.stage_status.get("stage:FRAME").unwrap();
        assert_eq!(stage_entry.status, StageStatus::Pending);

        // Update to active
        let stage_entry = status.stage_status.get_mut("stage:FRAME").unwrap();
        stage_entry.status = StageStatus::Active;
        stage_entry.entered_at = Some(Utc::now());

        assert_eq!(
            status.stage_status.get("stage:FRAME").unwrap().status,
            StageStatus::Active
        );

        // Update to passed
        let stage_entry = status.stage_status.get_mut("stage:FRAME").unwrap();
        stage_entry.status = StageStatus::Passed;
        stage_entry.completed_at = Some(Utc::now());

        assert_eq!(
            status.stage_status.get("stage:FRAME").unwrap().status,
            StageStatus::Passed
        );
    }

    #[test]
    fn test_dependency_graph_snapshot() {
        let plan = create_test_plan();
        let statuses: HashMap<String, WorkUnitStatus> = plan
            .work_units
            .iter()
            .map(|wu| {
                (
                    wu.work_unit_id.as_str().to_string(),
                    WorkUnitStatus::new(wu.work_unit_id.clone()),
                )
            })
            .collect();

        let snapshot = DependencyGraphSnapshot::from_plan_and_statuses(&plan, &statuses);

        assert_eq!(snapshot.nodes.len(), 2);
        assert_eq!(snapshot.edges.len(), 1);

        // Check unsatisfied dependencies for wu2
        let unsatisfied = snapshot.get_unsatisfied_deps(&WorkUnitId::new("002"));
        assert_eq!(unsatisfied.len(), 1);
        assert_eq!(unsatisfied[0].as_str(), "WU-001");
    }

    // ========================================================================
    // Deterministic Replay Tests per SR-EVENT-MANAGER §6
    // ========================================================================

    fn create_test_event(
        event_type: &str,
        stream_id: &str,
        payload: serde_json::Value,
    ) -> EventEnvelope {
        EventEnvelope {
            event_id: EventId::new(),
            event_type: event_type.to_string(),
            stream_kind: StreamKind::Loop,
            stream_id: stream_id.to_string(),
            stream_seq: 1,
            global_seq: Some(1),
            occurred_at: Utc::now(),
            payload,
            actor_kind: ActorKind::System,
            actor_id: "test".to_string(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            envelope_hash: "test_hash".to_string(),
        }
    }

    fn create_event_with_refs(
        event_type: &str,
        stream_id: &str,
        payload: serde_json::Value,
        refs: Vec<TypedRef>,
    ) -> EventEnvelope {
        let mut event = create_test_event(event_type, stream_id, payload);
        event.refs = refs;
        event
    }

    fn create_em_with_plan(plan: &PlanInstance) -> EventManager {
        let mut em = EventManager::new_in_memory();
        for wu in &plan.work_units {
            let mut status = WorkUnitStatus::new(wu.work_unit_id.clone());
            status.deps_satisfied = wu.depends_on.is_empty();
            em.statuses
                .insert(wu.work_unit_id.as_str().to_string(), status);
        }
        em.plan_instance = Some(plan.clone());
        em
    }

    #[test]
    fn test_deterministic_replay_work_surface_recorded() {
        // Test that replaying the same events produces the same state
        let plan = create_test_plan();

        // Create first event manager and apply events
        let mut em1 = create_em_with_plan(&plan);

        // Create event
        let event = create_test_event(
            "WorkSurfaceRecorded",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "stage_id": "stage:FRAME"
            }),
        );

        // Apply event
        em1.apply_event(&event).unwrap();

        // Get state after applying event
        let status1 = em1.get_status("WU-001").unwrap();
        assert!(status1.has_work_surface);
        assert_eq!(
            status1.current_stage_id.as_ref().map(|s| s.as_str()),
            Some("stage:FRAME")
        );

        // Create second event manager and replay
        let mut em2 = create_em_with_plan(&plan);

        // Replay same event
        em2.apply_event(&event).unwrap();

        // Verify state is identical
        let status2 = em2.get_status("WU-001").unwrap();
        assert_eq!(status1.has_work_surface, status2.has_work_surface);
        assert_eq!(status1.current_stage_id, status2.current_stage_id);
        assert_eq!(status1.coarse_status, status2.coarse_status);
    }

    #[test]
    fn test_deterministic_replay_stage_progression() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan(&plan);

        // Simulate full workflow for wu1
        let events = vec![
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "StageEntered",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "StageCompleted",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME",
                    "passed": true,
                    "is_terminal": true
                }),
            ),
        ];

        // Apply all events
        for event in &events {
            em.apply_event(event).unwrap();
        }

        // Verify wu1 is complete
        let status1 = em.get_status("WU-001").unwrap();
        assert_eq!(status1.coarse_status, CoarseStatus::Complete);

        // Verify wu2's dependency is now satisfied
        let status2 = em.get_status("WU-002").unwrap();
        assert!(status2.deps_satisfied);
    }

    #[test]
    fn test_eligible_set_computation_determinism() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan(&plan);

        // Initially, nothing is eligible (no work surface)
        let eligible1 = em.compute_eligible_set();
        assert!(eligible1.is_empty());

        // Record work surface for wu1
        em.apply_event(&create_test_event(
            "WorkSurfaceRecorded",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "stage_id": "stage:FRAME"
            }),
        ))
        .unwrap();

        // Now wu1 should be eligible (has work surface, deps satisfied)
        let eligible2 = em.compute_eligible_set();
        assert_eq!(eligible2.len(), 1);
        assert!(eligible2.contains(&WorkUnitId::new("001")));

        // wu2 should NOT be eligible (dependency not satisfied)
        assert!(!eligible2.contains(&WorkUnitId::new("002")));
    }

    fn create_em_with_plan_ready(plan: &PlanInstance) -> EventManager {
        let mut em = EventManager::new_in_memory();
        for wu in &plan.work_units {
            let mut status = WorkUnitStatus::new(wu.work_unit_id.clone());
            status.deps_satisfied = wu.depends_on.is_empty();
            status.has_work_surface = true; // Pre-set work surface
            status.block_reasons.clear();
            em.statuses
                .insert(wu.work_unit_id.as_str().to_string(), status);
        }
        em.plan_instance = Some(plan.clone());
        em
    }

    #[test]
    fn test_stop_trigger_blocks_work_unit() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan_ready(&plan);

        // wu1 should be eligible initially
        let eligible1 = em.compute_eligible_set();
        assert!(eligible1.contains(&WorkUnitId::new("001")));

        // Trigger stop
        em.apply_event(&create_test_event(
            "StopTriggered",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "reason": "Oracle failed",
                "requires_portal": true,
                "portal_id": "portal_relief"
            }),
        ))
        .unwrap();

        // wu1 should now be blocked
        let status = em.get_status("WU-001").unwrap();
        assert_eq!(status.coarse_status, CoarseStatus::Blocked);
        assert!(status.is_blocked());

        // wu1 should not be eligible
        let eligible2 = em.compute_eligible_set();
        assert!(!eligible2.contains(&WorkUnitId::new("001")));
    }

    #[test]
    fn test_approval_unblocks_work_unit() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan_ready(&plan);

        // Trigger stop
        em.apply_event(&create_test_event(
            "StopTriggered",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "reason": "Oracle failed",
                "requires_portal": true,
                "portal_id": "portal_relief"
            }),
        ))
        .unwrap();

        assert!(em.get_status("WU-001").unwrap().is_blocked());

        // Record approval to unblock
        em.apply_event(&create_event_with_refs(
            "ApprovalRecorded",
            "appr_123",
            serde_json::json!({
                "approval_type": "stop_trigger_relief"
            }),
            vec![TypedRef {
                kind: "WorkUnit".to_string(),
                id: "WU-001".to_string(),
                rel: "about".to_string(),
                meta: serde_json::json!({}),
            }],
        ))
        .unwrap();

        // wu1 should no longer be blocked
        let status = em.get_status("WU-001").unwrap();
        assert!(!status.is_blocked());
        assert!(status.is_eligible());
    }

    #[test]
    fn test_run_list_reflects_current_state() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan(&plan);

        // Initial run list - wu1 is TODO (no work surface but no deps), wu2 is TODO/BLOCKED (has dep)
        let run_list = em.compute_run_list().unwrap();
        // Both are TODO since we didn't add block reasons in the helper
        assert_eq!(run_list.todo.len() + run_list.blocked.len(), 2);
        assert_eq!(run_list.completion_percentage(), 0.0);

        // Complete wu1 workflow
        em.apply_event(&create_test_event(
            "WorkSurfaceRecorded",
            "WU-001",
            serde_json::json!({ "work_unit_id": "WU-001", "stage_id": "stage:FRAME" }),
        ))
        .unwrap();
        em.apply_event(&create_test_event(
            "StageCompleted",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "stage_id": "stage:FRAME",
                "passed": true,
                "is_terminal": true
            }),
        ))
        .unwrap();

        // Run list should now show 1 complete, 1 blocked
        let run_list = em.compute_run_list().unwrap();
        assert_eq!(run_list.complete.len(), 1);
        assert_eq!(run_list.completion_percentage(), 50.0);
    }

    // ========================================================================
    // Replay Proof Tests (D-36)
    // ========================================================================

    #[test]
    fn test_compute_state_hash_deterministic() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan(&plan);

        // Apply some events
        em.apply_event(&create_test_event(
            "WorkSurfaceRecorded",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "stage_id": "stage:FRAME"
            }),
        ))
        .unwrap();

        // Compute hash twice - should be identical
        let hash1 = em.compute_state_hash();
        let hash2 = em.compute_state_hash();

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
        assert_eq!(hash1.len(), 7 + 64); // "sha256:" + 64 hex chars
    }

    #[test]
    fn test_compute_state_hash_changes_with_state() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan(&plan);

        let hash1 = em.compute_state_hash();

        // Apply an event that changes state
        em.apply_event(&create_test_event(
            "WorkSurfaceRecorded",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "stage_id": "stage:FRAME"
            }),
        ))
        .unwrap();

        let hash2 = em.compute_state_hash();

        // Hash should be different after state change
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_replay_deterministic() {
        let plan = create_test_plan();

        // Use load_plan_instance for consistent initialization with verify_replay
        let mut em = EventManager::new_in_memory();
        em.load_plan_instance(plan.clone());

        // Create a sequence of events for BOTH work units
        // This ensures both work units are fully initialized
        let events = vec![
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-002",
                serde_json::json!({
                    "work_unit_id": "WU-002",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "StageEntered",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "StageCompleted",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME",
                    "passed": true,
                    "is_terminal": true
                }),
            ),
        ];

        // Apply all events
        for event in &events {
            em.apply_event(event).unwrap();
        }

        // Verify replay is deterministic
        let proof = em.verify_replay(&events);

        assert!(
            proof.is_deterministic(),
            "Replay should be deterministic. Discrepancies: {:?}",
            proof.discrepancies
        );
        assert_eq!(proof.discrepancy_count(), 0);
        assert_eq!(proof.original_state_hash, proof.replayed_state_hash);
        assert_eq!(proof.event_count, 4);
    }

    #[test]
    fn test_verify_replay_eligible_set_determinism() {
        let plan = create_test_plan();
        let mut em = create_em_with_plan(&plan);

        // Make wu1 complete so wu2's dependency is satisfied
        let events = vec![
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "StageCompleted",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME",
                    "passed": true,
                    "is_terminal": true
                }),
            ),
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-002",
                serde_json::json!({
                    "work_unit_id": "WU-002",
                    "stage_id": "stage:FRAME"
                }),
            ),
        ];

        // Apply events
        for event in &events {
            em.apply_event(event).unwrap();
        }

        // Get original eligible set
        let original_eligible = em.compute_eligible_set();

        // Create fresh EM and replay
        let mut replayed = create_em_with_plan(&plan);
        for event in &events {
            replayed.apply_event(event).unwrap();
        }

        // Get replayed eligible set
        let replayed_eligible = replayed.compute_eligible_set();

        // Eligible sets should be identical
        assert_eq!(original_eligible.len(), replayed_eligible.len());
        for entry in original_eligible.work_unit_ids() {
            assert!(replayed_eligible.contains(entry));
        }
    }

    #[test]
    fn test_find_discrepancies_empty_when_identical() {
        let plan = create_test_plan();
        let em1 = create_em_with_plan(&plan);
        let em2 = create_em_with_plan(&plan);

        let discrepancies = em1.find_discrepancies(&em2);
        assert!(discrepancies.is_empty());
    }

    #[test]
    fn test_find_discrepancies_detects_difference() {
        let plan = create_test_plan();
        let mut em1 = create_em_with_plan(&plan);
        let em2 = create_em_with_plan(&plan);

        // Modify em1 state
        em1.apply_event(&create_test_event(
            "WorkSurfaceRecorded",
            "WU-001",
            serde_json::json!({
                "work_unit_id": "WU-001",
                "stage_id": "stage:FRAME"
            }),
        ))
        .unwrap();

        let discrepancies = em1.find_discrepancies(&em2);
        assert!(!discrepancies.is_empty());

        // Should have discrepancy for has_work_surface
        let has_ws_discrepancy = discrepancies.iter().find(|d| d.field == "has_work_surface");
        assert!(has_ws_discrepancy.is_some());
    }

    #[test]
    fn test_replay_proof_full_workflow() {
        // This test simulates a complete Branch 0-style workflow
        // and proves that replay is deterministic
        let plan = create_test_plan();

        // Use load_plan_instance for consistent initialization with verify_replay
        let mut em = EventManager::new_in_memory();
        em.load_plan_instance(plan.clone());

        // Full workflow for both work units to ensure complete state
        let events = vec![
            // Initialize work surfaces for both work units
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_test_event(
                "WorkSurfaceRecorded",
                "WU-002",
                serde_json::json!({
                    "work_unit_id": "WU-002",
                    "stage_id": "stage:FRAME"
                }),
            ),
            // WU-001 workflow
            create_test_event(
                "StageEntered",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME"
                }),
            ),
            create_event_with_refs(
                "IterationStarted",
                "iter_001",
                serde_json::json!({}),
                vec![TypedRef {
                    kind: "WorkUnit".to_string(),
                    id: "WU-001".to_string(),
                    rel: "about".to_string(),
                    meta: serde_json::json!({}),
                }],
            ),
            create_event_with_refs(
                "CandidateMaterialized",
                "cand_001",
                serde_json::json!({}),
                vec![TypedRef {
                    kind: "WorkUnit".to_string(),
                    id: "WU-001".to_string(),
                    rel: "about".to_string(),
                    meta: serde_json::json!({}),
                }],
            ),
            create_event_with_refs(
                "EvidenceBundleRecorded",
                "bundle_001",
                serde_json::json!({
                    "stage_id": "stage:FRAME"
                }),
                vec![TypedRef {
                    kind: "WorkUnit".to_string(),
                    id: "WU-001".to_string(),
                    rel: "about".to_string(),
                    meta: serde_json::json!({}),
                }],
            ),
            create_test_event(
                "StageCompleted",
                "WU-001",
                serde_json::json!({
                    "work_unit_id": "WU-001",
                    "stage_id": "stage:FRAME",
                    "passed": true,
                    "is_terminal": true
                }),
            ),
        ];

        // Apply all events
        for event in &events {
            em.apply_event(event).unwrap();
        }

        // Verify wu1 is complete
        assert_eq!(
            em.get_status("WU-001").unwrap().coarse_status,
            CoarseStatus::Complete
        );

        // Verify replay is deterministic
        let proof = em.verify_replay(&events);

        assert!(
            proof.is_deterministic(),
            "Replay should be deterministic. Discrepancies: {:?}",
            proof.discrepancies
        );
        assert_eq!(proof.original_state_hash, proof.replayed_state_hash);
        assert_eq!(proof.event_count, events.len());
        assert_eq!(proof.work_unit_count, 2);
    }
}
