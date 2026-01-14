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

/// Metrics collection for API observability
#[derive(Debug, Default)]
pub struct Metrics {
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
}

/// Snapshot of metrics for JSON response
#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_client_error: u64,
    pub requests_server_error: u64,
    pub avg_latency_ms: f64,
}

/// Full metrics response
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub service: &'static str,
    pub version: &'static str,
    pub uptime_seconds: u64,
    pub timestamp: String,
    pub http: MetricsSnapshot,
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
    })
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
}
