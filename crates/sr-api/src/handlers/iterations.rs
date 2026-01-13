//! Iteration API Handlers (D-18)
//!
//! Per SR-SPEC §2.2: IterationStarted events MUST be emitted by SYSTEM actor only.
//! Iteration queries are available to all authenticated users.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::IterationProjection;
use sr_domain::{
    ActorKind, EventEnvelope, EventId, IterationId, IterationSummary, StreamKind, TypedRef,
};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to start an iteration (SYSTEM-only)
#[derive(Debug, Deserialize)]
pub struct StartIterationRequest {
    pub loop_id: String,
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

/// Request to complete an iteration
#[derive(Debug, Deserialize)]
pub struct CompleteIterationRequest {
    pub outcome: String, // "SUCCESS" or "FAILURE"
    #[serde(default)]
    pub summary: Option<IterationSummaryRequest>,
}

#[derive(Debug, Deserialize)]
pub struct IterationSummaryRequest {
    pub intent: String,
    #[serde(default)]
    pub actions: Vec<ActionRequest>,
    #[serde(default)]
    pub artifacts_touched: Vec<String>,
    #[serde(default)]
    pub candidates_produced: Vec<String>,
    #[serde(default)]
    pub runs_executed: Vec<String>,
    #[serde(default)]
    pub next_steps: Vec<NextStepRequest>,
    #[serde(default)]
    pub open_risks: Vec<OpenRiskRequest>,
}

#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub kind: String,
    pub summary: String,
    #[serde(default)]
    pub artifacts: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct NextStepRequest {
    pub kind: String,
    pub description: String,
    #[serde(default)]
    pub blocking: bool,
}

#[derive(Debug, Deserialize)]
pub struct OpenRiskRequest {
    pub severity: String,
    pub description: String,
    #[serde(default)]
    pub mitigation: String,
}

/// Response for a single iteration
#[derive(Debug, Serialize)]
pub struct IterationResponse {
    pub iteration_id: String,
    pub loop_id: String,
    pub sequence: i32,
    pub state: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub refs: serde_json::Value,
    pub summary: Option<serde_json::Value>,
}

/// Query parameters for listing iterations
#[derive(Debug, Deserialize)]
pub struct ListIterationsQuery {
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

/// Response for listing iterations
#[derive(Debug, Serialize)]
pub struct ListIterationsResponse {
    pub iterations: Vec<IterationResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for iteration actions
#[derive(Debug, Serialize)]
pub struct IterationActionResponse {
    pub iteration_id: String,
    pub loop_id: String,
    pub state: String,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Start a new iteration (SYSTEM-only per SR-SPEC §2.2)
///
/// POST /api/v1/iterations
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn start_iteration(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<StartIterationRequest>,
) -> ApiResult<Json<IterationActionResponse>> {
    // Enforce SYSTEM-only per SR-SPEC §2.2
    if !matches!(user.actor_kind, ActorKind::System) {
        return Err(ApiError::Forbidden {
            message: "IterationStarted events MUST be emitted by SYSTEM actor only (SR-SPEC §2.2)"
                .to_string(),
        });
    }

    // Verify loop exists and is ACTIVE
    let loop_projection = state
        .projections
        .get_loop(&body.loop_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Loop".to_string(),
            id: body.loop_id.clone(),
        })?;

    if loop_projection.state != "ACTIVE" {
        return Err(ApiError::InvalidTransition {
            current_state: loop_projection.state,
            action: "start iteration".to_string(),
        });
    }

    // Get next sequence number
    let iterations = state.projections.get_iterations(&body.loop_id).await?;
    let next_sequence = (iterations.len() + 1) as u32;

    let iteration_id = IterationId::new();
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
        "loop_id": body.loop_id,
        "sequence": next_sequence,
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
        .append(iteration_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        iteration_id = %iteration_id.as_str(),
        loop_id = %body.loop_id,
        sequence = next_sequence,
        "Iteration started"
    );

    Ok(Json(IterationActionResponse {
        iteration_id: iteration_id.as_str().to_string(),
        loop_id: body.loop_id,
        state: "STARTED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get an iteration by ID
///
/// GET /api/v1/iterations/{iteration_id}
#[instrument(skip(state, _user))]
pub async fn get_iteration(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(iteration_id): Path<String>,
) -> ApiResult<Json<IterationResponse>> {
    let projection = state
        .projections
        .get_iteration(&iteration_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Iteration".to_string(),
            id: iteration_id.clone(),
        })?;

    Ok(Json(projection_to_response(projection)))
}

/// List iterations for a loop
///
/// GET /api/v1/loops/{loop_id}/iterations
#[instrument(skip(state, _user))]
pub async fn list_iterations_for_loop(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(loop_id): Path<String>,
    Query(_query): Query<ListIterationsQuery>,
) -> ApiResult<Json<ListIterationsResponse>> {
    // Verify loop exists
    let _ = state
        .projections
        .get_loop(&loop_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Loop".to_string(),
            id: loop_id.clone(),
        })?;

    let iterations = state.projections.get_iterations(&loop_id).await?;
    let responses: Vec<IterationResponse> = iterations
        .into_iter()
        .map(projection_to_response)
        .collect();

    Ok(Json(ListIterationsResponse {
        total: responses.len(),
        iterations: responses,
        limit: 1000,
        offset: 0,
    }))
}

/// Complete an iteration
///
/// POST /api/v1/iterations/{iteration_id}/complete
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn complete_iteration(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(iteration_id): Path<String>,
    Json(body): Json<CompleteIterationRequest>,
) -> ApiResult<Json<IterationActionResponse>> {
    let projection = state
        .projections
        .get_iteration(&iteration_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Iteration".to_string(),
            id: iteration_id.clone(),
        })?;

    // Can only complete from STARTED or RUNNING
    if !["STARTED", "RUNNING"].contains(&projection.state.as_str()) {
        return Err(ApiError::InvalidTransition {
            current_state: projection.state,
            action: "complete".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    // Read current stream to get version
    let events = state
        .event_store
        .read_stream(&iteration_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let payload = serde_json::json!({
        "outcome": body.outcome,
        "summary": body.summary
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: iteration_id.clone(),
        stream_kind: StreamKind::Iteration,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "IterationCompleted".to_string(),
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
        .append(&iteration_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    let new_state = if body.outcome == "SUCCESS" {
        "COMPLETED"
    } else {
        "FAILED"
    };

    info!(
        iteration_id = %iteration_id,
        outcome = %body.outcome,
        "Iteration completed"
    );

    Ok(Json(IterationActionResponse {
        iteration_id: iteration_id.clone(),
        loop_id: projection.loop_id,
        state: new_state.to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn projection_to_response(p: IterationProjection) -> IterationResponse {
    IterationResponse {
        iteration_id: p.iteration_id,
        loop_id: p.loop_id,
        sequence: p.sequence,
        state: p.state,
        started_at: p.started_at.to_rfc3339(),
        completed_at: p.completed_at.map(|t| t.to_rfc3339()),
        refs: p.refs,
        summary: p.summary,
    }
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
