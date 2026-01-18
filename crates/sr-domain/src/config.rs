//! Config type definitions per SR-TYPES §4.5
//!
//! These structs provide schemas for config.* type keys needed for ontological
//! completeness (P2-TYPES-CONFIG).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Type key constants
pub const AGENT_DEFINITION_TYPE_KEY: &str = "config.agent_definition";
pub const ORACLE_DEFINITION_TYPE_KEY: &str = "config.oracle_definition";
pub const PORTAL_DEFINITION_TYPE_KEY: &str = "config.portal_definition";
pub const SEMANTIC_PROFILE_TYPE_KEY: &str = "config.semantic_profile";

/// AgentDefinition captures non-authoritative agent profile metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    pub id: String,
    pub name: String,
    pub actor_id: String,
    pub status: String,
    pub capabilities: Vec<String>,
    pub current_work_unit_id: Option<String>,
    pub iterations_completed: u32,
    pub proposals_produced: u32,
    pub last_active_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// PortalDefinition captures portal configuration (trust boundary)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalDefinition {
    pub portal_id: String,
    pub name: String,
    pub description: String,
    /// Approval decisions supported by the portal (e.g., APPROVED/REJECTED/DEFERRED)
    pub decisions: Vec<String>,
    /// Whether this portal is part of the seeded set
    pub seeded: bool,
}

/// Minimal oracle definition config for registry (config.oracle_definition)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleDefinitionConfig {
    pub oracle_id: String,
    pub oracle_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub expected_outputs: Vec<ExpectedOutputConfig>,
    #[serde(default)]
    pub classification: Option<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
}

/// Expected output description for oracle definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedOutputConfig {
    pub path: String,
    pub content_type: String,
    pub required: bool,
}

/// SemanticProfile defines stage → oracle suite bindings for semantic work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticProfile {
    pub profile_id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub stage_bindings: Vec<SemanticProfileBinding>,
    #[serde(default)]
    pub deliverable_kinds: Vec<String>,
}

/// Stage binding within a semantic profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticProfileBinding {
    pub stage_id: String,
    pub oracle_suite_id: String,
}
