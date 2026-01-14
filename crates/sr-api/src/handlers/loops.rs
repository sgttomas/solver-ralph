//! Loop API Handlers (D-18)
//!
//! Per SR-SPEC §2: Endpoints for creating, querying, and transitioning loops.
//! Loop state transitions follow the state machine defined in SR-SPEC §3.1.1.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::{LoopProjection, ProjectionBuilder};
use sr_domain::{
    ActorId, ActorKind, EventEnvelope, EventId, LoopBudgets, LoopId, LoopState,
    LoopStateMachine, StreamKind, TypedRef,
};
use sr_ports::EventStore;
use tracing::{debug, info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a new loop
#[derive(Debug, Deserialize)]
pub struct CreateLoopRequest {
    pub goal: String,
    #[serde(default)]
    pub work_unit: Option<String>,
    #[serde(default)]
    pub budgets: Option<LoopBudgetsRequest>,
    pub directive_ref: TypedRefRequest,
}

#[derive(Debug, Deserialize)]
pub struct LoopBudgetsRequest {
    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,
    #[serde(default = "default_max_oracle_runs")]
    pub max_oracle_runs: u32,
    #[serde(default = "default_max_wallclock_hours")]
    pub max_wallclock_hours: u32,
}

fn default_max_iterations() -> u32 {
    5
}
fn default_max_oracle_runs() -> u32 {
    25
}
fn default_max_wallclock_hours() -> u32 {
    16
}

impl Default for LoopBudgetsRequest {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            max_oracle_runs: default_max_oracle_runs(),
            max_wallclock_hours: default_max_wallclock_hours(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// Response for a single loop
#[derive(Debug, Serialize)]
pub struct LoopResponse {
    pub loop_id: String,
    pub goal: String,
    pub work_unit: String,
    pub state: String,
    pub budgets: LoopBudgets,
    pub directive_ref: serde_json::Value,
    pub created_by: ActorInfo,
    pub created_at: String,
    pub activated_at: Option<String>,
    pub closed_at: Option<String>,
    pub iteration_count: i32,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing loops
#[derive(Debug, Deserialize)]
pub struct ListLoopsQuery {
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing loops
#[derive(Debug, Serialize)]
pub struct ListLoopsResponse {
    pub loops: Vec<LoopResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Request to transition loop state
#[derive(Debug, Deserialize)]
pub struct TransitionLoopRequest {
    #[serde(default)]
    pub rationale: Option<String>,
}

/// Response for loop creation or transition
#[derive(Debug, Serialize)]
pub struct LoopActionResponse {
    pub loop_id: String,
    pub state: String,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Create a new loop
///
/// POST /api/v1/loops
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn create_loop(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateLoopRequest>,
) -> ApiResult<Json<LoopActionResponse>> {
    let loop_id = LoopId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    let budgets = body.budgets.unwrap_or_default();
    let work_unit = body.work_unit.unwrap_or_else(|| loop_id.as_str().to_string());

    let directive_ref = TypedRef {
        kind: body.directive_ref.kind,
        id: body.directive_ref.id,
        rel: body.directive_ref.rel,
        meta: body.directive_ref.meta,
    };

    let payload = serde_json::json!({
        "goal": body.goal,
        "work_unit": work_unit,
        "budgets": {
            "max_iterations": budgets.max_iterations,
            "max_oracle_runs": budgets.max_oracle_runs,
            "max_wallclock_hours": budgets.max_wallclock_hours
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
        actor_kind: user.actor_kind,
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

    // Append event to store
    state
        .event_store
        .append(loop_id.as_str(), 0, vec![event])
        .await?;

    // Process projection
    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(loop_id = %loop_id.as_str(), "Loop created");

    Ok(Json(LoopActionResponse {
        loop_id: loop_id.as_str().to_string(),
        state: "CREATED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get a loop by ID
///
/// GET /api/v1/loops/{loop_id}
#[instrument(skip(state, _user))]
pub async fn get_loop(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(loop_id): Path<String>,
) -> ApiResult<Json<LoopResponse>> {
    let projection = state
        .projections
        .get_loop(&loop_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Loop".to_string(),
            id: loop_id.clone(),
        })?;

    Ok(Json(projection_to_response(projection)))
}

/// List loops with optional filtering
///
/// GET /api/v1/loops
#[instrument(skip(state, _user))]
pub async fn list_loops(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListLoopsQuery>,
) -> ApiResult<Json<ListLoopsResponse>> {
    let loops = state.projections.list_loops(&query.state, query.limit, query.offset).await?;

    let responses: Vec<LoopResponse> = loops.iter().map(|p| projection_to_response(p.clone())).collect();

    Ok(Json(ListLoopsResponse {
        total: responses.len(),
        loops: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

/// Activate a loop (CREATED -> ACTIVE)
///
/// POST /api/v1/loops/{loop_id}/activate
#[instrument(skip(state, user, _body), fields(user_id = %user.actor_id))]
pub async fn activate_loop(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(loop_id): Path<String>,
    Json(_body): Json<TransitionLoopRequest>,
) -> ApiResult<Json<LoopActionResponse>> {
    transition_loop(&state, &user, &loop_id, "CREATED", "ACTIVE", "LoopActivated").await
}

/// Pause a loop (ACTIVE -> PAUSED)
///
/// POST /api/v1/loops/{loop_id}/pause
#[instrument(skip(state, user, _body), fields(user_id = %user.actor_id))]
pub async fn pause_loop(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(loop_id): Path<String>,
    Json(_body): Json<TransitionLoopRequest>,
) -> ApiResult<Json<LoopActionResponse>> {
    transition_loop(&state, &user, &loop_id, "ACTIVE", "PAUSED", "LoopPaused").await
}

/// Resume a loop (PAUSED -> ACTIVE)
///
/// POST /api/v1/loops/{loop_id}/resume
#[instrument(skip(state, user, _body), fields(user_id = %user.actor_id))]
pub async fn resume_loop(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(loop_id): Path<String>,
    Json(_body): Json<TransitionLoopRequest>,
) -> ApiResult<Json<LoopActionResponse>> {
    transition_loop(&state, &user, &loop_id, "PAUSED", "ACTIVE", "LoopResumed").await
}

/// Close a loop (any state -> CLOSED)
///
/// POST /api/v1/loops/{loop_id}/close
#[instrument(skip(state, user, _body), fields(user_id = %user.actor_id))]
pub async fn close_loop(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(loop_id): Path<String>,
    Json(_body): Json<TransitionLoopRequest>,
) -> ApiResult<Json<LoopActionResponse>> {
    // Close can happen from any state
    let projection = state
        .projections
        .get_loop(&loop_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Loop".to_string(),
            id: loop_id.clone(),
        })?;

    if projection.state == "CLOSED" {
        return Err(ApiError::InvalidTransition {
            current_state: projection.state,
            action: "close".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    // Read current stream to get version
    let events = state
        .event_store
        .read_stream(&loop_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.clone(),
        stream_kind: StreamKind::Loop,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "LoopClosed".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
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
        .append(&loop_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(loop_id = %loop_id, "Loop closed");

    Ok(Json(LoopActionResponse {
        loop_id,
        state: "CLOSED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

async fn transition_loop(
    state: &AppState,
    user: &AuthenticatedUser,
    loop_id: &str,
    expected_state: &str,
    new_state: &str,
    event_type: &str,
) -> ApiResult<Json<LoopActionResponse>> {
    let projection = state
        .projections
        .get_loop(loop_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Loop".to_string(),
            id: loop_id.to_string(),
        })?;

    if projection.state != expected_state {
        return Err(ApiError::InvalidTransition {
            current_state: projection.state,
            action: format!("transition to {}", new_state),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    // Read current stream to get version
    let events = state
        .event_store
        .read_stream(loop_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: loop_id.to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: event_type.to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
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
        .await?;

    info!(loop_id = %loop_id, new_state = %new_state, "Loop state transitioned");

    Ok(Json(LoopActionResponse {
        loop_id: loop_id.to_string(),
        state: new_state.to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

fn projection_to_response(p: LoopProjection) -> LoopResponse {
    let budgets: LoopBudgets = serde_json::from_value(p.budgets.clone()).unwrap_or_default();

    LoopResponse {
        loop_id: p.loop_id,
        goal: p.goal,
        work_unit: p.work_unit,
        state: p.state,
        budgets,
        directive_ref: p.directive_ref,
        created_by: ActorInfo {
            kind: p.created_by_kind,
            id: p.created_by_id,
        },
        created_at: p.created_at.to_rfc3339(),
        activated_at: p.activated_at.map(|t| t.to_rfc3339()),
        closed_at: p.closed_at.map(|t| t.to_rfc3339()),
        iteration_count: p.iteration_count,
    }
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    // In production, this would be a proper content hash
    // For now, use event_id as a placeholder
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_budgets() {
        let budgets = LoopBudgetsRequest::default();
        assert_eq!(budgets.max_iterations, 5);
        assert_eq!(budgets.max_oracle_runs, 25);
        assert_eq!(budgets.max_wallclock_hours, 16);
    }
}
