//! Work Surface Schemas per SR-WORK-SURFACE
//!
//! This module defines the governed artifacts that a Semantic Ralph Loop operates on:
//! - Intake: structured objective/scope/constraints for a work unit
//! - ProcedureTemplate: stage-gated procedure definition
//! - WorkSurfaceInstance: binding context for an iteration
//! - ManagedWorkSurface: runtime representation with lifecycle tracking (SR-PLAN-V4)
//!
//! Per SR-WORK-SURFACE, these artifacts define "what" must exist (schemas + invariants)
//! for semantic knowledge work.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use ulid::Ulid;

use crate::entities::{ActorId, ContentHash, TypedRef};
use crate::errors::DomainError;

// ============================================================================
// Identifiers
// ============================================================================

/// Intake identifier per SR-WORK-SURFACE §3
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntakeId(String);

impl IntakeId {
    pub fn new(work_unit_id: &str) -> Self {
        Self(format!("intake:{work_unit_id}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Procedure template identifier per SR-WORK-SURFACE §4
/// Format: `proc:<NAME>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcedureTemplateId(String);

impl ProcedureTemplateId {
    pub fn new(name: &str) -> Self {
        Self(format!("proc:{name}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stage identifier per SR-WORK-SURFACE §4
/// Format: `stage:<NAME>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StageId(String);

impl StageId {
    pub fn new(name: &str) -> Self {
        Self(format!("stage:{name}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Work unit identifier (referenced from entities)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkUnitId(String);

impl WorkUnitId {
    pub fn new(id: &str) -> Self {
        Self(format!("WU-{id}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Work Surface Instance identifier per SR-PLAN-V4 §1.1
/// Format: `ws:<ULID>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkSurfaceId(String);

impl WorkSurfaceId {
    /// Create a new WorkSurfaceId with a fresh ULID
    pub fn new() -> Self {
        Self(format!("ws:{}", Ulid::new()))
    }

    /// Create from an existing string
    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate the format (ws:<ULID>)
    pub fn is_valid(&self) -> bool {
        self.0.starts_with("ws:") && self.0.len() > 3
    }
}

impl Default for WorkSurfaceId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WorkSurfaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Intake Schema (v1) per SR-WORK-SURFACE §3
// ============================================================================

/// Work kind taxonomy per SR-WORK-SURFACE §3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkKind {
    ResearchMemo,
    DecisionRecord,
    OntologyBuild,
    AnalysisReport,
    TechnicalSpec,
    ImplementationPlan,
    IntakeProcessing,
    Custom(String),
}

impl Default for WorkKind {
    fn default() -> Self {
        Self::ResearchMemo
    }
}

/// Deliverable specification per SR-WORK-SURFACE §3.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    /// Path where the deliverable should be produced
    pub path: String,

    /// Media type (e.g., "text/markdown", "application/json")
    pub media_type: String,

    /// Optional description of the deliverable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Role of this deliverable (e.g., "primary", "supporting")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Constraint specification per SR-WORK-SURFACE §3.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Constraint description
    pub description: String,

    /// Constraint category (structure, content, tone, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Whether this constraint is mandatory or advisory
    #[serde(default)]
    pub mandatory: bool,
}

/// Input reference for intake context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRef {
    /// Relationship type
    pub rel: String,

    /// Kind of input
    pub kind: String,

    /// Locator (path or ref)
    pub locator: String,

    /// Content hash if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,
}

/// Unknown/question to resolve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unknown {
    /// Question or uncertainty
    pub question: String,

    /// Priority/severity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Suggested resolution approach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_approach: Option<String>,
}

/// Intake schema (v1) per SR-WORK-SURFACE §3
///
/// The Intake is a work-unit-scoped artifact that defines what work is to be done.
/// When used as binding iteration context, it MUST be treated as a commitment object
/// and referenced by hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intake {
    /// Artifact type identifier
    #[serde(default = "Intake::default_artifact_type")]
    pub artifact_type: String,

    /// Schema version
    #[serde(default = "Intake::default_artifact_version")]
    pub artifact_version: String,

    /// Work unit identifier (stable)
    pub work_unit_id: WorkUnitId,

    /// Human-readable title
    pub title: String,

    /// Work kind taxonomy
    pub kind: WorkKind,

    /// One-sentence objective
    pub objective: String,

    /// Target audience
    pub audience: String,

    /// Required deliverables
    pub deliverables: Vec<Deliverable>,

    /// Constraints (length, tone, structure, etc.)
    #[serde(default)]
    pub constraints: Vec<Constraint>,

    /// Term definitions (term -> definition)
    #[serde(default)]
    pub definitions: HashMap<String, String>,

    /// Input refs (provided context)
    #[serde(default)]
    pub inputs: Vec<InputRef>,

    /// Questions to resolve
    #[serde(default)]
    pub unknowns: Vec<Unknown>,

    /// Human-facing acceptance criteria (not authoritative gate)
    #[serde(default)]
    pub completion_criteria: Vec<String>,

    /// Content hash (computed when materialized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl Intake {
    fn default_artifact_type() -> String {
        "record.intake".to_string()
    }

    fn default_artifact_version() -> String {
        "v1".to_string()
    }

    /// Create a new intake with minimal required fields
    pub fn new(
        work_unit_id: WorkUnitId,
        title: String,
        kind: WorkKind,
        objective: String,
        audience: String,
        deliverables: Vec<Deliverable>,
    ) -> Self {
        Self {
            artifact_type: Self::default_artifact_type(),
            artifact_version: Self::default_artifact_version(),
            work_unit_id,
            title,
            kind,
            objective,
            audience,
            deliverables,
            constraints: Vec::new(),
            definitions: HashMap::new(),
            inputs: Vec::new(),
            unknowns: Vec::new(),
            completion_criteria: Vec::new(),
            content_hash: None,
            created_at: Some(Utc::now()),
        }
    }

    /// Validate the intake per SR-WORK-SURFACE §3.1 required fields
    pub fn validate(&self) -> Result<(), DomainError> {
        IntakeValidator::validate(self)
    }
}

// ============================================================================
// Procedure Template Schema (v1) per SR-WORK-SURFACE §4
// ============================================================================

/// Required output for a stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredOutput {
    /// Path where the output should be produced
    pub path: String,

    /// Role of this output (e.g., "context", "candidate", "evidence")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Media type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

/// Procedural step within a stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureStep {
    /// Step number/order
    pub step_number: u32,

    /// Step instruction
    pub instruction: String,

    /// Expected outputs from this step
    #[serde(default)]
    pub outputs: Vec<String>,
}

/// Gate rule for stage completion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateRule {
    /// All required oracles must pass
    AllRequiredOraclesPass,

    /// All oracles must pass (including advisory)
    AllOraclesPass,

    /// Custom rule with expression
    Custom(String),

    /// Portal approval required
    PortalApprovalRequired,
}

impl Default for GateRule {
    fn default() -> Self {
        Self::AllRequiredOraclesPass
    }
}

/// Transition target when stage passes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransitionTarget {
    /// Transition to another stage
    Stage(StageId),

    /// Terminal (procedure complete)
    Terminal,
}

/// Stage definition per SR-WORK-SURFACE §4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    /// Stage identifier
    pub stage_id: StageId,

    /// Human-readable stage name
    pub stage_name: String,

    /// Purpose of this stage
    pub purpose: String,

    /// Required outputs at this stage
    pub required_outputs: Vec<RequiredOutput>,

    /// Procedural steps (may be empty if implementation-specific)
    #[serde(default)]
    pub steps: Vec<ProcedureStep>,

    /// Required oracle suites
    pub required_oracle_suites: Vec<String>,

    /// Gate rule for completion
    #[serde(default)]
    pub gate_rule: GateRule,

    /// Next stage on pass (or terminal)
    pub transition_on_pass: TransitionTarget,

    /// Whether this stage requires portal approval
    #[serde(default)]
    pub requires_portal: bool,

    /// Portal identifier (if requires_portal is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,

    /// Whether this stage requires human approval before completion
    /// Per SR-PROCEDURE-KIT §1: Trust boundary flag for stage-gated approval
    #[serde(default)]
    pub requires_approval: bool,
}

/// Procedure Template schema (v1) per SR-WORK-SURFACE §4
///
/// Procedure Templates are governed configuration artifacts that define
/// stage-gated procedures for semantic work. They SHOULD be reusable
/// across work units of the same kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureTemplate {
    /// Artifact type identifier
    #[serde(default = "ProcedureTemplate::default_artifact_type")]
    pub artifact_type: String,

    /// Schema version
    #[serde(default = "ProcedureTemplate::default_artifact_version")]
    pub artifact_version: String,

    /// Template identifier
    pub procedure_template_id: ProcedureTemplateId,

    /// Work kinds this template applies to
    pub kind: Vec<WorkKind>,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Stage definitions
    pub stages: Vec<Stage>,

    /// Terminal stage identifier
    pub terminal_stage_id: StageId,

    /// Initial stage (if not first in stages list)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_stage_id: Option<StageId>,

    /// Content hash (computed when materialized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,

    /// Version string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ProcedureTemplate {
    fn default_artifact_type() -> String {
        "config.procedure_template".to_string()
    }

    fn default_artifact_version() -> String {
        "v1".to_string()
    }

    /// Get the initial stage ID
    pub fn get_initial_stage(&self) -> &StageId {
        self.initial_stage_id.as_ref().unwrap_or_else(|| {
            &self
                .stages
                .first()
                .expect("Template must have at least one stage")
                .stage_id
        })
    }

    /// Get a stage by ID
    pub fn get_stage(&self, stage_id: &StageId) -> Option<&Stage> {
        self.stages.iter().find(|s| &s.stage_id == stage_id)
    }

    /// Check if a stage is terminal
    pub fn is_terminal(&self, stage_id: &StageId) -> bool {
        stage_id == &self.terminal_stage_id
    }

    /// Get the next stage after a given stage
    pub fn get_next_stage(&self, current_stage_id: &StageId) -> Option<&StageId> {
        let stage = self.get_stage(current_stage_id)?;
        match &stage.transition_on_pass {
            TransitionTarget::Stage(next) => Some(next),
            TransitionTarget::Terminal => None,
        }
    }

    /// Validate the template per SR-WORK-SURFACE §4.1
    pub fn validate(&self) -> Result<(), DomainError> {
        ProcedureTemplateValidator::validate(self)
    }
}

// ============================================================================
// Work Surface Instance Schema (v1) per SR-WORK-SURFACE §5
// ============================================================================

/// Content-addressed reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAddressedRef {
    /// Identifier
    pub id: String,

    /// Content hash
    pub content_hash: ContentHash,
}

/// Oracle suite binding with hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleSuiteBinding {
    /// Suite identifier
    pub suite_id: String,

    /// Suite hash (incorporates semantic set definitions)
    pub suite_hash: ContentHash,
}

/// Work Surface Instance schema (v1) per SR-WORK-SURFACE §5
///
/// A Work Surface Instance is the binding of a specific work unit to:
/// - an Intake
/// - a Procedure Template
/// - the current stage
/// - the oracle profile/suites for that stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceInstance {
    /// Artifact type identifier
    #[serde(default = "WorkSurfaceInstance::default_artifact_type")]
    pub artifact_type: String,

    /// Schema version
    #[serde(default = "WorkSurfaceInstance::default_artifact_version")]
    pub artifact_version: String,

    /// Work unit identifier
    pub work_unit_id: WorkUnitId,

    /// Content-addressed reference to the Intake
    pub intake_ref: ContentAddressedRef,

    /// Content-addressed reference to the Procedure Template
    pub procedure_template_ref: ContentAddressedRef,

    /// Current stage identifier
    pub stage_id: StageId,

    /// Oracle suites for this stage (with hashes for determinism)
    pub oracle_suites: Vec<OracleSuiteBinding>,

    /// Optional stage parameters
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,

    /// Content hash of this work surface instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl WorkSurfaceInstance {
    fn default_artifact_type() -> String {
        "domain.work_surface".to_string()
    }

    fn default_artifact_version() -> String {
        "v1".to_string()
    }

    /// Create a new work surface instance
    pub fn new(
        work_unit_id: WorkUnitId,
        intake_ref: ContentAddressedRef,
        procedure_template_ref: ContentAddressedRef,
        stage_id: StageId,
        oracle_suites: Vec<OracleSuiteBinding>,
    ) -> Self {
        Self {
            artifact_type: Self::default_artifact_type(),
            artifact_version: Self::default_artifact_version(),
            work_unit_id,
            intake_ref,
            procedure_template_ref,
            stage_id,
            oracle_suites,
            params: HashMap::new(),
            content_hash: None,
            created_at: Some(Utc::now()),
        }
    }

    /// Convert to typed refs for IterationStarted
    pub fn to_typed_refs(&self) -> Vec<TypedRef> {
        let mut refs = Vec::new();

        // Intake ref
        refs.push(TypedRef {
            kind: "Intake".to_string(),
            id: self.intake_ref.id.clone(),
            rel: "depends_on".to_string(),
            meta: serde_json::json!({
                "content_hash": self.intake_ref.content_hash.as_str()
            }),
        });

        // Procedure template ref
        refs.push(TypedRef {
            kind: "ProcedureTemplate".to_string(),
            id: self.procedure_template_ref.id.clone(),
            rel: "depends_on".to_string(),
            meta: serde_json::json!({
                "content_hash": self.procedure_template_ref.content_hash.as_str(),
                "current_stage_id": self.stage_id.as_str()
            }),
        });

        // Oracle suite refs
        for suite in &self.oracle_suites {
            refs.push(TypedRef {
                kind: "OracleSuite".to_string(),
                id: suite.suite_id.clone(),
                rel: "depends_on".to_string(),
                meta: serde_json::json!({
                    "content_hash": suite.suite_hash.as_str(),
                    "suite_hash": suite.suite_hash.as_str()
                }),
            });
        }

        refs
    }

    /// Validate the work surface instance
    pub fn validate(&self) -> Result<(), DomainError> {
        WorkSurfaceInstanceValidator::validate(self)
    }
}

// ============================================================================
// Work Surface Lifecycle per SR-PLAN-V4
// ============================================================================

/// Work Surface status (lifecycle) per SR-PLAN-V4 §1.1
///
/// Note: Uses lifecycle values (Active/Completed/Archived) rather than
/// SR-TYPES §3.1 artifact status values (draft/governed/archived) because
/// Work Surfaces are always commitment objects once bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkSurfaceStatus {
    /// Active — work in progress
    #[default]
    Active,
    /// Completed — terminal stage reached with passing gate
    Completed,
    /// Archived — superseded or abandoned
    Archived,
}

impl WorkSurfaceStatus {
    /// Check if the work surface is still active
    pub fn is_active(&self) -> bool {
        matches!(self, WorkSurfaceStatus::Active)
    }

    /// Check if the work surface is terminal (completed or archived)
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            WorkSurfaceStatus::Completed | WorkSurfaceStatus::Archived
        )
    }

