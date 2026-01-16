# SR-PLAN-V4: Work Surface Composition Implementation Plan

**Status:** Implementation-Ready
**Created:** 2026-01-16
**Purpose:** Implementation plan for Phase 4: Work Surface Composition, enabling the binding of Intake + Procedure Template + Oracle Suite into Work Surface Instances that drive Semantic Ralph Loop iterations.

---

## Executive Summary

This plan implements the **Work Surface Composition** infrastructure in SOLVER-Ralph, enabling users to create bound Work Surface Instances that serve as the iteration context for Semantic Ralph Loops.

**What this enables:**
1. **Proceduralized semantic work** — Binding an Intake to a Procedure Template ensures work follows stage-gated procedures
2. **Oracle-backed evaluation** — Each stage binds specific Oracle Suites for deterministic, reproducible evaluation
3. **Stage progression tracking** — Work Surfaces track current stage and enable stage transitions
4. **No ghost inputs** — All iteration context is derivable from the Work Surface binding (per C-CTX-2)

**Key architectural decisions:**
1. Work Surface Instances are **Commitment Objects** once bound (immutable, content-addressed)
2. Oracle suites are bound **per-stage** (not once for the whole Work Surface) — aligned with SR-PROCEDURE-KIT
3. Work Surface Instance is **1:1 with a Work Unit** (not 1:1 with a Loop — a Work Unit may have multiple Loop restarts)
4. Stage transitions are recorded as events, enabling stage progression tracking
5. Compatibility checking enforces Intake `kind` matches Procedure Template's supported kinds
6. Backend-first implementation (Phase 4a-4c before UI phases)

---

## Architecture Overview

### Component Relationships

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Work Surface Instance                              │
│                                                                              │
│  ┌─────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐  │
│  │     Intake      │    │ Procedure Template  │    │   Oracle Suites     │  │
│  │                 │    │                     │    │   (per stage)       │  │
│  │ - work_unit_id  │    │ - stages[]          │    │                     │  │
│  │ - kind          │◄───┤ - kind[] (match!)   │───►│ - suite_id          │  │
│  │ - objective     │    │ - terminal_stage_id │    │ - suite_hash        │  │
│  │ - deliverables  │    │ - gate_rules        │    │                     │  │
│  │ - constraints   │    │                     │    │                     │  │
│  │ - content_hash  │    │ - content_hash      │    │                     │  │
│  └────────┬────────┘    └──────────┬──────────┘    └──────────┬──────────┘  │
│           │                        │                          │              │
│           └────────────────────────┼──────────────────────────┘              │
│                                    │                                         │
│                         ┌──────────▼──────────┐                              │
│                         │   current_stage_id  │                              │
│                         │   stage_status{}    │                              │
│                         │   content_hash      │                              │
│                         └─────────────────────┘                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     │ produces refs for
                                     ▼
                          ┌─────────────────────┐
                          │  IterationStarted   │
                          │  - refs[] includes: │
                          │    - Intake ref     │
                          │    - Template ref   │
                          │    - Stage ref      │
                          │    - Suite refs     │
                          └─────────────────────┘
```

### Data Flow

```
User creates Intake (draft) ──► User activates Intake (commitment object)
                                         │
                                         ▼
User selects compatible Procedure Template ──► System validates kind match
                                                        │
                                                        ▼
System resolves Oracle Suites for initial stage ──► User confirms binding
                                                              │
                                                              ▼
                              WorkSurfaceBound event emitted ──► Work Surface Instance created
                                                                          │
                                                                          ▼
                                              StageEntered event ──► Stage tracking begins
                                                        │
                                                        ▼
                                        IterationStarted references Work Surface
                                                        │
                                                        ▼
                                Evidence binds to (candidate, stage, work_surface)
                                                        │
                                                        ▼
                                Gate passes ──► StageCompleted ──► StageEntered (next)
                                                        │
                                                        ▼
                                      Terminal stage reached ──► Work Surface complete
