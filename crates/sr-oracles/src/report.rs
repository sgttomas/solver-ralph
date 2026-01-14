//! Oracle report structures for integration and E2E tests
//!
//! Per SR-PLAN D-26:
//! - Evidence Bundle with an e2e run transcript and artifacts
//!
//! This module provides structured report formats that are:
//! - Deterministic (sorted keys, consistent timestamps)
//! - Machine-readable (JSON)
//! - Compatible with Evidence Bundle manifests

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::flake_control::FlakeControl;

// ============================================================================
// Integration Test Report
// ============================================================================

/// Report from integration oracle tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationReport {
    /// Oracle ID
    pub oracle_id: String,
    /// Overall status (pass/fail/error)
    pub status: OracleStatus,
    /// Tests grouped by service
    pub services: BTreeMap<String, ServiceTestResult>,
    /// Total tests run
    pub tests_run: usize,
    /// Tests passed
    pub tests_passed: usize,
    /// Tests failed
    pub tests_failed: usize,
    /// Tests errored
    pub tests_errored: usize,
    /// Test duration in milliseconds
    pub duration_ms: u64,
    /// Flake control record
    pub flake_control: FlakeControl,
    /// Environment fingerprint
    pub environment: EnvironmentInfo,
    /// Report timestamp
    pub timestamp: DateTime<Utc>,
    /// Report content hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

impl IntegrationReport {
    /// Create a new integration report
    pub fn new(oracle_id: &str) -> Self {
        Self {
            oracle_id: oracle_id.to_string(),
            status: OracleStatus::Pending,
            services: BTreeMap::new(),
            tests_run: 0,
            tests_passed: 0,
            tests_failed: 0,
            tests_errored: 0,
            duration_ms: 0,
            flake_control: FlakeControl::default(),
            environment: EnvironmentInfo::default(),
            timestamp: Utc::now(),
            content_hash: None,
        }
    }

    /// Add a service test result
    pub fn add_service_result(&mut self, name: &str, result: ServiceTestResult) {
        self.tests_run += result.tests.len();
        self.tests_passed += result.tests.iter().filter(|t| t.passed).count();
        // Failed: test ran but didn't pass (may have error message explaining failure)
        self.tests_failed += result
            .tests
            .iter()
            .filter(|t| !t.passed && t.duration_ms > 0)
            .count();
        // Errored: test couldn't run (error + no duration means infrastructure issue)
        self.tests_errored += result
            .tests
            .iter()
            .filter(|t| !t.passed && t.duration_ms == 0 && t.error.is_some())
            .count();
        self.services.insert(name.to_string(), result);
    }

    /// Compute overall status from test results
    pub fn compute_status(&mut self) {
        if self.tests_errored > 0 {
            self.status = OracleStatus::Error;
        } else if self.tests_failed > 0 {
            self.status = OracleStatus::Fail;
        } else if self.tests_passed == self.tests_run && self.tests_run > 0 {
            self.status = OracleStatus::Pass;
        } else {
            self.status = OracleStatus::Error;
        }
    }

    /// Compute and set content hash
    pub fn compute_content_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.oracle_id.as_bytes());
        hasher.update(self.status.as_str().as_bytes());
        hasher.update(&self.tests_run.to_le_bytes());
        hasher.update(&self.tests_passed.to_le_bytes());
        hasher.update(&self.tests_failed.to_le_bytes());
        hasher.update(&self.tests_errored.to_le_bytes());

        for (name, result) in &self.services {
            hasher.update(name.as_bytes());
            for test in &result.tests {
                hasher.update(test.name.as_bytes());
                hasher.update(if test.passed { b"1" } else { b"0" });
            }
        }

        self.content_hash = Some(format!("sha256:{}", hex::encode(hasher.finalize())));
    }

    /// Finalize the report
    pub fn finalize(&mut self) {
        self.compute_status();
        self.compute_content_hash();
        self.timestamp = Utc::now();
    }
}

// ============================================================================
// E2E Test Report
// ============================================================================

