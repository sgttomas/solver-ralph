//! Core Oracle Suite Implementation (D-25)
//!
//! Implements the core oracle suite definitions per SR-DIRECTIVE.KIT-ORACLES_PROFILES_MATRIX.
//!
//! Per SR-PLAN D-25:
//! - Suite definition registers correctly
//! - All core oracles execute successfully
//! - Evidence manifest is produced with all required fields
//! - Verdict computation is correct
//! - Output is deterministic: same inputs → same manifest hash
//!
//! Oracle Suites:
//! - SR-SUITE-GOV: Governance-only checks (meta_validate, refs_validate)
//! - SR-SUITE-CORE: Default for most deliverables (build, test, lint, schema, integrity)
//! - SR-SUITE-FULL: Full stack testing (integration, e2e, replay, sbom)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, instrument};

use crate::oracle_runner::{
    EnvironmentConstraints, ExpectedOutput, NetworkMode, OracleClassification, OracleDefinition,
    OracleSuiteDefinition,
};
use crate::semantic_suite::{
    create_intake_admissibility_suite, to_oracle_suite_definition, SUITE_INTAKE_ADMISSIBILITY_ID,
};
use sr_ports::{
    OracleSuiteRecord, OracleSuiteRegistryError, OracleSuiteRegistryPort, OracleSuiteStatus,
    RegisterSuiteInput, SuiteFilter,
};

// ============================================================================
// Suite Registry
// ============================================================================

/// Oracle suite registry for managing suite definitions
pub struct OracleSuiteRegistry {
    /// Registered suite definitions
    suites: Arc<RwLock<BTreeMap<String, OracleSuiteDefinition>>>,
    /// Verification profiles
    profiles: Arc<RwLock<BTreeMap<String, VerificationProfile>>>,
}

impl OracleSuiteRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            suites: Arc::new(RwLock::new(BTreeMap::new())),
            profiles: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    /// Create registry with core suites pre-registered
    pub fn with_core_suites() -> Self {
        // We'll register suites synchronously using blocking
        let mut suites = BTreeMap::new();
        suites.insert(SUITE_GOV_ID.to_string(), create_gov_suite());
        suites.insert(SUITE_CORE_ID.to_string(), create_core_suite());
        suites.insert(SUITE_FULL_ID.to_string(), create_full_suite());

        // Register integration suite (V11-5, D-26)
        suites.insert(SUITE_INTEGRATION_ID.to_string(), create_integration_suite());

        // Register semantic oracle suite (D-39)
        let semantic_suite = create_intake_admissibility_suite();
        suites.insert(
            SUITE_INTAKE_ADMISSIBILITY_ID.to_string(),
            to_oracle_suite_definition(&semantic_suite),
        );

        let mut profiles = BTreeMap::new();
        profiles.insert(PROFILE_GOV_CORE.to_string(), create_gov_core_profile());
        profiles.insert(
            PROFILE_STRICT_CORE.to_string(),
            create_strict_core_profile(),
        );
        profiles.insert(
            PROFILE_STRICT_FULL.to_string(),
            create_strict_full_profile(),
        );

        Self {
            suites: Arc::new(RwLock::new(suites)),
            profiles: Arc::new(RwLock::new(profiles)),
        }
    }

    /// Register a suite definition
    pub async fn register_suite(&self, suite: OracleSuiteDefinition) {
        let mut suites = self.suites.write().await;
        info!(suite_id = %suite.suite_id, "Registering oracle suite");
        suites.insert(suite.suite_id.clone(), suite);
    }

    /// Get a suite by ID
    pub async fn get_suite(&self, suite_id: &str) -> Option<OracleSuiteDefinition> {
        let suites = self.suites.read().await;
        suites.get(suite_id).cloned()
    }

    /// Get all registered suites
    pub async fn list_suites(&self) -> Vec<OracleSuiteDefinition> {
        let suites = self.suites.read().await;
        suites.values().cloned().collect()
    }

    /// Register a verification profile
    pub async fn register_profile(&self, profile: VerificationProfile) {
        let mut profiles = self.profiles.write().await;
        info!(profile_id = %profile.profile_id, "Registering verification profile");
        profiles.insert(profile.profile_id.clone(), profile);
    }

    /// Get a profile by ID
    pub async fn get_profile(&self, profile_id: &str) -> Option<VerificationProfile> {
        let profiles = self.profiles.read().await;
        profiles.get(profile_id).cloned()
    }

    /// Get profile for a deliverable
    pub async fn get_profile_for_deliverable(
        &self,
        deliverable_id: &str,
    ) -> Option<VerificationProfile> {
        let profiles = self.profiles.read().await;

        // Check each profile's applicable deliverables
        for profile in profiles.values() {
            if profile
                .applicable_deliverables
                .contains(&deliverable_id.to_string())
            {
                return Some(profile.clone());
            }
        }

        // Default to STRICT-CORE for most deliverables
        profiles.get(PROFILE_STRICT_CORE).cloned()
    }
}

impl Default for OracleSuiteRegistry {
    fn default() -> Self {
        Self::with_core_suites()
    }
}

