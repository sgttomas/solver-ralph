---
doc_id: SR-OBSERVABILITY
doc_kind: governance.observability_guide
layer: platform
status: draft
refs:
- rel: depends_on
  to: SR-SPEC
- rel: depends_on
  to: SR-DEPLOYMENT
---

# SR-OBSERVABILITY: Observability Guide

**Purpose:** Document the observability infrastructure for SOLVER-Ralph, including metrics, health checks, logging, and tracing.

**Normative Status:** Informative (non-binding)

**Deliverable:** D-33 (Operational Observability)

---

## 1. Overview

SOLVER-Ralph provides comprehensive observability through:

1. **Health Endpoints** - Basic liveness and readiness checks
2. **Metrics Endpoint** - HTTP and domain metrics
3. **Structured Logging** - Request tracing with correlation IDs
4. **Request Context** - Distributed tracing support

---

## 2. Health Endpoints

### 2.1 Liveness (`/health`)

Basic health check that returns immediately.

**Request:**
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

**Use Case:** Kubernetes liveness probe, load balancer health check.

### 2.2 Readiness (`/ready`)

Dependency readiness check (V11-3). Verifies all dependencies are available.

**Request:**
```http
GET /ready
```

**Response (all healthy):**
```json
{
  "ready": true,
  "timestamp": "2026-01-17T12:00:00Z",
  "checks": [
    {
      "name": "postgresql",
      "status": "healthy",
      "latency_ms": 1.5,
      "error": null
    },
    {
      "name": "minio",
      "status": "healthy",
      "latency_ms": 3.2,
      "error": null
    },
    {
      "name": "nats",
      "status": "healthy",
      "latency_ms": 0.8,
      "error": null
    }
  ]
}
```

**Response (dependency down):**
```json
{
  "ready": false,
  "timestamp": "2026-01-17T12:00:00Z",
  "checks": [
    {
      "name": "postgresql",
      "status": "healthy",
      "latency_ms": 1.5,
      "error": null
    },
    {
      "name": "minio",
      "status": "unhealthy",
      "latency_ms": 5000.0,
      "error": "Connection refused"
    },
    {
      "name": "nats",
      "status": "healthy",
      "latency_ms": 0.8,
      "error": null
    }
  ]
}
```

**HTTP Status Codes:**
- `200 OK` - All dependencies healthy
- `503 Service Unavailable` - One or more dependencies unhealthy

**Use Case:** Kubernetes readiness probe, deployment verification.

---

## 3. Metrics Endpoint

### 3.1 Endpoint

**Request:**
```http
GET /api/v1/metrics
```

**Response:**
```json
{
  "service": "solver-ralph-api",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "timestamp": "2026-01-17T12:00:00Z",
  "http": {
    "requests_total": 10000,
    "requests_success": 9500,
    "requests_client_error": 400,
    "requests_server_error": 100,
    "avg_latency_ms": 25.5
  },
  "domain": {
    "loops_created": 50,
    "iterations_started": 250,
    "iterations_completed": 230,
    "candidates_registered": 180,
    "oracle_runs_total": 500,
    "oracle_runs_passed": 450,
    "oracle_runs_failed": 50,
    "oracle_avg_latency_ms": 150.0,
    "events_appended": 5000,
    "event_append_avg_latency_ms": 2.5
  }
}
```

### 3.2 HTTP Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `requests_total` | Counter | Total HTTP requests received |
| `requests_success` | Counter | Requests with 2xx status |
| `requests_client_error` | Counter | Requests with 4xx status |
| `requests_server_error` | Counter | Requests with 5xx status |
| `avg_latency_ms` | Gauge | Average request latency in milliseconds |

### 3.3 Domain Metrics (V11-3)

| Metric | Type | Description |
|--------|------|-------------|
| `loops_created` | Counter | Total loops created |
| `iterations_started` | Counter | Total iterations started |
| `iterations_completed` | Counter | Total iterations completed |
| `candidates_registered` | Counter | Total candidates registered |
| `oracle_runs_total` | Counter | Total oracle runs executed |
| `oracle_runs_passed` | Counter | Oracle runs with PASS verdict |
| `oracle_runs_failed` | Counter | Oracle runs with FAIL verdict |
| `oracle_avg_latency_ms` | Gauge | Average oracle run latency |
| `events_appended` | Counter | Total events appended to store |
| `event_append_avg_latency_ms` | Gauge | Average event append latency |

---

## 4. Structured Logging

### 4.1 Log Format

SOLVER-Ralph supports two log formats controlled by `SR_LOG_FORMAT`:

**Pretty (default, development):**
```
2026-01-17T12:00:00.000Z INFO sr_api::main Starting SOLVER-Ralph API host=0.0.0.0 port=3000 version=0.1.0
```

