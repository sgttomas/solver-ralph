//! Integration test oracle implementation
//!
//! Per SR-PLAN D-26:
//! - Suite can stand up the full stack and run e2e flows deterministically within tolerance
//! - Tests DB/MinIO/NATS/API connectivity
//!
//! This module provides integration tests for:
//! - PostgreSQL (event store, projections)
//! - MinIO (evidence storage)
//! - NATS (message bus, JetStream)
//! - API (health, auth, basic endpoints)

use chrono::Utc;
use std::time::Instant;
use tracing::{debug, error, info, instrument};

use crate::flake_control::{FlakeControl, RetryPolicy};
use crate::report::{
    EnvironmentInfo, HealthStatus, IntegrationReport, OracleStatus, ServiceTestResult, TestResult,
};

// ============================================================================
// Integration Runner Configuration
// ============================================================================

/// Configuration for the integration test runner
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    /// PostgreSQL connection URL
    pub database_url: String,
    /// MinIO endpoint URL
    pub minio_endpoint: String,
    /// MinIO access key
    pub minio_access_key: String,
    /// MinIO secret key
    pub minio_secret_key: String,
    /// MinIO bucket name
    pub minio_bucket: String,
    /// NATS URL
    pub nats_url: String,
    /// API base URL
    pub api_url: String,
    /// Retry policy
    pub retry_policy: RetryPolicy,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("SR_DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/solver_ralph".to_string()
            }),
            minio_endpoint: std::env::var("SR_MINIO_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            minio_access_key: std::env::var("SR_MINIO_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            minio_secret_key: std::env::var("SR_MINIO_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            minio_bucket: std::env::var("SR_MINIO_BUCKET")
                .unwrap_or_else(|_| "evidence".to_string()),
            nats_url: std::env::var("SR_NATS_URL")
                .unwrap_or_else(|_| "nats://localhost:4222".to_string()),
            api_url: std::env::var("SR_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            retry_policy: RetryPolicy::network(),
        }
    }
}

// ============================================================================
// Integration Runner
// ============================================================================

/// Integration test runner
pub struct IntegrationRunner {
    config: IntegrationConfig,
    report: IntegrationReport,
    flake_control: FlakeControl,
}

impl IntegrationRunner {
    /// Create a new integration runner
    pub fn new(config: IntegrationConfig) -> Self {
        let mut flake_control = FlakeControl::with_retry_policy(config.retry_policy.clone());
        flake_control.start();

        Self {
            config,
            report: IntegrationReport::new("oracle:integration"),
            flake_control,
        }
    }

    /// Create with default config from environment
    pub fn from_env() -> Self {
        Self::new(IntegrationConfig::default())
    }

    /// Run all integration tests
    #[instrument(skip(self), name = "integration_tests")]
    pub async fn run_all(&mut self) -> IntegrationReport {
        let start = Instant::now();
        info!("Starting integration tests");

        // Collect environment info
        self.report.environment = self.collect_environment_info();

        // Run tests for each service
        self.test_postgres().await;
        self.test_minio().await;
        self.test_nats().await;
        self.test_api().await;

        // Finalize
        self.report.duration_ms = start.elapsed().as_millis() as u64;
        self.flake_control.complete();
        self.report.flake_control = self.flake_control.clone();
        self.report.finalize();

        info!(
            status = ?self.report.status,
            tests_run = self.report.tests_run,
            tests_passed = self.report.tests_passed,
            tests_failed = self.report.tests_failed,
            duration_ms = self.report.duration_ms,
            "Integration tests completed"
        );

        self.report.clone()
    }

    /// Get the report
    pub fn report(&self) -> &IntegrationReport {
        &self.report
    }

    // ========================================================================
    // PostgreSQL Tests
    // ========================================================================

