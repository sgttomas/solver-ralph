//! SOLVER-Ralph HTTP API Service (D-17, D-18, D-19, D-20, D-33)
//!
//! This is the main entry point for the SOLVER-Ralph API server.
//! Per SR-SPEC §2, it provides HTTP endpoints for:
//! - Loops, Iterations, Candidates, Runs (D-18)
//! - Approvals, Exceptions, Decisions, Freeze Records (D-19)
//! - Evidence upload, retrieval, and association (D-20)
//! - Operational logging and observability (D-33)
//! - Staleness management
//!
//! Authentication is handled via OIDC (Zitadel) with JWT validation.

pub mod auth;
pub mod config;
pub mod governed;
pub mod handlers;
pub mod observability;
pub mod ref_validation;

use auth::{init_oidc, AuthenticatedUser, OptionalAuth};
use axum::{
    middleware,
    routing::{get, patch, post, put},
    Json, Router,
};
use config::ApiConfig;
use handlers::{
    approvals, attachments, candidates, config as config_handlers, decisions, evidence, exceptions,
    freeze, intakes, iterations, loops, oracles,
    prompt_loop::{prompt_loop, prompt_loop_stream},
    records, references, runs, staleness, templates, verification, work_surfaces,
};
use observability::{
    metrics_endpoint, ready_endpoint, request_context_middleware, Metrics, MetricsState,
    ReadinessState,
};
use serde::Serialize;
use sqlx::PgPool;
use sr_adapters::oracle_suite::OracleSuiteRegistry;
use sr_adapters::{
    AttachmentStoreConfig, CandidateWorkspaceConfig, EventManager, MinioAttachmentStore,
    MinioConfig, MinioEvidenceStore, NatsConfig, NatsMessageBus, PodmanOracleRunner,
    PodmanOracleRunnerConfig, PostgresEventStore, ProjectionBuilder, SemanticWorkerBridge,
    SemanticWorkerConfig, SimpleCandidateWorkspace,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
    pub event_store: Arc<PostgresEventStore>,
    pub projections: Arc<ProjectionBuilder>,
    pub evidence_store: Arc<MinioEvidenceStore>,
    pub oracle_registry: Arc<OracleSuiteRegistry>,
    /// Governed artifacts manifest computed at startup (V11-6)
    pub governed_manifest: Arc<governed::GovernedManifest>,
}

/// Oracle registry state for oracle-related endpoints
#[derive(Clone)]
pub struct OracleRegistryState {
    pub registry: Arc<OracleSuiteRegistry>,
}

/// Template registry state for template-related endpoints
pub use templates::{TemplateRegistry, TemplateRegistryState};

/// Work Surface state for work surface endpoints (SR-PLAN-V4)
pub use work_surfaces::WorkSurfaceState;

/// Attachment state for attachment upload endpoints (SR-PLAN-V7 V7-3)
pub use attachments::AttachmentState;

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

/// API info response
#[derive(Serialize)]
struct InfoResponse {
    name: &'static str,
    version: &'static str,
    description: &'static str,
}

/// Whoami response (current user info)
#[derive(Serialize)]
struct WhoamiResponse {
    authenticated: bool,
    actor_kind: Option<String>,
    actor_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    roles: Vec<String>,
}

/// Health check endpoint (unauthenticated)
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// API info endpoint (unauthenticated)
async fn info() -> Json<InfoResponse> {
    Json(InfoResponse {
        name: "SOLVER-Ralph API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Governance-first, event-sourced platform for controlled agentic work",
    })
}

/// Whoami endpoint (returns current user info, or anonymous if not authenticated)
async fn whoami(OptionalAuth(user): OptionalAuth) -> Json<WhoamiResponse> {
    match user {
        Some(u) => Json(WhoamiResponse {
            authenticated: true,
            actor_kind: Some(format!("{:?}", u.actor_kind)),
            actor_id: Some(u.actor_id),
            email: u.email,
            name: u.name,
            roles: u.roles,
        }),
        None => Json(WhoamiResponse {
            authenticated: false,
            actor_kind: None,
            actor_id: None,
            email: None,
            name: None,
            roles: vec![],
        }),
    }
}

