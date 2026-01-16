//! MinIO Attachment Store Adapter (SR-PLAN-V7 Phase V7-3)
//!
//! Per SR-PLAN-V7 §V7-3: Attachments are human-uploaded supporting files.
//! They share MinIO storage infrastructure but are NOT Evidence Bundles:
//! - Evidence Bundles (`domain.evidence_bundle`): Oracle output, satisfies C-VER-1
//! - Attachments (`record.attachment`): Human uploads, supporting context only
//!
//! Storage semantics (same as C-EVID-2 but for attachments):
//! - Content-addressed storage (`sha256/{hash}`)
//! - Immutable once stored
//! - Protected against modification

use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::{Builder as S3ConfigBuilder, Region},
    operation::get_object::GetObjectError,
    primitives::ByteStream,
    Client as S3Client,
};
use bytes::Bytes;
use sha2::{Digest, Sha256};
use tracing::{debug, info, instrument, warn};

/// Configuration for the MinIO attachment store
#[derive(Debug, Clone)]
pub struct AttachmentStoreConfig {
    /// MinIO endpoint URL (e.g., "http://localhost:9000")
    pub endpoint: String,
    /// AWS region (can be any value for MinIO)
    pub region: String,
    /// Access key ID
    pub access_key_id: String,
    /// Secret access key
    pub secret_access_key: String,
    /// Bucket name for attachments (separate from evidence bucket)
    pub bucket: String,
    /// Whether to force path-style addressing (required for MinIO)
    pub force_path_style: bool,
}

impl AttachmentStoreConfig {
    /// Create configuration for local development
    pub fn local_dev() -> Self {
        Self {
            endpoint: "http://localhost:9000".to_string(),
            region: "us-east-1".to_string(),
            access_key_id: "minioadmin".to_string(),
            secret_access_key: "minioadmin".to_string(),
            bucket: "attachments".to_string(),
            force_path_style: true,
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Self {
        Self {
            endpoint: std::env::var("MINIO_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            region: std::env::var("MINIO_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            access_key_id: std::env::var("MINIO_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            secret_access_key: std::env::var("MINIO_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            bucket: std::env::var("MINIO_ATTACHMENTS_BUCKET")
                .unwrap_or_else(|_| "attachments".to_string()),
            force_path_style: true,
        }
    }
}

/// Attachment store errors
#[derive(Debug, thiserror::Error)]
pub enum AttachmentStoreError {
    #[error("Attachment not found: {content_hash}")]
    NotFound { content_hash: String },

    #[error("Storage error: {message}")]
    StorageError { message: String },
}

/// MinIO-backed attachment store implementation
///
/// Stores attachments as content-addressed objects in MinIO.
/// Unlike Evidence Bundles, attachments are stored as single files at `sha256/{hash}`.
pub struct MinioAttachmentStore {
    client: S3Client,
    bucket: String,
}

impl MinioAttachmentStore {
    /// Create a new MinIO attachment store from configuration
    pub async fn new(config: AttachmentStoreConfig) -> Result<Self, AttachmentStoreError> {
        let credentials = Credentials::new(
            &config.access_key_id,
            &config.secret_access_key,
            None,
            None,
            "minio-attachment-store",
        );

        let s3_config = S3ConfigBuilder::new()
            .endpoint_url(&config.endpoint)
            .region(Region::new(config.region))
            .credentials_provider(credentials)
            .force_path_style(config.force_path_style)
            .behavior_version(BehaviorVersion::latest())
            .build();

        let client = S3Client::from_conf(s3_config);

        let store = Self {
            client,
            bucket: config.bucket,
        };

        store.ensure_bucket().await?;

        info!(bucket = %store.bucket, "MinIO attachment store initialized");
        Ok(store)
    }

    /// Ensure the attachments bucket exists
    async fn ensure_bucket(&self) -> Result<(), AttachmentStoreError> {
        match self.client.head_bucket().bucket(&self.bucket).send().await {
            Ok(_) => {
                debug!(bucket = %self.bucket, "Attachments bucket exists");
                Ok(())
            }
            Err(e) => {
                warn!(bucket = %self.bucket, error = ?e, "Bucket not found, creating");
                self.client
                    .create_bucket()
                    .bucket(&self.bucket)
                    .send()
                    .await
                    .map_err(|e| AttachmentStoreError::StorageError {
                        message: format!("Failed to create bucket: {}", e),
                    })?;
                info!(bucket = %self.bucket, "Created attachments bucket");
                Ok(())
            }
        }
    }

    /// Compute SHA-256 hash of content
    pub fn compute_hash(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }

    /// Get the S3 key for an attachment
    fn object_key(content_hash: &str) -> String {
        format!("sha256/{}", content_hash)
    }

    /// Check if an object exists in S3
    async fn object_exists(&self, key: &str) -> Result<bool, AttachmentStoreError> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                let service_err = e.into_service_error();
                if service_err.is_not_found() {
                    Ok(false)
                } else {
                    Err(AttachmentStoreError::StorageError {
                        message: format!("Failed to check object existence: {:?}", service_err),
                    })
                }
            }
        }
    }

    /// Store an attachment, returning its content hash
    ///
    /// This is idempotent: re-uploading the same content returns the same hash
    /// without re-writing the object.
    #[instrument(skip(self, content), fields(bucket = %self.bucket, size = content.len()))]
    pub async fn store(
        &self,
        content: &[u8],
        content_type: &str,
    ) -> Result<String, AttachmentStoreError> {
        // Compute content hash
        let content_hash = Self::compute_hash(content);
        let object_key = Self::object_key(&content_hash);

        debug!(content_hash = %content_hash, "Computed attachment hash");

        // Check if already exists (immutability / idempotency)
        if self.object_exists(&object_key).await? {
            info!(
                content_hash = %content_hash,
                "Attachment already exists, skipping upload"
            );
            return Ok(content_hash);
        }

        // Upload the attachment
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&object_key)
            .body(ByteStream::from(Bytes::copy_from_slice(content)))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| AttachmentStoreError::StorageError {
                message: format!("Failed to upload attachment: {}", e),
            })?;