/// Report from E2E oracle tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2EReport {
    /// Oracle ID
    pub oracle_id: String,
    /// Overall status (pass/fail/error)
    pub status: OracleStatus,
    /// Test scenarios executed
    pub scenarios: Vec<ScenarioResult>,
    /// Total scenarios run
    pub scenarios_run: usize,
    /// Scenarios passed
    pub scenarios_passed: usize,
    /// Scenarios failed
    pub scenarios_failed: usize,
    /// Total test duration in milliseconds
    pub duration_ms: u64,
    /// Flake control record
    pub flake_control: FlakeControl,
    /// Invariants checked
    pub invariants: Vec<InvariantCheck>,
    /// Produced entity IDs
    pub produced_ids: BTreeMap<String, Vec<String>>,
    /// Environment fingerprint
    pub environment: EnvironmentInfo,
    /// Report timestamp
    pub timestamp: DateTime<Utc>,
    /// Report content hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

impl E2EReport {
    /// Create a new E2E report
    pub fn new(oracle_id: &str) -> Self {
        Self {
            oracle_id: oracle_id.to_string(),
            status: OracleStatus::Pending,
            scenarios: Vec::new(),
            scenarios_run: 0,
            scenarios_passed: 0,
            scenarios_failed: 0,
            duration_ms: 0,
            flake_control: FlakeControl::default(),
            invariants: Vec::new(),
            produced_ids: BTreeMap::new(),
            environment: EnvironmentInfo::default(),
            timestamp: Utc::now(),
            content_hash: None,
        }
    }

    /// Add a scenario result
    pub fn add_scenario(&mut self, result: ScenarioResult) {
        self.scenarios_run += 1;
        if result.passed {
            self.scenarios_passed += 1;
        } else {
            self.scenarios_failed += 1;
        }
        self.scenarios.push(result);
    }

    /// Add an invariant check result
    pub fn add_invariant(&mut self, check: InvariantCheck) {
        self.invariants.push(check);
    }

    /// Record a produced entity ID
    pub fn record_id(&mut self, entity_type: &str, id: &str) {
        self.produced_ids
            .entry(entity_type.to_string())
            .or_default()
            .push(id.to_string());
    }

    /// Compute overall status from scenario results
    pub fn compute_status(&mut self) {
        let invariant_failures = self.invariants.iter().filter(|i| !i.passed).count();

        if invariant_failures > 0 {
            self.status = OracleStatus::Fail;
        } else if self.scenarios_failed > 0 {
            self.status = OracleStatus::Fail;
        } else if self.scenarios.iter().any(|s| s.error.is_some()) {
            self.status = OracleStatus::Error;
        } else if self.scenarios_passed == self.scenarios_run && self.scenarios_run > 0 {
            self.status = OracleStatus::Pass;
        } else {
            self.status = OracleStatus::Error;
        }
    }

    /// Compute and set content hash
    pub fn compute_content_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.oracle_id.as_bytes());
        hasher.update(self.status.as_str().as_bytes());
        hasher.update(&self.scenarios_run.to_le_bytes());
        hasher.update(&self.scenarios_passed.to_le_bytes());
        hasher.update(&self.scenarios_failed.to_le_bytes());

        for scenario in &self.scenarios {
            hasher.update(scenario.name.as_bytes());
            hasher.update(if scenario.passed { b"1" } else { b"0" });
        }

        for invariant in &self.invariants {
            hasher.update(invariant.name.as_bytes());
            hasher.update(if invariant.passed { b"1" } else { b"0" });
        }

        self.content_hash = Some(format!("sha256:{}", hex::encode(hasher.finalize())));
    }

    /// Finalize the report
    pub fn finalize(&mut self) {
        self.compute_status();
        self.compute_content_hash();
        self.timestamp = Utc::now();
    }
}

// ============================================================================
// Shared Types
// ============================================================================

/// Oracle execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OracleStatus {
    /// Not yet executed
    Pending,
    /// All tests passed
    Pass,
    /// One or more tests failed
    Fail,
    /// Execution error occurred
    Error,
    /// Tests were skipped
    Skipped,
}

impl OracleStatus {
    /// Get string representation
    pub fn as_str(&self) -> &str {
        match self {
            OracleStatus::Pending => "pending",
            OracleStatus::Pass => "pass",
            OracleStatus::Fail => "fail",
            OracleStatus::Error => "error",
            OracleStatus::Skipped => "skipped",
        }
    }
}

