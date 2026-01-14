//! Harness Transcript (D-34)
//!
//! Deterministic event+evidence transcript for audit and verification.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Complete harness execution transcript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessTranscript {
    /// Transcript ID
    pub transcript_id: String,
    /// Transcript version
    pub version: String,
    /// Harness start time
    pub started_at: DateTime<Utc>,
    /// Harness end time
    pub ended_at: Option<DateTime<Utc>>,
    /// Execution status
    pub status: TranscriptStatus,
    /// Chronological list of entries
    pub entries: Vec<TranscriptEntry>,
    /// Entity IDs produced during execution
    pub produced_ids: ProducedIds,
    /// Invariants that were checked
    pub invariants_checked: Vec<InvariantCheck>,
    /// Content hash of transcript for integrity
    pub content_hash: Option<String>,
}

impl HarnessTranscript {
    /// Create a new transcript
    pub fn new() -> Self {
        Self {
            transcript_id: format!("transcript_{}", ulid::Ulid::new()),
            version: "1.0.0".to_string(),
            started_at: Utc::now(),
            ended_at: None,
            status: TranscriptStatus::Running,
            entries: Vec::new(),
            produced_ids: ProducedIds::default(),
            invariants_checked: Vec::new(),
            content_hash: None,
        }
    }

    /// Add an entry to the transcript
    pub fn add_entry(&mut self, entry: TranscriptEntry) {
        self.entries.push(entry);
    }

    /// Record an operation start
    pub fn start_operation(&mut self, kind: TranscriptEntryKind, description: &str) {
        self.add_entry(TranscriptEntry {
            sequence: self.entries.len() as u32 + 1,
            timestamp: Utc::now(),
            kind,
            phase: EntryPhase::Start,
            description: description.to_string(),
            event_id: None,
            entity_id: None,
            error: None,
            details: None,
        });
    }

    /// Record an operation success
    pub fn complete_operation(
        &mut self,
        kind: TranscriptEntryKind,
        description: &str,
        event_id: Option<String>,
        entity_id: Option<String>,
        details: Option<serde_json::Value>,
    ) {
        self.add_entry(TranscriptEntry {
            sequence: self.entries.len() as u32 + 1,
            timestamp: Utc::now(),
            kind,
            phase: EntryPhase::Complete,
            description: description.to_string(),
            event_id,
            entity_id,
            error: None,
            details,
        });
    }

    /// Record an operation failure
    pub fn fail_operation(&mut self, kind: TranscriptEntryKind, description: &str, error: &str) {
        self.add_entry(TranscriptEntry {
            sequence: self.entries.len() as u32 + 1,
            timestamp: Utc::now(),
            kind,
            phase: EntryPhase::Failed,
            description: description.to_string(),
            event_id: None,
            entity_id: None,
            error: Some(error.to_string()),
            details: None,
        });
    }

    /// Record an invariant check
    pub fn check_invariant(&mut self, name: &str, passed: bool, message: &str) {
        self.invariants_checked.push(InvariantCheck {
            name: name.to_string(),
            passed,
            message: message.to_string(),
            checked_at: Utc::now(),
        });
    }

    /// Mark transcript as successful
    pub fn mark_success(&mut self) {
        self.ended_at = Some(Utc::now());
        self.status = TranscriptStatus::Success;
        self.compute_hash();
    }

    /// Mark transcript as failed
    pub fn mark_failed(&mut self, error: &str) {
        self.ended_at = Some(Utc::now());
        self.status = TranscriptStatus::Failed {
            error: error.to_string(),
        };
        self.compute_hash();
    }

    /// Compute content hash
    fn compute_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.transcript_id.as_bytes());
        hasher.update(self.started_at.to_rfc3339().as_bytes());

        for entry in &self.entries {
            hasher.update(entry.sequence.to_le_bytes());
            hasher.update(entry.timestamp.to_rfc3339().as_bytes());
            hasher.update(format!("{:?}", entry.kind).as_bytes());
            hasher.update(entry.description.as_bytes());
            if let Some(ref event_id) = entry.event_id {
                hasher.update(event_id.as_bytes());
            }
            if let Some(ref entity_id) = entry.entity_id {
                hasher.update(entity_id.as_bytes());
            }
        }

        self.content_hash = Some(format!("sha256:{}", hex::encode(hasher.finalize())));
    }

    /// Get all event IDs produced
    pub fn event_ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter_map(|e| e.event_id.clone())
            .collect()
    }

    /// Get all entity IDs produced
    pub fn entity_ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter_map(|e| e.entity_id.clone())
            .collect()
    }

    /// Check if all invariants passed
    pub fn all_invariants_passed(&self) -> bool {
        self.invariants_checked.iter().all(|i| i.passed)
    }

    /// Get failed invariants
    pub fn failed_invariants(&self) -> Vec<&InvariantCheck> {
        self.invariants_checked.iter().filter(|i| !i.passed).collect()
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize to deterministic JSON
    pub fn to_deterministic_json(&self) -> serde_json::Result<String> {
        // For determinism, we use sorted keys
        let value = serde_json::to_value(self)?;
        Ok(serde_json::to_string_pretty(&value)?)
    }
}

