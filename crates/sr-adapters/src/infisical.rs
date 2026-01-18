//! Infisical Secret Provider Adapter (D-16)
//!
//! Implements the SecretProvider port using Infisical for secrets management.
//! Per SR-SPEC and SR-PLAN D-16, this adapter provides:
//! - Secret retrieval and storage
//! - Envelope key management for restricted evidence
//! - Auditable key retrieval paths
//!
//! Infisical API reference: https://infisical.com/docs/api-reference

use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sr_ports::{EnvelopeKey, SecretMetadata, SecretProvider, SecretProviderError, SecretValue};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

/// Infisical configuration
#[derive(Debug, Clone)]
pub struct InfisicalConfig {
    /// Infisical API endpoint
    pub api_endpoint: String,
    /// Service token or API key
    pub token: String,
    /// Default environment (e.g., "dev", "staging", "prod")
    pub environment: String,
    /// Default workspace ID
    pub workspace_id: String,
    /// Key ID for envelope encryption (KEK)
    pub envelope_key_id: String,
}

impl InfisicalConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Result<Self, SecretProviderError> {
        Ok(Self {
            api_endpoint: std::env::var("INFISICAL_API_ENDPOINT")
                .unwrap_or_else(|_| "https://app.infisical.com".to_string()),
            token: std::env::var("INFISICAL_TOKEN").map_err(|_| {
                SecretProviderError::ProviderError {
                    message: "INFISICAL_TOKEN environment variable not set".to_string(),
                }
            })?,
            environment: std::env::var("INFISICAL_ENVIRONMENT")
                .unwrap_or_else(|_| "dev".to_string()),
            workspace_id: std::env::var("INFISICAL_WORKSPACE_ID").map_err(|_| {
                SecretProviderError::ProviderError {
                    message: "INFISICAL_WORKSPACE_ID environment variable not set".to_string(),
                }
            })?,
            envelope_key_id: std::env::var("INFISICAL_ENVELOPE_KEY_ID")
                .unwrap_or_else(|_| "solver-ralph-kek".to_string()),
        })
    }

    /// Create configuration for local development
    pub fn local_dev() -> Self {
        Self {
            api_endpoint: "http://localhost:8080".to_string(),
            token: "dev-token".to_string(),
            environment: "dev".to_string(),
            workspace_id: "dev-workspace".to_string(),
            envelope_key_id: "solver-ralph-kek".to_string(),
        }
    }
}

/// Infisical API response for a secret
#[derive(Debug, Deserialize)]
struct InfisicalSecretResponse {
    secret: InfisicalSecret,
}

/// Infisical secret structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InfisicalSecret {
    id: String,
    workspace: String,
    environment: String,
    secret_key: String,
    secret_value: String,
    version: u64,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

/// Request body for creating/updating a secret
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSecretRequest {
    workspace_id: String,
    environment: String,
    secret_key: String,
    secret_value: String,
    #[serde(rename = "type")]
    secret_type: String,
}

/// Infisical-backed secret provider
pub struct InfisicalSecretProvider {
    config: InfisicalConfig,
    client: Client,
    /// Cache for envelope keys (KEKs should rarely change)
    key_cache: Arc<RwLock<HashMap<String, EnvelopeKey>>>,
}

impl InfisicalSecretProvider {
    /// Create a new Infisical secret provider
    pub fn new(config: InfisicalConfig) -> Result<Self, SecretProviderError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| SecretProviderError::ConnectionError {
                message: format!("Failed to create HTTP client: {}", e),
            })?;

        Ok(Self {
            config,
            client,
            key_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Parse a secret path into components
    /// Path format: "folder/subfolder/secret_key" or just "secret_key"
    fn parse_path(path: &str) -> (Option<String>, String) {
        if let Some(pos) = path.rfind('/') {
            let folder = &path[..pos];
            let key = &path[pos + 1..];
            (Some(folder.to_string()), key.to_string())
        } else {
            (None, path.to_string())
        }
    }

    /// Build the API URL for a secret
    fn secret_url(&self, path: &str) -> String {
        let (folder, key) = Self::parse_path(path);
        let mut url = format!("{}/api/v3/secrets/raw/{}", self.config.api_endpoint, key);

        url.push_str(&format!(
            "?workspaceId={}&environment={}",
            self.config.workspace_id, self.config.environment
        ));

        if let Some(f) = folder {
            url.push_str(&format!("&secretPath=/{}", f));
        }

        url
    }

    /// Make an authenticated request
    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, SecretProviderError> {
        let response = request
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| SecretProviderError::ConnectionError {
                message: format!("Request failed: {}", e),
            })?;

        let status = response.status();
        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| SecretProviderError::ProviderError {
                    message: format!("Failed to parse response: {}", e),
                })
        } else if status.as_u16() == 404 {
            Err(SecretProviderError::NotFound {
                path: "unknown".to_string(),
            })
        } else if status.as_u16() == 403 || status.as_u16() == 401 {
            Err(SecretProviderError::AccessDenied {
                reason: format!("HTTP {}", status),
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(SecretProviderError::ProviderError {
                message: format!("HTTP {}: {}", status, error_text),
            })
        }
    }
}

