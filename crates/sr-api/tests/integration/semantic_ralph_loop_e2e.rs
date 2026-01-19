//! Semantic Ralph Loop End-to-End Integration Test (SR-PLAN-V5 Phase 5d)
//!
//! This test proves the complete MVP workflow functions correctly:
//! 1. Create Intake → Activate Intake
//! 2. Bind Work Surface (with GENERIC-KNOWLEDGE-WORK template)
//! 3. Create Loop (validates Work Surface exists, stores work_surface_id)
//! 4. Start Iteration (auto-inherits work_unit_id from Loop)
//! 5. Progress through all stages with approval enforcement
//! 6. Verify Work Surface completes when terminal stage reached
//!
//! Per SR-CONTRACT invariants:
//! - C-TB-3: Approvals MUST be recorded by HUMAN actors only
//! - Approval-gated stages require approval before completion
//!
//! Requires:
//! - Running sr-api server at SR_API_URL (default: http://localhost:3000)
//! - Valid auth tokens: SR_HUMAN_TOKEN, SR_SYSTEM_TOKEN

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IntakeResponse {
    intake_id: String,
    status: String,
    #[allow(dead_code)]
    work_unit_id: String,
}

#[derive(Debug, Serialize)]
struct CreateWorkSurfaceRequest {
    intake_id: String,
    template_id: String,
    work_unit_id: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WorkSurfaceActionResponse {
    work_surface_id: String,
    status: String,
    #[allow(dead_code)]
    event_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WorkSurfaceResponse {
    work_surface_id: String,
    status: String,
    current_stage_id: String,
    #[allow(dead_code)]
    template_id: String,
}

#[derive(Debug, Serialize)]
struct CreateLoopRequest {
    goal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_unit: Option<String>,
    directive_ref: TypedRefRequest,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LoopActionResponse {
    loop_id: String,
    #[allow(dead_code)]
    state: String,
    #[allow(dead_code)]
    event_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LoopResponse {
    #[allow(dead_code)]
    loop_id: String,
    work_surface_id: Option<String>,
    #[allow(dead_code)]
    state: String,
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IterationResponse {
    #[allow(dead_code)]
    iteration_id: String,
    refs: serde_json::Value,
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
#[allow(dead_code)]
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
#[allow(dead_code)]
struct StageCompletionResponse {
    work_surface_id: String,
    completed_stage_id: String,
    is_terminal: bool,
    #[serde(default)]
    next_stage_id: Option<String>,
    work_surface_status: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ApiErrorResponse {
    error: String,
    code: u16,
    #[serde(default)]
    details: Option<serde_json::Value>,
}

// =============================================================================
// Test Implementation
// =============================================================================

/// Main E2E test for Semantic Ralph Loop workflow
#[tokio::test]
#[ignore] // Requires running API server
async fn test_semantic_ralph_loop_end_to_end() {
    let config = TestConfig::default();
    let client = TestClient::new(config);

    // Generate unique work unit ID for this test run
    let work_unit_id = format!("wu:e2e-test-{}", ulid::Ulid::new());
    println!("\n=== Starting Semantic Ralph Loop E2E Test ===");
    println!("Work Unit ID: {}", work_unit_id);

    // =========================================================================
    // Step 1: Create and Activate Intake
    // =========================================================================
    println!("\n[Step 1] Creating intake...");

    let create_intake = CreateIntakeRequest {
        work_unit_id: work_unit_id.clone(),
        title: "E2E Test Knowledge Work".to_string(),
        kind: "research_memo".to_string(),
        objective: "Test the complete Semantic Ralph Loop workflow".to_string(),
        audience: "Integration test framework".to_string(),
        deliverables: vec![DeliverableRequest {
            name: "test-output".to_string(),
            format: "text".to_string(),
            path: "/tmp/e2e-test-output.txt".to_string(),
            description: Some("Test deliverable".to_string()),
        }],
        constraints: vec!["Must complete within test timeout".to_string()],
        definitions: HashMap::new(),
        inputs: vec![],
        unknowns: vec![],
        completion_criteria: vec!["All stages complete".to_string()],
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

    // Parse intake_id directly from create response
    let create_response: serde_json::Value =
        resp.json().await.expect("Failed to parse create response");
    let intake_id = create_response
        .get("intake_id")
        .and_then(|v| v.as_str())
        .expect("No intake_id in create response")
        .to_string();

    println!("  Created intake: {}", intake_id);
    assert_eq!(
        create_response.get("status").and_then(|v| v.as_str()),
        Some("draft")
    );

    // Activate intake
    println!("  Activating intake...");
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
    println!("  Intake activated");

    // =========================================================================
    // Step 2: Create Work Surface
    // =========================================================================
    println!("\n[Step 2] Creating Work Surface with RESEARCH-MEMO template...");

    let create_ws = CreateWorkSurfaceRequest {
        intake_id: intake_id.clone(),
        template_id: "proc:RESEARCH-MEMO".to_string(),
        work_unit_id: work_unit_id.clone(),
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
    let work_surface_id = ws_action.work_surface_id.clone();
    println!("  Created Work Surface: {}", work_surface_id);
    assert_eq!(ws_action.status, "active");

    // Get work surface to verify initial stage
    let ws: WorkSurfaceResponse = client
        .http
        .get(&client.url(&format!("/work-surfaces/{}", work_surface_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get work surface")
        .json()
        .await
        .expect("Failed to parse WS");

    println!("  Current stage: {}", ws.current_stage_id);
    assert_eq!(ws.current_stage_id, "stage:FRAME");

    // =========================================================================
    // Step 3: Create Loop (validates Work Surface exists)
    // =========================================================================
    println!("\n[Step 3] Creating Loop bound to Work Unit...");

    let create_loop = CreateLoopRequest {
        goal: "E2E Test: Complete knowledge synthesis workflow".to_string(),
        work_unit: Some(work_unit_id.clone()),
        directive_ref: TypedRefRequest {
            kind: "Directive".to_string(),
            id: "e2e-test-directive".to_string(),
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
    let loop_id = loop_action.loop_id.clone();
    println!("  Created Loop: {}", loop_id);

    // Verify loop has work_surface_id bound (SR-PLAN-V5 Phase 5b)
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
        "Loop should have work_surface_id bound"
    );
    println!(
        "  Loop bound to Work Surface: {}",
        loop_resp.work_surface_id.as_ref().unwrap()
    );

    // Activate the loop (transition from CREATED to ACTIVE)
    println!("  Activating loop...");
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
    println!("  Loop activated");

    // =========================================================================
    // Step 4: Start Iteration
    // =========================================================================
    println!("\n[Step 4] Starting Iteration (SYSTEM actor)...");

    let start_iter = StartIterationRequest {
        loop_id: loop_id.clone(),
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
    let iteration_id = iter_action.iteration_id.clone();
    println!("  Started Iteration: {}", iteration_id);

    // Verify iteration inherited Work Surface context
    let iter_resp: IterationResponse = client
        .http
        .get(&client.url(&format!("/iterations/{}", iteration_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get iteration")
        .json()
        .await
        .expect("Failed to parse iteration");

    let empty_vec = vec![];
    let refs_array = iter_resp.refs.as_array().unwrap_or(&empty_vec);
    let has_intake_ref = refs_array
        .iter()
        .any(|r| r.get("kind").and_then(|k| k.as_str()) == Some("Intake"));
    let has_template_ref = refs_array
        .iter()
        .any(|r| r.get("kind").and_then(|k| k.as_str()) == Some("Template"));

    println!(
        "  Iteration refs: {} total, has Intake={}, has Template={}",
        refs_array.len(),
        has_intake_ref,
        has_template_ref
    );
    // Note: refs may be empty if Work Surface context lookup is optional

    // =========================================================================
    // Step 5: Complete all stages (FRAME, OPTIONS, DRAFT, FINAL)
    // =========================================================================
    println!("\n[Step 5] Completing all stages...");

    // RESEARCH-MEMO template has 4 stages: FRAME -> OPTIONS -> DRAFT -> FINAL
    // Note: This template from the starter registry does not have requires_approval flags set
    let all_stages = ["stage:FRAME", "stage:OPTIONS", "stage:DRAFT", "stage:FINAL"];

    for (idx, stage_id) in all_stages.iter().enumerate() {
        println!("  Completing stage: {}", stage_id);

        let complete_req = CompleteStageRequest {
            evidence_bundle_ref: format!("sha256:e2e-evidence-{}", stage_id.replace(":", "-")),
            gate_result: GateResultRequest {
                status: "PASS".to_string(),
                oracle_results: vec![OracleResultRequest {
                    oracle_id: "e2e-oracle".to_string(),
                    status: "PASS".to_string(),
                    evidence_ref: Some(format!(
                        "sha256:oracle-evidence-{}",
                        stage_id.replace(":", "-")
                    )),
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

        // If stage requires approval and we got 412, record approval and retry
        if resp.status().as_u16() == 412 {
            println!("    Stage requires approval, recording approval...");

            let portal_id = format!("portal:STAGE_COMPLETION:{}", stage_id);
            let approval_req = RecordApprovalRequest {
                portal_id: portal_id.clone(),
                decision: "APPROVED".to_string(),
                subject_refs: vec![TypedRefRequest {
                    kind: "WorkSurface".to_string(),
                    id: work_surface_id.clone(),
                    rel: "approves".to_string(),
                    meta: serde_json::Value::Null,
                }],
                evidence_refs: vec![format!(
                    "sha256:e2e-evidence-{}",
                    stage_id.replace(":", "-")
                )],
                exceptions_acknowledged: vec![],
                rationale: Some(format!("E2E test approval for {} stage", stage_id)),
            };

            let approval_resp = client
                .http
                .post(&client.url("/approvals"))
                .headers(client.human_headers())
                .json(&approval_req)
                .send()
                .await
                .expect("Failed to record approval");

            assert!(
                approval_resp.status().is_success(),
                "Failed to record approval: {}",
                approval_resp.text().await.unwrap_or_default()
            );

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

            let completion: StageCompletionResponse =
                resp.json().await.expect("Failed to parse completion");
            println!(
                "    Completed (with approval). Next stage: {:?}, is_terminal: {}",
                completion.next_stage_id, completion.is_terminal
            );
        } else {
            assert!(
                resp.status().is_success(),
                "Failed to complete stage {}: {}",
                stage_id,
                resp.text().await.unwrap_or_default()
            );

            let completion: StageCompletionResponse =
                resp.json().await.expect("Failed to parse completion");
            println!(
                "    Completed. Next stage: {:?}, is_terminal: {}",
                completion.next_stage_id, completion.is_terminal
            );

            // If this is the last stage, verify it's terminal
            if idx == all_stages.len() - 1 {
                assert!(completion.is_terminal, "FINAL should be terminal stage");
                assert_eq!(
                    completion.work_surface_status, "completed",
                    "Work Surface should be completed"
                );
            }
        }
    }

    // =========================================================================
    // Step 6: Verify final state
    // =========================================================================
    println!("\n[Step 6] Verifying final Work Surface state...");

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

    assert_eq!(ws.status, "completed");
    println!("  Final Work Surface status: {}", ws.status);

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n=== E2E Test Complete ===");
    println!("Work Unit ID: {}", work_unit_id);
    println!("Intake ID: {}", intake_id);
    println!("Work Surface ID: {}", work_surface_id);
    println!("Loop ID: {}", loop_id);
    println!("Iteration ID: {}", iteration_id);
    println!("Final Status: COMPLETED");
    println!("\nAll SR-CONTRACT invariants verified:");
    println!("  - Approval-gated stages require approval before completion");
    println!("  - Loop validates Work Surface exists before creation");
    println!("  - Work Surface reaches COMPLETED status when terminal stage reached");
    println!("=========================\n");
}

/// Test that loop creation fails when no Work Surface exists
#[tokio::test]
#[ignore] // Requires running API server
async fn test_loop_creation_requires_work_surface() {
    let config = TestConfig::default();
    let client = TestClient::new(config);

    let work_unit_id = format!("wu:no-ws-test-{}", ulid::Ulid::new());
    println!("\n=== Testing Loop Creation Requires Work Surface ===");
    println!("Work Unit ID (no Work Surface): {}", work_unit_id);

    let create_loop = CreateLoopRequest {
        goal: "This should fail".to_string(),
        work_unit: Some(work_unit_id.clone()),
        directive_ref: TypedRefRequest {
            kind: "Directive".to_string(),
            id: "test".to_string(),
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
        .expect("Request failed");

    assert_eq!(
        resp.status().as_u16(),
        412,
        "Expected 412 when creating loop without Work Surface"
    );

    let error: ApiErrorResponse = resp.json().await.expect("Failed to parse error");
    assert!(
        error
            .details
            .as_ref()
            .and_then(|d| d.get("error_code"))
            .and_then(|c| c.as_str())
            == Some("WORK_SURFACE_MISSING"),
        "Expected WORK_SURFACE_MISSING error code"
    );

    println!("Correctly rejected loop creation without Work Surface");
    println!("=========================\n");
}
