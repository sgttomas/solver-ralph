//! Human judgment note endpoints (evaluation and assessment) per SR-SPEC §2.3.10.
//! Notes are explicitly non-binding and must not alter verification/approval state (C-TB-7).

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::HumanJudgmentRecord;
use sr_domain::{ActorKind, EventEnvelope, EventId, StreamKind, TypedRef};
use sr_ports::EventStore;
use ulid::Ulid;

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::ref_validation::normalize_and_validate_refs;
use crate::AppState;

/// Request to create an evaluation note (verification evidence review)
#[derive(Debug, Deserialize)]
pub struct CreateEvaluationNoteRequest {
    #[serde(default)]
    pub subject_refs: Vec<TypedRefRequest>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    pub content: String,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default)]
    pub recommendations: Option<String>,
}

/// Request to create an assessment note (validation judgment)
#[derive(Debug, Deserialize)]
pub struct CreateAssessmentNoteRequest {
    #[serde(default)]
    pub subject_refs: Vec<TypedRefRequest>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    pub content: String,
    #[serde(default)]
    pub fitness_judgment: Option<String>,
    #[serde(default)]
    pub context: Option<String>,
}

/// Request to create an intervention note (human intervention)
#[derive(Debug, Deserialize)]
pub struct CreateInterventionNoteRequest {
    #[serde(default)]
    pub subject_refs: Vec<TypedRefRequest>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    pub content: String,
    #[serde(default)]
    pub actions_taken: Option<String>,
    #[serde(default)]
    pub impact: Option<String>,
}

/// Typed reference request payload
#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    #[serde(default = "default_rel")]
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

fn default_rel() -> String {
    "relates_to".to_string()
}

/// Response after creating a note
#[derive(Debug, Serialize)]
pub struct RecordActionResponse {
    pub record_id: String,
    pub event_id: String,
}

/// Response for fetching a note
#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub record_id: String,
    pub record_type: String,
    pub subject_refs: serde_json::Value,
    pub evidence_refs: Vec<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fitness_judgment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<String>,
    #[serde(default)]
    pub details: serde_json::Value,
    pub recorded_by_kind: String,
    pub recorded_by_id: String,
    pub recorded_at: String,
}

/// POST /records/evaluation-notes
pub async fn create_evaluation_note(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateEvaluationNoteRequest>,
) -> ApiResult<Json<RecordActionResponse>> {
    ensure_human(&user)?;

    let refs: Vec<TypedRef> = body
        .subject_refs
        .into_iter()
        .map(|r| TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        })
        .collect();
    let refs = normalize_and_validate_refs(&state, refs).await?;

    let (record_id, event) = build_note_event(
        "record.evaluation_note",
        refs,
        body.evidence_refs,
        body.content,
        body.severity,
        None,
        body.recommendations,
        None,
        None,
        &user,
    );

    state
        .event_store
        .append(record_id.as_str(), 0, vec![event.clone()])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok(Json(RecordActionResponse {
        record_id,
        event_id: event.event_id.as_str().to_string(),
    }))
}

/// POST /records/assessment-notes
pub async fn create_assessment_note(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateAssessmentNoteRequest>,
) -> ApiResult<Json<RecordActionResponse>> {
    ensure_human(&user)?;

    let refs: Vec<TypedRef> = body
        .subject_refs
        .into_iter()
        .map(|r| TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        })
        .collect();
    let refs = normalize_and_validate_refs(&state, refs).await?;

    let (record_id, event) = build_note_event(
        "record.assessment_note",
        refs,
        body.evidence_refs,
        body.content,
        None,
        body.fitness_judgment,
        body.context,
        None,
        None,
        &user,
    );

    state
        .event_store
        .append(record_id.as_str(), 0, vec![event.clone()])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok(Json(RecordActionResponse {
        record_id,
        event_id: event.event_id.as_str().to_string(),
    }))
}

/// POST /records/intervention-notes
pub async fn create_intervention_note(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateInterventionNoteRequest>,
) -> ApiResult<Json<RecordActionResponse>> {
    ensure_human(&user)?;

    let refs: Vec<TypedRef> = body
        .subject_refs
        .into_iter()
        .map(|r| TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        })
        .collect();
    let refs = normalize_and_validate_refs(&state, refs).await?;

    let (record_id, event) = build_note_event(
        "record.intervention_note",
        refs,
        body.evidence_refs,
        body.content,
        None,
        None,
        None,
        body.actions_taken,
        body.impact,
        &user,
    );

    state
        .event_store
        .append(record_id.as_str(), 0, vec![event.clone()])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok(Json(RecordActionResponse {
        record_id,
        event_id: event.event_id.as_str().to_string(),
    }))
}

