//! Evidence API Handlers (D-20)
//!
//! Per SR-SPEC §1.9 and SR-CONTRACT §7: Evidence bundles are content-addressed
//! artifacts containing oracle outputs. This module provides:
//! - Evidence upload with manifest validation
//! - Evidence retrieval by content hash
//! - Association with runs/candidates/iterations
//! - Listing evidence for runs and candidates

use axum::{
    extract::{Path, Query, State},
    Json,
};
use base64::Engine;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::{EvidenceArtifact, EvidenceManifest, OracleResult, OracleResultStatus};
use sr_domain::{EventEnvelope, EventId, StreamKind};
use sr_ports::{EvidenceStore, EventStore};
use tracing::{debug, info, instrument, warn};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to upload an evidence bundle
#[derive(Debug, Deserialize)]
pub struct UploadEvidenceRequest {
    /// The evidence manifest (JSON-encoded)
    pub manifest: EvidenceManifest,
    /// Optional blobs to include (base64 encoded)
    /// Key is blob name, value is base64-encoded content
    #[serde(default)]
    pub blobs: std::collections::HashMap<String, String>,
}

/// Response for evidence upload
#[derive(Debug, Serialize)]
pub struct UploadEvidenceResponse {
    pub content_hash: String,
    pub bundle_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub verdict: String,
    pub stored_at: String,
}

/// Response for retrieving evidence
#[derive(Debug, Serialize)]
pub struct EvidenceResponse {
    pub content_hash: String,
    pub manifest: EvidenceManifest,
    pub blob_names: Vec<String>,
}

/// Request to associate evidence with domain objects
#[derive(Debug, Deserialize)]
pub struct AssociateEvidenceRequest {
    /// Run to associate with (required if not already linked via manifest)
    #[serde(default)]
    pub run_id: Option<String>,
    /// Candidate to associate with
    #[serde(default)]
    pub candidate_id: Option<String>,
    /// Iteration to associate with
    #[serde(default)]
    pub iteration_id: Option<String>,
}

/// Response for evidence association
#[derive(Debug, Serialize)]
pub struct AssociateEvidenceResponse {
    pub content_hash: String,
    pub associated_with: AssociatedEntities,
    pub event_id: String,
}

#[derive(Debug, Serialize)]
pub struct AssociatedEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration_id: Option<String>,
}

/// Query parameters for listing evidence
#[derive(Debug, Deserialize)]
pub struct ListEvidenceQuery {
    #[serde(default)]
    pub verdict: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing evidence
#[derive(Debug, Serialize)]
pub struct ListEvidenceResponse {
    pub evidence: Vec<EvidenceSummary>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Summary of an evidence bundle (for list responses)
#[derive(Debug, Serialize)]
pub struct EvidenceSummary {
    pub content_hash: String,
    pub bundle_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub oracle_suite_id: String,
    pub verdict: String,
    pub run_completed_at: String,
    pub artifact_count: usize,
}

/// Response for retrieving a specific blob
#[derive(Debug, Serialize)]
pub struct BlobResponse {
    pub content_hash: String,
    pub blob_name: String,
    /// Base64-encoded blob content
    pub content: String,
    pub size: usize,
}

// ============================================================================
// Handlers
// ============================================================================

/// Upload an evidence bundle
///
/// POST /api/v1/evidence
///
/// Validates the manifest, stores the bundle in content-addressed storage,
/// and records an EvidenceBundleRecorded event.
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn upload_evidence(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<UploadEvidenceRequest>,
) -> ApiResult<Json<UploadEvidenceResponse>> {
    // Validate the manifest
    body.manifest
        .validate()
        .map_err(|e| ApiError::BadRequest {
            message: format!("Invalid evidence manifest: {}", e),
        })?;

    // Serialize manifest to JSON
    let manifest_json = body
        .manifest
        .to_deterministic_json()
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to serialize manifest: {}", e),
        })?;