// ============================================================================
// OracleSuiteRegistryPort Implementation (V8-1)
// ============================================================================

impl OracleSuiteRegistryPort for OracleSuiteRegistry {
    /// Register a new oracle suite
    #[instrument(skip(self, input), fields(suite_id = %input.suite_id))]
    async fn register(
        &self,
        input: RegisterSuiteInput,
    ) -> Result<OracleSuiteRecord, OracleSuiteRegistryError> {
        let mut suites = self.suites.write().await;

        // Check if suite already exists
        if suites.contains_key(&input.suite_id) {
            return Err(OracleSuiteRegistryError::AlreadyExists {
                suite_id: input.suite_id,
            });
        }

        // Check for hash conflict
        if suites.values().any(|s| s.suite_hash == input.suite_hash) {
            return Err(OracleSuiteRegistryError::HashConflict {
                suite_hash: input.suite_hash,
            });
        }

        // Deserialize environment constraints
        let env_constraints: EnvironmentConstraints =
            serde_json::from_value(input.environment_constraints.clone()).map_err(|e| {
                OracleSuiteRegistryError::SerializationError {
                    message: format!("Failed to deserialize environment_constraints: {e}"),
                }
            })?;

        // Deserialize oracles
        let oracle_defs: Vec<OracleDefinition> = serde_json::from_value(input.oracles.clone())
            .map_err(|e| OracleSuiteRegistryError::SerializationError {
                message: format!("Failed to deserialize oracles: {e}"),
            })?;

        // Deserialize metadata
        let meta: BTreeMap<String, serde_json::Value> =
            serde_json::from_value(input.metadata.clone()).unwrap_or_default();

        // Create the suite definition
        let definition = OracleSuiteDefinition {
            suite_id: input.suite_id.clone(),
            suite_hash: input.suite_hash.clone(),
            oci_image: input.oci_image.clone(),
            oci_image_digest: input.oci_image_digest.clone(),
            environment_constraints: env_constraints,
            oracles: oracle_defs,
            metadata: meta,
        };

        suites.insert(input.suite_id.clone(), definition);

        info!(suite_id = %input.suite_id, suite_hash = %input.suite_hash, "Oracle suite registered (in-memory)");

        // Create the record to return
        let record = OracleSuiteRecord {
            suite_id: input.suite_id,
            suite_hash: input.suite_hash,
            oci_image: input.oci_image,
            oci_image_digest: input.oci_image_digest,
            environment_constraints: input.environment_constraints,
            oracles: input.oracles,
            metadata: input.metadata,
            registered_at: Utc::now(),
            registered_by_kind: input.actor_kind,
            registered_by_id: input.actor_id,
            status: OracleSuiteStatus::Active,
        };

        Ok(record)
    }

    /// Get an oracle suite by ID
    async fn get(
        &self,
        suite_id: &str,
    ) -> Result<Option<OracleSuiteRecord>, OracleSuiteRegistryError> {
        let suites = self.suites.read().await;

        match suites.get(suite_id) {
            Some(def) => Ok(Some(definition_to_record(def))),
            None => Ok(None),
        }
    }

    /// Get an oracle suite by its content hash
    async fn get_by_hash(
        &self,
        suite_hash: &str,
    ) -> Result<Option<OracleSuiteRecord>, OracleSuiteRegistryError> {
        let suites = self.suites.read().await;

        for def in suites.values() {
            if def.suite_hash == suite_hash {
                return Ok(Some(definition_to_record(def)));
            }
        }

        Ok(None)
    }

    /// List oracle suites with optional filtering
    async fn list(
        &self,
        filter: SuiteFilter,
    ) -> Result<Vec<OracleSuiteRecord>, OracleSuiteRegistryError> {
        let suites = self.suites.read().await;

        let mut records: Vec<OracleSuiteRecord> = suites
            .values()
            .map(definition_to_record)
            .filter(|_| {
                // In-memory registry only has active suites
                filter.status.is_none() || filter.status == Some(OracleSuiteStatus::Active)
            })
            .collect();

        // Apply limit if specified
        if let Some(limit) = filter.limit {
            records.truncate(limit);
        }

        Ok(records)
    }

    /// Deprecate an oracle suite (in-memory just removes it)
    async fn deprecate(
        &self,
        suite_id: &str,
        _actor_kind: &str,
        _actor_id: &str,
    ) -> Result<(), OracleSuiteRegistryError> {
        let mut suites = self.suites.write().await;

        if suites.remove(suite_id).is_none() {
            return Err(OracleSuiteRegistryError::NotFound {
                suite_id: suite_id.to_string(),
            });
        }

        info!(suite_id = %suite_id, "Oracle suite deprecated (in-memory)");
        Ok(())
    }
}