    /// Check if stage transitions are allowed
    pub fn can_transition_stage(&self) -> bool {
        matches!(self, WorkSurfaceStatus::Active)
    }
}

impl std::fmt::Display for WorkSurfaceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkSurfaceStatus::Active => write!(f, "active"),
            WorkSurfaceStatus::Completed => write!(f, "completed"),
            WorkSurfaceStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Stage completion status per SR-PLAN-V4 §1.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageCompletionStatus {
    /// Not yet entered
    #[default]
    Pending,
    /// Currently active
    Entered,
    /// Completed with passing gate
    Completed,
    /// Skipped (if allowed by template)
    Skipped,
}

impl StageCompletionStatus {
    /// Check if the stage is complete or skipped
    pub fn is_done(&self) -> bool {
        matches!(
            self,
            StageCompletionStatus::Completed | StageCompletionStatus::Skipped
        )
    }

    /// Check if the stage is currently active
    pub fn is_current(&self) -> bool {
        matches!(self, StageCompletionStatus::Entered)
    }
}

impl std::fmt::Display for StageCompletionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StageCompletionStatus::Pending => write!(f, "pending"),
            StageCompletionStatus::Entered => write!(f, "entered"),
            StageCompletionStatus::Completed => write!(f, "completed"),
            StageCompletionStatus::Skipped => write!(f, "skipped"),
        }
    }
}

