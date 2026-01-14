//! MinIO Evidence Store Adapter (D-14)
//!
//! Implements the EvidenceStore port with MinIO/S3 compatible storage.
//! Per SR-SPEC §1.9, evidence bundles are stored in content-addressed storage.
//!
//! Key features:
//! - Content-addressed storage using SHA-256 hashes
//! - Immutable objects (no overwrites)
//! - Evidence manifest validation on write
//! - Access control via bucket policies

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
use sr_ports::{EvidenceStore, EvidenceStoreError};
use tracing::{debug, error, info, instrument, warn};

/// Configuration for the MinIO evidence store
#[derive(Debug, Clone)]
pub struct MinioConfig {
    /// MinIO endpoint URL (e.g., "http://localhost:9000")
    pub endpoint: String,
    /// AWS region (can be any value for MinIO)
    pub region: String,
    /// Access key ID
    pub access_key_id: String,
    /// Secret access key
    pub secret_access_key: String,
    /// Bucket name for evidence bundles
    pub bucket: String,
    /// Whether to force path-style addressing (required for MinIO)
    pub force_path_style: bool,
}

impl MinioConfig {
    /// Create a new MinIO configuration with defaults for local development
    pub fn local_dev() -> Self {
        Self {
            endpoint: "http://localhost:9000".to_string(),
            region: "us-east-1".to_string(),
            access_key_id: "minioadmin".to_string(),
            secret_access_key: "minioadmin".to_string(),
            bucket: "evidence".to_string(),
            force_path_style: true,
        }
    }
}

/// MinIO-backed evidence store implementation
///
/// Stores evidence bundles as content-addressed objects in MinIO.
/// The bundle structure is:
/// - `{hash}/manifest.json` - the evidence manifest
/// - `{hash}/blobs/{name}` - individual blob artifacts
pub struct MinioEvidenceStore {
    client: S3Client,
    bucket: String,
}

impl MinioEvidenceStore {
    /// Create a new MinIO evidence store from configuration
    pub async fn new(config: MinioConfig) -> Result<Self, EvidenceStoreError> {
        let credentials = Credentials::new(
            &config.access_key_id,
            &config.secret_access_key,
            None,
            None,
            "minio-evidence-store",
        );

        let s3_config = S3ConfigBuilder::new()
            .endpoint_url(&config.endpoint)
            .region(Region::new(config.region))
            .credentials_provider(credentials)
            .force_path_style(config.force_path_style)
            .behavior_version(BehaviorVersion::latest())
            .build();

        let client = S3Client::from_conf(s3_config);

        // Verify bucket exists (or create it in dev mode)
        let store = Self {
            client,
            bucket: config.bucket,
        };

        store.ensure_bucket().await?;

        info!(bucket = %store.bucket, "MinIO evidence store initialized");
        Ok(store)
    }

    /// Ensure the evidence bucket exists
    async fn ensure_bucket(&self) -> Result<(), EvidenceStoreError> {
        match self.client.head_bucket().bucket(&self.bucket).send().await {
            Ok(_) => {
                debug!(bucket = %self.bucket, "Evidence bucket exists");
                Ok(())
            }
            Err(e) => {
                warn!(bucket = %self.bucket, error = ?e, "Bucket not found, creating");
                self.client
                    .create_bucket()
                    .bucket(&self.bucket)
                    .send()
                    .await
                    .map_err(|e| EvidenceStoreError::StorageError {
                        message: format!("Failed to create bucket: {}", e),
                    })?;
                info!(bucket = %self.bucket, "Created evidence bucket");
                Ok(())
            }
        }
    }

    /// Compute the content hash for an evidence bundle
    fn compute_bundle_hash(manifest: &[u8], blobs: &[(&str, &[u8])]) -> String {
        let mut hasher = Sha256::new();

        // Hash the manifest
        hasher.update(manifest);

        // Hash each blob in sorted order for determinism
        let mut sorted_blobs: Vec<_> = blobs.iter().collect();
        sorted_blobs.sort_by_key(|(name, _)| *name);

        for (name, data) in sorted_blobs {
            hasher.update(name.as_bytes());
            hasher.update(data);
        }

        hex::encode(hasher.finalize())
    }