**JSON (production):**
```json
{
  "timestamp": "2026-01-17T12:00:00.000Z",
  "level": "INFO",
  "target": "sr_api::main",
  "message": "Starting SOLVER-Ralph API",
  "fields": {
    "host": "0.0.0.0",
    "port": 3000,
    "version": "0.1.0"
  }
}
```

### 4.2 Log Levels

| Level | Use Case |
|-------|----------|
| `trace` | Detailed debugging (very verbose) |
| `debug` | Development debugging |
| `info` | Normal operations |
| `warn` | Potential issues |
| `error` | Errors requiring attention |

Configure via `SR_LOG_LEVEL` environment variable.

### 4.3 Request Logging

Every HTTP request is logged with:
- Request ID
- Correlation ID
- HTTP method and path
- Response status
- Latency

Example:
```json
{
  "timestamp": "2026-01-17T12:00:00.000Z",
  "level": "INFO",
  "message": "Request completed",
  "span": {
    "request_id": "req_192837465abc",
    "correlation_id": "corr_192837465abc",
    "method": "POST",
    "uri": "/api/v1/loops"
  },
  "fields": {
    "status": 201,
    "latency_ms": 15
  }
}
```

---

## 5. Request Context

### 5.1 Headers

SOLVER-Ralph propagates and generates request context via headers:

| Header | Description | Generated If Missing |
|--------|-------------|----------------------|
| `x-request-id` | Unique request identifier | Yes |
| `x-correlation-id` | Distributed trace ID | Yes (copies request-id) |

### 5.2 Usage

Clients can provide their own IDs for distributed tracing:

```http
POST /api/v1/loops
x-request-id: my-req-123
x-correlation-id: my-trace-456
Content-Type: application/json

{...}
```

Response includes the request ID:
```http
HTTP/1.1 201 Created
x-request-id: my-req-123
Content-Type: application/json

{...}
```

---

## 6. Integration with Monitoring Systems

### 6.1 Prometheus (Future)

A Prometheus-format endpoint (`/metrics/prometheus`) is planned for future release. Current metrics can be scraped by converting the JSON response.

### 6.2 Grafana Dashboards

Recommended dashboard panels:
- Request rate (per minute)
- Error rate (4xx, 5xx)
- Latency percentiles (p50, p95, p99)
- Loop/iteration throughput
- Oracle pass/fail ratio
- Event append rate

### 6.3 Alerting Rules

Recommended alerts:

| Alert | Condition | Severity |
|-------|-----------|----------|
| High Error Rate | `requests_server_error / requests_total > 0.05` | Critical |
| High Latency | `avg_latency_ms > 1000` | Warning |
| Oracle Failures | `oracle_runs_failed / oracle_runs_total > 0.20` | Warning |
| Dependency Down | `/ready` returns 503 | Critical |

---

## 7. Troubleshooting

### 7.1 No Metrics Data

If metrics show all zeros:
1. Verify the API is receiving traffic
2. Check that endpoints are being called correctly
3. Ensure metrics middleware is registered

### 7.2 Readiness Always Failing

If `/ready` always returns 503:
1. Check individual dependency status in response
2. Verify network connectivity to dependencies
3. Check dependency container logs
4. Verify environment variables point to correct endpoints

### 7.3 Missing Logs

If logs are not appearing:
1. Check `SR_LOG_LEVEL` is set appropriately
2. Verify stdout/stderr is being captured
3. Check log aggregator configuration

---

## 8. Configuration Reference

| Variable | Description | Default |
|----------|-------------|---------|
| `SR_LOG_LEVEL` | Minimum log level | `debug` |
| `SR_LOG_FORMAT` | Output format (`pretty`, `json`) | `pretty` |
| `MINIO_ENDPOINT` | MinIO endpoint for readiness check | `http://localhost:9000` |
| `SR_NATS_URL` | NATS URL for readiness check | `nats://localhost:4222` |

---

## 9. Best Practices

### 9.1 Production Recommendations

1. **Use JSON logging** - Set `SR_LOG_FORMAT=json` for log aggregators
2. **Set appropriate log level** - Use `info` in production, `debug` in staging
3. **Monitor readiness** - Use `/ready` for orchestrator health checks
4. **Set up alerting** - Alert on error rates, latency, and dependency failures
5. **Correlate requests** - Pass `x-correlation-id` through service calls

### 9.2 Development Recommendations

1. **Use pretty logging** - Default format is human-readable
2. **Enable debug logging** - Set `SR_LOG_LEVEL=debug` for detailed output
3. **Check metrics regularly** - Monitor `/api/v1/metrics` during development
4. **Use request IDs** - Include in bug reports and support requests
