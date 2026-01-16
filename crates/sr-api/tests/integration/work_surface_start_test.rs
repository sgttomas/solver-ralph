//! Work Surface /start Endpoint Integration Tests (SR-PLAN-V7 Phase V7-1)
//!
//! Tests the `POST /api/v1/work-surfaces/{id}/start` orchestration endpoint:
//! 1. Happy path - Active Work Surface starts successfully
//! 2. Idempotency - Second call returns already_started: true
//! 3. Precondition rejection - Non-active Work Surface returns 412
//! 4. CREATED Loop edge case - Existing CREATED Loop gets activated
//! 5. HUMAN actor on Loop - LoopCreated uses the calling user's actor
//! 6. SYSTEM actor on Iteration - IterationStarted uses SYSTEM actor (C-CTX-1)
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
    #[allow(dead_code)]
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

    fn url(&self, path: &str) -> String {
        format!(
            "{}/api/v1{}",
            self.config.api_base_url.trim_end_matches('/'),
            path
        )
    }
}

// =============================================================================
// Request Types
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

#[derive(Debug, Serialize)]
struct CreateLoopRequest {
    goal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_unit: Option<String>,
    directive_ref: TypedRefRequest,
}

#[derive(Debug, Serialize)]
struct StartIterationRequest {
    loop_id: String,
    #[serde(default)]
    refs: Vec<TypedRefRequest>,
}

// =============================================================================
// Response Types
// =============================================================================