/// Convert an OracleSuiteDefinition to an OracleSuiteRecord
///
/// Per SR-PLAN-V8 Amendment A-2:
/// - OracleSuiteDefinition = execution config
/// - OracleSuiteRecord = stored entity with lifecycle metadata
fn definition_to_record(def: &OracleSuiteDefinition) -> OracleSuiteRecord {
    OracleSuiteRecord {
        suite_id: def.suite_id.clone(),
        suite_hash: def.suite_hash.clone(),
        oci_image: def.oci_image.clone(),
        oci_image_digest: def.oci_image_digest.clone(),
        environment_constraints: serde_json::to_value(&def.environment_constraints)
            .unwrap_or_default(),
        oracles: serde_json::to_value(&def.oracles).unwrap_or_default(),
        metadata: serde_json::to_value(&def.metadata).unwrap_or_default(),
        // In-memory registry doesn't track registration metadata
        registered_at: Utc::now(),
        registered_by_kind: "SYSTEM".to_string(),
        registered_by_id: "in-memory-registry".to_string(),
        status: OracleSuiteStatus::Active,
    }
}

/// Convert an OracleSuiteRecord back to an OracleSuiteDefinition
///
/// This is useful when the execution layer needs the definition format.
pub fn record_to_definition(
    record: &OracleSuiteRecord,
) -> Result<OracleSuiteDefinition, OracleSuiteRegistryError> {
    let environment_constraints: EnvironmentConstraints =
        serde_json::from_value(record.environment_constraints.clone()).map_err(|e| {
            OracleSuiteRegistryError::SerializationError {
                message: format!("Failed to deserialize environment_constraints: {e}"),
            }
        })?;

    let oracles: Vec<OracleDefinition> =
        serde_json::from_value(record.oracles.clone()).map_err(|e| {
            OracleSuiteRegistryError::SerializationError {
                message: format!("Failed to deserialize oracles: {e}"),
            }
        })?;

    let metadata: BTreeMap<String, serde_json::Value> =
        serde_json::from_value(record.metadata.clone()).unwrap_or_default();

    Ok(OracleSuiteDefinition {
        suite_id: record.suite_id.clone(),
        suite_hash: record.suite_hash.clone(),
        oci_image: record.oci_image.clone(),
        oci_image_digest: record.oci_image_digest.clone(),
        environment_constraints,
        oracles,
        metadata,
    })
}

// ============================================================================
// Suite Identifiers
// ============================================================================

/// Governance-only suite ID
pub const SUITE_GOV_ID: &str = "suite:SR-SUITE-GOV";
/// Core verification suite ID
pub const SUITE_CORE_ID: &str = "suite:SR-SUITE-CORE";
/// Full verification suite ID
pub const SUITE_FULL_ID: &str = "suite:SR-SUITE-FULL";
/// Integration testing suite ID (V11-5, D-26)
pub const SUITE_INTEGRATION_ID: &str = "suite:SR-SUITE-INTEGRATION";

/// Governance-core profile ID
pub const PROFILE_GOV_CORE: &str = "profile:GOV-CORE";
/// Strict-core profile ID
pub const PROFILE_STRICT_CORE: &str = "profile:STRICT-CORE";
/// Strict-full profile ID
pub const PROFILE_STRICT_FULL: &str = "profile:STRICT-FULL";

// ============================================================================
// Oracle Identifiers
// ============================================================================

/// Oracle IDs for the core suite
pub mod oracle_ids {
    /// Validate governance document metadata
    pub const META_VALIDATE: &str = "oracle:meta_validate";
    /// Validate typed references
    pub const REFS_VALIDATE: &str = "oracle:refs_validate";
    /// Build all services and libraries
    pub const BUILD: &str = "oracle:build";
    /// Run unit tests
    pub const UNIT_TESTS: &str = "oracle:unit_tests";
    /// Lint and format checks
    pub const LINT: &str = "oracle:lint";
    /// Schema and migration validation
    pub const SCHEMA_VALIDATE: &str = "oracle:schema_validate";
    /// Oracle integrity smoke tests
    pub const INTEGRITY_SMOKE: &str = "oracle:integrity_smoke";
    /// Integration tests
    pub const INTEGRATION: &str = "oracle:integration";
    /// End-to-end tests
    pub const E2E: &str = "oracle:e2e";
    /// Replay verification
    pub const REPLAY_VERIFY: &str = "oracle:replay_verify";
    /// Software bill of materials
    pub const SBOM: &str = "oracle:sbom";
}

// ============================================================================
// Verification Profile
// ============================================================================

/// Verification profile defining which suites and oracles apply to a deliverable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProfile {
    /// Profile identifier
    pub profile_id: String,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: String,
    /// Required suite IDs
    pub required_suites: Vec<String>,
    /// Optional suite IDs (run but don't block)
    pub optional_suites: Vec<String>,
    /// Waivable failure conditions
    pub waivable_failures: Vec<WaivableCondition>,
    /// Non-waivable integrity conditions (always block)
    pub integrity_conditions: Vec<IntegrityCondition>,
    /// Deliverables this profile applies to
    pub applicable_deliverables: Vec<String>,
    /// Profile metadata
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
}

/// Waivable failure condition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaivableCondition {
    /// Build failure
    BuildFail,
    /// Unit test failure
    UnitFail,
    /// Lint failure
    LintFail,
    /// Schema validation failure
    SchemaFail,
    /// Integration test failure
    IntegrationFail,
    /// E2E test failure
    E2eFail,
}

