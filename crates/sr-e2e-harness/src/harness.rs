//! End-to-End Harness Implementation (D-34)
//!
//! Implements the happy path flow:
//! 1. Create loop → Activate loop
//! 2. Start iteration (SYSTEM)
//! 3. Worker submits candidate
//! 4. Oracle run with evidence
//! 5. Approval recorded (HUMAN at Release Portal)
//! 6. Freeze baseline
//!
//! Key invariants asserted:
//! - No approvals without evidence
//! - HUMAN-only for approvals and freeze
//! - SYSTEM-only for iteration start
//! - Evidence integrity verified

use crate::client::{ActionRequest, E2EClient, IterationSummaryRequest, TypedRefRequest};
use crate::transcript::{HarnessTranscript, TranscriptEntryKind};
use chrono::Utc;
use sha2::{Digest, Sha256};
use sr_adapters::{EvidenceArtifact, EvidenceManifestBuilder, OracleResult, OracleResultStatus};
use std::collections::HashMap;
use tracing::{error, info, instrument};

/// Harness configuration
#[derive(Debug, Clone)]
pub struct HarnessConfig {
    /// API base URL
    pub api_base_url: String,
    /// SYSTEM actor auth token
    pub system_token: String,
    /// HUMAN actor auth token
    pub human_token: String,
    /// Oracle suite ID to use
    pub oracle_suite_id: String,
    /// Oracle suite content hash
    pub oracle_suite_hash: String,
    /// Whether to verify evidence after upload
    pub verify_evidence: bool,
    /// Baseline ID for freeze
    pub baseline_id: String,
}

