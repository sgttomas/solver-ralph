//! Intake lifecycle and events per SR-WORK-SURFACE §3 and SR-PLAN-V3
//!
//! This module extends the Intake schema from work_surface.rs with:
//! - IntakeStatus enum for lifecycle management
//! - Intake lifecycle events (Created, Updated, Activated, Archived, Forked)
//! - IntakeId generation using ULID
//!
//! Per SR-PLAN-V3 §1.2, IntakeStatus maps to SR-TYPES §3.1:
//! - Draft = draft (proposal, editable)
//! - Active = governed (commitment object, immutable)
//! - Archived = archived (superseded, read-only)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use ulid::Ulid;

use crate::entities::ActorId;
use crate::refs::StrongTypedRef;
use crate::work_surface::{Deliverable, WorkKind, WorkUnitId};

// ============================================================================
// IntakeId — ULID-based identifier per SR-PLAN-V3
// ============================================================================

/// Intake identifier per SR-PLAN-V3 §1.3
/// Format: `intake:<ULID>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntakeUlidId(String);

impl IntakeUlidId {
    /// Create a new IntakeId with a fresh ULID
    pub fn new() -> Self {
        Self(format!("intake:{}", Ulid::new()))
    }

    /// Create from an existing string
    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate the format
    pub fn is_valid(&self) -> bool {
        self.0.starts_with("intake:") && self.0.len() > 7
    }
}

impl Default for IntakeUlidId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for IntakeUlidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// IntakeStatus — Lifecycle states per SR-PLAN-V3 §1.2
// ============================================================================

/// Intake lifecycle status per SR-PLAN-V3 §1.2
///
/// Maps to SR-TYPES §3.1 `status` enum:
/// - Draft = draft (proposal, editable)
/// - Active = governed (commitment object, immutable)
/// - Archived = archived (superseded, read-only)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntakeStatus {
    /// Proposal - editable, not yet a commitment object
    #[default]
    Draft,
    /// Commitment Object - immutable, content-addressed
    Active,
    /// Superseded - historical, read-only
    Archived,
}

impl IntakeStatus {
    /// Map to SR-TYPES §3.1 status enum
    pub fn to_sr_types_status(&self) -> &'static str {
        match self {
            IntakeStatus::Draft => "draft",
            IntakeStatus::Active => "governed",
            IntakeStatus::Archived => "archived",
        }
    }

    /// Check if the intake can be edited
    pub fn is_editable(&self) -> bool {
        matches!(self, IntakeStatus::Draft)
    }

    /// Check if the intake can be activated
    pub fn can_activate(&self) -> bool {
        matches!(self, IntakeStatus::Draft)
    }

    /// Check if the intake can be archived
    pub fn can_archive(&self) -> bool {
        matches!(self, IntakeStatus::Active)
    }

    /// Check if the intake can be forked
    pub fn can_fork(&self) -> bool {
        matches!(self, IntakeStatus::Active | IntakeStatus::Archived)
    }
}

impl std::fmt::Display for IntakeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntakeStatus::Draft => write!(f, "draft"),
            IntakeStatus::Active => write!(f, "active"),
            IntakeStatus::Archived => write!(f, "archived"),
        }
    }
}

// ============================================================================
// ManagedIntake — Intake with full lifecycle support
// ============================================================================

/// Managed Intake with full lifecycle support per SR-PLAN-V3 §1.3
///
/// This extends the base Intake schema with:
/// - ULID-based ID
/// - Status lifecycle
/// - Version tracking
/// - Activation timestamps
/// - Uses StrongTypedRef for inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedIntake {
    // === Identity ===
    /// Unique identifier (format: "intake:<ULID>")
    pub intake_id: IntakeUlidId,

    /// Work unit this intake belongs to
    pub work_unit_id: WorkUnitId,

    /// Content hash - computed on activation (format: "sha256:<hex>")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,

    // === Required fields per SR-WORK-SURFACE §3.1 ===
    pub title: String,
    pub kind: WorkKind,
    /// ONE sentence objective
    pub objective: String,
    pub audience: String,
    pub deliverables: Vec<Deliverable>,
    pub constraints: Vec<String>,
    pub definitions: HashMap<String, String>,
    /// Input references using StrongTypedRef
    pub inputs: Vec<StrongTypedRef>,
    pub unknowns: Vec<String>,
    pub completion_criteria: Vec<String>,

    // === Lifecycle ===
    pub status: IntakeStatus,
    /// Increments on fork
    pub version: u32,
    /// intake_id of prior version (if forked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<IntakeUlidId>,

    // === Attribution (per C-EVT-1) ===
    pub created_at: DateTime<Utc>,
    pub created_by: ActorId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_by: Option<ActorId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_by: Option<ActorId>,
}

