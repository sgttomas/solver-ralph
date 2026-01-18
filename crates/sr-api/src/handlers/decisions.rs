//! Decision API Handlers (D-19)
//!
//! Per SR-SPEC §1.11 and SR-CONTRACT C-DEC-1:
//! Decisions are binding human judgments recorded at decision points.
//! Decision decided_by.kind MUST be HUMAN.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_domain::{ActorKind, DecisionId, EventEnvelope, EventId, StreamKind, TypedRef};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::ref_validation::normalize_and_validate_refs;
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to record a decision
#[derive(Debug, Deserialize)]
pub struct RecordDecisionRequest {
    /// Trigger - what caused this decision point
    pub trigger: String,
    /// Scope - context for the decision
    pub scope: serde_json::Value,
    /// The decision itself
    pub decision: String,
    /// Rationale for the decision
    pub rationale: String,
    /// Subject refs - what the decision applies to
    #[serde(default)]
    pub subject_refs: Vec<TypedRefRequest>,
    /// Evidence refs - content hashes of supporting evidence
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Exception IDs acknowledged in this decision
    #[serde(default)]
    pub exceptions_acknowledged: Vec<String>,
    /// Whether this decision sets precedent
    #[serde(default)]
    pub is_precedent: bool,
    /// Applicability clause (for precedent decisions)
    #[serde(default)]
    pub applicability: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// Response for a single decision
#[derive(Debug, Serialize)]
pub struct DecisionResponse {
    pub decision_id: String,
    pub trigger: String,
    pub scope: serde_json::Value,
    pub decision: String,
    pub rationale: String,
    pub is_precedent: bool,
    pub applicability: Option<String>,
    pub subject_refs: serde_json::Value,
    pub evidence_refs: Vec<String>,
    pub exceptions_acknowledged: serde_json::Value,
    pub decided_by: ActorInfo,
    pub decided_at: String,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing decisions
#[derive(Debug, Deserialize)]
pub struct ListDecisionsQuery {
    #[serde(default)]
    pub is_precedent: Option<bool>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing decisions
#[derive(Debug, Serialize)]
pub struct ListDecisionsResponse {
    pub decisions: Vec<DecisionResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for decision creation
#[derive(Debug, Serialize)]
pub struct DecisionActionResponse {
    pub decision_id: String,
    pub is_precedent: bool,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Record a decision (HUMAN-only per SR-CONTRACT C-DEC-1)
///
/// POST /api/v1/decisions
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn record_decision(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<RecordDecisionRequest>,
) -> ApiResult<Json<DecisionActionResponse>> {
    // Enforce HUMAN-only per SR-CONTRACT C-DEC-1
    if !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "Decisions MUST be recorded by HUMAN actors only (SR-CONTRACT C-DEC-1)"
                .to_string(),
        });
    }

    let decision_id = DecisionId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    let subject_refs_raw: Vec<TypedRef> = body
        .subject_refs
        .into_iter()
        .map(|r| TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        })
        .collect();
    let subject_refs = normalize_and_validate_refs(&state, subject_refs_raw).await?;

    let payload = serde_json::json!({
        "trigger": body.trigger,
        "scope": body.scope,
        "decision": body.decision,
        "rationale": body.rationale,
        "is_precedent": body.is_precedent,
        "applicability": body.applicability,
        "evidence_refs": body.evidence_refs,
        "exceptions_acknowledged": body.exceptions_acknowledged
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: decision_id.as_str().to_string(),
        stream_kind: StreamKind::Decision,
        stream_seq: 1,
        global_seq: None,
        event_type: "DecisionRecorded".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: subject_refs,
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(decision_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        decision_id = %decision_id.as_str(),
        is_precedent = %body.is_precedent,
        "Decision recorded"
    );

    Ok(Json(DecisionActionResponse {
        decision_id: decision_id.as_str().to_string(),
        is_precedent: body.is_precedent,
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get a decision by ID
///
/// GET /api/v1/decisions/{decision_id}
#[instrument(skip(state, _user))]
pub async fn get_decision(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(decision_id): Path<String>,
) -> ApiResult<Json<DecisionResponse>> {
    let projection = state
        .projections
        .get_decision(&decision_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Decision".to_string(),
            id: decision_id.clone(),
        })?;

    Ok(Json(DecisionResponse {
        decision_id: projection.decision_id,
        trigger: projection.trigger,
        scope: projection.scope,
        decision: projection.decision,
        rationale: projection.rationale,
        is_precedent: projection.is_precedent,
        applicability: projection.applicability,
        subject_refs: serde_json::json!([]), // Derived from refs if needed
        evidence_refs: projection.evidence_refs,
        exceptions_acknowledged: projection.exceptions_acknowledged,
        decided_by: ActorInfo {
            kind: projection.decided_by_kind,
            id: projection.decided_by_id,
        },
        decided_at: projection.decided_at.to_rfc3339(),
    }))
}

/// List decisions
///
/// GET /api/v1/decisions
#[instrument(skip(state, _user))]
pub async fn list_decisions(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListDecisionsQuery>,
) -> ApiResult<Json<ListDecisionsResponse>> {
    let decisions = state
        .projections
        .list_decisions(query.is_precedent, query.limit, query.offset)
        .await?;

    let responses: Vec<DecisionResponse> = decisions
        .into_iter()
        .map(|p| DecisionResponse {
            decision_id: p.decision_id,
            trigger: p.trigger,
            scope: p.scope,
            decision: p.decision,
            rationale: p.rationale,
            is_precedent: p.is_precedent,
            applicability: p.applicability,
            subject_refs: serde_json::json!([]),
            evidence_refs: p.evidence_refs,
            exceptions_acknowledged: p.exceptions_acknowledged,
            decided_by: ActorInfo {
                kind: p.decided_by_kind,
                id: p.decided_by_id,
            },
            decided_at: p.decided_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(ListDecisionsResponse {
        total: responses.len(),
        decisions: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