/// Integrity condition (non-waivable)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrityCondition {
    /// Oracle execution was tampered with
    OracleTamper,
    /// Gap in oracle coverage
    OracleGap,
    /// Environment mismatch
    OracleEnvMismatch,
    /// Oracle flake detected
    OracleFlake,
    /// Evidence missing or incomplete
    EvidenceMissing,
    /// Manifest validation failed
    ManifestInvalid,
}

// ============================================================================
// Suite Factories
// ============================================================================

/// Create the SR-SUITE-GOV (governance-only) suite
pub fn create_gov_suite() -> OracleSuiteDefinition {
    let oracles = vec![create_meta_validate_oracle(), create_refs_validate_oracle()];

    let suite_hash = compute_suite_hash(&oracles);

    OracleSuiteDefinition {
        suite_id: SUITE_GOV_ID.to_string(),
        suite_hash,
        oci_image: "ghcr.io/solver-ralph/oracle-gov:latest".to_string(),
        oci_image_digest: "sha256:PLACEHOLDER_DIGEST_GOV".to_string(),
        environment_constraints: EnvironmentConstraints {
            runtime: "runsc".to_string(),
            network: NetworkMode::Disabled,
            cpu_arch: "amd64".to_string(),
            os: "linux".to_string(),
            workspace_readonly: true,
            additional_constraints: vec![],
        },
        oracles,
        metadata: BTreeMap::new(),
    }
}

/// Create the SR-SUITE-CORE suite
pub fn create_core_suite() -> OracleSuiteDefinition {
    let oracles = vec![
        create_meta_validate_oracle(),
        create_build_oracle(),
        create_unit_tests_oracle(),
        create_lint_oracle(),
        create_schema_validate_oracle(),
        create_integrity_smoke_oracle(),
    ];

    let suite_hash = compute_suite_hash(&oracles);

    OracleSuiteDefinition {
        suite_id: SUITE_CORE_ID.to_string(),
        suite_hash,
        oci_image: "ghcr.io/solver-ralph/oracle-core:latest".to_string(),
        oci_image_digest: "sha256:PLACEHOLDER_DIGEST_CORE".to_string(),
        environment_constraints: EnvironmentConstraints {
            runtime: "runsc".to_string(),
            network: NetworkMode::Disabled,
            cpu_arch: "amd64".to_string(),
            os: "linux".to_string(),
            workspace_readonly: true,
            additional_constraints: vec![
                "rust_toolchain=stable".to_string(),
                "cargo_version>=1.75".to_string(),
            ],
        },
        oracles,
        metadata: BTreeMap::new(),
    }
}

/// Create the SR-SUITE-FULL suite
pub fn create_full_suite() -> OracleSuiteDefinition {
    let oracles = vec![
        create_meta_validate_oracle(),
        create_build_oracle(),
        create_unit_tests_oracle(),
        create_lint_oracle(),
        create_schema_validate_oracle(),
        create_integrity_smoke_oracle(),
        create_integration_oracle(),
        create_e2e_oracle(),
        create_replay_verify_oracle(),
        create_sbom_oracle(),
    ];

    let suite_hash = compute_suite_hash(&oracles);

    OracleSuiteDefinition {
        suite_id: SUITE_FULL_ID.to_string(),
        suite_hash,
        oci_image: "ghcr.io/solver-ralph/oracle-full:latest".to_string(),
        oci_image_digest: "sha256:PLACEHOLDER_DIGEST_FULL".to_string(),
        environment_constraints: EnvironmentConstraints {
            runtime: "runsc".to_string(),
            network: NetworkMode::Private, // Integration tests need network
            cpu_arch: "amd64".to_string(),
            os: "linux".to_string(),
            workspace_readonly: true,
            additional_constraints: vec![
                "rust_toolchain=stable".to_string(),
                "cargo_version>=1.75".to_string(),
                "docker_available=true".to_string(),
            ],
        },
        oracles,
        metadata: BTreeMap::new(),
    }
}

/// Create the SR-SUITE-INTEGRATION suite (V11-5, D-26)
///
/// This suite wraps the IntegrationRunner from sr-oracles to test infrastructure
/// connectivity: PostgreSQL, MinIO, NATS, and API health checks.
pub fn create_integration_suite() -> OracleSuiteDefinition {
    let oracles = vec![create_integration_test_oracle()];

    let suite_hash = compute_suite_hash(&oracles);

    OracleSuiteDefinition {
        suite_id: SUITE_INTEGRATION_ID.to_string(),
        suite_hash,
        oci_image: "ghcr.io/solver-ralph/oracle-integration:latest".to_string(),
        oci_image_digest: "sha256:PLACEHOLDER_DIGEST_INTEGRATION".to_string(),
        environment_constraints: EnvironmentConstraints {
            runtime: "runsc".to_string(),
            network: NetworkMode::Private, // Integration tests need network
            cpu_arch: "amd64".to_string(),
            os: "linux".to_string(),
            workspace_readonly: true,
            additional_constraints: vec![
                "postgres_available=true".to_string(),
                "minio_available=true".to_string(),
                "nats_available=true".to_string(),
            ],
        },
        oracles,
        metadata: BTreeMap::new(),
    }
}

