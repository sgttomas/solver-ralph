//! Attachment Upload Handler (SR-PLAN-V7 Phase V7-3)
//!
//! Per SR-PLAN-V7 §V7-3: Attachments are human-uploaded supporting files.
//! They share MinIO storage infrastructure but are NOT Evidence Bundles.
//! They do NOT satisfy verification gates (C-VER-1).
//!
//! Ontological distinction:
//! - Evidence Bundles (`domain.evidence_bundle`): Oracle output, satisfies C-VER-1
//! - Attachments (`record.attachment`): Human uploads, supporting context only
//!
//! Storage semantics (same as C-EVID-2 but for attachments):
//! - Content-addressed storage (`sha256/{hash}`)
//! - Immutable once stored
//! - Protected against modification

use axum::{
    extract::{Multipart, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::{AttachmentStoreError, MinioAttachmentStore};
use sr_domain::{EventEnvelope, EventId, StreamKind};
use sr_ports::EventStore;
use std::sync::Arc;
use tracing::{info, instrument, warn};
use ulid::Ulid;

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Response for attachment upload
#[derive(Debug, Serialize)]
pub struct AttachmentUploadResponse {
    /// Unique attachment identifier (format: attach_<ULID>)
    pub attachment_id: String,
    /// Content hash (format: sha256:<hex>)
    pub content_hash: String,
    /// Size in bytes
    pub size_bytes: usize,
    /// Detected media type
    pub media_type: String,
    /// Original filename
    pub filename: String,
    /// Who uploaded the attachment
    pub uploaded_by: String,
    /// When the attachment was uploaded
    pub uploaded_at: String,
}

/// Attachment manifest stored alongside the file for audit purposes
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentManifest {
    /// Artifact type per SR-TYPES §4.4 (record.attachment)
    pub artifact_type: String,
    /// Unique attachment identifier
    pub attachment_id: String,
    /// Content hash
    pub content_hash: String,
    /// Size in bytes
    pub size_bytes: usize,
    /// Media type
    pub media_type: String,
    /// Original filename
    pub filename: String,
    /// Who uploaded the attachment
    pub uploaded_by: UploadedBy,
    /// When the attachment was uploaded
    pub uploaded_at: String,
}

/// Actor who uploaded the attachment per C-EVT-1 attribution
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedBy {
    pub actor_kind: String,
    pub actor_id: String,
}

// ============================================================================
// State
// ============================================================================

/// State for attachment routes
#[derive(Clone)]
pub struct AttachmentState {
    pub app_state: AppState,
    pub attachment_store: Arc<MinioAttachmentStore>,
}

// ============================================================================
// Handlers
// ============================================================================

/// Upload an attachment
///
/// POST /api/v1/attachments
///
/// Per SR-PLAN-V7 §V7-3:
/// - Accepts multipart/form-data with a `file` field
/// - Stores in MinIO with content-addressed key (`sha256/{hash}`)
/// - Returns attachment metadata
/// - Emits AttachmentRecorded event for auditability per C-EVT-1
/// - Idempotent: re-uploading same file returns same hash
#[instrument(skip(state, user, multipart), fields(user_id = %user.actor_id))]
pub async fn upload_attachment(
    State(state): State<AttachmentState>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> ApiResult<Json<AttachmentUploadResponse>> {
    // Extract file from multipart
    let mut file_content: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| ApiError::BadRequest {
        message: format!("Failed to read multipart field: {}", e),
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            content_type = field.content_type().map(|s| s.to_string());

            file_content = Some(field.bytes().await.map_err(|e| ApiError::BadRequest {
                message: format!("Failed to read file content: {}", e),
            })?.to_vec());
        }
    }

    let content = file_content.ok_or_else(|| ApiError::BadRequest {
        message: "No file field found in multipart request".to_string(),
    })?;

    let original_filename = filename.unwrap_or_else(|| "unnamed".to_string());
    let media_type = content_type
        .or_else(|| detect_media_type(&original_filename, &content))
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let size_bytes = content.len();

    // Store attachment in MinIO (content-addressed, idempotent)
    let content_hash = state
        .attachment_store
        .store(&content, &media_type)
        .await
        .map_err(|e| match e {
            AttachmentStoreError::StorageError { message } => ApiError::Internal { message },
            AttachmentStoreError::NotFound { content_hash } => ApiError::NotFound {
                resource: "Attachment".to_string(),
                id: content_hash,
            },
        })?;

    // Generate attachment ID
    let attachment_id = format!("attach_{}", Ulid::new());
    let now = Utc::now();

    // Emit AttachmentRecorded event per C-EVT-1
    let event_id = EventId::new();
    let stream_id = format!("attach:{}", attachment_id);

    let payload = serde_json::json!({
        "attachment_id": attachment_id,
        "content_hash": format!("sha256:{}", content_hash),
        "media_type": media_type,
        "size_bytes": size_bytes,
        "filename": original_filename
    });

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: stream_id.clone(),
        stream_kind: StreamKind::Attachment,
        stream_seq: 1, // First event in this stream
        global_seq: None,
        event_type: "AttachmentRecorded".to_string(),
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

    // Append event to event store
    if let Err(e) = state
        .app_state
        .event_store
        .append(&stream_id, 0, vec![event])
        .await
    {
        // Log warning but don't fail - attachment is already stored
        warn!(
            error = ?e,
            attachment_id = %attachment_id,
            "Failed to append AttachmentRecorded event"
        );
    }

    info!(
        attachment_id = %attachment_id,
        content_hash = %content_hash,
        size_bytes = %size_bytes,
        media_type = %media_type,
        "Attachment uploaded"
    );

    Ok(Json(AttachmentUploadResponse {
        attachment_id,
        content_hash: format!("sha256:{}", content_hash),
        size_bytes,
        media_type,
        filename: original_filename,
        uploaded_by: user.actor_id,
        uploaded_at: now.to_rfc3339(),
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Compute envelope hash (placeholder - matches evidence.rs pattern)
fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

/// Detect media type from filename extension or content
fn detect_media_type(filename: &str, _content: &[u8]) -> Option<String> {
    let extension = filename.rsplit('.').next()?.to_lowercase();

    match extension.as_str() {
        "pdf" => Some("application/pdf".to_string()),
        "json" => Some("application/json".to_string()),
        "md" | "markdown" => Some("text/markdown".to_string()),
        "txt" => Some("text/plain".to_string()),
        "html" | "htm" => Some("text/html".to_string()),
        "xml" => Some("application/xml".to_string()),
        "csv" => Some("text/csv".to_string()),
        "png" => Some("image/png".to_string()),
        "jpg" | "jpeg" => Some("image/jpeg".to_string()),
        "gif" => Some("image/gif".to_string()),
        "svg" => Some("image/svg+xml".to_string()),
        "webp" => Some("image/webp".to_string()),
        "zip" => Some("application/zip".to_string()),
        "tar" => Some("application/x-tar".to_string()),
        "gz" | "gzip" => Some("application/gzip".to_string()),
        "doc" => Some("application/msword".to_string()),
        "docx" => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()),
        "xls" => Some("application/vnd.ms-excel".to_string()),
        "xlsx" => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()),
        "ppt" => Some("application/vnd.ms-powerpoint".to_string()),
        "pptx" => Some("application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_media_type_pdf() {
        let result = detect_media_type("document.pdf", &[]);
        assert_eq!(result, Some("application/pdf".to_string()));
    }

    #[test]
    fn test_detect_media_type_json() {
        let result = detect_media_type("data.json", &[]);
        assert_eq!(result, Some("application/json".to_string()));
    }

    #[test]
    fn test_detect_media_type_markdown() {
        let result = detect_media_type("README.md", &[]);
        assert_eq!(result, Some("text/markdown".to_string()));
    }

    #[test]
    fn test_detect_media_type_unknown() {
        let result = detect_media_type("file.xyz", &[]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_media_type_case_insensitive() {
        let result = detect_media_type("Document.PDF", &[]);
        assert_eq!(result, Some("application/pdf".to_string()));
    }
}