impl SecretProvider for InfisicalSecretProvider {
    #[instrument(skip(self), fields(path = %path))]
    async fn get_secret(&self, path: &str) -> Result<SecretValue, SecretProviderError> {
        let url = self.secret_url(path);
        debug!(url = %url, "Fetching secret");

        let response: InfisicalSecretResponse = self
            .make_request(self.client.get(&url))
            .await
            .map_err(|e| match e {
                SecretProviderError::NotFound { .. } => SecretProviderError::NotFound {
                    path: path.to_string(),
                },
                other => other,
            })?;

        let secret = response.secret;

        // Parse timestamps
        let created_at = chrono::DateTime::parse_from_rfc3339(&secret.created_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        let updated_at = chrono::DateTime::parse_from_rfc3339(&secret.updated_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

        // Decode base64 value if it looks like base64, otherwise use raw
        let value = if secret.secret_value.starts_with("base64:") {
            base64::Engine::decode(
                &base64::engine::general_purpose::STANDARD,
                &secret.secret_value[7..],
            )
            .unwrap_or_else(|_| secret.secret_value.into_bytes())
        } else {
            secret.secret_value.into_bytes()
        };

        info!(path = %path, version = secret.version, "Retrieved secret");

        Ok(SecretValue {
            value,
            version: secret.version,
            created_at,
            updated_at,
            metadata: SecretMetadata::default(),
        })
    }

    #[instrument(skip(self, value, metadata), fields(path = %path))]
    async fn store_secret(
        &self,
        path: &str,
        value: &[u8],
        metadata: SecretMetadata,
    ) -> Result<String, SecretProviderError> {
        let (_folder, key) = Self::parse_path(path);

        // Encode value as base64 for safe storage
        let encoded_value = format!(
            "base64:{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, value)
        );

        let request = CreateSecretRequest {
            workspace_id: self.config.workspace_id.clone(),
            environment: self.config.environment.clone(),
            secret_key: key,
            secret_value: encoded_value,
            secret_type: "shared".to_string(),
        };

        let url = format!("{}/api/v3/secrets/raw", self.config.api_endpoint);

        // Try to create, if exists try to update
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| SecretProviderError::ConnectionError {
                message: format!("Request failed: {}", e),
            })?;

        let status = response.status();
        if status.is_success() {
            info!(path = %path, "Stored secret");
            Ok(path.to_string())
        } else if status.as_u16() == 400 {
            // Secret might already exist, try PATCH
            warn!(path = %path, "Secret may exist, attempting update");

            let patch_url = self.secret_url(path);
            let patch_response = self
                .client
                .patch(&patch_url)
                .header("Authorization", format!("Bearer {}", self.config.token))
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "secretValue": request.secret_value
                }))
                .send()
                .await
                .map_err(|e| SecretProviderError::ConnectionError {
                    message: format!("Update request failed: {}", e),
                })?;

