//! SOLVER-Ralph HTTP API Service (D-17)
//!
//! This is the main entry point for the SOLVER-Ralph API server.
//! Per SR-SPEC §2, it provides HTTP endpoints for:
//! - Loops, Iterations, Candidates, Runs
//! - Approvals, Exceptions, Decisions
//! - Evidence, Governed Artifacts, Freeze Records
//! - Staleness management
//!
//! Authentication is handled via OIDC (Zitadel) with JWT validation.

pub mod auth;
pub mod config;

use auth::{init_oidc, AuthenticatedUser, OptionalAuth};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use config::ApiConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
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
    let protected_routes = Router::new()
        .route("/api/v1/protected", get(protected_info));

    // Combine all routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
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

    // Create application state
    let state = AppState {
        config: config.clone(),
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

    fn test_state() -> AppState {
        AppState {
            config: ApiConfig::test(),
        }
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_router(test_state());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_info_endpoint() {
        let app = create_router(test_state());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/info")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_whoami_unauthenticated() {
        let app = create_router(test_state());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/whoami")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_protected_without_auth() {
        let app = create_router(test_state());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/protected")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should return 401 without auth header
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
