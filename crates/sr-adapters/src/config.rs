//! Adapter configuration

use serde::{Deserialize, Serialize};

/// Service configuration per SR-SPEC §5.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// PostgreSQL connection configuration
    pub postgres: PostgresConfig,

    /// MinIO configuration
    pub minio: MinioConfig,

    /// NATS configuration (optional in Phase 0)
    pub nats: Option<NatsConfig>,

    /// Zitadel OIDC configuration
    pub zitadel: ZitadelConfig,

    /// Infisical secrets configuration
    pub infisical: Option<InfisicalConfig>,
}

/// PostgreSQL configuration per SR-SPEC §5.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    /// Connection URL
    pub url: String,

    /// Maximum connections in pool
    pub max_connections: u32,
}

/// MinIO configuration per SR-SPEC §1.9.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinioConfig {
    /// MinIO endpoint URL
    pub endpoint: String,

    /// Access key
    pub access_key: String,

    /// Secret key (should come from Infisical in production)
    pub secret_key: String,

    /// Bucket for general evidence
    pub evidence_bucket: String,

    /// Bucket for restricted evidence
    pub restricted_bucket: String,
}

/// NATS configuration per SR-SPEC §4.6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsConfig {
    /// NATS server URL
    pub url: String,

    /// Enable JetStream
    pub jetstream_enabled: bool,
}

/// Zitadel OIDC configuration per SR-SPEC §2.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZitadelConfig {
    /// OIDC issuer URL
    pub issuer: String,

    /// Client ID for this service
    pub client_id: String,

    /// Audience for token validation
    pub audience: String,
}

/// Infisical secrets configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfisicalConfig {
    /// Infisical API endpoint
    pub endpoint: String,

    /// Environment (e.g., "development", "production")
    pub environment: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            postgres: PostgresConfig {
                url: "postgresql://sr_app:password@localhost:5432/solver_ralph".to_string(),
                max_connections: 10,
            },
            minio: MinioConfig {
                endpoint: "http://localhost:9000".to_string(),
                access_key: "minioadmin".to_string(),
                secret_key: "minioadmin".to_string(),
                evidence_bucket: "evidence-public".to_string(),
                restricted_bucket: "evidence-restricted".to_string(),
            },
            nats: None,
            zitadel: ZitadelConfig {
                issuer: "http://localhost:8080".to_string(),
                client_id: "solver-ralph-api".to_string(),
                audience: "solver-ralph".to_string(),
            },
            infisical: None,
        }
    }
}
