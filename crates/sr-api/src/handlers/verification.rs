//! Verification computation (C-VER-1..4)
//!
//! Computes candidate verification status from recorded runs/evidence and
//! emits `CandidateVerificationComputed` events with scope and basis metadata.

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sr_adapters::{
    oracle_suite::{VerificationProfile, PROFILE_STRICT_CORE},
    EvidenceManifest, OracleResultStatus,
};
use sr_domain::{
    EventEnvelope, EventId, StreamKind, TypedRef, VerificationComputer, VerificationStatus,
};
use sr_ports::{EventStore, EvidenceStore};

use crate::auth::AuthenticatedUser;
use crate::handlers::{ApiError, ApiResult};
use crate::AppState;

/// Request payload for verification endpoint
#[derive(Debug, Deserialize)]
pub struct VerificationRequest {
    /// Optional verification profile override (defaults to STRICT-CORE)
    #[serde(default)]
    pub verification_profile_id: Option<String>,
}

/// Response payload for verification computation
#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub candidate_id: String,
    pub verification_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_mode: Option<String>,
    pub verification_profile_id: String,
    pub event_id: String,
    pub integrity_conditions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_scope: Option<serde_json::Value>,
}

/// Verification computation result (internal)
pub struct VerificationComputation {
    pub candidate_id: String,
    pub status: VerificationStatus,
    pub verification_mode: Option<String>,
    pub verification_profile_id: String,
    pub verification_scope: VerificationScope,
    pub oracle_suite_summaries: Vec<OracleSuiteSummary>,
    pub waiver_ids: Vec<String>,
    pub waived_oracle_ids: Vec<String>,
    pub integrity_conditions: Vec<String>,
    pub evidence_bundle_hashes: Vec<String>,
    pub verification_basis: serde_json::Value,
}

#[derive(Debug, Default, Clone)]
pub struct VerificationScope {
    pub procedure_template_id: Option<String>,
    pub stage_id: Option<String>,
    pub work_surface_id: Option<String>,
}

impl VerificationScope {
    fn capture_from_manifest(&mut self, manifest: &EvidenceManifest) {
        if self.procedure_template_id.is_none() {
            self.procedure_template_id = manifest.procedure_template_id.clone();
        }
        if self.stage_id.is_none() {
            self.stage_id = manifest.stage_id.clone();
        }
        if self.work_surface_id.is_none() {
            self.work_surface_id = manifest.work_surface_id.clone();
        }
    }

    fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "procedure_template_id": self.procedure_template_id,
            "stage_id": self.stage_id,
            "work_surface_id": self.work_surface_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct OracleSuiteSummary {
    pub suite_id: String,
    pub suite_hash: String,
}

/// POST /api/v1/candidates/{candidate_id}/verify
pub async fn verify_candidate(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(candidate_id): Path<String>,
    Json(body): Json<VerificationRequest>,
) -> ApiResult<Json<VerificationResponse>> {
    let (event_id, computation) =
        compute_and_record_verification(&state, &candidate_id, body.verification_profile_id)
            .await?;

    Ok(Json(VerificationResponse {
        candidate_id,
        verification_status: status_to_string(&computation.status).to_string(),
        verification_mode: computation.verification_mode,
        verification_profile_id: computation.verification_profile_id,
        event_id,
        integrity_conditions: computation.integrity_conditions,
        verification_scope: Some(computation.verification_scope.as_json()),
    }))
}

/// Compute verification and emit CandidateVerificationComputed
pub async fn compute_and_record_verification(
    state: &AppState,
    candidate_id: &str,
    profile_id: Option<String>,
) -> ApiResult<(String, VerificationComputation)> {
    let computation = compute_verification(state, candidate_id, profile_id).await?;

    let event_id = EventId::new();
    let event = EventEnvelope {
        event_id: event_id.clone(),
        stream_id: candidate_id.to_string(),
        stream_kind: StreamKind::Candidate,
        stream_seq: next_stream_seq(state.event_store.as_ref(), candidate_id).await?,
        global_seq: None,
        event_type: "CandidateVerificationComputed".to_string(),
        occurred_at: Utc::now(),
        actor_kind: sr_domain::ActorKind::System, // computation is SYSTEM-derived
        actor_id: "system:verification-computer".to_string(),
        correlation_id: None,
        causation_id: None,
        supersedes: vec![],
        refs: computation
            .evidence_bundle_hashes
            .iter()
            .map(|hash| TypedRef {
                kind: "EvidenceBundle".to_string(),
                id: hash.clone(),
                rel: "verifies".to_string(),
                meta: serde_json::json!({ "content_hash": hash }),
            })
            .collect(),
        payload: serde_json::json!({
            "candidate_id": candidate_id,
            "verification_status": status_to_string(&computation.status),
            "verification_mode": computation.verification_mode.clone(),
            "verification_profile_id": computation.verification_profile_id.clone(),
            "verification_scope": computation.verification_scope.as_json(),
            "verification_basis": computation.verification_basis.clone(),
            "waiver_ids": computation.waiver_ids.clone(),
            "waived_oracle_ids": computation.waived_oracle_ids.clone(),
            "integrity_conditions": computation.integrity_conditions.clone(),
            "evidence_bundle_hashes": computation.evidence_bundle_hashes.clone(),
            "oracle_suite_summaries": computation.oracle_suite_summaries.iter().map(|s| serde_json::json!({
                "suite_id": s.suite_id,
                "suite_hash": s.suite_hash
            })).collect::<Vec<_>>(),
        }),
        envelope_hash: compute_envelope_hash(&event_id),
    };

    state
        .event_store
        .append(candidate_id, event.stream_seq - 1, vec![event])
        .await?;
    state
        .projections
        .process_events(&*state.event_store)
        .await?;

    Ok((event_id.as_str().to_string(), computation))
}

async fn compute_verification(
    state: &AppState,
    candidate_id: &str,
    profile_id: Option<String>,
) -> ApiResult<VerificationComputation> {
    // Ensure candidate exists
    let candidate = state
        .projections
        .get_candidate(candidate_id)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: "Candidate".to_string(),
            id: candidate_id.to_string(),
        })?;

    let profile_id = profile_id.unwrap_or_else(|| PROFILE_STRICT_CORE.to_string());
    let profile: VerificationProfile = state
        .oracle_registry
        .get_profile(&profile_id)
        .await
        .ok_or_else(|| ApiError::BadRequest {
            message: format!("Unknown verification profile '{}'", profile_id),
        })?;

    let runs = state
        .projections
        .get_runs_for_candidate(candidate_id)
        .await?;

    let mut required_results: Vec<(String, bool)> = Vec::new();
    let mut evidence_bundle_hashes = Vec::new();
    let mut integrity_conditions = Vec::new();
    let mut oracle_suite_results = Vec::new();
    let mut suite_summaries = Vec::new();
    let mut verification_scope = VerificationScope::default();

    for suite_id in &profile.required_suites {
        let suite = state
            .oracle_registry
            .get_suite(suite_id)
            .await
            .ok_or_else(|| ApiError::BadRequest {
                message: format!("Required oracle suite '{}' not registered", suite_id),
            })?;

        let run = runs
            .iter()
            .rev()
            .find(|r| r.oracle_suite_id == *suite_id && r.completed_at.is_some());

        if run.is_none() {
            integrity_conditions.push("ORACLE_GAP".to_string());
            continue;
        }

        let run = run.unwrap();
        let evidence_hash = match &run.evidence_bundle_hash {
            Some(h) => h.clone(),
            None => {
                integrity_conditions.push("EVIDENCE_MISSING".to_string());
                continue;
            }
        };

        let manifest_bytes = match state.evidence_store.as_ref().retrieve(&evidence_hash).await {
            Ok(bytes) => bytes,
            Err(_) => {
                integrity_conditions.push("EVIDENCE_MISSING".to_string());
                continue;
            }
        };

        let manifest: EvidenceManifest = match serde_json::from_slice(&manifest_bytes) {
            Ok(m) => m,
            Err(_) => {
                integrity_conditions.push("ORACLE_TAMPER".to_string());
                continue;
            }
        };

        evidence_bundle_hashes.push(evidence_hash.clone());
        suite_summaries.push(OracleSuiteSummary {
            suite_id: suite_id.clone(),
            suite_hash: manifest.oracle_suite_hash.clone(),
        });

        if manifest.candidate_id != candidate_id
            || manifest.run_id != run.run_id
            || manifest.oracle_suite_id != run.oracle_suite_id
        {
            integrity_conditions.push("ORACLE_TAMPER".to_string());
        }

        if manifest.oracle_suite_hash != run.oracle_suite_hash {
            integrity_conditions.push("ORACLE_TAMPER".to_string());
        }

        verification_scope.capture_from_manifest(&manifest);

        let expected_oracles: std::collections::HashSet<_> =
            suite.oracles.iter().map(|o| o.oracle_id.clone()).collect();

        let mut missing_oracles = Vec::new();
        let mut oracle_results_json = Vec::new();

        for oracle_id in expected_oracles {
            match manifest.results.iter().find(|r| r.oracle_id == oracle_id) {
                Some(result) => {
                    let passed = matches!(result.status, OracleResultStatus::Pass);
                    required_results.push((format!("{}::{}", suite_id, oracle_id), passed));
                    if matches!(
                        result.status,
                        OracleResultStatus::Error | OracleResultStatus::Skipped
                    ) {
                        integrity_conditions.push("ORACLE_GAP".to_string());
                    }
                    oracle_results_json.push(serde_json::json!({
                        "oracle_id": oracle_id,
                        "status": format!("{:?}", result.status),
                        "passed": passed
                    }));
                }
                None => {
                    missing_oracles.push(oracle_id.clone());
                    required_results.push((format!("{}::{}", suite_id, oracle_id), false));
                }
            }
        }

        if !missing_oracles.is_empty() {
            integrity_conditions.push("ORACLE_GAP".to_string());
        }

        oracle_suite_results.push(serde_json::json!({
            "suite_id": suite_id,
            "suite_hash": manifest.oracle_suite_hash,
            "oracle_results": oracle_results_json,
            "missing_oracles": missing_oracles,
            "verdict": format!("{:?}", manifest.verdict),
        }));
    }

    let (waiver_ids, waived_oracle_ids) =
        collect_active_waivers(state, &candidate.produced_by_iteration_id).await?;

    // Integrity conditions always block verification
    let has_integrity_conditions = !integrity_conditions.is_empty();
    if required_results.is_empty() {
        // No runs or no results => treat as integrity gap
        integrity_conditions.push("ORACLE_GAP".to_string());
    }

    let status = VerificationComputer::compute(
        &required_results,
        &waived_oracle_ids,
        has_integrity_conditions || required_results.is_empty(),
    )
    .map_err(|e| ApiError::Internal {
        message: format!("Verification computation failed: {}", e),
    })?;

    let verification_mode = match status {
        VerificationStatus::VerifiedStrict => Some("STRICT".to_string()),
        VerificationStatus::VerifiedWithExceptions => Some("WITH_EXCEPTIONS".to_string()),
        VerificationStatus::Unverified => None,
    };

    let verification_basis = serde_json::json!({
        "verification_profile_id": profile_id,
        "oracle_suite_results": oracle_suite_results,
        "waiver_ids": waiver_ids,
        "waived_oracle_ids": waived_oracle_ids,
        "integrity_conditions": integrity_conditions,
        "evidence_bundle_hashes": evidence_bundle_hashes,
    });

    Ok(VerificationComputation {
        candidate_id: candidate_id.to_string(),
        status,
        verification_mode,
        verification_profile_id: profile_id,
        verification_scope,
        waiver_ids,
        waived_oracle_ids,
        integrity_conditions,
        evidence_bundle_hashes,
        verification_basis,
        oracle_suite_summaries: suite_summaries,
    })
}

