//! Observability Module (D-33)
//!
//! Provides operational logging, request tracing, and metrics for SOLVER-Ralph API.
//! Per SR-SPEC requirements:
//! - Structured logs with request/trace identifiers
//! - Actor identity in logs where applicable
//! - Key state transitions logged with event IDs
//! - Minimal metrics endpoints

use axum::{
    extract::{Request, State},
    http::{header::HeaderValue, HeaderName, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use chrono::Utc;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, info_span, Instrument};

/// Header name for request ID propagation
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Header name for correlation ID propagation (traces across services)
pub const CORRELATION_ID_HEADER: &str = "x-correlation-id";

/// Request context stored in extensions for propagation
#[derive(Clone, Debug)]
pub struct RequestContext {
    /// Unique ID for this request
    pub request_id: String,
    /// Correlation ID for distributed tracing (may span multiple services)
    pub correlation_id: String,
    /// Timestamp when request started
    pub started_at: chrono::DateTime<Utc>,
}

impl RequestContext {
    /// Generate a new request context
    pub fn new() -> Self {
        let id = generate_request_id();
        Self {
            request_id: id.clone(),
            correlation_id: id,
            started_at: Utc::now(),
        }
    }

    /// Create request context from headers or generate new IDs
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        let request_id = headers
            .get(REQUEST_ID_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(generate_request_id);

        let correlation_id = headers
            .get(CORRELATION_ID_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| request_id.clone());

        Self {
            request_id,
            correlation_id,
            started_at: Utc::now(),
        }
    }
}

impl Default for RequestContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a unique request ID using timestamp + random suffix
fn generate_request_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let random: u32 = rand::random();
    format!("req_{:x}{:08x}", timestamp, random)
}

/// Metrics collection for API observability (D-33, V11-3)
#[derive(Debug, Default)]
pub struct Metrics {
    // HTTP metrics
    /// Total requests received
    pub requests_total: AtomicU64,
    /// Total requests completed successfully (2xx)
    pub requests_success: AtomicU64,
    /// Total requests with client errors (4xx)
    pub requests_client_error: AtomicU64,
    /// Total requests with server errors (5xx)
    pub requests_server_error: AtomicU64,
    /// Total request latency in microseconds (for averaging)
    pub request_latency_us_total: AtomicU64,

    // Domain metrics (V11-3)
    /// Total loops created
    pub loops_created: AtomicU64,
    /// Total iterations started
    pub iterations_started: AtomicU64,
    /// Total iterations completed
    pub iterations_completed: AtomicU64,
    /// Total candidates registered
    pub candidates_registered: AtomicU64,
    /// Total oracle runs
    pub oracle_runs_total: AtomicU64,
    /// Total oracle runs passed
    pub oracle_runs_passed: AtomicU64,
    /// Total oracle runs failed
    pub oracle_runs_failed: AtomicU64,
    /// Total oracle run latency in microseconds
    pub oracle_run_latency_us_total: AtomicU64,
    /// Total event append latency in microseconds
    pub event_append_latency_us_total: AtomicU64,
    /// Total events appended
    pub events_appended: AtomicU64,
}

impl Metrics {
    /// Create new metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a completed request
    pub fn record_request(&self, status: StatusCode, latency_us: u64) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.request_latency_us_total
            .fetch_add(latency_us, Ordering::Relaxed);

