//! Record type definitions per SR-TYPES §4.3/§4.4
//!
//! This module provides lightweight structs for runtime record artifacts that
//! were missing from the domain model (P2 Ontological completeness):
//! - LoopRecord (`domain.loop_record`) — iteration summary as a commitment object
//! - ProcedureInstance (`record.procedure_instance`) — Work Surface stage tracking
//! - Evaluation/Assessment/Intervention notes (`record.*_note`) — non-binding human judgment

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::commands::IterationSummary;
use crate::entities::{ActorId, TypedRef};
use crate::work_surface::{StageStatusRecord, WorkSurfaceId};

/// Type key constants for human judgment notes
pub const EVALUATION_NOTE_TYPE_KEY: &str = "record.evaluation_note";
pub const ASSESSMENT_NOTE_TYPE_KEY: &str = "record.assessment_note";
pub const INTERVENTION_NOTE_TYPE_KEY: &str = "record.intervention_note";

/// LoopRecord (domain.loop_record) — alias for iteration summary commitment objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopRecord {
    /// Canonical type key for loop records
    pub type_key: String,
    /// Iteration summary payload
    pub summary: IterationSummary,
}

impl LoopRecord {
    pub fn new(summary: IterationSummary) -> Self {
        Self {
            type_key: "domain.loop_record".to_string(),
            summary,
        }
    }
}

/// ProcedureInstance record (record.procedure_instance)
///
/// Represents the binding of a Work Surface (Intake + Procedure Template) with
/// current stage status and oracle bindings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureInstance {
    pub type_key: String,
    pub work_surface_id: WorkSurfaceId,
    pub work_unit_id: String,
    pub procedure_template_id: String,
    pub current_stage_id: String,
    pub stage_status: HashMap<String, StageStatusRecord>,
    pub current_oracle_suites: Vec<serde_json::Value>,
    pub params: serde_json::Value,
    pub content_hash: Option<String>,
}

impl ProcedureInstance {
    pub fn new(
        work_surface_id: WorkSurfaceId,
        work_unit_id: String,
        procedure_template_id: String,
        current_stage_id: String,
        stage_status: HashMap<String, StageStatusRecord>,
        current_oracle_suites: Vec<serde_json::Value>,
        params: serde_json::Value,
        content_hash: Option<String>,
    ) -> Self {
        Self {
            type_key: "record.procedure_instance".to_string(),
            work_surface_id,
            work_unit_id,
            procedure_template_id,
            current_stage_id,
            stage_status,
            current_oracle_suites,
            params,
            content_hash,
        }
    }
}

/// Evaluation note (record.evaluation_note) — human evaluation of verification evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationNote {
    pub record_id: String,
    pub subject_refs: Vec<TypedRef>,
    pub evidence_refs: Vec<String>,
    pub content: String,
    pub severity: Option<String>,
    pub recommendations: Option<String>,
    pub recorded_at: DateTime<Utc>,
    pub recorded_by: ActorId,
}

impl EvaluationNote {
    pub fn type_key() -> &'static str {
        EVALUATION_NOTE_TYPE_KEY
    }
}

/// Assessment note (record.assessment_note) — human assessment of validation evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentNote {
    pub record_id: String,
    pub subject_refs: Vec<TypedRef>,
    pub evidence_refs: Vec<String>,
    pub content: String,
    pub fitness_judgment: Option<String>,
    pub context: Option<String>,
    pub recorded_at: DateTime<Utc>,
    pub recorded_by: ActorId,
}

impl AssessmentNote {
    pub fn type_key() -> &'static str {
        ASSESSMENT_NOTE_TYPE_KEY
    }
}

/// Intervention note (record.intervention_note) — human intervention record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionNote {
    pub record_id: String,
    pub subject_refs: Vec<TypedRef>,
    pub evidence_refs: Vec<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_taken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    pub recorded_at: DateTime<Utc>,
    pub recorded_by: ActorId,
}

impl InterventionNote {
    pub fn type_key() -> &'static str {
        INTERVENTION_NOTE_TYPE_KEY
    }
}
