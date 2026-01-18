//! Approval API Handlers (D-19)
//!
//! Per SR-SPEC §1.2.2 and SR-CONTRACT C-TB-3:
//! Approvals are binding human decisions at portal touchpoints.
//! Approval actor.kind MUST be HUMAN.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_domain::{
    portal::{is_seeded_portal, SEEDED_PORTALS},
    ActorKind, ApprovalId, EventEnvelope, EventId, StreamKind, TypedRef,
};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::ref_validation::normalize_and_validate_refs;
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to record an approval
#[derive(Debug, Deserialize)]
pub struct RecordApprovalRequest {
    /// Portal ID where approval is recorded
    pub portal_id: String,
    /// Decision: APPROVED, REJECTED, DEFERRED
    pub decision: String,
    /// Subject refs - what is being approved
    pub subject_refs: Vec<TypedRefRequest>,
    /// Evidence refs - content hashes of evidence bundles
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Exception IDs being acknowledged
    #[serde(default)]
    pub exceptions_acknowledged: Vec<String>,
    /// Rationale for the decision
    #[serde(default)]
    pub rationale: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// Response for a single approval
#[derive(Debug, Serialize)]
pub struct ApprovalResponse {
    pub approval_id: String,
    pub portal_id: String,
    pub decision: String,
    pub subject_refs: serde_json::Value,
    pub evidence_refs: Vec<String>,
    pub exceptions_acknowledged: Vec<String>,
    pub rationale: Option<String>,
    pub approved_by: ActorInfo,
    pub approved_at: String,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing approvals
#[derive(Debug, Deserialize)]
pub struct ListApprovalsQuery {
    #[serde(default)]
    pub portal_id: Option<String>,
    #[serde(default)]
    pub decision: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing approvals
#[derive(Debug, Serialize)]
pub struct ListApprovalsResponse {
    pub approvals: Vec<ApprovalResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for approval creation
#[derive(Debug, Serialize)]
pub struct ApprovalActionResponse {
    pub approval_id: String,
    pub portal_id: String,
    pub decision: String,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

fn validate_approval_request(
    user: &AuthenticatedUser,
    decision: &str,
    portal_id: &str,
) -> Result<(), ApiError> {
    if !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "Approvals MUST be recorded by HUMAN actors only (SR-CONTRACT C-TB-3)"
                .to_string(),
        });
    }

    let valid_decisions = ["APPROVED", "REJECTED", "DEFERRED"];
    if !valid_decisions.contains(&decision) {
        return Err(ApiError::BadRequest {
            message: format!(
                "Invalid decision '{}'. Must be one of: {:?}",
                decision, valid_decisions
            ),
        });
    }

    if !is_seeded_portal(portal_id) {
        return Err(ApiError::BadRequest {
            message: format!(
                "Invalid portal_id '{}'. Allowed portals: {:?}",
                portal_id, SEEDED_PORTALS
            ),
        });
    }

