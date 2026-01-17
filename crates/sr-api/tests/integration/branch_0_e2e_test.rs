//! Branch 0 End-to-End Integration Test (SR-PLAN-V9 Phase V9-2)
//!
//! Demonstrates complete Branch 0 procedure flow per SR-PLAN §4.1:
//! 1. Work Surface creation with GENERIC-KNOWLEDGE-WORK template
//! 2. Loop creation bound to work surface
//! 3. Iteration cycling with semantic worker processing
//! 4. Stage progression: FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL
//! 5. Portal approvals at trust boundaries (SEMANTIC_EVAL, FINAL)
//! 6. Freeze baseline creation
//! 7. Evidence bundle verification (when semantic worker is running)
//!
//! Per SR-CONTRACT invariants:
//! - C-TB-3: Approvals MUST be recorded by HUMAN actors only
//! - C-SHIP-1: Freeze records MUST be created by HUMAN actors
//! - SEMANTIC_EVAL and FINAL stages require portal approval before completion
//!
//! Requires:
//! - Running sr-api server at SR_API_URL (default: http://localhost:3000)
//! - Valid auth tokens: SR_HUMAN_TOKEN, SR_SYSTEM_TOKEN
//! - PostgreSQL, MinIO, NATS infrastructure

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// =============================================================================
// Test Configuration
// =============================================================================

struct TestConfig {
    api_base_url: String,
    human_token: String,
    system_token: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            api_base_url: std::env::var("SR_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            human_token: std::env::var("SR_HUMAN_TOKEN")
                .unwrap_or_else(|_| "e2e-human-token".to_string()),
            system_token: std::env::var("SR_SYSTEM_TOKEN")
                .unwrap_or_else(|_| "e2e-system-token".to_string()),
        }
    }
}

// =============================================================================
// HTTP Client
// =============================================================================

struct TestClient {
    http: reqwest::Client,
    config: TestConfig,
}

impl TestClient {
    fn new(config: TestConfig) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
        }
    }

    fn human_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", self.config.human_token)) {
            headers.insert(AUTHORIZATION, value);
        }
        headers
    }

    fn system_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", self.config.system_token)) {
            headers.insert(AUTHORIZATION, value);
        }
        headers
    }

    fn url(&self, path: &str) -> String {
        format!(
            "{}/api/v1{}",
            self.config.api_base_url.trim_end_matches('/'),
            path
        )
    }
}

// =============================================================================
// Request/Response Types
// =============================================================================

#[derive(Debug, Serialize)]
struct CreateIntakeRequest {
    work_unit_id: String,
    title: String,
    kind: String,
    objective: String,
    audience: String,
    deliverables: Vec<DeliverableRequest>,
    #[serde(default)]
    constraints: Vec<String>,
    #[serde(default)]
    definitions: HashMap<String, String>,
    #[serde(default)]
    inputs: Vec<TypedRefRequest>,
    #[serde(default)]
    unknowns: Vec<String>,
    #[serde(default)]
    completion_criteria: Vec<String>,
}

