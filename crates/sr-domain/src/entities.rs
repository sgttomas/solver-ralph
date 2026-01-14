//! Domain entities per SR-SPEC §1.2.3
//!
//! Platform domain types aligned with SR-TYPES §4.3

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Actor kind per SR-SPEC §1.4.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorKind {
    Human,
    Agent,
    System,
}

/// Actor identity per SR-SPEC §1.4.2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorId {
    pub kind: ActorKind,
    pub id: String,
}

/// Loop identifier: `loop_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoopId(String);

impl LoopId {
    pub fn new() -> Self {
        Self(format!("loop_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for LoopId {
    fn default() -> Self {
        Self::new()
    }
}

/// Iteration identifier: `iter_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IterationId(String);

impl IterationId {
    pub fn new() -> Self {
        Self(format!("iter_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for IterationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Candidate identifier per SR-SPEC §1.3.3
/// Format: `git:<commit_sha>|sha256:<manifest_hash>|cand_<ulid>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CandidateId(String);

impl CandidateId {
    pub fn new(git_sha: Option<&str>, content_hash: &str) -> Self {
        let ulid = Ulid::new();
        let mut parts = Vec::new();
        if let Some(sha) = git_sha {
            parts.push(format!("git:{sha}"));
        }
        parts.push(format!("sha256:{content_hash}"));
        parts.push(format!("cand_{ulid}"));
        Self(parts.join("|"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Content hash per SR-SPEC §1.3.2
/// Format: `sha256:<64-hex>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn new(hex_digest: &str) -> Self {
        Self(format!("sha256:{hex_digest}"))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Loop state per SR-SPEC §3.1.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoopState {
    Created,
    Active,
    Paused,
    Closed,
}

/// Work Unit (Loop) entity per SR-TYPES `domain.work_unit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnit {
    pub id: LoopId,
    pub goal: String,
    pub state: LoopState,
    pub created_at: DateTime<Utc>,
    pub created_by: ActorId,
}

/// Typed reference per SR-SPEC §1.5.3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedRef {
    pub kind: String,
    pub id: String,
    pub rel: String,
    #[serde(default)]
    pub meta: serde_json::Value,
}

// ============================================================================
// Iteration Entity
// ============================================================================

/// Iteration state per SR-SPEC §3.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IterationState {
    Started,
    Running,
    Completed,
    Failed,
}

/// Iteration entity - a fresh-context execution cycle within a Work Unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iteration {
    pub id: IterationId,
    pub loop_id: LoopId,
    pub state: IterationState,
    pub sequence: u32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub refs: Vec<TypedRef>,
}

// ============================================================================
// Candidate Entity
// ============================================================================

/// Candidate verification status per SR-SPEC §3.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    Unverified,
    VerifiedStrict,
    VerifiedWithExceptions,
}

/// Candidate entity - a content-addressable snapshot of work products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: CandidateId,
    pub content_hash: ContentHash,
    pub produced_by_iteration_id: Option<IterationId>,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
    pub refs: Vec<TypedRef>,
}

// ============================================================================
// Run and Evidence Entities
// ============================================================================

/// Run identifier: `run_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(String);

impl RunId {
    pub fn new() -> Self {
        Self(format!("run_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

/// Run state per SR-SPEC §3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunState {
    Started,
    Running,
    Completed,
    Failed,
}

/// Run entity - execution of an oracle suite against a Candidate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: RunId,
    pub candidate_id: CandidateId,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: ContentHash,
    pub state: RunState,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub actor: ActorId,
}

/// Evidence Bundle entity per SR-SPEC §1.9
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub content_hash: ContentHash,
    pub run_id: RunId,
    pub candidate_id: CandidateId,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: ContentHash,
    pub produced_at: DateTime<Utc>,
    pub results: Vec<OracleResultRecord>,
}

/// Individual oracle result within an Evidence Bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResultRecord {
    pub oracle_id: String,
    pub oracle_name: String,
    pub classification: OracleClassification,
    pub status: OracleStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub artifact_refs: Vec<ContentHash>,
}

/// Oracle classification per SR-SPEC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OracleClassification {
    Required,
    Advisory,
}

/// Oracle status per SR-SPEC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OracleStatus {
    Pass,
    Fail,
    Error,
    Skipped,
}

// ============================================================================
// Approval Entity
// ============================================================================

/// Approval identifier: `appr_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApprovalId(String);

impl ApprovalId {
    pub fn new() -> Self {
        Self(format!("appr_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ApprovalId {
    fn default() -> Self {
        Self::new()
    }
}

/// Approval decision type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    Deferred,
}

/// Approval entity - binding human decision at a Portal per SR-SPEC §1.2.2
/// Invariant: actor.kind MUST be HUMAN (C-TB-3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub id: ApprovalId,
    pub portal_id: String,
    pub decision: ApprovalDecision,
    pub subject_refs: Vec<TypedRef>,
    pub evidence_refs: Vec<ContentHash>,
    pub exceptions_acknowledged: Vec<ExceptionId>,
    pub rationale: Option<String>,
    pub approved_by: ActorId,
    pub approved_at: DateTime<Utc>,
}

// ============================================================================
// Freeze Entity
// ============================================================================

/// Freeze identifier: `freeze_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FreezeId(String);

impl FreezeId {
    pub fn new() -> Self {
        Self(format!("freeze_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for FreezeId {
    fn default() -> Self {
        Self::new()
    }
}

/// Verification mode for freeze records
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationMode {
    Strict,
    WithExceptions,
}

/// Artifact manifest entry in a freeze record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifestEntry {
    pub artifact_id: String,
    pub version: String,
    pub content_hash: ContentHash,
}

/// Active exception entry in a freeze record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveExceptionEntry {
    pub exception_id: ExceptionId,
    pub kind: ExceptionKind,
    pub status: ExceptionStatus,
}

/// Freeze Record entity - binding baseline snapshot per SR-SPEC §1.12
/// Invariant: frozen_by.kind MUST be HUMAN (C-SHIP-1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreezeRecord {
    pub id: FreezeId,
    pub baseline_id: String,
    pub candidate_id: CandidateId,
    pub verification_mode: VerificationMode,
    pub oracle_suite_id: String,
    pub oracle_suite_hash: ContentHash,
    pub evidence_bundle_refs: Vec<ContentHash>,
    pub waiver_refs: Vec<ExceptionId>,
    pub release_approval_id: ApprovalId,
    pub artifact_manifest: Vec<ArtifactManifestEntry>,
    pub active_exceptions: Vec<ActiveExceptionEntry>,
    pub frozen_by: ActorId,
    pub frozen_at: DateTime<Utc>,
}

// ============================================================================
// Exception Entities (Deviation, Deferral, Waiver)
// ============================================================================

/// Exception identifier: `exc_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExceptionId(String);

impl ExceptionId {
    pub fn new() -> Self {
        Self(format!("exc_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ExceptionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Exception kind per SR-CONTRACT
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExceptionKind {
    /// Deviation: permission to deviate from a governing document
    Deviation,
    /// Deferral: permission to defer a requirement to a later phase
    Deferral,
    /// Waiver: permission to bypass a specific oracle FAIL (not integrity conditions)
    Waiver,
}

/// Exception status per SR-SPEC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExceptionStatus {
    Created,
    Active,
    Resolved,
    Expired,
}

/// Exception scope - defines what the exception applies to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionScope {
    pub loop_id: Option<LoopId>,
    pub candidate_id: Option<CandidateId>,
    pub oracle_id: Option<String>,
    pub artifact_refs: Vec<TypedRef>,
}

/// Exception entity - binding exception record per SR-CONTRACT
/// Invariant: created_by.kind MUST be HUMAN
/// Invariant: Waivers MUST NOT target integrity conditions (ORACLE_TAMPER, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    pub id: ExceptionId,
    pub kind: ExceptionKind,
    pub status: ExceptionStatus,
    pub scope: ExceptionScope,
    pub rationale: String,
    pub target_description: String,
    pub created_by: ActorId,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<ActorId>,
}

// ============================================================================
// Decision Entity
// ============================================================================

/// Decision identifier: `dec_<ULID>` per SR-SPEC §1.3.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecisionId(String);

impl DecisionId {
    pub fn new() -> Self {
        Self(format!("dec_{}", Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for DecisionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Decision entity - binding human judgment per SR-SPEC §1.11
/// Invariant: decided_by.kind MUST be HUMAN (C-DEC-1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: DecisionId,
    pub trigger: String,
    pub scope: serde_json::Value,
    pub decision: String,
    pub rationale: String,
    pub subject_refs: Vec<TypedRef>,
    pub evidence_refs: Vec<ContentHash>,
    pub exceptions_acknowledged: Vec<ExceptionId>,
    pub decided_by: ActorId,
    pub decided_at: DateTime<Utc>,
    pub is_precedent: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_id_generation() {
        let id1 = LoopId::new();
        let id2 = LoopId::new();

        assert!(id1.as_str().starts_with("loop_"));
        assert!(id2.as_str().starts_with("loop_"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_iteration_id_generation() {
        let id = IterationId::new();
        assert!(id.as_str().starts_with("iter_"));
    }

    #[test]
    fn test_candidate_id_with_git_sha() {
        let id = CandidateId::new(Some("abc123"), "deadbeef");
        let id_str = id.as_str();

        assert!(id_str.contains("git:abc123"));
        assert!(id_str.contains("sha256:deadbeef"));
        assert!(id_str.contains("cand_"));
    }

    #[test]
    fn test_candidate_id_without_git_sha() {
        let id = CandidateId::new(None, "deadbeef");
        let id_str = id.as_str();

        assert!(!id_str.contains("git:"));
        assert!(id_str.contains("sha256:deadbeef"));
        assert!(id_str.contains("cand_"));
    }

    #[test]
    fn test_content_hash_format() {
        let hash = ContentHash::new("abcdef1234567890");
        assert_eq!(hash.as_str(), "sha256:abcdef1234567890");
    }

    #[test]
    fn test_run_id_generation() {
        let id = RunId::new();
        assert!(id.as_str().starts_with("run_"));
    }

    #[test]
    fn test_approval_id_generation() {
        let id = ApprovalId::new();
        assert!(id.as_str().starts_with("appr_"));
    }

    #[test]
    fn test_freeze_id_generation() {
        let id = FreezeId::new();
        assert!(id.as_str().starts_with("freeze_"));
    }

    #[test]
    fn test_exception_id_generation() {
        let id = ExceptionId::new();
        assert!(id.as_str().starts_with("exc_"));
    }

    #[test]
    fn test_decision_id_generation() {
        let id = DecisionId::new();
        assert!(id.as_str().starts_with("dec_"));
    }

    #[test]
    fn test_actor_id_serialization() {
        let actor = ActorId {
            kind: ActorKind::Human,
            id: "oidc_sub:abc:user123".to_string(),
        };

        let json = serde_json::to_string(&actor).unwrap();
        let parsed: ActorId = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.kind, ActorKind::Human);
        assert_eq!(parsed.id, "oidc_sub:abc:user123");
    }

    #[test]
    fn test_loop_state_serialization() {
        assert_eq!(
            serde_json::to_string(&LoopState::Created).unwrap(),
            "\"CREATED\""
        );
        assert_eq!(
            serde_json::to_string(&LoopState::Active).unwrap(),
            "\"ACTIVE\""
        );
    }

    #[test]
    fn test_verification_status_serialization() {
        assert_eq!(
            serde_json::to_string(&VerificationStatus::Unverified).unwrap(),
            "\"UNVERIFIED\""
        );
        assert_eq!(
            serde_json::to_string(&VerificationStatus::VerifiedStrict).unwrap(),
            "\"VERIFIED_STRICT\""
        );
        assert_eq!(
            serde_json::to_string(&VerificationStatus::VerifiedWithExceptions).unwrap(),
            "\"VERIFIED_WITH_EXCEPTIONS\""
        );
    }

    #[test]
    fn test_exception_kind_serialization() {
        assert_eq!(
            serde_json::to_string(&ExceptionKind::Deviation).unwrap(),
            "\"DEVIATION\""
        );
        assert_eq!(
            serde_json::to_string(&ExceptionKind::Deferral).unwrap(),
            "\"DEFERRAL\""
        );
        assert_eq!(
            serde_json::to_string(&ExceptionKind::Waiver).unwrap(),
            "\"WAIVER\""
        );
    }

    #[test]
    fn test_typed_ref_with_meta() {
        let typed_ref = TypedRef {
            kind: "GovernedArtifact".to_string(),
            id: "SR-CONTRACT".to_string(),
            rel: "depends_on".to_string(),
            meta: serde_json::json!({
                "version": "1.0.0",
                "content_hash": "sha256:abc123"
            }),
        };

        let json = serde_json::to_string(&typed_ref).unwrap();
        let parsed: TypedRef = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.kind, "GovernedArtifact");
        assert_eq!(parsed.meta["version"], "1.0.0");
    }

    #[test]
    fn test_default_loop_budgets() {
        use crate::commands::LoopBudgets;

        let budgets = LoopBudgets::default();

        assert_eq!(budgets.max_iterations, 5);
        assert_eq!(budgets.max_oracle_runs, 25);
        assert_eq!(budgets.max_wallclock_hours, 16);
    }
}
