//! Staleness management endpoints (SR-SPEC §2.3.9, §1.13)
//! Implements marking, listing, and resolving staleness markers that gate shippable status.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::{DependencyEdge, GraphError, GraphProjection, StalenessReason};
use sr_domain::{ActorKind, EventEnvelope, EventId, StreamKind, TypedRef};
use sr_ports::EventStore;
use ulid::Ulid;

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

const DEFAULT_MAX_DEPTH: i32 = 5;

/// Request to mark a node as stale and propagate to dependents
#[derive(Debug, Deserialize)]
pub struct MarkStalenessRequest {
    pub root_ref: StaleRef,
    #[serde(default = "default_reason_code")]
    pub reason_code: String,
    #[serde(default)]
    pub reason_detail: Option<String>,
    #[serde(default = "default_max_depth")]
    pub max_depth: i32,
}

fn default_max_depth() -> i32 {
    DEFAULT_MAX_DEPTH
}

fn default_reason_code() -> String {
    "MANUAL_MARK".to_string()
}

/// Root or dependent reference
#[derive(Debug, Deserialize)]
pub struct StaleRef {
    pub kind: String,
    pub id: String,
}

/// Response after marking staleness
#[derive(Debug, Serialize)]
pub struct MarkStalenessResponse {
    pub stale_ids: Vec<String>,
    pub dependents_marked: Vec<StaleDependent>,
}

/// Stale dependent details
#[derive(Debug, Serialize)]
pub struct StaleDependent {
    pub stale_id: String,
    pub dependent_kind: String,
    pub dependent_id: String,
}

/// Query parameters for listing stale dependents
#[derive(Debug, Deserialize)]
pub struct StaleDependentsQuery {
    pub root_kind: String,
    pub root_id: String,
}

/// Response for listing stale dependents
#[derive(Debug, Serialize)]
pub struct StaleDependentsResponse {
    pub root_kind: String,
    pub root_id: String,
    pub dependents: Vec<StaleDependentDetail>,
}

/// Stale dependent detail
#[derive(Debug, Serialize)]
pub struct StaleDependentDetail {
    pub stale_id: String,
    pub dependent_kind: String,
    pub dependent_id: String,
    pub reason_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_detail: Option<String>,
    pub marked_at: String,
}

/// Request to resolve a staleness marker
#[derive(Debug, Deserialize)]
pub struct ResolveStalenessRequest {
    pub resolution_kind: String, // MECHANICAL | DECISION
    #[serde(default)]
    pub resolution_note: Option<String>,
    #[serde(default)]
    pub resolution_refs: Vec<TypedRefRequest>,
}

#[derive(Debug, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    #[serde(default = "default_relates_to")]
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

fn default_relates_to() -> String {
    "relates_to".to_string()
}

/// Response for resolving staleness
#[derive(Debug, Serialize)]
pub struct ResolveStalenessResponse {
    pub stale_id: String,
    pub resolved: bool,
    pub event_id: String,
}

/// POST /staleness/mark
pub async fn mark_staleness(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<MarkStalenessRequest>,
) -> ApiResult<Json<MarkStalenessResponse>> {
    let reason = parse_reason(&body.reason_code)?;
    if body.root_ref.id.is_empty() || body.root_ref.kind.is_empty() {
        return Err(ApiError::BadRequest {
            message: "root_ref.kind and root_ref.id are required".to_string(),
        });
    }

    let graph = GraphProjection::new(state.projections.pool().clone());
    graph
        .ensure_node(&body.root_ref.id, &body.root_ref.kind, None, None)
        .await
        .map_err(graph_err)?;

    let dependents = graph
        .get_dependents(&body.root_ref.id, body.max_depth)
        .await
        .map_err(graph_err)?;

    let dependents_to_mark = select_dependents(&graph, &body.root_ref, dependents).await?;

    let mut stale_ids = Vec::new();
    let mut marked = Vec::new();

    for dependent in dependents_to_mark {
        let stale_id = format!("stale_{}", Ulid::new());
        let event_id = EventId::new();
        let now = Utc::now();

        let refs = vec![
            TypedRef {
                kind: body.root_ref.kind.clone(),
                id: body.root_ref.id.clone(),
                rel: "root_cause".to_string(),
                meta: serde_json::Value::Null,
            },
            TypedRef {
                kind: dependent.kind.clone(),
                id: dependent.id.clone(),
                rel: "stale".to_string(),
                meta: serde_json::json!({"reason_code": body.reason_code}),
            },
        ];

        let event = EventEnvelope {
            event_id: event_id.clone(),
            stream_id: stale_id.clone(),
            stream_kind: StreamKind::Governance,
            stream_seq: 1,
            global_seq: None,
            event_type: "NodeMarkedStale".to_string(),
            occurred_at: now,
            actor_kind: user.actor_kind,
            actor_id: user.actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs,
            payload: serde_json::json!({
                "stale_id": stale_id,
                "root_kind": body.root_ref.kind,
                "root_id": body.root_ref.id,
                "dependent_kind": dependent.kind,
                "dependent_id": dependent.id,
                "reason_code": reason.as_str(),
                "reason_detail": body.reason_detail
            }),
            envelope_hash: compute_envelope_hash(&event_id),
        };

        let stream_id = event.stream_id.clone();
        state
            .event_store
            .append(stream_id.as_str(), 0, vec![event])
            .await?;

        stale_ids.push(stale_id.clone());
        marked.push(StaleDependent {
            stale_id,
            dependent_kind: dependent.kind,
            dependent_id: dependent.id,
        });
    }

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok(Json(MarkStalenessResponse {
        stale_ids,
        dependents_marked: marked,
    }))
}

