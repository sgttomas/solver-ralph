//! Freeze Record API Handlers (D-19)
//!
//! Per SR-SPEC §1.12 and SR-CONTRACT C-SHIP-1:
//! Freeze records are binding baseline snapshots that establish shippability.
//! Freeze frozen_by.kind MUST be HUMAN.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_domain::{ActorKind, EventEnvelope, EventId, FreezeId, StreamKind, TypedRef};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a freeze record
#[derive(Debug, Deserialize)]
pub struct CreateFreezeRequest {
    /// Baseline identifier (e.g., release version)
    pub baseline_id: String,
    /// Candidate being frozen
    pub candidate_id: String,
    /// Verification mode: STRICT or WITH_EXCEPTIONS
    pub verification_mode: String,
    /// Oracle suite used for verification
    pub oracle_suite_id: String,
    /// Oracle suite content hash
    pub oracle_suite_hash: String,
    /// Evidence bundle refs (content hashes)
    #[serde(default)]
    pub evidence_bundle_refs: Vec<String>,
    /// Waiver exception IDs
    #[serde(default)]
    pub waiver_refs: Vec<String>,
    /// Release approval ID (required)
    pub release_approval_id: String,
    /// Artifact manifest entries
    #[serde(default)]
    pub artifact_manifest: Vec<ArtifactManifestEntryRequest>,
    /// Active exceptions at freeze time
    #[serde(default)]
    pub active_exceptions: Vec<ActiveExceptionEntryRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtifactManifestEntryRequest {
    pub artifact_id: String,
    pub version: String,
    pub content_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActiveExceptionEntryRequest {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
}

/// Response for a single freeze record
#[derive(Debug, Serialize)]
pub struct FreezeRecordResponse {
    pub freeze_id: String,
    pub baseline_id: String,
    pub candidate_id: String,
    pub verification_mode: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub evidence_bundle_refs: Vec<String>,
    pub waiver_refs: Vec<String>,
    pub release_approval_id: String,
    pub artifact_manifest: serde_json::Value,
    pub active_exceptions: serde_json::Value,
    pub frozen_by: ActorInfo,
    pub frozen_at: String,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing freeze records
#[derive(Debug, Deserialize)]
pub struct ListFreezeRecordsQuery {
    #[serde(default)]
    pub candidate_id: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing freeze records
#[derive(Debug, Serialize)]
pub struct ListFreezeRecordsResponse {
    pub freeze_records: Vec<FreezeRecordResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for freeze record creation
#[derive(Debug, Serialize)]
pub struct FreezeActionResponse {
    pub freeze_id: String,
    pub baseline_id: String,
    pub candidate_id: String,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Create a freeze record (HUMAN-only per SR-CONTRACT C-SHIP-1)
///
/// POST /api/v1/freeze-records
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn create_freeze_record(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateFreezeRequest>,
) -> ApiResult<Json<FreezeActionResponse>> {
    // Enforce HUMAN-only per SR-CONTRACT C-SHIP-1
    if !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "Freeze records MUST be created by HUMAN actors only (SR-CONTRACT C-SHIP-1)"
                .to_string(),
        });
    }

    // Validate verification mode
    let valid_modes = ["STRICT", "WITH_EXCEPTIONS"];
    if !valid_modes.contains(&body.verification_mode.as_str()) {
        return Err(ApiError::BadRequest {
            message: format!(
                "Invalid verification_mode '{}'. Must be one of: {:?}",
                body.verification_mode, valid_modes
            ),
        });
    }

    // Verify candidate exists
    let _ = state
        .projections
        .get_candidate(&body.candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: body.candidate_id.clone(),
        })?;

    // Verify approval exists
    let _ = state
        .projections
        .get_approval(&body.release_approval_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Approval".to_string(),
            id: body.release_approval_id.clone(),
        })?;

    let freeze_id = FreezeId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    let artifact_manifest = serde_json::to_value(&body.artifact_manifest).unwrap_or_default();
    let active_exceptions = serde_json::to_value(&body.active_exceptions).unwrap_or_default();

    let payload = serde_json::json!({
        "baseline_id": body.baseline_id,
        "candidate_id": body.candidate_id,
        "verification_mode": body.verification_mode,
        "oracle_suite_id": body.oracle_suite_id,
        "oracle_suite_hash": body.oracle_suite_hash,
        "evidence_bundle_refs": body.evidence_bundle_refs,
        "waiver_refs": body.waiver_refs,
        "release_approval_id": body.release_approval_id,
        "artifact_manifest": artifact_manifest,
        "active_exceptions": active_exceptions
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: freeze_id.as_str().to_string(),
        stream_kind: StreamKind::Freeze,
        stream_seq: 1,
        global_seq: None,
        event_type: "FreezeRecordCreated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![TypedRef {
            kind: "Candidate".to_string(),
            id: body.candidate_id.clone(),
            rel: "freezes".to_string(),
            meta: serde_json::Value::Null,
        }],
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(freeze_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        freeze_id = %freeze_id.as_str(),
        baseline_id = %body.baseline_id,
        candidate_id = %body.candidate_id,
        "Freeze record created"
    );

    Ok(Json(FreezeActionResponse {
        freeze_id: freeze_id.as_str().to_string(),
        baseline_id: body.baseline_id,
        candidate_id: body.candidate_id,
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get a freeze record by ID
///
/// GET /api/v1/freeze-records/{freeze_id}
#[instrument(skip(state, _user))]
pub async fn get_freeze_record(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(freeze_id): Path<String>,
) -> ApiResult<Json<FreezeRecordResponse>> {
    let projection = state
        .projections
        .get_freeze_record(&freeze_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "FreezeRecord".to_string(),
            id: freeze_id.clone(),
        })?;

    Ok(Json(FreezeRecordResponse {
        freeze_id: projection.freeze_id,
        baseline_id: projection.baseline_id,
        candidate_id: projection.candidate_id,
        verification_mode: projection.verification_mode,
        oracle_suite_id: projection.oracle_suite_id,
        oracle_suite_hash: projection.oracle_suite_hash,
        evidence_bundle_refs: projection.evidence_bundle_refs,
        waiver_refs: projection.waiver_refs,
        release_approval_id: projection.release_approval_id,
        artifact_manifest: projection.artifact_manifest,
        active_exceptions: projection.active_exceptions,
        frozen_by: ActorInfo {
            kind: projection.frozen_by_kind,
            id: projection.frozen_by_id,
        },
        frozen_at: projection.frozen_at.to_rfc3339(),
    }))
}

/// List freeze records for a candidate
///
/// GET /api/v1/candidates/{candidate_id}/freeze-records
#[instrument(skip(state, _user))]
pub async fn list_freeze_records_for_candidate(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(candidate_id): Path<String>,
    Query(query): Query<ListFreezeRecordsQuery>,
) -> ApiResult<Json<ListFreezeRecordsResponse>> {
    // Verify candidate exists
    let _ = state
        .projections
        .get_candidate(&candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: candidate_id.clone(),
        })?;

    let records = state
        .projections
        .get_freeze_records_for_candidate(&candidate_id, query.limit, query.offset)
        .await?;

    let responses: Vec<FreezeRecordResponse> = records
        .into_iter()
        .map(|p| FreezeRecordResponse {
            freeze_id: p.freeze_id,
            baseline_id: p.baseline_id,
            candidate_id: p.candidate_id,
            verification_mode: p.verification_mode,
            oracle_suite_id: p.oracle_suite_id,
            oracle_suite_hash: p.oracle_suite_hash,
            evidence_bundle_refs: p.evidence_bundle_refs,
            waiver_refs: p.waiver_refs,
            release_approval_id: p.release_approval_id,
            artifact_manifest: p.artifact_manifest,
            active_exceptions: p.active_exceptions,
            frozen_by: ActorInfo {
                kind: p.frozen_by_kind,
                id: p.frozen_by_id,
            },
            frozen_at: p.frozen_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(ListFreezeRecordsResponse {
        total: responses.len(),
        freeze_records: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

/// List all freeze records
///
/// GET /api/v1/freeze-records
#[instrument(skip(state, _user))]
pub async fn list_freeze_records(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListFreezeRecordsQuery>,
) -> ApiResult<Json<ListFreezeRecordsResponse>> {
    let records = state
        .projections
        .list_freeze_records(query.limit, query.offset)
        .await?;

    let responses: Vec<FreezeRecordResponse> = records
        .into_iter()
        .map(|p| FreezeRecordResponse {
            freeze_id: p.freeze_id,
            baseline_id: p.baseline_id,
            candidate_id: p.candidate_id,
            verification_mode: p.verification_mode,
            oracle_suite_id: p.oracle_suite_id,
            oracle_suite_hash: p.oracle_suite_hash,
            evidence_bundle_refs: p.evidence_bundle_refs,
            waiver_refs: p.waiver_refs,
            release_approval_id: p.release_approval_id,
            artifact_manifest: p.artifact_manifest,
            active_exceptions: p.active_exceptions,
            frozen_by: ActorInfo {
                kind: p.frozen_by_kind,
                id: p.frozen_by_id,
            },
            frozen_at: p.frozen_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(ListFreezeRecordsResponse {
        total: responses.len(),
        freeze_records: responses,
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