/// Create integration test oracle for SR-SUITE-INTEGRATION
///
/// This oracle runs the IntegrationRunner which tests:
/// - PostgreSQL: connection, schema, event append/read, projections
/// - MinIO: connection, bucket exists, upload/download
/// - NATS: connection, publish, JetStream
/// - API: health, info, loops endpoint
fn create_integration_test_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: "oracle:integration_test".to_string(),
        oracle_name: "Infrastructure Integration Tests".to_string(),
        command: "sr-oracles integration --workspace /workspace --output /scratch/reports/integration.json 2>&1 | tee /scratch/logs/integration.log".to_string(),
        args: vec![],
        timeout_seconds: 900, // 15 minutes
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/integration.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
            ExpectedOutput {
                path: "logs/integration.log".to_string(),
                content_type: "text/plain".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::from_iter([
            ("RUST_BACKTRACE".to_string(), "1".to_string()),
        ]),
    }
}

// ============================================================================
// Oracle Factories
// ============================================================================

/// Create meta_validate oracle
fn create_meta_validate_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::META_VALIDATE.to_string(),
        oracle_name: "Governance Metadata Validation".to_string(),
        command: "sr-oracles meta-validate --workspace /workspace --output /scratch/reports/meta_validate.json".to_string(),
        args: vec![],
        timeout_seconds: 60,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/meta_validate.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create refs_validate oracle
fn create_refs_validate_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::REFS_VALIDATE.to_string(),
        oracle_name: "Typed Reference Validation".to_string(),
        command: "sr-oracles refs-validate --workspace /workspace --output /scratch/reports/refs_validate.json".to_string(),
        args: vec![],
        timeout_seconds: 60,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/refs_validate.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create build oracle
fn create_build_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::BUILD.to_string(),
        oracle_name: "Build Verification".to_string(),
        command: "cargo build --release --all-targets 2>&1 | tee /scratch/logs/build.log && sr-oracles report-build --log /scratch/logs/build.log --output /scratch/reports/build.json".to_string(),
        args: vec![],
        timeout_seconds: 600,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/build.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
            ExpectedOutput {
                path: "logs/build.log".to_string(),
                content_type: "text/plain".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::from_iter([
            ("CARGO_TERM_COLOR".to_string(), "never".to_string()),
            ("RUST_BACKTRACE".to_string(), "1".to_string()),
        ]),
    }
}

/// Create unit_tests oracle
fn create_unit_tests_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::UNIT_TESTS.to_string(),
        oracle_name: "Unit Test Verification".to_string(),
        command: "cargo test --lib --all-targets -- --test-threads=1 2>&1 | tee /scratch/logs/unit.log && sr-oracles report-tests --log /scratch/logs/unit.log --output /scratch/reports/unit.json".to_string(),
        args: vec![],
        timeout_seconds: 600,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/unit.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
            ExpectedOutput {
                path: "logs/unit.log".to_string(),
                content_type: "text/plain".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::from_iter([
            ("CARGO_TERM_COLOR".to_string(), "never".to_string()),
            ("RUST_BACKTRACE".to_string(), "1".to_string()),
        ]),
    }
}

/// Create lint oracle
fn create_lint_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::LINT.to_string(),
        oracle_name: "Lint and Format Verification".to_string(),
        command: "(cargo fmt --check && cargo clippy --all-targets -- -D warnings) 2>&1 | tee /scratch/logs/lint.log; sr-oracles report-lint --log /scratch/logs/lint.log --output /scratch/reports/lint.json".to_string(),
        args: vec![],
        timeout_seconds: 300,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/lint.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
            ExpectedOutput {
                path: "logs/lint.log".to_string(),
                content_type: "text/plain".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::from_iter([
            ("CARGO_TERM_COLOR".to_string(), "never".to_string()),
        ]),
    }
}

/// Create schema_validate oracle
fn create_schema_validate_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::SCHEMA_VALIDATE.to_string(),
        oracle_name: "Schema Validation".to_string(),
        command: "sr-oracles schema-validate --workspace /workspace --output /scratch/reports/schema.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/schema.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create integrity_smoke oracle
fn create_integrity_smoke_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::INTEGRITY_SMOKE.to_string(),
        oracle_name: "Oracle Integrity Smoke Test".to_string(),
        command: "sr-oracles integrity-smoke --workspace /workspace --output /scratch/reports/integrity_smoke.json".to_string(),
        args: vec![],
        timeout_seconds: 120,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/integrity_smoke.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create integration oracle