```

---

## Key Design Questions Resolved

### 1. Compatibility Checking

**Question:** How to validate that an Intake's `kind` matches Procedure Template's supported kinds?

**Resolution:**
- Procedure Templates declare `kind: Vec<WorkKind>` — the work kinds they support
- When binding, the system MUST verify `intake.kind ∈ procedure_template.kind`
- Binding fails with `INCOMPATIBLE_WORK_KIND` if the intake's kind is not in the template's supported kinds
- The UI filters Procedure Templates to show only compatible ones based on selected Intake's kind

```rust
pub fn validate_compatibility(intake: &Intake, template: &ProcedureTemplate) -> Result<(), DomainError> {
    if !template.kind.contains(&intake.kind) {
        return Err(DomainError::InvariantViolation {
            invariant: format!(
                "Intake kind '{}' is not supported by Procedure Template '{}' (supports: {:?})",
                intake.kind, template.procedure_template_id.as_str(), template.kind
            ),
        });
    }
    Ok(())
}
```

### 2. Stage Initialization

**Question:** Which stage does a new Work Surface start at?

**Resolution:**
- New Work Surface starts at the Procedure Template's **initial stage** (`initial_stage_id` or first stage in `stages[]`)
- `StageEntered` event is emitted immediately upon binding
- Initial stage status is `ENTERED` (not `COMPLETED`)

### 3. Oracle Suite Binding

**Question:** Are suites bound per-stage or once for the whole Work Surface?

**Resolution:** **Per-stage binding** based on SR-PROCEDURE-KIT:
- Each stage in the Procedure Template declares `required_oracle_suites[]`
- When entering a stage, the system resolves the Oracle Suites from the registry
- Suite hashes are captured at stage entry time for determinism (per SR-SEMANTIC-ORACLE-SPEC §2)
- Oracle suites are **not** stored on the Work Surface Instance directly — they are resolved dynamically from the template + registry when entering each stage
- The `IterationStarted.refs[]` includes the suite refs for the **current stage only**

### 4. Immutability

**Question:** Once bound, can a Work Surface Instance be modified?

**Resolution:** **Partial immutability** — commitment object semantics apply:
- **Immutable once bound:** `intake_ref`, `procedure_template_ref`, `work_unit_id`
- **Mutable by controlled events:** `current_stage_id`, `stage_status`, `params`
- Stage transitions are the only permitted mutations, and they are recorded as events
- If the intake or procedure template needs to change, a new Work Surface must be created (the old one can be archived)

### 5. Relationship to Loops

**Question:** Is Work Surface 1:1 with a Loop, or can multiple Loops share a Work Surface?

**Resolution:** **Work Surface 1:1 with Work Unit, not Loop**:
- A Work Unit has exactly one active Work Surface Instance at a time
- A Work Unit may have multiple Loops (if a loop is terminated and restarted)
- All Loops for a Work Unit share the same Work Surface Instance
- Work Surface tracks aggregate stage status across all loops/iterations

```
Work Unit (1) ──────► Work Surface Instance (1)
     │                         │
     └──► Loop (many) ──► Iteration (many) ──► references Work Surface
```

### 6. UI Workflow

**Question:** Step-by-step wizard vs. single form?

**Resolution:** **Step-by-step wizard** for clarity:
1. **Select Intake** — Browse/search active intakes, select one
2. **Select Procedure Template** — Filtered to compatible templates based on intake kind
3. **Review & Confirm** — Show summary of binding, oracle suites for initial stage, confirm
4. **Work Surface Created** — Navigate to Work Surface detail view

Alternative: "Quick bind" from Intake detail page (single click to use default/recommended template)

---

## 1. Domain Model

### 1.1 Work Surface Instance (Extended)

The existing `work_surface.rs` defines `WorkSurfaceInstance`. This plan extends it with lifecycle and stage tracking:

```rust
/// Work Surface Instance identifier
/// Format: `ws:<ULID>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkSurfaceId(String);

impl WorkSurfaceId {
    pub fn new() -> Self {
        Self(format!("ws:{}", ulid::Ulid::new()))
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Work Surface status (lifecycle)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "work_surface_status", rename_all = "snake_case")]
pub enum WorkSurfaceStatus {
    /// Active — work in progress
    Active,
    /// Completed — terminal stage reached with passing gate
    Completed,
    /// Archived — superseded or abandoned
    Archived,
}

/// Stage completion status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "stage_status", rename_all = "snake_case")]
pub enum StageCompletionStatus {
    /// Not yet entered
    Pending,
    /// Currently active
    Entered,
    /// Completed with passing gate
    Completed,
    /// Skipped (if allowed by template)
    Skipped,
}

/// Stage status record for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageStatusRecord {
    pub stage_id: StageId,
    pub status: StageCompletionStatus,
    pub entered_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub evidence_bundle_ref: Option<String>,
    pub iteration_count: u32,
}

/// Managed Work Surface Instance (runtime representation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedWorkSurface {
    // === Identity ===
    pub work_surface_id: WorkSurfaceId,
    pub work_unit_id: WorkUnitId,

    // === Binding refs (immutable once bound) ===
    pub intake_ref: ContentAddressedRef,
    pub procedure_template_ref: ContentAddressedRef,

    // === Current state (mutable via events) ===
    pub current_stage_id: StageId,
    pub status: WorkSurfaceStatus,
    pub stage_status: HashMap<String, StageStatusRecord>,

    // === Oracle context for current stage ===
    pub current_oracle_suites: Vec<OracleSuiteBinding>,

    // === Parameters ===
    pub params: HashMap<String, serde_json::Value>,

    // === Content addressing ===
    pub content_hash: Option<ContentHash>,

    // === Attribution ===
    pub bound_at: DateTime<Utc>,
    pub bound_by: ActorId,
    pub completed_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<ActorId>,
}
```

### 1.2 Work Surface Events

Per SR-SPEC Appendix A event patterns:

```rust
// === Work Surface Events ===