/// Test results for a single service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTestResult {
    /// Service name (e.g., "postgres", "minio", "nats", "api")
    pub service: String,
    /// Service endpoint URL
    pub endpoint: String,
    /// Whether the service is reachable
    pub reachable: bool,
    /// Individual test results
    pub tests: Vec<TestResult>,
    /// Service health check result
    pub health_status: Option<HealthStatus>,
    /// Connection latency in milliseconds
    pub latency_ms: Option<u64>,
}

impl ServiceTestResult {
    /// Create a new service test result
    pub fn new(service: &str, endpoint: &str) -> Self {
        Self {
            service: service.to_string(),
            endpoint: endpoint.to_string(),
            reachable: false,
            tests: Vec::new(),
            health_status: None,
            latency_ms: None,
        }
    }

    /// Add a test result
    pub fn add_test(&mut self, test: TestResult) {
        self.tests.push(test);
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.reachable && self.tests.iter().all(|t| t.passed)
    }
}

/// Individual test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test name
    pub name: String,
    /// Test description
    pub description: String,
    /// Whether the test passed
    pub passed: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Test duration in milliseconds
    pub duration_ms: u64,
    /// Additional test output
    #[serde(default)]
    pub output: BTreeMap<String, serde_json::Value>,
}

impl TestResult {
    /// Create a passed test result
    pub fn pass(name: &str, description: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passed: true,
            error: None,
            duration_ms,
            output: BTreeMap::new(),
        }
    }

    /// Create a failed test result
    pub fn fail(name: &str, description: &str, error: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passed: false,
            error: Some(error.to_string()),
            duration_ms,
            output: BTreeMap::new(),
        }
    }

    /// Create an errored test result
    pub fn error(name: &str, description: &str, error: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passed: false,
            error: Some(error.to_string()),
            duration_ms: 0,
            output: BTreeMap::new(),
        }
    }
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall health
    pub healthy: bool,
    /// Health check details
    pub details: BTreeMap<String, serde_json::Value>,
}

/// E2E scenario result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Whether scenario passed
    pub passed: bool,
    /// Error if failed
    pub error: Option<String>,
    /// Steps executed
    pub steps: Vec<StepResult>,
    /// Scenario duration in milliseconds
    pub duration_ms: u64,
}

impl ScenarioResult {
    /// Create a new scenario result
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passed: false,
            error: None,
            steps: Vec::new(),
            duration_ms: 0,
        }
    }

    /// Add a step result
    pub fn add_step(&mut self, step: StepResult) {
        self.steps.push(step);
    }

    /// Mark scenario as passed
    pub fn mark_passed(&mut self, duration_ms: u64) {
        self.passed = true;
        self.duration_ms = duration_ms;
    }

    /// Mark scenario as failed
    pub fn mark_failed(&mut self, error: &str, duration_ms: u64) {
        self.passed = false;
        self.error = Some(error.to_string());
        self.duration_ms = duration_ms;
    }
}

/// E2E step result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step name
    pub name: String,
    /// Whether step succeeded
    pub success: bool,
    /// Error if failed
    pub error: Option<String>,
    /// Step duration in milliseconds
    pub duration_ms: u64,
    /// Produced IDs
    #[serde(default)]
    pub produced_ids: BTreeMap<String, String>,
}

/// Invariant check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantCheck {
    /// Invariant name
    pub name: String,
    /// Invariant description
    pub description: String,
    /// Whether invariant holds
    pub passed: bool,
    /// Failure message if violated
    pub message: Option<String>,
}

impl InvariantCheck {
    /// Create a passed invariant check
    pub fn passed(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passed: true,
            message: None,
        }
    }

    /// Create a failed invariant check
    pub fn failed(name: &str, description: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passed: false,
            message: Some(message.to_string()),
        }
    }
}

