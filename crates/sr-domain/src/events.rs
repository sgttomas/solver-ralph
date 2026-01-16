//! Domain events per SR-SPEC §1.5 and Appendix A

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::entities::{ActorKind, TypedRef};

/// Event identifier: `evt_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(String);

impl EventId {
    pub fn new() -> Self {
        Self(format!("evt_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stream kind per SR-SPEC §1.5.2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamKind {
    Loop,
    Iteration,
    Candidate,
    Run,
    Approval,
    Decision,
    Governance,
    Exception,
    OracleSuite,
    Freeze,
    /// Intake stream per SR-PLAN-V3 §1.6
    Intake,
    /// Work Surface stream per SR-PLAN-V4 §1.2
    WorkSurface,
}

/// Event envelope per SR-SPEC §1.5.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: EventId,
    pub stream_id: String,
    pub stream_kind: StreamKind,
    pub stream_seq: u64,
    pub global_seq: Option<u64>,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub actor_kind: ActorKind,
    pub actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supersedes: Vec<String>,
    #[serde(default)]
    pub refs: Vec<TypedRef>,
    pub payload: serde_json::Value,
    pub envelope_hash: String,
}

/// Event types per SR-SPEC Appendix A
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    // Loop events
    LoopCreated,
    LoopActivated,
    IterationStarted,
    IterationCompleted,
    IterationSummaryRecorded,
    StopTriggered,
    LoopPaused,
    LoopResumed,
    LoopClosed,

    // Candidate events
    CandidateMaterialized,
    CandidateVerificationComputed,

    // Run/Evidence events
    RunStarted,
    RunCompleted,
    EvidenceBundleRecorded,

    // Oracle suite events
    OracleSuiteRegistered,
    OracleSuiteUpdated,
    OracleSuitePinned,
    OracleSuiteRebased,

    // Governed artifact events
    GovernedArtifactVersionRecorded,

    // Freeze/Release events
    FreezeRecordCreated,

    // Staleness events
    NodeMarkedStale,
    ReEvaluationTriggered,
    StalenessResolved,

    // Portal/Approval events
    ApprovalRecorded,

    // Evidence integrity events
    EvidenceMissingDetected,

    // Record events
    RecordCreated,
    RecordSuperseded,

    // Exception events
    DeviationCreated,
    DeferralCreated,
    WaiverCreated,
    ExceptionActivated,
    ExceptionResolved,
    ExceptionExpired,

    // Decision events
    DecisionRecorded,

    // Work Surface events (legacy)
    WorkSurfaceRecorded,
    ProcedureTemplateSelected,
    SemanticOracleEvaluated,

    // Work Surface lifecycle events per SR-PLAN-V4 §1.2
    WorkSurfaceBound,
    StageEntered,
    StageCompleted,
    WorkSurfaceCompleted,
    WorkSurfaceArchived,

    // Intake events per SR-PLAN-V3 §1.6
    IntakeCreated,
    IntakeUpdated,
    IntakeActivated,
    IntakeArchived,
    IntakeForked,
}

// ============================================================================
// Work Surface Events per SR-PLAN-V4 §1.2
// ============================================================================

use crate::work_surface::{ContentAddressedRef, OracleSuiteBinding};

/// Emitted when a Work Surface Instance is bound
///
/// Per SR-PLAN-V4 §1.2: This event creates a commitment object binding
/// an Intake to a Procedure Template for a specific Work Unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceBound {
    /// Work Surface Instance identifier (format: ws:<ULID>)
    pub work_surface_id: String,
    /// Work Unit this Work Surface is bound to
    pub work_unit_id: String,
    /// Content-addressed reference to the Intake
    pub intake_ref: ContentAddressedRef,
    /// Content-addressed reference to the Procedure Template
    pub procedure_template_ref: ContentAddressedRef,
    /// Initial stage ID (the stage entered upon binding)
    pub initial_stage_id: String,
    /// Content hash of the binding
    pub content_hash: String,
}

