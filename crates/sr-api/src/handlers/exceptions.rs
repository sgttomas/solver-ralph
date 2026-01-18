//! Exception API Handlers (D-19)
//!
//! Per SR-CONTRACT: Exceptions (Deviation, Deferral, Waiver) are binding
//! human-authorized departures from governing documents or oracle requirements.
//! Exception creator actor.kind MUST be HUMAN.
//! Waivers MUST NOT target integrity conditions.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sr_domain::{ActorKind, EventEnvelope, EventId, ExceptionId, StreamKind, TypedRef};
use sr_ports::EventStore;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::ref_validation::normalize_and_validate_refs;
use crate::AppState;

// Integrity conditions that cannot be waived
const INTEGRITY_CONDITIONS: [&str; 5] = [
    "ORACLE_TAMPER",
    "ORACLE_GAP",
    "ORACLE_ENV_MISMATCH",
    "ORACLE_FLAKE",
    "EVIDENCE_MISSING",
];

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create an exception
#[derive(Debug, Deserialize)]
pub struct CreateExceptionRequest {
    /// Exception kind: DEVIATION, DEFERRAL, WAIVER
    pub kind: String,
    /// Scope - what the exception applies to
    pub scope: ExceptionScopeRequest,
    /// Rationale for the exception
    pub rationale: String,
    /// Description of the target being excepted
    #[serde(default)]
    pub target_description: String,
    /// Expiration time (optional)
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExceptionScopeRequest {
    #[serde(default)]
    pub loop_id: Option<String>,
    #[serde(default)]
    pub candidate_id: Option<String>,
    #[serde(default)]
    pub oracle_id: Option<String>,
    #[serde(default)]
    pub artifact_refs: Vec<TypedRefRequest>,
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// Response for a single exception
#[derive(Debug, Serialize)]
pub struct ExceptionResponse {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
    pub scope: serde_json::Value,
    pub rationale: String,
    pub target_description: String,
    pub created_by: ActorInfo,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<ActorInfo>,
}

#[derive(Debug, Serialize)]
pub struct ActorInfo {
    pub kind: String,
    pub id: String,
}

/// Query parameters for listing exceptions
#[derive(Debug, Deserialize)]
pub struct ListExceptionsQuery {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// Response for listing exceptions
#[derive(Debug, Serialize)]
pub struct ListExceptionsResponse {
    pub exceptions: Vec<ExceptionResponse>,
    pub total: usize,
    pub limit: u32,
    pub offset: u32,
}

/// Response for exception creation/transition
#[derive(Debug, Serialize)]
pub struct ExceptionActionResponse {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
    pub event_id: String,
}

/// Request to resolve an exception
#[derive(Debug, Deserialize)]
pub struct ResolveExceptionRequest {
    #[serde(default)]
    pub resolution_notes: Option<String>,
}

// ============================================================================
// Handlers
// ============================================================================

/// Create an exception (HUMAN-only)
///
/// POST /api/v1/exceptions
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn create_exception(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateExceptionRequest>,
) -> ApiResult<Json<ExceptionActionResponse>> {
    // Enforce HUMAN-only
    if !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "Exceptions MUST be created by HUMAN actors only".to_string(),
        });
    }

    // Validate kind
    let valid_kinds = ["DEVIATION", "DEFERRAL", "WAIVER"];
    if !valid_kinds.contains(&body.kind.as_str()) {
        return Err(ApiError::BadRequest {
            message: format!(
                "Invalid exception kind '{}'. Must be one of: {:?}",
                body.kind, valid_kinds
            ),
        });
    }

    // For waivers, validate target is not an integrity condition
    if body.kind == "WAIVER" {
        if let Some(ref oracle_id) = body.scope.oracle_id {
            for condition in INTEGRITY_CONDITIONS {
                if oracle_id.contains(condition) || body.target_description.contains(condition) {
                    return Err(ApiError::BadRequest {
                        message: format!(
                            "Waivers MUST NOT target integrity conditions (found: {})",
                            condition
                        ),
                    });
                }
            }
        }
    }

    let exception_id = ExceptionId::new();
    let event_id = EventId::new();
    let now = Utc::now();

