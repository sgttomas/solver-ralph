//! API Configuration Module
//!
//! Centralized configuration for the SOLVER-Ralph API service.

use crate::auth::OidcConfig;
use serde::Deserialize;
use std::env;

/// API server configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// OIDC configuration
    pub oidc: OidcConfig,
    /// Database URL
    pub database_url: String,
    /// MinIO endpoint
    pub minio_endpoint: String,
    /// MinIO access key
    pub minio_access_key: String,
    /// MinIO secret key
    pub minio_secret_key: String,
    /// MinIO bucket for evidence
    pub minio_bucket: String,
    /// NATS URL
    pub nats_url: String,
    /// Log level
    pub log_level: String,
}

impl ApiConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            host: env::var("SR_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SR_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            oidc: OidcConfig {
                issuer: env::var("SR_OIDC_ISSUER")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
                audience: env::var("SR_OIDC_AUDIENCE")
                    .unwrap_or_else(|_| "solver-ralph".to_string()),
                jwks_uri: env::var("SR_OIDC_JWKS_URI").ok(),
                skip_validation: env::var("SR_OIDC_SKIP_VALIDATION")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(false),
            },
            database_url: env::var("SR_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/solver_ralph".to_string()),
            minio_endpoint: env::var("SR_MINIO_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            minio_access_key: env::var("SR_MINIO_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            minio_secret_key: env::var("SR_MINIO_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            minio_bucket: env::var("SR_MINIO_BUCKET")
                .unwrap_or_else(|_| "evidence".to_string()),
            nats_url: env::var("SR_NATS_URL")
                .unwrap_or_else(|_| "nats://localhost:4222".to_string()),
            log_level: env::var("SR_LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        }
    }

    /// Create local development configuration
    pub fn local_dev() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            oidc: OidcConfig::local_dev(),
            database_url: "postgres://postgres:postgres@localhost:5432/solver_ralph".to_string(),
            minio_endpoint: "http://localhost:9000".to_string(),
            minio_access_key: "minioadmin".to_string(),
            minio_secret_key: "minioadmin".to_string(),
            minio_bucket: "evidence".to_string(),
            nats_url: "nats://localhost:4222".to_string(),
            log_level: "debug".to_string(),
        }
    }

    /// Create test configuration
    pub fn test() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 0, // Random port
            oidc: OidcConfig::test_mode(),
            database_url: "postgres://postgres:postgres@localhost:5432/solver_ralph_test".to_string(),
            minio_endpoint: "http://localhost:9000".to_string(),
            minio_access_key: "minioadmin".to_string(),
            minio_secret_key: "minioadmin".to_string(),
            minio_bucket: "evidence-test".to_string(),
            nats_url: "nats://localhost:4222".to_string(),
            log_level: "debug".to_string(),
        }
    }

    /// Get the bind address
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
