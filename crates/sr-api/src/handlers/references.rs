//! References API Handlers per SR-PLAN-V3 Phase 0c
//!
//! Per SR-SPEC §3.2.1.1: Endpoints for browsing all typed references in the system.
//! Provides a unified view across 12 reference categories with standardized response format.
//!
//! Endpoints:
//! - GET  /api/v1/references                      - List all refs (paginated, filterable)
//! - GET  /api/v1/references/governed-artifacts   - List governed artifacts
//! - GET  /api/v1/references/governed-artifacts/:id - Get governed artifact detail
//! - GET  /api/v1/references/candidates           - List candidates
//! - GET  /api/v1/references/evidence-bundles     - List evidence bundles
//! - GET  /api/v1/references/evidence-bundles/:hash - Get evidence bundle detail
//! - GET  /api/v1/references/oracle-suites        - List oracle suites
//! - GET  /api/v1/references/procedure-templates  - List procedure templates
//! - GET  /api/v1/references/exceptions           - List active exceptions
//! - GET  /api/v1/references/iteration-summaries  - List iteration summaries
//! - GET  /api/v1/references/agent-definitions    - List agent definitions (stub)
//! - GET  /api/v1/references/gating-policies      - List gating policies
//! - GET  /api/v1/references/intakes              - List intakes
//! - POST /api/v1/references/documents            - Upload document (stub)
//! - GET  /api/v1/references/documents/:id        - Get document (stub)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sr_adapters::oracle_suite::OracleSuiteRegistry;
use std::sync::Arc;
use tracing::instrument;

use crate::auth::AuthenticatedUser;
use crate::handlers::templates::TemplateRegistry;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// State
// ============================================================================