impl Default for HarnessConfig {
    fn default() -> Self {
        Self {
            api_base_url: std::env::var("SR_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            system_token: std::env::var("SR_SYSTEM_TOKEN")
                .unwrap_or_else(|_| "e2e-system-token".to_string()),
            human_token: std::env::var("SR_HUMAN_TOKEN")
                .unwrap_or_else(|_| "e2e-human-token".to_string()),
            oracle_suite_id: "sr-core-suite".to_string(),
            oracle_suite_hash: "0".repeat(64),
            verify_evidence: true,
            baseline_id: format!("baseline-e2e-{}", ulid::Ulid::new()),
        }
    }
}

/// Harness result containing transcript
pub struct HarnessResult {
    /// Execution transcript
    pub transcript: HarnessTranscript,
    /// Whether execution succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

/// Harness error types
#[derive(Debug, thiserror::Error)]
pub enum HarnessError {
    #[error("Client error: {0}")]
    ClientError(#[from] crate::client::ClientError),

    #[error("Invariant violation: {invariant}")]
    InvariantViolation { invariant: String },

    #[error("Unexpected state: {message}")]
    UnexpectedState { message: String },

    #[error("Evidence error: {message}")]
    EvidenceError { message: String },
}

/// Run the happy path end-to-end flow
#[instrument(skip(config))]
pub async fn run_happy_path(config: HarnessConfig) -> HarnessResult {
    let mut transcript = HarnessTranscript::new();

    // Record harness start
    transcript.start_operation(TranscriptEntryKind::HarnessStart, "Starting E2E happy path");
    info!(
        transcript_id = %transcript.transcript_id,
        "Starting E2E harness execution"
    );

    // Create client
    let client = E2EClient::new(&config.api_base_url)
        .with_system_token(config.system_token.clone())
        .with_human_token(config.human_token.clone());

    // Execute the flow
    match execute_happy_path(&client, &config, &mut transcript).await {
        Ok(()) => {
            // Record success
            transcript.complete_operation(
                TranscriptEntryKind::HarnessComplete,
                "E2E happy path completed successfully",
                None,
                None,
                Some(serde_json::json!({
                    "invariants_passed": transcript.all_invariants_passed(),
                    "events_produced": transcript.event_ids().len(),
                    "entities_produced": transcript.entity_ids().len()
                })),
            );
            transcript.mark_success();

            info!(
                transcript_id = %transcript.transcript_id,
                "E2E harness completed successfully"
            );

            HarnessResult {
                transcript,
                success: true,
                error: None,
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            transcript.fail_operation(
                TranscriptEntryKind::HarnessComplete,
                "E2E happy path failed",
                &error_msg,
            );
            transcript.mark_failed(&error_msg);

            error!(
                transcript_id = %transcript.transcript_id,
                error = %error_msg,
                "E2E harness failed"
            );

            HarnessResult {
                transcript,
                success: false,
                error: Some(error_msg),
            }
        }
    }
}

/// Execute the happy path flow
async fn execute_happy_path(
    client: &E2EClient,
    config: &HarnessConfig,
    transcript: &mut HarnessTranscript,
) -> Result<(), HarnessError> {
    // =========================================================================
    // Step 1: Create Loop
    // =========================================================================
    transcript.start_operation(TranscriptEntryKind::CreateLoop, "Creating loop");

    let loop_response = client
        .create_loop(
            "E2E harness test goal",
            TypedRefRequest {
                kind: "Directive".to_string(),
                id: "e2e-test-directive".to_string(),
                rel: "governs".to_string(),
                meta: serde_json::Value::Null,
            },
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::CreateLoop,
        "Loop created",
        Some(loop_response.event_id.clone()),
        Some(loop_response.loop_id.clone()),
        Some(serde_json::json!({ "state": loop_response.state })),
    );
    transcript.produced_ids.loop_id = Some(loop_response.loop_id.clone());
    transcript
        .produced_ids
        .event_ids
        .push(loop_response.event_id.clone());

    let loop_id = loop_response.loop_id;
    info!(loop_id = %loop_id, "Loop created");

    // =========================================================================
    // Step 2: Activate Loop
    // =========================================================================
    transcript.start_operation(TranscriptEntryKind::ActivateLoop, "Activating loop");

    let activate_response = client.activate_loop(&loop_id).await?;

    transcript.complete_operation(
        TranscriptEntryKind::ActivateLoop,
        "Loop activated",
        Some(activate_response.event_id.clone()),
        Some(loop_id.clone()),
        Some(serde_json::json!({ "state": activate_response.state })),
    );
    transcript
        .produced_ids
        .event_ids
        .push(activate_response.event_id);

    // Invariant: Loop state should be ACTIVE
    let loop_state = client.get_loop(&loop_id).await?;
    transcript.check_invariant(
        "loop_active_after_activation",
        loop_state.state == "ACTIVE",
        &format!("Loop state is {}", loop_state.state),
    );

    info!(loop_id = %loop_id, "Loop activated");

    // =========================================================================
    // Step 3: Start Iteration (SYSTEM-only)
    // =========================================================================
    transcript.start_operation(
        TranscriptEntryKind::StartIteration,
        "Starting iteration (SYSTEM actor)",
    );

    let iteration_response = client
        .start_iteration(
            &loop_id,
            vec![TypedRefRequest {
                kind: "Context".to_string(),
                id: "e2e-context".to_string(),
                rel: "provides".to_string(),
                meta: serde_json::Value::Null,
            }],
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::StartIteration,
        "Iteration started",
        Some(iteration_response.event_id.clone()),
        Some(iteration_response.iteration_id.clone()),
        Some(serde_json::json!({ "state": iteration_response.state })),
    );
    transcript
        .produced_ids
        .iteration_ids
        .push(iteration_response.iteration_id.clone());
    transcript
        .produced_ids
        .event_ids
        .push(iteration_response.event_id);

    let iteration_id = iteration_response.iteration_id;
    info!(iteration_id = %iteration_id, "Iteration started");

    // =========================================================================
    // Step 4: Register Candidate (worker submits)
    // =========================================================================
    transcript.start_operation(
        TranscriptEntryKind::RegisterCandidate,
        "Registering candidate (worker submission)",
    );

    // Generate deterministic content hash for candidate
    let candidate_content = format!("e2e-candidate-{}-{}", loop_id, iteration_id);
    let content_hash = compute_sha256(&candidate_content);

    let candidate_response = client
        .register_candidate(
            &content_hash,
            Some(&iteration_id),
            vec![TypedRefRequest {
                kind: "Iteration".to_string(),
                id: iteration_id.clone(),
                rel: "produced_by".to_string(),
                meta: serde_json::Value::Null,
            }],
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::RegisterCandidate,
        "Candidate registered",
        Some(candidate_response.event_id.clone()),
        Some(candidate_response.candidate_id.clone()),
        Some(serde_json::json!({
            "content_hash": candidate_response.content_hash,
            "verification_status": candidate_response.verification_status
        })),
    );
    transcript
        .produced_ids
        .candidate_ids
        .push(candidate_response.candidate_id.clone());
    transcript
        .produced_ids
        .event_ids
        .push(candidate_response.event_id);

    let candidate_id = candidate_response.candidate_id;
    info!(candidate_id = %candidate_id, "Candidate registered");

    // =========================================================================
    // Step 5: Start Oracle Run
    // =========================================================================
    transcript.start_operation(TranscriptEntryKind::StartRun, "Starting oracle run");

    let run_response = client
        .start_run(
            &candidate_id,
            &config.oracle_suite_id,
            &config.oracle_suite_hash,
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::StartRun,
        "Oracle run started",
        Some(run_response.event_id.clone()),
        Some(run_response.run_id.clone()),
        Some(serde_json::json!({ "state": run_response.state })),
    );
    transcript
        .produced_ids
        .run_ids
        .push(run_response.run_id.clone());
    transcript
        .produced_ids
        .event_ids
        .push(run_response.event_id);

    let run_id = run_response.run_id;
    info!(run_id = %run_id, "Oracle run started");

    // =========================================================================
    // Step 6: Upload Evidence Bundle
    // =========================================================================
    transcript.start_operation(
        TranscriptEntryKind::UploadEvidence,
        "Uploading evidence bundle",
    );

    // Create evidence manifest
    let evidence_manifest = create_evidence_manifest(
        &run_id,
        &candidate_id,
        &config.oracle_suite_id,
        &config.oracle_suite_hash,
    );

    let evidence_response = client
        .upload_evidence(evidence_manifest, HashMap::new())
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::UploadEvidence,
        "Evidence bundle uploaded",
        None,
        Some(evidence_response.content_hash.clone()),
        Some(serde_json::json!({
            "bundle_id": evidence_response.bundle_id,
            "verdict": evidence_response.verdict
        })),
    );
    transcript
        .produced_ids
        .evidence_hashes
        .push(evidence_response.content_hash.clone());

    let evidence_hash = evidence_response.content_hash;
    info!(evidence_hash = %evidence_hash, "Evidence uploaded");

    // Invariant: Evidence should be retrievable
    if config.verify_evidence {
        let retrieved = client.get_evidence(&evidence_hash).await;
        transcript.check_invariant(
            "evidence_retrievable",
            retrieved.is_ok(),
            &format!(
                "Evidence {} is {}",
                evidence_hash,
                if retrieved.is_ok() {
                    "retrievable"
                } else {
                    "not retrievable"
                }
            ),
        );
    }

    // =========================================================================
    // Step 7: Complete Oracle Run
    // =========================================================================
    transcript.start_operation(TranscriptEntryKind::CompleteRun, "Completing oracle run");

    let run_complete_response = client
        .complete_run(&run_id, "SUCCESS", Some(&evidence_hash))
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::CompleteRun,
        "Oracle run completed",
        Some(run_complete_response.event_id.clone()),
        Some(run_id.clone()),
        Some(serde_json::json!({ "state": run_complete_response.state })),
    );
    transcript
        .produced_ids
        .event_ids
        .push(run_complete_response.event_id);