impl ManagedIntake {
    /// Create a new draft intake
    pub fn new_draft(
        work_unit_id: WorkUnitId,
        title: String,
        kind: WorkKind,
        objective: String,
        audience: String,
        deliverables: Vec<Deliverable>,
        created_by: ActorId,
    ) -> Self {
        Self {
            intake_id: IntakeUlidId::new(),
            work_unit_id,
            content_hash: None,
            title,
            kind,
            objective,
            audience,
            deliverables,
            constraints: Vec::new(),
            definitions: HashMap::new(),
            inputs: Vec::new(),
            unknowns: Vec::new(),
            completion_criteria: Vec::new(),
            status: IntakeStatus::Draft,
            version: 1,
            supersedes: None,
            created_at: Utc::now(),
            created_by,
            activated_at: None,
            activated_by: None,
            archived_at: None,
            archived_by: None,
        }
    }

    /// Compute the content hash for this intake
    ///
    /// Uses canonical JSON serialization (sorted keys) for determinism.
    pub fn compute_content_hash(&self) -> String {
        // Build canonical representation (only content fields, not lifecycle)
        let canonical = serde_json::json!({
            "work_unit_id": self.work_unit_id.as_str(),
            "title": self.title,
            "kind": self.kind,
            "objective": self.objective,
            "audience": self.audience,
            "deliverables": self.deliverables,
            "constraints": self.constraints,
            "definitions": self.definitions,
            "inputs": self.inputs,
            "unknowns": self.unknowns,
            "completion_criteria": self.completion_criteria,
            "version": self.version,
        });

        // Serialize with sorted keys for determinism
        let canonical_str = serde_json::to_string(&canonical).unwrap_or_default();

        // Compute SHA-256
        let mut hasher = Sha256::new();
        hasher.update(canonical_str.as_bytes());
        format!("sha256:{}", hex::encode(hasher.finalize()))
    }

    /// Activate this intake (transition to commitment object)
    ///
    /// Returns error if not in Draft status.
    pub fn activate(&mut self, actor: ActorId) -> Result<String, IntakeLifecycleError> {
        if !self.status.can_activate() {
            return Err(IntakeLifecycleError::InvalidTransition {
                from: self.status,
                to: IntakeStatus::Active,
                reason: "Can only activate from Draft status".to_string(),
            });
        }

        let content_hash = self.compute_content_hash();
        self.content_hash = Some(content_hash.clone());
        self.status = IntakeStatus::Active;
        self.activated_at = Some(Utc::now());
        self.activated_by = Some(actor);

        Ok(content_hash)
    }

    /// Archive this intake
    ///
    /// Returns error if not in Active status.
    pub fn archive(&mut self, actor: ActorId) -> Result<(), IntakeLifecycleError> {
        if !self.status.can_archive() {
            return Err(IntakeLifecycleError::InvalidTransition {
                from: self.status,
                to: IntakeStatus::Archived,
                reason: "Can only archive from Active status".to_string(),
            });
        }

        self.status = IntakeStatus::Archived;
        self.archived_at = Some(Utc::now());
        self.archived_by = Some(actor);

        Ok(())
    }

    /// Fork this intake to create a new draft
    ///
    /// Returns error if not in Active or Archived status.
    pub fn fork(&self, actor: ActorId) -> Result<ManagedIntake, IntakeLifecycleError> {
        if !self.status.can_fork() {
            return Err(IntakeLifecycleError::InvalidTransition {
                from: self.status,
                to: IntakeStatus::Draft,
                reason: "Can only fork from Active or Archived status".to_string(),
            });
        }

        Ok(ManagedIntake {
            intake_id: IntakeUlidId::new(),
            work_unit_id: self.work_unit_id.clone(),
            content_hash: None,
            title: self.title.clone(),
            kind: self.kind.clone(),
            objective: self.objective.clone(),
            audience: self.audience.clone(),
            deliverables: self.deliverables.clone(),
            constraints: self.constraints.clone(),
            definitions: self.definitions.clone(),
            inputs: self.inputs.clone(),
            unknowns: self.unknowns.clone(),
            completion_criteria: self.completion_criteria.clone(),
            status: IntakeStatus::Draft,
            version: self.version + 1,
            supersedes: Some(self.intake_id.clone()),
            created_at: Utc::now(),
            created_by: actor,
            activated_at: None,
            activated_by: None,
            archived_at: None,
            archived_by: None,
        })
    }
}

// ============================================================================
// Lifecycle errors
// ============================================================================

/// Errors from intake lifecycle operations
#[derive(Debug, Clone, PartialEq)]
pub enum IntakeLifecycleError {
    InvalidTransition {
        from: IntakeStatus,
        to: IntakeStatus,
        reason: String,
    },
    ValidationFailed {
        field: String,
        reason: String,
    },
}

