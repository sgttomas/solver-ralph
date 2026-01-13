//! Infisical Secret Provider Adapter (D-16)
//!
//! Implements the SecretProvider port using Infisical for secrets management.
//! Per SR-SPEC and SR-PLAN D-16, this adapter provides:
//! - Secret retrieval and storage
//! - Envelope key management for restricted evidence
//! - Auditable key retrieval paths
//!
//! Infisical API reference: https://infisical.com/docs/api-reference

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
            token: std::env::var("INFISICAL_TOKEN").map_err(|_| SecretProviderError::ProviderError {
                message: "INFISICAL_TOKEN environment variable not set".to_string(),
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
        let mut url = format!(
            "{}/api/v3/secrets/raw/{}",
            self.config.api_endpoint, key
        );

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
            response.json::<T>().await.map_err(|e| {
                SecretProviderError::ProviderError {
                    message: format!("Failed to parse response: {}", e),
                }
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