        if status.is_success() {
            self.requests_success.fetch_add(1, Ordering::Relaxed);
        } else if status.is_client_error() {
            self.requests_client_error.fetch_add(1, Ordering::Relaxed);
        } else if status.is_server_error() {
            self.requests_server_error.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Domain metric recording methods (V11-3)

    /// Record a loop creation
    pub fn record_loop_created(&self) {
        self.loops_created.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an iteration start
    pub fn record_iteration_started(&self) {
        self.iterations_started.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an iteration completion
    pub fn record_iteration_completed(&self) {
        self.iterations_completed.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a candidate registration
    pub fn record_candidate_registered(&self) {
        self.candidates_registered.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an oracle run completion
    pub fn record_oracle_run(&self, passed: bool, latency_us: u64) {
        self.oracle_runs_total.fetch_add(1, Ordering::Relaxed);
        self.oracle_run_latency_us_total
            .fetch_add(latency_us, Ordering::Relaxed);
        if passed {
            self.oracle_runs_passed.fetch_add(1, Ordering::Relaxed);
        } else {
            self.oracle_runs_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Record an event append
    pub fn record_event_append(&self, latency_us: u64) {
        self.events_appended.fetch_add(1, Ordering::Relaxed);
        self.event_append_latency_us_total
            .fetch_add(latency_us, Ordering::Relaxed);
    }

    /// Get snapshot of current metrics
    pub fn snapshot(&self) -> MetricsSnapshot {
        let total = self.requests_total.load(Ordering::Relaxed);
        let latency_total = self.request_latency_us_total.load(Ordering::Relaxed);

        MetricsSnapshot {
            requests_total: total,
            requests_success: self.requests_success.load(Ordering::Relaxed),
            requests_client_error: self.requests_client_error.load(Ordering::Relaxed),
            requests_server_error: self.requests_server_error.load(Ordering::Relaxed),
            avg_latency_ms: if total > 0 {
                (latency_total / total) as f64 / 1000.0
            } else {
                0.0
            },
        }
    }

    /// Get snapshot of domain metrics (V11-3)
    pub fn domain_snapshot(&self) -> DomainMetricsSnapshot {
        let oracle_total = self.oracle_runs_total.load(Ordering::Relaxed);
        let oracle_latency_total = self.oracle_run_latency_us_total.load(Ordering::Relaxed);
        let events_total = self.events_appended.load(Ordering::Relaxed);
        let event_latency_total = self.event_append_latency_us_total.load(Ordering::Relaxed);

        DomainMetricsSnapshot {
            loops_created: self.loops_created.load(Ordering::Relaxed),
            iterations_started: self.iterations_started.load(Ordering::Relaxed),
            iterations_completed: self.iterations_completed.load(Ordering::Relaxed),
            candidates_registered: self.candidates_registered.load(Ordering::Relaxed),
            oracle_runs_total: oracle_total,
            oracle_runs_passed: self.oracle_runs_passed.load(Ordering::Relaxed),
            oracle_runs_failed: self.oracle_runs_failed.load(Ordering::Relaxed),
            oracle_avg_latency_ms: if oracle_total > 0 {
                (oracle_latency_total / oracle_total) as f64 / 1000.0
            } else {
                0.0
            },
            events_appended: events_total,
            event_append_avg_latency_ms: if events_total > 0 {
                (event_latency_total / events_total) as f64 / 1000.0
            } else {
                0.0
            },
        }
    }
}

/// Snapshot of HTTP metrics for JSON response
#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_client_error: u64,
    pub requests_server_error: u64,
    pub avg_latency_ms: f64,
}

/// Snapshot of domain metrics for JSON response (V11-3)
#[derive(Debug, Serialize)]
pub struct DomainMetricsSnapshot {
    pub loops_created: u64,
    pub iterations_started: u64,
    pub iterations_completed: u64,
    pub candidates_registered: u64,
    pub oracle_runs_total: u64,
    pub oracle_runs_passed: u64,
    pub oracle_runs_failed: u64,
    pub oracle_avg_latency_ms: f64,
    pub events_appended: u64,
    pub event_append_avg_latency_ms: f64,
}

/// Full metrics response
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub service: &'static str,
    pub version: &'static str,
    pub uptime_seconds: u64,
    pub timestamp: String,
    pub http: MetricsSnapshot,
    pub domain: DomainMetricsSnapshot,
}

/// Middleware to inject request context and trace spans
pub async fn request_context_middleware(mut request: Request, next: Next) -> Response {
    let start = Instant::now();

    // Extract or generate request context
    let ctx = RequestContext::from_headers(request.headers());

    // Create span for this request
    let span = info_span!(
        "http_request",
        request_id = %ctx.request_id,
        correlation_id = %ctx.correlation_id,
        method = %request.method(),
        uri = %request.uri(),
        otel.name = %format!("{} {}", request.method(), request.uri().path()),
    );

    // Store context in request extensions
    request.extensions_mut().insert(ctx.clone());

    // Execute the rest of the middleware stack with tracing
    let response = next.run(request).instrument(span.clone()).await;

    // Log completion with status and latency
    let latency = start.elapsed();
    let status = response.status();

    span.in_scope(|| {
        info!(
            status = %status.as_u16(),
            latency_ms = %latency.as_millis(),
            "Request completed"
        );
    });

    // Add request ID to response headers
    let mut response = response;
    if let Ok(header_value) = HeaderValue::from_str(&ctx.request_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static(REQUEST_ID_HEADER), header_value);
    }

    response
}

/// Middleware to record metrics
pub async fn metrics_middleware(
    State(metrics): State<Arc<Metrics>>,
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let response = next.run(request).await;
    let latency_us = start.elapsed().as_micros() as u64;

    metrics.record_request(response.status(), latency_us);

    response
}

/// Metrics endpoint handler
pub async fn metrics_handler(
    State(metrics): State<Arc<Metrics>>,
    State(start_time): State<Instant>,
) -> Json<MetricsResponse> {
    let uptime = start_time.elapsed().as_secs();

    Json(MetricsResponse {
        service: "solver-ralph-api",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: uptime,
        timestamp: Utc::now().to_rfc3339(),
        http: metrics.snapshot(),
        domain: metrics.domain_snapshot(),
    })
}

/// Combined metrics state for endpoint
#[derive(Clone)]
pub struct MetricsState {
    pub metrics: Arc<Metrics>,
    pub start_time: Instant,
}

/// Metrics endpoint using combined state
pub async fn metrics_endpoint(State(state): State<MetricsState>) -> Json<MetricsResponse> {
    let uptime = state.start_time.elapsed().as_secs();

    Json(MetricsResponse {
        service: "solver-ralph-api",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: uptime,
        timestamp: Utc::now().to_rfc3339(),
        http: state.metrics.snapshot(),
        domain: state.metrics.domain_snapshot(),
    })
}

// ============================================================================
// Readiness Endpoint (V11-3)
// ============================================================================

/// Individual dependency check result
#[derive(Debug, Serialize, Clone)]
pub struct DependencyCheck {
    pub name: &'static str,
    pub status: &'static str,
    pub latency_ms: Option<f64>,
    pub error: Option<String>,
}

/// Readiness response
#[derive(Debug, Serialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub timestamp: String,
    pub checks: Vec<DependencyCheck>,
}

/// State for readiness checks (V11-3)
#[derive(Clone)]
pub struct ReadinessState {
    pub db_pool: sqlx::PgPool,
    pub minio_endpoint: String,
    pub minio_bucket: String,
    pub nats_url: String,
}

/// Check PostgreSQL connectivity
async fn check_postgres(pool: &sqlx::PgPool) -> DependencyCheck {
    let start = Instant::now();
    match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => DependencyCheck {
            name: "postgresql",
            status: "healthy",
            latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
            error: None,
        },
        Err(e) => DependencyCheck {
            name: "postgresql",
            status: "unhealthy",
            latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
            error: Some(e.to_string()),
        },
    }
}

/// Check MinIO connectivity
async fn check_minio(endpoint: &str, _bucket: &str) -> DependencyCheck {
    let start = Instant::now();
    let health_url = format!("{}/minio/health/live", endpoint.trim_end_matches('/'));

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    match client {
        Ok(client) => match client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => DependencyCheck {
                name: "minio",
                status: "healthy",
                latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                error: None,
            },
            Ok(resp) => DependencyCheck {
                name: "minio",
                status: "unhealthy",
                latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                error: Some(format!("HTTP {}", resp.status())),
            },
            Err(e) => DependencyCheck {
                name: "minio",
                status: "unhealthy",
                latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                error: Some(e.to_string()),
            },
        },
        Err(e) => DependencyCheck {
            name: "minio",
            status: "unhealthy",
            latency_ms: None,
            error: Some(e.to_string()),
        },
    }
}

/// Check NATS connectivity
async fn check_nats(nats_url: &str) -> DependencyCheck {
    let start = Instant::now();

    // Extract host:port from nats:// URL and check HTTP monitoring endpoint
    let monitoring_url = nats_url
        .replace("nats://", "http://")
        .replace(":4222", ":8222");
    let health_url = format!("{}/healthz", monitoring_url.trim_end_matches('/'));

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    match client {
        Ok(client) => match client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => DependencyCheck {
                name: "nats",
                status: "healthy",
                latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                error: None,
            },
            Ok(resp) => DependencyCheck {
                name: "nats",
                status: "unhealthy",
                latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                error: Some(format!("HTTP {}", resp.status())),
            },
            Err(e) => DependencyCheck {
                name: "nats",
                status: "unhealthy",
                latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
                error: Some(e.to_string()),
            },
        },
        Err(e) => DependencyCheck {
            name: "nats",
            status: "unhealthy",
            latency_ms: None,
            error: Some(e.to_string()),
        },
    }
}

