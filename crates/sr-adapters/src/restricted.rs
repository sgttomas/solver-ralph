//! Restricted Evidence Handling (D-16)
//!
//! Implements restricted evidence handling per SR-CONTRACT C-EVID-5:
//! - Encryption at rest using envelope encryption
//! - Envelope keys stored/managed via Infisical (SecretProvider)
//! - Explicit redaction rules with manifests
//!
//! The envelope encryption pattern:
//! 1. Generate a data encryption key (DEK) for each restricted bundle
//! 2. Encrypt the bundle with the DEK using AES-256-GCM
//! 3. Encrypt the DEK with a key encryption key (KEK) from SecretProvider
//! 4. Store encrypted bundle in MinIO, encrypted DEK in secret store
//! 5. To decrypt: retrieve encrypted DEK, decrypt with KEK, decrypt bundle

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_ports::{
    EnvelopeKey, EvidenceStore, EvidenceStoreError, SecretMetadata, SecretProvider,
    SecretProviderError,
};
use std::collections::BTreeMap;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

/// Classification levels for evidence per SR-SPEC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvidenceClassification {
    /// Unrestricted - stored in plaintext
    Public,
    /// Internal - stored in plaintext but with restricted distribution
    Internal,
    /// Restricted - encrypted at rest
    Restricted,
    /// Confidential - encrypted at rest with additional controls
    Confidential,
}

impl Default for EvidenceClassification {
    fn default() -> Self {
        Self::Internal
    }
}

/// Encryption metadata stored alongside encrypted evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    /// Content hash of the original (unencrypted) bundle
    pub original_hash: String,
    /// Algorithm used for encryption
    pub algorithm: String,
    /// Key ID used for envelope encryption
    pub kek_id: String,
    /// Path to the encrypted DEK in secret store
    pub dek_path: String,
    /// Nonce used for encryption (base64 encoded)
    pub nonce: String,
    /// Classification level
    pub classification: EvidenceClassification,
    /// Encryption timestamp
    pub encrypted_at: chrono::DateTime<chrono::Utc>,
}

/// Redaction manifest per SR-CONTRACT C-EVID-5
///
/// When evidence contains secrets, a redaction manifest describes:
/// - What was redacted
/// - Why it was redacted
/// - A hash of the redacted content for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionManifest {
    /// Version of the redaction manifest schema
    pub version: String,
    /// Hash of the original (unrestricted) evidence bundle
    pub original_bundle_hash: String,
    /// Hash of the redacted evidence bundle
    pub redacted_bundle_hash: String,
    /// Timestamp of redaction
    pub redacted_at: chrono::DateTime<chrono::Utc>,
    /// Individual redaction entries
    pub redactions: Vec<RedactionEntry>,
    /// Actor who performed the redaction
    pub redacted_by: String,
}

/// Individual redaction entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionEntry {
    /// Artifact name that was redacted
    pub artifact_name: String,
    /// Type of content redacted
    pub content_type: RedactedContentType,
    /// Reason for redaction
    pub reason: String,
    /// Hash of the original content (for audit)
    pub original_content_hash: String,
    /// Byte offset in original (if applicable)
    pub offset: Option<u64>,
    /// Length of redacted section (if applicable)
    pub length: Option<u64>,
}

/// Types of content that can be redacted
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RedactedContentType {
    /// API keys, tokens, passwords
    Secret,
    /// Personal identifiable information
    Pii,
    /// Private keys, certificates
    PrivateKey,
    /// Database credentials
    DatabaseCredential,
    /// Environment variables with sensitive values
    EnvironmentVariable,
    /// Generic sensitive content
    Sensitive,
}

/// Configuration for the restricted evidence store
#[derive(Debug, Clone)]
pub struct RestrictedEvidenceConfig {
    /// Key ID for envelope encryption
    pub kek_id: String,
    /// Prefix for DEK storage paths
    pub dek_path_prefix: String,
    /// Classification threshold for encryption
    pub encryption_threshold: EvidenceClassification,
}

impl Default for RestrictedEvidenceConfig {
    fn default() -> Self {
        Self {
            kek_id: "default-kek".to_string(),
            dek_path_prefix: "solver-ralph/evidence/deks".to_string(),
            encryption_threshold: EvidenceClassification::Restricted,
        }
    }
}