    Ok(())
}

/// Record an approval (HUMAN-only per SR-CONTRACT C-TB-3)
///
/// POST /api/v1/approvals
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn record_approval(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<RecordApprovalRequest>,
) -> ApiResult<Json<ApprovalActionResponse>> {
    validate_approval_request(&user, &body.decision, &body.portal_id)?;

    let approval_id = ApprovalId::new();
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
        "portal_id": body.portal_id,
        "decision": body.decision,
        "subject_refs": subject_refs,
        "evidence_refs": body.evidence_refs,
        "exceptions_acknowledged": body.exceptions_acknowledged,
        "rationale": body.rationale
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: approval_id.as_str().to_string(),
        stream_kind: StreamKind::Approval,
        stream_seq: 1,
        global_seq: None,
        event_type: "ApprovalRecorded".to_string(),
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
        .append(approval_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        approval_id = %approval_id.as_str(),
        portal_id = %body.portal_id,
        decision = %body.decision,
        "Approval recorded"
    );

    Ok(Json(ApprovalActionResponse {
        approval_id: approval_id.as_str().to_string(),
        portal_id: body.portal_id,
        decision: body.decision,
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get an approval by ID
///
/// GET /api/v1/approvals/{approval_id}
#[instrument(skip(state, _user))]
pub async fn get_approval(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(approval_id): Path<String>,
) -> ApiResult<Json<ApprovalResponse>> {
    let projection = state
        .projections
        .get_approval(&approval_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Approval".to_string(),
            id: approval_id.clone(),
        })?;

    Ok(Json(ApprovalResponse {
        approval_id: projection.approval_id,
        portal_id: projection.portal_id,
        decision: projection.decision,
        subject_refs: projection.subject_refs,
        evidence_refs: projection.evidence_refs,
        exceptions_acknowledged: projection.exceptions_acknowledged,
        rationale: projection.rationale,
        approved_by: ActorInfo {
            kind: projection.approved_by_kind,
            id: projection.approved_by_id,
        },
        approved_at: projection.approved_at.to_rfc3339(),
    }))
}

/// List approvals for a portal
///
/// GET /api/v1/portals/{portal_id}/approvals
#[instrument(skip(state, _user))]
pub async fn list_approvals_for_portal(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(portal_id): Path<String>,
    Query(query): Query<ListApprovalsQuery>,
) -> ApiResult<Json<ListApprovalsResponse>> {
    let approvals = state
        .projections
        .get_approvals_for_portal(&portal_id, query.limit, query.offset)
        .await?;

    let responses: Vec<ApprovalResponse> = approvals
        .into_iter()
        .map(|p| ApprovalResponse {
            approval_id: p.approval_id,
            portal_id: p.portal_id,
            decision: p.decision,
            subject_refs: p.subject_refs,
            evidence_refs: p.evidence_refs,
            exceptions_acknowledged: p.exceptions_acknowledged,
            rationale: p.rationale,
            approved_by: ActorInfo {
                kind: p.approved_by_kind,
                id: p.approved_by_id,
            },
            approved_at: p.approved_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(ListApprovalsResponse {
        total: responses.len(),
        approvals: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

/// List all approvals
///
/// GET /api/v1/approvals
#[instrument(skip(state, _user))]
pub async fn list_approvals(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListApprovalsQuery>,
) -> ApiResult<Json<ListApprovalsResponse>> {
    let approvals = state
        .projections
        .list_approvals(query.decision.as_deref(), query.limit, query.offset)
        .await?;

    let responses: Vec<ApprovalResponse> = approvals
        .into_iter()
        .map(|p| ApprovalResponse {
            approval_id: p.approval_id,
            portal_id: p.portal_id,
            decision: p.decision,
            subject_refs: p.subject_refs,
            evidence_refs: p.evidence_refs,
            exceptions_acknowledged: p.exceptions_acknowledged,
            rationale: p.rationale,
            approved_by: ActorInfo {
                kind: p.approved_by_kind,
                id: p.approved_by_id,
            },
            approved_at: p.approved_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(ListApprovalsResponse {
        total: responses.len(),
        approvals: responses,
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

    fn agent_user() -> AuthenticatedUser {
        AuthenticatedUser {
            actor_kind: ActorKind::Agent,
            actor_id: "agent".to_string(),
            subject: "sub".to_string(),
            email: None,
            name: None,
            roles: vec![],
            claims: serde_json::json!({}),
        }
    }

    #[test]
    fn rejects_non_human_actor() {
        let result = validate_approval_request(&agent_user(), "APPROVED", "ReleaseApprovalPortal");
        assert!(matches!(result, Err(ApiError::Forbidden { .. })));
    }

    #[test]
    fn rejects_invalid_decision() {
        let result = validate_approval_request(&human_user(), "NOT_A_DECISION", "ReleaseApprovalPortal");
        assert!(matches!(result, Err(ApiError::BadRequest { .. })));
    }

    #[test]
    fn rejects_unseeded_portal() {
        let result = validate_approval_request(&human_user(), "APPROVED", "portal:STAGE_COMPLETION:stage:FINAL");
        assert!(matches!(result, Err(ApiError::BadRequest { .. })));
    }

    #[test]
    fn accepts_seeded_portal() {
        let result = validate_approval_request(&human_user(), "APPROVED", "ReleaseApprovalPortal");
        assert!(result.is_ok());
    }
}
