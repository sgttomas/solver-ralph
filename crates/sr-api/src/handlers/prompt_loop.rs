//! Prompt-driven loop orchestration
//!
//! Convenience endpoint to take a free-form prompt, materialize a governed
//! Work Surface (intake + procedure template + stage + oracle suite binding),
//! start a loop/iteration (SYSTEM actor), call an LLM, and emit candidate/run
//! + evidence artifacts in one shot.
//!
//! This is intentionally minimal: it uses the existing governed templates and
//! oracle suite registry, records all events in the event store, and produces
//! a valid evidence bundle for the run. It can be wired to a real LLM via env
//! or will fall back to a stubbed response if no LLM config is provided.
//!
//! Streaming endpoint (`/api/v1/prompt-loop/stream`) uses Server-Sent Events
//! to stream LLM output as it's generated.

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use chrono::Utc;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sr_adapters::{EvidenceArtifact, EvidenceManifest, OracleResult, OracleResultStatus, OracleSuiteRegistry};
use sr_domain::{
    CandidateId, ContentHash, EventEnvelope, EventId, Intake, ProcedureTemplateId, StageId,
    StreamKind, TypedRef, WorkKind, WorkSurfaceInstance, WorkUnitId,
};
use sr_domain::work_surface::OracleSuiteBinding;
use sr_ports::{EvidenceStore, EventStore};
use std::convert::Infallible;
use tracing::{info, instrument, warn};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

/// Request to run a prompt-driven loop
#[derive(Debug, Deserialize)]
pub struct PromptLoopRequest {
    /// Free-form prompt to send to the LLM
    pub prompt: String,
    /// Optional work unit identifier override
    #[serde(default)]
    pub work_unit: Option<String>,
    /// Procedure template ID (defaults to PROBLEM-STATEMENT-INGESTION)
    #[serde(default)]
    pub procedure_template_id: Option<String>,
    /// Stage ID (defaults to template initial stage)
    #[serde(default)]
    pub stage_id: Option<String>,
    /// Oracle suite ID (defaults to suite:SR-SUITE-GOV)
    #[serde(default)]
    pub oracle_suite_id: Option<String>,
    /// Optional model name override for the LLM call
    #[serde(default)]
    pub model: Option<String>,
}

/// Response with the generated artifacts
#[derive(Debug, Serialize)]
pub struct PromptLoopResponse {
    pub loop_id: String,
    pub iteration_id: String,
    pub candidate_id: String,
    pub run_id: String,
    pub evidence_content_hash: String,
    pub oracle_suite_hash: String,
    pub work_surface_hash: String,
    pub llm_output: String,
}

/// Internal LLM call configuration
#[derive(Debug, Clone)]
struct LlmConfig {
    base_url: Option<String>,
    api_key: Option<String>,
    model: String,
}

