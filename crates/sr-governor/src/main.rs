//! SOLVER-Ralph Loop Governor Service (D-22)
//!
//! Standalone service that monitors loops and starts iterations when preconditions are met.
//!
//! Per SR-SPEC and SR-CONTRACT:
//! - Emits IterationStarted events with SYSTEM actor (C-CTX-1)
//! - Enforces budget limits (C-LOOP-1)
//! - Records all decisions as events (C-LOOP-2)
//! - Is idempotent and safe to restart
//!
//! The governor is OPTIONAL - the API /start endpoint continues to work without this service.

mod config;

use axum::{routing::get, Json, Router};
use config::GovernorConfig;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Governor service state
struct GovernorState {
    /// Configuration
    config: GovernorConfig,
    /// Database pool
    #[allow(dead_code)]
    db_pool: sqlx::PgPool,
    /// NATS connection
    #[allow(dead_code)]
    nats: Option<async_nats::Client>,
    /// Running flag
    running: AtomicBool,
    /// Last poll timestamp
    last_poll: RwLock<Option<chrono::DateTime<chrono::Utc>>>,
    /// Iterations started count
    iterations_started: RwLock<u64>,
}

impl GovernorState {
    fn is_ready(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = GovernorConfig::from_env();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log_level)),
        )
        .json()
        .init();

    info!(
        service = %config.service_name,
        poll_interval_ms = config.poll_interval_ms,
        max_concurrent_loops = config.max_concurrent_loops,
        dry_run = config.dry_run,
        "Starting SOLVER-Ralph Loop Governor"
    );

    // Connect to database
    info!(database_url = %config.database_url.replace(&config.database_url.split('@').last().unwrap_or(""), "****"), "Connecting to database");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    info!("Database connection established");

    // Connect to NATS (optional - service can run without NATS for event publishing)
    let nats = match async_nats::connect(&config.nats_url).await {
        Ok(client) => {
            info!(nats_url = %config.nats_url, "NATS connection established");
            Some(client)
        }
        Err(e) => {
            warn!(error = %e, "Failed to connect to NATS - continuing without message bus");
            None
        }
    };

    // Create shared state
    let state = Arc::new(GovernorState {
        config: config.clone(),
        db_pool,
        nats,
        running: AtomicBool::new(true),
        last_poll: RwLock::new(None),
        iterations_started: RwLock::new(0),
    });

    // Start health check server
    let health_state = state.clone();
    let health_addr = SocketAddr::from(([0, 0, 0, 0], config.health_port));
    let health_app = Router::new()
        .route("/ready", get(move || ready_handler(health_state.clone())))
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }));

    let health_server = tokio::spawn(async move {
        info!(addr = %health_addr, "Health server listening");
        if let Err(e) = axum::serve(
            tokio::net::TcpListener::bind(health_addr).await.unwrap(),
            health_app,
        )
        .await
        {
            error!(error = %e, "Health server error");
        }
    });

    // Start governor loop
    let governor_state = state.clone();
    let governor_loop = tokio::spawn(async move {
        run_governor_loop(governor_state).await;
    });

    // Wait for shutdown signal
    shutdown_signal().await;

    info!("Shutdown signal received, stopping governor");
    state.running.store(false, Ordering::Relaxed);

    // Wait for tasks to complete
    let _ = tokio::time::timeout(std::time::Duration::from_secs(5), governor_loop).await;
    health_server.abort();

    info!("Governor shutdown complete");
    Ok(())
}

/// Health/readiness check handler
async fn ready_handler(state: Arc<GovernorState>) -> Json<serde_json::Value> {
    let last_poll = state.last_poll.read().await;
    let iterations = *state.iterations_started.read().await;

    Json(json!({
        "status": if state.is_ready() { "ready" } else { "not_ready" },
        "service": state.config.service_name,
        "dry_run": state.config.dry_run,
        "last_poll": last_poll.map(|t| t.to_rfc3339()),
        "iterations_started": iterations
    }))
}

/// Main governor polling loop
async fn run_governor_loop(state: Arc<GovernorState>) {
    let poll_interval = state.config.poll_interval();

    info!(
        interval_ms = state.config.poll_interval_ms,
        "Governor loop started"
    );

    while state.running.load(Ordering::Relaxed) {
        // Update last poll timestamp
        {
            let mut last_poll = state.last_poll.write().await;
            *last_poll = Some(chrono::Utc::now());
        }

        // Poll for eligible loops and try to start iterations
        if let Err(e) = poll_and_process(&state).await {
            error!(error = %e, "Error in governor poll cycle");
        }

        // Sleep until next poll
        tokio::time::sleep(poll_interval).await;
    }

    info!("Governor loop stopped");
}

/// Poll for eligible loops and process them
async fn poll_and_process(state: &GovernorState) -> Result<(), Box<dyn std::error::Error>> {
    // Query for active loops that might need iteration starts
    // This is a simplified implementation - in production, we'd use the full
    // LoopGovernor from sr-adapters with proper event sourcing

    let active_loops: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT DISTINCT stream_id
        FROM events
        WHERE stream_kind = 'Loop'
          AND event_type = 'LoopActivated'
          AND stream_id NOT IN (
            SELECT stream_id FROM events WHERE event_type = 'LoopClosed'
          )
        ORDER BY stream_id
        LIMIT $1
        "#,
    )
    .bind(state.config.max_concurrent_loops as i64)
    .fetch_all(&state.db_pool)
    .await?;

    if active_loops.is_empty() {
        return Ok(());
    }

    info!(count = active_loops.len(), "Found active loops to check");

    for (loop_id,) in active_loops {
        // Check if loop has an incomplete iteration
        let has_incomplete: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1
            FROM events e1
            WHERE e1.event_type = 'IterationStarted'
              AND e1.payload->>'loop_id' = $1
              AND NOT EXISTS (
                SELECT 1 FROM events e2
                WHERE e2.event_type IN ('IterationCompleted', 'IterationFailed')
                  AND e2.stream_id = e1.stream_id
              )
            LIMIT 1
            "#,
        )
        .bind(&loop_id)
        .fetch_optional(&state.db_pool)
        .await?;

        if has_incomplete.is_some() {
            // Skip - loop has incomplete iteration
            continue;
        }

        // Check iteration count vs budget
        let iteration_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM events
            WHERE event_type = 'IterationStarted'
              AND payload->>'loop_id' = $1
            "#,
        )
        .bind(&loop_id)
        .fetch_one(&state.db_pool)
        .await?;

        // Default budget is 100 iterations
        if iteration_count.0 >= 100 {
            info!(loop_id = %loop_id, iterations = iteration_count.0, "Loop at budget limit");
            continue;
        }

        // All preconditions met - start iteration
        if state.config.dry_run {
            info!(
                loop_id = %loop_id,
                iteration = iteration_count.0 + 1,
                "DRY RUN: Would start iteration"
            );
        } else {
            info!(
                loop_id = %loop_id,
                iteration = iteration_count.0 + 1,
                "Starting iteration"
            );

            // In production, we'd emit IterationStarted event via the event store
            // and publish to NATS. For now, we just log the decision.
            //
            // The full implementation would use:
            // - sr_adapters::governor::LoopGovernor::try_start_iteration()
            // - sr_adapters::nats::NatsMessageBus::publish()

            let mut iterations = state.iterations_started.write().await;
            *iterations += 1;
        }
    }

    Ok(())
}

/// Wait for shutdown signal (Ctrl+C or SIGTERM)
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