/// Stage status record for tracking per SR-PLAN-V4 §1.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageStatusRecord {
    pub stage_id: StageId,
    pub status: StageCompletionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entered_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_bundle_ref: Option<String>,
    #[serde(default)]
    pub iteration_count: u32,
}

impl StageStatusRecord {
    /// Create a new pending stage record
    pub fn pending(stage_id: StageId) -> Self {
        Self {
            stage_id,
            status: StageCompletionStatus::Pending,
            entered_at: None,
            completed_at: None,
            evidence_bundle_ref: None,
            iteration_count: 0,
        }
    }

    /// Create a new entered stage record
    pub fn entered(stage_id: StageId) -> Self {
        Self {
            stage_id,
            status: StageCompletionStatus::Entered,
            entered_at: Some(Utc::now()),
            completed_at: None,
            evidence_bundle_ref: None,
            iteration_count: 0,
        }
    }

    /// Mark the stage as entered
    pub fn mark_entered(&mut self) {
        self.status = StageCompletionStatus::Entered;
        self.entered_at = Some(Utc::now());
    }

    /// Mark the stage as completed
    pub fn mark_completed(&mut self, evidence_bundle_ref: String) {
        self.status = StageCompletionStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.evidence_bundle_ref = Some(evidence_bundle_ref);
    }

    /// Increment iteration count
    pub fn increment_iteration(&mut self) {
        self.iteration_count += 1;
    }
}

