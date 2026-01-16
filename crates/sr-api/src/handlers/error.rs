//! API Error Types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use sr_adapters::ProjectionError;
use sr_ports::EventStoreError;

/// API result type alias
pub type ApiResult<T> = Result<T, ApiError>;

/// API error types
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found
    NotFound { resource: String, id: String },
    /// Bad request (validation error)
    BadRequest { message: String },
    /// Conflict (e.g., concurrency conflict)
    Conflict { message: String },
    /// Forbidden (insufficient permissions)
    Forbidden { message: String },
    /// Invalid state transition
    InvalidTransition {
        current_state: String,
        action: String,
    },
    /// Feature not implemented
    NotImplemented { feature: String },
    /// Internal server error
    Internal { message: String },
    /// Work Surface not found for work unit per SR-PLAN-V4 Phase 4c
    /// This is a precondition failure when starting iterations
    WorkSurfaceNotFound { work_unit_id: String },
    /// Approval required before stage completion per SR-PLAN-V5 Phase 5c
    /// This is a precondition failure for approval-gated stages
    ApprovalRequired {
        stage_id: String,
        portal_id: String,
        work_surface_id: String,
    },
}

/// Error response body
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, details) = match &self {
            ApiError::NotFound { resource, id } => (
                StatusCode::NOT_FOUND,
                format!("{} not found: {}", resource, id),
                None,
            ),
            ApiError::BadRequest { message } => (StatusCode::BAD_REQUEST, message.clone(), None),
            ApiError::Conflict { message } => (StatusCode::CONFLICT, message.clone(), None),
            ApiError::Forbidden { message } => (StatusCode::FORBIDDEN, message.clone(), None),
            ApiError::InvalidTransition {
                current_state,
                action,
            } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Cannot {} from state {}", action, current_state),
                Some(serde_json::json!({
                    "current_state": current_state,
                    "action": action
                })),
            ),
            ApiError::NotImplemented { feature } => (
                StatusCode::NOT_IMPLEMENTED,
                format!("{} is not yet implemented", feature),
                None,
            ),
            ApiError::Internal { message } => {
                tracing::error!(error = %message, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
            ApiError::WorkSurfaceNotFound { work_unit_id } => (
                StatusCode::PRECONDITION_FAILED,
                format!(
                    "No active Work Surface found for work unit: {}. \
                     Bind a Work Surface before starting iterations.",
                    work_unit_id
                ),
                Some(serde_json::json!({
                    "work_unit_id": work_unit_id,
                    "error_code": "WORK_SURFACE_MISSING"
                })),
            ),
            ApiError::ApprovalRequired {
                stage_id,
                portal_id,
                work_surface_id,
            } => (
                StatusCode::PRECONDITION_FAILED,
                format!(
                    "Approval required before completing stage '{}'. \
                     Record an approval at portal '{}' with this work surface as subject.",
                    stage_id, portal_id
                ),
                Some(serde_json::json!({
                    "stage_id": stage_id,
                    "portal_id": portal_id,
                    "work_surface_id": work_surface_id,
                    "error_code": "APPROVAL_REQUIRED"
                })),
            ),
        };

        let body = ErrorResponse {
            error,
            code: status.as_u16(),
            details,
        };

        (status, Json(body)).into_response()
    }
}

impl From<EventStoreError> for ApiError {
    fn from(e: EventStoreError) -> Self {
        match e {
            EventStoreError::ConcurrencyConflict { expected, actual } => ApiError::Conflict {
                message: format!(
                    "Concurrency conflict: expected version {}, got {}",
                    expected, actual
                ),
            },
            EventStoreError::StreamNotFound { stream_id } => ApiError::NotFound {
                resource: "Stream".to_string(),
                id: stream_id,
            },
            EventStoreError::ConnectionError { message } => ApiError::Internal { message },
            EventStoreError::SerializationError { message } => ApiError::Internal { message },
        }
    }
}

impl From<ProjectionError> for ApiError {
    fn from(e: ProjectionError) -> Self {
        match e {
            ProjectionError::DatabaseError { message } => ApiError::Internal { message },
            ProjectionError::DeserializationError { message } => ApiError::Internal { message },
            ProjectionError::EventStoreError(es) => es.into(),
            ProjectionError::UnknownEventType { event_type } => ApiError::Internal {
                message: format!("Unknown event type: {}", event_type),
            },
        }
    }
}