fn create_integration_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::INTEGRATION.to_string(),
        oracle_name: "Integration Tests".to_string(),
        command: "sr-oracles integration --workspace /workspace --output /scratch/reports/integration.json 2>&1 | tee /scratch/logs/integration.log".to_string(),
        args: vec![],
        timeout_seconds: 900,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/integration.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
            ExpectedOutput {
                path: "logs/integration.log".to_string(),
                content_type: "text/plain".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create e2e oracle
fn create_e2e_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::E2E.to_string(),
        oracle_name: "End-to-End Tests".to_string(),
        command: "sr-oracles e2e --workspace /workspace --output /scratch/reports/e2e.json 2>&1 | tee /scratch/logs/e2e.log".to_string(),
        args: vec![],
        timeout_seconds: 1200,
        expected_outputs: vec![
            ExpectedOutput {
                path: "reports/e2e.json".to_string(),
                content_type: "application/json".to_string(),
                required: true,
            },
            ExpectedOutput {
                path: "logs/e2e.log".to_string(),
                content_type: "text/plain".to_string(),
                required: true,
            },
        ],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create replay_verify oracle
fn create_replay_verify_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::REPLAY_VERIFY.to_string(),
        oracle_name: "Event Replay Verification".to_string(),
        command:
            "sr-oracles replay-verify --workspace /workspace --output /scratch/reports/replay.json"
                .to_string(),
        args: vec![],
        timeout_seconds: 600,
        expected_outputs: vec![ExpectedOutput {
            path: "reports/replay.json".to_string(),
            content_type: "application/json".to_string(),
            required: true,
        }],
        classification: OracleClassification::Required,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

/// Create sbom oracle
fn create_sbom_oracle() -> OracleDefinition {
    OracleDefinition {
        oracle_id: oracle_ids::SBOM.to_string(),
        oracle_name: "Software Bill of Materials".to_string(),
        command: "cargo sbom --output-format json > /scratch/reports/sbom.json".to_string(),
        args: vec![],
        timeout_seconds: 300,
        expected_outputs: vec![ExpectedOutput {
            path: "reports/sbom.json".to_string(),
            content_type: "application/json".to_string(),
            required: true,
        }],
        classification: OracleClassification::Advisory,
        working_dir: Some("/workspace".to_string()),
        env: BTreeMap::new(),
    }
}

// ============================================================================
// Profile Factories
// ============================================================================

/// Create GOV-CORE profile
fn create_gov_core_profile() -> VerificationProfile {
    VerificationProfile {
        profile_id: PROFILE_GOV_CORE.to_string(),
        name: "Governance Core".to_string(),
        description: "Governance-only verification for document changes".to_string(),
        required_suites: vec![SUITE_GOV_ID.to_string()],
        optional_suites: vec![],
        waivable_failures: vec![],
        integrity_conditions: vec![
            IntegrityCondition::OracleTamper,
            IntegrityCondition::OracleGap,
            IntegrityCondition::EvidenceMissing,
        ],
        applicable_deliverables: vec!["D-01".to_string()],
        metadata: BTreeMap::new(),
    }
}

/// Create STRICT-CORE profile
fn create_strict_core_profile() -> VerificationProfile {
    VerificationProfile {
        profile_id: PROFILE_STRICT_CORE.to_string(),
        name: "Strict Core".to_string(),
        description: "Standard verification for code deliverables".to_string(),
        required_suites: vec![SUITE_CORE_ID.to_string()],
        optional_suites: vec![],
        waivable_failures: vec![
            WaivableCondition::BuildFail,
            WaivableCondition::UnitFail,
            WaivableCondition::LintFail,
            WaivableCondition::SchemaFail,
        ],
        integrity_conditions: vec![
            IntegrityCondition::OracleTamper,
            IntegrityCondition::OracleGap,
            IntegrityCondition::OracleEnvMismatch,
            IntegrityCondition::OracleFlake,
            IntegrityCondition::EvidenceMissing,
            IntegrityCondition::ManifestInvalid,
        ],
        applicable_deliverables: (2..=33).map(|i| format!("D-{:02}", i)).collect(),
        metadata: BTreeMap::new(),
    }
}

/// Create STRICT-FULL profile
fn create_strict_full_profile() -> VerificationProfile {
    VerificationProfile {
        profile_id: PROFILE_STRICT_FULL.to_string(),
        name: "Strict Full".to_string(),
        description: "Full verification for integration and e2e deliverables".to_string(),
        required_suites: vec![SUITE_FULL_ID.to_string()],
        optional_suites: vec![],
        waivable_failures: vec![
            WaivableCondition::BuildFail,
            WaivableCondition::UnitFail,
            WaivableCondition::LintFail,
            WaivableCondition::SchemaFail,
            WaivableCondition::IntegrationFail,
            WaivableCondition::E2eFail,
        ],
        integrity_conditions: vec![
            IntegrityCondition::OracleTamper,
            IntegrityCondition::OracleGap,
            IntegrityCondition::OracleEnvMismatch,
            IntegrityCondition::OracleFlake,
            IntegrityCondition::EvidenceMissing,
            IntegrityCondition::ManifestInvalid,
        ],
        applicable_deliverables: vec!["D-34".to_string(), "D-35".to_string(), "D-36".to_string()],
        metadata: BTreeMap::new(),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Compute deterministic hash for a suite based on its oracle definitions
fn compute_suite_hash(oracles: &[OracleDefinition]) -> String {
    let mut hasher = Sha256::new();

    // Hash each oracle definition in order
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

/// Validate a suite definition
pub fn validate_suite(suite: &OracleSuiteDefinition) -> Result<(), SuiteValidationError> {
    // Check suite ID format
    if !suite.suite_id.starts_with("suite:") {
        return Err(SuiteValidationError::InvalidSuiteId {
            id: suite.suite_id.clone(),
            reason: "Suite ID must start with 'suite:'".to_string(),
        });
    }

    // Check at least one oracle
    if suite.oracles.is_empty() {
        return Err(SuiteValidationError::NoOracles {
            suite_id: suite.suite_id.clone(),
        });
    }

    // Check each oracle
    for oracle in &suite.oracles {
        if !oracle.oracle_id.starts_with("oracle:") {
            return Err(SuiteValidationError::InvalidOracleId {
                oracle_id: oracle.oracle_id.clone(),
                reason: "Oracle ID must start with 'oracle:'".to_string(),
            });
        }

        if oracle.command.is_empty() {
            return Err(SuiteValidationError::EmptyCommand {
                oracle_id: oracle.oracle_id.clone(),
            });
        }

        if oracle.timeout_seconds == 0 {
            return Err(SuiteValidationError::ZeroTimeout {
                oracle_id: oracle.oracle_id.clone(),
            });
        }
    }

    // Verify suite hash
    let computed_hash = compute_suite_hash(&suite.oracles);
    if suite.suite_hash != computed_hash {
        return Err(SuiteValidationError::HashMismatch {
            suite_id: suite.suite_id.clone(),
            expected: suite.suite_hash.clone(),
            actual: computed_hash,
        });
    }

    Ok(())
}

/// Suite validation errors
#[derive(Debug, thiserror::Error)]
pub enum SuiteValidationError {
    #[error("Invalid suite ID '{id}': {reason}")]
    InvalidSuiteId { id: String, reason: String },

    #[error("Suite '{suite_id}' has no oracles")]
    NoOracles { suite_id: String },

    #[error("Invalid oracle ID '{oracle_id}': {reason}")]
    InvalidOracleId { oracle_id: String, reason: String },

    #[error("Oracle '{oracle_id}' has empty command")]
    EmptyCommand { oracle_id: String },

    #[error("Oracle '{oracle_id}' has zero timeout")]
    ZeroTimeout { oracle_id: String },

    #[error("Suite '{suite_id}' hash mismatch: expected {expected}, got {actual}")]
    HashMismatch {
        suite_id: String,
        expected: String,
        actual: String,
    },
}

// ============================================================================
// Oracle Report Structures
// ============================================================================

/// Report from meta_validate oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaValidateReport {
    pub oracle_id: String,
    pub status: String,
    pub documents_checked: usize,
    pub documents_valid: usize,
    pub documents_invalid: usize,
    pub errors: Vec<MetaValidationError>,
    pub timestamp: DateTime<Utc>,
}

/// Meta validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaValidationError {
    pub document_path: String,
    pub field: String,
    pub message: String,
    pub severity: String,
}

/// Report from build oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildReport {
    pub oracle_id: String,
    pub status: String,
    pub targets_built: usize,
    pub targets_failed: usize,
    pub warnings: usize,
    pub errors: Vec<BuildError>,
    pub artifacts: Vec<BuildArtifact>,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Build error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildError {
    pub file: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub code: Option<String>,
}

/// Build artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub hash: String,
}

/// Report from unit tests oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTestReport {
    pub oracle_id: String,
    pub status: String,
    pub tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub tests_skipped: usize,
    pub failures: Vec<TestFailure>,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Test failure details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailure {
    pub test_name: String,
    pub module: String,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

/// Report from lint oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintReport {
    pub oracle_id: String,
    pub status: String,
    pub fmt_issues: usize,
    pub clippy_warnings: usize,
    pub clippy_errors: usize,
    pub issues: Vec<LintIssue>,
    pub timestamp: DateTime<Utc>,
}

/// Lint issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintIssue {
    pub tool: String,
    pub file: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub code: String,
    pub message: String,
    pub severity: String,
}

