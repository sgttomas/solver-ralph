//! Intake API Handlers per SR-PLAN-V3 Phase 0b
//!
//! Per SR-WORK-SURFACE §3: Endpoints for creating, querying, and managing intakes.
//! Intake state transitions follow the lifecycle: Draft → Active → Archived.
//!
//! Endpoints:
//! - POST   /api/v1/intakes                    - Create draft intake
//! - GET    /api/v1/intakes                    - List with filters
//! - GET    /api/v1/intakes/:intake_id         - Get by ID
//! - GET    /api/v1/intakes/by-hash/:hash      - Get by content hash (all statuses)
//! - PUT    /api/v1/intakes/:intake_id         - Update (draft only)
//! - POST   /api/v1/intakes/:intake_id/activate - Transition to active
//! - POST   /api/v1/intakes/:intake_id/archive  - Transition to archived
//! - POST   /api/v1/intakes/:intake_id/fork     - Create new draft from active/archived

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sr_domain::intake::IntakeUlidId;
use sr_domain::{EventEnvelope, EventId, StreamKind};
use sr_ports::EventStore;
use std::collections::HashMap;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a new intake
#[derive(Debug, Deserialize)]
pub struct CreateIntakeRequest {
    pub work_unit_id: String,
    pub title: String,
    pub kind: String,
    pub objective: String,
    pub audience: String,
    pub deliverables: Vec<DeliverableRequest>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub definitions: HashMap<String, String>,
    #[serde(default)]
    pub inputs: Vec<TypedRefRequest>,
    #[serde(default)]
    pub unknowns: Vec<String>,
    #[serde(default)]
    pub completion_criteria: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeliverableRequest {
    pub name: String,
    pub format: String,
    pub path: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: RefMetaRequest,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RefMetaRequest {
    #[serde(default)]
    pub content_hash: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub type_key: Option<String>,
    #[serde(default)]
    pub selector: Option<String>,
}

/// Request to update a draft intake
#[derive(Debug, Deserialize)]
pub struct UpdateIntakeRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub objective: Option<String>,
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub deliverables: Option<Vec<DeliverableRequest>>,
    #[serde(default)]
    pub constraints: Option<Vec<String>>,
    #[serde(default)]
    pub definitions: Option<HashMap<String, String>>,
    #[serde(default)]
    pub inputs: Option<Vec<TypedRefRequest>>,
    #[serde(default)]
    pub unknowns: Option<Vec<String>>,
    #[serde(default)]
    pub completion_criteria: Option<Vec<String>>,
}

/// Request to archive an intake
#[derive(Debug, Deserialize)]
pub struct ArchiveIntakeRequest {
    #[serde(default)]
    pub reason: Option<String>,
}

/// Response for a single intake
#[derive(Debug, Serialize)]
pub struct IntakeResponse {
    pub intake_id: String,
    pub work_unit_id: String,
    pub content_hash: Option<String>,
    pub title: String,
    pub kind: String,
    pub objective: String,
    pub audience: String,
    pub deliverables: serde_json::Value,
    pub constraints: serde_json::Value,
    pub definitions: serde_json::Value,
    pub inputs: serde_json::Value,
    pub unknowns: serde_json::Value,
    pub completion_criteria: serde_json::Value,
    pub status: String,
    pub version: i32,
    pub supersedes: Option<String>,
    pub created_by: ActorInfo,
    pub created_at: String,
    pub activated_at: Option<String>,
    pub activated_by: Option<ActorInfo>,
    pub archived_at: Option<String>,
    pub archived_by: Option<ActorInfo>,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing intakes
#[derive(Debug, Deserialize)]
pub struct ListIntakesQuery {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
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

/// Response for listing intakes
#[derive(Debug, Serialize)]
pub struct ListIntakesResponse {
    pub intakes: Vec<IntakeResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}

/// Response for intake action (create, activate, archive, fork)
#[derive(Debug, Serialize)]
pub struct IntakeActionResponse {
    pub intake_id: String,
    pub status: String,
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

// ============================================================================
// Handlers
// ============================================================================

/// Create a new draft intake
///
/// POST /api/v1/intakes
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn create_intake(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateIntakeRequest>,
) -> ApiResult<Json<IntakeActionResponse>> {
    let intake_id = generate_intake_id();
    let event_id = EventId::new();
    let now = Utc::now();

    // Validate kind
    validate_work_kind(&body.kind)?;

    // Validate deliverables
    if body.deliverables.is_empty() {
        return Err(ApiError::BadRequest {
            message: "At least one deliverable is required".to_string(),
        });
    }

    // Convert inputs to StrongTypedRef for storage
    let inputs: Vec<serde_json::Value> = body
        .inputs
        .iter()
        .map(|r| {
            serde_json::json!({
                "kind": r.kind,
                "id": r.id,
                "rel": r.rel,
                "meta": {
                    "content_hash": r.meta.content_hash,
                    "version": r.meta.version,
                    "type_key": r.meta.type_key,
                    "selector": r.meta.selector,
                },
                "label": r.label,
            })
        })
        .collect();

    let deliverables: Vec<serde_json::Value> = body
        .deliverables
        .iter()
        .map(|d| {
            serde_json::json!({
                "name": d.name,
                "format": d.format,
                "path": d.path,
                "description": d.description,
            })
        })
        .collect();

    let payload = serde_json::json!({
        "intake_id": intake_id,
        "work_unit_id": body.work_unit_id,
        "title": body.title,
        "kind": body.kind,
        "objective": body.objective,
        "audience": body.audience,
        "deliverables": deliverables,
        "constraints": body.constraints,
        "definitions": body.definitions,
        "inputs": inputs,
        "unknowns": body.unknowns,
        "completion_criteria": body.completion_criteria,
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: intake_id.clone(),
        stream_kind: StreamKind::Intake,
        stream_seq: 1,
        global_seq: None,
        event_type: "IntakeCreated".to_string(),
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

    // Append event to store
    state.event_store.append(&intake_id, 0, vec![event]).await?;

    // Process projection
    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(intake_id = %intake_id, "Intake created");

    Ok(Json(IntakeActionResponse {
        intake_id,
        status: "draft".to_string(),
        event_id: event_id.as_str().to_string(),
        content_hash: None,
    }))
}

/// Get an intake by ID
///
/// GET /api/v1/intakes/:intake_id
#[instrument(skip(state, _user))]
pub async fn get_intake(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(intake_id): Path<String>,
) -> ApiResult<Json<IntakeResponse>> {
    let intake = get_intake_projection(&state, &intake_id).await?;
    Ok(Json(intake))
}

/// Get intakes by content hash (returns all statuses per C-EVID-6)
///
/// GET /api/v1/intakes/by-hash/:content_hash
#[instrument(skip(state, _user))]
pub async fn get_intake_by_hash(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(content_hash): Path<String>,
) -> ApiResult<Json<ListIntakesResponse>> {
    let rows = sqlx::query(
        r#"
        SELECT intake_id, work_unit_id, content_hash, title, kind::text, objective, audience,
               deliverables, constraints, definitions, inputs, unknowns, completion_criteria,
               status::text, version, supersedes, created_at, created_by_kind, created_by_id,
               activated_at, activated_by_kind, activated_by_id,
               archived_at, archived_by_kind, archived_by_id
        FROM proj.intakes
        WHERE content_hash = $1
        ORDER BY activated_at DESC NULLS LAST
        "#,
    )
    .bind(&content_hash)
    .fetch_all(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    let intakes: Vec<IntakeResponse> = rows.iter().map(row_to_intake_response).collect();
    let count = intakes.len();

    Ok(Json(ListIntakesResponse {
        total: count as i64,
        intakes,
        page: 1,
        page_size: count as u32,
    }))
}

/// List intakes with optional filtering
///
/// GET /api/v1/intakes
#[instrument(skip(state, _user))]
pub async fn list_intakes(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListIntakesQuery>,
) -> ApiResult<Json<ListIntakesResponse>> {
    let offset = (query.page.saturating_sub(1)) * query.page_size;

    // Build dynamic query
    // Note: kind and status are PostgreSQL enums, cast to text for Rust String compatibility
    let mut sql = String::from(
        r#"
        SELECT intake_id, work_unit_id, content_hash, title, kind::text, objective, audience,
               deliverables, constraints, definitions, inputs, unknowns, completion_criteria,
               status::text, version, supersedes, created_at, created_by_kind, created_by_id,
               activated_at, activated_by_kind, activated_by_id,
               archived_at, archived_by_kind, archived_by_id
        FROM proj.intakes
        WHERE 1=1
        "#,
    );

    let mut count_sql = String::from("SELECT COUNT(*) FROM proj.intakes WHERE 1=1");

    if query.status.is_some() {
        sql.push_str(" AND status = $1::intake_status");
        count_sql.push_str(" AND status = $1::intake_status");
    }
    if query.kind.is_some() {
        let param = if query.status.is_some() { "$2" } else { "$1" };
        sql.push_str(&format!(" AND kind = {}::work_kind", param));
        count_sql.push_str(&format!(" AND kind = {}::work_kind", param));
    }
    if query.work_unit_id.is_some() {
        let param = match (query.status.is_some(), query.kind.is_some()) {
            (true, true) => "$3",
            (true, false) | (false, true) => "$2",
            (false, false) => "$1",
        };
        sql.push_str(&format!(" AND work_unit_id = {}", param));
        count_sql.push_str(&format!(" AND work_unit_id = {}", param));
    }

    sql.push_str(" ORDER BY created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", query.page_size, offset));

    // Execute count query
    let total: i64 = {
        let mut q = sqlx::query_scalar(&count_sql);
        if let Some(ref status) = query.status {
            q = q.bind(status);
        }
        if let Some(ref kind) = query.kind {
            q = q.bind(kind);
        }
        if let Some(ref work_unit_id) = query.work_unit_id {
            q = q.bind(work_unit_id);
        }
        q.fetch_one(state.projections.pool())
            .await
            .map_err(|e| ApiError::Internal {
                message: e.to_string(),
            })?
    };

    // Execute main query
    let rows = {
        let mut q = sqlx::query(&sql);
        if let Some(ref status) = query.status {
            q = q.bind(status);
        }
        if let Some(ref kind) = query.kind {
            q = q.bind(kind);
        }
        if let Some(ref work_unit_id) = query.work_unit_id {
            q = q.bind(work_unit_id);
        }
        q.fetch_all(state.projections.pool())
            .await
            .map_err(|e| ApiError::Internal {
                message: e.to_string(),
            })?
    };

    let intakes: Vec<IntakeResponse> = rows.iter().map(row_to_intake_response).collect();

    Ok(Json(ListIntakesResponse {
        intakes,
        total,
        page: query.page,
        page_size: query.page_size,
    }))
}

/// Update a draft intake
///
/// PUT /api/v1/intakes/:intake_id
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn update_intake(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(intake_id): Path<String>,
    Json(body): Json<UpdateIntakeRequest>,
) -> ApiResult<Json<IntakeActionResponse>> {
    // Get current intake and verify it's in Draft status
    let current = get_intake_projection(&state, &intake_id).await?;

    if current.status != "draft" {
        return Err(ApiError::InvalidTransition {
            current_state: current.status,
            action: "update".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    // Build changes payload (only include changed fields)
    let mut changes = serde_json::Map::new();
    if let Some(title) = &body.title {
        changes.insert("title".to_string(), serde_json::json!(title));
    }
    if let Some(objective) = &body.objective {
        changes.insert("objective".to_string(), serde_json::json!(objective));
    }
    if let Some(audience) = &body.audience {
        changes.insert("audience".to_string(), serde_json::json!(audience));
    }
    if let Some(deliverables) = &body.deliverables {
        let deliverables_json: Vec<serde_json::Value> = deliverables
            .iter()
            .map(|d| {
                serde_json::json!({
                    "name": d.name,
                    "format": d.format,
                    "path": d.path,
                    "description": d.description,
                })
            })
            .collect();
        changes.insert(
            "deliverables".to_string(),
            serde_json::json!(deliverables_json),
        );
    }
    if let Some(constraints) = &body.constraints {
        changes.insert("constraints".to_string(), serde_json::json!(constraints));
    }
    if let Some(definitions) = &body.definitions {
        changes.insert("definitions".to_string(), serde_json::json!(definitions));
    }
    if let Some(inputs) = &body.inputs {
        let inputs_json: Vec<serde_json::Value> = inputs
            .iter()
            .map(|r| {
                serde_json::json!({
                    "kind": r.kind,
                    "id": r.id,
                    "rel": r.rel,
                    "meta": {
                        "content_hash": r.meta.content_hash,
                        "version": r.meta.version,
                        "type_key": r.meta.type_key,
                        "selector": r.meta.selector,
                    },
                    "label": r.label,
                })
            })
            .collect();
        changes.insert("inputs".to_string(), serde_json::json!(inputs_json));
    }
    if let Some(unknowns) = &body.unknowns {
        changes.insert("unknowns".to_string(), serde_json::json!(unknowns));
    }
    if let Some(completion_criteria) = &body.completion_criteria {
        changes.insert(
            "completion_criteria".to_string(),
            serde_json::json!(completion_criteria),
        );
    }

    if changes.is_empty() {
        return Err(ApiError::BadRequest {
            message: "No changes provided".to_string(),
        });
    }

    let payload = serde_json::json!({
        "intake_id": intake_id,
        "changes": changes,
    });

    // Get current stream version
    let events = state.event_store.read_stream(&intake_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: intake_id.clone(),
        stream_kind: StreamKind::Intake,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "IntakeUpdated".to_string(),
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
        .append(&intake_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(intake_id = %intake_id, "Intake updated");

    Ok(Json(IntakeActionResponse {
        intake_id,
        status: "draft".to_string(),
        event_id: event_id.as_str().to_string(),
        content_hash: None,
    }))
}

/// Activate an intake (Draft → Active)
///
/// POST /api/v1/intakes/:intake_id/activate
#[instrument(skip(state, user), fields(user_id = %user.actor_id))]
pub async fn activate_intake(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(intake_id): Path<String>,
) -> ApiResult<Json<IntakeActionResponse>> {
    // Get current intake and verify it's in Draft status
    let current = get_intake_projection(&state, &intake_id).await?;

    if current.status != "draft" {
        return Err(ApiError::InvalidTransition {
            current_state: current.status,
            action: "activate".to_string(),
        });
    }

    // Compute content hash
    let content_hash = compute_intake_content_hash(&current);

    // Check for hash collision (same content already active)
    let existing = sqlx::query_scalar::<_, String>(
        "SELECT intake_id FROM proj.intakes WHERE content_hash = $1 AND status = 'active'",
    )
    .bind(&content_hash)
    .fetch_optional(state.projections.pool())
    .await
    .map_err(|e| ApiError::Internal {
        message: e.to_string(),
    })?;

    if let Some(existing_id) = existing {
        return Err(ApiError::Conflict {
            message: format!(
                "Content hash collision: intake {} already active with same content",
                existing_id
            ),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    let payload = serde_json::json!({
        "intake_id": intake_id,
        "content_hash": content_hash,
        "canonical_json_hash": content_hash, // Same as content_hash for now
    });

    // Get current stream version
    let events = state.event_store.read_stream(&intake_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: intake_id.clone(),
        stream_kind: StreamKind::Intake,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "IntakeActivated".to_string(),
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
        .append(&intake_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(intake_id = %intake_id, content_hash = %content_hash, "Intake activated");

    Ok(Json(IntakeActionResponse {
        intake_id,
        status: "active".to_string(),
        event_id: event_id.as_str().to_string(),
        content_hash: Some(content_hash),
    }))
}

/// Archive an intake (Active → Archived)
///
/// POST /api/v1/intakes/:intake_id/archive
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn archive_intake(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(intake_id): Path<String>,
    Json(body): Json<ArchiveIntakeRequest>,
) -> ApiResult<Json<IntakeActionResponse>> {
    // Get current intake and verify it's in Active status
    let current = get_intake_projection(&state, &intake_id).await?;

    if current.status != "active" {
        return Err(ApiError::InvalidTransition {
            current_state: current.status,
            action: "archive".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    let payload = serde_json::json!({
        "intake_id": intake_id,
        "reason": body.reason,
    });

    // Get current stream version
    let events = state.event_store.read_stream(&intake_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: intake_id.clone(),
        stream_kind: StreamKind::Intake,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "IntakeArchived".to_string(),
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
        .append(&intake_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(intake_id = %intake_id, "Intake archived");

    Ok(Json(IntakeActionResponse {
        intake_id,
        status: "archived".to_string(),
        event_id: event_id.as_str().to_string(),
        content_hash: current.content_hash,
    }))
}

/// Fork an intake to create a new draft (Active/Archived → new Draft)
///
/// POST /api/v1/intakes/:intake_id/fork
#[instrument(skip(state, user), fields(user_id = %user.actor_id))]
pub async fn fork_intake(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(source_intake_id): Path<String>,
) -> ApiResult<Json<IntakeActionResponse>> {
    // Get source intake and verify it can be forked
    let source = get_intake_projection(&state, &source_intake_id).await?;

    if source.status != "active" && source.status != "archived" {
        return Err(ApiError::InvalidTransition {
            current_state: source.status,
            action: "fork".to_string(),
        });
    }

    let new_intake_id = generate_intake_id();
    let event_id = EventId::new();
    let now = Utc::now();

    let new_version = source.version + 1;

    // Create IntakeForked event for the new intake
    let fork_payload = serde_json::json!({
        "intake_id": new_intake_id,
        "source_intake_id": source_intake_id,
        "source_content_hash": source.content_hash,
        "version": new_version,
    });

    // Also need to create the full intake content for the new stream
    let create_payload = serde_json::json!({
        "intake_id": new_intake_id,
        "work_unit_id": source.work_unit_id,
        "title": source.title,
        "kind": source.kind,
        "objective": source.objective,
        "audience": source.audience,
        "deliverables": source.deliverables,
        "constraints": source.constraints,
        "definitions": source.definitions,
        "inputs": source.inputs,
        "unknowns": source.unknowns,
        "completion_criteria": source.completion_criteria,
        "supersedes": source_intake_id,
        "version": new_version,
    });

    // Create two events: IntakeCreated (with forked data) then IntakeForked
    let create_event = EventEnvelope {
        event_id: EventId::new(),
        stream_id: new_intake_id.clone(),
        stream_kind: StreamKind::Intake,
        stream_seq: 1,
        global_seq: None,
        event_type: "IntakeCreated".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind.clone(),
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: create_payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    let fork_event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: new_intake_id.clone(),
        stream_kind: StreamKind::Intake,
        stream_seq: 2,
        global_seq: None,
        event_type: "IntakeForked".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![sr_domain::TypedRef {
            kind: "Intake".to_string(),
            id: source_intake_id.clone(),
            rel: "supersedes".to_string(),
            meta: serde_json::json!({
                "content_hash": source.content_hash,
            }),
        }],
        payload: fork_payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    // Append events to new stream
    state
        .event_store
        .append(&new_intake_id, 0, vec![create_event, fork_event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        new_intake_id = %new_intake_id,
        source_intake_id = %source_intake_id,
        "Intake forked"
    );

    Ok(Json(IntakeActionResponse {
        intake_id: new_intake_id,
        status: "draft".to_string(),
        event_id: event_id.as_str().to_string(),
        content_hash: None,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn generate_intake_id() -> String {
    IntakeUlidId::new().to_string()
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

fn validate_work_kind(kind: &str) -> Result<(), ApiError> {
    let valid_kinds = [
        "research_memo",
        "decision_record",
        "ontology_build",
        "analysis_report",
        "technical_spec",
        "implementation_plan",
        "intake_processing",
        "design_document",
        "review_response",
    ];

    if valid_kinds.contains(&kind) {
        Ok(())
    } else {
        Err(ApiError::BadRequest {
            message: format!(
                "Invalid work kind '{}'. Valid kinds: {}",
                kind,
                valid_kinds.join(", ")
            ),
        })
    }
}

fn compute_intake_content_hash(intake: &IntakeResponse) -> String {
    use sha2::{Digest, Sha256};

    let canonical = serde_json::json!({
        "work_unit_id": intake.work_unit_id,
        "title": intake.title,
        "kind": intake.kind,
        "objective": intake.objective,
        "audience": intake.audience,
        "deliverables": intake.deliverables,
        "constraints": intake.constraints,
        "definitions": intake.definitions,
        "inputs": intake.inputs,
        "unknowns": intake.unknowns,
        "completion_criteria": intake.completion_criteria,
        "version": intake.version,
    });

    let canonical_str = serde_json::to_string(&canonical).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(canonical_str.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

async fn get_intake_projection(
    state: &AppState,
    intake_id: &str,
) -> Result<IntakeResponse, ApiError> {
    let row = sqlx::query(
        r#"
        SELECT intake_id, work_unit_id, content_hash, title, kind::text, objective, audience,
               deliverables, constraints, definitions, inputs, unknowns, completion_criteria,
               status::text, version, supersedes, created_at, created_by_kind, created_by_id,
               activated_at, activated_by_kind, activated_by_id,
               archived_at, archived_by_kind, archived_by_id
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

    Ok(row_to_intake_response(&row))
}

fn row_to_intake_response(row: &sqlx::postgres::PgRow) -> IntakeResponse {
    use sqlx::Row;

    IntakeResponse {
        intake_id: row.get("intake_id"),
        work_unit_id: row.get("work_unit_id"),
        content_hash: row.get("content_hash"),
        title: row.get("title"),
        kind: row.get::<String, _>("kind"),
        objective: row.get("objective"),
        audience: row.get("audience"),
        deliverables: row.get("deliverables"),
        constraints: row.get("constraints"),
        definitions: row.get("definitions"),
        inputs: row.get("inputs"),
        unknowns: row.get("unknowns"),
        completion_criteria: row.get("completion_criteria"),
        status: row.get::<String, _>("status"),
        version: row.get("version"),
        supersedes: row.get("supersedes"),
        created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
        created_by: ActorInfo {
            kind: row.get("created_by_kind"),
            id: row.get("created_by_id"),
        },
        activated_at: row
            .get::<Option<DateTime<Utc>>, _>("activated_at")
            .map(|t| t.to_rfc3339()),
        activated_by: row
            .get::<Option<String>, _>("activated_by_kind")
            .map(|kind| ActorInfo {
                kind,
                id: row.get("activated_by_id"),
            }),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_intake_id() {
        let id = generate_intake_id();
        assert!(id.starts_with("intake:"));
        assert!(id.len() > 7);
    }

    #[test]
    fn test_validate_work_kind_valid() {
        assert!(validate_work_kind("research_memo").is_ok());
        assert!(validate_work_kind("decision_record").is_ok());
        assert!(validate_work_kind("design_document").is_ok());
    }

    #[test]
    fn test_validate_work_kind_invalid() {
        assert!(validate_work_kind("invalid_kind").is_err());
        assert!(validate_work_kind("").is_err());
    }

    #[test]
    fn test_default_query_params() {
        assert_eq!(default_page(), 1);
        assert_eq!(default_page_size(), 20);
    }
}
