//! Oracle API Handlers
//!
//! Per SR-SEMANTIC-ORACLE-SPEC: Endpoints for listing, retrieving, and registering
//! oracle suites and verification profiles. Oracle suites contain oracle definitions
//! that produce evidence bundles for stage gates.
//!
//! Endpoints:
//! - GET  /api/v1/oracles/suites           - List all registered oracle suites
//! - GET  /api/v1/oracles/suites/:suite_id - Get suite detail with oracles
//! - POST /api/v1/oracles/suites           - Register a new oracle suite
//! - GET  /api/v1/oracles/profiles         - List verification profiles
//! - GET  /api/v1/oracles/profiles/:id     - Get profile detail
//! - POST /api/v1/oracles/profiles         - Register a new verification profile

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sr_adapters::oracle_runner::{
    EnvironmentConstraints, ExpectedOutput, NetworkMode, OracleClassification, OracleDefinition,
    OracleSuiteDefinition,
};
use sr_adapters::oracle_suite::{IntegrityCondition, VerificationProfile, WaivableCondition};
use std::collections::BTreeMap;
use tracing::{info, instrument};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::OracleRegistryState;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Response for a single oracle suite (summary)
#[derive(Debug, Serialize)]
pub struct OracleSuiteSummary {
    pub suite_id: String,
    pub suite_hash: String,
    pub oracle_count: usize,
    pub oci_image: String,
    pub network_mode: String,
    pub runtime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_set_id: Option<String>,
}

/// Response for listing oracle suites
#[derive(Debug, Serialize)]
pub struct ListSuitesResponse {
    pub suites: Vec<OracleSuiteSummary>,
    pub total: usize,
}