/// Report from integrity smoke oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegritySmokeReport {
    pub oracle_id: String,
    pub status: String,
    pub pathways_tested: Vec<IntegrityPathway>,
    pub pathways_passed: usize,
    pub pathways_failed: usize,
    pub timestamp: DateTime<Utc>,
}

/// Integrity pathway test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityPathway {
    pub pathway: String,
    pub description: String,
    pub passed: bool,
    pub message: Option<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_with_core_suites() {
        let registry = OracleSuiteRegistry::with_core_suites();

        // Should have 5 suites (GOV, CORE, FULL, INTEGRATION, and semantic intake_admissibility)
        let suites = futures::executor::block_on(async { registry.list_suites().await });
        assert_eq!(suites.len(), 5);
    }

    #[test]
    fn test_registry_includes_semantic_suite() {
        let registry = OracleSuiteRegistry::with_core_suites();

        let suite = futures::executor::block_on(async {
            registry.get_suite(SUITE_INTAKE_ADMISSIBILITY_ID).await
        });

        assert!(suite.is_some());
        let suite = suite.unwrap();
        assert!(suite.metadata.contains_key("semantic_set_id"));
        assert!(suite.metadata.contains_key("semantic_set_hash"));
    }

    #[test]
    fn test_suite_hash_determinism() {
        let oracles = vec![create_meta_validate_oracle(), create_build_oracle()];

        let hash1 = compute_suite_hash(&oracles);
        let hash2 = compute_suite_hash(&oracles);

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }

    #[test]
    fn test_gov_suite_creation() {
        let suite = create_gov_suite();

        assert_eq!(suite.suite_id, SUITE_GOV_ID);
        assert_eq!(suite.oracles.len(), 2);
        assert_eq!(suite.environment_constraints.network, NetworkMode::Disabled);
    }

    #[test]
    fn test_core_suite_creation() {
        let suite = create_core_suite();

        assert_eq!(suite.suite_id, SUITE_CORE_ID);
        assert_eq!(suite.oracles.len(), 6);

        // Check oracle IDs
        let oracle_ids: Vec<_> = suite.oracles.iter().map(|o| o.oracle_id.as_str()).collect();
        assert!(oracle_ids.contains(&oracle_ids::META_VALIDATE));
        assert!(oracle_ids.contains(&oracle_ids::BUILD));
        assert!(oracle_ids.contains(&oracle_ids::UNIT_TESTS));
        assert!(oracle_ids.contains(&oracle_ids::LINT));
    }

    #[test]
    fn test_full_suite_creation() {
        let suite = create_full_suite();

        assert_eq!(suite.suite_id, SUITE_FULL_ID);
        assert_eq!(suite.oracles.len(), 10);
        // Full suite needs network for integration tests
        assert_eq!(suite.environment_constraints.network, NetworkMode::Private);
    }

    #[test]
    fn test_suite_validation() {
        let suite = create_core_suite();
        assert!(validate_suite(&suite).is_ok());
    }

    #[test]
    fn test_suite_validation_invalid_id() {
        let mut suite = create_core_suite();
        suite.suite_id = "invalid".to_string();

        let result = validate_suite(&suite);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuiteValidationError::InvalidSuiteId { .. }
        ));
    }

    #[test]
    fn test_suite_validation_hash_mismatch() {
        let mut suite = create_core_suite();
        suite.suite_hash = "sha256:wrong".to_string();

        let result = validate_suite(&suite);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuiteValidationError::HashMismatch { .. }
        ));
    }

    #[test]
    fn test_strict_core_profile() {
        let profile = create_strict_core_profile();

        assert_eq!(profile.profile_id, PROFILE_STRICT_CORE);
        assert!(profile.required_suites.contains(&SUITE_CORE_ID.to_string()));
        assert!(profile
            .waivable_failures
            .contains(&WaivableCondition::BuildFail));
        assert!(profile
            .integrity_conditions
            .contains(&IntegrityCondition::OracleTamper));
    }

    #[test]
    fn test_strict_full_profile() {
        let profile = create_strict_full_profile();

        assert_eq!(profile.profile_id, PROFILE_STRICT_FULL);
        assert!(profile.required_suites.contains(&SUITE_FULL_ID.to_string()));
        assert!(profile
            .applicable_deliverables
            .contains(&"D-34".to_string()));
    }

    #[test]
    fn test_build_oracle_env() {
        let oracle = create_build_oracle();

        assert!(oracle.env.contains_key("CARGO_TERM_COLOR"));
        assert_eq!(oracle.env.get("CARGO_TERM_COLOR").unwrap(), "never");
    }

    #[test]
    fn test_oracle_report_serialization() {
        let report = BuildReport {
            oracle_id: oracle_ids::BUILD.to_string(),
            status: "pass".to_string(),
            targets_built: 10,
            targets_failed: 0,
            warnings: 2,
            errors: vec![],
            artifacts: vec![BuildArtifact {
                name: "sr-api".to_string(),
                path: "target/release/sr-api".to_string(),
                size: 1024,
                hash: "sha256:abc123".to_string(),
            }],
            duration_ms: 5000,
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string_pretty(&report).unwrap();
        let parsed: BuildReport = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.oracle_id, oracle_ids::BUILD);
        assert_eq!(parsed.targets_built, 10);
    }

    #[test]
    fn test_waivable_condition_serialization() {
        let condition = WaivableCondition::BuildFail;
        let json = serde_json::to_string(&condition).unwrap();
        assert_eq!(json, "\"BUILD_FAIL\"");

        let parsed: WaivableCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, WaivableCondition::BuildFail);
    }

    #[test]
    fn test_integrity_condition_serialization() {
        let condition = IntegrityCondition::OracleTamper;
        let json = serde_json::to_string(&condition).unwrap();
        assert_eq!(json, "\"ORACLE_TAMPER\"");

        let parsed: IntegrityCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, IntegrityCondition::OracleTamper);
    }
}