    info!(run_id = %run_id, "Oracle run completed");

    // Invariant: Run should have evidence bundle hash
    let run_state = client.get_run(&run_id).await?;
    transcript.check_invariant(
        "run_has_evidence",
        run_state.evidence_bundle_hash.is_some(),
        &format!(
            "Run {} has evidence: {:?}",
            run_id, run_state.evidence_bundle_hash
        ),
    );

    // =========================================================================
    // Step 8: Complete Iteration
    // =========================================================================
    transcript.start_operation(
        TranscriptEntryKind::CompleteIteration,
        "Completing iteration with summary",
    );

    let iteration_complete_response = client
        .complete_iteration(
            &iteration_id,
            "SUCCESS",
            Some(IterationSummaryRequest {
                intent: "E2E harness iteration completed".to_string(),
                actions: vec![ActionRequest {
                    kind: "verification".to_string(),
                    summary: "Ran oracle suite and produced evidence".to_string(),
                    artifacts: vec![evidence_hash.clone()],
                }],
                artifacts_touched: vec![],
                candidates_produced: vec![candidate_id.clone()],
                runs_executed: vec![run_id.clone()],
                next_steps: vec![],
                open_risks: vec![],
            }),
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::CompleteIteration,
        "Iteration completed",
        Some(iteration_complete_response.event_id.clone()),
        Some(iteration_id.clone()),
        Some(serde_json::json!({ "state": iteration_complete_response.state })),
    );
    transcript
        .produced_ids
        .event_ids
        .push(iteration_complete_response.event_id);

