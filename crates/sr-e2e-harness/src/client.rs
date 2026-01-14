//! E2E API Client
//!
//! HTTP client for interacting with the SOLVER-Ralph API during e2e tests.

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use sr_adapters::EvidenceManifest;
use std::collections::HashMap;

/// E2E API client
pub struct E2EClient {
    base_url: String,
    http_client: reqwest::Client,
    /// Auth token for SYSTEM actor
    system_token: Option<String>,
    /// Auth token for HUMAN actor
    human_token: Option<String>,
}

impl E2EClient {
    /// Create a new E2E client
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http_client: reqwest::Client::new(),
            system_token: None,
            human_token: None,
        }
    }

    /// Set the SYSTEM actor token
    pub fn with_system_token(mut self, token: String) -> Self {
        self.system_token = Some(token);
        self
    }

    /// Set the HUMAN actor token
    pub fn with_human_token(mut self, token: String) -> Self {
        self.human_token = Some(token);
        self
    }

    fn auth_headers(&self, actor_kind: ActorKind) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let token = match actor_kind {
            ActorKind::System => self.system_token.as_deref(),
            ActorKind::Human => self.human_token.as_deref(),
        };

        if let Some(token) = token {
            if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert(AUTHORIZATION, value);
            }
        }

        headers
    }

    // =========================================================================
    // Loop Operations
    // =========================================================================

    /// Create a new loop
    pub async fn create_loop(
        &self,
        goal: &str,
        directive_ref: TypedRefRequest,
    ) -> Result<LoopActionResponse, ClientError> {
        let url = format!("{}/api/v1/loops", self.base_url);
        let body = CreateLoopRequest {
            goal: goal.to_string(),
            work_unit: None,
            budgets: None,
            directive_ref,
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Activate a loop
    pub async fn activate_loop(&self, loop_id: &str) -> Result<LoopActionResponse, ClientError> {
        let url = format!("{}/api/v1/loops/{}/activate", self.base_url, loop_id);
        let body = TransitionLoopRequest { rationale: None };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get loop by ID
    pub async fn get_loop(&self, loop_id: &str) -> Result<LoopResponse, ClientError> {
        let url = format!("{}/api/v1/loops/{}", self.base_url, loop_id);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Close a loop
    pub async fn close_loop(&self, loop_id: &str) -> Result<LoopActionResponse, ClientError> {
        let url = format!("{}/api/v1/loops/{}/close", self.base_url, loop_id);
        let body = TransitionLoopRequest { rationale: None };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Iteration Operations
    // =========================================================================

    /// Start an iteration (SYSTEM-only)
    pub async fn start_iteration(
        &self,
        loop_id: &str,
        refs: Vec<TypedRefRequest>,
    ) -> Result<IterationActionResponse, ClientError> {
        let url = format!("{}/api/v1/iterations", self.base_url);
        let body = StartIterationRequest {
            loop_id: loop_id.to_string(),
            refs,
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::System))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Complete an iteration
    pub async fn complete_iteration(
        &self,
        iteration_id: &str,
        outcome: &str,
        summary: Option<IterationSummaryRequest>,
    ) -> Result<IterationActionResponse, ClientError> {
        let url = format!(
            "{}/api/v1/iterations/{}/complete",
            self.base_url, iteration_id
        );
        let body = CompleteIterationRequest {
            outcome: outcome.to_string(),
            summary,
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::System))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Candidate Operations
    // =========================================================================

    /// Register a candidate
    pub async fn register_candidate(
        &self,
        content_hash: &str,
        produced_by_iteration_id: Option<&str>,
        refs: Vec<TypedRefRequest>,
    ) -> Result<CandidateActionResponse, ClientError> {
        let url = format!("{}/api/v1/candidates", self.base_url);
        let body = RegisterCandidateRequest {
            content_hash: content_hash.to_string(),
            git_sha: None,
            produced_by_iteration_id: produced_by_iteration_id.map(|s| s.to_string()),
            refs,
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::System))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get candidate by ID
    pub async fn get_candidate(
        &self,
        candidate_id: &str,
    ) -> Result<CandidateResponse, ClientError> {
        let url = format!("{}/api/v1/candidates/{}", self.base_url, candidate_id);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Run Operations
    // =========================================================================

    /// Start a run
    pub async fn start_run(
        &self,
        candidate_id: &str,
        oracle_suite_id: &str,
        oracle_suite_hash: &str,
    ) -> Result<RunActionResponse, ClientError> {
        let url = format!("{}/api/v1/runs", self.base_url);
        let body = StartRunRequest {
            candidate_id: candidate_id.to_string(),
            oracle_suite_id: oracle_suite_id.to_string(),
            oracle_suite_hash: oracle_suite_hash.to_string(),
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::System))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Complete a run
    pub async fn complete_run(
        &self,
        run_id: &str,
        outcome: &str,
        evidence_bundle_hash: Option<&str>,
    ) -> Result<RunActionResponse, ClientError> {
        let url = format!("{}/api/v1/runs/{}/complete", self.base_url, run_id);
        let body = CompleteRunRequest {
            outcome: outcome.to_string(),
            evidence_bundle_hash: evidence_bundle_hash.map(|s| s.to_string()),
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::System))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get run by ID
    pub async fn get_run(&self, run_id: &str) -> Result<RunResponse, ClientError> {
        let url = format!("{}/api/v1/runs/{}", self.base_url, run_id);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Evidence Operations
    // =========================================================================

    /// Upload evidence bundle
    pub async fn upload_evidence(
        &self,
        manifest: EvidenceManifest,
        blobs: HashMap<String, String>,
    ) -> Result<UploadEvidenceResponse, ClientError> {
        let url = format!("{}/api/v1/evidence", self.base_url);
        let body = UploadEvidenceRequest { manifest, blobs };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::System))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get evidence by content hash
    pub async fn get_evidence(&self, content_hash: &str) -> Result<EvidenceResponse, ClientError> {
        let url = format!("{}/api/v1/evidence/{}", self.base_url, content_hash);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Approval Operations
    // =========================================================================

    /// Record an approval (HUMAN-only)
    pub async fn record_approval(
        &self,
        portal_id: &str,
        decision: &str,
        subject_refs: Vec<TypedRefRequest>,
        evidence_refs: Vec<String>,
    ) -> Result<ApprovalActionResponse, ClientError> {
        let url = format!("{}/api/v1/approvals", self.base_url);
        let body = RecordApprovalRequest {
            portal_id: portal_id.to_string(),
            decision: decision.to_string(),
            subject_refs,
            evidence_refs,
            exceptions_acknowledged: vec![],
            rationale: Some("E2E harness approval".to_string()),
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get approval by ID
    pub async fn get_approval(&self, approval_id: &str) -> Result<ApprovalResponse, ClientError> {
        let url = format!("{}/api/v1/approvals/{}", self.base_url, approval_id);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Freeze Operations
    // =========================================================================

    /// Create a freeze record (HUMAN-only)
    pub async fn create_freeze_record(
        &self,
        baseline_id: &str,
        candidate_id: &str,
        verification_mode: &str,
        oracle_suite_id: &str,
        oracle_suite_hash: &str,
        evidence_bundle_refs: Vec<String>,
        release_approval_id: &str,
    ) -> Result<FreezeActionResponse, ClientError> {
        let url = format!("{}/api/v1/freeze-records", self.base_url);
        let body = CreateFreezeRequest {
            baseline_id: baseline_id.to_string(),
            candidate_id: candidate_id.to_string(),
            verification_mode: verification_mode.to_string(),
            oracle_suite_id: oracle_suite_id.to_string(),
            oracle_suite_hash: oracle_suite_hash.to_string(),
            evidence_bundle_refs,
            waiver_refs: vec![],
            release_approval_id: release_approval_id.to_string(),
            artifact_manifest: vec![],
            active_exceptions: vec![],
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get freeze record by ID
    pub async fn get_freeze_record(
        &self,
        freeze_id: &str,
    ) -> Result<FreezeRecordResponse, ClientError> {
        let url = format!("{}/api/v1/freeze-records/{}", self.base_url, freeze_id);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Exception Operations (D-35)
    // =========================================================================

    /// Create an exception (HUMAN-only)
    pub async fn create_exception(
        &self,
        kind: &str,
        scope: ExceptionScopeRequest,
        rationale: &str,
        target_description: &str,
        expires_at: Option<&str>,
    ) -> Result<ExceptionActionResponse, ClientError> {
        let url = format!("{}/api/v1/exceptions", self.base_url);
        let body = CreateExceptionRequest {
            kind: kind.to_string(),
            scope,
            rationale: rationale.to_string(),
            target_description: target_description.to_string(),
            expires_at: expires_at.map(String::from),
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Activate an exception
    pub async fn activate_exception(
        &self,
        exception_id: &str,
    ) -> Result<ExceptionActionResponse, ClientError> {
        let url = format!(
            "{}/api/v1/exceptions/{}/activate",
            self.base_url, exception_id
        );

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&serde_json::json!({}))
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Resolve an exception
    pub async fn resolve_exception(
        &self,
        exception_id: &str,
        resolution_notes: Option<&str>,
    ) -> Result<ExceptionActionResponse, ClientError> {
        let url = format!(
            "{}/api/v1/exceptions/{}/resolve",
            self.base_url, exception_id
        );
        let body = ResolveExceptionRequest {
            resolution_notes: resolution_notes.map(String::from),
        };

        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get exception by ID
    pub async fn get_exception(
        &self,
        exception_id: &str,
    ) -> Result<ExceptionResponse, ClientError> {
        let url = format!("{}/api/v1/exceptions/{}", self.base_url, exception_id);

        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers(ActorKind::Human))
            .send()
            .await?;

        self.handle_response(response).await
    }

    // =========================================================================
    // Response Handling
    // =========================================================================

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ClientError> {
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(ClientError::ApiError {
                status: status.as_u16(),
                message: body,
            });
        }

        serde_json::from_str(&body).map_err(|e| ClientError::ParseError {
            message: format!("Failed to parse response: {} - Body: {}", e, body),
        })
    }
}

// =============================================================================
// Actor Kind
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorKind {
    System,
    Human,
}

// =============================================================================
// Error Types
// =============================================================================

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("Parse error: {message}")]
    ParseError { message: String },
}

// =============================================================================
// Request Types
// =============================================================================

#[derive(Debug, Serialize)]
pub struct CreateLoopRequest {
    pub goal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<LoopBudgetsRequest>,
    pub directive_ref: TypedRefRequest,
}

#[derive(Debug, Serialize)]
pub struct LoopBudgetsRequest {
    pub max_iterations: u32,
    pub max_oracle_runs: u32,
    pub max_wallclock_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedRefRequest {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct TransitionLoopRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StartIterationRequest {
    pub loop_id: String,
    #[serde(default)]
    pub refs: Vec<TypedRefRequest>,
}

#[derive(Debug, Serialize)]
pub struct CompleteIterationRequest {
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<IterationSummaryRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationSummaryRequest {
    pub intent: String,
    #[serde(default)]
    pub actions: Vec<ActionRequest>,
    #[serde(default)]
    pub artifacts_touched: Vec<String>,
    #[serde(default)]
    pub candidates_produced: Vec<String>,
    #[serde(default)]
    pub runs_executed: Vec<String>,
    #[serde(default)]
    pub next_steps: Vec<NextStepRequest>,
    #[serde(default)]
    pub open_risks: Vec<OpenRiskRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequest {
    pub kind: String,
    pub summary: String,
    #[serde(default)]
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStepRequest {
    pub kind: String,
    pub description: String,
    #[serde(default)]
    pub blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRiskRequest {
    pub severity: String,
    pub description: String,
    #[serde(default)]
    pub mitigation: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterCandidateRequest {
    pub content_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produced_by_iteration_id: Option<String>,
    #[serde(default)]
    pub refs: Vec<TypedRefRequest>,
}

#[derive(Debug, Serialize)]
pub struct StartRunRequest {
    pub candidate_id: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
}

#[derive(Debug, Serialize)]
pub struct CompleteRunRequest {
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_bundle_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UploadEvidenceRequest {
    pub manifest: EvidenceManifest,
    #[serde(default)]
    pub blobs: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct RecordApprovalRequest {
    pub portal_id: String,
    pub decision: String,
    pub subject_refs: Vec<TypedRefRequest>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub exceptions_acknowledged: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateFreezeRequest {
    pub baseline_id: String,
    pub candidate_id: String,
    pub verification_mode: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    #[serde(default)]
    pub evidence_bundle_refs: Vec<String>,
    #[serde(default)]
    pub waiver_refs: Vec<String>,
    pub release_approval_id: String,
    #[serde(default)]
    pub artifact_manifest: Vec<ArtifactManifestEntryRequest>,
    #[serde(default)]
    pub active_exceptions: Vec<ActiveExceptionEntryRequest>,
}

#[derive(Debug, Serialize)]
pub struct ArtifactManifestEntryRequest {
    pub artifact_id: String,
    pub version: String,
    pub content_hash: String,
}

#[derive(Debug, Serialize)]
pub struct ActiveExceptionEntryRequest {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
}

// =============================================================================
// Response Types
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct LoopActionResponse {
    pub loop_id: String,
    pub state: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoopResponse {
    pub loop_id: String,
    pub goal: String,
    pub work_unit: String,
    pub state: String,
    pub created_at: String,
    pub activated_at: Option<String>,
    pub closed_at: Option<String>,
    pub iteration_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct IterationActionResponse {
    pub iteration_id: String,
    pub loop_id: String,
    pub state: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CandidateActionResponse {
    pub candidate_id: String,
    pub content_hash: String,
    pub verification_status: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CandidateResponse {
    pub candidate_id: String,
    pub content_hash: String,
    pub produced_by_iteration_id: Option<String>,
    pub verification_status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RunActionResponse {
    pub run_id: String,
    pub candidate_id: String,
    pub state: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RunResponse {
    pub run_id: String,
    pub candidate_id: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub state: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub evidence_bundle_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploadEvidenceResponse {
    pub content_hash: String,
    pub bundle_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub verdict: String,
    pub stored_at: String,
}

#[derive(Debug, Deserialize)]
pub struct EvidenceResponse {
    pub content_hash: String,
    pub manifest: EvidenceManifest,
    pub blob_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApprovalActionResponse {
    pub approval_id: String,
    pub portal_id: String,
    pub decision: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ApprovalResponse {
    pub approval_id: String,
    pub portal_id: String,
    pub decision: String,
    pub approved_at: String,
}

#[derive(Debug, Deserialize)]
pub struct FreezeActionResponse {
    pub freeze_id: String,
    pub baseline_id: String,
    pub candidate_id: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FreezeRecordResponse {
    pub freeze_id: String,
    pub baseline_id: String,
    pub candidate_id: String,
    pub verification_mode: String,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: String,
    pub evidence_bundle_refs: Vec<String>,
    pub release_approval_id: String,
    pub frozen_at: String,
}

// =============================================================================
// Exception Types (D-35)
// =============================================================================

#[derive(Debug, Serialize)]
pub struct CreateExceptionRequest {
    pub kind: String,
    pub scope: ExceptionScopeRequest,
    pub rationale: String,
    pub target_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionScopeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<String>,
    #[serde(default)]
    pub artifact_refs: Vec<TypedRefRequest>,
}

impl Default for ExceptionScopeRequest {
    fn default() -> Self {
        Self {
            loop_id: None,
            candidate_id: None,
            oracle_id: None,
            artifact_refs: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResolveExceptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExceptionActionResponse {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ExceptionResponse {
    pub exception_id: String,
    pub kind: String,
    pub status: String,
    pub scope: serde_json::Value,
    pub rationale: String,
    pub target_description: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub resolved_at: Option<String>,
}