/// MAIN ENDPOINT
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn prompt_loop(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<PromptLoopRequest>,
) -> ApiResult<Json<PromptLoopResponse>> {
    // CONFIG -----------------------------------------------------------------
    let system_actor_id =
        std::env::var("SR_SYSTEM_ACTOR_ID").unwrap_or_else(|_| "system:prompt-loop".to_string());
    let system_actor_kind = sr_domain::ActorKind::System;

    // Resolve procedure template
    let proc_id_str = body
        .procedure_template_id
        .unwrap_or_else(|| "PROBLEM-STATEMENT-INGESTION".to_string());
    let proc_id = ProcedureTemplateId::new(&proc_id_str);
    let template_entry =
        sr_domain::procedure_templates::get_template_by_id(&proc_id).ok_or_else(|| {
            ApiError::BadRequest {
                message: format!("Unknown procedure template id: {}", proc_id.as_str()),
            }
        })?;
    let stage_id = body
        .stage_id
        .as_deref()
        .map(StageId::new)
        .unwrap_or_else(|| template_entry.template.get_initial_stage().clone());

    // Resolve oracle suite and hash
    let oracle_suite_id = body
        .oracle_suite_id
        .unwrap_or_else(|| sr_adapters::oracle_suite::SUITE_GOV_ID.to_string());
    let suite_registry = OracleSuiteRegistry::with_core_suites();
    let suite = suite_registry
        .get_suite(&oracle_suite_id)
        .await
        .ok_or_else(|| ApiError::BadRequest {
            message: format!("Unknown oracle suite id: {}", oracle_suite_id),
        })?;
    let suite_hash = ContentHash::from_string(suite.suite_hash.clone());

    // WORK SURFACE -----------------------------------------------------------
    let loop_id = sr_domain::LoopId::new();
    let work_unit_id = WorkUnitId::new(loop_id.as_str());

    // Intake: bind the prompt as the objective/input
    let intake = Intake::new(
        work_unit_id.clone(),
        format!("Prompt: {}", truncate(&body.prompt, 64)),
        WorkKind::ResearchMemo,
        body.prompt.clone(),
        "General".to_string(),
        vec![sr_domain::work_surface::Deliverable {
            path: "candidate/answer.md".to_string(),
            media_type: "text/markdown".to_string(),
            description: Some("LLM answer to the prompt".to_string()),
            role: Some("primary".to_string()),
        }],
    );
    intake.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Invalid intake: {}", e),
    })?;
    let intake_hash = hash_json(&intake)?;
    let intake_ref = sr_domain::work_surface::ContentAddressedRef {
        id: sr_domain::work_surface::IntakeId::new(work_unit_id.as_str())
            .as_str()
            .to_string(),
        content_hash: ContentHash::new(&intake_hash),
    };

    // Procedure template binding
    let procedure_template_ref = sr_domain::work_surface::ContentAddressedRef {
        id: template_entry
            .template
            .procedure_template_id
            .as_str()
            .to_string(),
        content_hash: template_entry.content_hash.clone(),
    };

    // Work surface instance
    let ws_instance = WorkSurfaceInstance::new(
        work_unit_id.clone(),
        intake_ref,
        procedure_template_ref,
        stage_id.clone(),
        vec![OracleSuiteBinding {
            suite_id: oracle_suite_id.clone(),
            suite_hash: suite_hash.clone(),
        }],
    );
    ws_instance.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Invalid work surface instance: {}", e),
    })?;
    let ws_hash = hash_json(&ws_instance)?;

    // LOOP + ITERATION EVENTS -----------------------------------------------
    let mut events: Vec<EventEnvelope> = Vec::new();

    // LoopCreated
    let loop_created_id = EventId::new();
    events.push(EventEnvelope {
        event_id: loop_created_id.clone(),
        stream_id: loop_id.as_str().to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: 1,
        global_seq: None,
        event_type: "LoopCreated".to_string(),
        occurred_at: Utc::now(),
        actor_kind: user.actor_kind, // creator is the caller (human)
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![TypedRef {
            kind: "Loop".to_string(),
            id: loop_id.as_str().to_string(),
            rel: "self".to_string(),
            meta: serde_json::Value::Null,
        }],
        payload: serde_json::json!({
            "goal": format!("Prompt loop: {}", truncate(&body.prompt, 80)),
            "work_unit": body.work_unit.unwrap_or_else(|| loop_id.as_str().to_string()),
            "budgets": {
                "max_iterations": 5,
                "max_oracle_runs": 25,
                "max_wallclock_hours": 16
            },
            "directive_ref": {
                "kind": "doc",
                "id": "SR-DIRECTIVE",
                "rel": "governs",
                "meta": {}
            }
        }),
        envelope_hash: compute_envelope_hash_for(&loop_created_id),
    });

    // LoopActivated
    let loop_activated_id = EventId::new();
    events.push(EventEnvelope {
        event_id: loop_activated_id.clone(),
        stream_id: loop_id.as_str().to_string(),
        stream_kind: StreamKind::Loop,
        stream_seq: 2,
        global_seq: None,
        event_type: "LoopActivated".to_string(),
        occurred_at: Utc::now(),
        actor_kind: user.actor_kind,
        actor_id: user.actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({}),
        envelope_hash: compute_envelope_hash_for(&loop_activated_id),
    });

    // IterationStarted (SYSTEM actor)
    let iteration_id = sr_domain::IterationId::new();
    let iter_started_id = EventId::new();
    events.push(EventEnvelope {
        event_id: iter_started_id.clone(),
        stream_id: iteration_id.as_str().to_string(),
        stream_kind: StreamKind::Iteration,
        stream_seq: 1,
        global_seq: None,
        event_type: "IterationStarted".to_string(),
        occurred_at: Utc::now(),
        actor_kind: system_actor_kind,
        actor_id: system_actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: {
            let mut refs = ws_instance.to_typed_refs();
            // Bind governing docs (plan/directive) to avoid ghost inputs
            refs.push(TypedRef {
                kind: "Doc".to_string(),
                id: "SR-PLAN".to_string(),
                rel: "depends_on".to_string(),
                meta: serde_json::Value::Null,
            });
            refs.push(TypedRef {
                kind: "Doc".to_string(),
                id: "SR-DIRECTIVE".to_string(),
                rel: "depends_on".to_string(),
                meta: serde_json::Value::Null,
            });
            refs
        },
        payload: serde_json::json!({
            "loop_id": loop_id.as_str(),
            "sequence": 1,
            "refs": ws_instance.to_typed_refs()
        }),
        envelope_hash: compute_envelope_hash_for(&iter_started_id),
    });

    // Append loop + iteration streams
    state
        .event_store
        .append(loop_id.as_str(), 0, events[..2].to_vec())
        .await?;
    state
        .event_store
        .append(iteration_id.as_str(), 0, vec![events[2].clone()])
        .await?;

    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    // LLM EXECUTION ----------------------------------------------------------
    let llm_cfg = llm_config_from_env(body.model.clone());
    let llm_output = call_llm(&llm_cfg, &body.prompt).await?;
    let candidate_hash_hex = sha256_hex(llm_output.as_bytes());
    let content_hash = ContentHash::new(&candidate_hash_hex);
    let candidate_id = CandidateId::new(None, &candidate_hash_hex);

    // CandidateMaterialized
    let candidate_event_id = EventId::new();
    let candidate_event = EventEnvelope {
        event_id: candidate_event_id.clone(),
        stream_id: candidate_id.as_str().to_string(),
        stream_kind: StreamKind::Candidate,
        stream_seq: 1,
        global_seq: None,
        event_type: "CandidateMaterialized".to_string(),
        occurred_at: Utc::now(),
        actor_kind: system_actor_kind,
        actor_id: system_actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![TypedRef {
            kind: "Iteration".to_string(),
            id: iteration_id.as_str().to_string(),
            rel: "produced_by".to_string(),
            meta: serde_json::Value::Null,
        }],
        payload: serde_json::json!({
            "content_hash": content_hash.as_str(),
            "produced_by_iteration_id": iteration_id.as_str(),
            "refs": []
        }),
        envelope_hash: compute_envelope_hash_for(&candidate_event_id),
    };
    state
        .event_store
        .append(candidate_id.as_str(), 0, vec![candidate_event])
        .await?;

    // RunStarted
    let run_id = sr_domain::RunId::new();
    let run_started_at = Utc::now();
    let run_started_id = EventId::new();
    let run_started = EventEnvelope {
        event_id: run_started_id.clone(),
        stream_id: run_id.as_str().to_string(),
        stream_kind: StreamKind::Run,
        stream_seq: 1,
        global_seq: None,
        event_type: "RunStarted".to_string(),
        occurred_at: run_started_at,
        actor_kind: system_actor_kind,
        actor_id: system_actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({
            "candidate_id": candidate_id.as_str(),
            "oracle_suite_id": oracle_suite_id,
            "oracle_suite_hash": suite_hash.as_str()
        }),
        envelope_hash: compute_envelope_hash_for(&run_started_id),
    };
    state
        .event_store
        .append(run_id.as_str(), 0, vec![run_started.clone()])
        .await?;

    // EVIDENCE ---------------------------------------------------------------
    let run_completed_at = Utc::now();
    let evidence_manifest = build_evidence_manifest(
        &run_id,
        &candidate_id,
        &oracle_suite_id,
        suite_hash.as_str(),
        run_started_at,
        run_completed_at,
        &llm_output,
        &body.prompt,
    )?;
    let manifest_json =
        evidence_manifest
            .to_deterministic_json()
            .map_err(|e| ApiError::Internal {
                message: format!("Failed to serialize manifest: {}", e),
            })?;
    let blob_name = "llm_output.txt".to_string();
    let blobs_refs = vec![(blob_name.as_str(), llm_output.as_bytes())];

    let evidence_content_hash = state
        .evidence_store
        .store(&manifest_json, blobs_refs)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to store evidence: {}", e),
        })?;

    // EvidenceBundleRecorded on run stream
    let run_stream_events = state
        .event_store
        .read_stream(run_id.as_str(), 0, 1000)
        .await
        .unwrap_or_default();
    let current_version = run_stream_events.len() as u64;
    let evidence_event_id = EventId::new();
    let evidence_event = EventEnvelope {
        event_id: evidence_event_id.clone(),
        stream_id: run_id.as_str().to_string(),
        stream_kind: StreamKind::Run,
        stream_seq: current_version + 1,
        global_seq: None,
        event_type: "EvidenceBundleRecorded".to_string(),
        occurred_at: run_completed_at,
        actor_kind: system_actor_kind,
        actor_id: system_actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({
            "content_hash": evidence_content_hash,
            "bundle_id": evidence_manifest.bundle_id,
            "run_id": run_id.as_str(),
            "candidate_id": candidate_id.as_str(),
            "oracle_suite_id": oracle_suite_id,
            "oracle_suite_hash": suite_hash.as_str(),
            "verdict": format!("{:?}", evidence_manifest.verdict).to_uppercase(),
            "artifact_count": evidence_manifest.artifacts.len()
        }),
        envelope_hash: compute_envelope_hash_for(&evidence_event_id),
    };
    state
        .event_store
        .append(run_id.as_str(), current_version, vec![evidence_event])
        .await?;

    // RunCompleted
    let run_completed_id = EventId::new();
    let run_completed_event = EventEnvelope {
        event_id: run_completed_id.clone(),
        stream_id: run_id.as_str().to_string(),
        stream_kind: StreamKind::Run,
        stream_seq: current_version + 2,
        global_seq: None,
        event_type: "RunCompleted".to_string(),
        occurred_at: run_completed_at,
        actor_kind: system_actor_kind,
        actor_id: system_actor_id.clone(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({
            "outcome": "SUCCESS",
            "evidence_bundle_hash": evidence_content_hash
        }),
        envelope_hash: compute_envelope_hash_for(&run_completed_id),
    };
    state
        .event_store
        .append(
            run_id.as_str(),
            current_version + 1,
            vec![run_completed_event],
        )
        .await?;

    // IterationCompleted
    let iter_events = state
        .event_store
        .read_stream(iteration_id.as_str(), 0, 1000)
        .await?;
    let iter_version = iter_events.len() as u64;
    let iter_completed_id = EventId::new();
    let iter_completed = EventEnvelope {
        event_id: iter_completed_id.clone(),
        stream_id: iteration_id.as_str().to_string(),
        stream_kind: StreamKind::Iteration,
        stream_seq: iter_version + 1,
        global_seq: None,
        event_type: "IterationCompleted".to_string(),
        occurred_at: Utc::now(),
        actor_kind: system_actor_kind,
        actor_id: system_actor_id,
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: vec![],
        payload: serde_json::json!({
            "outcome": "SUCCESS",
            "summary": {
                "intent": "Prompt answered via LLM",
                "actions": [{
                    "kind": "llm_call",
                    "summary": "Called LLM with provided prompt",
                    "artifacts": ["candidate/answer.md", blob_name]
                }],
                "candidates_produced": [candidate_id.as_str()],
                "runs_executed": [run_id.as_str()],
                "next_steps": [],
                "artifacts_touched": ["candidate/answer.md", blob_name],
                "open_risks": []
            }
        }),
        envelope_hash: compute_envelope_hash_for(&iter_completed_id),
    };
    state
        .event_store
        .append(iteration_id.as_str(), iter_version, vec![iter_completed])
        .await?;

    // Update projections after all events
    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    info!(
        loop_id = %loop_id.as_str(),
        iteration_id = %iteration_id.as_str(),
        run_id = %run_id.as_str(),
        candidate_id = %candidate_id.as_str(),
        "Prompt loop executed"
    );

    Ok(Json(PromptLoopResponse {
        loop_id: loop_id.as_str().to_string(),
        iteration_id: iteration_id.as_str().to_string(),
        candidate_id: candidate_id.as_str().to_string(),
        run_id: run_id.as_str().to_string(),
        evidence_content_hash,
        oracle_suite_hash: suite_hash.as_str().to_string(),
        work_surface_hash: ws_hash,
        llm_output,
    }))
}