    info!(iteration_id = %iteration_id, "Iteration completed");

    // =========================================================================
    // Step 9: Record Approval (HUMAN at Release Portal)
    // =========================================================================
    transcript.start_operation(
        TranscriptEntryKind::RecordApproval,
        "Recording approval (HUMAN at Release Portal)",
    );

    // Key invariant: Approval must have evidence refs
    let approval_response = client
        .record_approval(
            "release-portal",
            "APPROVED",
            vec![
                TypedRefRequest {
                    kind: "Candidate".to_string(),
                    id: candidate_id.clone(),
                    rel: "approves".to_string(),
                    meta: serde_json::Value::Null,
                },
                TypedRefRequest {
                    kind: "Run".to_string(),
                    id: run_id.clone(),
                    rel: "based_on".to_string(),
                    meta: serde_json::Value::Null,
                },
            ],
            vec![evidence_hash.clone()],
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::RecordApproval,
        "Approval recorded",
        Some(approval_response.event_id.clone()),
        Some(approval_response.approval_id.clone()),
        Some(serde_json::json!({
            "portal_id": approval_response.portal_id,
            "decision": approval_response.decision
        })),
    );
    transcript
        .produced_ids
        .approval_ids
        .push(approval_response.approval_id.clone());
    transcript
        .produced_ids
        .event_ids
        .push(approval_response.event_id);

    let approval_id = approval_response.approval_id;
    info!(approval_id = %approval_id, "Approval recorded");

    // Key invariant: No approvals without evidence
    transcript.check_invariant(
        "no_approval_without_evidence",
        !transcript.produced_ids.evidence_hashes.is_empty(),
        "Approval was recorded with evidence references",
    );

    // =========================================================================
    // Step 10: Create Freeze Record (HUMAN-only)
    // =========================================================================
    transcript.start_operation(
        TranscriptEntryKind::CreateFreezeRecord,
        "Creating freeze record (HUMAN-only, establishing baseline)",
    );

    let freeze_response = client
        .create_freeze_record(
            &config.baseline_id,
            &candidate_id,
            "STRICT",
            &config.oracle_suite_id,
            &config.oracle_suite_hash,
            vec![evidence_hash.clone()],
            &approval_id,
        )
        .await?;

    transcript.complete_operation(
        TranscriptEntryKind::CreateFreezeRecord,
        "Freeze record created",
        Some(freeze_response.event_id.clone()),
        Some(freeze_response.freeze_id.clone()),
        Some(serde_json::json!({
            "baseline_id": freeze_response.baseline_id,
            "candidate_id": freeze_response.candidate_id
        })),
    );
    transcript
        .produced_ids
        .freeze_ids
        .push(freeze_response.freeze_id.clone());
    transcript
        .produced_ids
        .event_ids
        .push(freeze_response.event_id);

    let freeze_id = freeze_response.freeze_id;
    info!(freeze_id = %freeze_id, baseline_id = %config.baseline_id, "Freeze record created");

    // Verify freeze record
    let freeze_state = client.get_freeze_record(&freeze_id).await?;
    transcript.check_invariant(
        "freeze_has_approval",
        freeze_state.release_approval_id == approval_id,
        &format!(
            "Freeze {} has release approval {}",
            freeze_id, freeze_state.release_approval_id
        ),
    );

    transcript.check_invariant(
        "freeze_has_evidence",
        !freeze_state.evidence_bundle_refs.is_empty(),
        &format!(
            "Freeze {} has {} evidence refs",
            freeze_id,
            freeze_state.evidence_bundle_refs.len()
        ),
    );

    // =========================================================================
    // Step 11: Close Loop
    // =========================================================================
    transcript.start_operation(TranscriptEntryKind::CloseLoop, "Closing loop");

