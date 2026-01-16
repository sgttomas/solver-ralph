//! Strongly-typed reference schema per SR-SPEC §1.5.3
//!
//! This module provides the canonical TypedRef schema used throughout SOLVER-Ralph
//! for all reference types: iteration context refs, intake inputs, evidence refs, etc.
//!
//! Per SR-PLAN-V3 §1.1, this unifies the previous separate InputRef and TypedRef
//! into a single strongly-typed schema.

use serde::{Deserialize, Serialize};

// ============================================================================
// RefKind — Reference type taxonomy per SR-SPEC §1.5.3
// ============================================================================

/// Reference kinds per SR-SPEC §1.5.3
///
/// Categorizes what type of artifact or entity a reference points to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RefKind {
    // Core domain entities
    GovernedArtifact,
    Candidate,
    OracleSuite,
    EvidenceBundle,
    Approval,
    Record,
    Decision,

    // Exception types
    Deviation,
    Deferral,
    Waiver,

    // Loop/iteration entities
    Loop,
    Iteration,
    Run,
    Freeze,

    // Work surface entities
    Intake,
    ProcedureTemplate,
    ProcedureStage,
    SemanticSet,
    WorkSurface,

    // Configuration artifacts (per SR-PLAN-V3 Issue #10)
    AgentDefinition,
    GatingPolicy,
}

impl RefKind {
    /// Returns true if this ref kind is dereferenceable (has content)
    pub fn is_dereferenceable(&self) -> bool {
        matches!(
            self,
            RefKind::GovernedArtifact
                | RefKind::Candidate
                | RefKind::EvidenceBundle
                | RefKind::Intake
                | RefKind::ProcedureTemplate
                | RefKind::SemanticSet
                | RefKind::AgentDefinition
                | RefKind::GatingPolicy
        )
    }

    /// Returns true if this ref kind requires content_hash in meta
    pub fn requires_content_hash(&self) -> bool {
        self.is_dereferenceable()
    }

    /// Returns true if this ref kind requires version in meta
    pub fn requires_version(&self) -> bool {
        matches!(self, RefKind::GovernedArtifact)
    }

    /// Returns true if this ref kind requires type_key in meta
    pub fn requires_type_key(&self) -> bool {
        matches!(self, RefKind::Record | RefKind::GovernedArtifact)
    }
}

// ============================================================================
// RefRelation — Relationship types per SR-SPEC §1.5.3
// ============================================================================

/// Relationship types per SR-SPEC §1.5.3
///
/// Describes how the referencing entity relates to the referenced entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefRelation {
    /// The entity is about this reference (primary subject)
    About,
    /// The entity depends on this reference (input/prerequisite)
    DependsOn,
    /// The entity is supported by this reference (evidence)
    SupportedBy,
    /// The entity produces this reference (output)
    Produces,
    /// The entity verifies this reference (oracle)
    Verifies,
    /// The entity was approved by this reference (approval)
    ApprovedBy,
    /// The entity acknowledges this reference (exception)
    Acknowledges,
    /// The entity supersedes this reference (versioning)
    Supersedes,
    /// The entity releases this reference (freeze)
    Releases,
    /// The entity is governed by this reference (governance)
    GovernedBy,
    /// The entity is in scope of this reference (scoping)
    InScopeOf,
    /// The entity affects this reference (staleness)
    Affects,
    /// The entity is stale relative to this reference
    Stale,
    /// The entity is the root cause of this reference (analysis)
    RootCause,
    /// The entity relates to this reference (general association)
    RelatesTo,
}

impl RefRelation {
    /// Returns true if this relation implies a dependency
    pub fn is_dependency(&self) -> bool {
        matches!(
            self,
            RefRelation::DependsOn | RefRelation::GovernedBy | RefRelation::InScopeOf
        )
    }

    /// Returns true if this relation implies output/production
    pub fn is_production(&self) -> bool {
        matches!(
            self,
            RefRelation::Produces | RefRelation::Releases | RefRelation::Supersedes
        )
    }
}

// ============================================================================
// RefMeta — Reference metadata per SR-SPEC §1.5.3.1
// ============================================================================

/// Reference metadata per SR-SPEC §1.5.3.1
///
/// Contains optional metadata fields depending on the RefKind.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RefMeta {
    /// Content hash (REQUIRED for dereferenceable refs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,

    /// Version (REQUIRED for GovernedArtifact)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Type key (REQUIRED for Record refs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_key: Option<String>,

    /// Selector for stable slice (recommended for large artifacts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,

    /// Current stage (for ProcedureTemplate refs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage_id: Option<String>,

    /// Suite hash (for OracleSuite refs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite_hash: Option<String>,
}