/// Managed Work Surface Instance (runtime representation) per SR-PLAN-V4 §1.1
///
/// This extends WorkSurfaceInstance with:
/// - ULID-based ID (WorkSurfaceId)
/// - Status lifecycle (Active → Completed | Archived)
/// - Stage tracking with StageStatusRecord
/// - Attribution per C-EVT-1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedWorkSurface {
    // === Identity ===
    /// Unique identifier (format: "ws:<ULID>")
    pub work_surface_id: WorkSurfaceId,

    /// Work unit this work surface belongs to
    pub work_unit_id: WorkUnitId,

    // === Binding refs (immutable once bound) ===
    /// Content-addressed reference to the Intake
    pub intake_ref: ContentAddressedRef,

    /// Content-addressed reference to the Procedure Template
    pub procedure_template_ref: ContentAddressedRef,

    // === Current state (mutable via events) ===
    /// Current stage identifier
    pub current_stage_id: StageId,

    /// Work Surface status
    pub status: WorkSurfaceStatus,

    /// Stage status tracking {stage_id: StageStatusRecord}
    pub stage_status: HashMap<String, StageStatusRecord>,

    // === Oracle context for current stage ===
    /// Oracle suites for the current stage (refreshed on stage entry)
    pub current_oracle_suites: Vec<OracleSuiteBinding>,

    // === Parameters ===
    /// Stage parameters and semantic set selectors
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,

    // === Content addressing ===
    /// Content hash of the binding (intake + template + initial params)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,

    // === Attribution (per C-EVT-1) ===
    pub bound_at: DateTime<Utc>,
    pub bound_by: ActorId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<ActorId>,
}

impl ManagedWorkSurface {
    /// Create a new bound work surface
    #[allow(clippy::too_many_arguments)]
    pub fn new_bound(
        work_unit_id: WorkUnitId,
        intake_ref: ContentAddressedRef,
        procedure_template_ref: ContentAddressedRef,
        initial_stage_id: StageId,
        oracle_suites: Vec<OracleSuiteBinding>,
        params: HashMap<String, serde_json::Value>,
        bound_by: ActorId,
    ) -> Self {
        let work_surface_id = WorkSurfaceId::new();

        // Initialize stage status with the initial stage as entered
        let mut stage_status = HashMap::new();
        stage_status.insert(
            initial_stage_id.as_str().to_string(),
            StageStatusRecord::entered(initial_stage_id.clone()),
        );

        let mut ws = Self {
            work_surface_id,
            work_unit_id,
            intake_ref,
            procedure_template_ref,
            current_stage_id: initial_stage_id,
            status: WorkSurfaceStatus::Active,
            stage_status,
            current_oracle_suites: oracle_suites,
            params,
            content_hash: None,
            bound_at: Utc::now(),
            bound_by,
            completed_at: None,
            archived_at: None,
            archived_by: None,
        };

        // Compute content hash
        ws.content_hash = Some(ws.compute_content_hash());
        ws
    }

    /// Compute the content hash for this work surface binding
    ///
    /// Uses canonical JSON serialization (sorted keys) for determinism.
    /// Only includes immutable binding fields, not mutable state.
    pub fn compute_content_hash(&self) -> String {
        let canonical = serde_json::json!({
            "work_unit_id": self.work_unit_id.as_str(),
            "intake_ref": {
                "id": self.intake_ref.id,
                "content_hash": self.intake_ref.content_hash.as_str()
            },
            "procedure_template_ref": {
                "id": self.procedure_template_ref.id,
                "content_hash": self.procedure_template_ref.content_hash.as_str()
            },
            "params": self.params,
        });

        let canonical_str = serde_json::to_string(&canonical).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(canonical_str.as_bytes());
        format!("sha256:{}", hex::encode(hasher.finalize()))
    }

    /// Enter a new stage
    ///
    /// Returns error if work surface is not active.
    pub fn enter_stage(
        &mut self,
        stage_id: StageId,
        oracle_suites: Vec<OracleSuiteBinding>,
    ) -> Result<(), WorkSurfaceLifecycleError> {
        if !self.status.can_transition_stage() {
            return Err(WorkSurfaceLifecycleError::InvalidTransition {
                from: self.status,
                reason: "Cannot transition stages on non-active work surface".to_string(),
            });
        }

        // Update stage status
        let stage_key = stage_id.as_str().to_string();
        self.stage_status
            .entry(stage_key.clone())
            .and_modify(|record| record.mark_entered())
            .or_insert_with(|| StageStatusRecord::entered(stage_id.clone()));

        self.current_stage_id = stage_id;
        self.current_oracle_suites = oracle_suites;

        Ok(())
    }

    /// Complete the current stage
    ///
    /// Returns error if work surface is not active.
    pub fn complete_stage(
        &mut self,
        evidence_bundle_ref: String,
    ) -> Result<(), WorkSurfaceLifecycleError> {
        if !self.status.can_transition_stage() {
            return Err(WorkSurfaceLifecycleError::InvalidTransition {
                from: self.status,
                reason: "Cannot complete stage on non-active work surface".to_string(),
            });
        }

        let stage_key = self.current_stage_id.as_str().to_string();
        if let Some(record) = self.stage_status.get_mut(&stage_key) {
            record.mark_completed(evidence_bundle_ref);
        }

        Ok(())
    }

    /// Mark the work surface as completed (terminal stage passed)
    pub fn mark_completed(&mut self) -> Result<(), WorkSurfaceLifecycleError> {
        if !self.status.is_active() {
            return Err(WorkSurfaceLifecycleError::InvalidTransition {
                from: self.status,
                reason: "Can only complete an active work surface".to_string(),
            });
        }

        self.status = WorkSurfaceStatus::Completed;
        self.completed_at = Some(Utc::now());

        Ok(())
    }

    /// Archive the work surface
    pub fn archive(&mut self, actor: ActorId) -> Result<(), WorkSurfaceLifecycleError> {
        if self.status == WorkSurfaceStatus::Archived {
            return Err(WorkSurfaceLifecycleError::InvalidTransition {
                from: self.status,
                reason: "Work surface is already archived".to_string(),
            });
        }

        self.status = WorkSurfaceStatus::Archived;
        self.archived_at = Some(Utc::now());
        self.archived_by = Some(actor);

        Ok(())
    }

    /// Increment iteration count for current stage
    pub fn increment_iteration(&mut self) {
        let stage_key = self.current_stage_id.as_str().to_string();
        if let Some(record) = self.stage_status.get_mut(&stage_key) {
            record.increment_iteration();
        }
    }