/// Emitted when a Work Surface Instance is bound
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceBound {
    pub work_surface_id: String,
    pub work_unit_id: String,
    pub intake_ref: ContentAddressedRef,
    pub procedure_template_ref: ContentAddressedRef,
    pub initial_stage_id: String,
    pub content_hash: String,
}

/// Emitted when entering a new stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageEntered {
    pub work_surface_id: String,
    pub stage_id: String,
    pub previous_stage_id: Option<String>,
    pub oracle_suites: Vec<OracleSuiteBinding>,
}

/// Emitted when a stage is completed (gate passed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageCompleted {
    pub work_surface_id: String,
    pub stage_id: String,
    pub evidence_bundle_ref: String,
    pub gate_result: GateResult,
    pub next_stage_id: Option<String>, // None if terminal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub status: GateResultStatus,
    pub oracle_results: Vec<OracleResultSummary>,
    pub waiver_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GateResultStatus {
    Pass,
    PassWithWaivers,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResultSummary {
    pub oracle_id: String,
    pub status: String, // PASS | FAIL | ERROR
    pub evidence_ref: Option<String>,
}

/// Emitted when Work Surface is completed (terminal stage passed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceCompleted {
    pub work_surface_id: String,
    pub final_stage_id: String,
    pub evidence_bundle_ref: String,
}

/// Emitted when Work Surface is archived
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSurfaceArchived {
    pub work_surface_id: String,
    pub reason: Option<String>,
}

// Stream configuration
impl WorkSurfaceBound {
    pub fn stream_id(&self) -> String {
        format!("work_surface:{}", self.work_surface_id)
    }
    pub fn stream_kind() -> &'static str { "WORK_SURFACE" }
    pub fn event_type() -> &'static str { "WorkSurfaceBound" }
}
```

---

## 2. Database Schema

### 2.1 Enums

```sql
-- Work Surface status enum
CREATE TYPE work_surface_status AS ENUM ('active', 'completed', 'archived');

-- Stage completion status enum
CREATE TYPE stage_completion_status AS ENUM ('pending', 'entered', 'completed', 'skipped');
```

### 2.2 Work Surfaces Table

```sql
CREATE TABLE proj.work_surfaces (
    -- Identity
    work_surface_id         TEXT PRIMARY KEY,           -- format: ws:<ULID>
    work_unit_id            TEXT NOT NULL,              -- format: WU-<id>

    -- Binding refs (immutable)
    intake_id               TEXT NOT NULL REFERENCES proj.intakes(intake_id),
    intake_content_hash     TEXT NOT NULL,              -- sha256:<hex>
    procedure_template_id   TEXT NOT NULL,              -- format: proc:<NAME>
    procedure_template_hash TEXT NOT NULL,              -- sha256:<hex>

    -- Current state
    current_stage_id        TEXT NOT NULL,              -- format: stage:<NAME>
    status                  work_surface_status NOT NULL DEFAULT 'active',
    stage_status            JSONB NOT NULL DEFAULT '{}'::jsonb, -- {stage_id: StageStatusRecord}

    -- Oracle context (for current stage, refreshed on stage entry)
    current_oracle_suites   JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Parameters
    params                  JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Content hash (of binding - intake + template + initial params)
    content_hash            TEXT NOT NULL,

    -- Attribution
    bound_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    bound_by_kind           TEXT NOT NULL,              -- HUMAN | SYSTEM
    bound_by_id             TEXT NOT NULL,
    completed_at            TIMESTAMPTZ,
    archived_at             TIMESTAMPTZ,
    archived_by_kind        TEXT,
    archived_by_id          TEXT,

    -- Event tracking
    last_event_id           TEXT NOT NULL,
    last_global_seq         BIGINT NOT NULL,

    -- Constraints
    CONSTRAINT ws_id_format CHECK (work_surface_id ~ '^ws:[0-9A-Z]+$'),
    CONSTRAINT intake_hash_format CHECK (intake_content_hash ~ '^sha256:[a-f0-9]{64}$'),
    CONSTRAINT template_hash_format CHECK (procedure_template_hash ~ '^sha256:[a-f0-9]{64}$'),
    CONSTRAINT content_hash_format CHECK (content_hash ~ '^sha256:[a-f0-9]{64}$'),
    CONSTRAINT completed_has_timestamp CHECK (status != 'completed' OR completed_at IS NOT NULL),
    CONSTRAINT archived_has_timestamp CHECK (status != 'archived' OR archived_at IS NOT NULL)
);