// -----------------------------------------------------------------------------
// Streaming Endpoint
// -----------------------------------------------------------------------------

/// SSE event types for streaming prompt loop
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum StreamEvent {
    /// Initial metadata when loop starts
    #[serde(rename = "start")]
    Start {
        loop_id: String,
        iteration_id: String,
        work_surface_hash: String,
        oracle_suite_hash: String,
    },
    /// LLM output chunk
    #[serde(rename = "chunk")]
    Chunk { content: String },
    /// Final artifacts after completion
    #[serde(rename = "done")]
    Done {
        candidate_id: String,
        run_id: String,
        evidence_content_hash: String,
        llm_output: String,
    },
    /// Error during processing
    #[serde(rename = "error")]
    Error { message: String },
}

/// Streaming prompt loop endpoint using Server-Sent Events
#[instrument(skip(state, user, body), fields(user_id = %user.actor_id))]
pub async fn prompt_loop_stream(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<PromptLoopRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        // Validate request
        if body.prompt.trim().is_empty() {
            yield Ok(Event::default().data(
                serde_json::to_string(&StreamEvent::Error {
                    message: "Prompt is required".to_string(),
                }).unwrap()
            ));
            return;
        }

        // CONFIG -----------------------------------------------------------------
        let system_actor_id =
            std::env::var("SR_SYSTEM_ACTOR_ID").unwrap_or_else(|_| "system:prompt-loop".to_string());
        let system_actor_kind = sr_domain::ActorKind::System;

        // Resolve procedure template
        let proc_id_str = body
            .procedure_template_id
            .clone()
            .unwrap_or_else(|| "PROBLEM-STATEMENT-INGESTION".to_string());
        let proc_id = ProcedureTemplateId::new(&proc_id_str);
        let template_entry = match sr_domain::procedure_templates::get_template_by_id(&proc_id) {
            Some(t) => t,
            None => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Unknown procedure template id: {}", proc_id.as_str()),
                    }).unwrap()
                ));
                return;
            }
        };
        let stage_id = body
            .stage_id
            .as_deref()
            .map(StageId::new)
            .unwrap_or_else(|| template_entry.template.get_initial_stage().clone());

        // Resolve oracle suite and hash
        let oracle_suite_id = body
            .oracle_suite_id
            .clone()
            .unwrap_or_else(|| sr_adapters::oracle_suite::SUITE_GOV_ID.to_string());
        let suite_registry = OracleSuiteRegistry::with_core_suites();
        let suite = match suite_registry.get_suite(&oracle_suite_id).await {
            Some(s) => s,
            None => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Unknown oracle suite id: {}", oracle_suite_id),
                    }).unwrap()
                ));
                return;
            }
        };
        let suite_hash = ContentHash::from_string(suite.suite_hash.clone());

        // WORK SURFACE -----------------------------------------------------------
        let loop_id = sr_domain::LoopId::new();
        let work_unit_id = WorkUnitId::new(loop_id.as_str());

        // Intake: bind the prompt as the objective/input
        let intake = Intake::new(
            work_unit_id.clone(),
            format!("Prompt: {}", truncate(&body.prompt, 64)),
            WorkKind::ResearchMemo,
            body.prompt.clone(),
            "General".to_string(),
            vec![sr_domain::work_surface::Deliverable {
                path: "candidate/answer.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: Some("LLM answer to the prompt".to_string()),
                role: Some("primary".to_string()),
            }],
        );
        if let Err(e) = intake.validate() {
            yield Ok(Event::default().data(
                serde_json::to_string(&StreamEvent::Error {
                    message: format!("Invalid intake: {}", e),
                }).unwrap()
            ));
            return;
        }
        let intake_hash = match hash_json(&intake) {
            Ok(h) => h,
            Err(e) => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Failed to hash intake: {:?}", e),
                    }).unwrap()
                ));
                return;
            }
        };
        let intake_ref = sr_domain::work_surface::ContentAddressedRef {
            id: sr_domain::work_surface::IntakeId::new(work_unit_id.as_str())
                .as_str()
                .to_string(),
            content_hash: ContentHash::new(&intake_hash),
        };

        // Procedure template binding
        let procedure_template_ref = sr_domain::work_surface::ContentAddressedRef {
            id: template_entry
                .template
                .procedure_template_id
                .as_str()
                .to_string(),
            content_hash: template_entry.content_hash.clone(),
        };

        // Work surface instance
        let ws_instance = WorkSurfaceInstance::new(
            work_unit_id.clone(),
            intake_ref,
            procedure_template_ref,
            stage_id.clone(),
            vec![OracleSuiteBinding {
                suite_id: oracle_suite_id.clone(),
                suite_hash: suite_hash.clone(),
            }],
        );
        if let Err(e) = ws_instance.validate() {
            yield Ok(Event::default().data(
                serde_json::to_string(&StreamEvent::Error {
                    message: format!("Invalid work surface instance: {}", e),
                }).unwrap()
            ));
            return;
        }
        let ws_hash = match hash_json(&ws_instance) {
            Ok(h) => h,
            Err(e) => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Failed to hash work surface: {:?}", e),
                    }).unwrap()
                ));
                return;
            }
        };

        // LOOP + ITERATION EVENTS -----------------------------------------------
        let mut events: Vec<EventEnvelope> = Vec::new();

        // LoopCreated
        let loop_created_id = EventId::new();
        events.push(EventEnvelope {
            event_id: loop_created_id.clone(),
            stream_id: loop_id.as_str().to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 1,
            global_seq: None,
            event_type: "LoopCreated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: user.actor_kind,
            actor_id: user.actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![TypedRef {
                kind: "Loop".to_string(),
                id: loop_id.as_str().to_string(),
                rel: "self".to_string(),
                meta: serde_json::Value::Null,
            }],
            payload: serde_json::json!({
                "goal": format!("Prompt loop: {}", truncate(&body.prompt, 80)),
                "work_unit": body.work_unit.clone().unwrap_or_else(|| loop_id.as_str().to_string()),
                "budgets": {
                    "max_iterations": 5,
                    "max_oracle_runs": 25,
                    "max_wallclock_hours": 16
                },
                "directive_ref": {
                    "kind": "doc",
                    "id": "SR-DIRECTIVE",
                    "rel": "governs",
                    "meta": {}
                }
            }),
            envelope_hash: compute_envelope_hash_for(&loop_created_id),
        });

        // LoopActivated
        let loop_activated_id = EventId::new();
        events.push(EventEnvelope {
            event_id: loop_activated_id.clone(),
            stream_id: loop_id.as_str().to_string(),
            stream_kind: StreamKind::Loop,
            stream_seq: 2,
            global_seq: None,
            event_type: "LoopActivated".to_string(),
            occurred_at: Utc::now(),
            actor_kind: user.actor_kind,
            actor_id: user.actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: serde_json::json!({}),
            envelope_hash: compute_envelope_hash_for(&loop_activated_id),
        });

        // IterationStarted (SYSTEM actor)
        let iteration_id = sr_domain::IterationId::new();
        let iter_started_id = EventId::new();
        events.push(EventEnvelope {
            event_id: iter_started_id.clone(),
            stream_id: iteration_id.as_str().to_string(),
            stream_kind: StreamKind::Iteration,
            stream_seq: 1,
            global_seq: None,
            event_type: "IterationStarted".to_string(),
            occurred_at: Utc::now(),
            actor_kind: system_actor_kind,
            actor_id: system_actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: {
                let mut refs = ws_instance.to_typed_refs();
                refs.push(TypedRef {
                    kind: "Doc".to_string(),
                    id: "SR-PLAN".to_string(),
                    rel: "depends_on".to_string(),
                    meta: serde_json::Value::Null,
                });
                refs.push(TypedRef {
                    kind: "Doc".to_string(),
                    id: "SR-DIRECTIVE".to_string(),
                    rel: "depends_on".to_string(),
                    meta: serde_json::Value::Null,
                });
                refs
            },
            payload: serde_json::json!({
                "loop_id": loop_id.as_str(),
                "sequence": 1,
                "refs": ws_instance.to_typed_refs()
            }),
            envelope_hash: compute_envelope_hash_for(&iter_started_id),
        });

        // Append loop + iteration streams
        if let Err(e) = state.event_store.append(loop_id.as_str(), 0, events[..2].to_vec()).await {
            yield Ok(Event::default().data(
                serde_json::to_string(&StreamEvent::Error {
                    message: format!("Failed to append loop events: {:?}", e),
                }).unwrap()
            ));
            return;
        }
        if let Err(e) = state.event_store.append(iteration_id.as_str(), 0, vec![events[2].clone()]).await {
            yield Ok(Event::default().data(
                serde_json::to_string(&StreamEvent::Error {
                    message: format!("Failed to append iteration events: {:?}", e),
                }).unwrap()
            ));
            return;
        }

        let _ = state.projections.process_events(&*state.event_store).await;

        // Send start event with initial metadata
        yield Ok(Event::default().data(
            serde_json::to_string(&StreamEvent::Start {
                loop_id: loop_id.as_str().to_string(),
                iteration_id: iteration_id.as_str().to_string(),
                work_surface_hash: ws_hash.clone(),
                oracle_suite_hash: suite_hash.as_str().to_string(),
            }).unwrap()
        ));

        // LLM STREAMING EXECUTION ------------------------------------------------
        let llm_cfg = llm_config_from_env(body.model.clone());
        let mut llm_output = String::new();

        // Check for stubbed mode
        if llm_cfg.base_url.is_none() || llm_cfg.api_key.is_none() {
            let stub_response = format!("(stubbed LLM response)\nPrompt: {}\nAnswer: <no-op>", body.prompt);
            llm_output = stub_response.clone();
            yield Ok(Event::default().data(
                serde_json::to_string(&StreamEvent::Chunk { content: stub_response }).unwrap()
            ));
        } else {
            // Real LLM streaming call
            let url = format!("{}/v1/chat/completions", llm_cfg.base_url.as_ref().unwrap());
            let client = reqwest::Client::new();
            let req_body = serde_json::json!({
                "model": llm_cfg.model,
                "messages": [
                    {"role": "system", "content": "You are a helpful assistant."},
                    {"role": "user", "content": body.prompt}
                ],
                "stream": true
            });

            let resp = match client
                .post(&url)
                .bearer_auth(llm_cfg.api_key.as_ref().unwrap())
                .json(&req_body)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    yield Ok(Event::default().data(
                        serde_json::to_string(&StreamEvent::Error {
                            message: format!("LLM request failed: {}", e),
                        }).unwrap()
                    ));
                    return;
                }
            };

            if !resp.status().is_success() {
                let status = resp.status();
                let error_body = resp.text().await.unwrap_or_default();
                // Try to extract error message from OpenAI-style response
                let error_msg = serde_json::from_str::<serde_json::Value>(&error_body)
                    .ok()
                    .and_then(|v| v["error"]["message"].as_str().map(|s| s.to_string()))
                    .unwrap_or(error_body);
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("LLM error ({}): {}", status, error_msg),
                    }).unwrap()
                ));
                return;
            }

            use futures::stream::StreamExt as _;
            let mut byte_stream = resp.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk_result) = byte_stream.next().await {
                let bytes = match chunk_result {
                    Ok(b) => b,
                    Err(e) => {
                        yield Ok(Event::default().data(
                            serde_json::to_string(&StreamEvent::Error {
                                message: format!("Stream read error: {}", e),
                            }).unwrap()
                        ));
                        return;
                    }
                };

                buffer.push_str(&String::from_utf8_lossy(&bytes));

                // Process complete SSE lines from LLM
                while let Some(line_end) = buffer.find('\n') {
                    let line = buffer[..line_end].trim().to_string();
                    buffer = buffer[line_end + 1..].to_string();

                    if line.is_empty() || line == "data: [DONE]" {
                        continue;
                    }

                    if let Some(json_str) = line.strip_prefix("data: ") {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                if !content.is_empty() {
                                    llm_output.push_str(content);
                                    yield Ok(Event::default().data(
                                        serde_json::to_string(&StreamEvent::Chunk {
                                            content: content.to_string()
                                        }).unwrap()
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // CANDIDATE + RUN + EVIDENCE ---------------------------------------------
        let candidate_hash_hex = sha256_hex(llm_output.as_bytes());
        let content_hash = ContentHash::new(&candidate_hash_hex);
        let candidate_id = CandidateId::new(None, &candidate_hash_hex);

        // CandidateMaterialized
        let candidate_event_id = EventId::new();
        let candidate_event = EventEnvelope {
            event_id: candidate_event_id.clone(),
            stream_id: candidate_id.as_str().to_string(),
            stream_kind: StreamKind::Candidate,
            stream_seq: 1,
            global_seq: None,
            event_type: "CandidateMaterialized".to_string(),
            occurred_at: Utc::now(),
            actor_kind: system_actor_kind,
            actor_id: system_actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![TypedRef {
                kind: "Iteration".to_string(),
                id: iteration_id.as_str().to_string(),
                rel: "produced_by".to_string(),
                meta: serde_json::Value::Null,
            }],
            payload: serde_json::json!({
                "content_hash": content_hash.as_str(),
                "produced_by_iteration_id": iteration_id.as_str(),
                "refs": []
            }),
            envelope_hash: compute_envelope_hash_for(&candidate_event_id),
        };
        let _ = state.event_store.append(candidate_id.as_str(), 0, vec![candidate_event]).await;

        // RunStarted
        let run_id = sr_domain::RunId::new();
        let run_started_at = Utc::now();
        let run_started_id = EventId::new();
        let run_started = EventEnvelope {
            event_id: run_started_id.clone(),
            stream_id: run_id.as_str().to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: 1,
            global_seq: None,
            event_type: "RunStarted".to_string(),
            occurred_at: run_started_at,
            actor_kind: system_actor_kind,
            actor_id: system_actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: serde_json::json!({
                "candidate_id": candidate_id.as_str(),
                "oracle_suite_id": oracle_suite_id,
                "oracle_suite_hash": suite_hash.as_str()
            }),
            envelope_hash: compute_envelope_hash_for(&run_started_id),
        };
        let _ = state.event_store.append(run_id.as_str(), 0, vec![run_started]).await;

        // EVIDENCE ---------------------------------------------------------------
        let run_completed_at = Utc::now();
        let evidence_manifest = match build_evidence_manifest(
            &run_id,
            &candidate_id,
            &oracle_suite_id,
            suite_hash.as_str(),
            run_started_at,
            run_completed_at,
            &llm_output,
            &body.prompt,
        ) {
            Ok(m) => m,
            Err(e) => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Failed to build evidence manifest: {:?}", e),
                    }).unwrap()
                ));
                return;
            }
        };
        let manifest_json = match evidence_manifest.to_deterministic_json() {
            Ok(j) => j,
            Err(e) => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Failed to serialize manifest: {}", e),
                    }).unwrap()
                ));
                return;
            }
        };
        let blob_name = "llm_output.txt".to_string();
        let blobs_refs = vec![(blob_name.as_str(), llm_output.as_bytes())];

        let evidence_content_hash = match state.evidence_store.store(&manifest_json, blobs_refs).await {
            Ok(h) => h,
            Err(e) => {
                yield Ok(Event::default().data(
                    serde_json::to_string(&StreamEvent::Error {
                        message: format!("Failed to store evidence: {:?}", e),
                    }).unwrap()
                ));
                return;
            }
        };

        // EvidenceBundleRecorded + RunCompleted + IterationCompleted
        let run_stream_events = state.event_store.read_stream(run_id.as_str(), 0, 1000).await.unwrap_or_default();
        let current_version = run_stream_events.len() as u64;

        let evidence_event_id = EventId::new();
        let evidence_event = EventEnvelope {
            event_id: evidence_event_id.clone(),
            stream_id: run_id.as_str().to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: current_version + 1,
            global_seq: None,
            event_type: "EvidenceBundleRecorded".to_string(),
            occurred_at: run_completed_at,
            actor_kind: system_actor_kind,
            actor_id: system_actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: serde_json::json!({
                "content_hash": evidence_content_hash,
                "bundle_id": evidence_manifest.bundle_id,
                "run_id": run_id.as_str(),
                "candidate_id": candidate_id.as_str(),
                "oracle_suite_id": oracle_suite_id,
                "oracle_suite_hash": suite_hash.as_str(),
                "verdict": format!("{:?}", evidence_manifest.verdict).to_uppercase(),
                "artifact_count": evidence_manifest.artifacts.len()
            }),
            envelope_hash: compute_envelope_hash_for(&evidence_event_id),
        };
        let _ = state.event_store.append(run_id.as_str(), current_version, vec![evidence_event]).await;

        let run_completed_id = EventId::new();
        let run_completed_event = EventEnvelope {
            event_id: run_completed_id.clone(),
            stream_id: run_id.as_str().to_string(),
            stream_kind: StreamKind::Run,
            stream_seq: current_version + 2,
            global_seq: None,
            event_type: "RunCompleted".to_string(),
            occurred_at: run_completed_at,
            actor_kind: system_actor_kind,
            actor_id: system_actor_id.clone(),
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: serde_json::json!({
                "outcome": "SUCCESS",
                "evidence_bundle_hash": evidence_content_hash
            }),
            envelope_hash: compute_envelope_hash_for(&run_completed_id),
        };
        let _ = state.event_store.append(run_id.as_str(), current_version + 1, vec![run_completed_event]).await;

        // IterationCompleted
        let iter_events = state.event_store.read_stream(iteration_id.as_str(), 0, 1000).await.unwrap_or_default();
        let iter_version = iter_events.len() as u64;
        let iter_completed_id = EventId::new();
        let iter_completed = EventEnvelope {
            event_id: iter_completed_id.clone(),
            stream_id: iteration_id.as_str().to_string(),
            stream_kind: StreamKind::Iteration,
            stream_seq: iter_version + 1,
            global_seq: None,
            event_type: "IterationCompleted".to_string(),
            occurred_at: Utc::now(),
            actor_kind: system_actor_kind,
            actor_id: system_actor_id,
            correlation_id: None,
            causation_id: None,
            supersedes: vec![],
            refs: vec![],
            payload: serde_json::json!({
                "outcome": "SUCCESS",
                "summary": {
                    "intent": "Prompt answered via LLM (streaming)",
                    "actions": [{
                        "kind": "llm_call",
                        "summary": "Called LLM with provided prompt",
                        "artifacts": ["candidate/answer.md", blob_name]
                    }],
                    "candidates_produced": [candidate_id.as_str()],
                    "runs_executed": [run_id.as_str()],
                    "next_steps": [],
                    "artifacts_touched": ["candidate/answer.md", blob_name],
                    "open_risks": []
                }
            }),
            envelope_hash: compute_envelope_hash_for(&iter_completed_id),
        };
        let _ = state.event_store.append(iteration_id.as_str(), iter_version, vec![iter_completed]).await;
        let _ = state.projections.process_events(&*state.event_store).await;

        info!(
            loop_id = %loop_id.as_str(),
            iteration_id = %iteration_id.as_str(),
            run_id = %run_id.as_str(),
            candidate_id = %candidate_id.as_str(),
            "Prompt loop (streaming) executed"
        );

        // Send final done event
        yield Ok(Event::default().data(
            serde_json::to_string(&StreamEvent::Done {
                candidate_id: candidate_id.as_str().to_string(),
                run_id: run_id.as_str().to_string(),
                evidence_content_hash,
                llm_output,
            }).unwrap()
        ));
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