/// Combined state for references endpoints
#[derive(Clone)]
pub struct ReferencesState {
    pub app_state: AppState,
    pub oracle_registry: Arc<OracleSuiteRegistry>,
    pub template_registry: Arc<TemplateRegistry>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Query parameters for listing references
#[derive(Debug, Deserialize)]
pub struct ListRefsParams {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub rel: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

/// Standardized response for listing references per SR-PLAN-V3 §2.2
#[derive(Debug, Serialize)]
pub struct ReferencesListResponse {
    pub refs: Vec<TypedRefResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}

/// TypedRef response per SR-SPEC §1.5.3
#[derive(Debug, Clone, Serialize)]
pub struct TypedRefResponse {
    pub kind: String,
    pub id: String,
    pub rel: String,
    pub meta: RefMetaResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Reference metadata per SR-SPEC §1.5.3.1
#[derive(Debug, Clone, Default, Serialize)]
pub struct RefMetaResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage_id: Option<String>,
}

/// Governed artifact detail response
#[derive(Debug, Serialize)]
pub struct GovernedArtifactDetailResponse {
    pub artifact_id: String,
    pub artifact_type: String,
    pub version: String,
    pub content_hash: String,
    pub status: String,
    pub normative_status: String,
    pub authority_kind: String,
    pub governed_by: Vec<String>,
    pub tags: Vec<String>,
    pub supersedes: Vec<String>,
    pub is_current: bool,
    pub recorded_at: String,
    pub recorded_by: ActorInfo,
}

/// Evidence bundle detail response
#[derive(Debug, Serialize)]
pub struct EvidenceBundleDetailResponse {
    pub content_hash: String,
    pub bundle_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub iteration_id: Option<String>,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub verdict: String,
    pub artifact_count: i32,
    pub run_completed_at: String,
    pub recorded_at: String,
    pub recorded_by: ActorInfo,
}

/// Actor info
#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// List all references (aggregated from all sources)
///
/// GET /api/v1/references
#[instrument(skip(state, _user))]
pub async fn list_references(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    // For the aggregated view, we'll collect from all sources
    // This is a simplified implementation that returns a subset
    // A full implementation would query all sources and merge

    let mut all_refs: Vec<TypedRefResponse> = Vec::new();

    // Get governed artifacts
    let governed = get_governed_artifacts_refs(&state.app_state, None).await?;
    all_refs.extend(governed);

    // Get candidates
    let candidates = get_candidates_refs(&state.app_state, None).await?;
    all_refs.extend(candidates);

    // Get exceptions
    let exceptions = get_exceptions_refs(&state.app_state, None).await?;
    all_refs.extend(exceptions);

    // Get intakes
    let intakes = get_intakes_refs(&state.app_state, None).await?;
    all_refs.extend(intakes);

    // Filter by kind if specified
    if let Some(ref kind) = params.kind {
        all_refs.retain(|r| r.kind.to_lowercase() == kind.to_lowercase());
    }

    // Filter by rel if specified
    if let Some(ref rel) = params.rel {
        all_refs.retain(|r| r.rel.to_lowercase() == rel.to_lowercase());
    }

    let total = all_refs.len() as i64;

    // Paginate
    let offset = ((params.page.saturating_sub(1)) * params.page_size) as usize;
    let refs: Vec<TypedRefResponse> = all_refs
        .into_iter()
        .skip(offset)
        .take(params.page_size as usize)
        .collect();

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List governed artifacts
///
/// GET /api/v1/references/governed-artifacts
#[instrument(skip(state, _user))]
pub async fn list_governed_artifacts(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let offset = (params.page.saturating_sub(1)) * params.page_size;

    // Count total
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM proj.governed_artifacts")
        .fetch_one(state.app_state.projections.pool())
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let refs =
        get_governed_artifacts_refs(&state.app_state, Some((params.page_size, offset))).await?;

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// Get governed artifact by ID
///
/// GET /api/v1/references/governed-artifacts/:id
#[instrument(skip(state, _user))]
pub async fn get_governed_artifact(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Path(artifact_id): Path<String>,
) -> ApiResult<Json<GovernedArtifactDetailResponse>> {
    let row = sqlx::query(
        r#"
        SELECT artifact_id, artifact_type, version, content_hash, status, normative_status,
               authority_kind, governed_by, tags, supersedes, is_current,
               recorded_at, recorded_by_kind, recorded_by_id
        FROM proj.governed_artifacts
        WHERE artifact_id = $1 AND is_current = TRUE
        "#,
    )
    .bind(&artifact_id)
    .fetch_optional(state.app_state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?
    .ok_or_else(|| ApiError::NotFound {
        resource: "GovernedArtifact".to_string(),
        id: artifact_id.clone(),
    })?;

    use sqlx::Row;

    Ok(Json(GovernedArtifactDetailResponse {
        artifact_id: row.get("artifact_id"),
        artifact_type: row.get("artifact_type"),
        version: row.get("version"),
        content_hash: row.get("content_hash"),
        status: row.get("status"),
        normative_status: row.get("normative_status"),
        authority_kind: row.get("authority_kind"),
        governed_by: row.get("governed_by"),
        tags: row.get("tags"),
        supersedes: row.get("supersedes"),
        is_current: row.get("is_current"),
        recorded_at: row.get::<DateTime<Utc>, _>("recorded_at").to_rfc3339(),
        recorded_by: ActorInfo {
            kind: row.get("recorded_by_kind"),
            id: row.get("recorded_by_id"),
        },
    }))
}

/// List candidates
///
/// GET /api/v1/references/candidates
#[instrument(skip(state, _user))]
pub async fn list_candidates(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let offset = (params.page.saturating_sub(1)) * params.page_size;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM proj.candidates")
        .fetch_one(state.app_state.projections.pool())
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let refs = get_candidates_refs(&state.app_state, Some((params.page_size, offset))).await?;

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List evidence bundles
///
/// GET /api/v1/references/evidence-bundles
#[instrument(skip(state, _user))]
pub async fn list_evidence_bundles(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let offset = (params.page.saturating_sub(1)) * params.page_size;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM proj.evidence_bundles")
        .fetch_one(state.app_state.projections.pool())
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let rows = sqlx::query(
        r#"
        SELECT content_hash, bundle_id, run_id, candidate_id, oracle_suite_id, verdict
        FROM proj.evidence_bundles
        ORDER BY recorded_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.page_size as i64)
    .bind(offset as i64)
    .fetch_all(state.app_state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    use sqlx::Row;

    let refs: Vec<TypedRefResponse> = rows
        .iter()
        .map(|row| {
            let content_hash: String = row.get("content_hash");
            let bundle_id: String = row.get("bundle_id");
            let verdict: String = row.get("verdict");

            TypedRefResponse {
                kind: "EvidenceBundle".to_string(),
                id: bundle_id.clone(),
                rel: "supported_by".to_string(),
                meta: RefMetaResponse {
                    content_hash: Some(content_hash),
                    type_key: Some("domain.evidence_bundle".to_string()),
                    ..Default::default()
                },
                label: Some(format!("{} ({})", bundle_id, verdict)),
            }
        })
        .collect();

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// Get evidence bundle by content hash
///
/// GET /api/v1/references/evidence-bundles/:hash
#[instrument(skip(state, _user))]
pub async fn get_evidence_bundle(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Path(content_hash): Path<String>,
) -> ApiResult<Json<EvidenceBundleDetailResponse>> {
    let row = sqlx::query(
        r#"
        SELECT content_hash, bundle_id, run_id, candidate_id, iteration_id,
               oracle_suite_id, oracle_suite_hash, verdict, artifact_count,
               run_completed_at, recorded_at, recorded_by_kind, recorded_by_id
        FROM proj.evidence_bundles
        WHERE content_hash = $1
        "#,
    )
    .bind(&content_hash)
    .fetch_optional(state.app_state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?
    .ok_or_else(|| ApiError::NotFound {
        resource: "EvidenceBundle".to_string(),
        id: content_hash.clone(),
    })?;

    use sqlx::Row;

    Ok(Json(EvidenceBundleDetailResponse {
        content_hash: row.get("content_hash"),
        bundle_id: row.get("bundle_id"),
        run_id: row.get("run_id"),
        candidate_id: row.get("candidate_id"),
        iteration_id: row.get("iteration_id"),
        oracle_suite_id: row.get("oracle_suite_id"),
        oracle_suite_hash: row.get("oracle_suite_hash"),
        verdict: row.get("verdict"),
        artifact_count: row.get("artifact_count"),
        run_completed_at: row.get::<DateTime<Utc>, _>("run_completed_at").to_rfc3339(),
        recorded_at: row.get::<DateTime<Utc>, _>("recorded_at").to_rfc3339(),
        recorded_by: ActorInfo {
            kind: row.get("recorded_by_kind"),
            id: row.get("recorded_by_id"),
        },
    }))
}

/// List oracle suites
///
/// GET /api/v1/references/oracle-suites
#[instrument(skip(state, _user))]
pub async fn list_oracle_suites(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let suites = state.oracle_registry.list_suites().await;

    let refs: Vec<TypedRefResponse> = suites
        .iter()
        .map(|s| TypedRefResponse {
            kind: "OracleSuite".to_string(),
            id: s.suite_id.clone(),
            rel: "verifies".to_string(),
            meta: RefMetaResponse {
                content_hash: Some(s.suite_hash.clone()),
                type_key: Some("oracle_suite".to_string()),
                ..Default::default()
            },
            label: Some(s.suite_id.clone()),
        })
        .collect();

    let total = refs.len() as i64;

    // Paginate
    let offset = ((params.page.saturating_sub(1)) * params.page_size) as usize;
    let refs: Vec<TypedRefResponse> = refs
        .into_iter()
        .skip(offset)
        .take(params.page_size as usize)
        .collect();

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List procedure templates
///
/// GET /api/v1/references/procedure-templates
#[instrument(skip(state, _user))]
pub async fn list_procedure_templates(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let templates = state.template_registry.list_templates(None).await;

    // Filter to procedure templates only
    let refs: Vec<TypedRefResponse> = templates
        .iter()
        .filter(|t| t.type_key == "config.procedure_template")
        .map(|t| TypedRefResponse {
            kind: "ProcedureTemplate".to_string(),
            id: t.id.clone(),
            rel: "depends_on".to_string(),
            meta: RefMetaResponse {
                content_hash: Some(t.content_hash.clone()),
                type_key: Some(t.type_key.clone()),
                ..Default::default()
            },
            label: Some(t.name.clone()),
        })
        .collect();

    let total = refs.len() as i64;

    // Paginate
    let offset = ((params.page.saturating_sub(1)) * params.page_size) as usize;
    let refs: Vec<TypedRefResponse> = refs
        .into_iter()
        .skip(offset)
        .take(params.page_size as usize)
        .collect();

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List exceptions
///
/// GET /api/v1/references/exceptions
#[instrument(skip(state, _user))]
pub async fn list_exceptions(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let offset = (params.page.saturating_sub(1)) * params.page_size;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM proj.exceptions")
        .fetch_one(state.app_state.projections.pool())
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let refs = get_exceptions_refs(&state.app_state, Some((params.page_size, offset))).await?;

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List iteration summaries
///
/// GET /api/v1/references/iteration-summaries
#[instrument(skip(state, _user))]
pub async fn list_iteration_summaries(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let offset = (params.page.saturating_sub(1)) * params.page_size;

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM proj.iterations WHERE state = 'COMPLETED'")
            .fetch_one(state.app_state.projections.pool())
            .await
            .map_err(|e| ApiError::Internal {
                message: e.to_string(),
            })?;

    let rows = sqlx::query(
        r#"
        SELECT iteration_id, loop_id, sequence, state, started_at, completed_at
        FROM proj.iterations
        WHERE state = 'COMPLETED'
        ORDER BY completed_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.page_size as i64)
    .bind(offset as i64)
    .fetch_all(state.app_state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    use sqlx::Row;

    let refs: Vec<TypedRefResponse> = rows
        .iter()
        .map(|row| {
            let iteration_id: String = row.get("iteration_id");
            let loop_id: String = row.get("loop_id");
            let sequence: i32 = row.get("sequence");

            TypedRefResponse {
                kind: "Iteration".to_string(),
                id: iteration_id.clone(),
                rel: "relates_to".to_string(),
                meta: RefMetaResponse {
                    type_key: Some("domain.iteration".to_string()),
                    ..Default::default()
                },
                label: Some(format!("{} #{}", loop_id, sequence)),
            }
        })
        .collect();

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List agent definitions (stub - returns empty)
///
/// GET /api/v1/references/agent-definitions
#[instrument(skip(_state, _user))]
pub async fn list_agent_definitions(
    State(_state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    // Stub: No agent definitions data source yet
    Ok(Json(ReferencesListResponse {
        refs: vec![],
        total: 0,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List gating policies
///
/// GET /api/v1/references/gating-policies
#[instrument(skip(state, _user))]
pub async fn list_gating_policies(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let templates = state.template_registry.list_templates(None).await;

    // Filter to gating policies only
    let refs: Vec<TypedRefResponse> = templates
        .iter()
        .filter(|t| t.type_key == "config.gating_policy")
        .map(|t| TypedRefResponse {
            kind: "GatingPolicy".to_string(),
            id: t.id.clone(),
            rel: "governed_by".to_string(),
            meta: RefMetaResponse {
                content_hash: Some(t.content_hash.clone()),
                type_key: Some(t.type_key.clone()),
                ..Default::default()
            },
            label: Some(t.name.clone()),
        })
        .collect();

    let total = refs.len() as i64;

    // Paginate
    let offset = ((params.page.saturating_sub(1)) * params.page_size) as usize;
    let refs: Vec<TypedRefResponse> = refs
        .into_iter()
        .skip(offset)
        .take(params.page_size as usize)
        .collect();

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// List intakes as references
///
/// GET /api/v1/references/intakes
#[instrument(skip(state, _user))]
pub async fn list_intakes_as_refs(
    State(state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>> {
    let offset = (params.page.saturating_sub(1)) * params.page_size;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM proj.intakes")
        .fetch_one(state.app_state.projections.pool())
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let refs = get_intakes_refs(&state.app_state, Some((params.page_size, offset))).await?;

    Ok(Json(ReferencesListResponse {
        refs,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// Upload document (stub - not implemented)
///
/// POST /api/v1/references/documents
#[instrument(skip(_state, _user))]
pub async fn upload_document(
    State(_state): State<ReferencesState>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented {
        feature: "Document upload".to_string(),
    })
}

/// Get document (stub - not implemented)
///
/// GET /api/v1/references/documents/:id
#[instrument(skip(_state, _user))]
pub async fn get_document(
    State(_state): State<ReferencesState>,
    _user: AuthenticatedUser,
    Path(document_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotFound {
        resource: "Document".to_string(),
        id: document_id,
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

async fn get_governed_artifacts_refs(
    state: &AppState,
    pagination: Option<(u32, u32)>,
) -> Result<Vec<TypedRefResponse>, ApiError> {
    let (limit, offset) = pagination.unwrap_or((100, 0));

    let rows = sqlx::query(
        r#"
        SELECT artifact_id, artifact_type, version, content_hash, status
        FROM proj.governed_artifacts
        WHERE is_current = TRUE
        ORDER BY recorded_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    use sqlx::Row;

    Ok(rows
        .iter()
        .map(|row| {
            let artifact_id: String = row.get("artifact_id");
            let artifact_type: String = row.get("artifact_type");
            let version: String = row.get("version");
            let content_hash: String = row.get("content_hash");

            TypedRefResponse {
                kind: "GovernedArtifact".to_string(),
                id: artifact_id.clone(),
                rel: "governed_by".to_string(),
                meta: RefMetaResponse {
                    content_hash: Some(content_hash),
                    version: Some(version),
                    type_key: Some(artifact_type),
                    ..Default::default()
                },
                label: Some(artifact_id),
            }
        })
        .collect())
}

async fn get_candidates_refs(
    state: &AppState,
    pagination: Option<(u32, u32)>,
) -> Result<Vec<TypedRefResponse>, ApiError> {
    let (limit, offset) = pagination.unwrap_or((100, 0));

    let rows = sqlx::query(
        r#"
        SELECT candidate_id, content_hash, verification_status
        FROM proj.candidates
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    use sqlx::Row;

    Ok(rows
        .iter()
        .map(|row| {
            let candidate_id: String = row.get("candidate_id");
            let content_hash: String = row.get("content_hash");
            let verification_status: String = row.get("verification_status");

            TypedRefResponse {
                kind: "Candidate".to_string(),
                id: candidate_id.clone(),
                rel: "produces".to_string(),
                meta: RefMetaResponse {
                    content_hash: Some(content_hash),
                    type_key: Some("domain.candidate".to_string()),
                    ..Default::default()
                },
                label: Some(format!("{} ({})", candidate_id, verification_status)),
            }
        })
        .collect())
}

async fn get_exceptions_refs(
    state: &AppState,
    pagination: Option<(u32, u32)>,
) -> Result<Vec<TypedRefResponse>, ApiError> {
    let (limit, offset) = pagination.unwrap_or((100, 0));

    let rows = sqlx::query(
        r#"
        SELECT exception_id, kind, status, target_description
        FROM proj.exceptions
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    use sqlx::Row;

    Ok(rows
        .iter()
        .map(|row| {
            let exception_id: String = row.get("exception_id");
            let kind: String = row.get("kind");
            let status: String = row.get("status");
            let target_description: String = row.get("target_description");

            // Map kind to RefKind
            let ref_kind = match kind.as_str() {
                "DEVIATION" => "Deviation",
                "DEFERRAL" => "Deferral",
                "WAIVER" => "Waiver",
                _ => "Record",
            };

            TypedRefResponse {
                kind: ref_kind.to_string(),
                id: exception_id.clone(),
                rel: "acknowledges".to_string(),
                meta: RefMetaResponse {
                    type_key: Some(format!("record.{}", kind.to_lowercase())),
                    ..Default::default()
                },
                label: Some(format!("{}: {} ({})", kind, target_description, status)),
            }
        })
        .collect())
}

async fn get_intakes_refs(
    state: &AppState,
    pagination: Option<(u32, u32)>,
) -> Result<Vec<TypedRefResponse>, ApiError> {
    let (limit, offset) = pagination.unwrap_or((100, 0));

    let rows = sqlx::query(
        r#"
        SELECT intake_id, title, status::text, content_hash
        FROM proj.intakes
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    use sqlx::Row;

    Ok(rows
        .iter()
        .map(|row| {
            let intake_id: String = row.get("intake_id");
            let title: String = row.get("title");
            let status: String = row.get("status");
            let content_hash: Option<String> = row.get("content_hash");

            TypedRefResponse {
                kind: "Intake".to_string(),
                id: intake_id.clone(),
                rel: "about".to_string(),
                meta: RefMetaResponse {
                    content_hash,
                    type_key: Some("record.intake".to_string()),
                    ..Default::default()
                },
                label: Some(format!("{} ({})", title, status)),
            }
        })
        .collect())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pagination() {
        assert_eq!(default_page(), 1);
        assert_eq!(default_page_size(), 20);
    }

    #[test]
    fn test_typed_ref_response_serialization() {
        let r = TypedRefResponse {
            kind: "GovernedArtifact".to_string(),
            id: "SR-CONTRACT".to_string(),
            rel: "governed_by".to_string(),
            meta: RefMetaResponse {
                content_hash: Some("sha256:abc123".to_string()),
                version: Some("1.0.0".to_string()),
                type_key: Some("governance.contract".to_string()),
                ..Default::default()
            },
            label: Some("Architectural Contract".to_string()),
        };

        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("GovernedArtifact"));
        assert!(json.contains("SR-CONTRACT"));
        assert!(json.contains("sha256:abc123"));
    }

    #[test]
    fn test_references_list_response_serialization() {
        let response = ReferencesListResponse {
            refs: vec![TypedRefResponse {
                kind: "Intake".to_string(),
                id: "intake:01ABC".to_string(),
                rel: "about".to_string(),
                meta: RefMetaResponse::default(),
                label: None,
            }],
            total: 1,
            page: 1,
            page_size: 20,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"total\":1"));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"page_size\":20"));
    }
}