/// Detailed oracle suite response
#[derive(Debug, Serialize)]
pub struct OracleSuiteDetailResponse {
    pub suite_id: String,
    pub suite_hash: String,
    pub oci_image: String,
    pub oci_image_digest: String,
    pub environment_constraints: EnvironmentConstraintsResponse,
    pub oracles: Vec<OracleDefinitionResponse>,
    pub metadata: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct EnvironmentConstraintsResponse {
    pub runtime: String,
    pub network: String,
    pub cpu_arch: String,
    pub os: String,
    pub workspace_readonly: bool,
    pub additional_constraints: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct OracleDefinitionResponse {
    pub oracle_id: String,
    pub oracle_name: String,
    pub command: String,
    pub timeout_seconds: u64,
    pub expected_outputs: Vec<ExpectedOutputResponse>,
    pub classification: String,
    pub working_dir: Option<String>,
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ExpectedOutputResponse {
    pub path: String,
    pub content_type: String,
    pub required: bool,
}

/// Request to register a new oracle suite
#[derive(Debug, Deserialize)]
pub struct RegisterSuiteRequest {
    pub suite_id: String,
    pub oci_image: String,
    pub oci_image_digest: String,
    pub environment_constraints: EnvironmentConstraintsRequest,
    pub oracles: Vec<OracleDefinitionRequest>,
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentConstraintsRequest {
    #[serde(default = "default_runtime")]
    pub runtime: String,
    #[serde(default)]
    pub network: NetworkModeRequest,
    #[serde(default = "default_arch")]
    pub cpu_arch: String,
    #[serde(default = "default_os")]
    pub os: String,
    #[serde(default = "default_readonly")]
    pub workspace_readonly: bool,
    #[serde(default)]
    pub additional_constraints: Vec<String>,
}

fn default_runtime() -> String {
    "runsc".to_string()
}
fn default_arch() -> String {
    "amd64".to_string()
}
fn default_os() -> String {
    "linux".to_string()
}
fn default_readonly() -> bool {
    true
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkModeRequest {
    #[default]
    Disabled,
    Private,
    Host,
}

#[derive(Debug, Deserialize)]
pub struct OracleDefinitionRequest {
    pub oracle_id: String,
    pub oracle_name: String,
    pub command: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    pub expected_outputs: Vec<ExpectedOutputRequest>,
    #[serde(default)]
    pub classification: OracleClassificationRequest,
    pub working_dir: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

fn default_timeout() -> u64 {
    300
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OracleClassificationRequest {
    #[default]
    Required,
    Advisory,
    Optional,
}

#[derive(Debug, Deserialize)]
pub struct ExpectedOutputRequest {
    pub path: String,
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(default = "default_required")]
    pub required: bool,
}

fn default_content_type() -> String {
    "application/json".to_string()
}
fn default_required() -> bool {
    true
}

/// Response for suite registration
#[derive(Debug, Serialize)]
pub struct RegisterSuiteResponse {
    pub suite_id: String,
    pub suite_hash: String,
    pub oracle_count: usize,
}

/// Response for a single verification profile (summary)
#[derive(Debug, Serialize)]
pub struct VerificationProfileSummary {
    pub profile_id: String,
    pub name: String,
    pub description: String,
    pub required_suite_count: usize,
    pub applicable_deliverable_count: usize,
}

/// Response for listing verification profiles
#[derive(Debug, Serialize)]
pub struct ListProfilesResponse {
    pub profiles: Vec<VerificationProfileSummary>,
    pub total: usize,
}

/// Detailed verification profile response
#[derive(Debug, Serialize)]
pub struct VerificationProfileDetailResponse {
    pub profile_id: String,
    pub name: String,
    pub description: String,
    pub required_suites: Vec<String>,
    pub optional_suites: Vec<String>,
    pub waivable_failures: Vec<String>,
    pub integrity_conditions: Vec<String>,
    pub applicable_deliverables: Vec<String>,
    pub metadata: BTreeMap<String, serde_json::Value>,
}

/// Request to register a new verification profile
#[derive(Debug, Deserialize)]
pub struct RegisterProfileRequest {
    pub profile_id: String,
    pub name: String,
    pub description: String,
    pub required_suites: Vec<String>,
    #[serde(default)]
    pub optional_suites: Vec<String>,
    #[serde(default)]
    pub waivable_failures: Vec<String>,
    #[serde(default)]
    pub integrity_conditions: Vec<String>,
    pub applicable_deliverables: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
}

/// Response for profile registration
#[derive(Debug, Serialize)]
pub struct RegisterProfileResponse {
    pub profile_id: String,
    pub name: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// List all registered oracle suites
///
/// GET /api/v1/oracles/suites
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id))]
pub async fn list_suites(
    State(state): State<OracleRegistryState>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<ListSuitesResponse>> {
    let suites = state.registry.list_suites().await;

    let summaries: Vec<OracleSuiteSummary> = suites
        .iter()
        .map(|s| OracleSuiteSummary {
            suite_id: s.suite_id.clone(),
            suite_hash: s.suite_hash.clone(),
            oracle_count: s.oracles.len(),
            oci_image: s.oci_image.clone(),
            network_mode: format!("{:?}", s.environment_constraints.network),
            runtime: s.environment_constraints.runtime.clone(),
            semantic_set_id: s
                .metadata
                .get("semantic_set_id")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
        })
        .collect();

    let total = summaries.len();

    Ok(Json(ListSuitesResponse {
        suites: summaries,
        total,
    }))
}

/// Get a single oracle suite by ID
///
/// GET /api/v1/oracles/suites/:suite_id
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id, suite_id = %suite_id))]
pub async fn get_suite(
    State(state): State<OracleRegistryState>,
    Path(suite_id): Path<String>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<OracleSuiteDetailResponse>> {
    let suite = state
        .registry
        .get_suite(&suite_id)
        .await
        .ok_or_else(|| ApiError::NotFound {
            resource: "OracleSuite".to_string(),
            id: suite_id.clone(),
        })?;

    Ok(Json(suite_to_detail_response(suite)))
}

/// Register a new oracle suite
///
/// POST /api/v1/oracles/suites
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn register_suite(
    State(state): State<OracleRegistryState>,
    user: AuthenticatedUser,
    Json(body): Json<RegisterSuiteRequest>,
) -> ApiResult<Json<RegisterSuiteResponse>> {
    // Validate suite ID format
    if !body.suite_id.starts_with("suite:") {
        return Err(ApiError::BadRequest {
            message: "Suite ID must start with 'suite:'".to_string(),
        });
    }

    // Check if suite already exists
    if state.registry.get_suite(&body.suite_id).await.is_some() {
        return Err(ApiError::Conflict {
            message: format!("Suite '{}' already exists", body.suite_id),
        });
    }

    // Convert request to domain type
    let oracles: Vec<OracleDefinition> = body
        .oracles
        .into_iter()
        .map(|o| OracleDefinition {
            oracle_id: o.oracle_id,
            oracle_name: o.oracle_name,
            command: o.command,
            args: vec![],
            timeout_seconds: o.timeout_seconds,
            expected_outputs: o
                .expected_outputs
                .into_iter()
                .map(|e| ExpectedOutput {
                    path: e.path,
                    content_type: e.content_type,
                    required: e.required,
                })
                .collect(),
            classification: match o.classification {
                OracleClassificationRequest::Required => OracleClassification::Required,
                OracleClassificationRequest::Advisory => OracleClassification::Advisory,
                OracleClassificationRequest::Optional => OracleClassification::Optional,
            },
            working_dir: o.working_dir,
            env: o.env,
        })
        .collect();

    // Compute suite hash
    let suite_hash = compute_suite_hash(&oracles);

    let suite = OracleSuiteDefinition {
        suite_id: body.suite_id.clone(),
        suite_hash: suite_hash.clone(),
        oci_image: body.oci_image,
        oci_image_digest: body.oci_image_digest,
        environment_constraints: EnvironmentConstraints {
            runtime: body.environment_constraints.runtime,
            network: match body.environment_constraints.network {
                NetworkModeRequest::Disabled => NetworkMode::Disabled,
                NetworkModeRequest::Private => NetworkMode::Private,
                NetworkModeRequest::Host => NetworkMode::Host,
            },
            cpu_arch: body.environment_constraints.cpu_arch,
            os: body.environment_constraints.os,
            workspace_readonly: body.environment_constraints.workspace_readonly,
            additional_constraints: body.environment_constraints.additional_constraints,
        },
        oracles: oracles.clone(),
        metadata: body.metadata,
    };

    let oracle_count = suite.oracles.len();

    info!(
        suite_id = %body.suite_id,
        suite_hash = %suite_hash,
        oracle_count = oracle_count,
        registered_by = %user.actor_id,
        "Registering new oracle suite"
    );

    state.registry.register_suite(suite).await;

    Ok(Json(RegisterSuiteResponse {
        suite_id: body.suite_id,
        suite_hash,
        oracle_count,
    }))
}

/// List all verification profiles
///
/// GET /api/v1/oracles/profiles
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id))]
pub async fn list_profiles(
    State(state): State<OracleRegistryState>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<ListProfilesResponse>> {
    // Get all profiles - need to iterate through known profile IDs
    let profile_ids = vec![
        sr_adapters::oracle_suite::PROFILE_GOV_CORE,
        sr_adapters::oracle_suite::PROFILE_STRICT_CORE,
        sr_adapters::oracle_suite::PROFILE_STRICT_FULL,
    ];

    let mut profiles = Vec::new();
    for profile_id in profile_ids {
        if let Some(profile) = state.registry.get_profile(profile_id).await {
            profiles.push(VerificationProfileSummary {
                profile_id: profile.profile_id.clone(),
                name: profile.name.clone(),
                description: profile.description.clone(),
                required_suite_count: profile.required_suites.len(),
                applicable_deliverable_count: profile.applicable_deliverables.len(),
            });
        }
    }

    let total = profiles.len();

    Ok(Json(ListProfilesResponse { profiles, total }))
}

/// Get a single verification profile by ID
///
/// GET /api/v1/oracles/profiles/:profile_id
#[instrument(skip(state, _user), fields(user_id = %_user.actor_id, profile_id = %profile_id))]
pub async fn get_profile(
    State(state): State<OracleRegistryState>,
    Path(profile_id): Path<String>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<VerificationProfileDetailResponse>> {
    let profile = state
        .registry
        .get_profile(&profile_id)
        .await
        .ok_or_else(|| ApiError::NotFound {
            resource: "VerificationProfile".to_string(),
            id: profile_id.clone(),
        })?;

    Ok(Json(VerificationProfileDetailResponse {
        profile_id: profile.profile_id,
        name: profile.name,
        description: profile.description,
        required_suites: profile.required_suites,
        optional_suites: profile.optional_suites,
        waivable_failures: profile
            .waivable_failures
            .iter()
            .map(|w| format!("{:?}", w))
            .collect(),
        integrity_conditions: profile
            .integrity_conditions
            .iter()
            .map(|i| format!("{:?}", i))
            .collect(),
        applicable_deliverables: profile.applicable_deliverables,
        metadata: profile.metadata,
    }))
}

/// Register a new verification profile
///
/// POST /api/v1/oracles/profiles
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn register_profile(
    State(state): State<OracleRegistryState>,
    user: AuthenticatedUser,
    Json(body): Json<RegisterProfileRequest>,
) -> ApiResult<Json<RegisterProfileResponse>> {
    // Validate profile ID format
    if !body.profile_id.starts_with("profile:") {
        return Err(ApiError::BadRequest {
            message: "Profile ID must start with 'profile:'".to_string(),
        });
    }

    // Check if profile already exists
    if state.registry.get_profile(&body.profile_id).await.is_some() {
        return Err(ApiError::Conflict {
            message: format!("Profile '{}' already exists", body.profile_id),
        });
    }

    // Parse waivable failures
    let waivable_failures: Vec<WaivableCondition> = body
        .waivable_failures
        .iter()
        .filter_map(|s| match s.to_uppercase().as_str() {
            "BUILD_FAIL" | "BUILDFAIL" => Some(WaivableCondition::BuildFail),
            "UNIT_FAIL" | "UNITFAIL" => Some(WaivableCondition::UnitFail),
            "LINT_FAIL" | "LINTFAIL" => Some(WaivableCondition::LintFail),
            "SCHEMA_FAIL" | "SCHEMAFAIL" => Some(WaivableCondition::SchemaFail),
            "INTEGRATION_FAIL" | "INTEGRATIONFAIL" => Some(WaivableCondition::IntegrationFail),
            "E2E_FAIL" | "E2EFAIL" => Some(WaivableCondition::E2eFail),
            _ => None,
        })
        .collect();

    // Parse integrity conditions
    let integrity_conditions: Vec<IntegrityCondition> = body
        .integrity_conditions
        .iter()
        .filter_map(|s| match s.to_uppercase().as_str() {
            "ORACLE_TAMPER" | "ORACLETAMPER" => Some(IntegrityCondition::OracleTamper),
            "ORACLE_GAP" | "ORACLEGAP" => Some(IntegrityCondition::OracleGap),
            "ORACLE_ENV_MISMATCH" | "ORACLEENVMISMATCH" => {
                Some(IntegrityCondition::OracleEnvMismatch)
            }
            "ORACLE_FLAKE" | "ORACLEFLAKE" => Some(IntegrityCondition::OracleFlake),
            "EVIDENCE_MISSING" | "EVIDENCEMISSING" => Some(IntegrityCondition::EvidenceMissing),
            "MANIFEST_INVALID" | "MANIFESTINVALID" => Some(IntegrityCondition::ManifestInvalid),
            _ => None,
        })
        .collect();

    let profile = VerificationProfile {
        profile_id: body.profile_id.clone(),
        name: body.name.clone(),
        description: body.description,
        required_suites: body.required_suites,
        optional_suites: body.optional_suites,
        waivable_failures,
        integrity_conditions,
        applicable_deliverables: body.applicable_deliverables,
        metadata: body.metadata,
    };

    info!(
        profile_id = %body.profile_id,
        name = %body.name,
        registered_by = %user.actor_id,
        "Registering new verification profile"
    );

    state.registry.register_profile(profile).await;

    Ok(Json(RegisterProfileResponse {
        profile_id: body.profile_id,
        name: body.name,
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

fn suite_to_detail_response(suite: OracleSuiteDefinition) -> OracleSuiteDetailResponse {
    OracleSuiteDetailResponse {
        suite_id: suite.suite_id,
        suite_hash: suite.suite_hash,
        oci_image: suite.oci_image,
        oci_image_digest: suite.oci_image_digest,
        environment_constraints: EnvironmentConstraintsResponse {
            runtime: suite.environment_constraints.runtime,
            network: format!("{:?}", suite.environment_constraints.network),
            cpu_arch: suite.environment_constraints.cpu_arch,
            os: suite.environment_constraints.os,
            workspace_readonly: suite.environment_constraints.workspace_readonly,
            additional_constraints: suite.environment_constraints.additional_constraints,
        },
        oracles: suite
            .oracles
            .into_iter()
            .map(|o| OracleDefinitionResponse {
                oracle_id: o.oracle_id,
                oracle_name: o.oracle_name,
                command: o.command,
                timeout_seconds: o.timeout_seconds,
                expected_outputs: o
                    .expected_outputs
                    .into_iter()
                    .map(|e| ExpectedOutputResponse {
                        path: e.path,
                        content_type: e.content_type,
                        required: e.required,
                    })
                    .collect(),
                classification: format!("{:?}", o.classification),
                working_dir: o.working_dir,
                env: o.env,
            })
            .collect(),
        metadata: suite.metadata,
    }
}

/// Compute deterministic hash for a suite based on its oracle definitions
fn compute_suite_hash(oracles: &[OracleDefinition]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();

    for oracle in oracles {
        hasher.update(oracle.oracle_id.as_bytes());
        hasher.update(b":");
        hasher.update(oracle.command.as_bytes());
        hasher.update(b":");
        hasher.update(&oracle.timeout_seconds.to_le_bytes());
        hasher.update(b":");
        for output in &oracle.expected_outputs {
            hasher.update(output.path.as_bytes());
            hasher.update(b",");
        }
        hasher.update(b"\n");
    }

    format!("sha256:{}", hex::encode(hasher.finalize()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suite_hash_determinism() {
        let oracles = vec![OracleDefinition {
            oracle_id: "oracle:test".to_string(),
            oracle_name: "Test Oracle".to_string(),
            command: "echo hello".to_string(),
            args: vec![],
            timeout_seconds: 60,
            expected_outputs: vec![ExpectedOutput {
                path: "output.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            }],
            classification: OracleClassification::Required,
            working_dir: None,
            env: BTreeMap::new(),
        }];

        let hash1 = compute_suite_hash(&oracles);
        let hash2 = compute_suite_hash(&oracles);

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }

    #[test]
    fn test_network_mode_request_default() {
        let mode: NetworkModeRequest = Default::default();
        assert!(matches!(mode, NetworkModeRequest::Disabled));
    }
}