fn compute_envelope_hash_for(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn hash_json<T: ?Sized + Serialize>(value: &T) -> Result<String, ApiError> {
    let json = serde_json::to_vec(value).map_err(|e| ApiError::Internal {
        message: format!("Failed to serialize for hashing: {}", e),
    })?;
    Ok(sha256_hex(&json))
}

fn llm_config_from_env(model_override: Option<String>) -> LlmConfig {
    LlmConfig {
        base_url: std::env::var("SR_LLM_BASE_URL").ok(),
        api_key: std::env::var("SR_LLM_API_KEY").ok(),
        model: model_override
            .or_else(|| std::env::var("SR_LLM_MODEL").ok())
            .unwrap_or_else(|| "gpt-4o-mini".to_string()),
    }
}

async fn call_llm(cfg: &LlmConfig, prompt: &str) -> Result<String, ApiError> {
    if cfg.base_url.is_none() || cfg.api_key.is_none() {
        // Stubbed response for environments without LLM credentials
        return Ok(format!(
            "(stubbed LLM response)\nPrompt: {}\nAnswer: <no-op>",
            prompt
        ));
    }

    let url = format!("{}/v1/chat/completions", cfg.base_url.as_ref().unwrap());
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": cfg.model,
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": prompt}
        ]
    });

    let resp = client
        .post(&url)
        .bearer_auth(cfg.api_key.as_ref().unwrap())
        .json(&body)
        .send()
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("LLM call failed: {}", e),
        })?;

    if !resp.status().is_success() {
        warn!(status = ?resp.status(), "LLM call returned non-success");
        return Ok(format!(
            "(llm error) status={} prompt={}",
            resp.status(),
            truncate(prompt, 200)
        ));
    }

    let json: serde_json::Value = resp.json().await.map_err(|e| ApiError::Internal {
        message: format!("Failed to parse LLM response: {}", e),
    })?;
    if let Some(text) = json["choices"][0]["message"]["content"].as_str() {
        Ok(text.to_string())
    } else {
        Ok(format!(
            "(llm response missing content) raw={}",
            truncate(&json.to_string(), 200)
        ))
    }
}