/// GET /records/{record_id}
pub async fn get_record(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(record_id): Path<String>,
) -> ApiResult<Json<RecordResponse>> {
    let record = state
        .projections
        .get_human_judgment_record(&record_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Record".to_string(),
            id: record_id.clone(),
        })?;

    Ok(Json(to_response(record)))
}

fn ensure_human(user: &AuthenticatedUser) -> ApiResult<()> {
    if !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "Human judgment notes MUST be recorded by HUMAN actors (C-TB-7)".to_string(),
        });
    }
    Ok(())
}

fn build_note_event(
    record_type: &str,
    refs: Vec<TypedRef>,
    evidence_refs: Vec<String>,
    content: String,
    severity: Option<String>,
    fitness_judgment: Option<String>,
    recommendations: Option<String>,
    actions_taken: Option<String>,
    impact: Option<String>,
    user: &AuthenticatedUser,
) -> (String, EventEnvelope) {
    let record_id = format!("rec_{}", Ulid::new());
    let event_id = EventId::new();
    let now = Utc::now();

    let payload_subject_refs = serde_json::to_value(&refs).unwrap_or_default();

    let mut details = serde_json::Map::new();
    if let Some(actions) = actions_taken {
        details.insert("actions_taken".to_string(), serde_json::json!(actions));
    }
    if let Some(impact) = impact {
        details.insert("impact".to_string(), serde_json::json!(impact));
    }

    let payload = serde_json::json!({
        "record_type": record_type,
        "subject_refs": payload_subject_refs,
        "evidence_refs": evidence_refs,
        "content": content,
        "severity": severity,
        "fitness_judgment": fitness_judgment,
        "recommendations": recommendations,
        "details": serde_json::Value::Object(details)
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: record_id.clone(),
        stream_kind: StreamKind::Governance,
        stream_seq: 1,
        global_seq: None,
        event_type: "RecordCreated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs,
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    (record_id, event)
}

fn to_response(record: HumanJudgmentRecord) -> RecordResponse {
    RecordResponse {
        record_id: record.record_id,
        record_type: record.record_type,
        subject_refs: record.subject_refs,
        evidence_refs: record.evidence_refs,
        content: record.content,
        severity: record.severity,
        fitness_judgment: record.fitness_judgment,
        recommendations: record.recommendations,
        details: record.details,
        recorded_by_kind: record.recorded_by_kind,
        recorded_by_id: record.recorded_by_id,
        recorded_at: record.recorded_at.to_rfc3339(),
    }
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthenticatedUser;

    fn human_user() -> AuthenticatedUser {
        AuthenticatedUser {
            actor_kind: ActorKind::Human,
            actor_id: "human".to_string(),
            subject: "sub".to_string(),
            email: None,
            name: None,
            roles: vec![],
            claims: serde_json::json!({}),
        }
    }

    #[test]
    fn build_eval_note_sets_record_type() {
        let (_, event) = build_note_event(
            "record.evaluation_note",
            vec![],
            vec!["sha256:abc".to_string()],
            "content".to_string(),
            Some("HIGH".to_string()),
            None,
            None,
            None,
            None,
            &human_user(),
        );

        assert_eq!(event.event_type, "RecordCreated");
        assert_eq!(event.payload["record_type"], "record.evaluation_note");
        assert_eq!(event.payload["severity"], "HIGH");
    }

    #[test]
    fn build_intervention_note_includes_details() {
        let (_, event) = build_note_event(
            "record.intervention_note",
            vec![],
            vec![],
            "content".to_string(),
            None,
            None,
            None,
            Some("ACTION".to_string()),
            Some("IMPACT".to_string()),
            &human_user(),
        );

        assert_eq!(event.payload["record_type"], "record.intervention_note");
        assert_eq!(event.payload["details"]["actions_taken"], "ACTION");
        assert_eq!(event.payload["details"]["impact"], "IMPACT");
    }

    #[test]
    fn ensure_human_rejects_agents() {
        let agent = AuthenticatedUser {
            actor_kind: ActorKind::Agent,
            actor_id: "agent".to_string(),
            subject: "sub".to_string(),
            email: None,
            name: None,
            roles: vec![],
            claims: serde_json::json!({}),
        };

        let result = ensure_human(&agent);
        assert!(result.is_err());
    }
}