impl std::fmt::Display for IntakeLifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTransition { from, to, reason } => {
                write!(f, "Invalid transition from {from} to {to}: {reason}")
            }
            Self::ValidationFailed { field, reason } => {
                write!(f, "Validation failed for {field}: {reason}")
            }
        }
    }
}

impl std::error::Error for IntakeLifecycleError {}

// ============================================================================
// Intake Events per SR-PLAN-V3 §1.6
// ============================================================================

/// Event emitted when a new intake is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeCreated {
    pub intake_id: String,
    pub work_unit_id: String,
    pub title: String,
    pub kind: WorkKind,
    pub objective: String,
    pub audience: String,
    pub deliverables: Vec<Deliverable>,
    pub constraints: Vec<String>,
    pub definitions: HashMap<String, String>,
    pub inputs: Vec<StrongTypedRef>,
    pub unknowns: Vec<String>,
    pub completion_criteria: Vec<String>,
}

impl IntakeCreated {
    pub fn stream_id(&self) -> String {
        format!("intake:{}", self.intake_id)
    }

    pub fn stream_kind() -> &'static str {
        "INTAKE"
    }

    pub fn event_type() -> &'static str {
        "IntakeCreated"
    }

    /// Create from a ManagedIntake
    pub fn from_intake(intake: &ManagedIntake) -> Self {
        Self {
            intake_id: intake.intake_id.as_str().to_string(),
            work_unit_id: intake.work_unit_id.as_str().to_string(),
            title: intake.title.clone(),
            kind: intake.kind.clone(),
            objective: intake.objective.clone(),
            audience: intake.audience.clone(),
            deliverables: intake.deliverables.clone(),
            constraints: intake.constraints.clone(),
            definitions: intake.definitions.clone(),
            inputs: intake.inputs.clone(),
            unknowns: intake.unknowns.clone(),
            completion_criteria: intake.completion_criteria.clone(),
        }
    }
}

/// Event emitted when a draft intake is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeUpdated {
    pub intake_id: String,
    /// Fields that changed (delta representation)
    pub changes: IntakeChanges,
}

/// Delta representation of intake changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntakeChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverables: Option<Vec<Deliverable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<StrongTypedRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknowns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_criteria: Option<Vec<String>>,
}

impl IntakeUpdated {
    pub fn stream_id(&self) -> String {
        format!("intake:{}", self.intake_id)
    }

    pub fn stream_kind() -> &'static str {
        "INTAKE"
    }

    pub fn event_type() -> &'static str {
        "IntakeUpdated"
    }
}

/// Event emitted when intake transitions to Active (commitment object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeActivated {
    pub intake_id: String,
    /// Content hash computed during activation
    pub content_hash: String,
    /// Canonical JSON hash used to compute content_hash (for auditability)
    pub canonical_json_hash: String,
}

impl IntakeActivated {
    pub fn stream_id(&self) -> String {
        format!("intake:{}", self.intake_id)
    }

    pub fn stream_kind() -> &'static str {
        "INTAKE"
    }

    pub fn event_type() -> &'static str {
        "IntakeActivated"
    }
}

/// Event emitted when an active intake is archived
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeArchived {
    pub intake_id: String,
    /// Reason for archiving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl IntakeArchived {
    pub fn stream_id(&self) -> String {
        format!("intake:{}", self.intake_id)
    }

    pub fn stream_kind() -> &'static str {
        "INTAKE"
    }

    pub fn event_type() -> &'static str {
        "IntakeArchived"
    }
}

/// Event emitted when a new draft is forked from an active/archived intake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeForked {
    /// New intake ID
    pub intake_id: String,
    /// Source intake ID
    pub source_intake_id: String,
    /// Source content hash
    pub source_content_hash: String,
    /// New version number
    pub version: u32,
}

impl IntakeForked {
    pub fn stream_id(&self) -> String {
        format!("intake:{}", self.intake_id)
    }