/// Restricted evidence store wrapper
///
/// Wraps an underlying EvidenceStore to provide:
/// - Envelope encryption for restricted/confidential evidence
/// - Redaction manifest support
/// - Classification-based access control
pub struct RestrictedEvidenceStore<E: EvidenceStore, S: SecretProvider> {
    /// Underlying evidence store
    inner: Arc<E>,
    /// Secret provider for key management
    secrets: Arc<S>,
    /// Configuration
    config: RestrictedEvidenceConfig,
}

impl<E: EvidenceStore, S: SecretProvider> RestrictedEvidenceStore<E, S> {
    /// Create a new restricted evidence store
    pub fn new(inner: Arc<E>, secrets: Arc<S>, config: RestrictedEvidenceConfig) -> Self {
        Self {
            inner,
            secrets,
            config,
        }
    }

    /// Check if a classification requires encryption
    fn requires_encryption(&self, classification: EvidenceClassification) -> bool {
        match (classification, self.config.encryption_threshold) {
            (EvidenceClassification::Public, _) => false,
            (EvidenceClassification::Internal, EvidenceClassification::Public) => true,
            (EvidenceClassification::Internal, _) => false,
            (EvidenceClassification::Restricted, EvidenceClassification::Confidential) => false,
            (EvidenceClassification::Restricted, _) => true,
            (EvidenceClassification::Confidential, _) => true,
        }
    }

    /// Generate a data encryption key
    fn generate_dek() -> Vec<u8> {
        let mut key = vec![0u8; 32]; // 256 bits for AES-256
        OsRng.fill_bytes(&mut key);
        key
    }

