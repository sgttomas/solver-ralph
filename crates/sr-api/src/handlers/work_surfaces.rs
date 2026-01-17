//! Work Surface API Handlers per SR-PLAN-V4 Phase 4b
//!
//! Per SR-WORK-SURFACE §5 and SR-PLAN-V4 §3: Endpoints for creating, querying,
//! and managing Work Surface Instances that bind Intake + Procedure Template.
//!
//! Endpoints:
//! - POST   /api/v1/work-surfaces                              - Create/bind a Work Surface
//! - GET    /api/v1/work-surfaces                              - List with filters
//! - GET    /api/v1/work-surfaces/:work_surface_id             - Get by ID
//! - GET    /api/v1/work-surfaces/by-work-unit/:work_unit_id   - Get active for Work Unit
//! - POST   /api/v1/work-surfaces/:id/stages/:stage_id/complete - Complete stage
//! - GET    /api/v1/work-surfaces/:id/iteration-context        - Get iteration refs
//! - POST   /api/v1/work-surfaces/:id/archive                  - Archive Work Surface
//! - GET    /api/v1/work-surfaces/compatibility                - Check compatibility
//! - GET    /api/v1/work-surfaces/compatible-templates         - List compatible templates
//! - POST   /api/v1/work-surfaces/:id/start                    - Start work (create Loop + Iteration)
//! - GET    /api/v1/work-surfaces/:id/iterations               - List iterations (V7-5)
//! - POST   /api/v1/work-surfaces/:id/iterations               - Start new iteration (V7-5)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sr_domain::entities::ContentHash;
use sr_domain::events::{GateResult, GateResultStatus, OracleResultSummary};
use sr_domain::work_surface::{
    validate_work_kind_compatibility, ContentAddressedRef, ManagedWorkSurface, OracleSuiteBinding,
    ProcedureTemplate, StageId, WorkKind, WorkSurfaceId, WorkUnitId,
};
use sr_domain::{ActorKind, EventEnvelope, EventId, IterationId, LoopId, StreamKind, TypedRef};
use sr_ports::EventStore;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::templates::{TemplateCategory, TemplateRegistry};
use crate::AppState;
use sr_adapters::oracle_suite::OracleSuiteRegistry;

// ============================================================================
// Combined State for Work Surfaces
// ============================================================================