    pub fn stream_kind() -> &'static str {
        "INTAKE"
    }

    pub fn event_type() -> &'static str {
        "IntakeForked"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::ActorKind;

    fn test_actor() -> ActorId {
        ActorId {
            kind: ActorKind::Human,
            id: "test_user".to_string(),
        }
    }

    #[test]
    fn test_intake_ulid_id_generation() {
        let id1 = IntakeUlidId::new();
        let id2 = IntakeUlidId::new();

        assert!(id1.as_str().starts_with("intake:"));
        assert!(id2.as_str().starts_with("intake:"));
        assert_ne!(id1, id2);
        assert!(id1.is_valid());
    }

    #[test]
    fn test_intake_status_serialization() {
        assert_eq!(
            serde_json::to_string(&IntakeStatus::Draft).unwrap(),
            "\"draft\""
        );
        assert_eq!(
            serde_json::to_string(&IntakeStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&IntakeStatus::Archived).unwrap(),
            "\"archived\""
        );
    }

    #[test]
    fn test_intake_status_sr_types_mapping() {
        assert_eq!(IntakeStatus::Draft.to_sr_types_status(), "draft");
        assert_eq!(IntakeStatus::Active.to_sr_types_status(), "governed");
        assert_eq!(IntakeStatus::Archived.to_sr_types_status(), "archived");
    }

    #[test]
    fn test_intake_status_transitions() {
        assert!(IntakeStatus::Draft.is_editable());
        assert!(!IntakeStatus::Active.is_editable());
        assert!(!IntakeStatus::Archived.is_editable());

        assert!(IntakeStatus::Draft.can_activate());
        assert!(!IntakeStatus::Active.can_activate());
        assert!(!IntakeStatus::Archived.can_activate());

        assert!(!IntakeStatus::Draft.can_archive());
        assert!(IntakeStatus::Active.can_archive());
        assert!(!IntakeStatus::Archived.can_archive());

        assert!(!IntakeStatus::Draft.can_fork());
        assert!(IntakeStatus::Active.can_fork());
        assert!(IntakeStatus::Archived.can_fork());
    }

    #[test]
    fn test_managed_intake_creation() {
        let intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test Intake".to_string(),
            WorkKind::ResearchMemo,
            "Test objective".to_string(),
            "Test audience".to_string(),
            vec![Deliverable {
                path: "candidate/main.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        assert!(intake.intake_id.is_valid());
        assert_eq!(intake.status, IntakeStatus::Draft);
        assert_eq!(intake.version, 1);
        assert!(intake.content_hash.is_none());
        assert!(intake.activated_at.is_none());
    }

    #[test]
    fn test_managed_intake_activation() {
        let mut intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test Intake".to_string(),
            WorkKind::ResearchMemo,
            "Test objective".to_string(),
            "Test audience".to_string(),
            vec![Deliverable {
                path: "candidate/main.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        let result = intake.activate(test_actor());
        assert!(result.is_ok());

        let content_hash = result.unwrap();
        assert!(content_hash.starts_with("sha256:"));
        assert_eq!(intake.status, IntakeStatus::Active);
        assert!(intake.content_hash.is_some());
        assert!(intake.activated_at.is_some());
        assert!(intake.activated_by.is_some());
    }

    #[test]
    fn test_managed_intake_cannot_activate_twice() {
        let mut intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        intake.activate(test_actor()).unwrap();
        let result = intake.activate(test_actor());

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(IntakeLifecycleError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn test_managed_intake_archive() {
        let mut intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        intake.activate(test_actor()).unwrap();
        let result = intake.archive(test_actor());

        assert!(result.is_ok());
        assert_eq!(intake.status, IntakeStatus::Archived);
        assert!(intake.archived_at.is_some());
    }

    #[test]
    fn test_managed_intake_cannot_archive_draft() {
        let mut intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        let result = intake.archive(test_actor());
        assert!(result.is_err());
    }

    #[test]
    fn test_managed_intake_fork() {
        let mut intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        intake.activate(test_actor()).unwrap();

        let forked = intake.fork(test_actor()).unwrap();

        assert_ne!(forked.intake_id, intake.intake_id);
        assert_eq!(forked.status, IntakeStatus::Draft);
        assert_eq!(forked.version, 2);
        assert_eq!(forked.supersedes, Some(intake.intake_id.clone()));
        assert!(forked.content_hash.is_none());
    }

    #[test]
    fn test_managed_intake_cannot_fork_draft() {
        let intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        let result = intake.fork(test_actor());
        assert!(result.is_err());
    }

    #[test]
    fn test_content_hash_determinism() {
        let intake1 = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![Deliverable {
                path: "test.md".to_string(),
                media_type: "text/markdown".to_string(),
                description: None,
                role: None,
            }],
            test_actor(),
        );

        let intake2 = ManagedIntake {
            intake_id: IntakeUlidId::new(), // Different ID
            created_at: Utc::now(),         // Different time
            ..intake1.clone()
        };

        // Content hash should be the same (based on content, not metadata)
        let hash1 = intake1.compute_content_hash();
        let hash2 = intake2.compute_content_hash();

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("sha256:"));
    }

    #[test]
    fn test_intake_created_event() {
        let intake = ManagedIntake::new_draft(
            WorkUnitId::new("test"),
            "Test".to_string(),
            WorkKind::ResearchMemo,
            "Test".to_string(),
            "Test".to_string(),
            vec![],
            test_actor(),
        );

        let event = IntakeCreated::from_intake(&intake);

        assert_eq!(event.intake_id, intake.intake_id.as_str());
        assert!(event.stream_id().starts_with("intake:"));
        assert_eq!(IntakeCreated::stream_kind(), "INTAKE");
        assert_eq!(IntakeCreated::event_type(), "IntakeCreated");
    }
}