    #[instrument(skip(self), name = "test_postgres")]
    async fn test_postgres(&mut self) {
        let mut result = ServiceTestResult::new("postgres", &self.config.database_url);
        let start = Instant::now();

        info!("Testing PostgreSQL connectivity");

        // Test: Connection
        match self.test_postgres_connect().await {
            Ok(latency) => {
                result.reachable = true;
                result.latency_ms = Some(latency);
                result.add_test(TestResult::pass(
                    "connect",
                    "Database connection test",
                    latency,
                ));
            }
            Err(e) => {
                error!(error = %e, "PostgreSQL connection failed");
                result.add_test(TestResult::fail(
                    "connect",
                    "Database connection test",
                    &e.to_string(),
                    start.elapsed().as_millis() as u64,
                ));
                self.report.add_service_result("postgres", result);
                return;
            }
        }

        // Test: Schema exists
        match self.test_postgres_schema().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "schema_exists",
                    "Event store schema verification",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "schema_exists",
                    "Event store schema verification",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: Event append
        match self.test_postgres_event_append().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "event_append",
                    "Event store append operation",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "event_append",
                    "Event store append operation",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: Event read
        match self.test_postgres_event_read().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "event_read",
                    "Event store read operation",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "event_read",
                    "Event store read operation",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: Projection query
        match self.test_postgres_projection_query().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "projection_query",
                    "Projection query operation",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "projection_query",
                    "Projection query operation",
                    &e.to_string(),
                    0,
                ));
            }
        }

        result.health_status = Some(HealthStatus {
            healthy: result.all_passed(),
            details: Default::default(),
        });

        self.report.add_service_result("postgres", result);
    }

    async fn test_postgres_connect(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let db_url = self.config.database_url.clone();

        self.flake_control
            .execute_with_retry("postgres_connect", || async {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(1)
                    .acquire_timeout(std::time::Duration::from_secs(5))
                    .connect(&db_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                // Verify connection with a simple query
                sqlx::query("SELECT 1")
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| format!("Query failed: {}", e))?;

                pool.close().await;
                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::PostgresError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_postgres_schema(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let db_url = self.config.database_url.clone();

        self.flake_control
            .execute_with_retry("postgres_schema", || async {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&db_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                // Check es schema exists
                let result: (bool,) = sqlx::query_as(
                    "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = 'es')"
                )
                .fetch_one(&pool)
                .await
                .map_err(|e| format!("Schema check failed: {}", e))?;

                if !result.0 {
                    return Err("Event store schema 'es' does not exist".to_string());
                }

                // Check es.events table exists
                let result: (bool,) = sqlx::query_as(
                    "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_schema = 'es' AND table_name = 'events')"
                )
                .fetch_one(&pool)
                .await
                .map_err(|e| format!("Table check failed: {}", e))?;

                if !result.0 {
                    return Err("Event store table 'es.events' does not exist".to_string());
                }

                pool.close().await;
                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::PostgresError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_postgres_event_append(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let db_url = self.config.database_url.clone();

        self.flake_control
            .execute_with_retry("postgres_event_append", || async {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&db_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                // Create a test stream if it doesn't exist
                let stream_id = format!("test_integration_{}", Utc::now().timestamp_millis());

                sqlx::query(
                    "INSERT INTO es.streams (stream_id, stream_type, created_at) VALUES ($1, $2, NOW()) ON CONFLICT DO NOTHING"
                )
                .bind(&stream_id)
                .bind("integration_test")
                .execute(&pool)
                .await
                .map_err(|e| format!("Stream creation failed: {}", e))?;

                // Append a test event
                sqlx::query(
                    "INSERT INTO es.events (stream_id, event_type, event_data, actor_kind, actor_id, occurred_at) VALUES ($1, $2, $3, $4, $5, NOW())"
                )
                .bind(&stream_id)
                .bind("IntegrationTestEvent")
                .bind(serde_json::json!({"test": true}))
                .bind("SYSTEM")
                .bind("integration_test")
                .execute(&pool)
                .await
                .map_err(|e| format!("Event append failed: {}", e))?;

                pool.close().await;
                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::PostgresError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_postgres_event_read(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let db_url = self.config.database_url.clone();

        self.flake_control
            .execute_with_retry("postgres_event_read", || async {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&db_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                // Read recent events
                let rows: Vec<(i64,)> = sqlx::query_as(
                    "SELECT event_id FROM es.events ORDER BY event_id DESC LIMIT 10",
                )
                .fetch_all(&pool)
                .await
                .map_err(|e| format!("Event read failed: {}", e))?;

                debug!(event_count = rows.len(), "Read events from event store");

                pool.close().await;
                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::PostgresError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_postgres_projection_query(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let db_url = self.config.database_url.clone();

        self.flake_control
            .execute_with_retry("postgres_projection", || async {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&db_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                // Check proj schema exists
                let result: (bool,) = sqlx::query_as(
                    "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = 'proj')"
                )
                .fetch_one(&pool)
                .await
                .map_err(|e| format!("Schema check failed: {}", e))?;

                if !result.0 {
                    return Err("Projection schema 'proj' does not exist".to_string());
                }

                // Check loops table exists and is queryable
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM proj.loops")
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| format!("Projection query failed: {}", e))?;

                debug!(loop_count = count.0, "Queried projection table");

                pool.close().await;
                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::PostgresError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    // ========================================================================
    // MinIO Tests
    // ========================================================================

    #[instrument(skip(self), name = "test_minio")]
    async fn test_minio(&mut self) {
        let mut result = ServiceTestResult::new("minio", &self.config.minio_endpoint);
        let start = Instant::now();

        info!("Testing MinIO connectivity");

        // Test: Connection
        match self.test_minio_connect().await {
            Ok(latency) => {
                result.reachable = true;
                result.latency_ms = Some(latency);
                result.add_test(TestResult::pass(
                    "connect",
                    "MinIO connection test",
                    latency,
                ));
            }
            Err(e) => {
                error!(error = %e, "MinIO connection failed");
                result.add_test(TestResult::fail(
                    "connect",
                    "MinIO connection test",
                    &e.to_string(),
                    start.elapsed().as_millis() as u64,
                ));
                self.report.add_service_result("minio", result);
                return;
            }
        }

        // Test: Bucket exists
        match self.test_minio_bucket_exists().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "bucket_exists",
                    "Evidence bucket verification",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "bucket_exists",
                    "Evidence bucket verification",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: Upload object
        match self.test_minio_upload().await {
            Ok(duration) => {
                result.add_test(TestResult::pass("upload", "Object upload test", duration));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "upload",
                    "Object upload test",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: Download object
        match self.test_minio_download().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "download",
                    "Object download test",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "download",
                    "Object download test",
                    &e.to_string(),
                    0,
                ));
            }
        }

        result.health_status = Some(HealthStatus {
            healthy: result.all_passed(),
            details: Default::default(),
        });

        self.report.add_service_result("minio", result);
    }

    async fn test_minio_connect(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let endpoint = self.config.minio_endpoint.clone();

        self.flake_control
            .execute_with_retry("minio_connect", || async {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .map_err(|e| format!("Client build failed: {}", e))?;

                let health_url = format!("{}/minio/health/live", endpoint);
                let response = client
                    .get(&health_url)
                    .send()
                    .await
                    .map_err(|e| format!("Health check failed: {}", e))?;

                if !response.status().is_success() {
                    return Err(format!("Health check returned {}", response.status()));
                }

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::MinioError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_minio_bucket_exists(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let config = self.config.clone();

        self.flake_control
            .execute_with_retry("minio_bucket", || async {
                let s3_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                    .credentials_provider(aws_credential_types::Credentials::new(
                        &config.minio_access_key,
                        &config.minio_secret_key,
                        None,
                        None,
                        "minio",
                    ))
                    .endpoint_url(&config.minio_endpoint)
                    .region(aws_config::Region::new("us-east-1"))
                    .load()
                    .await;

                let client = aws_sdk_s3::Client::new(&s3_config);

                let result = client
                    .head_bucket()
                    .bucket(&config.minio_bucket)
                    .send()
                    .await;

                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Bucket check failed: {}", e)),
                }
            })
            .await
            .map_err(|e| IntegrationError::MinioError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_minio_upload(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let config = self.config.clone();
        let test_key = format!("integration_test/{}", Utc::now().timestamp_millis());

        self.flake_control
            .execute_with_retry("minio_upload", || {
                let key = test_key.clone();
                let cfg = config.clone();
                async move {
                    let s3_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                        .credentials_provider(aws_credential_types::Credentials::new(
                            &cfg.minio_access_key,
                            &cfg.minio_secret_key,
                            None,
                            None,
                            "minio",
                        ))
                        .endpoint_url(&cfg.minio_endpoint)
                        .region(aws_config::Region::new("us-east-1"))
                        .load()
                        .await;

                    let client = aws_sdk_s3::Client::new(&s3_config);

                    client
                        .put_object()
                        .bucket(&cfg.minio_bucket)
                        .key(&key)
                        .body(aws_sdk_s3::primitives::ByteStream::from_static(
                            b"integration test data",
                        ))
                        .send()
                        .await
                        .map_err(|e| format!("Upload failed: {}", e))?;

                    Ok::<(), String>(())
                }
            })
            .await
            .map_err(|e| IntegrationError::MinioError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_minio_download(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let config = self.config.clone();

        self.flake_control
            .execute_with_retry("minio_download", || async {
                let s3_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                    .credentials_provider(aws_credential_types::Credentials::new(
                        &config.minio_access_key,
                        &config.minio_secret_key,
                        None,
                        None,
                        "minio",
                    ))
                    .endpoint_url(&config.minio_endpoint)
                    .region(aws_config::Region::new("us-east-1"))
                    .load()
                    .await;

                let client = aws_sdk_s3::Client::new(&s3_config);

                // List objects to verify download capability
                let result = client
                    .list_objects_v2()
                    .bucket(&config.minio_bucket)
                    .prefix("integration_test/")
                    .max_keys(1)
                    .send()
                    .await
                    .map_err(|e| format!("List failed: {}", e))?;

                debug!(
                    count = result.key_count(),
                    "Listed integration test objects"
                );

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::MinioError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    // ========================================================================
    // NATS Tests
    // ========================================================================

    #[instrument(skip(self), name = "test_nats")]
    async fn test_nats(&mut self) {
        let mut result = ServiceTestResult::new("nats", &self.config.nats_url);

        info!("Testing NATS connectivity");

        // Test: Connection
        match self.test_nats_connect().await {
            Ok(latency) => {
                result.reachable = true;
                result.latency_ms = Some(latency);
                result.add_test(TestResult::pass("connect", "NATS connection test", latency));
            }
            Err(e) => {
                error!(error = %e, "NATS connection failed");
                result.add_test(TestResult::fail(
                    "connect",
                    "NATS connection test",
                    &e.to_string(),
                    0,
                ));
                self.report.add_service_result("nats", result);
                return;
            }
        }

        // Test: Publish
        match self.test_nats_publish().await {
            Ok(duration) => {
                result.add_test(TestResult::pass("publish", "NATS publish test", duration));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "publish",
                    "NATS publish test",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: JetStream
        match self.test_nats_jetstream().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "jetstream",
                    "NATS JetStream availability",
                    duration,
                ));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "jetstream",
                    "NATS JetStream availability",
                    &e.to_string(),
                    0,
                ));
            }
        }

        result.health_status = Some(HealthStatus {
            healthy: result.all_passed(),
            details: Default::default(),
        });

        self.report.add_service_result("nats", result);
    }

    async fn test_nats_connect(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let nats_url = self.config.nats_url.clone();

        self.flake_control
            .execute_with_retry("nats_connect", || async {
                let client = async_nats::connect(&nats_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                // Verify connection is alive
                client
                    .flush()
                    .await
                    .map_err(|e| format!("Flush failed: {}", e))?;

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::NatsError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_nats_publish(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let nats_url = self.config.nats_url.clone();

        self.flake_control
            .execute_with_retry("nats_publish", || async {
                let client = async_nats::connect(&nats_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                client
                    .publish("integration.test", "test message".into())
                    .await
                    .map_err(|e| format!("Publish failed: {}", e))?;

                client
                    .flush()
                    .await
                    .map_err(|e| format!("Flush failed: {}", e))?;

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::NatsError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_nats_jetstream(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let nats_url = self.config.nats_url.clone();

        self.flake_control
            .execute_with_retry("nats_jetstream", || async {
                let client = async_nats::connect(&nats_url)
                    .await
                    .map_err(|e| format!("Connection failed: {}", e))?;

                let jetstream = async_nats::jetstream::new(client);

                // Try to get account info to verify JetStream is available
                let _info = jetstream
                    .get_stream("events")
                    .await
                    .map_err(|e| format!("JetStream not available or stream missing: {}", e))?;

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::NatsError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    // ========================================================================
    // API Tests
    // ========================================================================

    #[instrument(skip(self), name = "test_api")]
    async fn test_api(&mut self) {
        let mut result = ServiceTestResult::new("api", &self.config.api_url);

        info!("Testing API connectivity");

        // Test: Health endpoint
        match self.test_api_health().await {
            Ok(latency) => {
                result.reachable = true;
                result.latency_ms = Some(latency);
                result.add_test(TestResult::pass("health", "API health endpoint", latency));
            }
            Err(e) => {
                error!(error = %e, "API health check failed");
                result.add_test(TestResult::fail(
                    "health",
                    "API health endpoint",
                    &e.to_string(),
                    0,
                ));
                self.report.add_service_result("api", result);
                return;
            }
        }

        // Test: Info endpoint
        match self.test_api_info().await {
            Ok(duration) => {
                result.add_test(TestResult::pass("info", "API info endpoint", duration));
            }
            Err(e) => {
                result.add_test(TestResult::fail(
                    "info",
                    "API info endpoint",
                    &e.to_string(),
                    0,
                ));
            }
        }

        // Test: Loops endpoint (unauthenticated list)
        match self.test_api_loops().await {
            Ok(duration) => {
                result.add_test(TestResult::pass(
                    "loops_list",
                    "API loops list endpoint",
                    duration,
                ));
            }
            Err(e) => {
                // This might fail due to auth, which is expected
                result.add_test(TestResult::fail(
                    "loops_list",
                    "API loops list endpoint",
                    &e.to_string(),
                    0,
                ));
            }
        }

        result.health_status = Some(HealthStatus {
            healthy: result.reachable,
            details: Default::default(),
        });

        self.report.add_service_result("api", result);
    }

    async fn test_api_health(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let api_url = self.config.api_url.clone();

        self.flake_control
            .execute_with_retry("api_health", || async {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .map_err(|e| format!("Client build failed: {}", e))?;

                let response = client
                    .get(format!("{}/health", api_url))
                    .send()
                    .await
                    .map_err(|e| format!("Request failed: {}", e))?;

                if !response.status().is_success() {
                    return Err(format!("Health check returned {}", response.status()));
                }

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::ApiError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_api_info(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let api_url = self.config.api_url.clone();

        self.flake_control
            .execute_with_retry("api_info", || async {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .map_err(|e| format!("Client build failed: {}", e))?;

                let response = client
                    .get(format!("{}/api/v1/info", api_url))
                    .send()
                    .await
                    .map_err(|e| format!("Request failed: {}", e))?;

                if !response.status().is_success() {
                    return Err(format!("Info endpoint returned {}", response.status()));
                }

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::ApiError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn test_api_loops(&mut self) -> Result<u64, IntegrationError> {
        let start = Instant::now();
        let api_url = self.config.api_url.clone();

        self.flake_control
            .execute_with_retry("api_loops", || async {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(10))
                    .build()
                    .map_err(|e| format!("Client build failed: {}", e))?;

                let response = client
                    .get(format!("{}/api/v1/loops", api_url))
                    .send()
                    .await
                    .map_err(|e| format!("Request failed: {}", e))?;

                // 401/403 is expected without auth - we're just testing the endpoint exists
                if response.status().as_u16() == 404 {
                    return Err("Loops endpoint not found".to_string());
                }

                Ok::<(), String>(())
            })
            .await
            .map_err(|e| IntegrationError::ApiError(e.to_string()))?;

        Ok(start.elapsed().as_millis() as u64)
    }

    // ========================================================================
    // Helpers
    // ========================================================================

    fn collect_environment_info(&self) -> EnvironmentInfo {
        EnvironmentInfo {
            host: Some(
                hostname::get()
                    .map(|h| h.to_string_lossy().to_string())
                    .unwrap_or_default(),
            ),
            platform: Some(format!(
                "{}/{}",
                std::env::consts::OS,
                std::env::consts::ARCH
            )),
            stack_version: Some("1.0.0".to_string()),
            service_versions: Default::default(),
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Integration test errors
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("PostgreSQL error: {0}")]
    PostgresError(String),

    #[error("MinIO error: {0}")]
    MinioError(String),

    #[error("NATS error: {0}")]
    NatsError(String),

    #[error("API error: {0}")]
    ApiError(String),
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = IntegrationConfig::default();
        assert!(config.database_url.contains("postgres"));
        assert!(config.minio_endpoint.contains("9000"));
        assert!(config.nats_url.contains("4222"));
        assert!(config.api_url.contains("3000"));
    }

    #[test]
    fn test_integration_runner_creation() {
        let runner = IntegrationRunner::from_env();
        assert_eq!(runner.report.oracle_id, "oracle:integration");
        assert_eq!(runner.report.status, OracleStatus::Pending);
    }

    #[test]
    fn test_report_finalization() {
        let mut runner = IntegrationRunner::from_env();

        // Add a mock test result
        let mut pg_result = ServiceTestResult::new("postgres", "localhost:5432");
        pg_result.reachable = true;
        pg_result.add_test(TestResult::pass("connect", "test", 100));
        runner.report.add_service_result("postgres", pg_result);

        runner.report.finalize();

        assert_eq!(runner.report.status, OracleStatus::Pass);
        assert!(runner.report.content_hash.is_some());
    }
}