impl RefMeta {
    /// Create an empty RefMeta
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create RefMeta with just content_hash
    pub fn with_hash(content_hash: impl Into<String>) -> Self {
        Self {
            content_hash: Some(content_hash.into()),
            ..Default::default()
        }
    }

    /// Create RefMeta for a governed artifact
    pub fn for_governed_artifact(
        content_hash: impl Into<String>,
        version: impl Into<String>,
        type_key: impl Into<String>,
    ) -> Self {
        Self {
            content_hash: Some(content_hash.into()),
            version: Some(version.into()),
            type_key: Some(type_key.into()),
            ..Default::default()
        }
    }

    /// Create RefMeta for an oracle suite
    pub fn for_oracle_suite(suite_hash: impl Into<String>) -> Self {
        let hash = suite_hash.into();
        Self {
            content_hash: Some(hash.clone()),
            suite_hash: Some(hash),
            ..Default::default()
        }
    }

    /// Create RefMeta for a procedure template with stage
    pub fn for_procedure_template(
        content_hash: impl Into<String>,
        current_stage_id: impl Into<String>,
    ) -> Self {
        Self {
            content_hash: Some(content_hash.into()),
            current_stage_id: Some(current_stage_id.into()),
            ..Default::default()
        }
    }
}

// ============================================================================
// StrongTypedRef — Strongly-typed reference per SR-SPEC §1.5.3
// ============================================================================

/// Strongly-typed reference per SR-SPEC §1.5.3
///
/// This is the canonical reference schema used throughout SOLVER-Ralph.
/// It replaces the previous loosely-typed `TypedRef` with enum-based
/// kind and relation fields for compile-time safety.
///
/// Per SR-PLAN-V3, this unifies InputRef and TypedRef into a single schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrongTypedRef {
    /// Reference kind (what type of entity is referenced)
    pub kind: RefKind,

    /// Stable identifier of the referenced entity
    pub id: String,

    /// Relationship type (how the referencing entity relates)
    pub rel: RefRelation,

    /// Metadata (content_hash, version, etc.)
    #[serde(default)]
    pub meta: RefMeta,

    /// Human-readable label (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl StrongTypedRef {
    /// Create a new StrongTypedRef
    pub fn new(kind: RefKind, id: impl Into<String>, rel: RefRelation) -> Self {
        Self {
            kind,
            id: id.into(),
            rel,
            meta: RefMeta::default(),
            label: None,
        }
    }

    /// Create with metadata
    pub fn with_meta(
        kind: RefKind,
        id: impl Into<String>,
        rel: RefRelation,
        meta: RefMeta,
    ) -> Self {
        Self {
            kind,
            id: id.into(),
            rel,
            meta,
            label: None,
        }
    }

    /// Add a label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Validate the reference per SR-SPEC §1.5.3 requirements
    pub fn validate(&self) -> Result<(), RefValidationError> {
        // Check required content_hash
        if self.kind.requires_content_hash() && self.meta.content_hash.is_none() {
            return Err(RefValidationError::MissingContentHash {
                kind: self.kind,
                id: self.id.clone(),
            });
        }

        // Check required version
        if self.kind.requires_version() && self.meta.version.is_none() {
            return Err(RefValidationError::MissingVersion {
                kind: self.kind,
                id: self.id.clone(),
            });
        }

        // Check required type_key
        if self.kind.requires_type_key() && self.meta.type_key.is_none() {
            return Err(RefValidationError::MissingTypeKey {
                kind: self.kind,
                id: self.id.clone(),
            });
        }

        // Check ID is not empty
        if self.id.is_empty() {
            return Err(RefValidationError::EmptyId { kind: self.kind });
        }

        Ok(())
    }

    // === Convenience constructors ===

    /// Create a reference to a governed artifact
    pub fn governed_artifact(
        id: impl Into<String>,
        rel: RefRelation,
        content_hash: impl Into<String>,
        version: impl Into<String>,
        type_key: impl Into<String>,
    ) -> Self {
        Self::with_meta(
            RefKind::GovernedArtifact,
            id,
            rel,
            RefMeta::for_governed_artifact(content_hash, version, type_key),
        )
    }

    /// Create a reference to an intake
    pub fn intake(
        id: impl Into<String>,
        rel: RefRelation,
        content_hash: impl Into<String>,
    ) -> Self {
        Self::with_meta(RefKind::Intake, id, rel, RefMeta::with_hash(content_hash))
    }

    /// Create a reference to a procedure template
    pub fn procedure_template(
        id: impl Into<String>,
        rel: RefRelation,
        content_hash: impl Into<String>,
        current_stage_id: impl Into<String>,
    ) -> Self {
        Self::with_meta(
            RefKind::ProcedureTemplate,
            id,
            rel,
            RefMeta::for_procedure_template(content_hash, current_stage_id),
        )
    }

    /// Create a reference to an oracle suite
    pub fn oracle_suite(
        id: impl Into<String>,
        rel: RefRelation,
        suite_hash: impl Into<String>,
    ) -> Self {
        Self::with_meta(
            RefKind::OracleSuite,
            id,
            rel,
            RefMeta::for_oracle_suite(suite_hash),
        )
    }

    /// Create a reference to a candidate
    pub fn candidate(
        id: impl Into<String>,
        rel: RefRelation,
        content_hash: impl Into<String>,
    ) -> Self {
        Self::with_meta(
            RefKind::Candidate,
            id,
            rel,
            RefMeta::with_hash(content_hash),
        )
    }

    /// Create a reference to an evidence bundle
    pub fn evidence_bundle(
        id: impl Into<String>,
        rel: RefRelation,
        content_hash: impl Into<String>,
    ) -> Self {
        Self::with_meta(
            RefKind::EvidenceBundle,
            id,
            rel,
            RefMeta::with_hash(content_hash),
        )
    }

    /// Create a reference to an approval (no content hash required)
    pub fn approval(id: impl Into<String>, rel: RefRelation) -> Self {
        Self::new(RefKind::Approval, id, rel)
    }

    /// Create a reference to a loop (no content hash required)
    pub fn loop_ref(id: impl Into<String>, rel: RefRelation) -> Self {
        Self::new(RefKind::Loop, id, rel)
    }

    /// Create a reference to an iteration (no content hash required)
    pub fn iteration(id: impl Into<String>, rel: RefRelation) -> Self {
        Self::new(RefKind::Iteration, id, rel)
    }
}