/// Protected endpoint example (requires authentication)
async fn protected_info(user: AuthenticatedUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "You have access to protected resources",
        "actor_kind": format!("{:?}", user.actor_kind),
        "actor_id": user.actor_id,
    }))
}

/// References state for references endpoints (SR-PLAN-V3 Phase 0c)
pub use references::ReferencesState;

/// Create the API router with all routes (D-33: includes metrics and observability)
fn create_router(
    state: AppState,
    metrics_state: MetricsState,
    readiness_state: ReadinessState,
    oracle_state: OracleRegistryState,
    template_state: TemplateRegistryState,
    references_state: ReferencesState,
    work_surface_state: WorkSurfaceState,
    attachment_state: AttachmentState,
) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health))
        .route("/api/v1/info", get(info))
        .route("/api/v1/whoami", get(whoami))
        .route("/api/v1/metrics", get(metrics_endpoint))
        .with_state(metrics_state);

    // Readiness route (V11-3) - separate state
    let readiness_routes = Router::new()
        .route("/ready", get(ready_endpoint))
        .with_state(readiness_state);

    // Protected routes (authentication required)
    let protected_routes = Router::new().route("/api/v1/protected", get(protected_info));

    // Loop routes (D-18, V10-5)
    let loop_routes = Router::new()
        .route("/api/v1/loops", post(loops::create_loop))
        .route("/api/v1/loops", get(loops::list_loops))
        .route("/api/v1/loops/:loop_id", get(loops::get_loop))
        .route("/api/v1/loops/:loop_id", patch(loops::patch_loop)) // V10-5: PATCH with budget monotonicity
        .route(
            "/api/v1/loops/:loop_id/activate",
            post(loops::activate_loop),
        )
        .route("/api/v1/loops/:loop_id/pause", post(loops::pause_loop))
        .route("/api/v1/loops/:loop_id/resume", post(loops::resume_loop))
        .route("/api/v1/loops/:loop_id/close", post(loops::close_loop))
        .route(
            "/api/v1/loops/:loop_id/iterations",
            get(iterations::list_iterations_for_loop),
        );

    // Iteration routes (D-18)
    let iteration_routes = Router::new()
        .route("/api/v1/iterations", post(iterations::start_iteration))
        .route(
            "/api/v1/iterations/:iteration_id",
            get(iterations::get_iteration),
        )
        .route(
            "/api/v1/iterations/:iteration_id/loop-record",
            get(iterations::get_loop_record),
        )
        .route(
            "/api/v1/iterations/:iteration_id/complete",
            post(iterations::complete_iteration),
        )
        .route(
            "/api/v1/iterations/:iteration_id/candidates",
            get(candidates::list_candidates_for_iteration),
        );

    // Candidate routes (D-18)
    let candidate_routes = Router::new()
        .route("/api/v1/candidates", post(candidates::register_candidate))
        .route("/api/v1/candidates", get(candidates::list_candidates))
        .route(
            "/api/v1/candidates/:candidate_id",
            get(candidates::get_candidate),
        )
        .route(
            "/api/v1/candidates/:candidate_id/verify",
            post(verification::verify_candidate),
        )
        .route(
            "/api/v1/candidates/:candidate_id/runs",
            get(runs::list_runs_for_candidate),
        );

    // Run routes (D-18)
    let run_routes = Router::new()
        .route("/api/v1/runs", post(runs::start_run))
        .route("/api/v1/runs", get(runs::list_runs))
        .route("/api/v1/runs/:run_id", get(runs::get_run))
        .route("/api/v1/runs/:run_id/complete", post(runs::complete_run));

    // Approval routes (D-19) - HUMAN-only per SR-CONTRACT C-TB-3
    let approval_routes = Router::new()
        .route("/api/v1/approvals", post(approvals::record_approval))
        .route("/api/v1/approvals", get(approvals::list_approvals))
        .route(
            "/api/v1/approvals/:approval_id",
            get(approvals::get_approval),
        )
        .route(
            "/api/v1/portals/:portal_id/approvals",
            get(approvals::list_approvals_for_portal),
        );

    // Exception routes (D-19) - HUMAN-only per SR-SPEC §1.8
    let exception_routes = Router::new()
        .route("/api/v1/exceptions", post(exceptions::create_exception))
        .route("/api/v1/exceptions", get(exceptions::list_exceptions))
        .route(
            "/api/v1/exceptions/:exception_id",
            get(exceptions::get_exception),
        )
        .route(
            "/api/v1/exceptions/:exception_id/activate",
            post(exceptions::activate_exception),
        )
        .route(
            "/api/v1/exceptions/:exception_id/resolve",
            post(exceptions::resolve_exception),
        );

    // Decision routes (D-19) - HUMAN-only per SR-CONTRACT C-DEC-1
    let decision_routes = Router::new()
        .route("/api/v1/decisions", post(decisions::record_decision))
        .route("/api/v1/decisions", get(decisions::list_decisions))
        .route(
            "/api/v1/decisions/:decision_id",
            get(decisions::get_decision),
        );

    // Freeze record routes (D-19) - HUMAN-only per SR-CONTRACT C-SHIP-1
    let freeze_routes = Router::new()
        .route("/api/v1/freeze-records", post(freeze::create_freeze_record))
        .route("/api/v1/freeze-records", get(freeze::list_freeze_records))
        .route(
            "/api/v1/freeze-records/:freeze_id",
            get(freeze::get_freeze_record),
        )
        .route(
            "/api/v1/candidates/:candidate_id/freeze-records",
            get(freeze::list_freeze_records_for_candidate),
        );

    // Evidence routes (D-20) - Per SR-SPEC §1.9 and SR-CONTRACT §7
    let evidence_routes = Router::new()
        .route("/api/v1/evidence", post(evidence::upload_evidence))
        .route("/api/v1/evidence", get(evidence::list_evidence))
        .route(
            "/api/v1/evidence/:content_hash",
            get(evidence::get_evidence),
        )
        .route(
            "/api/v1/evidence/:content_hash/status",
            get(evidence::get_evidence_status),
        )
        .route(
            "/api/v1/evidence/:content_hash/associate",
            post(evidence::associate_evidence),
        )
        .route(
            "/api/v1/evidence/:content_hash/verify",
            post(evidence::verify_evidence),
        )
        .route(
            "/api/v1/evidence/:content_hash/blobs/:blob_name",
            get(evidence::get_evidence_blob),
        )
        .route(
            "/api/v1/runs/:run_id/evidence",
            get(evidence::list_evidence_for_run),
        )
        .route(
            "/api/v1/candidates/:candidate_id/evidence",
            get(evidence::list_evidence_for_candidate),
        );

    // Prompt-driven loop orchestration
    let prompt_routes = Router::new()
        .route("/api/v1/prompt-loop", post(prompt_loop))
        .route("/api/v1/prompt-loop/stream", post(prompt_loop_stream));

    // Oracle registry routes - Per SR-SEMANTIC-ORACLE-SPEC
    let oracle_routes = Router::new()
        .route(
            "/api/v1/oracles/suites",
            get(oracles::list_suites).post(oracles::register_suite),
        )
        .route("/api/v1/oracles/suites/:suite_id", get(oracles::get_suite))
        .route(
            "/api/v1/oracles/profiles",
            get(oracles::list_profiles).post(oracles::register_profile),
        )
        .route(
            "/api/v1/oracles/profiles/:profile_id",
            get(oracles::get_profile),
        )
        .with_state(oracle_state);

    // Template registry routes - Per SR-TEMPLATES
    let template_routes = Router::new()
        .route(
            "/api/v1/templates",
            get(templates::list_templates).post(templates::create_template),
        )
        .route("/api/v1/templates/schemas", get(templates::list_schemas))
        .route(
            "/api/v1/templates/schemas/:type_key",
            get(templates::get_schema),
        )
        .route(
            "/api/v1/templates/:template_id",
            get(templates::get_template).put(templates::update_template),
        )
        .with_state(template_state);

    // Config definition routes - P2-TYPES-CONFIG
    let config_routes = Router::new()
        .route(
            "/api/v1/config/definitions",
            get(config_handlers::list_config_definitions),
        )
        .route(
            "/api/v1/agents",
            post(config_handlers::create_agent_definition).get(config_handlers::list_agents),
        )
        .route(
            "/api/v1/portals",
            post(config_handlers::create_portal_definition),
        )
        .route(
            "/api/v1/oracle-definitions",
            post(config_handlers::create_oracle_definition),
        )
        .route(
            "/api/v1/semantic-profiles",
            post(config_handlers::create_semantic_profile),
        );

    // Intake routes - Per SR-PLAN-V3 Phase 0b
    let intake_routes = Router::new()
        .route(
            "/api/v1/intakes",
            get(intakes::list_intakes).post(intakes::create_intake),
        )
        .route("/api/v1/intakes/:intake_id", get(intakes::get_intake))
        .route("/api/v1/intakes/:intake_id", put(intakes::update_intake))
        .route(
            "/api/v1/intakes/:intake_id/activate",
            post(intakes::activate_intake),
        )
        .route(
            "/api/v1/intakes/:intake_id/archive",
            post(intakes::archive_intake),
        )
        .route(
            "/api/v1/intakes/:intake_id/fork",
            post(intakes::fork_intake),
        )
        .route(
            "/api/v1/intakes/by-hash/:content_hash",
            get(intakes::get_intake_by_hash),
        );

    // Work Surface routes - Per SR-PLAN-V4 Phase 4b
    let work_surface_routes = Router::new()
        .route(
            "/api/v1/work-surfaces",
            get(work_surfaces::list_work_surfaces).post(work_surfaces::create_work_surface),
        )
        .route(
            "/api/v1/work-surfaces/compatibility",
            get(work_surfaces::check_compatibility),
        )
        .route(
            "/api/v1/work-surfaces/compatible-templates",
            get(work_surfaces::get_compatible_templates),
        )
        .route(
            "/api/v1/work-surfaces/by-work-unit/:work_unit_id",
            get(work_surfaces::get_work_surface_by_work_unit),
        )
        .route(
            "/api/v1/work-surfaces/:work_surface_id",
            get(work_surfaces::get_work_surface),
        )
        .route(
            "/api/v1/procedure-instances/:work_surface_id",
            get(work_surfaces::get_procedure_instance),
        )
        .route(
            "/api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete",
            post(work_surfaces::complete_stage),
        )
        .route(
            "/api/v1/work-surfaces/:work_surface_id/stages/:stage_id/approval-status",
            get(work_surfaces::get_stage_approval_status),
        )
        .route(
            "/api/v1/work-surfaces/:work_surface_id/iteration-context",
            get(work_surfaces::get_iteration_context),
        )
        .route(
            "/api/v1/work-surfaces/:work_surface_id/archive",
            post(work_surfaces::archive_work_surface),
        )
        .route(
            "/api/v1/work-surfaces/:work_surface_id/start",
            post(work_surfaces::start_work_surface),
        )
        // V7-5: Work Surface iteration endpoints
        .route(
            "/api/v1/work-surfaces/:work_surface_id/iterations",
            get(work_surfaces::get_work_surface_iterations)
                .post(work_surfaces::start_work_surface_iteration),
        )
        // B3: Evidence by work surface (auto-linked evidence for WorkScreen)
        .route(
            "/api/v1/work-surfaces/:work_surface_id/evidence",
            get(evidence::list_evidence_for_work_surface),
        )
        .with_state(work_surface_state);

    // Attachment routes - Per SR-PLAN-V7 Phase V7-3
    let attachment_routes = Router::new()
        .route("/api/v1/attachments", post(attachments::upload_attachment))
        .with_state(attachment_state);

    // References routes - Per SR-PLAN-V3 Phase 0c
    let references_routes = Router::new()
        .route("/api/v1/references", get(references::list_references))
        .route(
            "/api/v1/references/governed-artifacts",
            get(references::list_governed_artifacts),
        )
        .route(
            "/api/v1/references/governed-artifacts/:id",
            get(references::get_governed_artifact),
        )
        .route(
            "/api/v1/references/candidates",
            get(references::list_candidates),
        )
        .route(
            "/api/v1/references/evidence-bundles",
            get(references::list_evidence_bundles),
        )
        .route(
            "/api/v1/references/evidence-bundles/:hash",
            get(references::get_evidence_bundle),
        )
        .route(
            "/api/v1/references/oracle-suites",
            get(references::list_oracle_suites),
        )
        .route(
            "/api/v1/references/templates",
            get(references::list_templates),
        )
        .route(
            "/api/v1/references/exceptions",
            get(references::list_exceptions),
        )
        .route(
            "/api/v1/references/iteration-summaries",
            get(references::list_iteration_summaries),
        )
        .route(
            "/api/v1/references/agent-definitions",
            get(references::list_agent_definitions),
        )
        .route(
            "/api/v1/references/gating-policies",
            get(references::list_gating_policies),
        )
        .route(
            "/api/v1/references/intakes",
            get(references::list_intakes_as_refs),
        )
        .route(
            "/api/v1/references/documents",
            post(references::upload_document),
        )
        .route(
            "/api/v1/references/documents/:id",
            get(references::get_document),
        )
        .with_state(references_state);

    // Staleness routes per SR-SPEC §2.3.9
    let staleness_routes = Router::new()
        .route("/api/v1/staleness/mark", post(staleness::mark_staleness))
        .route(
            "/api/v1/staleness/dependents",
            get(staleness::list_stale_dependents),
        )
        .route(
            "/api/v1/staleness/:stale_id/resolve",
            post(staleness::resolve_staleness),
        );

    // Human judgment records (non-binding notes)
    let record_routes = Router::new()
        .route(
            "/api/v1/records/evaluation-notes",
            post(records::create_evaluation_note),
        )
        .route(
            "/api/v1/records/assessment-notes",
            post(records::create_assessment_note),
        )
        .route(
            "/api/v1/records/intervention-notes",
            post(records::create_intervention_note),
        )
        .route("/api/v1/records/:record_id", get(records::get_record));

    // Combine all routes (D-33: request context middleware for correlation tracking)
    Router::new()
        .merge(public_routes)
        .merge(readiness_routes)
        .merge(protected_routes)
        .merge(loop_routes)
        .merge(iteration_routes)
        .merge(candidate_routes)
        .merge(run_routes)
        .merge(approval_routes)
        .merge(exception_routes)
        .merge(decision_routes)
        .merge(freeze_routes)
        .merge(evidence_routes)
        .merge(prompt_routes)
        .merge(oracle_routes)
        .merge(template_routes)
        .merge(config_routes)
        .merge(intake_routes)
        .merge(work_surface_routes)
        .merge(attachment_routes)
        .merge(references_routes)
        .merge(staleness_routes)
        .merge(record_routes)
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(request_context_middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Initialize tracing/logging (D-33)
///
/// Supports two output modes:
/// - JSON: Structured logging for production (SR_LOG_FORMAT=json)
/// - Pretty: Human-readable logging for development (default)
fn init_tracing(log_level: &str) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "sr_api={},sr_adapters={},sr_domain={},tower_http=debug",
            log_level, log_level, log_level
        )
        .into()
    });

    let use_json = std::env::var("SR_LOG_FORMAT")
        .map(|v| v.to_lowercase() == "json")
        .unwrap_or(false);

    if use_json {
        // JSON output for production: structured, machine-parseable
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_ids(true)
                    .with_target(true)
                    .flatten_event(false),
            )
            .init();
    } else {
        // Pretty output for development: human-readable
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_level(true)
                    .with_thread_ids(false)
                    .with_file(false)
                    .with_line_number(false),
            )
            .init();
    }
}

