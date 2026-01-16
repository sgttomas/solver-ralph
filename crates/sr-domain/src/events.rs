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

    // Work Surface events
    WorkSurfaceRecorded,
    ProcedureTemplateSelected,
    StageEntered,
    StageCompleted,
    SemanticOracleEvaluated,

    // Intake events per SR-PLAN-V3 §1.6
    IntakeCreated,
    IntakeUpdated,
    IntakeActivated,
    IntakeArchived,
    IntakeForked,
}
