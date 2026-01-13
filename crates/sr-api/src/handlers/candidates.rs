//! Candidate API Handlers (D-18)
//!
//! Per SR-SPEC §3.3: Candidates are content-addressable snapshots of work products.
//! Registration creates a CandidateMaterialized event.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::CandidateProjection;
use sr_domain::{CandidateId, ContentHash, EventEnvelope, EventId, StreamKind, TypedRef};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to register a new candidate
#[derive(Debug, Deserialize)]
pub struct RegisterCandidateRequest {
    /// Content hash (SHA-256 hex digest)
    pub content_hash: String,
    /// Optional git commit SHA
    #[serde(default)]
    pub git_sha: Option<String>,
    /// Optional iteration that produced this candidate
    #[serde(default)]
    pub produced_by_iteration_id: Option<String>,
    /// Typed references to related artifacts
    #[serde(default)]
    pub refs: Vec<TypedRefRequest>,
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// Response for a single candidate
#[derive(Debug, Serialize)]
pub struct CandidateResponse {
    pub candidate_id: String,
    pub content_hash: String,
    pub produced_by_iteration_id: Option<String>,
    pub verification_status: String,
    pub created_at: String,
    pub refs: serde_json::Value,
}

/// Query parameters for listing candidates
#[derive(Debug, Deserialize)]
pub struct ListCandidatesQuery {
    #[serde(default)]
    pub verification_status: Option<String>,
    #[serde(default)]
    pub iteration_id: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing candidates
#[derive(Debug, Serialize)]
pub struct ListCandidatesResponse {
    pub candidates: Vec<CandidateResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for candidate registration
#[derive(Debug, Serialize)]
pub struct CandidateActionResponse {
    pub candidate_id: String,
    pub content_hash: String,
    pub verification_status: String,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Register a new candidate
///
/// POST /api/v1/candidates
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn register_candidate(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<RegisterCandidateRequest>,
) -> ApiResult<Json<CandidateActionResponse>> {
    // Validate content hash format
    if body.content_hash.len() != 64 || !body.content_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ApiError::BadRequest {
            message: "content_hash must be a 64-character hex string (SHA-256)".to_string(),
        });
    }

    // If produced_by_iteration_id is provided, verify it exists
    if let Some(ref iter_id) = body.produced_by_iteration_id {
        let _ = state
            .projections
            .get_iteration(iter_id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: "Iteration".to_string(),
                id: iter_id.clone(),
            })?;
    }

    let candidate_id = CandidateId::new(body.git_sha.as_deref(), &body.content_hash);
    let content_hash = ContentHash::new(&body.content_hash);
    let event_id = EventId::new();
    let now = Utc::now();

    let refs: Vec<TypedRef> = body
        .refs
        .into_iter()
        .map(|r| TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        })
        .collect();

    let payload = serde_json::json!({
        "content_hash": content_hash.as_str(),
        "produced_by_iteration_id": body.produced_by_iteration_id,
        "refs": refs
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: candidate_id.as_str().to_string(),
        stream_kind: StreamKind::Candidate,
        stream_seq: 1,
        global_seq: None,
        event_type: "CandidateMaterialized".to_string(),
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

    state
        .event_store
        .append(candidate_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        candidate_id = %candidate_id.as_str(),
        content_hash = %content_hash.as_str(),
        "Candidate registered"
    );

    Ok(Json(CandidateActionResponse {
        candidate_id: candidate_id.as_str().to_string(),
        content_hash: content_hash.as_str().to_string(),
        verification_status: "UNVERIFIED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get a candidate by ID
///
/// GET /api/v1/candidates/{candidate_id}
#[instrument(skip(state, _user))]
pub async fn get_candidate(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(candidate_id): Path<String>,
) -> ApiResult<Json<CandidateResponse>> {
    let projection = state
        .projections
        .get_candidate(&candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: candidate_id.clone(),
        })?;

    Ok(Json(projection_to_response(projection)))
}

/// List candidates for an iteration
///
/// GET /api/v1/iterations/{iteration_id}/candidates
#[instrument(skip(state, _user))]
pub async fn list_candidates_for_iteration(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(iteration_id): Path<String>,
    Query(_query): Query<ListCandidatesQuery>,
) -> ApiResult<Json<ListCandidatesResponse>> {
    // Verify iteration exists
    let _ = state
        .projections
        .get_iteration(&iteration_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Iteration".to_string(),
            id: iteration_id.clone(),
        })?;

    let candidates = state
        .projections
        .get_candidates_for_iteration(&iteration_id)
        .await?;

    let responses: Vec<CandidateResponse> = candidates
        .into_iter()
        .map(projection_to_response)
        .collect();

    Ok(Json(ListCandidatesResponse {
        total: responses.len(),
        candidates: responses,
        limit: 1000,
        offset: 0,
    }))
}

/// List all candidates
///
/// GET /api/v1/candidates
#[instrument(skip(state, _user))]
pub async fn list_candidates(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListCandidatesQuery>,
) -> ApiResult<Json<ListCandidatesResponse>> {
    let candidates = state
        .projections
        .list_candidates(query.verification_status.as_deref(), query.limit, query.offset)
        .await?;

    let responses: Vec<CandidateResponse> = candidates
        .into_iter()
        .map(projection_to_response)
        .collect();

    Ok(Json(ListCandidatesResponse {
        total: responses.len(),
        candidates: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn projection_to_response(p: CandidateProjection) -> CandidateResponse {
    CandidateResponse {
        candidate_id: p.candidate_id,
        content_hash: p.content_hash,
        produced_by_iteration_id: p.produced_by_iteration_id,
        verification_status: p.verification_status,
        created_at: p.created_at.to_rfc3339(),
        refs: p.refs,
    }
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_hash_validation() {
        // Valid 64-char hex
        let valid = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
        assert_eq!(valid.len(), 64);
        assert!(valid.chars().all(|c| c.is_ascii_hexdigit()));

        // Invalid - too short
        let short = "abcdef";
        assert_ne!(short.len(), 64);
    }
}