-- Indexes
CREATE INDEX idx_work_surfaces_work_unit ON proj.work_surfaces(work_unit_id);
CREATE INDEX idx_work_surfaces_intake ON proj.work_surfaces(intake_id);
CREATE INDEX idx_work_surfaces_status ON proj.work_surfaces(status);
CREATE INDEX idx_work_surfaces_current_stage ON proj.work_surfaces(current_stage_id);

-- Unique constraint: only one active work surface per work unit
CREATE UNIQUE INDEX uniq_work_surfaces_active_per_work_unit
    ON proj.work_surfaces(work_unit_id)
    WHERE status = 'active';
```

### 2.3 Stage History Table (Optional — for detailed audit)

```sql
CREATE TABLE proj.work_surface_stage_history (
    id                      BIGSERIAL PRIMARY KEY,
    work_surface_id         TEXT NOT NULL REFERENCES proj.work_surfaces(work_surface_id),
    stage_id                TEXT NOT NULL,
    status                  stage_completion_status NOT NULL,
    entered_at              TIMESTAMPTZ,
    completed_at            TIMESTAMPTZ,
    evidence_bundle_ref     TEXT,                       -- sha256:<hex>
    iteration_count         INTEGER NOT NULL DEFAULT 0,
    event_id                TEXT NOT NULL,
    recorded_at             TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ws_stage_history_ws ON proj.work_surface_stage_history(work_surface_id);
CREATE INDEX idx_ws_stage_history_stage ON proj.work_surface_stage_history(work_surface_id, stage_id);
```

---

## 3. API Specification

### 3.1 Work Surface Endpoints

All endpoints under `/api/v1/work-surfaces`.

```rust
// === Create / Bind ===

/// Create (bind) a new Work Surface Instance
/// POST /api/v1/work-surfaces
///
/// Validates compatibility, resolves oracle suites for initial stage,
/// computes content hash, emits WorkSurfaceBound and StageEntered events.
pub async fn create_work_surface(
    State(state): State<AppState>,
    actor: AuthenticatedActor,
    Json(request): Json<CreateWorkSurfaceRequest>,
) -> ApiResult<Json<WorkSurfaceResponse>>;

#[derive(Debug, Deserialize)]
pub struct CreateWorkSurfaceRequest {
    /// Work unit identifier
    pub work_unit_id: String,
    /// Active intake ID (must be status = active)
    pub intake_id: String,
    /// Procedure template ID
    pub procedure_template_id: String,
    /// Optional stage parameters
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct WorkSurfaceResponse {
    pub work_surface_id: String,
    pub work_unit_id: String,
    pub intake_id: String,
    pub intake_content_hash: String,
    pub procedure_template_id: String,
    pub procedure_template_hash: String,
    pub current_stage_id: String,
    pub status: WorkSurfaceStatus,
    pub stage_status: HashMap<String, StageStatusRecord>,
    pub current_oracle_suites: Vec<OracleSuiteBinding>,
    pub params: HashMap<String, serde_json::Value>,
    pub content_hash: String,
    pub bound_at: DateTime<Utc>,
    pub bound_by: ActorId,
}

// === Read ===

/// Get Work Surface by ID
/// GET /api/v1/work-surfaces/:work_surface_id
pub async fn get_work_surface(
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<WorkSurfaceDetailResponse>>;

/// Get Work Surface by Work Unit ID
/// GET /api/v1/work-surfaces/by-work-unit/:work_unit_id
/// Returns the active work surface for the work unit (if any)
pub async fn get_work_surface_by_work_unit(
    Path(work_unit_id): Path<String>,
) -> ApiResult<Json<Option<WorkSurfaceDetailResponse>>>;

/// List Work Surfaces with filters
/// GET /api/v1/work-surfaces
/// Query params: status, intake_id, procedure_template_id, page, page_size
pub async fn list_work_surfaces(
    Query(params): Query<ListWorkSurfacesParams>,
) -> ApiResult<Json<WorkSurfacesListResponse>>;

#[derive(Debug, Deserialize)]
pub struct ListWorkSurfacesParams {
    pub status: Option<WorkSurfaceStatus>,
    pub intake_id: Option<String>,
    pub procedure_template_id: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct WorkSurfacesListResponse {
    pub work_surfaces: Vec<WorkSurfaceSummary>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize)]
pub struct WorkSurfaceSummary {
    pub work_surface_id: String,
    pub work_unit_id: String,
    pub intake_title: String,
    pub procedure_template_name: String,
    pub current_stage_id: String,
    pub status: WorkSurfaceStatus,
    pub bound_at: DateTime<Utc>,
}

// === Stage Transitions ===

/// Record stage completion (gate passed)
/// POST /api/v1/work-surfaces/:work_surface_id/stages/:stage_id/complete
///
/// Validates evidence, records StageCompleted, advances to next stage (StageEntered).
/// If terminal stage, records WorkSurfaceCompleted.
pub async fn complete_stage(
    Path((work_surface_id, stage_id)): Path<(String, String)>,
    actor: AuthenticatedActor,
    Json(request): Json<CompleteStageRequest>,
) -> ApiResult<Json<StageCompletionResponse>>;

#[derive(Debug, Deserialize)]
pub struct CompleteStageRequest {
    /// Evidence bundle hash proving gate passage
    pub evidence_bundle_ref: String,
    /// Gate result details
    pub gate_result: GateResult,
}

#[derive(Debug, Serialize)]
pub struct StageCompletionResponse {
    pub work_surface_id: String,
    pub completed_stage_id: String,
    pub next_stage_id: Option<String>,
    pub is_terminal: bool,
    pub work_surface_status: WorkSurfaceStatus,
}

/// Get iteration context for current stage
/// GET /api/v1/work-surfaces/:work_surface_id/iteration-context
///
/// Returns the TypedRef[] needed for IterationStarted per C-CTX-1/C-CTX-2.
pub async fn get_iteration_context(
    Path(work_surface_id): Path<String>,
) -> ApiResult<Json<IterationContextResponse>>;

#[derive(Debug, Serialize)]
pub struct IterationContextResponse {
    pub work_surface_id: String,
    pub refs: Vec<TypedRef>,
}

// === Lifecycle ===

/// Archive a Work Surface
/// POST /api/v1/work-surfaces/:work_surface_id/archive
pub async fn archive_work_surface(
    Path(work_surface_id): Path<String>,
    actor: AuthenticatedActor,
    Json(request): Json<ArchiveWorkSurfaceRequest>,
) -> ApiResult<Json<WorkSurfaceResponse>>;

#[derive(Debug, Deserialize)]
pub struct ArchiveWorkSurfaceRequest {
    pub reason: Option<String>,
}
```

### 3.2 Compatibility Check Endpoint

```rust
/// Check compatibility between Intake and Procedure Template
/// GET /api/v1/work-surfaces/compatibility
/// Query params: intake_id, procedure_template_id
///
/// Returns compatibility status and any issues.
pub async fn check_compatibility(
    Query(params): Query<CompatibilityCheckParams>,
) -> ApiResult<Json<CompatibilityCheckResponse>>;

#[derive(Debug, Deserialize)]
pub struct CompatibilityCheckParams {
    pub intake_id: String,
    pub procedure_template_id: String,
}

#[derive(Debug, Serialize)]
pub struct CompatibilityCheckResponse {
    pub compatible: bool,
    pub intake_kind: String,
    pub template_supported_kinds: Vec<String>,
    pub issues: Vec<String>,
}
```

### 3.3 Compatible Templates Endpoint

```rust
/// Get Procedure Templates compatible with an Intake
/// GET /api/v1/work-surfaces/compatible-templates
/// Query params: intake_id
///
/// Returns procedure templates whose kind[] includes the intake's kind.
pub async fn get_compatible_templates(
    Query(params): Query<CompatibleTemplatesParams>,
) -> ApiResult<Json<CompatibleTemplatesResponse>>;

#[derive(Debug, Deserialize)]
pub struct CompatibleTemplatesParams {
    pub intake_id: String,
}

#[derive(Debug, Serialize)]
pub struct CompatibleTemplatesResponse {
    pub intake_id: String,
    pub intake_kind: String,
    pub templates: Vec<ProcedureTemplateSummary>,
}

#[derive(Debug, Serialize)]
pub struct ProcedureTemplateSummary {
    pub procedure_template_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub stages_count: u32,
    pub supported_kinds: Vec<String>,
}
```

---

## 4. Event Integration

### 4.1 IterationStarted Integration

Per SR-CONTRACT C-CTX-1 and C-CTX-2, `IterationStarted.refs[]` must include the Work Surface components:

```rust
impl ManagedWorkSurface {
    /// Compile iteration context refs per SR-SPEC §3.2.1.1
    pub fn compile_iteration_context_refs(
        &self,
        loop_id: &str,
        governed_artifacts: &[GovernedArtifactRef],
        prior_iterations: &[IterationRef],
        active_exceptions: &[ExceptionRef],
    ) -> Vec<TypedRef> {
        let mut refs = Vec::new();

        // 1. Loop reference
        refs.push(TypedRef {
            kind: RefKind::Loop,
            id: loop_id.to_string(),
            rel: RefRelation::InScopeOf,
            meta: RefMeta::default(),
            label: None,
        });

        // 2. Governing artifacts in force
        for artifact in governed_artifacts {
            refs.push(TypedRef {
                kind: RefKind::GovernedArtifact,
                id: artifact.id.clone(),
                rel: RefRelation::DependsOn,
                meta: RefMeta {
                    content_hash: Some(artifact.content_hash.clone()),
                    version: Some(artifact.version.clone()),
                    ..Default::default()
                },
                label: artifact.label.clone(),
            });
        }

        // 3. Prior iteration summaries
        for iteration in prior_iterations {
            refs.push(TypedRef {
                kind: RefKind::Iteration,
                id: iteration.id.clone(),
                rel: RefRelation::DependsOn,
                meta: RefMeta {
                    content_hash: iteration.content_hash.clone(),
                    ..Default::default()
                },
                label: None,
            });
        }

        // 4. Intake reference (Work Surface component)
        refs.push(TypedRef {
            kind: RefKind::Intake,
            id: self.intake_ref.id.clone(),
            rel: RefRelation::DependsOn,
            meta: RefMeta {
                content_hash: Some(self.intake_ref.content_hash.as_str().to_string()),
                ..Default::default()
            },
            label: None,
        });

        // 5. Procedure Template reference (Work Surface component)
        refs.push(TypedRef {
            kind: RefKind::ProcedureTemplate,
            id: self.procedure_template_ref.id.clone(),
            rel: RefRelation::DependsOn,
            meta: RefMeta {
                content_hash: Some(self.procedure_template_ref.content_hash.as_str().to_string()),
                current_stage_id: Some(self.current_stage_id.as_str().to_string()),
                ..Default::default()
            },
            label: None,
        });

        // 6. Oracle Suites for current stage
        for suite in &self.current_oracle_suites {
            refs.push(TypedRef {
                kind: RefKind::OracleSuite,
                id: suite.suite_id.clone(),
                rel: RefRelation::DependsOn,
                meta: RefMeta {
                    content_hash: Some(suite.suite_hash.as_str().to_string()),
                    ..Default::default()
                },
                label: None,
            });
        }

        // 7. Active exceptions in scope
        for exc in active_exceptions {
            refs.push(TypedRef {
                kind: exc.kind.clone(),
                id: exc.id.clone(),
                rel: RefRelation::DependsOn,
                meta: RefMeta {
                    content_hash: exc.content_hash.clone(),
                    ..Default::default()
                },
                label: None,
            });
        }

        refs
    }
}
```

### 4.2 Evidence Binding

Per SR-SPEC §1.9.1, `EvidenceBundleRecorded` must bind evidence to the procedure context:

```rust
/// Evidence bundle context for Work Surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundleWorkSurfaceContext {
    pub work_surface_id: String,
    pub procedure_template_id: String,
    pub stage_id: String,
    pub candidate_id: String,
}
```

When recording evidence, the handler must include:
- `procedure_template_id` from the Work Surface
- `stage_id` from the Work Surface's current stage

---

## 5. UI Specification

### 5.1 New Pages

| Route | Component | Purpose |
|-------|-----------|---------|
| `/work-surfaces` | `WorkSurfaces.tsx` | List view with filters |
| `/work-surfaces/new` | `WorkSurfaceCompose.tsx` | Composition wizard |
| `/work-surfaces/:id` | `WorkSurfaceDetail.tsx` | Detail view with stage progress |

### 5.2 Composition Wizard (`WorkSurfaceCompose.tsx`)

**Step 1: Select Intake**
```tsx
// Intake selection with search and filters
interface IntakeSelectionStep {
  intakes: IntakeSummary[];
  selectedIntake: IntakeSummary | null;
  filters: {
    status: 'active'; // Only active intakes can be bound
    kind?: WorkKind;
    search?: string;
  };
}
```

**Step 2: Select Procedure Template**
```tsx
// Template selection (filtered by intake kind)
interface TemplateSelectionStep {
  templates: ProcedureTemplateSummary[];
  selectedTemplate: ProcedureTemplateSummary | null;
  // Templates pre-filtered by API based on intake.kind
}
```

**Step 3: Review & Confirm**
```tsx
// Binding summary and confirmation
interface ReviewStep {
  intake: IntakeDetail;
  template: ProcedureTemplateDetail;
  initialStage: StageDetail;
  oracleSuites: OracleSuiteSummary[];
  params: Record<string, unknown>;
}
```

### 5.3 Work Surface Detail (`WorkSurfaceDetail.tsx`)

```tsx
interface WorkSurfaceDetailProps {
  workSurfaceId: string;
}

// Main sections:
// 1. Header with status badge, work unit, timestamps
// 2. Binding summary (intake, template)
// 3. Stage progress visualization (horizontal stepper or vertical timeline)
// 4. Current stage details (oracle suites, evidence, actions)
// 5. Actions: Archive, View Iteration Context
```

**Stage Progress Visualization:**
```
[FRAME]──────►[OPTIONS]──────►[DRAFT]──────►[SEMANTIC_EVAL]──────►[FINAL]
   ✓            ✓              ●              ○                    ○
Completed    Completed      Current        Pending              Pending
```

### 5.4 Sidebar Navigation Update

```tsx
const items = [
  // ... existing items ...
  { to: "/loops", label: "Loops", icon: RefreshCw },
  { to: "/intakes", label: "Intakes", icon: FileInput },
  { to: "/work-surfaces", label: "Work Surfaces", icon: Layers },  // NEW
  { to: "/references", label: "References", icon: Library },
  // ... remaining items ...
];
```

### 5.5 Route Updates

```tsx
const routes = [
  // Work Surfaces (NEW)
  { path: "/work-surfaces", element: <WorkSurfaces /> },
  { path: "/work-surfaces/new", element: <WorkSurfaceCompose /> },
  { path: "/work-surfaces/:workSurfaceId", element: <WorkSurfaceDetail /> },
];
```

---

## 6. Implementation Phases

### Phase 4a: Core Infrastructure

**Duration:** Foundation
**Dependencies:** Phase 0-3 complete (Intakes, References)

**Deliverables:**

1. **Work Surface ID type** (`crates/sr-domain/src/work_surface.rs`)
   - `WorkSurfaceId` with ULID generation
   - `WorkSurfaceStatus` enum
   - `StageCompletionStatus` enum
   - `StageStatusRecord` struct
   - `ManagedWorkSurface` struct

2. **Work Surface Events** (`crates/sr-domain/src/events.rs`)
   - `WorkSurfaceBound`
   - `StageEntered`
   - `StageCompleted`
   - `WorkSurfaceCompleted`
   - `WorkSurfaceArchived`
   - Add `StreamKind::WorkSurface`

3. **Database migrations** (`migrations/006_work_surfaces.sql`)
   - Create `work_surface_status` enum
   - Create `stage_completion_status` enum
   - Create `proj.work_surfaces` table
   - Create indexes
   - Optional: Create `proj.work_surface_stage_history` table

**Verification Checklist:**
- [ ] `cargo build` passes
- [ ] `cargo test` passes
- [ ] Migration applies cleanly: `sqlx migrate run`
- [ ] Types align with SR-WORK-SURFACE §5

---

### Phase 4b: Work Surface API

**Duration:** API endpoints
**Dependencies:** Phase 4a

**Deliverables:**

1. **Work Surface Handler** (`crates/sr-api/src/handlers/work_surfaces.rs`)
   - `POST /api/v1/work-surfaces` — Create/bind
   - `GET /api/v1/work-surfaces` — List with filters
   - `GET /api/v1/work-surfaces/:id` — Get by ID
   - `GET /api/v1/work-surfaces/by-work-unit/:work_unit_id` — Get by Work Unit
   - `POST /api/v1/work-surfaces/:id/stages/:stage_id/complete` — Complete stage
   - `GET /api/v1/work-surfaces/:id/iteration-context` — Get iteration refs
   - `POST /api/v1/work-surfaces/:id/archive` — Archive
   - `GET /api/v1/work-surfaces/compatibility` — Check compatibility
   - `GET /api/v1/work-surfaces/compatible-templates` — List compatible templates

2. **Compatibility validation logic**
   - Kind matching between Intake and Procedure Template
   - Oracle suite resolution from registry

3. **Content hash computation**
   - Canonical JSON of binding (intake_ref + template_ref + params)
   - SHA-256 hash

4. **Router updates** (`crates/sr-api/src/main.rs`)

5. **Projection handlers** (`crates/sr-adapters/src/projections.rs`)
   - Handle Work Surface events

**Verification Checklist:**
- [ ] All CRUD endpoints working
- [ ] Compatibility validation enforced
- [ ] Stage transitions working
- [ ] Iteration context compilation correct
- [ ] Events emitted correctly

---

### Phase 4c: Event Integration

**Duration:** Iteration integration
**Dependencies:** Phase 4b

**Deliverables:**

1. **IterationStarted integration**
   - Update iteration creation to fetch Work Surface refs
   - Include Work Surface refs in `IterationStarted.refs[]`

2. **EvidenceBundleRecorded integration**
   - Include `procedure_template_id` and `stage_id` in evidence context

3. **Loop governor updates**
   - Validate Work Surface exists for iteration
   - Stop trigger: `WORK_SURFACE_MISSING`

**Verification Checklist:**
- [ ] IterationStarted includes Work Surface refs
- [ ] Evidence binds to stage context
- [ ] Stop trigger fires when Work Surface missing

---

### Phase 4d: Work Surface UI

**Duration:** UI implementation
**Dependencies:** Phase 4b, 4c

**Deliverables:**

1. **Work Surfaces list page** (`ui/src/pages/WorkSurfaces.tsx`)
   - Filter by status
   - Search by work unit, intake title
   - Status badges
   - Pagination

2. **Work Surface composition wizard** (`ui/src/pages/WorkSurfaceCompose.tsx`)
   - Step 1: Intake selection
   - Step 2: Template selection (filtered)
   - Step 3: Review & confirm
   - Success: Navigate to detail

3. **Work Surface detail page** (`ui/src/pages/WorkSurfaceDetail.tsx`)
   - Header with status
   - Binding summary
   - Stage progress visualization
   - Current stage details
   - Actions

4. **Sidebar and routes update**

5. **Integration with Intake detail**
   - "Create Work Surface" action on active intakes

**Verification Checklist:**
- [ ] List page displays work surfaces
- [ ] Wizard completes successfully
- [ ] Stage progress displays correctly
- [ ] Actions work correctly

---

## 7. Verification Checklist (All Phases)

| Check | Command | Expected |
|-------|---------|----------|
| Rust build | `cargo build` | No errors |
| Rust tests | `cargo test --workspace` | All pass |
| Clippy | `cargo clippy` | No warnings |
| TypeScript | `cd ui && npm run type-check` | No errors |
| UI build | `cd ui && npm run build` | Success |
| Migrations | `sqlx migrate run` | Applied cleanly |

---

## 8. Intentional Deviations from SR-* Specifications

| Deviation | Specification | Reason |
|-----------|---------------|--------|
| None | N/A | This plan is fully aligned with SR-* specifications |

---

## 9. Governance Document References

| Document | Relevance | Key Sections |
|----------|-----------|--------------|
| SR-WORK-SURFACE | Primary specification | §2 (concepts), §4 (Procedure Template), §5 (Work Surface Instance) |
| SR-PROCEDURE-KIT | Stage mechanics | §1-2 (registry, baseline template) |
| SR-SEMANTIC-ORACLE-SPEC | Oracle binding | §2-4 (suite identity, outputs) |
| SR-CONTRACT | Binding invariants | §2.8 (Commitment Objects), C-CTX-1, C-CTX-2 |
| SR-SPEC | Event integration | §1.9 (IterationStarted), §1.5.3 (TypedRef) |
| SR-TYPES | Type registry | §4.3 (`domain.work_surface`) |

---

## 10. User Preferences (To Establish)

The following decisions should be confirmed with the user before implementation:

1. **Work Surface naming** — Is "Work Surface" acceptable user-facing terminology? Alternative: "Work Context", "Procedure Binding"
2. **Wizard vs. quick-bind** — Should the wizard be the only way, or also support a "quick bind" from Intake detail?
3. **Stage visualization** — Horizontal stepper vs. vertical timeline?
4. **Archive behavior** — Should archiving a Work Surface archive the associated Work Unit too?

---

## Appendix A: Example Work Surface Lifecycle

```
1. User creates Intake (draft) for "API Rate Limiting Analysis"
   └── IntakeCreated event

2. User activates Intake
   └── IntakeActivated event
   └── content_hash: sha256:abc123...

3. User navigates to Work Surfaces, clicks "New"

4. Wizard Step 1: User selects the activated Intake
   └── API calls GET /api/v1/work-surfaces/compatible-templates?intake_id=intake:01ABC

5. Wizard Step 2: User selects "proc:GENERIC-KNOWLEDGE-WORK" template
   └── API confirms compatibility

6. Wizard Step 3: User reviews binding, confirms
   └── API calls POST /api/v1/work-surfaces
   └── WorkSurfaceBound event (work_surface_id: ws:01DEF)
   └── StageEntered event (stage: stage:FRAME)

7. System starts Loop for Work Unit
   └── LoopCreated event

8. System starts Iteration
   └── IterationStarted event with refs[] including:
       - Intake ref (content_hash: sha256:abc123)
       - ProcedureTemplate ref (current_stage_id: stage:FRAME)
       - OracleSuite refs for FRAME stage

9. Agent produces candidate, runs oracles
   └── CandidateMaterialized event
   └── RunCompleted event
   └── EvidenceBundleRecorded event (stage_id: stage:FRAME)

10. Gate passes, stage completes
    └── POST /api/v1/work-surfaces/ws:01DEF/stages/stage:FRAME/complete
    └── StageCompleted event
    └── StageEntered event (stage: stage:OPTIONS)

11. ... repeat for OPTIONS, DRAFT, SEMANTIC_EVAL ...

12. Terminal stage (FINAL) completes
    └── StageCompleted event (is_terminal: true)
    └── WorkSurfaceCompleted event
    └── Work Surface status → "completed"
```