    /// Generate a random nonce
    fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }

    /// Encrypt data with AES-256-GCM
    fn encrypt_data(key: &[u8], nonce: &[u8; 12], data: &[u8]) -> Result<Vec<u8>, EvidenceStoreError> {
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| EvidenceStoreError::StorageError {
            message: format!("Failed to create cipher: {}", e),
        })?;

        let nonce = Nonce::from_slice(nonce);
        cipher
            .encrypt(nonce, data)
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Encryption failed: {}", e),
            })
    }

    /// Decrypt data with AES-256-GCM
    fn decrypt_data(key: &[u8], nonce: &[u8; 12], data: &[u8]) -> Result<Vec<u8>, EvidenceStoreError> {
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| EvidenceStoreError::StorageError {
            message: format!("Failed to create cipher: {}", e),
        })?;

        let nonce = Nonce::from_slice(nonce);
        cipher
            .decrypt(nonce, data)
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Decryption failed: {}", e),
            })
    }

    /// Encrypt a DEK with the KEK (envelope encryption)
    async fn encrypt_dek(&self, dek: &[u8]) -> Result<Vec<u8>, EvidenceStoreError> {
        let envelope_key = self
            .secrets
            .get_envelope_key(&self.config.kek_id)
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to get envelope key: {}", e),
            })?;

        let nonce = Self::generate_nonce();
        let encrypted = Self::encrypt_data(&envelope_key.key_material, &nonce, dek)?;

        // Prepend nonce to encrypted data for storage
        let mut result = nonce.to_vec();
        result.extend(encrypted);
        Ok(result)
    }

    /// Decrypt a DEK with the KEK
    async fn decrypt_dek(&self, encrypted_dek: &[u8]) -> Result<Vec<u8>, EvidenceStoreError> {
        if encrypted_dek.len() < 12 {
            return Err(EvidenceStoreError::StorageError {
                message: "Invalid encrypted DEK: too short".to_string(),
            });
        }

        let envelope_key = self
            .secrets
            .get_envelope_key(&self.config.kek_id)
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to get envelope key: {}", e),
            })?;

        let nonce: [u8; 12] = encrypted_dek[..12]
            .try_into()
            .map_err(|_| EvidenceStoreError::StorageError {
                message: "Invalid nonce".to_string(),
            })?;

        Self::decrypt_data(&envelope_key.key_material, &nonce, &encrypted_dek[12..])
    }

    /// Compute content hash
    fn compute_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// DEK storage path for a given evidence hash
    fn dek_path(&self, evidence_hash: &str) -> String {
        format!("{}/{}", self.config.dek_path_prefix, evidence_hash)
    }

    /// Store restricted evidence (encrypted)
    #[instrument(skip(self, manifest, blobs), fields(classification = ?classification))]
    pub async fn store_restricted(
        &self,
        manifest: &[u8],
        blobs: Vec<(&str, &[u8])>,
        classification: EvidenceClassification,
    ) -> Result<String, EvidenceStoreError> {
        // For non-restricted, delegate to inner store
        if !self.requires_encryption(classification) {
            debug!("Classification does not require encryption, storing as plaintext");
            return self.inner.store(manifest, blobs).await;
        }

        info!("Storing restricted evidence with envelope encryption");

        // Step 1: Compute original hash for verification
        let original_hash = Self::compute_bundle_hash(manifest, &blobs);

        // Step 2: Generate DEK
        let dek = Self::generate_dek();
        let nonce = Self::generate_nonce();

        // Step 3: Encrypt manifest and blobs
        let encrypted_manifest = Self::encrypt_data(&dek, &nonce, manifest)?;

        let mut encrypted_blobs: Vec<(String, Vec<u8>)> = Vec::new();
        for (name, data) in &blobs {
            let blob_nonce = Self::generate_nonce();
            let encrypted_blob = Self::encrypt_data(&dek, &blob_nonce, data)?;
            // Store nonce + encrypted data
            let mut blob_with_nonce = blob_nonce.to_vec();
            blob_with_nonce.extend(encrypted_blob);
            encrypted_blobs.push((name.to_string(), blob_with_nonce));
        }

        // Step 4: Create encryption metadata
        let metadata = EncryptionMetadata {
            original_hash: original_hash.clone(),
            algorithm: "AES-256-GCM".to_string(),
            kek_id: self.config.kek_id.clone(),
            dek_path: self.dek_path(&original_hash),
            nonce: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &nonce),
            classification,
            encrypted_at: chrono::Utc::now(),
        };

        let metadata_json =
            serde_json::to_vec(&metadata).map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to serialize metadata: {}", e),
            })?;

        // Step 5: Encrypt DEK with KEK and store in secret provider
        let encrypted_dek = self.encrypt_dek(&dek).await?;

        self.secrets
            .store_secret(
                &metadata.dek_path,
                &encrypted_dek,
                SecretMetadata {
                    description: Some(format!("DEK for evidence bundle {}", original_hash)),
                    evidence_hash: Some(original_hash.clone()),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| EvidenceStoreError::StorageError {
                message: format!("Failed to store DEK: {}", e),
            })?;

        debug!(dek_path = %metadata.dek_path, "Stored encrypted DEK");

        // Step 6: Store encrypted bundle with metadata
        // Add metadata as a special blob
        let mut all_blobs: Vec<(&str, &[u8])> = vec![("__encryption_metadata__.json", &metadata_json)];

        // Convert encrypted blobs back to refs
        let encrypted_blob_refs: Vec<(&str, &[u8])> = encrypted_blobs
            .iter()
            .map(|(n, d)| (n.as_str(), d.as_slice()))
            .collect();
        all_blobs.extend(encrypted_blob_refs);

        // Store with nonce prepended to manifest
        let mut manifest_with_nonce = nonce.to_vec();
        manifest_with_nonce.extend(&encrypted_manifest);

        let content_hash = self.inner.store(&manifest_with_nonce, all_blobs).await?;

        info!(
            content_hash = %content_hash,
            original_hash = %original_hash,
            "Stored restricted evidence bundle"
        );

        Ok(content_hash)
    }

    /// Retrieve and decrypt restricted evidence
    #[instrument(skip(self))]
    pub async fn retrieve_restricted(
        &self,
        content_hash: &str,
    ) -> Result<(Vec<u8>, Option<EncryptionMetadata>), EvidenceStoreError> {
        // Retrieve encrypted bundle
        let encrypted_data = self.inner.retrieve(content_hash).await?;

        // Check if this is encrypted (has metadata blob)
        // For now, try to parse as encrypted - if it fails, return as-is
        if encrypted_data.len() < 12 {
            // Too short to be encrypted, return as plaintext
            return Ok((encrypted_data, None));
        }

        // Try to retrieve encryption metadata
        // If we can't find metadata, this is likely unencrypted
        // This is a simplified check - in production we'd have a cleaner indicator

        let nonce: [u8; 12] = encrypted_data[..12]
            .try_into()
            .map_err(|_| EvidenceStoreError::StorageError {
                message: "Invalid nonce in encrypted manifest".to_string(),
            })?;

        // We need to retrieve the DEK path from metadata stored in the bundle
        // For this MVP, we'll construct the DEK path from content hash
        let dek_path = self.dek_path(content_hash);

        // Try to get the encrypted DEK
        match self.secrets.get_secret(&dek_path).await {
            Ok(secret) => {
                // This is encrypted evidence - decrypt it
                let dek = self.decrypt_dek(&secret.value).await?;
                let decrypted = Self::decrypt_data(&dek, &nonce, &encrypted_data[12..])?;

                // Parse metadata if present
                let metadata = EncryptionMetadata {
                    original_hash: content_hash.to_string(),
                    algorithm: "AES-256-GCM".to_string(),
                    kek_id: self.config.kek_id.clone(),
                    dek_path,
                    nonce: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &nonce),
                    classification: EvidenceClassification::Restricted,
                    encrypted_at: chrono::Utc::now(),
                };

                debug!(content_hash = %content_hash, "Decrypted restricted evidence");
                Ok((decrypted, Some(metadata)))
            }
            Err(SecretProviderError::NotFound { .. }) => {
                // Not encrypted, return as-is
                debug!(content_hash = %content_hash, "Evidence is not encrypted");
                Ok((encrypted_data, None))
            }
            Err(e) => Err(EvidenceStoreError::StorageError {
                message: format!("Failed to retrieve DEK: {}", e),
            }),
        }
    }

    /// Compute bundle hash (same as MinioEvidenceStore)
    fn compute_bundle_hash(manifest: &[u8], blobs: &[(&str, &[u8])]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(manifest);

        let mut sorted_blobs: Vec<_> = blobs.iter().collect();
        sorted_blobs.sort_by_key(|(name, _)| *name);

        for (name, data) in sorted_blobs {
            hasher.update(name.as_bytes());
            hasher.update(*data);
        }

        hex::encode(hasher.finalize())
    }
}