    // Decode blobs from base64
    let mut blobs_decoded: Vec<(String, Vec<u8>)> = Vec::new();
    for (name, content_b64) in &body.blobs {
        let decoded = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            content_b64,
        )
        .map_err(|e| ApiError::BadRequest {
            message: format!("Invalid base64 in blob '{}': {}", name, e),
        })?;
        blobs_decoded.push((name.clone(), decoded));
    }

    // Prepare blobs for storage
    let blobs_refs: Vec<(&str, &[u8])> = blobs_decoded
        .iter()
        .map(|(n, d)| (n.as_str(), d.as_slice()))
        .collect();

    // Store in evidence store
    let content_hash = state
        .evidence_store
        .store(&manifest_json, blobs_refs)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to store evidence: {}", e),
        })?;

    // Record EvidenceBundleRecorded event
    let event_id = EventId::new();
    let now = Utc::now();

    let payload = serde_json::json!({
        "content_hash": content_hash,
        "bundle_id": body.manifest.bundle_id,
        "run_id": body.manifest.run_id,
        "candidate_id": body.manifest.candidate_id,
        "oracle_suite_id": body.manifest.oracle_suite_id,
        "oracle_suite_hash": body.manifest.oracle_suite_hash,
        "verdict": format!("{:?}", body.manifest.verdict),
        "artifact_count": body.manifest.artifacts.len()
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: body.manifest.run_id.clone(),
        stream_kind: StreamKind::Run,
        stream_seq: 0, // Will be determined by event store
        global_seq: None,
        event_type: "EvidenceBundleRecorded".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    // Get current stream version for the run
    let events = state
        .event_store
        .read_stream(&body.manifest.run_id, 0, 1000)
        .await
        .unwrap_or_default();
    let current_version = events.len() as u64;

    // Append event (use current_version for optimistic concurrency)
    let event_with_seq = EventEnvelope {
        stream_seq: current_version + 1,
        ..event
    };

    if let Err(e) = state
        .event_store
        .append(&body.manifest.run_id, current_version, vec![event_with_seq])
        .await
    {
        // Log warning but don't fail - evidence is already stored
        warn!(
            error = ?e,
            run_id = %body.manifest.run_id,
            "Failed to append EvidenceBundleRecorded event"
        );
    }

    // Process projections
    if let Err(e) = state
        .projections
        .process_events(&*state.event_store)
        .await
    {
        warn!(error = ?e, "Failed to process projections after evidence upload");
    }

    info!(
        content_hash = %content_hash,
        bundle_id = %body.manifest.bundle_id,
        run_id = %body.manifest.run_id,
        "Evidence bundle uploaded"
    );

    Ok(Json(UploadEvidenceResponse {
        content_hash,
        bundle_id: body.manifest.bundle_id,
        run_id: body.manifest.run_id,
        candidate_id: body.manifest.candidate_id,
        verdict: format!("{:?}", body.manifest.verdict),
        stored_at: now.to_rfc3339(),
    }))
}

/// Get evidence by content hash
///
/// GET /api/v1/evidence/{content_hash}
#[instrument(skip(state, _user))]
pub async fn get_evidence(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(content_hash): Path<String>,
) -> ApiResult<Json<EvidenceResponse>> {
    // Check if evidence exists
    let exists = state
        .evidence_store
        .exists(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to check evidence existence: {}", e),
        })?;

    if !exists {
        return Err(ApiError::NotFound {
            resource: "Evidence".to_string(),
            id: content_hash,
        });
    }

    // Retrieve manifest
    let manifest_bytes = state
        .evidence_store
        .retrieve(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to retrieve evidence: {}", e),
        })?;

    let manifest: EvidenceManifest =
        serde_json::from_slice(&manifest_bytes).map_err(|e| ApiError::Internal {
            message: format!("Failed to parse evidence manifest: {}", e),
        })?;

    // List blobs
    let blob_names = state
        .evidence_store
        .list_blobs(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to list blobs: {}", e),
        })?;

    Ok(Json(EvidenceResponse {
        content_hash,
        manifest,
        blob_names,
    }))
}

