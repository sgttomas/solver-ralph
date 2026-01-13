//! SOLVER-Ralph HTTP API Service
//!
//! This is the main entry point for the SOLVER-Ralph API server.
//! Per SR-SPEC §2, it provides HTTP endpoints for:
//! - Loops, Iterations, Candidates, Runs
//! - Approvals, Exceptions, Decisions
//! - Evidence, Governed Artifacts, Freeze Records
//! - Staleness management

use axum::{
    routing::get,
    Router,
    Json,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Serialize)]
struct InfoResponse {
    name: &'static str,
    version: &'static str,
    description: &'static str,
}

async fn info() -> Json<InfoResponse> {
    Json(InfoResponse {
        name: "SOLVER-Ralph API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Governance-first, event-sourced platform for controlled agentic work",
    })
}

fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/info", get(info))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sr_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("SOLVER-Ralph API listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_router();

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
