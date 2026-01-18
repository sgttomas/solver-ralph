//! Config definition endpoints (P2-TYPES-CONFIG)
//!
//! Provides minimal CRUD-style registry for config.* type keys:
//! - config.agent_definition
//! - config.oracle_definition
//! - config.portal_definition
//! - config.semantic_profile

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_domain::config::{
    AgentDefinition, ExpectedOutputConfig, OracleDefinitionConfig, PortalDefinition,
    SemanticProfile, SemanticProfileBinding, AGENT_DEFINITION_TYPE_KEY, ORACLE_DEFINITION_TYPE_KEY,
    PORTAL_DEFINITION_TYPE_KEY, SEMANTIC_PROFILE_TYPE_KEY,
};
use sr_domain::{ActorKind, EventEnvelope, EventId, StreamKind};
use sr_ports::EventStore;
use ulid::Ulid;

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ConfigActionResponse {
    pub config_id: String,
    pub type_key: String,
    pub event_id: String,
}

/// List config definitions (optionally filtered by type_key)
#[derive(Debug, Deserialize)]
pub struct ListConfigQuery {
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConfigDefinitionResponse {
    pub config_id: String,
    pub type_key: String,
    pub name: String,
    pub definition: serde_json::Value,
    pub recorded_by_kind: String,
    pub recorded_by_id: String,
    pub recorded_at: String,
}

// ============================================================================
// Agent Definitions
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateAgentDefinitionRequest {
    #[serde(default = "default_agent_id")]
    pub id: String,
    pub name: String,
    pub actor_id: String,
    #[serde(default = "default_agent_status")]
    pub status: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub current_work_unit_id: Option<String>,
    #[serde(default)]
    pub iterations_completed: Option<u32>,
    #[serde(default)]
    pub proposals_produced: Option<u32>,
    #[serde(default)]
    pub last_active_at: Option<String>,
}

fn default_agent_id() -> String {
    format!("agent_{}", Ulid::new())
}

fn default_agent_status() -> String {
    "active".to_string()
}

#[derive(Debug, Serialize)]
pub struct AgentsResponse {
    pub agents: Vec<AgentDefinition>,
}

pub async fn create_agent_definition(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateAgentDefinitionRequest>,
) -> ApiResult<Json<ConfigActionResponse>> {
    ensure_human(&user)?;

    let created_at = Utc::now();
    let def = AgentDefinition {
        id: body.id.clone(),
        name: body.name.clone(),
        actor_id: body.actor_id.clone(),
        status: body.status.clone(),
        capabilities: body.capabilities.clone(),
        current_work_unit_id: body.current_work_unit_id.clone(),
        iterations_completed: body.iterations_completed.unwrap_or(0),
        proposals_produced: body.proposals_produced.unwrap_or(0),
        last_active_at: body
            .last_active_at
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        created_at,
    };

    record_config_definition(
        &state,
        &user,
        AGENT_DEFINITION_TYPE_KEY,
        &def.name,
        serde_json::to_value(&def).map_err(|e| ApiError::BadRequest {
            message: format!("Invalid agent definition: {e}"),
        })?,
    )
    .await
}

pub async fn list_agents(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> ApiResult<Json<AgentsResponse>> {
    let defs = state
        .projections
        .list_config_definitions(Some(AGENT_DEFINITION_TYPE_KEY))
        .await?;

    let mut agents = Vec::new();
    for def in defs {
        if let Ok(agent) = serde_json::from_value::<AgentDefinition>(def.definition.clone()) {
            agents.push(agent);
        }
    }

    Ok(Json(AgentsResponse { agents }))
}

// ============================================================================
// Portal Definitions
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreatePortalDefinitionRequest {
    pub portal_id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_portal_decisions")]
    pub decisions: Vec<String>,
    #[serde(default)]
    pub seeded: bool,
}

fn default_portal_decisions() -> Vec<String> {
    vec![
        "APPROVED".to_string(),
        "REJECTED".to_string(),
        "DEFERRED".to_string(),
    ]
}

pub async fn create_portal_definition(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreatePortalDefinitionRequest>,
) -> ApiResult<Json<ConfigActionResponse>> {
    ensure_human(&user)?;

    let def = PortalDefinition {
        portal_id: body.portal_id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        decisions: body.decisions.clone(),
        seeded: body.seeded,
    };

    record_config_definition(
        &state,
        &user,
        PORTAL_DEFINITION_TYPE_KEY,
        &def.name,
        serde_json::to_value(&def).map_err(|e| ApiError::BadRequest {
            message: format!("Invalid portal definition: {e}"),
        })?,
    )
    .await
}

// ============================================================================
// Oracle Definitions
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateOracleDefinitionRequest {
    pub oracle_id: String,
    pub oracle_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub expected_outputs: Vec<ExpectedOutputRequest>,
    #[serde(default)]
    pub classification: Option<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
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

pub async fn create_oracle_definition(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateOracleDefinitionRequest>,
) -> ApiResult<Json<ConfigActionResponse>> {
    ensure_human(&user)?;

    let outputs: Vec<ExpectedOutputConfig> = body
        .expected_outputs
        .iter()
        .map(|o| ExpectedOutputConfig {
            path: o.path.clone(),
            content_type: o.content_type.clone(),
            required: o.required,
        })
        .collect();

    let def = OracleDefinitionConfig {
        oracle_id: body.oracle_id.clone(),
        oracle_name: body.oracle_name.clone(),
        description: body.description.clone(),
        expected_outputs: outputs,
        classification: body.classification.clone(),
        metadata: serde_json::from_value(body.metadata.clone()).unwrap_or_default(),
    };

    record_config_definition(
        &state,
        &user,
        ORACLE_DEFINITION_TYPE_KEY,
        &def.oracle_name,
        serde_json::to_value(&def).map_err(|e| ApiError::BadRequest {
            message: format!("Invalid oracle definition: {e}"),
        })?,
    )
    .await
}

// ============================================================================
// Semantic Profiles
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateSemanticProfileRequest {
    pub profile_id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub stage_bindings: Vec<SemanticProfileBinding>,
    #[serde(default)]
    pub deliverable_kinds: Vec<String>,
}

pub async fn create_semantic_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<CreateSemanticProfileRequest>,
) -> ApiResult<Json<ConfigActionResponse>> {
    ensure_human(&user)?;

    let def = SemanticProfile {
        profile_id: body.profile_id.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        stage_bindings: body.stage_bindings.clone(),
        deliverable_kinds: body.deliverable_kinds.clone(),
    };

    record_config_definition(
        &state,
        &user,
        SEMANTIC_PROFILE_TYPE_KEY,
        &def.name,
        serde_json::to_value(&def).map_err(|e| ApiError::BadRequest {
            message: format!("Invalid semantic profile: {e}"),
        })?,
    )
    .await
}

// ============================================================================
// Shared helpers
// ============================================================================

pub async fn list_config_definitions(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListConfigQuery>,
) -> ApiResult<Json<Vec<ConfigDefinitionResponse>>> {
    let defs = state
        .projections
        .list_config_definitions(query.r#type.as_deref())
        .await?;

    let items = defs
        .into_iter()
        .map(|d| ConfigDefinitionResponse {
            config_id: d.config_id,
            type_key: d.type_key,
            name: d.name,
            definition: d.definition,
            recorded_by_kind: d.recorded_by_kind,
            recorded_by_id: d.recorded_by_id,
            recorded_at: d.recorded_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(items))
}

fn ensure_human(user: &AuthenticatedUser) -> ApiResult<()> {
    if !matches!(user.actor_kind, ActorKind::Human) {
        return Err(ApiError::Forbidden {
            message: "Config definitions must be recorded by HUMAN actors".to_string(),
        });
    }
    Ok(())
}

async fn record_config_definition(
    state: &AppState,
    user: &AuthenticatedUser,
    type_key: &str,
    name: &str,
    definition: serde_json::Value,
) -> ApiResult<Json<ConfigActionResponse>> {
    let config_id = format!("cfg_{}", Ulid::new());
    let event_id = EventId::new();
    let now = Utc::now();

    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: config_id.clone(),
        stream_kind: StreamKind::Governance,
        stream_seq: 1,
        global_seq: None,
        event_type: "ConfigDefinitionRecorded".to_string(),
        occurred_at: now,
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({
            "config_id": config_id,
            "type_key": type_key,
            "name": name,
            "definition": definition
        }),
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(&event.stream_id, 0, vec![event.clone()])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok(Json(ConfigActionResponse {
        config_id,
        type_key: type_key.to_string(),
        event_id: event_id.as_str().to_string(),
    }))
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}