/// Get a specific blob from an evidence bundle
///
/// GET /api/v1/evidence/{content_hash}/blobs/{blob_name}
#[instrument(skip(state, _user))]
pub async fn get_evidence_blob(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path((content_hash, blob_name)): Path<(String, String)>,
) -> ApiResult<Json<BlobResponse>> {
    // Check if evidence exists
    let exists = state
        .evidence_store
        .exists(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to check evidence existence: {}", e),
        })?;

    if !exists {
        return Err(ApiError::NotFound {
            resource: "Evidence".to_string(),
            id: content_hash.clone(),
        });
    }

    // Retrieve blob
    let blob_data = state
        .evidence_store
        .retrieve_blob(&content_hash, &blob_name)
        .await
        .map_err(|e| ApiError::NotFound {
            resource: "Blob".to_string(),
            id: format!("{}:{}", content_hash, blob_name),
        })?;

    // Encode as base64
    let content = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &blob_data);

    Ok(Json(BlobResponse {
        content_hash,
        blob_name,
        content,
        size: blob_data.len(),
    }))
}

/// Associate evidence with domain objects
///
/// POST /api/v1/evidence/{content_hash}/associate
///
/// Creates association records linking evidence to runs, candidates, or iterations.
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn associate_evidence(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(content_hash): Path<String>,
    Json(body): Json<AssociateEvidenceRequest>,
) -> ApiResult<Json<AssociateEvidenceResponse>> {
    // Verify evidence exists
    let exists = state
        .evidence_store
        .exists(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to check evidence existence: {}", e),
        })?;

    if !exists {
        return Err(ApiError::NotFound {
            resource: "Evidence".to_string(),
            id: content_hash.clone(),
        });
    }

    // At least one association must be specified
    if body.run_id.is_none() && body.candidate_id.is_none() && body.iteration_id.is_none() {
        return Err(ApiError::BadRequest {
            message: "At least one of run_id, candidate_id, or iteration_id must be specified"
                .to_string(),
        });
    }

    // Verify referenced entities exist
    if let Some(ref run_id) = body.run_id {
        let _ = state
            .projections
            .get_run(run_id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: "Run".to_string(),
                id: run_id.clone(),
            })?;
    }

    if let Some(ref candidate_id) = body.candidate_id {
        let _ = state
            .projections
            .get_candidate(candidate_id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: "Candidate".to_string(),
                id: candidate_id.clone(),
            })?;
    }

    if let Some(ref iteration_id) = body.iteration_id {
        let _ = state
            .projections
            .get_iteration(iteration_id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: "Iteration".to_string(),
                id: iteration_id.clone(),
            })?;
    }

    // Record association event
    let event_id = EventId::new();
    let now = Utc::now();

    // Determine stream to record event in (prefer run > candidate > iteration)
    let (stream_id, stream_kind) = if let Some(ref run_id) = body.run_id {
        (run_id.clone(), StreamKind::Run)
    } else if let Some(ref candidate_id) = body.candidate_id {
        (candidate_id.clone(), StreamKind::Candidate)
    } else {
        (body.iteration_id.clone().unwrap(), StreamKind::Iteration)
    };

    let payload = serde_json::json!({
        "content_hash": content_hash,
        "run_id": body.run_id,
        "candidate_id": body.candidate_id,
        "iteration_id": body.iteration_id
    });

    // Get current stream version
    let events = state
        .event_store
        .read_stream(&stream_id, 0, 1000)
        .await
        .unwrap_or_default();
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: stream_id.clone(),
        stream_kind,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "EvidenceAssociated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(&stream_id, current_version, vec![event])
        .await?;

    // Process projections
    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        content_hash = %content_hash,
        run_id = ?body.run_id,
        candidate_id = ?body.candidate_id,
        iteration_id = ?body.iteration_id,
        "Evidence associated"
    );

    Ok(Json(AssociateEvidenceResponse {
        content_hash,
        associated_with: AssociatedEntities {
            run_id: body.run_id,
            candidate_id: body.candidate_id,
            iteration_id: body.iteration_id,
        },
        event_id: event_id.as_str().to_string(),
    }))
}