/// GET /staleness/dependents
pub async fn list_stale_dependents(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<StaleDependentsQuery>,
) -> ApiResult<Json<StaleDependentsResponse>> {
    let graph = GraphProjection::new(state.projections.pool().clone());
    let markers = graph
        .get_stale_dependents(&query.root_kind, &query.root_id)
        .await
        .map_err(graph_err)?;

    let dependents = markers
        .into_iter()
        .map(|m| StaleDependentDetail {
            stale_id: m.stale_id,
            dependent_kind: m.dependent_kind,
            dependent_id: m.dependent_id,
            reason_code: m.reason_code,
            reason_detail: m.reason_detail,
            marked_at: m.marked_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(StaleDependentsResponse {
        root_kind: query.root_kind,
        root_id: query.root_id,
        dependents,
    }))
}

/// POST /staleness/{stale_id}/resolve
pub async fn resolve_staleness(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(stale_id): Path<String>,
    Json(body): Json<ResolveStalenessRequest>,
) -> ApiResult<Json<ResolveStalenessResponse>> {
    let resolution_kind = body.resolution_kind.to_uppercase();
    if resolution_kind != "MECHANICAL" && resolution_kind != "DECISION" {
        return Err(ApiError::BadRequest {
            message: "resolution_kind must be MECHANICAL or DECISION".to_string(),
        });
    }

    if resolution_kind == "DECISION" && !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "DECISION resolution requires HUMAN actor".to_string(),
        });
    }

    let graph = GraphProjection::new(state.projections.pool().clone());
    let marker = graph
        .get_staleness_marker_by_id(&stale_id)
        .await
        .map_err(graph_err)?;

    let marker = marker.ok_or_else(|| ApiError::NotFound {
        resource: "StalenessMarker".to_string(),
        id: stale_id.clone(),
    })?;

    let event_id = EventId::new();
    let now = Utc::now();

    let mut refs = vec![
        TypedRef {
            kind: marker.root_kind.clone(),
            id: marker.root_id.clone(),
            rel: "root_cause".to_string(),
            meta: serde_json::Value::Null,
        },
        TypedRef {
            kind: marker.dependent_kind.clone(),
            id: marker.dependent_id.clone(),
            rel: "stale".to_string(),
            meta: serde_json::Value::Null,
        },
    ];

    for r in body.resolution_refs {
        refs.push(TypedRef {
            kind: r.kind,
            id: r.id,
            rel: r.rel,
            meta: r.meta,
        });
    }

    let existing = state.event_store.read_stream(&stale_id, 0, 1000).await?;
    let current_version = existing.len() as u64;

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: stale_id.clone(),
        stream_kind: StreamKind::Governance,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "StalenessResolved".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs,
        payload: serde_json::json!({
            "stale_id": stale_id,
            "resolution_kind": resolution_kind,
            "resolution_note": body.resolution_note
        }),
        envelope_hash: compute_envelope_hash(&event_id),
    };

    let stream_id = event.stream_id.clone();
    state
        .event_store
        .append(&stream_id, current_version, vec![event])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok(Json(ResolveStalenessResponse {
        stale_id,
        resolved: true,
        event_id: event_id.as_str().to_string(),
    }))
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

fn parse_reason(reason: &str) -> Result<StalenessReason, ApiError> {
    StalenessReason::from_str(reason).ok_or_else(|| ApiError::BadRequest {
        message: format!(
            "Invalid reason_code '{}'. Expected one of: GOVERNED_ARTIFACT_CHANGED, ORACLE_SUITE_REBASED, EXCEPTION_ACTIVATED, DEPENDENCY_STALE, MANUAL_MARK",
            reason
        ),
    })
}

fn graph_err(e: GraphError) -> ApiError {
    ApiError::Internal {
        message: format!("Graph operation failed: {}", e),
    }
}

#[derive(Debug)]
struct DependentTarget {
    kind: String,
    id: String,
}

async fn select_dependents(
    graph: &GraphProjection,
    root: &StaleRef,
    dependents: Vec<DependencyEdge>,
) -> Result<Vec<DependentTarget>, ApiError> {
    let mut targets = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // If no dependents were found, mark the root itself
    if dependents.is_empty() {
        targets.push(DependentTarget {
            kind: root.kind.clone(),
            id: root.id.clone(),
        });
        return Ok(targets);
    }

    for dep in dependents {
        if seen.insert(dep.src_id.clone()) {
            let kind = graph
                .get_node(&dep.src_id)
                .await
                .map_err(graph_err)?
                .map(|n| n.node_type)
                .unwrap_or_else(|| "Unknown".to_string());

            targets.push(DependentTarget {
                kind,
                id: dep.src_id,
            });
        }
    }

    Ok(targets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_reason_codes() {
        assert!(parse_reason("GOVERNED_ARTIFACT_CHANGED").is_ok());
        assert!(parse_reason("MANUAL_MARK").is_ok());
        assert!(parse_reason("DEPENDENCY_STALE").is_ok());
    }

    #[test]
    fn rejects_invalid_reason_code() {
        let err = parse_reason("NOT_A_REASON").unwrap_err();
        match err {
            ApiError::BadRequest { message } => {
                assert!(message.contains("Invalid reason_code"));
            }
            _ => panic!("expected bad request"),
        }
    }
}