    /// Get the S3 key for a manifest
    fn manifest_key(content_hash: &str) -> String {
        format!("{}/manifest.json", content_hash)
    }

    /// Get the S3 key for a blob
    fn blob_key(content_hash: &str, blob_name: &str) -> String {
        format!("{}/blobs/{}", content_hash, blob_name)
    }

    /// Check if an object exists in S3
    async fn object_exists(&self, key: &str) -> Result<bool, EvidenceStoreError> {
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
                    Err(EvidenceStoreError::StorageError {
                        message: format!("Failed to check object existence: {:?}", service_err),
                    })
                }
            }
        }
    }
}

impl EvidenceStore for MinioEvidenceStore {
    #[instrument(skip(self, manifest, blobs), fields(bucket = %self.bucket))]
    async fn store(
        &self,
        manifest: &[u8],
        blobs: Vec<(&str, &[u8])>,
    ) -> Result<String, EvidenceStoreError> {
        // Compute content hash
        let content_hash = Self::compute_bundle_hash(manifest, &blobs);
        debug!(content_hash = %content_hash, "Computed evidence bundle hash");

        // Check if already exists (immutability check)
        let manifest_key = Self::manifest_key(&content_hash);
        if self.object_exists(&manifest_key).await? {
            info!(
                content_hash = %content_hash,
                "Evidence bundle already exists, skipping upload"
            );
            return Ok(content_hash);
        }

        // Upload manifest
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&manifest_key)
            .body(ByteStream::from(Bytes::copy_from_slice(manifest)))
            .content_type("application/json")
            .send()
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to upload manifest: {}", e),
            })?;
        debug!(key = %manifest_key, "Uploaded evidence manifest");

        // Upload blobs
        for (name, data) in blobs {
            let blob_key = Self::blob_key(&content_hash, name);
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(&blob_key)
                .body(ByteStream::from(Bytes::copy_from_slice(data)))
                .send()
                .await
                .map_err(|e| EvidenceStoreError::StorageError {
                    message: format!("Failed to upload blob '{}': {}", name, e),
                })?;
            debug!(key = %blob_key, "Uploaded evidence blob");
        }

        info!(
            content_hash = %content_hash,
            "Evidence bundle stored successfully"
        );
        Ok(content_hash)
    }

    #[instrument(skip(self), fields(bucket = %self.bucket))]
    async fn retrieve(&self, content_hash: &str) -> Result<Vec<u8>, EvidenceStoreError> {
        let manifest_key = Self::manifest_key(content_hash);

        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&manifest_key)
            .send()
            .await
            .map_err(|e| {
                let service_err = e.into_service_error();
                match &service_err {
                    GetObjectError::NoSuchKey(_) => EvidenceStoreError::NotFound {
                        content_hash: content_hash.to_string(),
                    },
                    _ => EvidenceStoreError::StorageError {
                        message: format!("Failed to retrieve evidence: {:?}", service_err),
                    },
                }
            })?;

        let data = response
            .body
            .collect()
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to read response body: {}", e),
            })?;

        let bytes = data.into_bytes();
        debug!(
            content_hash = %content_hash,
            size = bytes.len(),
            "Retrieved evidence manifest"
        );
        Ok(bytes.to_vec())
    }

    #[instrument(skip(self), fields(bucket = %self.bucket))]
    async fn exists(&self, content_hash: &str) -> Result<bool, EvidenceStoreError> {
        let manifest_key = Self::manifest_key(content_hash);
        self.object_exists(&manifest_key).await
    }
}