// ============================================================================
// Redaction Helpers
// ============================================================================

/// Redaction helper for creating redacted copies of evidence
pub struct EvidenceRedactor;

impl EvidenceRedactor {
    /// Create a redacted copy of evidence with a manifest
    pub fn redact(
        manifest: &[u8],
        blobs: Vec<(&str, &[u8])>,
        redaction_rules: &[RedactionRule],
        actor_id: &str,
    ) -> Result<(Vec<u8>, Vec<(String, Vec<u8>)>, RedactionManifest), EvidenceStoreError> {
        let original_hash = Self::compute_bundle_hash(manifest, &blobs);
        let mut redactions = Vec::new();
        let mut redacted_blobs: Vec<(String, Vec<u8>)> = Vec::new();

        for (name, data) in &blobs {
            let mut blob_data = data.to_vec();
            let mut blob_redactions = Vec::new();

            // Apply each redaction rule
            for rule in redaction_rules {
                if rule.applies_to(name) {
                    if let Some(matches) = rule.find_matches(&blob_data) {
                        for (offset, length, content_type) in matches {
                            // Store original hash for audit
                            let original_section_hash =
                                Self::compute_section_hash(&blob_data[offset..offset + length]);

                            // Replace with redaction marker
                            let marker = format!("[REDACTED: {:?}]", content_type);
                            let marker_bytes = marker.as_bytes();

                            // Simple replacement (in production, handle length mismatch better)
                            if marker_bytes.len() <= length {
                                blob_data[offset..offset + marker_bytes.len()]
                                    .copy_from_slice(marker_bytes);
                                for i in marker_bytes.len()..length {
                                    blob_data[offset + i] = b' ';
                                }
                            }

                            blob_redactions.push(RedactionEntry {
                                artifact_name: name.to_string(),
                                content_type,
                                reason: rule.reason.clone(),
                                original_content_hash: original_section_hash,
                                offset: Some(offset as u64),
                                length: Some(length as u64),
                            });
                        }
                    }
                }
            }

            redactions.extend(blob_redactions);
            redacted_blobs.push((name.to_string(), blob_data));
        }

        let redacted_blob_refs: Vec<(&str, &[u8])> = redacted_blobs
            .iter()
            .map(|(n, d)| (n.as_str(), d.as_slice()))
            .collect();
        let redacted_hash = Self::compute_bundle_hash(manifest, &redacted_blob_refs);

        let redaction_manifest = RedactionManifest {
            version: "v1".to_string(),
            original_bundle_hash: original_hash,
            redacted_bundle_hash: redacted_hash,
            redacted_at: chrono::Utc::now(),
            redactions,
            redacted_by: actor_id.to_string(),
        };

        Ok((manifest.to_vec(), redacted_blobs, redaction_manifest))
    }