    /// Convert to WorkSurfaceInstance (for backward compatibility)
    pub fn to_work_surface_instance(&self) -> WorkSurfaceInstance {
        WorkSurfaceInstance {
            artifact_type: "domain.work_surface".to_string(),
            artifact_version: "v1".to_string(),
            work_unit_id: self.work_unit_id.clone(),
            intake_ref: self.intake_ref.clone(),
            procedure_template_ref: self.procedure_template_ref.clone(),
            stage_id: self.current_stage_id.clone(),
            oracle_suites: self.current_oracle_suites.clone(),
            params: self.params.clone(),
            content_hash: self.content_hash.as_ref().map(|s| ContentHash::new(s)),
            created_at: Some(self.bound_at),
        }
    }
}

/// Errors from work surface lifecycle operations
#[derive(Debug, Clone, PartialEq)]
pub enum WorkSurfaceLifecycleError {
    InvalidTransition {
        from: WorkSurfaceStatus,
        reason: String,
    },
    StageNotFound {
        stage_id: String,
    },
    IncompatibleWorkKind {
        intake_kind: String,
        template_kinds: Vec<String>,
    },
}

impl std::fmt::Display for WorkSurfaceLifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTransition { from, reason } => {
                write!(f, "Invalid transition from {from}: {reason}")
            }
            Self::StageNotFound { stage_id } => {
                write!(f, "Stage not found: {stage_id}")
            }
            Self::IncompatibleWorkKind {
                intake_kind,
                template_kinds,
            } => {
                write!(
                    f,
                    "Intake kind '{}' not supported by template (supports: {:?})",
                    intake_kind, template_kinds
                )
            }
        }
    }
}

impl std::error::Error for WorkSurfaceLifecycleError {}

/// Validate compatibility between Intake and Procedure Template per SR-PLAN-V4 §1
pub fn validate_work_kind_compatibility(
    intake_kind: &WorkKind,
    template_kinds: &[WorkKind],
) -> Result<(), WorkSurfaceLifecycleError> {
    if !template_kinds.contains(intake_kind) {
        return Err(WorkSurfaceLifecycleError::IncompatibleWorkKind {
            intake_kind: format!("{:?}", intake_kind),
            template_kinds: template_kinds.iter().map(|k| format!("{:?}", k)).collect(),
        });
    }
    Ok(())
}

// ============================================================================
// Validators
// ============================================================================

/// Intake validator per SR-WORK-SURFACE §3.1 required fields
pub struct IntakeValidator;

impl IntakeValidator {
    /// Validate an intake for required fields and format
    pub fn validate(intake: &Intake) -> Result<(), DomainError> {
        // Check required fields per SR-WORK-SURFACE §3.1
        if intake.work_unit_id.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "Intake.work_unit_id is required".to_string(),
            });
        }

        if intake.title.trim().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "Intake.title is required".to_string(),
            });
        }

        if intake.objective.trim().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "Intake.objective is required".to_string(),
            });
        }

        if intake.audience.trim().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "Intake.audience is required".to_string(),
            });
        }

        if intake.deliverables.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "Intake.deliverables[] must have at least one entry".to_string(),
            });
        }

        // Validate each deliverable
        for (i, deliverable) in intake.deliverables.iter().enumerate() {
            if deliverable.path.trim().is_empty() {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("Intake.deliverables[{i}].path is required"),
                });
            }
            if deliverable.media_type.trim().is_empty() {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("Intake.deliverables[{i}].media_type is required"),
                });
            }
        }

        Ok(())
    }
}

/// Procedure template validator per SR-WORK-SURFACE §4.1
pub struct ProcedureTemplateValidator;

impl ProcedureTemplateValidator {
    /// Validate a procedure template
    pub fn validate(template: &ProcedureTemplate) -> Result<(), DomainError> {
        // Check required fields
        if template.procedure_template_id.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "ProcedureTemplate.procedure_template_id is required".to_string(),
            });
        }

        if template.kind.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "ProcedureTemplate.kind[] must have at least one entry".to_string(),
            });
        }

        if template.stages.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "ProcedureTemplate.stages[] must have at least one entry".to_string(),
            });
        }

        // Validate each stage
        for (i, stage) in template.stages.iter().enumerate() {
            Self::validate_stage(stage, i)?;
        }

        // Verify terminal stage exists
        let terminal_exists = template
            .stages
            .iter()
            .any(|s| s.stage_id == template.terminal_stage_id);
        if !terminal_exists {
            return Err(DomainError::InvariantViolation {
                invariant: format!(
                    "terminal_stage_id '{}' not found in stages",
                    template.terminal_stage_id.as_str()
                ),
            });
        }

        // Verify all transition targets exist or are terminal
        for stage in &template.stages {
            match &stage.transition_on_pass {
                TransitionTarget::Stage(target_id) => {
                    let target_exists = template.stages.iter().any(|s| &s.stage_id == target_id);
                    if !target_exists {
                        return Err(DomainError::InvariantViolation {
                            invariant: format!(
                                "Stage '{}' transition target '{}' not found",
                                stage.stage_id.as_str(),
                                target_id.as_str()
                            ),
                        });
                    }
                }
                TransitionTarget::Terminal => {}
            }
        }

        // Verify initial stage exists (if specified)
        if let Some(initial) = &template.initial_stage_id {
            let initial_exists = template.stages.iter().any(|s| &s.stage_id == initial);
            if !initial_exists {
                return Err(DomainError::InvariantViolation {
                    invariant: format!(
                        "initial_stage_id '{}' not found in stages",
                        initial.as_str()
                    ),
                });
            }
        }

        Ok(())
    }

    fn validate_stage(stage: &Stage, index: usize) -> Result<(), DomainError> {
        if stage.stage_id.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: format!("stages[{index}].stage_id is required"),
            });
        }

        if stage.stage_name.trim().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: format!("stages[{index}].stage_name is required"),
            });
        }

        if stage.purpose.trim().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: format!("stages[{index}].purpose is required"),
            });
        }

        // required_oracle_suites may be empty for some stages
        // required_outputs may be empty for some stages

        // Validate portal configuration
        if stage.requires_portal && stage.portal_id.is_none() {
            return Err(DomainError::InvariantViolation {
                invariant: format!("stages[{index}] requires_portal=true but portal_id is not set"),
            });
        }

        Ok(())
    }
}