/// List evidence for a run
///
/// GET /api/v1/runs/{run_id}/evidence
#[instrument(skip(state, _user))]
pub async fn list_evidence_for_run(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(run_id): Path<String>,
    Query(_query): Query<ListEvidenceQuery>,
) -> ApiResult<Json<ListEvidenceResponse>> {
    // Verify run exists
    let run = state
        .projections
        .get_run(&run_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Run".to_string(),
            id: run_id.clone(),
        })?;

    // Get evidence from the run's evidence_bundle_hash
    let mut evidence = Vec::new();

    if let Some(ref evidence_hash) = run.evidence_bundle_hash {
        if let Ok(manifest_bytes) = state.evidence_store.retrieve(evidence_hash).await {
            if let Ok(manifest) = serde_json::from_slice::<EvidenceManifest>(&manifest_bytes) {
                evidence.push(EvidenceSummary {
                    content_hash: evidence_hash.clone(),
                    bundle_id: manifest.bundle_id,
                    run_id: manifest.run_id,
                    candidate_id: manifest.candidate_id,
                    oracle_suite_id: manifest.oracle_suite_id,
                    verdict: format!("{:?}", manifest.verdict),
                    run_completed_at: manifest.run_completed_at.to_rfc3339(),
                    artifact_count: manifest.artifacts.len(),
                });
            }
        }
    }

    // Also check projection for additional evidence associations
    let evidence_bundles = state
        .projections
        .get_evidence_for_run(&run_id)
        .await
        .unwrap_or_default();

    for evidence_hash in evidence_bundles {
        // Skip if we already have this one
        if evidence.iter().any(|e| e.content_hash == evidence_hash) {
            continue;
        }

        if let Ok(manifest_bytes) = state.evidence_store.retrieve(&evidence_hash).await {
            if let Ok(manifest) = serde_json::from_slice::<EvidenceManifest>(&manifest_bytes) {
                evidence.push(EvidenceSummary {
                    content_hash: evidence_hash,
                    bundle_id: manifest.bundle_id,
                    run_id: manifest.run_id,
                    candidate_id: manifest.candidate_id,
                    oracle_suite_id: manifest.oracle_suite_id,
                    verdict: format!("{:?}", manifest.verdict),
                    run_completed_at: manifest.run_completed_at.to_rfc3339(),
                    artifact_count: manifest.artifacts.len(),
                });
            }
        }
    }

    Ok(Json(ListEvidenceResponse {
        total: evidence.len(),
        evidence,
        limit: 1000,
        offset: 0,
    }))
}