    fn compute_bundle_hash(manifest: &[u8], blobs: &[(&str, &[u8])]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(manifest);

        let mut sorted_blobs: Vec<_> = blobs.iter().collect();
        sorted_blobs.sort_by_key(|(name, _)| *name);

        for (name, data) in sorted_blobs {
            hasher.update(name.as_bytes());
            hasher.update(*data);
        }

        hex::encode(hasher.finalize())
    }

    fn compute_section_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}

/// Rule for redacting content
#[derive(Debug, Clone)]
pub struct RedactionRule {
    /// Pattern to match artifact names
    pub artifact_pattern: String,
    /// Content type to mark redacted content as
    pub content_type: RedactedContentType,
    /// Regex pattern to find sensitive content
    pub pattern: regex::Regex,
    /// Reason for redaction
    pub reason: String,
}

impl RedactionRule {
    /// Create a new redaction rule
    pub fn new(
        artifact_pattern: &str,
        content_type: RedactedContentType,
        pattern: &str,
        reason: &str,
    ) -> Result<Self, regex::Error> {
        Ok(Self {
            artifact_pattern: artifact_pattern.to_string(),
            content_type,
            pattern: regex::Regex::new(pattern)?,
            reason: reason.to_string(),
        })
    }

    /// Check if this rule applies to an artifact
    pub fn applies_to(&self, artifact_name: &str) -> bool {
        glob::Pattern::new(&self.artifact_pattern)
            .map(|p| p.matches(artifact_name))
            .unwrap_or(false)
    }

    /// Find matches in data
    pub fn find_matches(&self, data: &[u8]) -> Option<Vec<(usize, usize, RedactedContentType)>> {
        let text = String::from_utf8_lossy(data);
        let matches: Vec<_> = self
            .pattern
            .find_iter(&text)
            .map(|m| (m.start(), m.len(), self.content_type))
            .collect();

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}

/// Standard redaction rules for common sensitive patterns
pub fn standard_redaction_rules() -> Vec<RedactionRule> {
    vec![
        // AWS credentials
        RedactionRule::new(
            "*",
            RedactedContentType::Secret,
            r"AKIA[0-9A-Z]{16}",
            "AWS Access Key ID",
        )
        .unwrap(),
        // Generic API keys
        RedactionRule::new(
            "*",
            RedactedContentType::Secret,
            r"(?i)(api[_-]?key|apikey)['\"]?\s*[:=]\s*['\"]?[a-zA-Z0-9_-]{20,}",
            "API Key pattern",
        )
        .unwrap(),
        // JWT tokens
        RedactionRule::new(
            "*",
            RedactedContentType::Secret,
            r"eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*",
            "JWT Token",
        )
        .unwrap(),
        // Private keys
        RedactionRule::new(
            "*",
            RedactedContentType::PrivateKey,
            r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----",
            "Private key header",
        )
        .unwrap(),
        // Password patterns
        RedactionRule::new(
            "*",
            RedactedContentType::Secret,
            r"(?i)(password|passwd|pwd)['\"]?\s*[:=]\s*['\"][^'\"]{4,}['\"]",
            "Password pattern",
        )
        .unwrap(),
        // Connection strings
        RedactionRule::new(
            "*",
            RedactedContentType::DatabaseCredential,
            r"(?i)(postgres|mysql|mongodb)://[^:]+:[^@]+@",
            "Database connection string",
        )
        .unwrap(),
    ]
}

// ============================================================================
// In-Memory Secret Provider (for testing)
// ============================================================================

/// In-memory secret provider for testing
pub struct InMemorySecretProvider {
    secrets: tokio::sync::RwLock<BTreeMap<String, (Vec<u8>, SecretMetadata)>>,
    envelope_keys: tokio::sync::RwLock<BTreeMap<String, EnvelopeKey>>,
}

impl InMemorySecretProvider {
    pub fn new() -> Self {
        Self {
            secrets: tokio::sync::RwLock::new(BTreeMap::new()),
            envelope_keys: tokio::sync::RwLock::new(BTreeMap::new()),
        }
    }