/// Work surface instance validator per SR-WORK-SURFACE §5.1
pub struct WorkSurfaceInstanceValidator;

impl WorkSurfaceInstanceValidator {
    /// Validate a work surface instance
    pub fn validate(instance: &WorkSurfaceInstance) -> Result<(), DomainError> {
        // Check required fields
        if instance.work_unit_id.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "WorkSurfaceInstance.work_unit_id is required".to_string(),
            });
        }

        if instance.intake_ref.id.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "WorkSurfaceInstance.intake_ref.id is required".to_string(),
            });
        }

        if instance.intake_ref.content_hash.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "WorkSurfaceInstance.intake_ref.content_hash is required".to_string(),
            });
        }

        if instance.procedure_template_ref.id.is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "WorkSurfaceInstance.procedure_template_ref.id is required".to_string(),
            });
        }

        if instance
            .procedure_template_ref
            .content_hash
            .as_str()
            .is_empty()
        {
            return Err(DomainError::InvariantViolation {
                invariant: "WorkSurfaceInstance.procedure_template_ref.content_hash is required"
                    .to_string(),
            });
        }

        if instance.stage_id.as_str().is_empty() {
            return Err(DomainError::InvariantViolation {
                invariant: "WorkSurfaceInstance.stage_id is required".to_string(),
            });
        }

        // Validate oracle suite bindings
        for (i, suite) in instance.oracle_suites.iter().enumerate() {
            if suite.suite_id.is_empty() {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("oracle_suites[{i}].suite_id is required"),
                });
            }
            if suite.suite_hash.as_str().is_empty() {
                return Err(DomainError::InvariantViolation {
                    invariant: format!("oracle_suites[{i}].suite_hash is required"),
                });
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
    fn test_intake_id_format() {
        let id = IntakeId::new("test-work-unit");
        assert_eq!(id.as_str(), "intake:test-work-unit");
    }

    #[test]
    fn test_procedure_template_id_format() {
        let id = ProcedureTemplateId::new("GENERIC-KNOWLEDGE-WORK");
        assert_eq!(id.as_str(), "proc:GENERIC-KNOWLEDGE-WORK");
    }

    #[test]
    fn test_stage_id_format() {
        let id = StageId::new("FRAME");
        assert_eq!(id.as_str(), "stage:FRAME");
    }

    #[test]
    fn test_work_unit_id_format() {
        let id = WorkUnitId::new("001");
        assert_eq!(id.as_str(), "WU-001");
    }

    #[test]
    fn test_intake_creation() {
        let intake = Intake::new(
            WorkUnitId::new("test"),
            "Test Intake".to_string(),
            WorkKind::ResearchMemo,
            "Produce a research memo".to_string(),
            "Technical team".to_string(),
            vec![Deliverable {
                path: "candidate/main.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
        );

        assert_eq!(intake.artifact_type, "record.intake");
        assert_eq!(intake.artifact_version, "v1");
        assert!(intake.validate().is_ok());
    }

    #[test]
    fn test_intake_validation_missing_title() {
        let intake = Intake {
            artifact_type: "record.intake".to_string(),
            artifact_version: "v1".to_string(),
            work_unit_id: WorkUnitId::new("test"),
            title: "".to_string(),
            kind: WorkKind::ResearchMemo,
            objective: "test".to_string(),
            audience: "test".to_string(),
            deliverables: vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            constraints: Vec::new(),
            definitions: HashMap::new(),
            inputs: Vec::new(),
            unknowns: Vec::new(),
            completion_criteria: Vec::new(),
            content_hash: None,
            created_at: None,
        };

        let result = intake.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("title"));
    }

    #[test]
    fn test_intake_validation_no_deliverables() {
        let intake = Intake {
            artifact_type: "record.intake".to_string(),
            artifact_version: "v1".to_string(),
            work_unit_id: WorkUnitId::new("test"),
            title: "Test".to_string(),
            kind: WorkKind::ResearchMemo,
            objective: "test".to_string(),
            audience: "test".to_string(),
            deliverables: vec![],
            constraints: Vec::new(),
            definitions: HashMap::new(),
            inputs: Vec::new(),
            unknowns: Vec::new(),
            completion_criteria: Vec::new(),
            content_hash: None,
            created_at: None,
        };

        let result = intake.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("deliverables"));
    }

    #[test]
    fn test_procedure_template_creation() {
        let template = ProcedureTemplate {
            artifact_type: "config.procedure_template".to_string(),
            artifact_version: "v1".to_string(),
            procedure_template_id: ProcedureTemplateId::new("TEST"),
            kind: vec![WorkKind::ResearchMemo],
            name: Some("Test Template".to_string()),
            description: None,
            stages: vec![
                Stage {
                    stage_id: StageId::new("FRAME"),
                    stage_name: "Frame the problem".to_string(),
                    purpose: "Restate objective".to_string(),
                    required_outputs: vec![],
                    steps: vec![],
                    required_oracle_suites: vec!["suite:SR-SUITE-STRUCTURE".to_string()],
                    gate_rule: GateRule::AllRequiredOraclesPass,
                    transition_on_pass: TransitionTarget::Stage(StageId::new("FINAL")),
                    requires_portal: false,
                    portal_id: None,
                    requires_approval: false,
                },
                Stage {
                    stage_id: StageId::new("FINAL"),
                    stage_name: "Final".to_string(),
                    purpose: "Package final output".to_string(),
                    required_outputs: vec![],
                    steps: vec![],
                    required_oracle_suites: vec![],
                    gate_rule: GateRule::AllRequiredOraclesPass,
                    transition_on_pass: TransitionTarget::Terminal,
                    requires_portal: false,
                    portal_id: None,
                    requires_approval: false,
                },
            ],
            terminal_stage_id: StageId::new("FINAL"),
            initial_stage_id: None,
            content_hash: None,
            version: Some("1.0.0".to_string()),
        };

        assert!(template.validate().is_ok());
        assert_eq!(template.get_initial_stage().as_str(), "stage:FRAME");
        assert!(template.is_terminal(&StageId::new("FINAL")));
    }

    #[test]
    fn test_procedure_template_invalid_terminal() {
        let template = ProcedureTemplate {
            artifact_type: "config.procedure_template".to_string(),
            artifact_version: "v1".to_string(),
            procedure_template_id: ProcedureTemplateId::new("TEST"),
            kind: vec![WorkKind::ResearchMemo],
            name: None,
            description: None,
            stages: vec![Stage {
                stage_id: StageId::new("FRAME"),
                stage_name: "Frame".to_string(),
                purpose: "Frame".to_string(),
                required_outputs: vec![],
                steps: vec![],
                required_oracle_suites: vec![],
                gate_rule: GateRule::AllRequiredOraclesPass,
                transition_on_pass: TransitionTarget::Terminal,
                requires_portal: false,
                portal_id: None,
                requires_approval: false,
            }],
            terminal_stage_id: StageId::new("NONEXISTENT"),
            initial_stage_id: None,
            content_hash: None,
            version: None,
        };

        let result = template.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("terminal_stage_id"));
    }

    #[test]
    fn test_work_surface_instance_creation() {
        let instance = WorkSurfaceInstance::new(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:test".to_string(),
                content_hash: ContentHash::new("abc123"),
            },
            ContentAddressedRef {
                id: "proc:TEST".to_string(),
                content_hash: ContentHash::new("def456"),
            },
            StageId::new("FRAME"),
            vec![OracleSuiteBinding {
                suite_id: "suite:SR-SUITE-STRUCTURE".to_string(),
                suite_hash: ContentHash::new("ghi789"),
            }],
        );

        assert_eq!(instance.artifact_type, "domain.work_surface");
        assert!(instance.validate().is_ok());

        let refs = instance.to_typed_refs();
        assert_eq!(refs.len(), 3); // Intake, ProcedureTemplate, OracleSuite
        assert!(refs.iter().any(|r| r.kind == "Intake"));
        assert!(refs.iter().any(|r| r.kind == "ProcedureTemplate"));
        assert!(refs.iter().any(|r| r.kind == "OracleSuite"));
    }

    #[test]
    fn test_work_surface_instance_validation_missing_intake() {
        let instance = WorkSurfaceInstance {
            artifact_type: "domain.work_surface".to_string(),
            artifact_version: "v1".to_string(),
            work_unit_id: WorkUnitId::new("test"),
            intake_ref: ContentAddressedRef {
                id: "".to_string(),
                content_hash: ContentHash::new("abc"),
            },
            procedure_template_ref: ContentAddressedRef {
                id: "proc:TEST".to_string(),
                content_hash: ContentHash::new("def"),
            },
            stage_id: StageId::new("FRAME"),
            oracle_suites: vec![],
            params: HashMap::new(),
            content_hash: None,
            created_at: None,
        };

        let result = instance.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("intake_ref.id"));
    }

    #[test]
    fn test_work_kind_serialization() {
        assert_eq!(
            serde_json::to_string(&WorkKind::ResearchMemo).unwrap(),
            "\"research_memo\""
        );
        assert_eq!(
            serde_json::to_string(&WorkKind::DecisionRecord).unwrap(),
            "\"decision_record\""
        );
        assert_eq!(
            serde_json::to_string(&WorkKind::IntakeProcessing).unwrap(),
            "\"intake_processing\""
        );
    }

    #[test]
    fn test_gate_rule_serialization() {
        assert_eq!(
            serde_json::to_string(&GateRule::AllRequiredOraclesPass).unwrap(),
            "\"all_required_oracles_pass\""
        );
        assert_eq!(
            serde_json::to_string(&GateRule::PortalApprovalRequired).unwrap(),
            "\"portal_approval_required\""
        );
    }

    // ========================================================================
    // Work Surface Lifecycle Tests (SR-PLAN-V4)
    // ========================================================================

    use crate::entities::ActorKind;

    fn test_actor() -> ActorId {
        ActorId {
            kind: ActorKind::Human,
            id: "test_user".to_string(),
        }
    }

    #[test]
    fn test_work_surface_id_format() {
        let id = WorkSurfaceId::new();
        assert!(id.as_str().starts_with("ws:"));
        assert!(id.is_valid());
        assert_eq!(format!("{}", id), id.as_str());
    }

    #[test]
    fn test_work_surface_id_uniqueness() {
        let id1 = WorkSurfaceId::new();
        let id2 = WorkSurfaceId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_work_surface_status_serialization() {
        assert_eq!(
            serde_json::to_string(&WorkSurfaceStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&WorkSurfaceStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&WorkSurfaceStatus::Archived).unwrap(),
            "\"archived\""
        );
    }

    #[test]
    fn test_work_surface_status_transitions() {
        assert!(WorkSurfaceStatus::Active.is_active());
        assert!(!WorkSurfaceStatus::Completed.is_active());
        assert!(!WorkSurfaceStatus::Archived.is_active());

        assert!(!WorkSurfaceStatus::Active.is_terminal());
        assert!(WorkSurfaceStatus::Completed.is_terminal());
        assert!(WorkSurfaceStatus::Archived.is_terminal());

        assert!(WorkSurfaceStatus::Active.can_transition_stage());
        assert!(!WorkSurfaceStatus::Completed.can_transition_stage());
        assert!(!WorkSurfaceStatus::Archived.can_transition_stage());
    }

    #[test]
    fn test_stage_completion_status_serialization() {
        assert_eq!(
            serde_json::to_string(&StageCompletionStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&StageCompletionStatus::Entered).unwrap(),
            "\"entered\""
        );
        assert_eq!(
            serde_json::to_string(&StageCompletionStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&StageCompletionStatus::Skipped).unwrap(),
            "\"skipped\""
        );
    }

    #[test]
    fn test_stage_completion_status_checks() {
        assert!(!StageCompletionStatus::Pending.is_done());
        assert!(!StageCompletionStatus::Entered.is_done());
        assert!(StageCompletionStatus::Completed.is_done());
        assert!(StageCompletionStatus::Skipped.is_done());

        assert!(!StageCompletionStatus::Pending.is_current());
        assert!(StageCompletionStatus::Entered.is_current());
        assert!(!StageCompletionStatus::Completed.is_current());
    }

    #[test]
    fn test_stage_status_record_lifecycle() {
        let mut record = StageStatusRecord::pending(StageId::new("FRAME"));
        assert_eq!(record.status, StageCompletionStatus::Pending);
        assert!(record.entered_at.is_none());

        record.mark_entered();
        assert_eq!(record.status, StageCompletionStatus::Entered);
        assert!(record.entered_at.is_some());

        record.increment_iteration();
        assert_eq!(record.iteration_count, 1);

        record.mark_completed("sha256:abc123".to_string());
        assert_eq!(record.status, StageCompletionStatus::Completed);
        assert!(record.completed_at.is_some());
        assert_eq!(
            record.evidence_bundle_ref,
            Some("sha256:abc123".to_string())
        );
    }

    #[test]
    fn test_managed_work_surface_creation() {
        let ws = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![OracleSuiteBinding {
                suite_id: "suite:SR-SUITE-STRUCTURE".to_string(),
                suite_hash: ContentHash::new("sha256:suite_hash"),
            }],
            HashMap::new(),
            test_actor(),
        );

        assert!(ws.work_surface_id.is_valid());
        assert_eq!(ws.status, WorkSurfaceStatus::Active);
        assert_eq!(ws.current_stage_id.as_str(), "stage:FRAME");
        assert!(ws.content_hash.is_some());
        assert!(ws.content_hash.as_ref().unwrap().starts_with("sha256:"));

        // Initial stage should be entered
        let stage_record = ws.stage_status.get("stage:FRAME").unwrap();
        assert_eq!(stage_record.status, StageCompletionStatus::Entered);
    }

    #[test]
    fn test_managed_work_surface_stage_transitions() {
        let mut ws = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![],
            HashMap::new(),
            test_actor(),
        );

        // Complete current stage
        ws.complete_stage("sha256:evidence1".to_string()).unwrap();
        let frame_record = ws.stage_status.get("stage:FRAME").unwrap();
        assert_eq!(frame_record.status, StageCompletionStatus::Completed);

        // Enter next stage
        ws.enter_stage(StageId::new("OPTIONS"), vec![]).unwrap();
        assert_eq!(ws.current_stage_id.as_str(), "stage:OPTIONS");

        let options_record = ws.stage_status.get("stage:OPTIONS").unwrap();
        assert_eq!(options_record.status, StageCompletionStatus::Entered);
    }

    #[test]
    fn test_managed_work_surface_completion() {
        let mut ws = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FINAL"),
            vec![],
            HashMap::new(),
            test_actor(),
        );

        assert!(ws.mark_completed().is_ok());
        assert_eq!(ws.status, WorkSurfaceStatus::Completed);
        assert!(ws.completed_at.is_some());

        // Cannot complete again
        assert!(ws.mark_completed().is_err());
    }

    #[test]
    fn test_managed_work_surface_archive() {
        let mut ws = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![],
            HashMap::new(),
            test_actor(),
        );

        assert!(ws.archive(test_actor()).is_ok());
        assert_eq!(ws.status, WorkSurfaceStatus::Archived);
        assert!(ws.archived_at.is_some());
        assert!(ws.archived_by.is_some());

        // Cannot archive again
        assert!(ws.archive(test_actor()).is_err());
    }

    #[test]
    fn test_managed_work_surface_cannot_transition_when_archived() {
        let mut ws = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![],
            HashMap::new(),
            test_actor(),
        );

        ws.archive(test_actor()).unwrap();

        // Cannot enter stage when archived
        let result = ws.enter_stage(StageId::new("OPTIONS"), vec![]);
        assert!(result.is_err());

        // Cannot complete stage when archived
        let result = ws.complete_stage("sha256:evidence".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_managed_work_surface_content_hash_determinism() {
        let ws1 = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![],
            HashMap::new(),
            test_actor(),
        );

        let ws2 = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![],
            HashMap::new(),
            test_actor(),
        );

        // Content hash should be the same (based on binding, not ID or timestamps)
        assert_eq!(ws1.content_hash, ws2.content_hash);
        // But IDs should be different
        assert_ne!(ws1.work_surface_id, ws2.work_surface_id);
    }

    #[test]
    fn test_validate_work_kind_compatibility() {
        let intake_kind = WorkKind::ResearchMemo;
        let compatible_kinds = vec![WorkKind::ResearchMemo, WorkKind::DecisionRecord];
        let incompatible_kinds = vec![WorkKind::OntologyBuild];

        assert!(validate_work_kind_compatibility(&intake_kind, &compatible_kinds).is_ok());
        assert!(validate_work_kind_compatibility(&intake_kind, &incompatible_kinds).is_err());
    }

    #[test]
    fn test_managed_work_surface_to_instance() {
        let ws = ManagedWorkSurface::new_bound(
            WorkUnitId::new("test"),
            ContentAddressedRef {
                id: "intake:01ABC".to_string(),
                content_hash: ContentHash::new("sha256:intake_hash"),
            },
            ContentAddressedRef {
                id: "proc:GENERIC".to_string(),
                content_hash: ContentHash::new("sha256:template_hash"),
            },
            StageId::new("FRAME"),
            vec![OracleSuiteBinding {
                suite_id: "suite:TEST".to_string(),
                suite_hash: ContentHash::new("sha256:suite_hash"),
            }],
            HashMap::new(),
            test_actor(),
        );

        let instance = ws.to_work_surface_instance();

        assert_eq!(instance.artifact_type, "domain.work_surface");
        assert_eq!(instance.work_unit_id.as_str(), ws.work_unit_id.as_str());
        assert_eq!(instance.intake_ref.id, ws.intake_ref.id);
        assert_eq!(instance.stage_id.as_str(), ws.current_stage_id.as_str());
        assert_eq!(instance.oracle_suites.len(), 1);
    }
}