impl WorkSurfaceBound {
    pub fn stream_id(&self) -> String {
        format!("work_surface:{}", self.work_surface_id)
    }

    pub fn stream_kind() -> StreamKind {
        StreamKind::WorkSurface
    }

    pub fn event_type() -> &'static str {
        "WorkSurfaceBound"
    }
}

/// Emitted when entering a new stage
///
/// Per SR-PLAN-V4 §1.2: Stage entry resolves oracle suites for that stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceStageEntered {
    /// Work Surface Instance identifier
    pub work_surface_id: String,
    /// Stage being entered
    pub stage_id: String,
    /// Previous stage (None if this is initial entry)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_stage_id: Option<String>,
    /// Oracle suites resolved for this stage
    pub oracle_suites: Vec<OracleSuiteBinding>,
}

impl WorkSurfaceStageEntered {
    pub fn stream_id(&self) -> String {
        format!("work_surface:{}", self.work_surface_id)
    }

    pub fn stream_kind() -> StreamKind {
        StreamKind::WorkSurface
    }

    pub fn event_type() -> &'static str {
        "StageEntered"
    }
}

/// Emitted when a stage is completed (gate passed)
///
/// Per SR-PLAN-V4 §1.2: Stage completion requires evidence and gate result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceStageCompleted {
    /// Work Surface Instance identifier
    pub work_surface_id: String,
    /// Stage that was completed
    pub stage_id: String,
    /// Evidence bundle hash proving gate passage
    pub evidence_bundle_ref: String,
    /// Gate result details
    pub gate_result: GateResult,
    /// Next stage ID (None if terminal stage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_stage_id: Option<String>,
}

impl WorkSurfaceStageCompleted {
    pub fn stream_id(&self) -> String {
        format!("work_surface:{}", self.work_surface_id)
    }

    pub fn stream_kind() -> StreamKind {
        StreamKind::WorkSurface
    }

    pub fn event_type() -> &'static str {
        "StageCompleted"
    }
}

/// Gate result per SR-PLAN-V4 §1.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub status: GateResultStatus,
    #[serde(default)]
    pub oracle_results: Vec<OracleResultSummary>,
    #[serde(default)]
    pub waiver_refs: Vec<String>,
}

/// Gate result status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GateResultStatus {
    Pass,
    PassWithWaivers,
    Fail,
}

/// Oracle result summary for gate evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResultSummary {
    pub oracle_id: String,
    /// PASS | FAIL | ERROR
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// Emitted when Work Surface is completed (terminal stage passed)
///
/// Per SR-PLAN-V4 §1.2: Terminal completion marks the Work Surface as done.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceCompletedEvent {
    /// Work Surface Instance identifier
    pub work_surface_id: String,
    /// The terminal stage that was completed
    pub final_stage_id: String,
    /// Evidence bundle hash for the final stage
    pub evidence_bundle_ref: String,
}

impl WorkSurfaceCompletedEvent {
    pub fn stream_id(&self) -> String {
        format!("work_surface:{}", self.work_surface_id)
    }

    pub fn stream_kind() -> StreamKind {
        StreamKind::WorkSurface
    }

    pub fn event_type() -> &'static str {
        "WorkSurfaceCompleted"
    }
}

/// Emitted when Work Surface is archived
///
/// Per SR-PLAN-V4 §1.2: Archiving marks a Work Surface as superseded or abandoned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceArchivedEvent {
    /// Work Surface Instance identifier
    pub work_surface_id: String,
    /// Reason for archiving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl WorkSurfaceArchivedEvent {
    pub fn stream_id(&self) -> String {
        format!("work_surface:{}", self.work_surface_id)
    }

    pub fn stream_kind() -> StreamKind {
        StreamKind::WorkSurface
    }

    pub fn event_type() -> &'static str {
        "WorkSurfaceArchived"
    }
}