async fn collect_active_waivers(
    state: &AppState,
    produced_by_iteration_id: &Option<String>,
) -> ApiResult<(Vec<String>, Vec<String>)> {
    if let Some(iter_id) = produced_by_iteration_id {
        if let Some(iteration) = state.projections.get_iteration(iter_id).await? {
            let loop_id = iteration.loop_id;
            let exceptions = state
                .projections
                .get_active_exceptions_for_loop(&loop_id, Utc::now())
                .await?;

            let mut waiver_ids = Vec::new();
            let mut waived_oracle_ids = Vec::new();

            for exception in exceptions {
                if exception.kind != "WAIVER" {
                    continue;
                }
                waiver_ids.push(exception.exception_id.clone());
                if let Some(oracle_id) = exception.scope.get("oracle_id").and_then(|v| v.as_str()) {
                    waived_oracle_ids.push(oracle_id.to_string());
                }
            }

            return Ok((waiver_ids, waived_oracle_ids));
        }
    }

    Ok((Vec::new(), Vec::new()))
}

async fn next_stream_seq(
    event_store: &sr_adapters::PostgresEventStore,
    stream_id: &str,
) -> ApiResult<u64> {
    let events = event_store
        .read_stream(stream_id, 0, 1000)
        .await
        .map_err(|e| ApiError::Internal {
            message: format!("Failed to read stream {stream_id}: {e}"),
        })?;
    Ok(events.len() as u64 + 1)
}

fn compute_envelope_hash(event_id: &EventId) -> String {
    format!("sha256:{}", event_id.as_str().replace("evt_", ""))
}

fn status_to_string(status: &VerificationStatus) -> &'static str {
    match status {
        VerificationStatus::Unverified => "UNVERIFIED",
        VerificationStatus::VerifiedStrict => "VERIFIED_STRICT",
        VerificationStatus::VerifiedWithExceptions => "VERIFIED_WITH_EXCEPTIONS",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_pass_is_verified_strict() {
        let results = vec![("suite::oracle:build".to_string(), true)];
        let status =
            VerificationComputer::compute(&results, &[], false).expect("verification compute");
        assert_eq!(status_to_string(&status), "VERIFIED_STRICT");
    }

    #[test]
    fn waived_fail_is_verified_with_exceptions() {
        let results = vec![("suite::oracle:lint".to_string(), false)];
        let status =
            VerificationComputer::compute(&results, &["suite::oracle:lint".to_string()], false)
                .expect("verification compute");
        assert_eq!(status_to_string(&status), "VERIFIED_WITH_EXCEPTIONS");
    }

    #[test]
    fn uncovered_fail_is_unverified() {
        let results = vec![("suite::oracle:test".to_string(), false)];
        let status =
            VerificationComputer::compute(&results, &[], false).expect("verification compute");
        assert_eq!(status_to_string(&status), "UNVERIFIED");
    }

    #[test]
    fn integrity_condition_blocks_verification() {
        let results = vec![("suite::oracle:test".to_string(), true)];
        let status =
            VerificationComputer::compute(&results, &[], true).expect("verification compute");
        assert_eq!(status_to_string(&status), "UNVERIFIED");
    }
}