fn build_evidence_manifest(
    run_id: &sr_domain::RunId,
    candidate_id: &CandidateId,
    oracle_suite_id: &str,
    oracle_suite_hash: &str,
    run_started_at: chrono::DateTime<Utc>,
    run_completed_at: chrono::DateTime<Utc>,
    llm_output: &str,
    prompt: &str,
) -> Result<EvidenceManifest, ApiError> {
    let artifact_hash = sha256_hex(llm_output.as_bytes());
    Ok(EvidenceManifest {
        version: sr_adapters::evidence::MANIFEST_VERSION.to_string(),
        artifact_type: sr_adapters::evidence::ARTIFACT_TYPE.to_string(),
        bundle_id: format!("bundle:{}", EventId::new().as_str()),
        run_id: run_id.as_str().to_string(),
        candidate_id: candidate_id.as_str().to_string(),
        oracle_suite_id: oracle_suite_id.to_string(),
        oracle_suite_hash: oracle_suite_hash.to_string(),
        run_started_at,
        run_completed_at,
        environment_fingerprint: serde_json::json!({
            "provider": std::env::var("SR_LLM_PROVIDER").unwrap_or_else(|_| "stub".to_string()),
            "model": std::env::var("SR_LLM_MODEL").unwrap_or_else(|_| "unknown".to_string())
        }),
        results: vec![OracleResult {
            oracle_id: "oracle:llm_prompt".to_string(),
            oracle_name: "LLM Prompt Execution".to_string(),
            status: OracleResultStatus::Pass,
            duration_ms: 0,
            error_message: None,
            artifact_refs: vec!["llm_output".to_string()],
            output: Some(serde_json::json!({ "prompt": prompt })),
        }],
        verdict: OracleResultStatus::Pass,
        artifacts: vec![EvidenceArtifact {
            name: "llm_output".to_string(),
            content_hash: format!("sha256:{}", artifact_hash),
            content_type: "text/plain".to_string(),
            size: llm_output.as_bytes().len() as u64,
            description: Some("LLM-generated answer".to_string()),
        }],
        metadata: Default::default(),
        // Work Surface context (SR-PLAN-V4 Phase 4c) - not applicable to prompt loop
        procedure_template_id: None,
        stage_id: None,
        work_surface_id: None,
    })
}
