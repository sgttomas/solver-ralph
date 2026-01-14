//! Run API Handlers (D-18)
//!
//! Per SR-SPEC: Runs are oracle suite executions against a candidate.
//! RunStarted creates a new oracle run, RunCompleted records the result.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::RunProjection;
use sr_domain::{ContentHash, EventEnvelope, EventId, RunId, StreamKind};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to start a new oracle run
#[derive(Debug, Deserialize)]
pub struct StartRunRequest {
    /// Candidate being evaluated
    pub candidate_id: String,
    /// Oracle suite to run
    pub oracle_suite_id: String,
    /// Content hash of the oracle suite
    pub oracle_suite_hash: String,
}

/// Request to complete an oracle run
#[derive(Debug, Deserialize)]
pub struct CompleteRunRequest {
    /// Outcome: "SUCCESS" or "FAILURE"
    pub outcome: String,
    /// Evidence bundle hash (optional)
    #[serde(default)]
    pub evidence_bundle_hash: Option<String>,
}

/// Response for a single run
#[derive(Debug, Serialize)]
pub struct RunResponse {
    pub run_id: String,
    pub candidate_id: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub state: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub actor: ActorInfo,
    pub evidence_bundle_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing runs
#[derive(Debug, Deserialize)]
pub struct ListRunsQuery {
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

/// Response for listing runs
#[derive(Debug, Serialize)]
pub struct ListRunsResponse {
    pub runs: Vec<RunResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for run actions
#[derive(Debug, Serialize)]
pub struct RunActionResponse {
    pub run_id: String,
    pub candidate_id: String,
    pub state: String,
    pub event_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Start a new oracle run
///
/// POST /api/v1/runs
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn start_run(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<StartRunRequest>,
) -> ApiResult<Json<RunActionResponse>> {
    // Verify candidate exists
    let _ = state
        .projections
        .get_candidate(&body.candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: body.candidate_id.clone(),
        })?;

    let run_id = RunId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    let payload = serde_json::json!({
        "candidate_id": body.candidate_id,
        "oracle_suite_id": body.oracle_suite_id,
        "oracle_suite_hash": body.oracle_suite_hash
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: run_id.as_str().to_string(),
        stream_kind: StreamKind::Run,
        stream_seq: 1,
        global_seq: None,
        event_type: "RunStarted".to_string(),
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
        .append(run_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        run_id = %run_id.as_str(),
        candidate_id = %body.candidate_id,
        oracle_suite_id = %body.oracle_suite_id,
        "Run started"
    );

    Ok(Json(RunActionResponse {
        run_id: run_id.as_str().to_string(),
        candidate_id: body.candidate_id,
        state: "STARTED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get a run by ID
///
/// GET /api/v1/runs/{run_id}
#[instrument(skip(state, _user))]
pub async fn get_run(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(run_id): Path<String>,
) -> ApiResult<Json<RunResponse>> {
    let projection = state
        .projections
        .get_run(&run_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Run".to_string(),
            id: run_id.clone(),
        })?;

    Ok(Json(projection_to_response(projection)))
}

/// List runs for a candidate
///
/// GET /api/v1/candidates/{candidate_id}/runs
#[instrument(skip(state, _user))]
pub async fn list_runs_for_candidate(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(candidate_id): Path<String>,
    Query(_query): Query<ListRunsQuery>,
) -> ApiResult<Json<ListRunsResponse>> {
    // Verify candidate exists
    let _ = state
        .projections
        .get_candidate(&candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: candidate_id.clone(),
        })?;

    let runs = state
        .projections
        .get_runs_for_candidate(&candidate_id)
        .await?;

    let responses: Vec<RunResponse> = runs.into_iter().map(projection_to_response).collect();

    Ok(Json(ListRunsResponse {
        total: responses.len(),
        runs: responses,
        limit: 1000,
        offset: 0,
    }))
}

/// List all runs
///
/// GET /api/v1/runs
#[instrument(skip(state, _user))]
pub async fn list_runs(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListRunsQuery>,
) -> ApiResult<Json<ListRunsResponse>> {
    let runs = state
        .projections
        .list_runs(query.state.as_deref(), query.limit, query.offset)
        .await?;

    let responses: Vec<RunResponse> = runs.into_iter().map(projection_to_response).collect();

    Ok(Json(ListRunsResponse {
        total: responses.len(),
        runs: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

/// Complete a run
///
/// POST /api/v1/runs/{run_id}/complete
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn complete_run(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(run_id): Path<String>,
    Json(body): Json<CompleteRunRequest>,
) -> ApiResult<Json<RunActionResponse>> {
    let projection = state
        .projections
        .get_run(&run_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Run".to_string(),
            id: run_id.clone(),
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
    let events = state.event_store.read_stream(&run_id, 0, 1000).await?;
    let current_version = events.len() as u64;

    let payload = serde_json::json!({
        "outcome": body.outcome,
        "evidence_bundle_hash": body.evidence_bundle_hash
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: run_id.clone(),
        stream_kind: StreamKind::Run,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "RunCompleted".to_string(),
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
        .append(&run_id, current_version, vec![event])
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
        run_id = %run_id,
        outcome = %body.outcome,
        "Run completed"
    );

    Ok(Json(RunActionResponse {
        run_id: run_id.clone(),
        candidate_id: projection.candidate_id,
        state: new_state.to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn projection_to_response(p: RunProjection) -> RunResponse {
    RunResponse {
        run_id: p.run_id,
        candidate_id: p.candidate_id,
        oracle_suite_id: p.oracle_suite_id,
        oracle_suite_hash: p.oracle_suite_hash,
        state: p.state,
        started_at: p.started_at.to_rfc3339(),
        completed_at: p.completed_at.map(|t| t.to_rfc3339()),
        actor: ActorInfo {
            kind: p.actor_kind,
            id: p.actor_id,
        },
        evidence_bundle_hash: p.evidence_bundle_hash,
    }
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