/// List evidence for a candidate
///
/// GET /api/v1/candidates/{candidate_id}/evidence
#[instrument(skip(state, _user))]
pub async fn list_evidence_for_candidate(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(candidate_id): Path<String>,
    Query(_query): Query<ListEvidenceQuery>,
) -> ApiResult<Json<ListEvidenceResponse>> {
    // Verify candidate exists
    let _ = state
        .projections
        .get_candidate(&candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: candidate_id.clone(),
        })?;

    // Get all runs for this candidate and collect their evidence
    let runs = state
        .projections
        .get_runs_for_candidate(&candidate_id)
        .await?;

    let mut evidence = Vec::new();
    let mut seen_hashes = std::collections::HashSet::new();

    for run in runs {
        if let Some(ref evidence_hash) = run.evidence_bundle_hash {
            if seen_hashes.insert(evidence_hash.clone()) {
                if let Ok(manifest_bytes) = state.evidence_store.retrieve(evidence_hash).await {
                    if let Ok(manifest) = serde_json::from_slice::<EvidenceManifest>(&manifest_bytes)
                    {
                        evidence.push(EvidenceSummary {
                            content_hash: evidence_hash.clone(),
                            bundle_id: manifest.bundle_id,
                            run_id: manifest.run_id,
                            candidate_id: manifest.candidate_id,
                            oracle_suite_id: manifest.oracle_suite_id,
                            verdict: format!("{:?}", manifest.verdict),
                            run_completed_at: manifest.run_completed_at.to_rfc3339(),
                            artifact_count: manifest.artifacts.len(),
                        });
                    }
                }
            }
        }
    }

    // Also check projection for direct associations
    let evidence_bundles = state
        .projections
        .get_evidence_for_candidate(&candidate_id)
        .await
        .unwrap_or_default();

    for evidence_hash in evidence_bundles {
        if seen_hashes.insert(evidence_hash.clone()) {
            if let Ok(manifest_bytes) = state.evidence_store.retrieve(&evidence_hash).await {
                if let Ok(manifest) = serde_json::from_slice::<EvidenceManifest>(&manifest_bytes) {
                    evidence.push(EvidenceSummary {
                        content_hash: evidence_hash,
                        bundle_id: manifest.bundle_id,
                        run_id: manifest.run_id,
                        candidate_id: manifest.candidate_id,
                        oracle_suite_id: manifest.oracle_suite_id,
                        verdict: format!("{:?}", manifest.verdict),
                        run_completed_at: manifest.run_completed_at.to_rfc3339(),
                        artifact_count: manifest.artifacts.len(),
                    });
                }
            }
        }
    }

    Ok(Json(ListEvidenceResponse {
        total: evidence.len(),
        evidence,
        limit: 1000,
        offset: 0,
    }))
}

/// List all evidence
///
/// GET /api/v1/evidence
#[instrument(skip(state, _user))]
pub async fn list_evidence(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListEvidenceQuery>,
) -> ApiResult<Json<ListEvidenceResponse>> {
    // Get all evidence from projections
    let evidence_bundles = state
        .projections
        .list_evidence(query.verdict.as_deref(), query.limit, query.offset)
        .await
        .unwrap_or_default();

    let mut evidence = Vec::new();

    for evidence_record in evidence_bundles {
        if let Ok(manifest_bytes) = state
            .evidence_store
            .retrieve(&evidence_record.content_hash)
            .await
        {
            if let Ok(manifest) = serde_json::from_slice::<EvidenceManifest>(&manifest_bytes) {
                evidence.push(EvidenceSummary {
                    content_hash: evidence_record.content_hash,
                    bundle_id: manifest.bundle_id,
                    run_id: manifest.run_id,
                    candidate_id: manifest.candidate_id,
                    oracle_suite_id: manifest.oracle_suite_id,
                    verdict: format!("{:?}", manifest.verdict),
                    run_completed_at: manifest.run_completed_at.to_rfc3339(),
                    artifact_count: manifest.artifacts.len(),
                });
            }
        }
    }

    Ok(Json(ListEvidenceResponse {
        total: evidence.len(),
        evidence,
        limit: query.limit,
        offset: query.offset,
    }))
}

/// Verify evidence integrity
///
/// POST /api/v1/evidence/{content_hash}/verify
#[instrument(skip(state, _user))]
pub async fn verify_evidence(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(content_hash): Path<String>,
) -> ApiResult<Json<VerifyEvidenceResponse>> {
    // Check if evidence exists
    let exists = state
        .evidence_store
        .exists(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to check evidence existence: {}", e),
        })?;

    if !exists {
        return Err(ApiError::NotFound {
            resource: "Evidence".to_string(),
            id: content_hash,
        });
    }

    // Verify integrity
    let valid = state
        .evidence_store
        .verify_integrity(&content_hash)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to verify evidence integrity: {}", e),
        })?;

    Ok(Json(VerifyEvidenceResponse {
        content_hash,
        valid,
        verified_at: Utc::now().to_rfc3339(),
    }))
}

/// Response for evidence verification
#[derive(Debug, Serialize)]
pub struct VerifyEvidenceResponse {
    pub content_hash: String,
    pub valid: bool,
    pub verified_at: String,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
