//! OIDC Authentication Module (D-17)
//!
//! Implements JWT validation for Zitadel OIDC tokens.
//! Per SR-SPEC §2.1, the identity provider validates OIDC JWTs
//! and derives actor identity for request authorization.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sr_domain::ActorKind;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// OIDC configuration for Zitadel
#[derive(Debug, Clone)]
pub struct OidcConfig {
    /// Issuer URL (e.g., "https://auth.example.com")
    pub issuer: String,
    /// Expected audience
    pub audience: String,
    /// JWKS endpoint (defaults to {issuer}/.well-known/jwks.json)
    pub jwks_uri: Option<String>,
    /// Whether to skip validation (for testing only)
    pub skip_validation: bool,
}

impl OidcConfig {
    /// Create configuration for local development
    pub fn local_dev() -> Self {
        Self {
            issuer: "http://localhost:8080".to_string(),
            audience: "solver-ralph".to_string(),
            jwks_uri: None,
            skip_validation: false,
        }
    }

    /// Create configuration with validation disabled (testing only)
    pub fn test_mode() -> Self {
        Self {
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            jwks_uri: None,
            skip_validation: true,
        }
    }

    /// Get the JWKS URI
    pub fn jwks_uri(&self) -> String {
        self.jwks_uri
            .clone()
            .unwrap_or_else(|| format!("{}/.well-known/jwks.json", self.issuer))
    }
}

/// OIDC provider for JWT validation
#[derive(Clone)]
pub struct OidcProvider {
    config: OidcConfig,
    jwks: Arc<RwLock<Option<JwkSet>>>,
    http_client: reqwest::Client,
}

impl OidcProvider {
    /// Create a new OIDC provider
    pub fn new(config: OidcConfig) -> Self {
        Self {
            config,
            jwks: Arc::new(RwLock::new(None)),
            http_client: reqwest::Client::new(),
        }
    }

    /// Initialize the provider by fetching JWKS
    pub async fn initialize(&self) -> Result<(), AuthError> {
        if self.config.skip_validation {
            info!("OIDC validation disabled (test mode)");
            return Ok(());
        }

        self.refresh_jwks().await
    }

    /// Refresh the JWKS from the issuer
    pub async fn refresh_jwks(&self) -> Result<(), AuthError> {
        let jwks_uri = self.config.jwks_uri();
        debug!(uri = %jwks_uri, "Fetching JWKS");

        let response = self
            .http_client
            .get(&jwks_uri)
            .send()
            .await
            .map_err(|e| AuthError::ProviderError {
                message: format!("Failed to fetch JWKS: {}", e),
            })?;

        if !response.status().is_success() {
            return Err(AuthError::ProviderError {
                message: format!("JWKS endpoint returned status: {}", response.status()),
            });
        }

        let jwks: JwkSet = response
            .json()
            .await
            .map_err(|e| AuthError::ProviderError {
                message: format!("Failed to parse JWKS: {}", e),
            })?;

        let mut guard = self.jwks.write().await;
        *guard = Some(jwks);

        info!("JWKS refreshed successfully");
        Ok(())
    }

    /// Validate a JWT token and return the identity
    pub async fn validate_token(&self, token: &str) -> Result<AuthenticatedUser, AuthError> {
        if self.config.skip_validation {
            // In test mode, create a mock identity
            return Ok(AuthenticatedUser {
                actor_kind: ActorKind::Human,
                actor_id: "test-user".to_string(),
                subject: "test-subject".to_string(),
                email: Some("test@example.com".to_string()),
                name: Some("Test User".to_string()),
                roles: vec!["admin".to_string()],
                claims: serde_json::json!({}),
            });
        }

        // Decode header to get key ID
        let header = decode_header(token).map_err(|e| AuthError::InvalidToken {
            reason: format!("Invalid JWT header: {}", e),
        })?;

        let kid = header.kid.ok_or(AuthError::InvalidToken {
            reason: "Token missing 'kid' claim".to_string(),
        })?;

        // Get decoding key from JWKS
        let jwks_guard = self.jwks.read().await;
        let jwks = jwks_guard.as_ref().ok_or(AuthError::ProviderError {
            message: "JWKS not initialized".to_string(),
        })?;

        let jwk = jwks.find(&kid).ok_or(AuthError::InvalidToken {
            reason: format!("Unknown key ID: {}", kid),
        })?;

        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| AuthError::ProviderError {
            message: format!("Failed to create decoding key: {}", e),
        })?;

        // Configure validation
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&[&self.config.issuer]);

        // Decode and validate token
        let token_data = decode::<TokenClaims>(token, &decoding_key, &validation).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidAudience => AuthError::InvalidToken {
                    reason: "Invalid audience".to_string(),
                },
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthError::InvalidToken {
                    reason: "Invalid issuer".to_string(),
                },
                _ => AuthError::InvalidToken {
                    reason: format!("Token validation failed: {}", e),
                },
            }
        })?;

        let claims = token_data.claims;

        // Derive actor kind from claims
        let actor_kind = if claims.roles.contains(&"system".to_string()) {
            ActorKind::System
        } else if claims.roles.contains(&"agent".to_string()) {
            ActorKind::Agent
        } else {
            ActorKind::Human
        };

        Ok(AuthenticatedUser {
            actor_kind,
            actor_id: claims.sub.clone(),
            subject: claims.sub,
            email: claims.email,
            name: claims.name,
            roles: claims.roles,
            claims: serde_json::to_value(&claims).unwrap_or_default(),
        })
    }
}