/// Environment information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Host information
    pub host: Option<String>,
    /// Platform (os + arch)
    pub platform: Option<String>,
    /// Docker compose stack version
    pub stack_version: Option<String>,
    /// Service versions
    #[serde(default)]
    pub service_versions: BTreeMap<String, String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_report_creation() {
        let report = IntegrationReport::new("oracle:integration");
        assert_eq!(report.oracle_id, "oracle:integration");
        assert_eq!(report.status, OracleStatus::Pending);
        assert_eq!(report.tests_run, 0);
    }

    #[test]
    fn test_integration_report_add_service() {
        let mut report = IntegrationReport::new("oracle:integration");

        let mut pg_result = ServiceTestResult::new("postgres", "localhost:5432");
        pg_result.reachable = true;
        pg_result.add_test(TestResult::pass("connect", "Connection test", 100));
        pg_result.add_test(TestResult::pass("query", "Query test", 50));

        report.add_service_result("postgres", pg_result);

        assert_eq!(report.tests_run, 2);
        assert_eq!(report.tests_passed, 2);
        assert!(report.services.contains_key("postgres"));
    }

    #[test]
    fn test_integration_report_status_computation() {
        let mut report = IntegrationReport::new("oracle:integration");

        let mut pg_result = ServiceTestResult::new("postgres", "localhost:5432");
        pg_result.reachable = true;
        pg_result.add_test(TestResult::pass("connect", "Connection test", 100));
        report.add_service_result("postgres", pg_result);

        report.compute_status();
        assert_eq!(report.status, OracleStatus::Pass);
    }

    #[test]
    fn test_integration_report_failed_status() {
        let mut report = IntegrationReport::new("oracle:integration");

        let mut pg_result = ServiceTestResult::new("postgres", "localhost:5432");
        pg_result.reachable = true;
        pg_result.add_test(TestResult::fail("connect", "Connection test", "timeout", 100));
        report.add_service_result("postgres", pg_result);

        report.compute_status();
        assert_eq!(report.status, OracleStatus::Fail);
    }

    #[test]
    fn test_e2e_report_creation() {
        let report = E2EReport::new("oracle:e2e");
        assert_eq!(report.oracle_id, "oracle:e2e");
        assert_eq!(report.scenarios_run, 0);
    }

    #[test]
    fn test_e2e_report_add_scenario() {
        let mut report = E2EReport::new("oracle:e2e");

        let mut scenario = ScenarioResult::new("happy_path", "Happy path E2E test");
        scenario.mark_passed(5000);
        report.add_scenario(scenario);

        assert_eq!(report.scenarios_run, 1);
        assert_eq!(report.scenarios_passed, 1);
    }

    #[test]
    fn test_e2e_report_invariant_failure() {
        let mut report = E2EReport::new("oracle:e2e");

        let mut scenario = ScenarioResult::new("happy_path", "Happy path");
        scenario.mark_passed(1000);
        report.add_scenario(scenario);

        report.add_invariant(InvariantCheck::failed(
            "no_approval_without_evidence",
            "Approvals must have evidence",
            "Found approval without evidence ref",
        ));

        report.compute_status();
        assert_eq!(report.status, OracleStatus::Fail);
    }

    #[test]
    fn test_report_content_hash() {
        let mut report1 = IntegrationReport::new("oracle:integration");
        let mut pg_result = ServiceTestResult::new("postgres", "localhost:5432");
        pg_result.reachable = true;
        pg_result.add_test(TestResult::pass("connect", "Connection test", 100));
        report1.add_service_result("postgres", pg_result);
        report1.finalize();

        let mut report2 = IntegrationReport::new("oracle:integration");
        let mut pg_result2 = ServiceTestResult::new("postgres", "localhost:5432");
        pg_result2.reachable = true;
        pg_result2.add_test(TestResult::pass("connect", "Connection test", 100));
        report2.add_service_result("postgres", pg_result2);
        report2.finalize();

        // Content hashes should be identical for identical test results
        assert_eq!(report1.content_hash, report2.content_hash);
    }

    #[test]
    fn test_oracle_status_serialization() {
        let status = OracleStatus::Pass;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"pass\"");

        let parsed: OracleStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, OracleStatus::Pass);
    }

    #[test]
    fn test_service_test_result_all_passed() {
        let mut result = ServiceTestResult::new("test", "localhost");
        result.reachable = true;
        result.add_test(TestResult::pass("t1", "test 1", 10));
        result.add_test(TestResult::pass("t2", "test 2", 20));

        assert!(result.all_passed());
    }

    #[test]
    fn test_service_test_result_not_all_passed() {
        let mut result = ServiceTestResult::new("test", "localhost");
        result.reachable = true;
        result.add_test(TestResult::pass("t1", "test 1", 10));
        result.add_test(TestResult::fail("t2", "test 2", "failed", 20));

        assert!(!result.all_passed());
    }
}