impl Default for HarnessTranscript {
    fn default() -> Self {
        Self::new()
    }
}

/// Transcript status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TranscriptStatus {
    Running,
    Success,
    Failed { error: String },
}

/// A single transcript entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEntry {
    /// Sequence number within transcript
    pub sequence: u32,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Entry kind
    pub kind: TranscriptEntryKind,
    /// Phase (start/complete/failed)
    pub phase: EntryPhase,
    /// Human-readable description
    pub description: String,
    /// Event ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Entity ID (loop, iteration, candidate, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Entry phase
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryPhase {
    Start,
    Complete,
    Failed,
}

/// Transcript entry kinds
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TranscriptEntryKind {
    // Loop lifecycle
    CreateLoop,
    ActivateLoop,
    CloseLoop,

    // Iteration lifecycle
    StartIteration,
    CompleteIteration,

    // Candidate lifecycle
    RegisterCandidate,

    // Run lifecycle
    StartRun,
    CompleteRun,

    // Evidence
    UploadEvidence,
    VerifyEvidence,

    // Approval
    RecordApproval,

    // Freeze
    CreateFreezeRecord,

    // Invariants
    InvariantCheck,

    // Meta
    HarnessStart,
    HarnessComplete,

    // =========================================================================
    // Failure Mode Entry Kinds (D-35)
    // =========================================================================

    // Oracle failure flow
    OracleFailure,

    // Integrity conditions
    IntegrityCheck,
    TamperDetected,
    CoverageGapDetected,
    EnvironmentMismatchDetected,
    FlakeDetected,

    // Exception flow
    CreateException,
    ActivateException,
    ResolveException,

    // Waiver flow
    CreateWaiver,
    ApprovalWithWaiver,

    // Stop triggers
    StopTriggered,
    PortalSubmission,

    // Verified-with-exceptions
    VerifiedWithExceptions,
}

/// Produced entity IDs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProducedIds {
    pub loop_id: Option<String>,
    pub iteration_ids: Vec<String>,
    pub candidate_ids: Vec<String>,
    pub run_ids: Vec<String>,
    pub evidence_hashes: Vec<String>,
    pub approval_ids: Vec<String>,
    pub freeze_ids: Vec<String>,
    pub event_ids: Vec<String>,
    /// Exception IDs (D-35)
    pub exception_ids: Vec<String>,
    /// Waiver IDs (D-35)
    pub waiver_ids: Vec<String>,
}

/// Invariant check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantCheck {
    /// Invariant name
    pub name: String,
    /// Whether it passed
    pub passed: bool,
    /// Message (success or failure reason)
    pub message: String,
    /// When checked
    pub checked_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcript_creation() {
        let transcript = HarnessTranscript::new();
        assert!(transcript.transcript_id.starts_with("transcript_"));
        assert_eq!(transcript.status, TranscriptStatus::Running);
        assert!(transcript.entries.is_empty());
    }

    #[test]
    fn test_transcript_entries() {
        let mut transcript = HarnessTranscript::new();

        transcript.start_operation(TranscriptEntryKind::CreateLoop, "Creating test loop");
        transcript.complete_operation(
            TranscriptEntryKind::CreateLoop,
            "Loop created",
            Some("evt_123".to_string()),
            Some("loop_456".to_string()),
            None,
        );

        assert_eq!(transcript.entries.len(), 2);
        assert_eq!(transcript.entries[0].phase, EntryPhase::Start);
        assert_eq!(transcript.entries[1].phase, EntryPhase::Complete);
    }

    #[test]
    fn test_invariant_checks() {
        let mut transcript = HarnessTranscript::new();

        transcript.check_invariant(
            "no_approvals_without_evidence",
            true,
            "All approvals have evidence",
        );
        transcript.check_invariant(
            "human_approval_required",
            false,
            "Missing human approval",
        );

        assert!(!transcript.all_invariants_passed());
        assert_eq!(transcript.failed_invariants().len(), 1);
    }

    #[test]
    fn test_transcript_hash() {
        let mut transcript = HarnessTranscript::new();
        transcript.start_operation(TranscriptEntryKind::HarnessStart, "Starting harness");
        transcript.mark_success();

        assert!(transcript.content_hash.is_some());
        assert!(transcript.content_hash.as_ref().unwrap().starts_with("sha256:"));
    }
}