            if patch_response.status().is_success() {
                info!(path = %path, "Updated existing secret");
                Ok(path.to_string())
            } else {
                let error_text = patch_response.text().await.unwrap_or_default();
                Err(SecretProviderError::ProviderError {
                    message: format!("Failed to store/update secret: {}", error_text),
                })
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(SecretProviderError::ProviderError {
                message: format!("Failed to store secret: {}", error_text),
            })
        }
    }

    #[instrument(skip(self), fields(path = %path))]
    async fn delete_secret(&self, path: &str) -> Result<(), SecretProviderError> {
        let url = self.secret_url(path);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .send()
            .await
            .map_err(|e| SecretProviderError::ConnectionError {
                message: format!("Request failed: {}", e),
            })?;

        if response.status().is_success() {
            info!(path = %path, "Deleted secret");
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(SecretProviderError::ProviderError {
                message: format!("Failed to delete secret: {}", error_text),
            })
        }
    }

    #[instrument(skip(self), fields(path = %path))]
    async fn secret_exists(&self, path: &str) -> Result<bool, SecretProviderError> {
        match self.get_secret(path).await {
            Ok(_) => Ok(true),
            Err(SecretProviderError::NotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    #[instrument(skip(self), fields(key_id = %key_id))]
    async fn get_envelope_key(&self, key_id: &str) -> Result<EnvelopeKey, SecretProviderError> {
        // Check cache first
        {
            let cache = self.key_cache.read().await;
            if let Some(key) = cache.get(key_id) {
                debug!(key_id = %key_id, "Returning cached envelope key");
                return Ok(key.clone());
            }
        }

        // Fetch from Infisical
        // KEK path convention: solver-ralph/kek/{key_id}
        let kek_path = format!("solver-ralph/kek/{}", key_id);
        let secret = self.get_secret(&kek_path).await?;

        // KEK should be 32 bytes for AES-256
        if secret.value.len() != 32 {
            return Err(SecretProviderError::EncryptionError {
                message: format!(
                    "Invalid KEK length: expected 32 bytes, got {}",
                    secret.value.len()
                ),
            });
        }

        let envelope_key = EnvelopeKey {
            key_id: key_id.to_string(),
            key_material: secret.value,
            algorithm: "AES-256-GCM".to_string(),
            version: secret.version,
        };

        // Cache the key
        {
            let mut cache = self.key_cache.write().await;
            cache.insert(key_id.to_string(), envelope_key.clone());
        }

        info!(key_id = %key_id, version = envelope_key.version, "Retrieved envelope key");
        Ok(envelope_key)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path() {
        let (folder, key) = InfisicalSecretProvider::parse_path("simple-key");
        assert!(folder.is_none());
        assert_eq!(key, "simple-key");

        let (folder, key) = InfisicalSecretProvider::parse_path("folder/key");
        assert_eq!(folder, Some("folder".to_string()));
        assert_eq!(key, "key");

        let (folder, key) = InfisicalSecretProvider::parse_path("a/b/c/key");
        assert_eq!(folder, Some("a/b/c".to_string()));
        assert_eq!(key, "key");
    }

    #[test]
    fn test_config_local_dev() {
        let config = InfisicalConfig::local_dev();
        assert_eq!(config.environment, "dev");
        assert_eq!(config.api_endpoint, "http://localhost:8080");
    }
}

// ============================================================================
// Integration Tests with Mock Infisical (V11-1)
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn should_run_integration() -> bool {
        std::env::var("RUN_INFISICAL_TESTS").is_ok()
    }

    /// Create a test config pointing to a mock server
    fn test_config(mock_url: &str) -> InfisicalConfig {
        InfisicalConfig {
            api_endpoint: mock_url.to_string(),
            token: "test-token".to_string(),
            environment: "dev".to_string(),
            workspace_id: "test-workspace".to_string(),
            envelope_key_id: "test-kek".to_string(),
        }
    }

    #[tokio::test]
    async fn test_get_secret_success() {
        if !should_run_integration() {
            return;
        }
        // Start mock server
        let mock_server = MockServer::start().await;

        // Setup mock response
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/test-secret.*"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "secret-123",
                    "workspace": "test-workspace",
                    "environment": "dev",
                    "secretKey": "test-secret",
                    "secretValue": "secret-value-123",
                    "version": 1,
                    "createdAt": "2026-01-17T00:00:00Z",
                    "updatedAt": "2026-01-17T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        // Create provider and get secret
        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_secret("test-secret").await;
        assert!(result.is_ok());

        let secret = result.unwrap();
        assert_eq!(secret.value, b"secret-value-123");
        assert_eq!(secret.version, 1);
    }

    #[tokio::test]
    async fn test_get_secret_with_folder() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/my-key.*"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "secret-456",
                    "workspace": "test-workspace",
                    "environment": "dev",
                    "secretKey": "my-key",
                    "secretValue": "nested-value",
                    "version": 2,
                    "createdAt": "2026-01-17T00:00:00Z",
                    "updatedAt": "2026-01-17T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_secret("folder/subfolder/my-key").await;
        assert!(result.is_ok());

        let secret = result.unwrap();
        assert_eq!(secret.value, b"nested-value");
    }

    #[tokio::test]
    async fn test_get_secret_not_found() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/nonexistent.*"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_secret("nonexistent").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            SecretProviderError::NotFound { path } => {
                assert_eq!(path, "nonexistent");
            }
            e => panic!("Expected NotFound error, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get_secret_unauthorized() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/.*"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_secret("some-secret").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            SecretProviderError::AccessDenied { reason } => {
                assert!(reason.contains("401"));
            }
            e => panic!("Expected AccessDenied error, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get_secret_forbidden() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/.*"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_secret("forbidden-secret").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            SecretProviderError::AccessDenied { reason } => {
                assert!(reason.contains("403"));
            }
            e => panic!("Expected AccessDenied error, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get_envelope_key_success() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        // KEK is 32 bytes for AES-256, base64 encoded with "base64:" prefix
        let kek_bytes: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let kek_b64 = format!(
            "base64:{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &kek_bytes)
        );

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/test-kek.*"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "kek-123",
                    "workspace": "test-workspace",
                    "environment": "dev",
                    "secretKey": "test-kek",
                    "secretValue": kek_b64,
                    "version": 1,
                    "createdAt": "2026-01-17T00:00:00Z",
                    "updatedAt": "2026-01-17T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_envelope_key("solver-ralph/kek/test-kek").await;
        assert!(result.is_ok());

        let key = result.unwrap();
        assert_eq!(key.key_material.len(), 32);
        assert_eq!(key.algorithm, "AES-256-GCM");
        assert_eq!(key.version, 1);
    }

    #[tokio::test]
    async fn test_get_envelope_key_invalid_length() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        // Return a key that's not 32 bytes (only 16 bytes)
        let short_key: [u8; 16] = [0u8; 16];
        let short_b64 = format!(
            "base64:{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &short_key)
        );

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/bad-kek.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "kek-bad",
                    "workspace": "test-workspace",
                    "environment": "dev",
                    "secretKey": "bad-kek",
                    "secretValue": short_b64,
                    "version": 1,
                    "createdAt": "2026-01-17T00:00:00Z",
                    "updatedAt": "2026-01-17T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_envelope_key("solver-ralph/kek/bad-kek").await;
        assert!(result.is_err());

        match result.unwrap_err() {
            SecretProviderError::EncryptionError { message } => {
                assert!(message.contains("Invalid KEK length"));
            }
            e => panic!("Expected EncryptionError, got: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get_envelope_key_caching() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        let kek_bytes: [u8; 32] = [0x42u8; 32];
        let kek_b64 = format!(
            "base64:{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &kek_bytes)
        );

        // This mock should only be called once due to caching
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/cached-kek.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "kek-cached",
                    "workspace": "test-workspace",
                    "environment": "dev",
                    "secretKey": "cached-kek",
                    "secretValue": kek_b64,
                    "version": 5,
                    "createdAt": "2026-01-17T00:00:00Z",
                    "updatedAt": "2026-01-17T00:00:00Z"
                }
            })))
            .expect(1) // Should only be called once
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        // First call - fetches from server
        let key1 = provider
            .get_envelope_key("solver-ralph/kek/cached-kek")
            .await
            .unwrap();
        assert_eq!(key1.version, 5);

        // Second call - should use cache
        let key2 = provider
            .get_envelope_key("solver-ralph/kek/cached-kek")
            .await
            .unwrap();
        assert_eq!(key2.version, 5);

        // Third call - should still use cache
        let key3 = provider
            .get_envelope_key("solver-ralph/kek/cached-kek")
            .await
            .unwrap();
        assert_eq!(key3.version, 5);

        // Mock expectation of 1 call will verify caching worked
    }

    #[tokio::test]
    async fn test_secret_exists_true() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/exists-secret.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "secret-exists",
                    "workspace": "test-workspace",
                    "environment": "dev",
                    "secretKey": "exists-secret",
                    "secretValue": "value",
                    "version": 1,
                    "createdAt": "2026-01-17T00:00:00Z",
                    "updatedAt": "2026-01-17T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.secret_exists("exists-secret").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_secret_exists_false() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/missing-secret.*"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.secret_exists("missing-secret").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_store_secret_success() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path_regex(r"/api/v3/secrets/raw"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secret": {
                    "id": "new-secret",
                    "secretKey": "my-new-secret"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider
            .store_secret(
                "my-new-secret",
                b"my-secret-value",
                SecretMetadata::default(),
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "my-new-secret");
    }

    #[tokio::test]
    async fn test_delete_secret_success() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v3/secrets/raw/delete-me.*"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.delete_secret("delete-me").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_network_timeout() {
        if !should_run_integration() {
            return;
        }
        let mock_server = MockServer::start().await;

        // Respond with a long delay to trigger timeout
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v3/secrets/raw/slow-secret.*"))
            .respond_with(ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(35)))
            .mount(&mock_server)
            .await;

        let config = test_config(&mock_server.uri());
        let provider = InfisicalSecretProvider::new(config).unwrap();

        let result = provider.get_secret("slow-secret").await;
        assert!(result.is_err());

        // The error could be a timeout or a connection error depending on timing
        match result.unwrap_err() {
            SecretProviderError::ConnectionError { message } => {
                // Accept any connection-related error (timeout, request error, etc.)
                assert!(
                    message.contains("timeout")
                        || message.contains("timed out")
                        || message.contains("error sending request")
                        || message.contains("Request failed"),
                    "Expected connection-related error, got: {}",
                    message
                );
            }
            e => panic!("Expected ConnectionError, got: {:?}", e),
        }
    }
}