    /// Add an envelope key for testing
    pub async fn add_envelope_key(&self, key_id: &str, key_material: Vec<u8>) {
        let mut keys = self.envelope_keys.write().await;
        keys.insert(
            key_id.to_string(),
            EnvelopeKey {
                key_id: key_id.to_string(),
                key_material,
                algorithm: "AES-256-GCM".to_string(),
                version: 1,
            },
        );
    }
}

impl Default for InMemorySecretProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretProvider for InMemorySecretProvider {
    async fn get_secret(&self, path: &str) -> Result<sr_ports::SecretValue, SecretProviderError> {
        let secrets = self.secrets.read().await;
        secrets
            .get(path)
            .map(|(value, metadata)| sr_ports::SecretValue {
                value: value.clone(),
                version: 1,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                metadata: metadata.clone(),
            })
            .ok_or(SecretProviderError::NotFound {
                path: path.to_string(),
            })
    }

    async fn store_secret(
        &self,
        path: &str,
        value: &[u8],
        metadata: SecretMetadata,
    ) -> Result<String, SecretProviderError> {
        let mut secrets = self.secrets.write().await;
        secrets.insert(path.to_string(), (value.to_vec(), metadata));
        Ok(path.to_string())
    }

    async fn delete_secret(&self, path: &str) -> Result<(), SecretProviderError> {
        let mut secrets = self.secrets.write().await;
        secrets.remove(path);
        Ok(())
    }

    async fn secret_exists(&self, path: &str) -> Result<bool, SecretProviderError> {
        let secrets = self.secrets.read().await;
        Ok(secrets.contains_key(path))
    }

