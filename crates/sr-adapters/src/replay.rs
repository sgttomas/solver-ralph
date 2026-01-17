//! Replay Proof Module (D-36)
//!
//! Provides types and utilities for proving deterministic replay of the EventManager.
//!
//! Per SR-CONTRACT C-EVT-7: "Projections MUST be rebuildable from the event log."
//!
//! This module enables verification that:
//! 1. `EventManager.rebuild()` produces identical state from the same event sequence
//! 2. `compute_eligible_set()` returns identical results after replay
//! 3. All status projections are identical after replay
//! 4. No ghost inputs influence `apply_event()` — only event data is used
//!
//! See: docs/platform/SR-REPLAY-PROOF.md for formal proof documentation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sr_domain::EventEnvelope;

/// Replay proof artifact per SR-PLAN-V9 §3.3
///
/// Captures the result of verifying that replaying an event sequence
/// produces identical EventManager state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayProof {
    /// Proof identifier
    pub proof_id: String,

    /// Number of events in the sequence
    pub event_count: usize,

    /// First event ID in sequence (if any)
    pub first_event_id: Option<String>,

    /// Last event ID in sequence (if any)
    pub last_event_id: Option<String>,

    /// State hash after original event processing
    pub original_state_hash: String,

    /// State hash after replay
    pub replayed_state_hash: String,

    /// Whether replay produced identical state
    pub deterministic: bool,

    /// Any discrepancies found during comparison
    pub discrepancies: Vec<ReplayDiscrepancy>,

    /// When the proof was computed
    pub proof_computed_at: DateTime<Utc>,

    /// Plan instance ID that was used
    pub plan_instance_id: Option<String>,

    /// Number of work units in the projection
    pub work_unit_count: usize,
}

impl ReplayProof {
    /// Create a new replay proof
    pub fn new(
        events: &[EventEnvelope],
        original_hash: String,
        replayed_hash: String,
        discrepancies: Vec<ReplayDiscrepancy>,
        plan_instance_id: Option<String>,
        work_unit_count: usize,
    ) -> Self {
        let deterministic = original_hash == replayed_hash && discrepancies.is_empty();

        Self {
            proof_id: format!("proof_{}", ulid::Ulid::new()),
            event_count: events.len(),
            first_event_id: events.first().map(|e| e.event_id.as_str().to_string()),
            last_event_id: events.last().map(|e| e.event_id.as_str().to_string()),
            original_state_hash: original_hash,
            replayed_state_hash: replayed_hash,
            deterministic,
            discrepancies,
            proof_computed_at: Utc::now(),
            plan_instance_id,
            work_unit_count,
        }
    }

    /// Check if replay was deterministic
    pub fn is_deterministic(&self) -> bool {
        self.deterministic
    }

    /// Get discrepancy count
    pub fn discrepancy_count(&self) -> usize {
        self.discrepancies.len()
    }
}

/// Discrepancy found during replay comparison
///
/// Reports a field-level difference between original and replayed state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayDiscrepancy {
    /// Work unit ID where discrepancy was found
    pub work_unit_id: String,

    /// Field name that differs
    pub field: String,

    /// Original value (as JSON)
    pub original_value: serde_json::Value,

    /// Replayed value (as JSON)
    pub replayed_value: serde_json::Value,

    /// Human-readable description
    pub description: String,
}

impl ReplayDiscrepancy {
    /// Create a new discrepancy
    pub fn new(
        work_unit_id: impl Into<String>,
        field: impl Into<String>,
        original: serde_json::Value,
        replayed: serde_json::Value,
    ) -> Self {
        let field_str = field.into();
        Self {
            work_unit_id: work_unit_id.into(),
            field: field_str.clone(),
            original_value: original.clone(),
            replayed_value: replayed.clone(),
            description: format!(
                "Field '{}' differs: original={}, replayed={}",
                field_str, original, replayed
            ),
        }
    }
}

/// Summary of eligible set comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibleSetComparison {
    /// Original eligible work unit IDs (sorted)
    pub original_eligible: Vec<String>,

    /// Replayed eligible work unit IDs (sorted)
    pub replayed_eligible: Vec<String>,

    /// Whether sets are identical
    pub sets_match: bool,

    /// IDs only in original
    pub only_in_original: Vec<String>,

    /// IDs only in replayed
    pub only_in_replayed: Vec<String>,
}