    // Parse expiration if provided
    let expires_at = body
        .expires_at
        .as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    // Convert scope to artifact refs
    let artifact_refs_raw: Vec<TypedRef> = body
        .scope
        .artifact_refs
        .into_iter()
        .map(|r| TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        })
        .collect();
    let artifact_refs = normalize_and_validate_refs(&state, artifact_refs_raw).await?;

    let scope = serde_json::json!({
        "loop_id": body.scope.loop_id,
        "candidate_id": body.scope.candidate_id,
        "oracle_id": body.scope.oracle_id,
        "artifact_refs": artifact_refs
    });

    let event_type = match body.kind.as_str() {
        "DEVIATION" => "DeviationCreated",
        "DEFERRAL" => "DeferralCreated",
        "WAIVER" => "WaiverCreated",
        _ => "DeviationCreated",
    };

    let payload = serde_json::json!({
        "scope": scope,
        "rationale": body.rationale,
        "target_description": body.target_description,
        "expires_at": expires_at
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: exception_id.as_str().to_string(),
        stream_kind: StreamKind::Exception,
        stream_seq: 1,
        global_seq: None,
        event_type: event_type.to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: artifact_refs,
        payload,
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(exception_id.as_str(), 0, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        exception_id = %exception_id.as_str(),
        kind = %body.kind,
        "Exception created"
    );

    Ok(Json(ExceptionActionResponse {
        exception_id: exception_id.as_str().to_string(),
        kind: body.kind,
        status: "CREATED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

/// Get an exception by ID
///
/// GET /api/v1/exceptions/{exception_id}
#[instrument(skip(state, _user))]
pub async fn get_exception(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(exception_id): Path<String>,
) -> ApiResult<Json<ExceptionResponse>> {
    let projection = state
        .projections
        .get_exception(&exception_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Exception".to_string(),
            id: exception_id.clone(),
        })?;

    Ok(Json(ExceptionResponse {
        exception_id: projection.exception_id,
        kind: projection.kind,
        status: projection.status,
        scope: projection.scope,
        rationale: projection.rationale,
        target_description: projection.target_description,
        created_by: ActorInfo {
            kind: projection.created_by_kind,
            id: projection.created_by_id,
        },
        created_at: projection.created_at.to_rfc3339(),
        expires_at: projection.expires_at.map(|t| t.to_rfc3339()),
        resolved_at: projection.resolved_at.map(|t| t.to_rfc3339()),
        resolved_by: projection.resolved_by_kind.map(|kind| ActorInfo {
            kind,
            id: projection.resolved_by_id.unwrap_or_default(),
        }),
    }))
}

/// List exceptions
///
/// GET /api/v1/exceptions
#[instrument(skip(state, _user))]
pub async fn list_exceptions(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListExceptionsQuery>,
) -> ApiResult<Json<ListExceptionsResponse>> {
    let exceptions = state
        .projections
        .list_exceptions(
            query.kind.as_deref(),
            query.status.as_deref(),
            query.limit,
            query.offset,
        )
        .await?;

    let responses: Vec<ExceptionResponse> = exceptions
        .into_iter()
        .map(|p| ExceptionResponse {
            exception_id: p.exception_id,
            kind: p.kind,
            status: p.status,
            scope: p.scope,
            rationale: p.rationale,
            target_description: p.target_description,
            created_by: ActorInfo {
                kind: p.created_by_kind,
                id: p.created_by_id,
            },
            created_at: p.created_at.to_rfc3339(),
            expires_at: p.expires_at.map(|t| t.to_rfc3339()),
            resolved_at: p.resolved_at.map(|t| t.to_rfc3339()),
            resolved_by: p.resolved_by_kind.map(|kind| ActorInfo {
                kind,
                id: p.resolved_by_id.unwrap_or_default(),
            }),
        })
        .collect();

    Ok(Json(ListExceptionsResponse {
        total: responses.len(),
        exceptions: responses,
        limit: query.limit,
        offset: query.offset,
    }))
}

/// Activate an exception (CREATED -> ACTIVE)
///
/// POST /api/v1/exceptions/{exception_id}/activate
#[instrument(skip(state, user), fields(user_id = %user.actor_id))]
pub async fn activate_exception(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(exception_id): Path<String>,
) -> ApiResult<Json<ExceptionActionResponse>> {
    let projection = state
        .projections
        .get_exception(&exception_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Exception".to_string(),
            id: exception_id.clone(),
        })?;

    if projection.status != "CREATED" {
        return Err(ApiError::InvalidTransition {
            current_state: projection.status,
            action: "activate".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    let events = state
        .event_store
        .read_stream(&exception_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: exception_id.clone(),
        stream_kind: StreamKind::Exception,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "ExceptionActivated".to_string(),
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
        .append(&exception_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(exception_id = %exception_id, "Exception activated");

    Ok(Json(ExceptionActionResponse {
        exception_id: exception_id.clone(),
        kind: projection.kind,
        status: "ACTIVE".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

/// Resolve an exception (ACTIVE -> RESOLVED)
///
/// POST /api/v1/exceptions/{exception_id}/resolve
#[instrument(skip(state, user, _body), fields(user_id = %user.actor_id))]
pub async fn resolve_exception(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(exception_id): Path<String>,
    Json(_body): Json<ResolveExceptionRequest>,
) -> ApiResult<Json<ExceptionActionResponse>> {
    let projection = state
        .projections
        .get_exception(&exception_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Exception".to_string(),
            id: exception_id.clone(),
        })?;

    if projection.status != "ACTIVE" {
        return Err(ApiError::InvalidTransition {
            current_state: projection.status,
            action: "resolve".to_string(),
        });
    }

    let event_id = EventId::new();
    let now = Utc::now();

    let events = state
        .event_store
        .read_stream(&exception_id, 0, 1000)
        .await?;
    let current_version = events.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: exception_id.clone(),
        stream_kind: StreamKind::Exception,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "ExceptionResolved".to_string(),
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
        .append(&exception_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(exception_id = %exception_id, "Exception resolved");

    Ok(Json(ExceptionActionResponse {
        exception_id: exception_id.clone(),
        kind: projection.kind,
        status: "RESOLVED".to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