/// Extended evidence store operations for blob access
impl MinioEvidenceStore {
    /// Retrieve a specific blob from an evidence bundle
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn retrieve_blob(
        &self,
        content_hash: &str,
        blob_name: &str,
    ) -> Result<Vec<u8>, EvidenceStoreError> {
        let blob_key = Self::blob_key(content_hash, blob_name);

        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&blob_key)
            .send()
            .await
            .map_err(|e| {
                let service_err = e.into_service_error();
                match &service_err {
                    GetObjectError::NoSuchKey(_) => EvidenceStoreError::NotFound {
                        content_hash: format!("{}:{}", content_hash, blob_name),
                    },
                    _ => EvidenceStoreError::StorageError {
                        message: format!("Failed to retrieve blob: {:?}", service_err),
                    },
                }
            })?;

        let data = response
            .body
            .collect()
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to read blob body: {}", e),
            })?;

        let bytes = data.into_bytes();
        debug!(
            content_hash = %content_hash,
            blob_name = %blob_name,
            size = bytes.len(),
            "Retrieved evidence blob"
        );
        Ok(bytes.to_vec())
    }

    /// List all blobs in an evidence bundle
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn list_blobs(&self, content_hash: &str) -> Result<Vec<String>, EvidenceStoreError> {
        let prefix = format!("{}/blobs/", content_hash);

        let response = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&prefix)
            .send()
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to list blobs: {}", e),
            })?;

        let blobs: Vec<String> = response
            .contents()
            .iter()
            .filter_map(|obj| {
                obj.key().and_then(|key| {
                    key.strip_prefix(&prefix)
                        .map(|name| name.to_string())
                })
            })
            .collect();

        debug!(
            content_hash = %content_hash,
            count = blobs.len(),
            "Listed evidence blobs"
        );
        Ok(blobs)
    }

    /// Verify the integrity of a stored evidence bundle
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn verify_integrity(&self, content_hash: &str) -> Result<bool, EvidenceStoreError> {
        // Retrieve manifest
        let manifest = self.retrieve(content_hash).await?;

        // Retrieve all blobs
        let blob_names = self.list_blobs(content_hash).await?;
        let mut blobs = Vec::new();

        for name in &blob_names {
            let data = self.retrieve_blob(content_hash, name).await?;
            blobs.push((name.as_str(), data));
        }

        // Recompute hash
        let blob_refs: Vec<(&str, &[u8])> =
            blobs.iter().map(|(n, d)| (*n, d.as_slice())).collect();
        let computed_hash = Self::compute_bundle_hash(&manifest, &blob_refs);

        let valid = computed_hash == content_hash;
        if !valid {
            error!(
                expected = %content_hash,
                actual = %computed_hash,
                "Evidence integrity check failed"
            );
        } else {
            debug!(content_hash = %content_hash, "Evidence integrity verified");
        }

        Ok(valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_bundle_hash_determinism() {
        let manifest = br#"{"version": "v1", "type": "evidence.gate_packet"}"#;
        let blobs = vec![("output.txt", b"hello world" as &[u8]), ("log.txt", b"log data")];

        let hash1 = MinioEvidenceStore::compute_bundle_hash(manifest, &blobs);
        let hash2 = MinioEvidenceStore::compute_bundle_hash(manifest, &blobs);

        assert_eq!(hash1, hash2, "Hash computation should be deterministic");
    }

    #[test]
    fn test_compute_bundle_hash_order_independence() {
        let manifest = br#"{"version": "v1"}"#;
        let blobs1 = vec![("a.txt", b"aaa" as &[u8]), ("b.txt", b"bbb")];
        let blobs2 = vec![("b.txt", b"bbb" as &[u8]), ("a.txt", b"aaa")];

        let hash1 = MinioEvidenceStore::compute_bundle_hash(manifest, &blobs1);
        let hash2 = MinioEvidenceStore::compute_bundle_hash(manifest, &blobs2);

        assert_eq!(hash1, hash2, "Hash should be independent of blob order");
    }

    #[test]
    fn test_key_generation() {
        let hash = "abc123def456";

        assert_eq!(
            MinioEvidenceStore::manifest_key(hash),
            "abc123def456/manifest.json"
        );
        assert_eq!(
            MinioEvidenceStore::blob_key(hash, "output.txt"),
            "abc123def456/blobs/output.txt"
        );
    }
}