impl EligibleSetComparison {
    /// Create from two sets of eligible work unit IDs
    pub fn from_sets(original: Vec<String>, replayed: Vec<String>) -> Self {
        let mut original_sorted = original.clone();
        let mut replayed_sorted = replayed.clone();
        original_sorted.sort();
        replayed_sorted.sort();

        let sets_match = original_sorted == replayed_sorted;

        let only_in_original: Vec<String> = original_sorted
            .iter()
            .filter(|id| !replayed_sorted.contains(id))
            .cloned()
            .collect();

        let only_in_replayed: Vec<String> = replayed_sorted
            .iter()
            .filter(|id| !original_sorted.contains(id))
            .cloned()
            .collect();

        Self {
            original_eligible: original_sorted,
            replayed_eligible: replayed_sorted,
            sets_match,
            only_in_original,
            only_in_replayed,
        }
    }
}

/// Extended replay proof including eligible set comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedReplayProof {
    /// Core replay proof
    pub core_proof: ReplayProof,

    /// Eligible set comparison
    pub eligible_set_comparison: EligibleSetComparison,

    /// Whether all verifications passed
    pub all_verified: bool,
}

impl ExtendedReplayProof {
    /// Create from core proof and eligible set comparison
    pub fn new(core_proof: ReplayProof, eligible_set_comparison: EligibleSetComparison) -> Self {
        let all_verified =
            core_proof.is_deterministic() && eligible_set_comparison.sets_match;

        Self {
            core_proof,
            eligible_set_comparison,
            all_verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replay_proof_deterministic() {
        let proof = ReplayProof::new(
            &[],
            "sha256:abc123".to_string(),
            "sha256:abc123".to_string(),
            vec![],
            Some("plan_001".to_string()),
            5,
        );

        assert!(proof.is_deterministic());
        assert_eq!(proof.discrepancy_count(), 0);
        assert!(proof.proof_id.starts_with("proof_"));
    }

    #[test]
    fn test_replay_proof_non_deterministic_hash() {
        let proof = ReplayProof::new(
            &[],
            "sha256:abc123".to_string(),
            "sha256:def456".to_string(),
            vec![],
            None,
            3,
        );

        assert!(!proof.is_deterministic());
    }

    #[test]
    fn test_replay_proof_non_deterministic_discrepancies() {
        let discrepancy = ReplayDiscrepancy::new(
            "WU-001",
            "coarse_status",
            serde_json::json!("TODO"),
            serde_json::json!("ELIGIBLE"),
        );

        let proof = ReplayProof::new(
            &[],
            "sha256:abc123".to_string(),
            "sha256:abc123".to_string(),
            vec![discrepancy],
            None,
            1,
        );

        // Has discrepancies, so not deterministic
        assert!(!proof.is_deterministic());
        assert_eq!(proof.discrepancy_count(), 1);
    }

    #[test]
    fn test_replay_discrepancy() {
        let discrepancy = ReplayDiscrepancy::new(
            "WU-001",
            "deps_satisfied",
            serde_json::json!(true),
            serde_json::json!(false),
        );

        assert_eq!(discrepancy.work_unit_id, "WU-001");
        assert_eq!(discrepancy.field, "deps_satisfied");
        assert!(discrepancy.description.contains("deps_satisfied"));
    }

    #[test]
    fn test_eligible_set_comparison_match() {
        let comparison = EligibleSetComparison::from_sets(
            vec!["WU-001".to_string(), "WU-002".to_string()],
            vec!["WU-002".to_string(), "WU-001".to_string()],
        );

        assert!(comparison.sets_match);
        assert!(comparison.only_in_original.is_empty());
        assert!(comparison.only_in_replayed.is_empty());
    }

    #[test]
    fn test_eligible_set_comparison_mismatch() {
        let comparison = EligibleSetComparison::from_sets(
            vec!["WU-001".to_string(), "WU-002".to_string()],
            vec!["WU-002".to_string(), "WU-003".to_string()],
        );

        assert!(!comparison.sets_match);
        assert_eq!(comparison.only_in_original, vec!["WU-001"]);
        assert_eq!(comparison.only_in_replayed, vec!["WU-003"]);
    }

    #[test]
    fn test_extended_replay_proof() {
        let core = ReplayProof::new(
            &[],
            "sha256:abc".to_string(),
            "sha256:abc".to_string(),
            vec![],
            None,
            2,
        );

        let eligible = EligibleSetComparison::from_sets(
            vec!["WU-001".to_string()],
            vec!["WU-001".to_string()],
        );

        let extended = ExtendedReplayProof::new(core, eligible);
        assert!(extended.all_verified);
    }
}