/// Readiness endpoint handler (V11-3)
///
/// Returns 200 if all dependencies are healthy, 503 otherwise.
/// Checks: PostgreSQL, MinIO, NATS
pub async fn ready_endpoint(
    State(state): State<ReadinessState>,
) -> (StatusCode, Json<ReadinessResponse>) {
    // Run all checks concurrently
    let (pg_check, minio_check, nats_check) = tokio::join!(
        check_postgres(&state.db_pool),
        check_minio(&state.minio_endpoint, &state.minio_bucket),
        check_nats(&state.nats_url),
    );

    let checks = vec![pg_check.clone(), minio_check.clone(), nats_check.clone()];
    let all_healthy = checks.iter().all(|c| c.status == "healthy");

    let response = ReadinessResponse {
        ready: all_healthy,
        timestamp: Utc::now().to_rfc3339(),
        checks,
    };

    let status = if all_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, Json(response))
}

/// Extract request context from request extensions
pub fn get_request_context(extensions: &axum::http::Extensions) -> Option<&RequestContext> {
    extensions.get::<RequestContext>()
}

/// Extractor for request context in handlers
#[derive(Clone, Debug)]
pub struct ExtractRequestContext(pub RequestContext);

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for ExtractRequestContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let ctx = parts
            .extensions
            .get::<RequestContext>()
            .cloned()
            .unwrap_or_else(RequestContext::new);

        Ok(ExtractRequestContext(ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn test_request_id_generation() {
        let id = generate_request_id();
        assert!(id.starts_with("req_"));
        assert!(id.len() > 4);
    }

    #[test]
    fn test_request_context_from_headers_empty() {
        let headers = HeaderMap::new();
        let ctx = RequestContext::from_headers(&headers);

        assert!(ctx.request_id.starts_with("req_"));
        assert_eq!(ctx.request_id, ctx.correlation_id);
    }

    #[test]
    fn test_request_context_from_headers_with_request_id() {
        let mut headers = HeaderMap::new();
        headers.insert(REQUEST_ID_HEADER, "my-request-123".parse().unwrap());

        let ctx = RequestContext::from_headers(&headers);

        assert_eq!(ctx.request_id, "my-request-123");
        assert_eq!(ctx.correlation_id, "my-request-123");
    }

    #[test]
    fn test_request_context_from_headers_with_correlation_id() {
        let mut headers = HeaderMap::new();
        headers.insert(REQUEST_ID_HEADER, "req-123".parse().unwrap());
        headers.insert(CORRELATION_ID_HEADER, "corr-456".parse().unwrap());

        let ctx = RequestContext::from_headers(&headers);

        assert_eq!(ctx.request_id, "req-123");
        assert_eq!(ctx.correlation_id, "corr-456");
    }

    #[test]
    fn test_metrics_record_request() {
        let metrics = Metrics::new();

        metrics.record_request(StatusCode::OK, 1000);
        metrics.record_request(StatusCode::CREATED, 2000);
        metrics.record_request(StatusCode::NOT_FOUND, 500);
        metrics.record_request(StatusCode::INTERNAL_SERVER_ERROR, 3000);

        let snapshot = metrics.snapshot();

        assert_eq!(snapshot.requests_total, 4);
        assert_eq!(snapshot.requests_success, 2);
        assert_eq!(snapshot.requests_client_error, 1);
        assert_eq!(snapshot.requests_server_error, 1);
    }

    #[test]
    fn test_metrics_avg_latency() {
        let metrics = Metrics::new();

        // 1000us + 2000us + 3000us = 6000us total, 3 requests = 2000us avg = 2ms
        metrics.record_request(StatusCode::OK, 1000);
        metrics.record_request(StatusCode::OK, 2000);
        metrics.record_request(StatusCode::OK, 3000);

        let snapshot = metrics.snapshot();

        assert_eq!(snapshot.requests_total, 3);
        assert!((snapshot.avg_latency_ms - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_metrics_snapshot_empty() {
        let metrics = Metrics::new();
        let snapshot = metrics.snapshot();

        assert_eq!(snapshot.requests_total, 0);
        assert_eq!(snapshot.avg_latency_ms, 0.0);
    }

    // V11-3: Domain metrics tests

    #[test]
    fn test_domain_metrics_loops() {
        let metrics = Metrics::new();

        metrics.record_loop_created();
        metrics.record_loop_created();
        metrics.record_loop_created();

        let snapshot = metrics.domain_snapshot();
        assert_eq!(snapshot.loops_created, 3);
    }

    #[test]
    fn test_domain_metrics_iterations() {
        let metrics = Metrics::new();

        metrics.record_iteration_started();
        metrics.record_iteration_started();
        metrics.record_iteration_completed();

        let snapshot = metrics.domain_snapshot();
        assert_eq!(snapshot.iterations_started, 2);
        assert_eq!(snapshot.iterations_completed, 1);
    }

    #[test]
    fn test_domain_metrics_candidates() {
        let metrics = Metrics::new();

        metrics.record_candidate_registered();
        metrics.record_candidate_registered();

        let snapshot = metrics.domain_snapshot();
        assert_eq!(snapshot.candidates_registered, 2);
    }

    #[test]
    fn test_domain_metrics_oracle_runs() {
        let metrics = Metrics::new();

        // Record 3 oracle runs: 2 passed, 1 failed
        metrics.record_oracle_run(true, 1000);  // 1ms
        metrics.record_oracle_run(true, 2000);  // 2ms
        metrics.record_oracle_run(false, 3000); // 3ms

        let snapshot = metrics.domain_snapshot();
        assert_eq!(snapshot.oracle_runs_total, 3);
        assert_eq!(snapshot.oracle_runs_passed, 2);
        assert_eq!(snapshot.oracle_runs_failed, 1);
        // Average: (1000 + 2000 + 3000) / 3 / 1000 = 2.0 ms
        assert!((snapshot.oracle_avg_latency_ms - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_domain_metrics_event_append() {
        let metrics = Metrics::new();

        metrics.record_event_append(500);  // 0.5ms
        metrics.record_event_append(1500); // 1.5ms

        let snapshot = metrics.domain_snapshot();
        assert_eq!(snapshot.events_appended, 2);
        // Average: (500 + 1500) / 2 / 1000 = 1.0 ms
        assert!((snapshot.event_append_avg_latency_ms - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_domain_metrics_snapshot_empty() {
        let metrics = Metrics::new();
        let snapshot = metrics.domain_snapshot();

        assert_eq!(snapshot.loops_created, 0);
        assert_eq!(snapshot.iterations_started, 0);
        assert_eq!(snapshot.iterations_completed, 0);
        assert_eq!(snapshot.candidates_registered, 0);
        assert_eq!(snapshot.oracle_runs_total, 0);
        assert_eq!(snapshot.oracle_runs_passed, 0);
        assert_eq!(snapshot.oracle_runs_failed, 0);
        assert_eq!(snapshot.oracle_avg_latency_ms, 0.0);
        assert_eq!(snapshot.events_appended, 0);
        assert_eq!(snapshot.event_append_avg_latency_ms, 0.0);
    }

    #[test]
    fn test_dependency_check_struct() {
        let check = DependencyCheck {
            name: "test-service",
            status: "healthy",
            latency_ms: Some(5.5),
            error: None,
        };

        assert_eq!(check.name, "test-service");
        assert_eq!(check.status, "healthy");
        assert_eq!(check.latency_ms, Some(5.5));
        assert!(check.error.is_none());
    }

    #[test]
    fn test_readiness_response_all_healthy() {
        let checks = vec![
            DependencyCheck {
                name: "postgresql",
                status: "healthy",
                latency_ms: Some(1.0),
                error: None,
            },
            DependencyCheck {
                name: "minio",
                status: "healthy",
                latency_ms: Some(2.0),
                error: None,
            },
            DependencyCheck {
                name: "nats",
                status: "healthy",
                latency_ms: Some(3.0),
                error: None,
            },
        ];

        let all_healthy = checks.iter().all(|c| c.status == "healthy");
        assert!(all_healthy);
    }

    #[test]
    fn test_readiness_response_one_unhealthy() {
        let checks = vec![
            DependencyCheck {
                name: "postgresql",
                status: "healthy",
                latency_ms: Some(1.0),
                error: None,
            },
            DependencyCheck {
                name: "minio",
                status: "unhealthy",
                latency_ms: Some(5000.0),
                error: Some("Connection refused".to_string()),
            },
            DependencyCheck {
                name: "nats",
                status: "healthy",
                latency_ms: Some(3.0),
                error: None,
            },
        ];

        let all_healthy = checks.iter().all(|c| c.status == "healthy");
        assert!(!all_healthy);
    }
}