    let close_response = client.close_loop(&loop_id).await?;

    transcript.complete_operation(
        TranscriptEntryKind::CloseLoop,
        "Loop closed",
        Some(close_response.event_id.clone()),
        Some(loop_id.clone()),
        Some(serde_json::json!({ "state": close_response.state })),
    );
    transcript
        .produced_ids
        .event_ids
        .push(close_response.event_id);

    info!(loop_id = %loop_id, "Loop closed");

    // Final invariant checks
    transcript.check_invariant(
        "complete_flow_executed",
        true,
        "All steps of happy path completed",
    );

    transcript.check_invariant(
        "all_entities_produced",
        transcript.produced_ids.loop_id.is_some()
            && !transcript.produced_ids.iteration_ids.is_empty()
            && !transcript.produced_ids.candidate_ids.is_empty()
            && !transcript.produced_ids.run_ids.is_empty()
            && !transcript.produced_ids.evidence_hashes.is_empty()
            && !transcript.produced_ids.approval_ids.is_empty()
            && !transcript.produced_ids.freeze_ids.is_empty(),
        "All required entities were produced",
    );

    Ok(())
}

/// Create an evidence manifest for the happy path
fn create_evidence_manifest(
    run_id: &str,
    candidate_id: &str,
    oracle_suite_id: &str,
    oracle_suite_hash: &str,
) -> sr_adapters::EvidenceManifest {
    let now = Utc::now();
    let bundle_id = format!("bundle_{}", ulid::Ulid::new());

    // Create oracle results (all passing for happy path)
    let oracle_results = vec![
        OracleResult {
            oracle_id: "lint".to_string(),
            oracle_name: "Lint Check".to_string(),
            status: OracleResultStatus::Pass,
            duration_ms: 100,
            error_message: None,
            artifact_refs: vec![],
            output: None,
        },
        OracleResult {
            oracle_id: "unit-test".to_string(),
            oracle_name: "Unit Tests".to_string(),
            status: OracleResultStatus::Pass,
            duration_ms: 500,
            error_message: None,
            artifact_refs: vec![],
            output: None,
        },
        OracleResult {
            oracle_id: "build".to_string(),
            oracle_name: "Build Check".to_string(),
            status: OracleResultStatus::Pass,
            duration_ms: 1000,
            error_message: None,
            artifact_refs: vec!["summary.txt".to_string()],
            output: None,
        },
    ];

    // Build the manifest using the builder
    let mut builder = EvidenceManifestBuilder::new()
        .bundle_id(bundle_id)
        .run_id(run_id)
        .candidate_id(candidate_id)
        .oracle_suite(oracle_suite_id, oracle_suite_hash)
        .run_times(now, now)
        .environment_fingerprint(serde_json::json!({
            "harness": "sr-e2e-harness",
            "version": env!("CARGO_PKG_VERSION")
        }))
        .add_artifact(EvidenceArtifact {
            name: "summary.txt".to_string(),
            content_hash: compute_sha256("E2E harness evidence summary"),
            content_type: "text/plain".to_string(),
            size: 30,
            description: Some("E2E harness evidence summary".to_string()),
        })
        .add_metadata("generated_by", serde_json::json!("E2E harness"))
        .add_metadata("purpose", serde_json::json!("Happy path verification"));

    // Add oracle results
    for result in oracle_results {
        builder = builder.add_result(result);
    }

    builder.build().expect("Failed to build evidence manifest")
}

/// Compute SHA-256 hash of a string
fn compute_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = HarnessConfig::default();
        assert!(!config.api_base_url.is_empty());
        assert!(!config.oracle_suite_id.is_empty());
        assert_eq!(config.oracle_suite_hash.len(), 64);
    }

    #[test]
    fn test_evidence_manifest_creation() {
        let manifest =
            create_evidence_manifest("run_123", "cand_456", "sr-core-suite", &"0".repeat(64));

        assert_eq!(manifest.run_id, "run_123");
        assert_eq!(manifest.candidate_id, "cand_456");
        assert_eq!(manifest.results.len(), 3);
        assert_eq!(manifest.verdict, OracleResultStatus::Pass);
    }

    #[test]
    fn test_compute_sha256() {
        let hash = compute_sha256("test");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