/// Work Surface state combining AppState with registry access
#[derive(Clone)]
pub struct WorkSurfaceState {
    pub app_state: AppState,
    pub oracle_registry: Arc<OracleSuiteRegistry>,
    pub template_registry: Arc<TemplateRegistry>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a new Work Surface
#[derive(Debug, Deserialize)]
pub struct CreateWorkSurfaceRequest {
    /// Work unit identifier
    pub work_unit_id: String,
    /// Active intake ID (must be status = active)
    pub intake_id: String,
    /// Procedure template ID
    pub procedure_template_id: String,
    /// Optional stage parameters
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

/// Response for a single Work Surface
#[derive(Debug, Serialize)]
pub struct WorkSurfaceResponse {
    pub work_surface_id: String,
    pub work_unit_id: String,
    pub intake_id: String,
    pub intake_content_hash: String,
    pub procedure_template_id: String,
    pub procedure_template_hash: String,
    pub current_stage_id: String,
    pub status: String,
    pub stage_status: serde_json::Value,
    pub current_oracle_suites: serde_json::Value,
    pub params: serde_json::Value,
    pub content_hash: String,
    pub bound_at: String,
    pub bound_by: ActorInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<ActorInfo>,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Summary for list responses
#[derive(Debug, Serialize)]
pub struct WorkSurfaceSummary {
    pub work_surface_id: String,
    pub work_unit_id: String,
    pub intake_id: String,
    pub intake_title: Option<String>,
    pub procedure_template_id: String,
    pub procedure_template_name: Option<String>,
    pub current_stage_id: String,
    pub status: String,
    pub bound_at: String,
}

/// Query parameters for listing Work Surfaces
#[derive(Debug, Deserialize)]
pub struct ListWorkSurfacesQuery {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub intake_id: Option<String>,
    #[serde(default)]
    pub procedure_template_id: Option<String>,
    #[serde(default)]
    pub work_unit_id: Option<String>,
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

/// Response for listing Work Surfaces
#[derive(Debug, Serialize)]
pub struct ListWorkSurfacesResponse {
    pub work_surfaces: Vec<WorkSurfaceSummary>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}

/// Request to complete a stage
#[derive(Debug, Deserialize)]
pub struct CompleteStageRequest {
    /// Evidence bundle hash proving gate passage
    pub evidence_bundle_ref: String,
    /// Gate result details
    pub gate_result: GateResultRequest,
}

#[derive(Debug, Deserialize)]
pub struct GateResultRequest {
    pub status: String,
    #[serde(default)]
    pub oracle_results: Vec<OracleResultRequest>,
    #[serde(default)]
    pub waiver_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct OracleResultRequest {
    pub oracle_id: String,
    pub status: String,
    #[serde(default)]
    pub evidence_ref: Option<String>,
}

/// Response for stage completion
#[derive(Debug, Serialize)]
pub struct StageCompletionResponse {
    pub work_surface_id: String,
    pub completed_stage_id: String,
    pub next_stage_id: Option<String>,
    pub is_terminal: bool,
    pub work_surface_status: String,
}

/// Request to archive a Work Surface
#[derive(Debug, Deserialize)]
pub struct ArchiveWorkSurfaceRequest {
    #[serde(default)]
    pub reason: Option<String>,
}

/// Query parameters for compatibility check
#[derive(Debug, Deserialize)]
pub struct CompatibilityCheckParams {
    pub intake_id: String,
    pub procedure_template_id: String,
}

/// Response for compatibility check
#[derive(Debug, Serialize)]
pub struct CompatibilityCheckResponse {
    pub compatible: bool,
    pub intake_kind: String,
    pub template_supported_kinds: Vec<String>,
    pub issues: Vec<String>,
}

/// Query parameters for compatible templates
#[derive(Debug, Deserialize)]
pub struct CompatibleTemplatesParams {
    pub intake_id: String,
}

/// Response for compatible templates
#[derive(Debug, Serialize)]
pub struct CompatibleTemplatesResponse {
    pub intake_id: String,
    pub intake_kind: String,
    pub templates: Vec<ProcedureTemplateSummary>,
}

#[derive(Debug, Serialize)]
pub struct ProcedureTemplateSummary {
    pub procedure_template_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub stages_count: u32,
    pub supported_kinds: Vec<String>,
}

/// Response for iteration context
#[derive(Debug, Serialize)]
pub struct IterationContextResponse {
    pub work_surface_id: String,
    pub refs: Vec<TypedRef>,
}

// ============================================================================
// Iteration List/Create Types (SR-PLAN-V7 Phase V7-5)
// ============================================================================

/// Response for listing iterations for a Work Surface
#[derive(Debug, Serialize)]
pub struct WorkSurfaceIterationsResponse {
    pub iterations: Vec<IterationSummary>,
    pub loop_id: String,
    pub total: usize,
}

/// Summary of a single iteration
#[derive(Debug, Serialize)]
pub struct IterationSummary {
    pub iteration_id: String,
    pub iteration_number: i32,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
}

/// Response for starting a new iteration
#[derive(Debug, Serialize)]
pub struct StartIterationResponse {
    pub iteration_id: String,
    pub iteration_number: i32,
}

/// Response for Work Surface action (create, archive)
#[derive(Debug, Serialize)]
pub struct WorkSurfaceActionResponse {
    pub work_surface_id: String,
    pub status: String,
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

// ============================================================================
// Handlers
// ============================================================================

/// Create a new Work Surface (bind Intake + Procedure Template)
///
/// POST /api/v1/work-surfaces
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn create_work_surface(
    State(state): State<WorkSurfaceState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateWorkSurfaceRequest>,
) -> ApiResult<Json<WorkSurfaceActionResponse>> {
    // 1. Validate intake exists and is active
    let intake = get_intake_projection(&state.app_state, &body.intake_id).await?;

    if intake.status != "active" {
        return Err(ApiError::BadRequest {
            message: format!(
                "Intake must be active to bind. Current status: {}",
                intake.status
            ),
        });
    }

    // 2. Validate procedure template exists
    let template =
        get_procedure_template_from_registry(&state.template_registry, &body.procedure_template_id)
            .await
            .ok_or_else(|| ApiError::NotFound {
                resource: "ProcedureTemplate".to_string(),
                id: body.procedure_template_id.clone(),
            })?;

    // 3. Validate work kind compatibility
    let intake_kind = parse_work_kind(&intake.kind)?;
    let template_kinds: Vec<WorkKind> = template.kind.clone();

    if let Err(e) = validate_work_kind_compatibility(&intake_kind, &template_kinds) {
        return Err(ApiError::BadRequest {
            message: format!("Incompatible work kind: {}", e),
        });
    }

    // 4. Check no existing active Work Surface for this work unit
    let existing = sqlx::query_scalar::<_, String>(
        "SELECT work_surface_id FROM proj.work_surfaces WHERE work_unit_id = $1 AND status = 'active'",
    )
    .bind(&body.work_unit_id)
    .fetch_optional(state.app_state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    if let Some(existing_id) = existing {
        return Err(ApiError::Conflict {
            message: format!(
                "Work unit {} already has an active Work Surface: {}",
                body.work_unit_id, existing_id
            ),
        });
    }

    // 5. Resolve oracle suites for initial stage
    let initial_stage = template.get_initial_stage();
    let oracle_suites =
        resolve_oracle_suites_for_stage(&state.oracle_registry, &template, initial_stage);

    // 6. Create ManagedWorkSurface
    let intake_content_hash = intake
        .content_hash
        .clone()
        .unwrap_or_else(|| "sha256:unknown".to_string());
    let template_content_hash = template
        .content_hash
        .as_ref()
        .map(|h: &ContentHash| h.as_str().to_string())
        .unwrap_or_else(|| compute_template_hash(&template));

    let work_surface = ManagedWorkSurface::new_bound(
        WorkUnitId::from_string(body.work_unit_id.clone()),
        ContentAddressedRef {
            id: body.intake_id.clone(),
            content_hash: ContentHash::new(&intake_content_hash),
        },
        ContentAddressedRef {
            id: body.procedure_template_id.clone(),
            content_hash: ContentHash::new(&template_content_hash),
        },
        initial_stage.clone(),
        oracle_suites.clone(),
        body.params.clone(),
        sr_domain::entities::ActorId {
            kind: user.actor_kind.clone(),
            id: user.actor_id.clone(),
        },
    );

    let work_surface_id = work_surface.work_surface_id.as_str().to_string();
    let content_hash = work_surface.content_hash.clone().unwrap_or_default();

    // 7. Emit WorkSurfaceBound event
    let bound_event_id = EventId::new();
    let now = Utc::now();

    let bound_payload = serde_json::json!({
        "work_surface_id": work_surface_id,
        "work_unit_id": body.work_unit_id,
        "intake_ref": {
            "id": body.intake_id,
            "content_hash": intake_content_hash,
        },
        "procedure_template_ref": {
            "id": body.procedure_template_id,
            "content_hash": template_content_hash,
        },
        "initial_stage_id": initial_stage.as_str(),
        "content_hash": content_hash,
        "params": body.params,
    });

    let bound_event = EventEnvelope {
        event_id: bound_event_id.clone(),
        stream_id: work_surface_id.clone(),
        stream_kind: StreamKind::WorkSurface,
        stream_seq: 1,
        global_seq: None,
        event_type: "WorkSurfaceBound".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(),
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![
            TypedRef {
                kind: "Intake".to_string(),
                id: body.intake_id.clone(),
                rel: "depends_on".to_string(),
                meta: serde_json::json!({ "content_hash": intake_content_hash }),
            },
            TypedRef {
                kind: "ProcedureTemplate".to_string(),
                id: body.procedure_template_id.clone(),
                rel: "depends_on".to_string(),
                meta: serde_json::json!({ "content_hash": template_content_hash }),
            },
        ],
        payload: bound_payload,
        envelope_hash: compute_envelope_hash(&bound_event_id),
    };

    // 8. Emit StageEntered event for initial stage
    let stage_event_id = EventId::new();

    let stage_payload = serde_json::json!({
        "work_surface_id": work_surface_id,
        "stage_id": initial_stage.as_str(),
        "previous_stage_id": null,
        "oracle_suites": oracle_suites,
    });

    let stage_event = EventEnvelope {
        event_id: stage_event_id,
        stream_id: work_surface_id.clone(),
        stream_kind: StreamKind::WorkSurface,
        stream_seq: 2,
        global_seq: None,
        event_type: "StageEntered".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(),
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: stage_payload,
        envelope_hash: compute_envelope_hash(&bound_event_id),
    };

    // Append events
    state
        .app_state
        .event_store
        .append(&work_surface_id, 0, vec![bound_event, stage_event])
        .await?;

    // Process projections
    state
        .app_state
        .projections
        .process_events(&*state.app_state.event_store)
        .await?;

    info!(work_surface_id = %work_surface_id, "Work Surface created");

    Ok(Json(WorkSurfaceActionResponse {
        work_surface_id,
        status: "active".to_string(),
        event_id: bound_event_id.as_str().to_string(),
        content_hash: Some(content_hash),
    }))
}

/// Get a Work Surface by ID
///
/// GET /api/v1/work-surfaces/:work_surface_id
#[instrument(skip(state, _user))]
pub async fn get_work_surface(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<WorkSurfaceResponse>> {
    let ws = get_work_surface_projection(&state.app_state, &work_surface_id).await?;
    Ok(Json(ws))
}

/// Get the active Work Surface for a Work Unit
///
/// GET /api/v1/work-surfaces/by-work-unit/:work_unit_id
#[instrument(skip(state, _user))]
pub async fn get_work_surface_by_work_unit(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Path(work_unit_id): Path<String>,
) -> ApiResult<Json<Option<WorkSurfaceResponse>>> {
    let row = sqlx::query(
        r#"
        SELECT work_surface_id, work_unit_id, intake_id, intake_content_hash,
               procedure_template_id, procedure_template_hash, current_stage_id,
               status, stage_status, current_oracle_suites, params, content_hash,
               bound_at, bound_by_kind, bound_by_id, completed_at, archived_at,
               archived_by_kind, archived_by_id
        FROM proj.work_surfaces
        WHERE work_unit_id = $1 AND status = 'active'
        "#,
    )
    .bind(&work_unit_id)
    .fetch_optional(state.app_state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    match row {
        Some(r) => Ok(Json(Some(row_to_work_surface_response(&r)))),
        None => Ok(Json(None)),
    }
}

/// List Work Surfaces with optional filtering
///
/// GET /api/v1/work-surfaces
#[instrument(skip(state, _user))]
pub async fn list_work_surfaces(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListWorkSurfacesQuery>,
) -> ApiResult<Json<ListWorkSurfacesResponse>> {
    let offset = (query.page.saturating_sub(1)) * query.page_size;

    // Build dynamic query
    let mut sql = String::from(
        r#"
        SELECT ws.work_surface_id, ws.work_unit_id, ws.intake_id, ws.procedure_template_id,
               ws.current_stage_id, ws.status::text, ws.bound_at,
               i.title as intake_title
        FROM proj.work_surfaces ws
        LEFT JOIN proj.intakes i ON ws.intake_id = i.intake_id
        WHERE 1=1
        "#,
    );

    let mut count_sql = String::from("SELECT COUNT(*) FROM proj.work_surfaces ws WHERE 1=1");

    let mut param_idx = 1;
    let mut bindings: Vec<String> = vec![];

    if let Some(ref status) = query.status {
        sql.push_str(&format!(" AND ws.status = ${}", param_idx));
        count_sql.push_str(&format!(" AND ws.status = ${}", param_idx));
        bindings.push(status.clone());
        param_idx += 1;
    }
    if let Some(ref intake_id) = query.intake_id {
        sql.push_str(&format!(" AND ws.intake_id = ${}", param_idx));
        count_sql.push_str(&format!(" AND ws.intake_id = ${}", param_idx));
        bindings.push(intake_id.clone());
        param_idx += 1;
    }
    if let Some(ref template_id) = query.procedure_template_id {
        sql.push_str(&format!(" AND ws.procedure_template_id = ${}", param_idx));
        count_sql.push_str(&format!(" AND ws.procedure_template_id = ${}", param_idx));
        bindings.push(template_id.clone());
        param_idx += 1;
    }
    if let Some(ref work_unit_id) = query.work_unit_id {
        sql.push_str(&format!(" AND ws.work_unit_id = ${}", param_idx));
        count_sql.push_str(&format!(" AND ws.work_unit_id = ${}", param_idx));
        bindings.push(work_unit_id.clone());
    }

    sql.push_str(" ORDER BY ws.bound_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", query.page_size, offset));

    // Execute count query
    let total: i64 = {
        let mut q = sqlx::query_scalar(&count_sql);
        for binding in &bindings {
            q = q.bind(binding);
        }
        q.fetch_one(state.app_state.projections.pool())
            .await
            .map_err(|e| ApiError::Internal {
                message: e.to_string(),
            })?
    };

    // Execute main query
    let rows = {
        let mut q = sqlx::query(&sql);
        for binding in &bindings {
            q = q.bind(binding);
        }
        q.fetch_all(state.app_state.projections.pool())
            .await
            .map_err(|e| ApiError::Internal {
                message: e.to_string(),
            })?
    };

    let work_surfaces: Vec<WorkSurfaceSummary> = rows
        .iter()
        .map(|row| row_to_work_surface_summary(row))
        .collect();

    Ok(Json(ListWorkSurfacesResponse {
        work_surfaces,
        total,
        page: query.page,
        page_size: query.page_size,
    }))
}

/// Complete a stage (record gate passage)
///
/// POST /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn complete_stage(
    State(state): State<WorkSurfaceState>,
    user: AuthenticatedUser,
    Path((work_surface_id, stage_id)): Path<(String, String)>,
    Json(body): Json<CompleteStageRequest>,
) -> ApiResult<Json<StageCompletionResponse>> {
    // 1. Get current Work Surface and verify status=active
    let current = get_work_surface_projection(&state.app_state, &work_surface_id).await?;

    if current.status != "active" {
        return Err(ApiError::InvalidTransition {
            current_state: current.status,
            action: "complete_stage".to_string(),
        });
    }

    // 2. Verify stage_id matches current_stage_id
    if current.current_stage_id != stage_id {
        return Err(ApiError::BadRequest {
            message: format!(
                "Cannot complete stage '{}'. Current stage is '{}'",
                stage_id, current.current_stage_id
            ),
        });
    }

    // 3. Get procedure template to determine next stage
    let template = get_procedure_template_from_registry(
        &state.template_registry,
        &current.procedure_template_id,
    )
    .await
    .ok_or_else(|| ApiError::Internal {
        message: format!(
            "Procedure template not found: {}",
            current.procedure_template_id
        ),
    })?;

    let current_stage_obj = StageId::from_string(stage_id.clone());
    let is_terminal = template.is_terminal(&current_stage_obj);
    let next_stage_id: Option<StageId> = template.get_next_stage(&current_stage_obj).cloned();

    // 3b. Check if stage requires approval (SR-PLAN-V5 Phase 5c)
    if let Some(stage_def) = template.get_stage(&current_stage_obj) {
        if stage_def.requires_approval {
            let portal_id = format!("portal:STAGE_COMPLETION:{}", stage_id);

            // Query for existing approval
            let approval = state
                .app_state
                .projections
                .get_stage_approval(&portal_id, &work_surface_id)
                .await?;

            if approval.is_none() {
                return Err(ApiError::ApprovalRequired {
                    stage_id: stage_id.clone(),
                    portal_id,
                    work_surface_id: work_surface_id.clone(),
                });
            }
        }
    }

    // 4. Build gate result
    let gate_result = GateResult {
        status: parse_gate_result_status(&body.gate_result.status)?,
        oracle_results: body
            .gate_result
            .oracle_results
            .iter()
            .map(|r| OracleResultSummary {
                oracle_id: r.oracle_id.clone(),
                status: r.status.clone(),
                evidence_ref: r.evidence_ref.clone(),
            })
            .collect(),
        waiver_refs: body.gate_result.waiver_refs.clone(),
    };

    // 5. Get current stream version
    let events = state
        .app_state
        .event_store
        .read_stream(&work_surface_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let now = Utc::now();
    let mut events_to_emit = Vec::new();

    // 6. Emit StageCompleted event
    let complete_event_id = EventId::new();
    let complete_payload = serde_json::json!({
        "work_surface_id": work_surface_id,
        "stage_id": stage_id,
        "evidence_bundle_ref": body.evidence_bundle_ref,
        "gate_result": gate_result,
        "next_stage_id": next_stage_id.as_ref().map(|s| s.as_str()),
    });

    events_to_emit.push(EventEnvelope {
        event_id: complete_event_id.clone(),
        stream_id: work_surface_id.clone(),
        stream_kind: StreamKind::WorkSurface,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "StageCompleted".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(),
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: complete_payload,
        envelope_hash: compute_envelope_hash(&complete_event_id),
    });

    let mut final_status = "active".to_string();

    // 7. If terminal, emit WorkSurfaceCompleted
    if is_terminal {
        let ws_complete_event_id = EventId::new();
        let ws_complete_payload = serde_json::json!({
            "work_surface_id": work_surface_id,
            "final_stage_id": stage_id,
            "evidence_bundle_ref": body.evidence_bundle_ref,
        });

        events_to_emit.push(EventEnvelope {
            event_id: ws_complete_event_id.clone(),
            stream_id: work_surface_id.clone(),
            stream_kind: StreamKind::WorkSurface,
            stream_seq: current_version + 2,
            global_seq: None,
            event_type: "WorkSurfaceCompleted".to_string(),
            occurred_at: now,
            actor_kind: user.actor_kind.clone(),
            actor_id: user.actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: ws_complete_payload,
            envelope_hash: compute_envelope_hash(&ws_complete_event_id),
        });

        final_status = "completed".to_string();
    } else if let Some(ref next_stage) = next_stage_id {
        // 8. If not terminal, emit StageEntered for next stage
        let oracle_suites =
            resolve_oracle_suites_for_stage(&state.oracle_registry, &template, next_stage);

        let enter_event_id = EventId::new();
        let enter_payload = serde_json::json!({
            "work_surface_id": work_surface_id,
            "stage_id": next_stage.as_str(),
            "previous_stage_id": stage_id,
            "oracle_suites": oracle_suites,
        });

        events_to_emit.push(EventEnvelope {
            event_id: enter_event_id.clone(),
            stream_id: work_surface_id.clone(),
            stream_kind: StreamKind::WorkSurface,
            stream_seq: current_version + 2,
            global_seq: None,
            event_type: "StageEntered".to_string(),
            occurred_at: now,
            actor_kind: user.actor_kind.clone(),
            actor_id: user.actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: enter_payload,
            envelope_hash: compute_envelope_hash(&enter_event_id),
        });
    }

    // Append events
    state
        .app_state
        .event_store
        .append(&work_surface_id, current_version, events_to_emit)
        .await?;

    // Process projections
    state
        .app_state
        .projections
        .process_events(&*state.app_state.event_store)
        .await?;

    info!(
        work_surface_id = %work_surface_id,
        stage_id = %stage_id,
        is_terminal = %is_terminal,
        "Stage completed"
    );

    Ok(Json(StageCompletionResponse {
        work_surface_id,
        completed_stage_id: stage_id,
        next_stage_id: next_stage_id.map(|s| s.as_str().to_string()),
        is_terminal,
        work_surface_status: final_status,
    }))
}

/// Response for stage approval status endpoint
#[derive(Debug, Serialize)]
pub struct StageApprovalStatusResponse {
    pub stage_id: String,
    pub requires_approval: bool,
    pub portal_id: String,
    pub approval: Option<ApprovalInfo>,
}

/// Approval info for stage approval status
#[derive(Debug, Serialize)]
pub struct ApprovalInfo {
    pub approval_id: String,
    pub decision: String,
    pub recorded_at: String,
    pub recorded_by: ActorInfo,
}

/// Get stage approval status
///
/// GET /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/approval-status
///
/// Per SR-PLAN-V5 Phase 5c: Returns whether the stage requires approval and
/// if an approval exists for this work surface at the stage gate portal.
#[instrument(skip(state, _user))]
pub async fn get_stage_approval_status(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Path((work_surface_id, stage_id)): Path<(String, String)>,
) -> ApiResult<Json<StageApprovalStatusResponse>> {
    // 1. Get current Work Surface to verify it exists
    let current = get_work_surface_projection(&state.app_state, &work_surface_id).await?;

    // 2. Get procedure template to find stage definition
    let template = get_procedure_template_from_registry(
        &state.template_registry,
        &current.procedure_template_id,
    )
    .await
    .ok_or_else(|| ApiError::Internal {
        message: format!(
            "Procedure template not found: {}",
            current.procedure_template_id
        ),
    })?;

    let stage_obj = StageId::from_string(stage_id.clone());
    let stage_def = template
        .get_stage(&stage_obj)
        .ok_or_else(|| ApiError::NotFound {
            resource: "Stage".to_string(),
            id: stage_id.clone(),
        })?;

    let requires_approval = stage_def.requires_approval;
    let portal_id = format!("portal:STAGE_COMPLETION:{}", stage_id);

    // 3. If approval required, check if one exists
    let approval = if requires_approval {
        let approval_proj = state
            .app_state
            .projections
            .get_stage_approval(&portal_id, &work_surface_id)
            .await?;

        approval_proj.map(|a| ApprovalInfo {
            approval_id: a.approval_id,
            decision: a.decision,
            recorded_at: a.approved_at.to_rfc3339(),
            recorded_by: ActorInfo {
                kind: a.approved_by_kind,
                id: a.approved_by_id,
            },
        })
    } else {
        None
    };

    Ok(Json(StageApprovalStatusResponse {
        stage_id,
        requires_approval,
        portal_id,
        approval,
    }))
}

/// Get iteration context refs for the Work Surface
///
/// GET /api/v1/work-surfaces/:work_surface_id/iteration-context
#[instrument(skip(state, _user))]
pub async fn get_iteration_context(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<IterationContextResponse>> {
    let ws = get_work_surface_projection(&state.app_state, &work_surface_id).await?;

    let mut refs = Vec::new();

    // 1. Intake ref
    refs.push(TypedRef {
        kind: "Intake".to_string(),
        id: ws.intake_id.clone(),
        rel: "depends_on".to_string(),
        meta: serde_json::json!({
            "content_hash": ws.intake_content_hash,
        }),
    });

    // 2. Procedure Template ref
    refs.push(TypedRef {
        kind: "ProcedureTemplate".to_string(),
        id: ws.procedure_template_id.clone(),
        rel: "depends_on".to_string(),
        meta: serde_json::json!({
            "content_hash": ws.procedure_template_hash,
            "current_stage_id": ws.current_stage_id,
        }),
    });

    // 3. Oracle Suite refs for current stage
    let suites: Vec<OracleSuiteBinding> =
        serde_json::from_value(ws.current_oracle_suites.clone()).unwrap_or_default();

    for suite in suites {
        refs.push(TypedRef {
            kind: "OracleSuite".to_string(),
            id: suite.suite_id.clone(),
            rel: "depends_on".to_string(),
            meta: serde_json::json!({
                "content_hash": suite.suite_hash.as_str(),
            }),
        });
    }

    Ok(Json(IterationContextResponse {
        work_surface_id,
        refs,
    }))
}

/// Archive a Work Surface
///
/// POST /api/v1/work-surfaces/:work_surface_id/archive
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn archive_work_surface(
    State(state): State<WorkSurfaceState>,
    user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
    Json(body): Json<ArchiveWorkSurfaceRequest>,
) -> ApiResult<Json<WorkSurfaceActionResponse>> {
    // Get current Work Surface
    let current = get_work_surface_projection(&state.app_state, &work_surface_id).await?;

    if current.status == "archived" {
        return Err(ApiError::InvalidTransition {
            current_state: current.status,
            action: "archive".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    let payload = serde_json::json!({
        "work_surface_id": work_surface_id,
        "reason": body.reason,
    });

    // Get current stream version
    let events = state
        .app_state
        .event_store
        .read_stream(&work_surface_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: work_surface_id.clone(),
        stream_kind: StreamKind::WorkSurface,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "WorkSurfaceArchived".to_string(),
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
        .app_state
        .event_store
        .append(&work_surface_id, current_version, vec![event])
        .await?;

    state
        .app_state
        .projections
        .process_events(&*state.app_state.event_store)
        .await?;

    info!(work_surface_id = %work_surface_id, "Work Surface archived");

    Ok(Json(WorkSurfaceActionResponse {
        work_surface_id,
        status: "archived".to_string(),
        event_id: event_id.as_str().to_string(),
        content_hash: Some(current.content_hash),
    }))
}

/// Check compatibility between Intake and Procedure Template
///
/// GET /api/v1/work-surfaces/compatibility
#[instrument(skip(state, _user))]
pub async fn check_compatibility(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Query(params): Query<CompatibilityCheckParams>,
) -> ApiResult<Json<CompatibilityCheckResponse>> {
    // Get intake
    let intake = get_intake_projection(&state.app_state, &params.intake_id).await?;

    // Get template
    let template = get_procedure_template_from_registry(
        &state.template_registry,
        &params.procedure_template_id,
    )
    .await
    .ok_or_else(|| ApiError::NotFound {
        resource: "ProcedureTemplate".to_string(),
        id: params.procedure_template_id.clone(),
    })?;

    let intake_kind = intake.kind.clone();
    let template_kinds: Vec<String> = template
        .kind
        .iter()
        .map(|k: &WorkKind| format!("{:?}", k).to_lowercase())
        .collect();

    let mut issues = Vec::new();

    // Check kind compatibility
    let intake_work_kind = parse_work_kind_option(&intake_kind);
    let template_work_kinds: Vec<WorkKind> = template.kind.clone();

    let compatible = match intake_work_kind {
        Some(ref k) => template_work_kinds.contains(k),
        None => false,
    };

    if !compatible {
        issues.push(format!(
            "Intake kind '{}' is not supported by template (supports: {:?})",
            intake_kind, template_kinds
        ));
    }

    // Check intake status
    if intake.status != "active" {
        issues.push(format!(
            "Intake must be active (current status: {})",
            intake.status
        ));
    }

    Ok(Json(CompatibilityCheckResponse {
        compatible: compatible && intake.status == "active",
        intake_kind,
        template_supported_kinds: template_kinds,
        issues,
    }))
}

/// Get Procedure Templates compatible with an Intake
///
/// GET /api/v1/work-surfaces/compatible-templates
#[instrument(skip(state, _user))]
pub async fn get_compatible_templates(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Query(params): Query<CompatibleTemplatesParams>,
) -> ApiResult<Json<CompatibleTemplatesResponse>> {
    // Get intake
    let intake = get_intake_projection(&state.app_state, &params.intake_id).await?;
    let intake_kind = parse_work_kind_option(&intake.kind);

    // Get all procedure templates and filter
    let all_templates = list_procedure_templates_from_registry(&state.template_registry).await;

    let compatible_templates: Vec<ProcedureTemplateSummary> = all_templates
        .into_iter()
        .filter(|t: &ProcedureTemplate| match &intake_kind {
            Some(k) => t.kind.contains(k),
            None => false,
        })
        .map(|t: ProcedureTemplate| ProcedureTemplateSummary {
            procedure_template_id: t.procedure_template_id.as_str().to_string(),
            name: t.name.clone(),
            description: t.description.clone(),
            stages_count: t.stages.len() as u32,
            supported_kinds: t
                .kind
                .iter()
                .map(|k: &WorkKind| format!("{:?}", k).to_lowercase())
                .collect(),
        })
        .collect();

    Ok(Json(CompatibleTemplatesResponse {
        intake_id: params.intake_id,
        intake_kind: intake.kind,
        templates: compatible_templates,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

fn compute_template_hash(template: &sr_domain::work_surface::ProcedureTemplate) -> String {
    use sha2::{Digest, Sha256};
    let canonical = serde_json::to_string(template).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

/// V10-1: Emit StopTriggered event to pause a Loop (C-LOOP-1, C-LOOP-3)
///
/// Emits a StopTriggered event that transitions the Loop to PAUSED state.
/// The `requires_decision` flag indicates whether a Decision must be recorded
/// before the Loop can be resumed.
async fn emit_stop_triggered(
    state: &AppState,
    loop_id: &str,
    trigger: &str,
    requires_decision: bool,
) -> Result<(), ApiError> {
    let event_id = EventId::new();
    let now = Utc::now();

    // Read current stream to get version
    let events = state.event_store.read_stream(loop_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "StopTriggered".to_string(),
        occurred_at: now,
        actor_kind: ActorKind::System,
        actor_id: "system:loop-governor".to_string(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({
            "trigger": trigger,
            "requires_decision": requires_decision,
        }),
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(loop_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    info!(
        loop_id = %loop_id,
        trigger = %trigger,
        requires_decision = %requires_decision,
        "StopTriggered event emitted - Loop paused"
    );

    Ok(())
}

fn parse_work_kind(kind: &str) -> Result<WorkKind, ApiError> {
    parse_work_kind_option(kind).ok_or_else(|| ApiError::BadRequest {
        message: format!("Invalid work kind: {}", kind),
    })
}

fn parse_work_kind_option(kind: &str) -> Option<WorkKind> {
    match kind.to_lowercase().as_str() {
        "research_memo" => Some(WorkKind::ResearchMemo),
        "decision_record" => Some(WorkKind::DecisionRecord),
        "ontology_build" => Some(WorkKind::OntologyBuild),
        "analysis_report" => Some(WorkKind::AnalysisReport),
        "technical_spec" => Some(WorkKind::TechnicalSpec),
        "implementation_plan" => Some(WorkKind::ImplementationPlan),
        "intake_processing" => Some(WorkKind::IntakeProcessing),
        _ => Some(WorkKind::Custom(kind.to_string())),
    }
}

fn parse_gate_result_status(status: &str) -> Result<GateResultStatus, ApiError> {
    match status.to_uppercase().as_str() {
        "PASS" => Ok(GateResultStatus::Pass),
        "PASS_WITH_WAIVERS" => Ok(GateResultStatus::PassWithWaivers),
        "FAIL" => Ok(GateResultStatus::Fail),
        _ => Err(ApiError::BadRequest {
            message: format!("Invalid gate result status: {}", status),
        }),
    }
}

/// Get a ProcedureTemplate from the TemplateRegistry by ID
async fn get_procedure_template_from_registry(
    registry: &TemplateRegistry,
    template_id: &str,
) -> Option<ProcedureTemplate> {
    // Get all templates that are procedure templates
    let templates = registry
        .list_templates(Some(TemplateCategory::WorkSurface))
        .await;

    // Find the template with matching ID or content that references this procedure_template_id
    for template in templates {
        if template.type_key != "config.procedure_template" {
            continue;
        }

        // Check if the content has this procedure_template_id
        if let Some(proc_id) = template.content.get("procedure_template_id") {
            if proc_id.as_str() == Some(template_id) {
                // Deserialize the content to ProcedureTemplate
                match serde_json::from_value::<ProcedureTemplate>(template.content.clone()) {
                    Ok(pt) => return Some(pt),
                    Err(e) => {
                        tracing::error!("Failed to deserialize ProcedureTemplate: {}", e);
                        return None;
                    }
                }
            }
        }

        // Also check if template.id matches
        if template.id == template_id {
            return serde_json::from_value(template.content.clone()).ok();
        }
    }

    None
}

/// List all ProcedureTemplates from the TemplateRegistry
async fn list_procedure_templates_from_registry(
    registry: &TemplateRegistry,
) -> Vec<ProcedureTemplate> {
    let templates = registry
        .list_templates(Some(TemplateCategory::WorkSurface))
        .await;

    templates
        .into_iter()
        .filter(|t| t.type_key == "config.procedure_template")
        .filter_map(|t| serde_json::from_value(t.content).ok())
        .collect()
}

fn resolve_oracle_suites_for_stage(
    _registry: &OracleSuiteRegistry,
    template: &sr_domain::work_surface::ProcedureTemplate,
    stage_id: &StageId,
) -> Vec<OracleSuiteBinding> {
    // Get the stage from the template
    let stage = match template.get_stage(stage_id) {
        Some(s) => s,
        None => return vec![],
    };

    // For each required oracle suite, create a binding
    // In a full implementation, we would look up the suite in the registry
    // and get its current hash. For now, we create placeholder bindings.
    stage
        .required_oracle_suites
        .iter()
        .map(|suite_id| OracleSuiteBinding {
            suite_id: suite_id.clone(),
            suite_hash: ContentHash::new(&format!("{:064x}", 0)), // V10-6: Fixed - ContentHash::new adds sha256: prefix
        })
        .collect()
}

/// Intake projection row for internal use
struct IntakeProjection {
    status: String,
    kind: String,
    content_hash: Option<String>,
}

async fn get_intake_projection(
    state: &AppState,
    intake_id: &str,
) -> Result<IntakeProjection, ApiError> {
    let row = sqlx::query(
        r#"
        SELECT status::text, kind::text, content_hash
        FROM proj.intakes
        WHERE intake_id = $1
        "#,
    )
    .bind(intake_id)
    .fetch_optional(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?
    .ok_or_else(|| ApiError::NotFound {
        resource: "Intake".to_string(),
        id: intake_id.to_string(),
    })?;

    use sqlx::Row;
    Ok(IntakeProjection {
        status: row.get::<String, _>("status"),
        kind: row.get::<String, _>("kind"),
        content_hash: row.get("content_hash"),
    })
}

async fn get_work_surface_projection(
    state: &AppState,
    work_surface_id: &str,
) -> Result<WorkSurfaceResponse, ApiError> {
    let row = sqlx::query(
        r#"
        SELECT work_surface_id, work_unit_id, intake_id, intake_content_hash,
               procedure_template_id, procedure_template_hash, current_stage_id,
               status::text, stage_status, current_oracle_suites, params, content_hash,
               bound_at, bound_by_kind, bound_by_id, completed_at, archived_at,
               archived_by_kind, archived_by_id
        FROM proj.work_surfaces
        WHERE work_surface_id = $1
        "#,
    )
    .bind(work_surface_id)
    .fetch_optional(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?
    .ok_or_else(|| ApiError::NotFound {
        resource: "WorkSurface".to_string(),
        id: work_surface_id.to_string(),
    })?;

    Ok(row_to_work_surface_response(&row))
}

fn row_to_work_surface_response(row: &sqlx::postgres::PgRow) -> WorkSurfaceResponse {
    use sqlx::Row;

    WorkSurfaceResponse {
        work_surface_id: row.get("work_surface_id"),
        work_unit_id: row.get("work_unit_id"),
        intake_id: row.get("intake_id"),
        intake_content_hash: row.get("intake_content_hash"),
        procedure_template_id: row.get("procedure_template_id"),
        procedure_template_hash: row.get("procedure_template_hash"),
        current_stage_id: row.get("current_stage_id"),
        status: row.get::<String, _>("status"),
        stage_status: row.get("stage_status"),
        current_oracle_suites: row.get("current_oracle_suites"),
        params: row.get("params"),
        content_hash: row.get("content_hash"),
        bound_at: row.get::<DateTime<Utc>, _>("bound_at").to_rfc3339(),
        bound_by: ActorInfo {
            kind: row.get("bound_by_kind"),
            id: row.get("bound_by_id"),
        },
        completed_at: row
            .get::<Option<DateTime<Utc>>, _>("completed_at")
            .map(|t| t.to_rfc3339()),
        archived_at: row
            .get::<Option<DateTime<Utc>>, _>("archived_at")
            .map(|t| t.to_rfc3339()),
        archived_by: row
            .get::<Option<String>, _>("archived_by_kind")
            .map(|kind| ActorInfo {
                kind,
                id: row.get("archived_by_id"),
            }),
    }
}

fn row_to_work_surface_summary(row: &sqlx::postgres::PgRow) -> WorkSurfaceSummary {
    use sqlx::Row;

    WorkSurfaceSummary {
        work_surface_id: row.get("work_surface_id"),
        work_unit_id: row.get("work_unit_id"),
        intake_id: row.get("intake_id"),
        intake_title: row.get("intake_title"),
        procedure_template_id: row.get("procedure_template_id"),
        procedure_template_name: None, // Would need to join with template registry
        current_stage_id: row.get("current_stage_id"),
        status: row.get::<String, _>("status"),
        bound_at: row.get::<DateTime<Utc>, _>("bound_at").to_rfc3339(),
    }
}

// ============================================================================
// Start Work Surface (SR-PLAN-V6 Phase V6-1)
// ============================================================================

/// Response for starting work on a Work Surface
#[derive(Debug, Serialize)]
pub struct StartWorkResponse {
    pub work_surface_id: String,
    pub loop_id: String,
    pub iteration_id: String,
    pub already_started: bool,
}

/// Start work on a Work Surface (HUMAN-callable, SYSTEM-mediated iteration)
///
/// POST /api/v1/work-surfaces/{id}/start
///
/// Per SR-PLAN-V6 §3.6-3.8:
/// - Creates Loop bound to work_unit (or reuses existing)
/// - Activates Loop if needed
/// - Starts Iteration as SYSTEM actor
/// - Idempotent: safe to retry
#[instrument(skip(state, user), fields(user_id = %user.actor_id))]
pub async fn start_work_surface(
    State(state): State<WorkSurfaceState>,
    user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<StartWorkResponse>> {
    // 1. Get work surface and verify active
    let ws = get_work_surface_projection(&state.app_state, &work_surface_id).await?;
    if ws.status != "active" {
        return Err(ApiError::PreconditionFailed {
            code: "WORK_SURFACE_NOT_ACTIVE".to_string(),
            message: format!("Work Surface status is '{}', must be 'active'", ws.status),
        });
    }

    // 2. Check for existing Loop (idempotency per §3.8)
    let existing_loop = state
        .app_state
        .projections
        .get_loop_by_work_unit(&ws.work_unit_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let (loop_id, already_started) = match existing_loop {
        Some(loop_proj) if loop_proj.state == "ACTIVE" && loop_proj.iteration_count > 0 => {
            // Already fully started - idempotent return
            let iterations = state
                .app_state
                .projections
                .get_iterations(&loop_proj.loop_id)
                .await
                .map_err(|e| ApiError::Internal {
                    message: e.to_string(),
                })?;
            let latest_iter = iterations.last().ok_or_else(|| ApiError::Internal {
                message: "Loop has iteration_count > 0 but no iterations found".to_string(),
            })?;
            return Ok(Json(StartWorkResponse {
                work_surface_id,
                loop_id: loop_proj.loop_id,
                iteration_id: latest_iter.iteration_id.clone(),
                already_started: true,
            }));
        }
        Some(loop_proj) => {
            // Loop exists but needs activation or iteration
            (loop_proj.loop_id, false)
        }
        None => {
            // Create new Loop
            let new_loop_id = create_loop_internal(&state, &ws, &user).await?;
            (new_loop_id, false)
        }
    };

    // 3. Activate Loop if not ACTIVE
    let loop_proj = state
        .app_state
        .projections
        .get_loop(&loop_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?
        .ok_or_else(|| ApiError::Internal {
            message: "Loop just created/found is missing".to_string(),
        })?;

    if loop_proj.state == "CREATED" {
        activate_loop_internal(&state.app_state, &loop_id, &user).await?;
    }

    // 4. Start Iteration as SYSTEM (per §3.6)
    let iteration_id = start_iteration_as_system(&state, &loop_id, &ws).await?;

    info!(
        work_surface_id = %work_surface_id,
        loop_id = %loop_id,
        iteration_id = %iteration_id,
        "Work started on Work Surface"
    );

    Ok(Json(StartWorkResponse {
        work_surface_id,
        loop_id,
        iteration_id,
        already_started,
    }))
}

// ============================================================================
// Work Surface Iterations (SR-PLAN-V7 Phase V7-5)
// ============================================================================

/// Get iterations for a Work Surface
///
/// GET /api/v1/work-surfaces/{id}/iterations
///
/// Per SR-PLAN-V7 §V7-5: Returns the list of iterations for the Work Surface's Loop.
#[instrument(skip(state, _user))]
pub async fn get_work_surface_iterations(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<WorkSurfaceIterationsResponse>> {
    // 1. Get work surface to find work_unit_id
    let ws = get_work_surface_projection(&state.app_state, &work_surface_id).await?;

    // 2. Get Loop for this Work Surface
    let loop_proj = state
        .app_state
        .projections
        .get_loop_by_work_unit(&ws.work_unit_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Loop".to_string(),
            id: format!("for work_unit {}", ws.work_unit_id),
        })?;

    // 3. Get iterations for this Loop
    let iterations = state
        .app_state
        .projections
        .get_iterations(&loop_proj.loop_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    // 4. Convert to response format
    let iteration_summaries: Vec<IterationSummary> = iterations
        .iter()
        .map(|iter| {
            // Extract stage_id from refs if present
            let stage_id = iter.refs.as_object().and_then(|refs| {
                // Look for ProcedureTemplate ref with current_stage_id
                if let Some(arr) = refs.get("refs").and_then(|v| v.as_array()) {
                    for r in arr {
                        if r.get("kind").and_then(|k| k.as_str()) == Some("ProcedureTemplate") {
                            return r
                                .get("meta")
                                .and_then(|m| m.get("current_stage_id"))
                                .and_then(|s| s.as_str())
                                .map(|s| s.to_string());
                        }
                    }
                }
                None
            });

            IterationSummary {
                iteration_id: iter.iteration_id.clone(),
                iteration_number: iter.sequence,
                started_at: iter.started_at.to_rfc3339(),
                completed_at: iter.completed_at.map(|t| t.to_rfc3339()),
                status: iter.state.clone(),
                stage_id,
            }
        })
        .collect();

    let total = iteration_summaries.len();

    Ok(Json(WorkSurfaceIterationsResponse {
        iterations: iteration_summaries,
        loop_id: loop_proj.loop_id,
        total,
    }))
}

/// Start a new iteration for a Work Surface
///
/// POST /api/v1/work-surfaces/{id}/iterations
///
/// Per SR-PLAN-V7 §V7-5: Starts a new iteration using SYSTEM actor mediation.
/// Requires that the current iteration is completed or a stop trigger was fired.
#[instrument(skip(state, _user), fields(work_surface_id = %work_surface_id))]
pub async fn start_work_surface_iteration(
    State(state): State<WorkSurfaceState>,
    _user: AuthenticatedUser,
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<StartIterationResponse>> {
    // 1. Get work surface and verify active
    let ws = get_work_surface_projection(&state.app_state, &work_surface_id).await?;
    if ws.status != "active" {
        return Err(ApiError::PreconditionFailed {
            code: "WORK_SURFACE_NOT_ACTIVE".to_string(),
            message: format!("Work Surface status is '{}', must be 'active'", ws.status),
        });
    }

    // 2. Get Loop for this Work Surface
    let loop_proj = state
        .app_state
        .projections
        .get_loop_by_work_unit(&ws.work_unit_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?
        .ok_or_else(|| ApiError::PreconditionFailed {
            code: "NO_LOOP".to_string(),
            message: "Work Surface has no Loop. Call /start first.".to_string(),
        })?;

    // 3. Verify Loop is ACTIVE
    if loop_proj.state != "ACTIVE" {
        return Err(ApiError::PreconditionFailed {
            code: "LOOP_NOT_ACTIVE".to_string(),
            message: format!("Loop state is '{}', must be 'ACTIVE'", loop_proj.state),
        });
    }

    // 3b. V10-1: Check budget before starting iteration (C-LOOP-1, C-LOOP-3)
    let max_iterations = loop_proj.budgets["max_iterations"].as_i64().unwrap_or(5) as i32;
    if loop_proj.iteration_count >= max_iterations {
        // Emit StopTriggered event and return error
        emit_stop_triggered(
            &state.app_state,
            &loop_proj.loop_id,
            "BUDGET_EXHAUSTED",
            true, // requires_decision = true
        )
        .await?;

        return Err(ApiError::PreconditionFailed {
            code: "BUDGET_EXHAUSTED".to_string(),
            message: format!(
                "Loop has reached max_iterations ({}/{}). Decision required to extend budget.",
                loop_proj.iteration_count, max_iterations
            ),
        });
    }

    // 3c. V10-1: Check consecutive failures for REPEATED_FAILURE trigger (C-LOOP-3)
    if loop_proj.consecutive_failures >= 3 {
        // Emit StopTriggered event and return error
        emit_stop_triggered(
            &state.app_state,
            &loop_proj.loop_id,
            "REPEATED_FAILURE",
            true, // requires_decision = true
        )
        .await?;

        return Err(ApiError::PreconditionFailed {
            code: "REPEATED_FAILURE".to_string(),
            message: format!(
                "Loop has {} consecutive failed iterations. Decision required to continue.",
                loop_proj.consecutive_failures
            ),
        });
    }

    // 4. Get current iterations and check if new iteration is allowed
    let iterations = state
        .app_state
        .projections
        .get_iterations(&loop_proj.loop_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    if let Some(last_iter) = iterations.last() {
        // Current iteration must be completed or failed to start a new one
        let can_start_new = last_iter.state == "COMPLETED" || last_iter.state == "FAILED";
        if !can_start_new {
            return Err(ApiError::PreconditionFailed {
                code: "ITERATION_IN_PROGRESS".to_string(),
                message: format!(
                    "Current iteration {} is '{}'. Complete it before starting a new one.",
                    last_iter.iteration_id, last_iter.state
                ),
            });
        }
    }

    // 5. Start new iteration as SYSTEM (reuse existing helper)
    let iteration_id = start_iteration_as_system(&state, &loop_proj.loop_id, &ws).await?;

    // 6. Get the new iteration's sequence number
    let new_iterations = state
        .app_state
        .projections
        .get_iterations(&loop_proj.loop_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    let iteration_number = new_iterations
        .iter()
        .find(|i| i.iteration_id == iteration_id)
        .map(|i| i.sequence)
        .unwrap_or(new_iterations.len() as i32);

    info!(
        work_surface_id = %work_surface_id,
        loop_id = %loop_proj.loop_id,
        iteration_id = %iteration_id,
        iteration_number = iteration_number,
        "New iteration started for Work Surface"
    );

    Ok(Json(StartIterationResponse {
        iteration_id,
        iteration_number,
    }))
}

/// Create Loop with default directive_ref (per SR-PLAN-V6 §3.7)
async fn create_loop_internal(
    state: &WorkSurfaceState,
    ws: &WorkSurfaceResponse,
    user: &AuthenticatedUser,
) -> Result<String, ApiError> {
    let loop_id = LoopId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    // Default directive_ref per §3.7
    let directive_ref = serde_json::json!({
        "kind": "doc",
        "id": "SR-DIRECTIVE",
        "rel": "governs",
        "meta": {}
    });

    let payload = serde_json::json!({
        "goal": format!("Process work surface {}", ws.work_surface_id),
        "work_unit": ws.work_unit_id,
        "work_surface_id": ws.work_surface_id,
        "budgets": {
            "max_iterations": 5,
            "max_oracle_runs": 25,
            "max_wallclock_hours": 16
        },
        "directive_ref": directive_ref
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.as_str().to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: 1,
        global_seq: None,
        event_type: "LoopCreated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(), // HUMAN creates the Loop
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![TypedRef {
            kind: "Loop".to_string(),
            id: loop_id.as_str().to_string(),
            rel: "self".to_string(),
            meta: serde_json::Value::Null,
        }],
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .app_state
        .event_store
        .append(loop_id.as_str(), 0, vec![event])
        .await?;

    state
        .app_state
        .projections
        .process_events(&*state.app_state.event_store)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    info!(loop_id = %loop_id.as_str(), "Loop created for Work Surface");

    Ok(loop_id.as_str().to_string())
}

/// Activate a loop (CREATED -> ACTIVE)
async fn activate_loop_internal(
    state: &AppState,
    loop_id: &str,
    user: &AuthenticatedUser,
) -> Result<(), ApiError> {
    let event_id = EventId::new();
    let now = Utc::now();

    // Read current stream to get version
    let events = state.event_store.read_stream(loop_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "LoopActivated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(),
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({}),
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(loop_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    info!(loop_id = %loop_id, "Loop activated");

    Ok(())
}

/// Start iteration as SYSTEM actor (per SR-PLAN-V6 §3.6)
async fn start_iteration_as_system(
    state: &WorkSurfaceState,
    loop_id: &str,
    ws: &WorkSurfaceResponse,
) -> Result<String, ApiError> {
    // Per §3.6: Use SYSTEM actor for iteration start
    let system_actor_kind = ActorKind::System;
    let system_actor_id = "system:work-surface-start".to_string();

    let iteration_id = IterationId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    // Get iteration sequence
    let iterations = state
        .app_state
        .projections
        .get_iterations(loop_id)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;
    let sequence = (iterations.len() + 1) as u32;

    // Fetch Work Surface refs for iteration context (V10-G4: include Loop ref)
    let refs = fetch_work_surface_refs(ws, loop_id);

    let payload = serde_json::json!({
        "loop_id": loop_id,
        "sequence": sequence,
        "work_surface_id": ws.work_surface_id,
        "refs": refs
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: iteration_id.as_str().to_string(),
        stream_kind: StreamKind::Iteration,
        stream_seq: 1,
        global_seq: None,
        event_type: "IterationStarted".to_string(),
        occurred_at: now,
        actor_kind: system_actor_kind, // SYSTEM per §3.6
        actor_id: system_actor_id,
        correlation_id: Some(loop_id.to_string()),
        causation_id: None,
        supersedes: vec![],
        refs,
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .app_state
        .event_store
        .append(iteration_id.as_str(), 0, vec![event])
        .await?;

    state
        .app_state
        .projections
        .process_events(&*state.app_state.event_store)
        .await
        .map_err(|e| ApiError::Internal {
            message: e.to_string(),
        })?;

    info!(
        iteration_id = %iteration_id.as_str(),
        loop_id = %loop_id,
        sequence = sequence,
        "Iteration started as SYSTEM"
    );

    Ok(iteration_id.as_str().to_string())
}

/// Fetch Work Surface refs for iteration context (per C-CTX-1/C-CTX-2)
///
/// V10-G4: Includes Loop ref with rel="in_scope_of" per SR-PLAN-LOOPS Test 9.
fn fetch_work_surface_refs(ws: &WorkSurfaceResponse, loop_id: &str) -> Vec<TypedRef> {
    let mut refs = Vec::new();

    // 0. V10-G4: Loop ref (required by C-CTX-1)
    refs.push(TypedRef {
        kind: "Loop".to_string(),
        id: loop_id.to_string(),
        rel: "in_scope_of".to_string(),
        meta: serde_json::Value::Null,
    });

    // 1. Intake ref
    refs.push(TypedRef {
        kind: "Intake".to_string(),
        id: ws.intake_id.clone(),
        rel: "depends_on".to_string(),
        meta: serde_json::json!({
            "content_hash": ws.intake_content_hash,
        }),
    });

    // 2. Procedure Template ref
    refs.push(TypedRef {
        kind: "ProcedureTemplate".to_string(),
        id: ws.procedure_template_id.clone(),
        rel: "depends_on".to_string(),
        meta: serde_json::json!({
            "content_hash": ws.procedure_template_hash,
            "current_stage_id": ws.current_stage_id,
        }),
    });

    // 3. Oracle Suite refs for current stage
    let suites: Vec<OracleSuiteBinding> =
        serde_json::from_value(ws.current_oracle_suites.clone()).unwrap_or_default();

    for suite in suites {
        refs.push(TypedRef {
            kind: "OracleSuite".to_string(),
            id: suite.suite_id.clone(),
            rel: "depends_on".to_string(),
            meta: serde_json::json!({
                "content_hash": suite.suite_hash.as_str(),
            }),
        });
    }

    refs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_work_kind() {
        assert!(matches!(
            parse_work_kind("research_memo").unwrap(),
            WorkKind::ResearchMemo
        ));
        assert!(matches!(
            parse_work_kind("decision_record").unwrap(),
            WorkKind::DecisionRecord
        ));
    }

    #[test]
    fn test_parse_gate_result_status() {
        assert!(matches!(
            parse_gate_result_status("PASS").unwrap(),
            GateResultStatus::Pass
        ));
        assert!(matches!(
            parse_gate_result_status("pass_with_waivers").unwrap(),
            GateResultStatus::PassWithWaivers
        ));
        assert!(matches!(
            parse_gate_result_status("FAIL").unwrap(),
            GateResultStatus::Fail
        ));
    }

    #[test]
    fn test_default_query_params() {
        assert_eq!(default_page(), 1);
        assert_eq!(default_page_size(), 20);
    }
}
