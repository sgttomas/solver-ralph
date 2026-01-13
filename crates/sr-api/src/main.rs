//! SOLVER-Ralph HTTP API Service (D-17, D-18)
//!
//! This is the main entry point for the SOLVER-Ralph API server.
//! Per SR-SPEC §2, it provides HTTP endpoints for:
//! - Loops, Iterations, Candidates, Runs (D-18)
//! - Approvals, Exceptions, Decisions (D-19)
//! - Evidence, Governed Artifacts, Freeze Records (D-19, D-20)
//! - Staleness management
//!
//! Authentication is handled via OIDC (Zitadel) with JWT validation.

pub mod auth;
pub mod config;
pub mod handlers;

use auth::{init_oidc, AuthenticatedUser, OptionalAuth};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use config::ApiConfig;
use handlers::{candidates, iterations, loops, runs};
use serde::{Deserialize, Serialize};
use sr_adapters::{PostgresEventStore, ProjectionBuilder};
use sr_ports::EventStore;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
    pub event_store: Arc<dyn EventStore>,
    pub projections: Arc<ProjectionBuilder>,
}

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

/// Create the API router with all routes
fn create_router(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health))
        .route("/api/v1/info", get(info))
        .route("/api/v1/whoami", get(whoami));

    // Protected routes (authentication required)
    let protected_routes = Router::new().route("/api/v1/protected", get(protected_info));

    // Loop routes (D-18)
    let loop_routes = Router::new()
        .route("/api/v1/loops", post(loops::create_loop))
        .route("/api/v1/loops", get(loops::list_loops))
        .route("/api/v1/loops/:loop_id", get(loops::get_loop))
        .route("/api/v1/loops/:loop_id/activate", post(loops::activate_loop))
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
            "/api/v1/candidates/:candidate_id/runs",
            get(runs::list_runs_for_candidate),
        );

    // Run routes (D-18)
    let run_routes = Router::new()
        .route("/api/v1/runs", post(runs::start_run))
        .route("/api/v1/runs", get(runs::list_runs))
        .route("/api/v1/runs/:run_id", get(runs::get_run))
        .route("/api/v1/runs/:run_id/complete", post(runs::complete_run));

    // Combine all routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(loop_routes)
        .merge(iteration_routes)
        .merge(candidate_routes)
        .merge(run_routes)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Initialize tracing/logging
fn init_tracing(log_level: &str) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("sr_api={},tower_http=debug", log_level).into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    // Load configuration
    let config = ApiConfig::from_env();

    // Initialize tracing
    init_tracing(&config.log_level);

    info!(
        host = %config.host,
        port = %config.port,
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

    // Create application state
    let state = AppState {
        config: config.clone(),
        event_store,
        projections,
    };

    // Create router
    let app = create_router(state);

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
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

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