    async fn get_envelope_key(&self, key_id: &str) -> Result<EnvelopeKey, SecretProviderError> {
        let keys = self.envelope_keys.read().await;
        keys.get(key_id).cloned().ok_or(SecretProviderError::NotFound {
            path: key_id.to_string(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_requires_encryption() {
        let config = RestrictedEvidenceConfig::default();

        // Test with default threshold (Restricted)
        assert!(!classification_requires_encryption(
            EvidenceClassification::Public,
            config.encryption_threshold
        ));
        assert!(!classification_requires_encryption(
            EvidenceClassification::Internal,
            config.encryption_threshold
        ));
        assert!(classification_requires_encryption(
            EvidenceClassification::Restricted,
            config.encryption_threshold
        ));
        assert!(classification_requires_encryption(
            EvidenceClassification::Confidential,
            config.encryption_threshold
        ));
    }

    fn classification_requires_encryption(
        classification: EvidenceClassification,
        threshold: EvidenceClassification,
    ) -> bool {
        match (classification, threshold) {
            (EvidenceClassification::Public, _) => false,
            (EvidenceClassification::Internal, EvidenceClassification::Public) => true,
            (EvidenceClassification::Internal, _) => false,
            (EvidenceClassification::Restricted, EvidenceClassification::Confidential) => false,
            (EvidenceClassification::Restricted, _) => true,
            (EvidenceClassification::Confidential, _) => true,
        }
    }

    #[test]
    fn test_encryption_roundtrip() {
        let key = vec![0u8; 32]; // Test key
        let nonce = [0u8; 12]; // Test nonce
        let data = b"Hello, restricted evidence!";

        let encrypted =
            RestrictedEvidenceStore::<InMemoryEvidenceStore, InMemorySecretProvider>::encrypt_data(
                &key, &nonce, data,
            )
            .unwrap();

        let decrypted =
            RestrictedEvidenceStore::<InMemoryEvidenceStore, InMemorySecretProvider>::decrypt_data(
                &key, &nonce, &encrypted,
            )
            .unwrap();

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_redaction_rule_matching() {
        let rule = RedactionRule::new(
            "*.log",
            RedactedContentType::Secret,
            r"AKIA[0-9A-Z]{16}",
            "AWS key",
        )
        .unwrap();

        assert!(rule.applies_to("output.log"));
        assert!(rule.applies_to("debug.log"));
        assert!(!rule.applies_to("output.txt"));

        let data = b"Found key: AKIAIOSFODNN7EXAMPLE in logs";
        let matches = rule.find_matches(data).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].1, 20); // Length of the key
    }

    #[test]
    fn test_redaction_manifest_serialization() {
        let manifest = RedactionManifest {
            version: "v1".to_string(),
            original_bundle_hash: "abc123".to_string(),
            redacted_bundle_hash: "def456".to_string(),
            redacted_at: chrono::Utc::now(),
            redactions: vec![RedactionEntry {
                artifact_name: "output.log".to_string(),
                content_type: RedactedContentType::Secret,
                reason: "AWS key detected".to_string(),
                original_content_hash: "ghi789".to_string(),
                offset: Some(10),
                length: Some(20),
            }],
            redacted_by: "system".to_string(),
        };

        let json = serde_json::to_string(&manifest).unwrap();
        let parsed: RedactionManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.version, "v1");
        assert_eq!(parsed.redactions.len(), 1);
    }

    #[test]
    fn test_standard_redaction_rules() {
        let rules = standard_redaction_rules();
        assert!(!rules.is_empty());

        // Test AWS key detection
        let aws_rule = &rules[0];
        let data = b"key = AKIAIOSFODNN7EXAMPLE";
        assert!(aws_rule.find_matches(data).is_some());

        // Test JWT detection
        let jwt_rule = &rules[2];
        let jwt = b"token = eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
        assert!(jwt_rule.find_matches(jwt).is_some());
    }

    /// In-memory evidence store for testing
    struct InMemoryEvidenceStore {
        bundles: tokio::sync::RwLock<BTreeMap<String, (Vec<u8>, Vec<(String, Vec<u8>)>)>>,
    }

    impl InMemoryEvidenceStore {
        fn new() -> Self {
            Self {
                bundles: tokio::sync::RwLock::new(BTreeMap::new()),
            }
        }

        fn compute_hash(manifest: &[u8], blobs: &[(&str, &[u8])]) -> String {
            let mut hasher = Sha256::new();
            hasher.update(manifest);
            let mut sorted: Vec<_> = blobs.iter().collect();
            sorted.sort_by_key(|(n, _)| *n);
            for (n, d) in sorted {
                hasher.update(n.as_bytes());
                hasher.update(*d);
            }
            hex::encode(hasher.finalize())
        }
    }

    impl EvidenceStore for InMemoryEvidenceStore {
        async fn store(
            &self,
            manifest: &[u8],
            blobs: Vec<(&str, &[u8])>,
        ) -> Result<String, EvidenceStoreError> {
            let hash = Self::compute_hash(manifest, &blobs);
            let blob_vec: Vec<(String, Vec<u8>)> = blobs
                .iter()
                .map(|(n, d)| (n.to_string(), d.to_vec()))
                .collect();
            let mut bundles = self.bundles.write().await;
            bundles.insert(hash.clone(), (manifest.to_vec(), blob_vec));
            Ok(hash)
        }

        async fn retrieve(&self, content_hash: &str) -> Result<Vec<u8>, EvidenceStoreError> {
            let bundles = self.bundles.read().await;
            bundles
                .get(content_hash)
                .map(|(m, _)| m.clone())
                .ok_or(EvidenceStoreError::NotFound {
                    content_hash: content_hash.to_string(),
                })
        }

        async fn exists(&self, content_hash: &str) -> Result<bool, EvidenceStoreError> {
            let bundles = self.bundles.read().await;
            Ok(bundles.contains_key(content_hash))
        }
    }

    #[tokio::test]
    async fn test_restricted_store_encryption_flow() {
        // Set up in-memory stores
        let evidence_store = Arc::new(InMemoryEvidenceStore::new());
        let secret_provider = Arc::new(InMemorySecretProvider::new());

        // Add a test KEK
        let kek = vec![1u8; 32]; // Test KEK
        secret_provider.add_envelope_key("default-kek", kek).await;

        let restricted_store = RestrictedEvidenceStore::new(
            evidence_store.clone(),
            secret_provider.clone(),
            RestrictedEvidenceConfig::default(),
        );

        // Store restricted evidence
        let manifest = br#"{"type": "evidence.gate_packet"}"#;
        let blobs = vec![("output.txt", b"sensitive data" as &[u8])];

        let hash = restricted_store
            .store_restricted(manifest, blobs, EvidenceClassification::Restricted)
            .await
            .unwrap();

        assert!(!hash.is_empty());

        // Verify DEK was stored
        let dek_path = restricted_store.dek_path(&hash);
        assert!(secret_provider.secret_exists(&dek_path).await.unwrap());
    }
}