        info!(
            content_hash = %content_hash,
            size = content.len(),
            content_type = %content_type,
            "Attachment stored successfully"
        );
        Ok(content_hash)
    }

    /// Retrieve an attachment by content hash
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn retrieve(&self, content_hash: &str) -> Result<Vec<u8>, AttachmentStoreError> {
        let object_key = Self::object_key(content_hash);

        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&object_key)
            .send()
            .await
            .map_err(|e| {
                let service_err = e.into_service_error();
                match &service_err {
                    GetObjectError::NoSuchKey(_) => AttachmentStoreError::NotFound {
                        content_hash: content_hash.to_string(),
                    },
                    _ => AttachmentStoreError::StorageError {
                        message: format!("Failed to retrieve attachment: {:?}", service_err),
                    },
                }
            })?;

        let data =
            response
                .body
                .collect()
                .await
                .map_err(|e| AttachmentStoreError::StorageError {
                    message: format!("Failed to read response body: {}", e),
                })?;

        let bytes = data.into_bytes();
        debug!(
            content_hash = %content_hash,
            size = bytes.len(),
            "Retrieved attachment"
        );
        Ok(bytes.to_vec())
    }

    /// Check if an attachment exists
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn exists(&self, content_hash: &str) -> Result<bool, AttachmentStoreError> {
        let object_key = Self::object_key(content_hash);
        self.object_exists(&object_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_determinism() {
        let content = b"hello world";
        let hash1 = MinioAttachmentStore::compute_hash(content);
        let hash2 = MinioAttachmentStore::compute_hash(content);
        assert_eq!(hash1, hash2, "Hash computation should be deterministic");
    }

    #[test]
    fn test_compute_hash_different_content() {
        let hash1 = MinioAttachmentStore::compute_hash(b"hello");
        let hash2 = MinioAttachmentStore::compute_hash(b"world");
        assert_ne!(
            hash1, hash2,
            "Different content should have different hashes"
        );
    }

    #[test]
    fn test_object_key_format() {
        let hash = "abc123def456";
        let key = MinioAttachmentStore::object_key(hash);
        assert_eq!(key, "sha256/abc123def456");
    }
}