// ============================================================================
// Validation errors
// ============================================================================

/// Errors from reference validation
#[derive(Debug, Clone, PartialEq)]
pub enum RefValidationError {
    MissingContentHash { kind: RefKind, id: String },
    MissingVersion { kind: RefKind, id: String },
    MissingTypeKey { kind: RefKind, id: String },
    EmptyId { kind: RefKind },
}

impl std::fmt::Display for RefValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingContentHash { kind, id } => {
                write!(f, "{kind:?} ref '{id}' requires content_hash in meta")
            }
            Self::MissingVersion { kind, id } => {
                write!(f, "{kind:?} ref '{id}' requires version in meta")
            }
            Self::MissingTypeKey { kind, id } => {
                write!(f, "{kind:?} ref '{id}' requires type_key in meta")
            }
            Self::EmptyId { kind } => {
                write!(f, "{kind:?} ref has empty id")
            }
        }
    }
}

impl std::error::Error for RefValidationError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_kind_serialization() {
        assert_eq!(
            serde_json::to_string(&RefKind::GovernedArtifact).unwrap(),
            "\"GovernedArtifact\""
        );
        assert_eq!(
            serde_json::to_string(&RefKind::Intake).unwrap(),
            "\"Intake\""
        );
        assert_eq!(
            serde_json::to_string(&RefKind::AgentDefinition).unwrap(),
            "\"AgentDefinition\""
        );
    }

    #[test]
    fn test_ref_relation_serialization() {
        assert_eq!(
            serde_json::to_string(&RefRelation::DependsOn).unwrap(),
            "\"depends_on\""
        );
        assert_eq!(
            serde_json::to_string(&RefRelation::GovernedBy).unwrap(),
            "\"governed_by\""
        );
        assert_eq!(
            serde_json::to_string(&RefRelation::Supersedes).unwrap(),
            "\"supersedes\""
        );
    }

    #[test]
    fn test_strong_typed_ref_creation() {
        let r = StrongTypedRef::intake("intake:01ABC123", RefRelation::DependsOn, "sha256:abc123");

        assert_eq!(r.kind, RefKind::Intake);
        assert_eq!(r.id, "intake:01ABC123");
        assert_eq!(r.rel, RefRelation::DependsOn);
        assert_eq!(r.meta.content_hash, Some("sha256:abc123".to_string()));
        assert!(r.validate().is_ok());
    }

    #[test]
    fn test_governed_artifact_ref() {
        let r = StrongTypedRef::governed_artifact(
            "SR-CONTRACT",
            RefRelation::GovernedBy,
            "sha256:abc123",
            "1.0.0",
            "governance.contract",
        );

        assert_eq!(r.kind, RefKind::GovernedArtifact);
        assert_eq!(r.meta.version, Some("1.0.0".to_string()));
        assert_eq!(r.meta.type_key, Some("governance.contract".to_string()));
        assert!(r.validate().is_ok());
    }

    #[test]
    fn test_validation_missing_content_hash() {
        let r = StrongTypedRef::new(RefKind::Intake, "intake:01ABC123", RefRelation::DependsOn);

        let result = r.validate();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(RefValidationError::MissingContentHash { .. })
        ));
    }

    #[test]
    fn test_validation_missing_version() {
        let r = StrongTypedRef::with_meta(
            RefKind::GovernedArtifact,
            "SR-CONTRACT",
            RefRelation::GovernedBy,
            RefMeta::with_hash("sha256:abc123"),
        );

        let result = r.validate();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(RefValidationError::MissingVersion { .. })
        ));
    }

    #[test]
    fn test_ref_without_content_hash_requirement() {
        // Approval refs don't require content_hash
        let r = StrongTypedRef::approval("appr_01ABC123", RefRelation::ApprovedBy);
        assert!(r.validate().is_ok());

        // Loop refs don't require content_hash
        let r = StrongTypedRef::loop_ref("loop_01ABC123", RefRelation::InScopeOf);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn test_procedure_template_ref() {
        let r = StrongTypedRef::procedure_template(
            "proc:RESEARCH-MEMO",
            RefRelation::DependsOn,
            "sha256:def456",
            "stage:FRAME",
        );

        assert_eq!(r.kind, RefKind::ProcedureTemplate);
        assert_eq!(r.meta.current_stage_id, Some("stage:FRAME".to_string()));
        assert!(r.validate().is_ok());
    }

    #[test]
    fn test_oracle_suite_ref() {
        let r = StrongTypedRef::oracle_suite(
            "suite:SR-SUITE-CORE",
            RefRelation::DependsOn,
            "sha256:ghi789",
        );

        assert_eq!(r.kind, RefKind::OracleSuite);
        assert_eq!(r.meta.suite_hash, Some("sha256:ghi789".to_string()));
        assert!(r.validate().is_ok());
    }

    #[test]
    fn test_ref_with_label() {
        let r = StrongTypedRef::intake("intake:01ABC123", RefRelation::DependsOn, "sha256:abc123")
            .with_label("API Rate Limiting Analysis");

        assert_eq!(r.label, Some("API Rate Limiting Analysis".to_string()));
    }

    #[test]
    fn test_ref_kind_requirements() {
        assert!(RefKind::GovernedArtifact.requires_content_hash());
        assert!(RefKind::GovernedArtifact.requires_version());
        assert!(RefKind::GovernedArtifact.requires_type_key());

        assert!(RefKind::Intake.requires_content_hash());
        assert!(!RefKind::Intake.requires_version());
        assert!(!RefKind::Intake.requires_type_key());

        assert!(!RefKind::Approval.requires_content_hash());
        assert!(!RefKind::Loop.requires_content_hash());
    }

    #[test]
    fn test_ref_relation_properties() {
        assert!(RefRelation::DependsOn.is_dependency());
        assert!(RefRelation::GovernedBy.is_dependency());
        assert!(!RefRelation::Produces.is_dependency());

        assert!(RefRelation::Produces.is_production());
        assert!(RefRelation::Supersedes.is_production());
        assert!(!RefRelation::DependsOn.is_production());
    }

    #[test]
    fn test_full_serialization_roundtrip() {
        let original = StrongTypedRef::governed_artifact(
            "SR-CONTRACT",
            RefRelation::GovernedBy,
            "sha256:abc123def456",
            "1.0.0",
            "governance.contract",
        )
        .with_label("Architectural Contract");

        let json = serde_json::to_string(&original).unwrap();
        let parsed: StrongTypedRef = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
    }
}