#[tokio::main]
async fn main() {
    // Record startup time for uptime metrics (D-33)
    let start_time = Instant::now();

    // Load configuration
    let config = ApiConfig::from_env();

    // Initialize tracing (D-33)
    init_tracing(&config.log_level);

    // Log startup with structured fields (D-33)
    info!(
        host = %config.host,
        port = %config.port,
        version = %env!("CARGO_PKG_VERSION"),
        log_format = %std::env::var("SR_LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string()),
        "Starting SOLVER-Ralph API"
    );

    // Initialize OIDC provider
    match init_oidc(config.oidc.clone()).await {
        Ok(_) => info!("OIDC provider initialized"),
        Err(e) => {
            warn!(error = ?e, "Failed to initialize OIDC provider - auth may not work");
        }
    }

    // Connect to database
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Database connection established");

    // Create adapters
    let event_store = Arc::new(PostgresEventStore::new(pool.clone()));
    let projections = Arc::new(ProjectionBuilder::new(pool.clone()));

    // Create evidence store (MinIO)
    let minio_config = MinioConfig {
        endpoint: std::env::var("MINIO_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:9000".to_string()),
        region: std::env::var("MINIO_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
        access_key_id: std::env::var("MINIO_ACCESS_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string()),
        secret_access_key: std::env::var("MINIO_SECRET_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string()),
        bucket: std::env::var("MINIO_BUCKET").unwrap_or_else(|_| "evidence".to_string()),
        force_path_style: true,
    };

    let evidence_store = Arc::new(
        MinioEvidenceStore::new(minio_config)
            .await
            .expect("Failed to initialize MinIO evidence store"),
    );

    info!("MinIO evidence store initialized");

    // Create attachment store (MinIO) - SR-PLAN-V7 Phase V7-3
    let attachment_store = Arc::new(
        MinioAttachmentStore::new(AttachmentStoreConfig::from_env())
            .await
            .expect("Failed to initialize MinIO attachment store"),
    );

    info!("MinIO attachment store initialized");

    // Compute governed artifacts manifest (V11-6)
    // Default to current directory + docs if DOCS_PATH is not set
    let docs_path = std::env::var("DOCS_PATH").unwrap_or_else(|_| "docs".to_string());
    let governed_manifest = Arc::new(governed::GovernedManifest::compute(std::path::Path::new(
        &docs_path,
    )));

    info!(
        artifact_count = governed_manifest.artifacts.len(),
        manifest_hash = %governed_manifest.manifest_hash,
        "Governed artifacts manifest computed"
    );

    // Create metrics state (D-33)
    let metrics = Arc::new(Metrics::new());
    let metrics_state = MetricsState {
        metrics: metrics.clone(),
        start_time,
    };

    info!("Metrics collection initialized");

    // Create readiness state (V11-3)
    let readiness_state = ReadinessState {
        db_pool: pool.clone(),
        minio_endpoint: std::env::var("MINIO_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:9000".to_string()),
        minio_bucket: std::env::var("MINIO_BUCKET").unwrap_or_else(|_| "evidence".to_string()),
        nats_url: config.nats_url.clone(),
    };

    info!("Readiness checks initialized");

    // Create oracle registry state (SR-SEMANTIC-ORACLE-SPEC)
    let oracle_registry = Arc::new(OracleSuiteRegistry::with_core_suites());
    let oracle_state = OracleRegistryState {
        registry: oracle_registry.clone(),
    };

    info!("Oracle registry initialized with core suites");

    // Create template registry state (SR-TEMPLATES)
    let template_registry = Arc::new(TemplateRegistry::new());
    let template_state = TemplateRegistryState {
        registry: template_registry.clone(),
    };

    info!("Template registry initialized with schemas");

    // Create application state
    let state = AppState {
        config: config.clone(),
        event_store,
        projections,
        evidence_store,
        oracle_registry: oracle_registry.clone(),
        governed_manifest,
    };

    // Create work surface state (SR-PLAN-V4 Phase 4b)
    let work_surface_state = WorkSurfaceState {
        app_state: state.clone(),
        oracle_registry: oracle_registry.clone(),
        template_registry: template_registry.clone(),
    };

    info!("Work surface state initialized");

    // Create attachment state (SR-PLAN-V7 Phase V7-3)
    let attachment_state = AttachmentState {
        app_state: state.clone(),
        attachment_store,
    };

    info!("Attachment state initialized");

    // Create references state (SR-PLAN-V3 Phase 0c)
    let references_state = ReferencesState {
        app_state: state.clone(),
        oracle_registry,
        template_registry,
    };

    info!("References state initialized");

    // Start semantic worker if enabled (V9-1)
    if config.enable_semantic_worker {
        info!("Semantic worker enabled - initializing dependencies");

        // Create NATS message bus
        let nats_config = NatsConfig {
            url: config.nats_url.clone(),
            stream_prefix: std::env::var("SR_NATS_STREAM_PREFIX")
                .unwrap_or_else(|_| "sr".to_string()),
            consumer_prefix: std::env::var("SR_NATS_CONSUMER_PREFIX")
                .unwrap_or_else(|_| "sr-consumer".to_string()),
            message_ttl_secs: std::env::var("SR_NATS_MESSAGE_TTL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(604800), // 7 days
            max_msgs_per_subject: std::env::var("SR_NATS_MAX_MSGS_PER_SUBJECT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(-1), // Unlimited
            duplicate_window_secs: std::env::var("SR_NATS_DUPLICATE_WINDOW_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(120),
        };

        match NatsMessageBus::connect(nats_config).await {
            Ok(nats_bus) => {
                let nats_bus = Arc::new(nats_bus);
                info!("NATS message bus connected for semantic worker");

                // Create Event Manager (using the database pool)
                let event_manager = Arc::new(RwLock::new(EventManager::new(pool.clone())));
                info!("Event manager initialized for semantic worker");

                // Create oracle runner
                let oracle_runner_config = PodmanOracleRunnerConfig::from_env();
                let oracle_runner = Arc::new(PodmanOracleRunner::new(
                    oracle_runner_config,
                    state.evidence_store.clone(),
                ));
                info!("Oracle runner initialized for semantic worker");

                // Create candidate workspace materializer
                let workspace_config = CandidateWorkspaceConfig::default();
                let candidate_workspace = Arc::new(SimpleCandidateWorkspace::new(workspace_config));
                info!("Candidate workspace materializer initialized");

                // Create semantic worker
                let worker_config = SemanticWorkerConfig::from_env();
                let semantic_worker = SemanticWorkerBridge::new(
                    worker_config,
                    Some(nats_bus),
                    state.event_store.clone(),
                    event_manager,
                    oracle_runner,
                    state.evidence_store.clone(),
                    references_state.oracle_registry.clone(),
                    candidate_workspace,
                );

                // Spawn semantic worker in background task
                tokio::spawn(async move {
                    info!("Starting semantic worker");
                    if let Err(e) = semantic_worker.start().await {
                        tracing::error!(error = ?e, "Semantic worker failed");
                    }
                });

                info!("Semantic worker spawned");
            }
            Err(e) => {
                warn!(
                    error = ?e,
                    "Failed to connect to NATS - semantic worker not started"
                );
            }
        }
    }

    // Create router (D-33: includes metrics endpoint and request tracing)
    let app = create_router(
        state,
        metrics_state,
        readiness_state,
        oracle_state,
        template_state,
        references_state,
        work_surface_state,
        attachment_state,
    );

    // Start server
    let bind_addr = config.bind_addr();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect(&format!("Failed to bind to {}", bind_addr));

    info!(address = %bind_addr, "SOLVER-Ralph API listening");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Tests that require database are skipped in this module.
    // Full integration tests will be in a separate test crate or require
    // a test database setup.

    #[tokio::test]
    async fn test_health_endpoint_format() {
        // This test just checks the response format is valid JSON
        let response = HealthResponse {
            status: "healthy",
            version: "0.1.0",
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
    }

    #[tokio::test]
    async fn test_info_endpoint_format() {
        let response = InfoResponse {
            name: "SOLVER-Ralph API",
            version: "0.1.0",
            description: "Test",
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("SOLVER-Ralph API"));
    }

    #[tokio::test]
    async fn test_whoami_response_unauthenticated() {
        let response = WhoamiResponse {
            authenticated: false,
            actor_kind: None,
            actor_id: None,
            email: None,
            name: None,
            roles: vec![],
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"authenticated\":false"));
    }
}