/// JWT token claims (Zitadel format)
#[derive(Debug, Deserialize)]
struct TokenClaims {
    /// Subject (user ID)
    sub: String,
    /// Issued at
    iat: i64,
    /// Expiration
    exp: i64,
    /// Issuer
    iss: String,
    /// Audience
    aud: Vec<String>,
    /// Email (optional)
    #[serde(default)]
    email: Option<String>,
    /// Name (optional)
    #[serde(default)]
    name: Option<String>,
    /// Roles (Zitadel custom claim)
    #[serde(default, rename = "urn:zitadel:iam:org:project:roles")]
    zitadel_roles: Option<serde_json::Value>,
    /// Simplified roles array
    #[serde(default)]
    roles: Vec<String>,
}

/// Authenticated user identity
#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
    /// Actor kind (Human, Agent, System)
    pub actor_kind: ActorKind,
    /// Actor ID (unique identifier)
    pub actor_id: String,
    /// OIDC subject
    pub subject: String,
    /// Email address (if available)
    pub email: Option<String>,
    /// Display name (if available)
    pub name: Option<String>,
    /// Roles assigned to the user
    pub roles: Vec<String>,
    /// Raw claims (for extensibility)
    pub claims: serde_json::Value,
}

impl AuthenticatedUser {
    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if user is a system actor
    pub fn is_system(&self) -> bool {
        matches!(self.actor_kind, ActorKind::System)
    }

    /// Check if user is a human actor
    pub fn is_human(&self) -> bool {
        matches!(self.actor_kind, ActorKind::Human)
    }

    /// Check if user is an agent actor
    pub fn is_agent(&self) -> bool {
        matches!(self.actor_kind, ActorKind::Agent)
    }
}

/// Authentication errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    MissingAuthHeader,

    #[error("Invalid authorization header format")]
    InvalidAuthHeader,

    #[error("Invalid token: {reason}")]
    InvalidToken { reason: String },

    #[error("Token expired")]
    TokenExpired,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Provider error: {message}")]
    ProviderError { message: String },
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AuthError::MissingAuthHeader => (StatusCode::UNAUTHORIZED, "Missing authorization"),
            AuthError::InvalidAuthHeader => (StatusCode::UNAUTHORIZED, "Invalid authorization"),
            AuthError::InvalidToken { .. } => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
            AuthError::ProviderError { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error")
            }
        };

        let body = Json(serde_json::json!({
            "error": message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

/// Global OIDC provider instance
static OIDC_PROVIDER: OnceCell<OidcProvider> = OnceCell::new();

/// Initialize the global OIDC provider
pub async fn init_oidc(config: OidcConfig) -> Result<(), AuthError> {
    let provider = OidcProvider::new(config);
    provider.initialize().await?;
    OIDC_PROVIDER
        .set(provider)
        .map_err(|_| AuthError::ProviderError {
            message: "OIDC provider already initialized".to_string(),
        })?;
    Ok(())
}

/// Get the global OIDC provider
pub fn get_oidc_provider() -> Option<&'static OidcProvider> {
    OIDC_PROVIDER.get()
}

/// Axum extractor for authenticated users
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AuthError::MissingAuthHeader)?;

        let auth_str = auth_header.to_str().map_err(|_| AuthError::InvalidAuthHeader)?;

        // Parse Bearer token
        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidAuthHeader)?;

        // Validate token
        let provider = get_oidc_provider().ok_or(AuthError::ProviderError {
            message: "OIDC provider not initialized".to_string(),
        })?;

        provider.validate_token(token).await
    }
}

/// Optional authenticated user extractor (doesn't fail on missing auth)
pub struct OptionalAuth(pub Option<AuthenticatedUser>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthenticatedUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuth(Some(user))),
            Err(_) => Ok(OptionalAuth(None)),
        }
    }
}

/// Require specific role middleware
pub fn require_role(user: &AuthenticatedUser, role: &str) -> Result<(), AuthError> {
    if user.has_role(role) || user.is_system() {
        Ok(())
    } else {
        Err(AuthError::InsufficientPermissions)
    }
}

/// Require human actor middleware
pub fn require_human(user: &AuthenticatedUser) -> Result<(), AuthError> {
    if user.is_human() {
        Ok(())
    } else {
        Err(AuthError::InsufficientPermissions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_test_mode_validation() {
        let config = OidcConfig::test_mode();
        let provider = OidcProvider::new(config);
        provider.initialize().await.unwrap();

        let user = provider.validate_token("any-token").await.unwrap();
        assert_eq!(user.actor_id, "test-user");
        assert!(user.has_role("admin"));
    }

    #[test]
    fn test_authenticated_user_roles() {
        let user = AuthenticatedUser {
            actor_kind: ActorKind::Human,
            actor_id: "user-1".to_string(),
            subject: "sub-1".to_string(),
            email: None,
            name: None,
            roles: vec!["admin".to_string(), "reviewer".to_string()],
            claims: serde_json::json!({}),
        };

        assert!(user.has_role("admin"));
        assert!(user.has_role("reviewer"));
        assert!(!user.has_role("system"));
        assert!(user.is_human());
        assert!(!user.is_system());
    }

    #[test]
    fn test_auth_error_response() {
        let error = AuthError::TokenExpired;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