#[derive(Debug, Deserialize)]
struct WorkSurfaceActionResponse {
    work_surface_id: String,
    status: String,
    #[allow(dead_code)]
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct StartWorkResponse {
    work_surface_id: String,
    loop_id: String,
    iteration_id: String,
    already_started: bool,
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
    state: String,
    created_by: ActorInfo,
    #[allow(dead_code)]
    goal: String,
    #[allow(dead_code)]
    work_unit: String,
    #[allow(dead_code)]
    iteration_count: i32,
}

#[derive(Debug, Deserialize)]
struct ActorInfo {
    kind: String,
    #[allow(dead_code)]
    id: String,
}

#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    #[allow(dead_code)]
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
        title: "V7-1 Test Intake".to_string(),
        kind: "research_memo".to_string(),
        objective: "Test the /start endpoint".to_string(),
        audience: "Integration tests".to_string(),
        deliverables: vec![DeliverableRequest {
            name: "test-output".to_string(),
            format: "text".to_string(),
            path: "/tmp/v7-1-test.txt".to_string(),
            description: Some("Test deliverable".to_string()),
        }],
        constraints: vec![],
        definitions: HashMap::new(),
        inputs: vec![],
        unknowns: vec![],
        completion_criteria: vec![],
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

/// Creates an active Work Surface and returns its ID
async fn create_active_work_surface(client: &TestClient, work_unit_id: &str) -> String {
    // Create and activate intake
    let intake_id = create_intake(client, work_unit_id).await;
    activate_intake(client, &intake_id).await;

    // Create work surface
    let create_ws = CreateWorkSurfaceRequest {
        intake_id,
        procedure_template_id: "proc:RESEARCH-MEMO".to_string(),
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

// =============================================================================
// Test 1: Happy Path
// =============================================================================

/// Test that /start creates Loop, activates it, and starts Iteration
#[tokio::test]
#[ignore] // Requires running API server
async fn test_start_happy_path() {
    let config = TestConfig::default();
    let client = TestClient::new(config);
    let work_unit_id = format!("wu:start-happy-{}", ulid::Ulid::new());

    println!("\n=== Test: start_happy_path ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create active Work Surface
    println!("[Setup] Creating active Work Surface...");
    let work_surface_id = create_active_work_surface(&client, &work_unit_id).await;
    println!("  Work Surface ID: {}", work_surface_id);

    // Action: Call /start
    println!(
        "[Action] Calling POST /work-surfaces/{}/start...",
        work_surface_id
    );
    let resp = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("Failed to call /start");

    // Assert: HTTP 200 with expected response
    assert!(
        resp.status().is_success(),
        "Expected 200, got {}: {}",
        resp.status(),
        resp.text().await.unwrap_or_default()
    );

    let start_resp: StartWorkResponse = resp.json().await.expect("Failed to parse start response");

    println!("  Response:");
    println!("    work_surface_id: {}", start_resp.work_surface_id);
    println!("    loop_id: {}", start_resp.loop_id);
    println!("    iteration_id: {}", start_resp.iteration_id);
    println!("    already_started: {}", start_resp.already_started);

    assert_eq!(start_resp.work_surface_id, work_surface_id);
    assert!(
        !start_resp.loop_id.is_empty(),
        "loop_id should not be empty"
    );
    assert!(
        !start_resp.iteration_id.is_empty(),
        "iteration_id should not be empty"
    );
    assert!(
        !start_resp.already_started,
        "already_started should be false on first call"
    );

    println!("=== PASS: start_happy_path ===\n");
}

// =============================================================================
// Test 2: Idempotency
// =============================================================================

/// Test that calling /start twice returns already_started: true on second call
#[tokio::test]
#[ignore] // Requires running API server
async fn test_start_idempotent() {
    let config = TestConfig::default();
    let client = TestClient::new(config);
    let work_unit_id = format!("wu:start-idem-{}", ulid::Ulid::new());

    println!("\n=== Test: start_idempotent ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create active Work Surface
    println!("[Setup] Creating active Work Surface...");
    let work_surface_id = create_active_work_surface(&client, &work_unit_id).await;

    // First call
    println!("[Action] First call to /start...");
    let resp1 = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("First /start failed");

    assert!(resp1.status().is_success());
    let first: StartWorkResponse = resp1.json().await.expect("Failed to parse first response");
    println!("  First call: already_started={}", first.already_started);
    assert!(
        !first.already_started,
        "First call should have already_started=false"
    );

    // Second call
    println!("[Action] Second call to /start...");
    let resp2 = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("Second /start failed");

    assert!(resp2.status().is_success());
    let second: StartWorkResponse = resp2.json().await.expect("Failed to parse second response");
    println!("  Second call: already_started={}", second.already_started);

    // Assert idempotency
    assert!(
        second.already_started,
        "Second call should have already_started=true"
    );
    assert_eq!(first.loop_id, second.loop_id, "Loop ID should be the same");
    assert_eq!(
        first.iteration_id, second.iteration_id,
        "Iteration ID should be the same"
    );

    println!("=== PASS: start_idempotent ===\n");
}

// =============================================================================
// Test 3: Rejects Inactive Work Surface
// =============================================================================

/// Test that /start returns 412 for non-active Work Surface
#[tokio::test]
#[ignore] // Requires running API server
async fn test_start_rejects_inactive() {
    let config = TestConfig::default();
    let client = TestClient::new(config);
    let work_unit_id = format!("wu:start-inactive-{}", ulid::Ulid::new());

    println!("\n=== Test: start_rejects_inactive ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create active Work Surface, then archive it
    println!("[Setup] Creating Work Surface and archiving...");
    let work_surface_id = create_active_work_surface(&client, &work_unit_id).await;

    // Archive the Work Surface
    let archive_resp = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/archive", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("Failed to archive work surface");

    assert!(
        archive_resp.status().is_success(),
        "Failed to archive: {}",
        archive_resp.text().await.unwrap_or_default()
    );
    println!("  Work Surface archived");

    // Action: Try to start the archived Work Surface
    println!("[Action] Calling /start on archived Work Surface...");
    let resp = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("Request failed");

    // Assert: HTTP 412 with WORK_SURFACE_NOT_ACTIVE
    assert_eq!(
        resp.status().as_u16(),
        412,
        "Expected 412 Precondition Failed, got {}",
        resp.status()
    );

    let error: ApiErrorResponse = resp.json().await.expect("Failed to parse error response");
    assert_eq!(error.code, 412);

    let error_code = error
        .details
        .as_ref()
        .and_then(|d| d.get("error_code"))
        .and_then(|c| c.as_str());
    assert_eq!(
        error_code,
        Some("WORK_SURFACE_NOT_ACTIVE"),
        "Expected WORK_SURFACE_NOT_ACTIVE error code"
    );

    println!("  Correctly rejected with 412 WORK_SURFACE_NOT_ACTIVE");
    println!("=== PASS: start_rejects_inactive ===\n");
}

// =============================================================================
// Test 4: Activates Existing CREATED Loop
// =============================================================================

/// Test that /start activates an existing CREATED Loop and starts iteration
#[tokio::test]
#[ignore] // Requires running API server
async fn test_start_activates_created_loop() {
    let config = TestConfig::default();
    let client = TestClient::new(config);
    let work_unit_id = format!("wu:start-created-{}", ulid::Ulid::new());

    println!("\n=== Test: start_activates_created_loop ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create active Work Surface
    println!("[Setup] Creating active Work Surface...");
    let work_surface_id = create_active_work_surface(&client, &work_unit_id).await;

    // Manually create a Loop (without activating it)
    println!("[Setup] Creating Loop manually (will be in CREATED state)...");
    let create_loop = CreateLoopRequest {
        goal: "Test loop for CREATED state".to_string(),
        work_unit: Some(work_unit_id.clone()),
        directive_ref: TypedRefRequest {
            kind: "doc".to_string(),
            id: "SR-DIRECTIVE".to_string(),
            rel: "governs".to_string(),
            meta: serde_json::Value::Null,
        },
    };

    let loop_resp = client
        .http
        .post(&client.url("/loops"))
        .headers(client.human_headers())
        .json(&create_loop)
        .send()
        .await
        .expect("Failed to create loop");

    assert!(loop_resp.status().is_success());
    let loop_action: LoopActionResponse = loop_resp.json().await.expect("Failed to parse loop");
    let loop_id = loop_action.loop_id;
    println!("  Created Loop: {} (state: CREATED)", loop_id);

    // Verify Loop is in CREATED state
    let get_loop: LoopResponse = client
        .http
        .get(&client.url(&format!("/loops/{}", loop_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get loop")
        .json()
        .await
        .expect("Failed to parse loop");
    assert_eq!(get_loop.state, "CREATED", "Loop should be in CREATED state");

    // Action: Call /start - should activate the existing Loop
    println!("[Action] Calling /start (should activate existing CREATED loop)...");
    let resp = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("/start request failed");

    assert!(
        resp.status().is_success(),
        "Expected 200, got {}: {}",
        resp.status(),
        resp.text().await.unwrap_or_default()
    );

    let start_resp: StartWorkResponse = resp.json().await.expect("Failed to parse start response");
    println!(
        "  Response: loop_id={}, iteration_id={}",
        start_resp.loop_id, start_resp.iteration_id
    );

    // Assert: Same Loop was used (activated) and Iteration was created
    assert_eq!(start_resp.loop_id, loop_id, "Should use the existing Loop");
    assert!(
        !start_resp.iteration_id.is_empty(),
        "Should create an Iteration"
    );

    // Verify Loop is now ACTIVE
    let get_loop_after: LoopResponse = client
        .http
        .get(&client.url(&format!("/loops/{}", loop_id)))
        .headers(client.human_headers())
        .send()
        .await
        .expect("Failed to get loop")
        .json()
        .await
        .expect("Failed to parse loop");
    assert_eq!(get_loop_after.state, "ACTIVE", "Loop should now be ACTIVE");
    println!("  Loop is now ACTIVE");

    println!("=== PASS: start_activates_created_loop ===\n");
}

// =============================================================================
// Test 5: HUMAN Actor on LoopCreated
// =============================================================================

/// Test that LoopCreated event has HUMAN actor (the calling user)
#[tokio::test]
#[ignore] // Requires running API server
async fn test_start_human_on_loop_created() {
    let config = TestConfig::default();
    let client = TestClient::new(config);
    let work_unit_id = format!("wu:start-human-{}", ulid::Ulid::new());

    println!("\n=== Test: start_human_on_loop_created ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create active Work Surface
    println!("[Setup] Creating active Work Surface...");
    let work_surface_id = create_active_work_surface(&client, &work_unit_id).await;

    // Action: Call /start
    println!("[Action] Calling /start...");
    let resp = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("/start failed");

    assert!(resp.status().is_success());
    let start_resp: StartWorkResponse = resp.json().await.expect("Failed to parse response");
    let loop_id = start_resp.loop_id;
    println!("  Created Loop: {}", loop_id);

    // Verify Loop was created by HUMAN actor
    println!("[Verify] Getting Loop to check created_by actor...");
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

    println!(
        "  Loop created_by: kind={}, id={}",
        loop_resp.created_by.kind, loop_resp.created_by.id
    );

    assert_eq!(
        loop_resp.created_by.kind.to_uppercase(),
        "HUMAN",
        "LoopCreated should have HUMAN actor, got {}",
        loop_resp.created_by.kind
    );

    println!("=== PASS: start_human_on_loop_created ===\n");
}

// =============================================================================
// Test 6: SYSTEM Actor on IterationStarted
// =============================================================================

/// Test that IterationStarted uses SYSTEM actor (per C-CTX-1)
///
/// Since the IterationProjection doesn't expose actor info, we verify the
/// mediation pattern indirectly:
/// 1. HUMAN calls /start → Iteration is created successfully
/// 2. HUMAN directly calls POST /iterations → Should fail with 403
///
/// This proves the /start endpoint internally uses SYSTEM actor for iterations,
/// since HUMAN cannot create iterations directly (SR-SPEC §2.2).
#[tokio::test]
#[ignore] // Requires running API server
async fn test_start_system_on_iteration() {
    let config = TestConfig::default();
    let client = TestClient::new(config);
    let work_unit_id = format!("wu:start-system-{}", ulid::Ulid::new());

    println!("\n=== Test: start_system_on_iteration ===");
    println!("Work Unit ID: {}", work_unit_id);

    // Setup: Create active Work Surface
    println!("[Setup] Creating active Work Surface...");
    let work_surface_id = create_active_work_surface(&client, &work_unit_id).await;

    // Action 1: HUMAN calls /start - should succeed
    println!("[Action 1] HUMAN calls /start (should succeed via SYSTEM mediation)...");
    let resp = client
        .http
        .post(&client.url(&format!("/work-surfaces/{}/start", work_surface_id)))
        .headers(client.human_headers())
        .json(&serde_json::json!({}))
        .send()
        .await
        .expect("/start failed");

    assert!(
        resp.status().is_success(),
        "HUMAN /start should succeed: {}",
        resp.text().await.unwrap_or_default()
    );
    let start_resp: StartWorkResponse = resp.json().await.expect("Failed to parse");
    println!(
        "  Iteration created via /start: {}",
        start_resp.iteration_id
    );

    // Action 2: HUMAN directly calls POST /iterations - should fail with 403
    println!("[Action 2] HUMAN directly calls POST /iterations (should fail with 403)...");
    let start_iter = StartIterationRequest {
        loop_id: start_resp.loop_id.clone(),
        refs: vec![],
    };

    let direct_resp = client
        .http
        .post(&client.url("/iterations"))
        .headers(client.human_headers())
        .json(&start_iter)
        .send()
        .await
        .expect("Direct iteration request failed");

    assert_eq!(
        direct_resp.status().as_u16(),
        403,
        "HUMAN should NOT be able to call POST /iterations directly (SR-SPEC §2.2)"
    );
    println!("  Correctly rejected HUMAN direct iteration creation with 403");

    // This proves the mediation pattern:
    // - HUMAN initiates work via /start
    // - /start internally creates iteration as SYSTEM
    // - Direct iteration creation is SYSTEM-only
    println!("\n  Mediation pattern verified:");
    println!("  - HUMAN can start work via /start endpoint");
    println!("  - HUMAN cannot create iterations directly (403)");
    println!("  - Therefore, /start uses SYSTEM actor internally for IterationStarted");

    println!("=== PASS: start_system_on_iteration ===\n");
}