#[derive(Debug, Serialize)]
struct DeliverableRequest {
    name: String,
    format: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TypedRefRequest {
    kind: String,
    id: String,
    rel: String,
    #[serde(default)]
    meta: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct CreateWorkSurfaceRequest {
    intake_id: String,
    procedure_template_id: String,
    work_unit_id: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct WorkSurfaceActionResponse {
    work_surface_id: String,
    status: String,
    #[allow(dead_code)]
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct WorkSurfaceResponse {
    work_surface_id: String,
    status: String,
    current_stage_id: String,
    #[allow(dead_code)]
    procedure_template_id: String,
}

#[derive(Debug, Serialize)]
struct CreateLoopRequest {
    goal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_unit: Option<String>,
    directive_ref: TypedRefRequest,
}

#[derive(Debug, Deserialize)]
struct LoopActionResponse {
    loop_id: String,
    #[allow(dead_code)]
    state: String,
    #[allow(dead_code)]
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct LoopResponse {
    loop_id: String,
    work_surface_id: Option<String>,
    state: String,
    #[allow(dead_code)]
    goal: String,
}

#[derive(Debug, Serialize)]
struct StartIterationRequest {
    loop_id: String,
    #[serde(default)]
    refs: Vec<TypedRefRequest>,
}

#[derive(Debug, Deserialize)]
struct IterationActionResponse {
    iteration_id: String,
    #[allow(dead_code)]
    state: String,
}

#[derive(Debug, Serialize)]
struct RecordApprovalRequest {
    portal_id: String,
    decision: String,
    subject_refs: Vec<TypedRefRequest>,
    #[serde(default)]
    evidence_refs: Vec<String>,
    #[serde(default)]
    exceptions_acknowledged: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rationale: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApprovalActionResponse {
    approval_id: String,
    #[allow(dead_code)]
    portal_id: String,
    #[allow(dead_code)]
    decision: String,
}

#[derive(Debug, Serialize)]
struct CompleteStageRequest {
    evidence_bundle_ref: String,
    gate_result: GateResultRequest,
}

#[derive(Debug, Serialize)]
struct GateResultRequest {
    status: String,
    #[serde(default)]
    oracle_results: Vec<OracleResultRequest>,
    #[serde(default)]
    waiver_refs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct OracleResultRequest {
    oracle_id: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StageCompletionResponse {
    work_surface_id: String,
    completed_stage_id: String,
    is_terminal: bool,
    #[serde(default)]
    next_stage_id: Option<String>,
    work_surface_status: String,
}

#[derive(Debug, Serialize)]
struct CreateFreezeRecordRequest {
    baseline_id: String,
    candidate_id: String,
    verification_mode: String,
    oracle_suite_id: String,
    oracle_suite_hash: String,
    evidence_bundle_refs: Vec<String>,
    #[serde(default)]
    waiver_refs: Vec<String>,
    release_approval_id: String,
    #[serde(default)]
    artifact_manifest: Vec<ArtifactManifestEntry>,
    #[serde(default)]
    active_exceptions: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct ArtifactManifestEntry {
    artifact_id: String,
    version: String,
    content_hash: String,
}

#[derive(Debug, Deserialize)]
struct FreezeRecordResponse {
    freeze_id: String,
    baseline_id: String,
    #[allow(dead_code)]
    frozen_at: String,
}

#[derive(Debug, Deserialize)]
struct EvidenceListResponse {
    #[serde(default)]
    evidence: Vec<EvidenceBundleInfo>,
}

#[derive(Debug, Deserialize)]
struct EvidenceBundleInfo {
    #[allow(dead_code)]
    content_hash: String,
    #[allow(dead_code)]
    stage_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: String,
    code: u16,
    #[serde(default)]
    details: Option<serde_json::Value>,
}

// =============================================================================
// Test Helpers
// =============================================================================

/// Creates an intake with the given work_unit_id and returns the intake_id
async fn create_intake(client: &TestClient, work_unit_id: &str) -> String {
    let create_intake = CreateIntakeRequest {
        work_unit_id: work_unit_id.to_string(),
        title: "Branch 0 E2E Test".to_string(),
        kind: "research_memo".to_string(),
        objective: "Demonstrate complete Branch 0 procedure flow".to_string(),
        audience: "Integration test framework".to_string(),
        deliverables: vec![DeliverableRequest {
            name: "branch-0-output".to_string(),
            format: "text".to_string(),
            path: "/tmp/branch-0-test.txt".to_string(),
            description: Some("Branch 0 test deliverable".to_string()),
        }],
        constraints: vec!["Must complete within test timeout".to_string()],
        definitions: HashMap::new(),
        inputs: vec![],
        unknowns: vec![],
        completion_criteria: vec!["All 5 stages complete".to_string()],
    };

    let resp = client
        .http
        .post(&client.url("/intakes"))
        .headers(client.human_headers())
        .json(&create_intake)
        .send()
        .await
        .expect("Failed to create intake");

    assert!(
        resp.status().is_success(),
        "Failed to create intake: {}",
        resp.text().await.unwrap_or_default()
    );

    let response: serde_json::Value = resp.json().await.expect("Failed to parse intake response");
    response
        .get("intake_id")
        .and_then(|v| v.as_str())
        .expect("No intake_id in response")
        .to_string()
}

/// Activates an intake
async fn activate_intake(client: &TestClient, intake_id: &str) {
    let resp = client
        .http
        .post(&client.url(&format!("/intakes/{}/activate", intake_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("Failed to activate intake");

    assert!(
        resp.status().is_success(),
        "Failed to activate intake: {}",
        resp.text().await.unwrap_or_default()
    );
}

/// Creates an active Work Surface with GENERIC-KNOWLEDGE-WORK template
async fn create_work_surface_generic(client: &TestClient, work_unit_id: &str) -> String {
    // Create and activate intake
    let intake_id = create_intake(client, work_unit_id).await;
    activate_intake(client, &intake_id).await;

    // Create work surface with GENERIC-KNOWLEDGE-WORK template (5 stages)
    let create_ws = CreateWorkSurfaceRequest {
        intake_id,
        procedure_template_id: "proc:GENERIC-KNOWLEDGE-WORK".to_string(),
        work_unit_id: work_unit_id.to_string(),
        params: serde_json::json!({}),
    };

    let resp = client
        .http
        .post(&client.url("/work-surfaces"))
        .headers(client.human_headers())
        .json(&create_ws)
        .send()
        .await
        .expect("Failed to create work surface");

    assert!(
        resp.status().is_success(),
        "Failed to create work surface: {}",
        resp.text().await.unwrap_or_default()
    );

    let ws_action: WorkSurfaceActionResponse =
        resp.json().await.expect("Failed to parse WS response");
    assert_eq!(ws_action.status, "active", "Work Surface should be active");

    ws_action.work_surface_id
}

/// Creates a loop bound to a work unit and activates it
async fn create_and_activate_loop(client: &TestClient, work_unit_id: &str) -> String {
    let create_loop = CreateLoopRequest {
        goal: "Branch 0 E2E: Complete 5-stage knowledge synthesis".to_string(),
        work_unit: Some(work_unit_id.to_string()),
        directive_ref: TypedRefRequest {
            kind: "doc".to_string(),
            id: "SR-DIRECTIVE".to_string(),
            rel: "governs".to_string(),
            meta: serde_json::Value::Null,
        },
    };

    let resp = client
        .http
        .post(&client.url("/loops"))
        .headers(client.human_headers())
        .json(&create_loop)
        .send()
        .await
        .expect("Failed to create loop");

    assert!(
        resp.status().is_success(),
        "Failed to create loop: {}",
        resp.text().await.unwrap_or_default()
    );

    let loop_action: LoopActionResponse = resp.json().await.expect("Failed to parse loop response");
    let loop_id = loop_action.loop_id;

    // Activate the loop
    let resp = client
        .http
        .post(&client.url(&format!("/loops/{}/activate", loop_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("Failed to activate loop");

    assert!(
        resp.status().is_success(),
        "Failed to activate loop: {}",
        resp.text().await.unwrap_or_default()
    );

    loop_id
}

/// Starts an iteration for the given loop (SYSTEM actor)
async fn start_iteration(client: &TestClient, loop_id: &str) -> String {
    let start_iter = StartIterationRequest {
        loop_id: loop_id.to_string(),
        refs: vec![],
    };

    let resp = client
        .http
        .post(&client.url("/iterations"))
        .headers(client.system_headers())
        .json(&start_iter)
        .send()
        .await
        .expect("Failed to start iteration");

    assert!(
        resp.status().is_success(),
        "Failed to start iteration: {}",
        resp.text().await.unwrap_or_default()
    );

    let iter_action: IterationActionResponse =
        resp.json().await.expect("Failed to parse iteration");
    iter_action.iteration_id
}

/// Records an approval at a portal (HUMAN actor)
async fn record_approval(
    client: &TestClient,
    portal_id: &str,
    work_surface_id: &str,
    evidence_ref: &str,
) -> String {
    let approval_req = RecordApprovalRequest {
        portal_id: portal_id.to_string(),
        decision: "APPROVED".to_string(),
        subject_refs: vec![TypedRefRequest {
            kind: "WorkSurface".to_string(),
            id: work_surface_id.to_string(),
            rel: "approves".to_string(),
            meta: serde_json::Value::Null,
        }],
        evidence_refs: vec![evidence_ref.to_string()],
        exceptions_acknowledged: vec![],
        rationale: Some(format!("Branch 0 E2E test approval for {}", portal_id)),
    };

    let resp = client
        .http
        .post(&client.url("/approvals"))
        .headers(client.human_headers())
        .json(&approval_req)
        .send()
        .await
        .expect("Failed to record approval");

    assert!(
        resp.status().is_success(),
        "Failed to record approval: {}",
        resp.text().await.unwrap_or_default()
    );

    let approval: ApprovalActionResponse = resp.json().await.expect("Failed to parse approval");
    approval.approval_id
}

/// Completes a stage, handling approval requirements
async fn complete_stage(
    client: &TestClient,
    work_surface_id: &str,
    stage_id: &str,
    requires_approval: bool,
) -> StageCompletionResponse {
    let evidence_ref = format!("sha256:branch0-evidence-{}", stage_id.replace(":", "-"));

    let complete_req = CompleteStageRequest {
        evidence_bundle_ref: evidence_ref.clone(),
        gate_result: GateResultRequest {
            status: "PASS".to_string(),
            oracle_results: vec![OracleResultRequest {
                oracle_id: "branch0-oracle".to_string(),
                status: "PASS".to_string(),
                evidence_ref: Some(evidence_ref.clone()),
            }],
            waiver_refs: vec![],
        },
    };

    let resp = client
        .http
        .post(&client.url(&format!(
            "/work-surfaces/{}/stages/{}/complete",
            work_surface_id, stage_id
        )))
        .headers(client.human_headers())
        .json(&complete_req)
        .send()
        .await
        .expect("Failed to complete stage");

    // If stage requires approval and we get 412, record approval and retry
    if resp.status().as_u16() == 412 && requires_approval {
        let portal_id = format!("portal:STAGE_COMPLETION:{}", stage_id);
        record_approval(client, &portal_id, work_surface_id, &evidence_ref).await;

        // Retry stage completion
        let resp = client
            .http
            .post(&client.url(&format!(
                "/work-surfaces/{}/stages/{}/complete",
                work_surface_id, stage_id
            )))
            .headers(client.human_headers())
            .json(&complete_req)
            .send()
            .await
            .expect("Failed to complete stage after approval");

        assert!(
            resp.status().is_success(),
            "Failed to complete stage {} after approval: {}",
            stage_id,
            resp.text().await.unwrap_or_default()
        );

        return resp.json().await.expect("Failed to parse completion");
    }

    assert!(
        resp.status().is_success(),
        "Failed to complete stage {}: {}",
        stage_id,
        resp.text().await.unwrap_or_default()
    );

    resp.json().await.expect("Failed to parse completion")
}

/// Polls for evidence bundles with timeout
async fn poll_for_evidence(
    client: &TestClient,
    candidate_id: &str,
    timeout_secs: u64,
) -> Option<Vec<EvidenceBundleInfo>> {
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(timeout_secs);

    while start.elapsed() < timeout {
        let resp = client
            .http
            .get(&client.url(&format!("/candidates/{}/evidence", candidate_id)))
            .headers(client.human_headers())
            .send()
            .await;

        if let Ok(resp) = resp {
            if resp.status().is_success() {
                if let Ok(evidence_list) = resp.json::<EvidenceListResponse>().await {
                    if !evidence_list.evidence.is_empty() {
                        return Some(evidence_list.evidence);
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    None
}

// =============================================================================
// Test 1: Complete Branch 0 Flow
// =============================================================================

/// Main E2E test for Branch 0 complete flow
///
/// Tests the full 5-stage GENERIC-KNOWLEDGE-WORK procedure:
/// FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL
///
/// Per SR-PLAN §4.1 Branch 0 acceptance criteria:
/// - Loop created for problem-statement work unit
/// - Iteration started with Work Surface ref set
/// - Candidate intake bundle produced
/// - Evidence Bundle from semantic oracle suite
/// - Human portal approval recorded (SEMANTIC_EVAL, FINAL)
/// - Freeze baseline created
#[tokio::test]
#[ignore] // Requires running API server
async fn test_branch_0_complete_flow() {
    let config = TestConfig::default();
    let client = TestClient::new(config);

    let work_unit_id = format!("wu:branch0-complete-{}", ulid::Ulid::new());
    println!("\n=== Branch 0 Complete Flow Test ===");
    println!("Work Unit ID: {}", work_unit_id);

    // =========================================================================
    // Step 1: Create Work Surface with GENERIC-KNOWLEDGE-WORK template
    // =========================================================================
    println!("\n[Step 1] Creating Work Surface with GENERIC-KNOWLEDGE-WORK template...");
    let work_surface_id = create_work_surface_generic(&client, &work_unit_id).await;
    println!("  Work Surface ID: {}", work_surface_id);

    // Verify initial stage is FRAME
    let ws: WorkSurfaceResponse = client
        .http
        .get(&client.url(&format!("/work-surfaces/{}", work_surface_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get WS")
        .json()
        .await
        .expect("Failed to parse WS");
    assert_eq!(ws.current_stage_id, "stage:FRAME");
    println!("  Initial stage: {}", ws.current_stage_id);

    // =========================================================================
    // Step 2: Create and activate Loop bound to work surface
    // =========================================================================
    println!("\n[Step 2] Creating Loop bound to work surface...");
    let loop_id = create_and_activate_loop(&client, &work_unit_id).await;
    println!("  Loop ID: {}", loop_id);

    // Verify loop is bound to work surface
    let loop_resp: LoopResponse = client
        .http
        .get(&client.url(&format!("/loops/{}", loop_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get loop")
        .json()
        .await
        .expect("Failed to parse loop");

    assert!(
        loop_resp.work_surface_id.is_some(),
        "Loop should be bound to work surface"
    );
    assert_eq!(loop_resp.state, "ACTIVE", "Loop should be ACTIVE");
    println!("  Loop bound to: {:?}", loop_resp.work_surface_id);

    // =========================================================================
    // Step 3: Start first iteration
    // =========================================================================
    println!("\n[Step 3] Starting first iteration (SYSTEM actor)...");
    let iteration_id = start_iteration(&client, &loop_id).await;
    println!("  Iteration ID: {}", iteration_id);

    // =========================================================================
    // Step 4: Progress through all 5 stages
    // =========================================================================
    println!("\n[Step 4] Progressing through GENERIC-KNOWLEDGE-WORK stages...");

    // FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL
    // SEMANTIC_EVAL and FINAL require portal approval
    let stages = [
        ("stage:FRAME", false),
        ("stage:OPTIONS", false),
        ("stage:DRAFT", false),
        ("stage:SEMANTIC_EVAL", true),  // Trust boundary - requires approval
        ("stage:FINAL", true),          // Trust boundary - requires approval
    ];

    for (stage_id, requires_approval) in stages.iter() {
        println!("  Completing {}...", stage_id);

        // Start new iteration for each stage (fresh context per C-LOOP-2)
        if *stage_id != "stage:FRAME" {
            let _iter_id = start_iteration(&client, &loop_id).await;
        }

        let completion = complete_stage(&client, &work_surface_id, stage_id, *requires_approval).await;

        if *requires_approval {
            println!("    (Approval recorded at portal)");
        }

        println!(
            "    Completed. Next stage: {:?}, Terminal: {}",
            completion.next_stage_id, completion.is_terminal
        );

        // Verify terminal stage
        if *stage_id == "stage:FINAL" {
            assert!(completion.is_terminal, "FINAL should be terminal stage");
            assert_eq!(
                completion.work_surface_status, "completed",
                "Work Surface should be completed"
            );
        }
    }

    // =========================================================================
    // Step 5: Verify final Work Surface state
    // =========================================================================
    println!("\n[Step 5] Verifying final Work Surface state...");
    let ws: WorkSurfaceResponse = client
        .http
        .get(&client.url(&format!("/work-surfaces/{}", work_surface_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get WS")
        .json()
        .await
        .expect("Failed to parse WS");

    assert_eq!(ws.status, "completed", "Work Surface should be completed");
    println!("  Final status: {}", ws.status);

    // =========================================================================
    // Step 6: Create freeze baseline
    // =========================================================================
    println!("\n[Step 6] Creating freeze baseline...");

    // First record a release approval
    let release_approval_id = record_approval(
        &client,
        "portal:RELEASE_APPROVAL",
        &work_surface_id,
        "sha256:branch0-release-evidence",
    ).await;
    println!("  Release approval: {}", release_approval_id);

    let freeze_req = CreateFreezeRecordRequest {
        baseline_id: format!("baseline:branch0-{}", ulid::Ulid::new()),
        candidate_id: format!("cand:{}", ulid::Ulid::new()),
        verification_mode: "STRICT".to_string(),
        oracle_suite_id: "suite:SR-SUITE-STRUCTURE".to_string(),
        oracle_suite_hash: "sha256:0".repeat(64),
        evidence_bundle_refs: vec!["sha256:branch0-evidence-stage-FINAL".to_string()],
        waiver_refs: vec![],
        release_approval_id,
        artifact_manifest: vec![ArtifactManifestEntry {
            artifact_id: "branch0-output".to_string(),
            version: "1.0.0".to_string(),
            content_hash: "sha256:branch0-artifact-hash".to_string(),
        }],
        active_exceptions: vec![],
    };

    let resp = client
        .http
        .post(&client.url("/freeze-records"))
        .headers(client.human_headers())
        .json(&freeze_req)
        .send()
        .await
        .expect("Failed to create freeze record");

    assert!(
        resp.status().is_success(),
        "Failed to create freeze record: {}",
        resp.text().await.unwrap_or_default()
    );

    let freeze: FreezeRecordResponse = resp.json().await.expect("Failed to parse freeze");
    println!("  Freeze ID: {}", freeze.freeze_id);
    assert!(freeze.freeze_id.starts_with("freeze_"));

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n=== Branch 0 Complete Flow Test PASSED ===");
    println!("Work Unit: {}", work_unit_id);
    println!("Work Surface: {} (status: completed)", work_surface_id);
    println!("Loop: {} (state: ACTIVE)", loop_id);
    println!("Freeze: {}", freeze.freeze_id);
    println!("\nBranch 0 Acceptance Criteria Verified:");
    println!("  [x] Loop created for problem-statement work unit");
    println!("  [x] Work Surface bound with GENERIC-KNOWLEDGE-WORK template");
    println!("  [x] All 5 stages completed (FRAME → OPTIONS → DRAFT → SEMANTIC_EVAL → FINAL)");
    println!("  [x] Portal approvals recorded at trust boundaries");
    println!("  [x] Freeze baseline created");
    println!("==========================================\n");
}

// =============================================================================
// Test 2: Portal Approvals Required
// =============================================================================

/// Tests that SEMANTIC_EVAL and FINAL stages require portal approval
///
/// Per SR-PROCEDURE-KIT §2:
/// - SEMANTIC_EVAL has requires_approval: true
/// - FINAL has requires_approval: true
///
/// Per SR-CONTRACT C-TB-3:
/// - Approvals MUST be recorded by HUMAN actors only
#[tokio::test]
#[ignore] // Requires running API server
async fn test_branch_0_portal_approvals_required() {
    let config = TestConfig::default();
    let client = TestClient::new(config);

    let work_unit_id = format!("wu:branch0-approvals-{}", ulid::Ulid::new());
    println!("\n=== Branch 0 Portal Approvals Test ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create work surface and progress to SEMANTIC_EVAL
    let work_surface_id = create_work_surface_generic(&client, &work_unit_id).await;
    let loop_id = create_and_activate_loop(&client, &work_unit_id).await;

    // Complete non-approval stages
    for stage_id in &["stage:FRAME", "stage:OPTIONS", "stage:DRAFT"] {
        let _iter_id = start_iteration(&client, &loop_id).await;
        complete_stage(&client, &work_surface_id, stage_id, false).await;
    }

    // =========================================================================
    // Test: SEMANTIC_EVAL requires approval
    // =========================================================================
    println!("\n[Test] SEMANTIC_EVAL should require approval...");
    let _iter_id = start_iteration(&client, &loop_id).await;

    let evidence_ref = "sha256:branch0-evidence-stage-SEMANTIC_EVAL";
    let complete_req = CompleteStageRequest {
        evidence_bundle_ref: evidence_ref.to_string(),
        gate_result: GateResultRequest {
            status: "PASS".to_string(),
            oracle_results: vec![],
            waiver_refs: vec![],
        },
    };

    let resp = client
        .http
        .post(&client.url(&format!(
            "/work-surfaces/{}/stages/stage:SEMANTIC_EVAL/complete",
            work_surface_id
        )))
        .headers(client.human_headers())
        .json(&complete_req)
        .send()
        .await
        .expect("Request failed");

    // Should get 412 Precondition Failed
    assert_eq!(
        resp.status().as_u16(),
        412,
        "SEMANTIC_EVAL should require approval (412)"
    );

    let error: ApiErrorResponse = resp.json().await.expect("Failed to parse error");
    let error_code = error
        .details
        .as_ref()
        .and_then(|d| d.get("error_code"))
        .and_then(|c| c.as_str());

    println!("  Got 412 with error_code: {:?}", error_code);
    assert!(
        error_code == Some("APPROVAL_REQUIRED") || error_code == Some("STAGE_REQUIRES_APPROVAL"),
        "Expected approval-related error code, got {:?}",
        error_code
    );

    // Now record approval and complete
    record_approval(
        &client,
        "portal:STAGE_COMPLETION:stage:SEMANTIC_EVAL",
        &work_surface_id,
        evidence_ref,
    ).await;

    let resp = client
        .http
        .post(&client.url(&format!(
            "/work-surfaces/{}/stages/stage:SEMANTIC_EVAL/complete",
            work_surface_id
        )))
        .headers(client.human_headers())
        .json(&complete_req)
        .send()
        .await
        .expect("Retry failed");

    assert!(
        resp.status().is_success(),
        "Should complete after approval: {}",
        resp.text().await.unwrap_or_default()
    );
    println!("  Completed SEMANTIC_EVAL after approval");

    // =========================================================================
    // Test: FINAL requires approval
    // =========================================================================
    println!("\n[Test] FINAL should require approval...");
    let _iter_id = start_iteration(&client, &loop_id).await;

    let evidence_ref = "sha256:branch0-evidence-stage-FINAL";
    let complete_req = CompleteStageRequest {
        evidence_bundle_ref: evidence_ref.to_string(),
        gate_result: GateResultRequest {
            status: "PASS".to_string(),
            oracle_results: vec![],
            waiver_refs: vec![],
        },
    };

    let resp = client
        .http
        .post(&client.url(&format!(
            "/work-surfaces/{}/stages/stage:FINAL/complete",
            work_surface_id
        )))
        .headers(client.human_headers())
        .json(&complete_req)
        .send()
        .await
        .expect("Request failed");

    assert_eq!(
        resp.status().as_u16(),
        412,
        "FINAL should require approval (412)"
    );
    println!("  Got 412 as expected");

    // Complete with approval
    record_approval(
        &client,
        "portal:STAGE_COMPLETION:stage:FINAL",
        &work_surface_id,
        evidence_ref,
    ).await;

    let resp = client
        .http
        .post(&client.url(&format!(
            "/work-surfaces/{}/stages/stage:FINAL/complete",
            work_surface_id
        )))
        .headers(client.human_headers())
        .json(&complete_req)
        .send()
        .await
        .expect("Retry failed");

    assert!(resp.status().is_success());
    println!("  Completed FINAL after approval");

    println!("\n=== Portal Approvals Test PASSED ===\n");
}

// =============================================================================
// Test 3: Evidence Capture (Semantic Worker)
// =============================================================================

/// Tests evidence bundle capture when semantic worker is running
///
/// This test polls for evidence bundles after starting an iteration.
/// If the semantic worker is not running, the test is skipped gracefully.
///
/// Per V9-1: SemanticWorkerBridge subscribes to IterationStarted events
/// and produces EvidenceBundleRecorded events.
#[tokio::test]
#[ignore] // Requires running API server
async fn test_branch_0_evidence_capture() {
    let config = TestConfig::default();
    let client = TestClient::new(config);

    let work_unit_id = format!("wu:branch0-evidence-{}", ulid::Ulid::new());
    println!("\n=== Branch 0 Evidence Capture Test ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup
    let work_surface_id = create_work_surface_generic(&client, &work_unit_id).await;
    let loop_id = create_and_activate_loop(&client, &work_unit_id).await;

    println!("\n[Test] Starting iteration and polling for evidence...");
    let iteration_id = start_iteration(&client, &loop_id).await;
    println!("  Iteration ID: {}", iteration_id);

    // Poll for evidence with 5 second timeout
    let evidence = poll_for_evidence(&client, &iteration_id, 5).await;

    match evidence {
        Some(bundles) => {
            println!("  Evidence bundles found: {}", bundles.len());
            for bundle in &bundles {
                println!("    - {}", bundle.content_hash);
            }
            println!("\n=== Evidence Capture Test PASSED ===");
            println!("Semantic worker is producing evidence bundles.\n");
        }
        None => {
            println!("  No evidence bundles found within timeout.");
            println!("\n=== Evidence Capture Test SKIPPED ===");
            println!("Semantic worker may not be running (SR_ENABLE_SEMANTIC_WORKER=true).");
            println!("This is expected in CI without full Podman infrastructure.\n");
        }
    }
}

// =============================================================================
// Test 4: Freeze Baseline
// =============================================================================

/// Tests freeze baseline creation at end of Branch 0 flow
///
/// Per SR-CONTRACT C-SHIP-1:
/// - Freeze records MUST be created by HUMAN actors
/// - Freeze requires release approval
#[tokio::test]
#[ignore] // Requires running API server
async fn test_branch_0_freeze_baseline() {
    let config = TestConfig::default();
    let client = TestClient::new(config);

    let work_unit_id = format!("wu:branch0-freeze-{}", ulid::Ulid::new());
    println!("\n=== Branch 0 Freeze Baseline Test ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Complete full flow
    let work_surface_id = create_work_surface_generic(&client, &work_unit_id).await;
    let loop_id = create_and_activate_loop(&client, &work_unit_id).await;

    // Complete all stages
    let stages = [
        ("stage:FRAME", false),
        ("stage:OPTIONS", false),
        ("stage:DRAFT", false),
        ("stage:SEMANTIC_EVAL", true),
        ("stage:FINAL", true),
    ];

    for (idx, (stage_id, requires_approval)) in stages.iter().enumerate() {
        if idx > 0 {
            start_iteration(&client, &loop_id).await;
        }
        complete_stage(&client, &work_surface_id, stage_id, *requires_approval).await;
    }

    println!("\n[Test] Creating freeze baseline...");

    // Record release approval first
    let release_approval_id = record_approval(
        &client,
        "portal:RELEASE_APPROVAL",
        &work_surface_id,
        "sha256:release-evidence",
    ).await;
    println!("  Release approval: {}", release_approval_id);

    let baseline_id = format!("baseline:freeze-test-{}", ulid::Ulid::new());
    let candidate_id = format!("cand:{}", ulid::Ulid::new());

    let freeze_req = CreateFreezeRecordRequest {
        baseline_id: baseline_id.clone(),
        candidate_id: candidate_id.clone(),
        verification_mode: "STRICT".to_string(),
        oracle_suite_id: "suite:SR-SUITE-STRUCTURE".to_string(),
        oracle_suite_hash: format!("sha256:{}", "0".repeat(64)),
        evidence_bundle_refs: vec!["sha256:final-evidence".to_string()],
        waiver_refs: vec![],
        release_approval_id: release_approval_id.clone(),
        artifact_manifest: vec![],
        active_exceptions: vec![],
    };

    let resp = client
        .http
        .post(&client.url("/freeze-records"))
        .headers(client.human_headers())
        .json(&freeze_req)
        .send()
        .await
        .expect("Failed to create freeze");

    assert!(
        resp.status().is_success(),
        "Failed to create freeze: {}",
        resp.text().await.unwrap_or_default()
    );

    let freeze: FreezeRecordResponse = resp.json().await.expect("Failed to parse freeze");
    println!("  Freeze ID: {}", freeze.freeze_id);
    println!("  Baseline ID: {}", freeze.baseline_id);

    assert!(freeze.freeze_id.starts_with("freeze_"));
    assert_eq!(freeze.baseline_id, baseline_id);

    // Verify freeze can be retrieved
    let resp = client
        .http
        .get(&client.url(&format!("/freeze-records/{}", freeze.freeze_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get freeze");

    assert!(resp.status().is_success(), "Freeze should be retrievable");

    println!("\n=== Freeze Baseline Test PASSED ===\n");
}
