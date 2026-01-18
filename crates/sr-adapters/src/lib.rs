//! SOLVER-Ralph Adapter Implementations
//!
//! This crate provides adapter implementations for the ports defined in sr-ports.
//! Per SR-SPEC, these include:
//! - PostgreSQL event store adapter (D-10)
//! - Projection builder (D-11)
//! - Dependency graph projection (D-12)
//! - Outbox publisher (D-13)
//! - MinIO evidence store adapter (D-14)
//! - Evidence manifest library (D-15)
//! - Restricted evidence handling (D-16)
//! - Zitadel identity provider adapter (D-17)
//! - NATS message bus adapter (D-21)
//! - Loop governor service (D-22)
//! - Reference worker bridge (D-23)
//! - Oracle runner service (D-24)
//! - Core oracle suite implementation (D-25)
//! - Oracle integrity checks (D-27)
//! - Semantic oracle suite implementation (D-39)
//! - Event Manager: work-unit + stage projection + eligible-set computation (D-40)
//! - Reference semantic worker (D-41)
//! - MinIO attachment store adapter (SR-PLAN-V7 V7-3)
//! - PostgreSQL oracle suite registry (SR-PLAN-V8 V8-1)
//! - Oracle execution worker (SR-PLAN-V8 V8-2)

pub mod attachment_store;
pub mod candidate_store;
pub mod config;
pub mod event_manager;
pub mod evidence;
pub mod governor;
pub mod graph;
pub mod infisical;
pub mod integrity;
pub mod minio;
pub mod nats;
pub mod oracle_runner;
pub mod oracle_suite;
pub mod oracle_worker;
pub mod outbox;
pub mod postgres;
pub mod postgres_oracle_registry;
pub mod projections;
pub mod replay;
pub mod restricted;
pub mod semantic_suite;
pub mod semantic_worker;
pub mod worker;

pub use attachment_store::{AttachmentStoreConfig, AttachmentStoreError, MinioAttachmentStore};
pub use candidate_store::{
    CandidateWorkspace, CandidateWorkspaceConfig, SimpleCandidateWorkspace, TempWorkspace,
    WorkspaceError,
};
pub use config::*;
pub use event_manager::{
    BlockReason, BlockReasonType, CoarseStatus, DependencyGraphEdge, DependencyGraphNode,
    DependencyGraphSnapshot, EligibleSet, EligibleSetEntry, EventManager, EventManagerError,
    RunList, RunListEntry, StageStatus, StageStatusEntry, StalenessMarkerEntry, WorkUnitStatus,
};
pub use evidence::{
    compute_verdict, EvidenceArtifact, EvidenceManifest, EvidenceManifestBuilder,
    ManifestValidationError, OracleResult, OracleResultStatus,
};
pub use governor::{
    GovernorDecision, GovernorDecisionType, GovernorError, GovernorOutcome, IterationPreconditions,
    LoopBudget, LoopGovernor, LoopTrackingState, PreconditionSnapshot, StopCondition,
};
pub use graph::{
    DependencyEdge, EdgeType, GraphError, GraphNode, GraphProjection, StalenessMarker,
    StalenessReason,
};
pub use infisical::{InfisicalConfig, InfisicalSecretProvider};
pub use integrity::{
    ArtifactHashCheck, ConstraintViolation, EnvironmentCheckResult, FlakeCheckResult,
    FlakeEvidence, FlakeHistoryWindow, GapCheckResult, IntegrityCheckDetails, IntegrityCheckResult,
    IntegrityChecker, IntegrityCheckerConfig, IntegrityError, IntegrityViolation,
    ProfileVerificationResult, StopTrigger, StopTriggerType, TamperCheckResult, ViolationSeverity,
};
pub use minio::{MinioConfig, MinioEvidenceStore};
pub use nats::{
    create_envelope, serialize_envelope, streams, subjects, MessageEnvelope, NatsConfig,
    NatsConsumer, NatsMessage, NatsMessageBus, SCHEMA_VERSION,
};
pub use oracle_runner::{
    CapturedArtifact, EnvironmentConstraints, EnvironmentFingerprint, ExpectedOutput, NetworkMode,
    OracleClassification, OracleDefinition, OracleExecutionResult, OracleSuiteDefinition,
    PodmanOracleRunner, PodmanOracleRunnerConfig,
};
pub use oracle_suite::{
    create_core_suite, create_full_suite, create_gov_suite, oracle_ids, record_to_definition,
    validate_suite, BuildArtifact, BuildError, BuildReport, IntegrityCondition, IntegrityPathway,
    IntegritySmokeReport, LintIssue, LintReport, MetaValidateReport, MetaValidationError,
    OracleSuiteRegistry, SuiteValidationError, TestFailure, UnitTestReport, VerificationProfile,
    WaivableCondition, PROFILE_GOV_CORE, PROFILE_STRICT_CORE, PROFILE_STRICT_FULL, SUITE_CORE_ID,
    SUITE_FULL_ID, SUITE_GOV_ID,
};
pub use oracle_worker::{OracleExecutionWorker, OracleWorkerConfig, OracleWorkerError};
pub use outbox::{OutboxEntry, OutboxError, OutboxPublisher, OutboxPublisherConfig, OutboxWriter};
pub use postgres::PostgresEventStore;
pub use postgres_oracle_registry::PostgresOracleSuiteRegistry;
pub use projections::{
    ApprovalProjection, CandidateProjection, ConfigDefinitionProjection, DecisionProjection,
    EvidenceProjection, ExceptionProjection, FreezeRecordProjection, HumanJudgmentRecord,
    IterationProjection, LoopProjection, LoopStopTriggerProjection, ProjectionBuilder,
    ProjectionConfig, ProjectionError, RunProjection, ShippableStatusProjection,
};
pub use replay::{EligibleSetComparison, ExtendedReplayProof, ReplayDiscrepancy, ReplayProof};
pub use restricted::{
    standard_redaction_rules, EncryptionMetadata, EvidenceClassification, EvidenceRedactor,
    InMemorySecretProvider, RedactedContentType, RedactionEntry, RedactionManifest, RedactionRule,
    RestrictedEvidenceConfig, RestrictedEvidenceStore,
};
pub use semantic_suite::{
    create_intake_admissibility_suite, to_oracle_suite_definition, IntakeAdmissibilityRunner,
    SemanticOracleDefinition, SemanticOracleSuiteDefinition, SemanticReportBundle,
    SemanticSetBindingRef, SEMANTIC_ORACLE_PREFIX, SUITE_INTAKE_ADMISSIBILITY_ID,
    SUITE_SEMANTIC_PREFIX,
};
pub use semantic_worker::{
    EvidenceBundlePayload, GateVerdict, IterationSummary, NextStepRecommendation,
    SelectionRationale, SemanticOracleResult, SemanticWorkerBridge, SemanticWorkerConfig,
    StageArtifact, StageExecutionResult, StopTriggerInfo, StopTriggerReason,
};
pub use worker::{
    run_worker, ContentResolver, ReferenceWorkerBridge, WorkAction, WorkResult, WorkerConfig,
    WorkerError,
};
