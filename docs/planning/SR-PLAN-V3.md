# SR-PLAN-V3: Intakes and References Implementation Plan

**Status:** Implementation-Ready
**Created:** 2026-01-16
**Supersedes:** SR-PLAN-V2.md
**Purpose:** Implementation plan for Intakes UI/API and References browser, aligned with SR-* governance framework. All 10 issues from V2 are resolved.

---

## Executive Summary

This plan implements the UI and API infrastructure for **Intakes** and **References** in SOLVER-Ralph, ensuring semantic consistency with the governed document set (SR-CONTRACT, SR-SPEC, SR-TYPES, SR-WORK-SURFACE, SR-PROCEDURE-KIT, SR-SEMANTIC-ORACLE-SPEC).

**Key architectural decisions:**
1. Intakes are **Commitment Objects** when activated (content-addressed, immutable)
2. References use a unified **TypedRef** schema per SR-SPEC §1.5.3
3. Work Surface = Intake + Procedure Template + Oracle Profile (bound together for iterations)
4. Backend-first implementation (Phase 0) to avoid UI calling non-existent endpoints
5. Phase 0 split into sub-phases for manageability

---

## V2 Issue Resolutions

All 10 issues from SR-PLAN-V2 are resolved in this plan:

| # | Issue | Resolution |
|---|-------|------------|
| 1 | Separate `InputRef` and `TypedRef` schemas | **Unified** to single `TypedRef` schema (§1.1) |
| 2 | Events mentioned but not specified | **Full event model** defined (§1.6) |
| 3 | No PostgreSQL schema for intakes | **Complete schema** in `proj.intakes` (§1.4) |
| 4 | Status terminology mismatch | **Aligned** with SR-TYPES: `draft \| active \| archived` with mapping (§1.2) |
| 5 | By-hash retrieval semantics unclear | **Clarified**: returns all statuses, C-EVID-6 compliance (§1.5) |
| 6 | References API response format inconsistent | **Standardized** to `{ refs, total, page, page_size }` (§2.2) |
| 7 | RefRelation enum may be incomplete | **Verified** against SR-SPEC §1.5.3, complete enum (§1.1) |
| 8 | Phase 0 scope too large | **Split** into 0a, 0b, 0c sub-phases (§3) |
| 9 | May duplicate existing templates.rs | **Reviewed**: templates.rs handles schemas, not runtime; new handler needed (§1.7) |
| 10 | Missing ref categories | **Added** Agent Definitions, Gating Policies (§2.1) |

---

## 1. Unified Type Definitions

### 1.1 TypedRef Schema (Canonical — SR-SPEC §1.5.3)

One ref schema for all uses: Intake inputs, References browser, event refs.

```rust
/// Unified typed reference per SR-SPEC §1.5.3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypedRef {
    /// Reference kind
    pub kind: RefKind,
    /// Stable identifier
    pub id: String,
    /// Relationship type
    pub rel: RefRelation,
    /// Required metadata
    pub meta: RefMeta,
    /// Human-readable label (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Reference kinds per SR-SPEC §1.5.3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum RefKind {
    GovernedArtifact,
    Candidate,
    OracleSuite,
    EvidenceBundle,
    Approval,
    Record,
    Decision,
    Deviation,
    Deferral,
    Waiver,
    Loop,
    Iteration,
    Run,
    Freeze,
    Intake,
    ProcedureTemplate,
    ProcedureStage,
    SemanticSet,
    WorkSurface,
    // Configuration artifacts
    AgentDefinition,
    GatingPolicy,
}

/// Relationship types per SR-SPEC §1.5.3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefRelation {
    About,
    DependsOn,
    SupportedBy,
    Produces,
    Verifies,
    ApprovedBy,
    Acknowledges,
    Supersedes,
    Releases,
    GovernedBy,
    InScopeOf,
    Affects,
    Stale,
    RootCause,
    RelatesTo,
}

/// Reference metadata per SR-SPEC §1.5.3.1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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
    /// Selector for stable slice (recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Current stage (for ProcedureTemplate refs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage_id: Option<String>,
}
```

### 1.2 IntakeStatus (Aligned with SR-TYPES §3.1)

```rust
/// Intake lifecycle status
///
/// Mapping to SR-TYPES §3.1 `status` enum:
/// - Draft = draft (proposal, editable)
/// - Active = governed (commitment object, immutable)
/// - Archived = archived (superseded, read-only)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "intake_status", rename_all = "snake_case")]
pub enum IntakeStatus {
    /// Proposal - editable, not yet a commitment object
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
}
```

### 1.3 Intake Schema (SR-WORK-SURFACE §3.1)

```rust
/// Intake per SR-WORK-SURFACE §3.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intake {
    // === Identity ===
    /// Unique identifier (format: "intake:<ULID>")
    pub intake_id: String,
    /// Work unit this intake belongs to
    pub work_unit_id: String,
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
    /// Input references using unified TypedRef
    pub inputs: Vec<TypedRef>,
    pub unknowns: Vec<String>,
    pub completion_criteria: Vec<String>,

    // === Lifecycle ===
    pub status: IntakeStatus,
    /// Increments on fork
    pub version: u32,
    /// intake_id of prior version (if forked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,

    // === Attribution (per C-EVT-1) ===
    pub created_at: DateTime<Utc>,
    pub created_by: ActorId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_by: Option<ActorId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    /// Deliverable name
    pub name: String,
    /// Output format (e.g., "markdown", "json", "pdf")
    pub format: String,
    /// Conventional output path (e.g., "candidate/main.md")
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "work_kind", rename_all = "snake_case")]
pub enum WorkKind {
    ResearchMemo,
    DecisionRecord,
    OntologyBuild,
    AnalysisReport,
    DesignDocument,
    ReviewResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorId {
    pub actor_kind: ActorKind,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorKind {
    Human,
    Agent,
    System,
}
```

### 1.4 PostgreSQL Schema

```sql
-- Intake status enum
CREATE TYPE intake_status AS ENUM ('draft', 'active', 'archived');

-- Work kind enum per SR-WORK-SURFACE §3.1
CREATE TYPE work_kind AS ENUM (
    'research_memo',
    'decision_record',
    'ontology_build',
    'analysis_report',
    'design_document',
    'review_response'
);

-- Intakes table
CREATE TABLE proj.intakes (
    intake_id           TEXT PRIMARY KEY,           -- format: intake:<ULID>
    work_unit_id        TEXT NOT NULL,              -- format: wu:<identifier>
    content_hash        TEXT,                       -- sha256:<hex> - set on activation

    -- Required fields per SR-WORK-SURFACE §3.1
    title               TEXT NOT NULL,
    kind                work_kind NOT NULL,
    objective           TEXT NOT NULL,              -- ONE sentence
    audience            TEXT NOT NULL,
    deliverables        JSONB NOT NULL DEFAULT '[]'::jsonb,
    constraints         JSONB NOT NULL DEFAULT '[]'::jsonb,
    definitions         JSONB NOT NULL DEFAULT '{}'::jsonb,
    inputs              JSONB NOT NULL DEFAULT '[]'::jsonb,  -- TypedRef[]
    unknowns            JSONB NOT NULL DEFAULT '[]'::jsonb,
    completion_criteria JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- Lifecycle
    status              intake_status NOT NULL DEFAULT 'draft',
    version             INTEGER NOT NULL DEFAULT 1,
    supersedes          TEXT REFERENCES proj.intakes(intake_id),

    -- Attribution (per C-EVT-1)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by_kind     TEXT NOT NULL,              -- HUMAN | AGENT | SYSTEM
    created_by_id       TEXT NOT NULL,              -- per SR-SPEC §1.4.2
    activated_at        TIMESTAMPTZ,
    activated_by_kind   TEXT,
    activated_by_id     TEXT,

    -- Event tracking
    last_event_id       TEXT NOT NULL,
    last_global_seq     BIGINT NOT NULL,

    -- Constraints
    CONSTRAINT intake_id_format CHECK (intake_id ~ '^intake:[0-9A-Z]+$'),
    CONSTRAINT content_hash_format CHECK (content_hash IS NULL OR content_hash ~ '^sha256:[a-f0-9]{64}$'),
    CONSTRAINT active_has_content_hash CHECK (status != 'active' OR content_hash IS NOT NULL),
    CONSTRAINT active_has_activation CHECK (status != 'active' OR (activated_at IS NOT NULL AND activated_by_id IS NOT NULL))
);

-- Indexes
CREATE INDEX idx_intakes_work_unit ON proj.intakes(work_unit_id);
CREATE INDEX idx_intakes_status ON proj.intakes(status);
CREATE INDEX idx_intakes_kind ON proj.intakes(kind);
CREATE INDEX idx_intakes_content_hash ON proj.intakes(content_hash) WHERE content_hash IS NOT NULL;

-- Unique constraint: only one active intake per content_hash
CREATE UNIQUE INDEX uniq_intakes_content_hash ON proj.intakes(content_hash) WHERE status = 'active';
```

### 1.5 By-Hash Retrieval Semantics

Per SR-CONTRACT C-EVID-6, archived intakes must remain retrievable:

```rust
/// Get intake by content hash
///
/// Returns intakes of ANY status with matching content_hash.
/// Per C-EVID-6: archived intakes must remain retrievable for
/// evidence availability requirements.
///
/// Returns: Vec<Intake> (may return multiple if same content was
/// activated, archived, and re-activated under different intake_ids)
pub async fn get_by_hash(content_hash: &str) -> Result<Vec<Intake>, Error>;
```

**SQL Query:**
```sql
SELECT * FROM proj.intakes
WHERE content_hash = $1
ORDER BY activated_at DESC;
```

### 1.6 Event Model

Per SR-SPEC Appendix A, all intake lifecycle events:

```rust
// === Intake Events ===

/// Emitted when a new intake is created
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
    pub inputs: Vec<TypedRef>,
    pub unknowns: Vec<String>,
    pub completion_criteria: Vec<String>,
}

/// Emitted when a draft intake is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeUpdated {
    pub intake_id: String,
    /// Fields that changed (delta representation)
    pub changes: IntakeChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    pub inputs: Option<Vec<TypedRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknowns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_criteria: Option<Vec<String>>,
}

/// Emitted when intake transitions to Active (commitment object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeActivated {
    pub intake_id: String,
    /// Content hash computed during activation
    pub content_hash: String,
    /// Canonical JSON used to compute hash (for auditability)
    pub canonical_json_hash: String,
}

/// Emitted when an active intake is archived
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeArchived {
    pub intake_id: String,
    /// Reason for archiving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Emitted when a new draft is forked from an active/archived intake
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
```

**Event Stream Configuration:**
```rust
// Stream kind: INTAKE
// Stream id: intake:{intake_id}

impl IntakeCreated {
    pub fn stream_id(&self) -> String {
        format!("intake:{}", self.intake_id)
    }
    pub fn stream_kind() -> &'static str { "INTAKE" }
    pub fn event_type() -> &'static str { "IntakeCreated" }
}
```

### 1.7 Relationship to templates.rs

**Review finding:** The existing `handlers/templates.rs` handles:
- Template schema definitions (what fields an Intake template requires)
- Template instances (starter/reference templates for users to clone)
- Schema validation helpers

It does **NOT** handle:
- Runtime Intake CRUD operations
- Intake lifecycle state machine (draft → active → archived)
- Intake activation (content-addressing)
- Event emission for intake changes

**Decision:** Create new `handlers/intakes.rs` for runtime operations. The templates handler provides schema scaffolding; the intakes handler provides the runtime API per SR-WORK-SURFACE.

---

## 2. References API

### 2.1 Reference Categories (SR-SPEC §3.2.1.1)

Complete list including Issue #10 additions:

| Category | RefKind | Source | Description |
|----------|---------|--------|-------------|
| Governing Artifacts | `GovernedArtifact` | SR-* docs | Normative governance documents |
| Procedure Templates | `ProcedureTemplate` | proc:* | Stage-gated procedures |
| Oracle Suites | `OracleSuite` | suite:* | Oracle configurations |
| Uploaded Documents | `GovernedArtifact` | User uploads | User-provided context |
| Evidence Bundles | `EvidenceBundle` | Run outputs | Oracle execution evidence |
| Iteration Summaries | `Iteration` | Completed iterations | Loop records |
| Candidates | `Candidate` | Materialized snapshots | Work product snapshots |
| Active Exceptions | `Deviation`/`Deferral`/`Waiver` | Exception records | Active exceptions |
| Intervention Notes | `Record` | Human notes | Human intervention |
| **Agent Definitions** | `AgentDefinition` | config.agent_definition | Agent profiles (Issue #10) |
| **Gating Policies** | `GatingPolicy` | config.gating_policy | Human judgment hooks (Issue #10) |
| Intakes | `Intake` | record.intake | Work unit specifications |

### 2.2 Standardized Response Format (Issue #6)

All References endpoints use this response format:

```typescript
interface ReferencesListResponse {
  refs: TypedRef[];
  total: number;
  page: number;
  page_size: number;
}
```

**JSON Example:**
```json
{
  "refs": [
    {
      "kind": "GovernedArtifact",
      "id": "SR-CONTRACT",
      "rel": "depends_on",
      "meta": {
        "version": "1.0.0",
        "content_hash": "sha256:abc123def456..."
      },
      "label": "Architectural Contract"
    },
    {
      "kind": "Intake",
      "id": "intake:01HQXYZ123ABC",
      "rel": "about",
      "meta": {
        "content_hash": "sha256:789xyz...",
        "type_key": "record.intake"
      },
      "label": "API Rate Limiting Analysis"
    }
  ],
  "total": 42,
  "page": 1,
  "page_size": 20
}
```

### 2.3 References Endpoints

```rust
// All endpoints under /api/v1/references

/// List all refs (paginated, filterable)
/// GET /api/v1/references
/// Query params: kind, rel, page, page_size
pub async fn list_references(
    Query(params): Query<ListRefsParams>,
) -> ApiResult<Json<ReferencesListResponse>>;

/// List governed artifacts
/// GET /api/v1/references/governed-artifacts
pub async fn list_governed_artifacts() -> ApiResult<Json<ReferencesListResponse>>;

/// Get governed artifact detail
/// GET /api/v1/references/governed-artifacts/:id
pub async fn get_governed_artifact(Path(id): Path<String>) -> ApiResult<Json<GovernedArtifactDetail>>;

/// List candidates
/// GET /api/v1/references/candidates
pub async fn list_candidates() -> ApiResult<Json<ReferencesListResponse>>;

/// List evidence bundles
/// GET /api/v1/references/evidence-bundles
pub async fn list_evidence_bundles() -> ApiResult<Json<ReferencesListResponse>>;

/// Get evidence bundle by hash
/// GET /api/v1/references/evidence-bundles/:hash
pub async fn get_evidence_bundle(Path(hash): Path<String>) -> ApiResult<Json<EvidenceBundleDetail>>;

/// List oracle suites
/// GET /api/v1/references/oracle-suites
pub async fn list_oracle_suites() -> ApiResult<Json<ReferencesListResponse>>;

/// List procedure templates
/// GET /api/v1/references/procedure-templates
pub async fn list_procedure_templates() -> ApiResult<Json<ReferencesListResponse>>;

/// List active exceptions (deviation, deferral, waiver)
/// GET /api/v1/references/exceptions
pub async fn list_exceptions() -> ApiResult<Json<ReferencesListResponse>>;

/// List iteration summaries
/// GET /api/v1/references/iteration-summaries
pub async fn list_iteration_summaries() -> ApiResult<Json<ReferencesListResponse>>;

/// List agent definitions (Issue #10)
/// GET /api/v1/references/agent-definitions
pub async fn list_agent_definitions() -> ApiResult<Json<ReferencesListResponse>>;

/// List gating policies (Issue #10)
/// GET /api/v1/references/gating-policies
pub async fn list_gating_policies() -> ApiResult<Json<ReferencesListResponse>>;

/// List intakes
/// GET /api/v1/references/intakes
pub async fn list_intakes() -> ApiResult<Json<ReferencesListResponse>>;

/// Upload a document
/// POST /api/v1/references/documents
pub async fn upload_document(
    Multipart(body): Multipart,
) -> ApiResult<Json<DocumentUploadResponse>>;

/// Get document detail
/// GET /api/v1/references/documents/:id
pub async fn get_document(Path(id): Path<String>) -> ApiResult<Json<DocumentDetail>>;
```

---

## 3. Implementation Phases (Issue #8 — Split Phase 0)

### Phase 0a: Core Infrastructure

**Duration:** Backend foundation
**Dependencies:** None

**Deliverables:**

1. **TypedRef module** (`crates/sr-domain/src/refs.rs`)
   - Unified TypedRef struct
   - RefKind enum with all kinds
   - RefRelation enum (verified against SR-SPEC §1.5.3)
   - RefMeta with validation

2. **Intake domain** (`crates/sr-domain/src/intake.rs`)
   - Intake struct per SR-WORK-SURFACE §3.1
   - IntakeStatus enum
   - WorkKind enum
   - Deliverable struct
   - ActorId struct

3. **Database migrations** (`migrations/`)
   - Create `intake_status` enum
   - Create `work_kind` enum
   - Create `proj.intakes` table
   - Create indexes

4. **Event definitions** (`crates/sr-domain/src/events/intake.rs`)
   - IntakeCreated
   - IntakeUpdated
   - IntakeActivated
   - IntakeArchived
   - IntakeForked

**Verification Checklist:**
- [ ] `cargo build` passes
- [ ] `cargo test` passes
- [ ] Migration applies cleanly: `sqlx migrate run`
- [ ] Types align with SR-WORK-SURFACE §3.1
- [ ] Events align with SR-SPEC Appendix A pattern

---

### Phase 0b: Intake API

**Duration:** Intake CRUD
**Dependencies:** Phase 0a

**Deliverables:**

1. **Intake handler** (`crates/sr-api/src/handlers/intakes.rs`)
   - `POST /api/v1/intakes` — Create draft
   - `GET /api/v1/intakes` — List with filters
   - `GET /api/v1/intakes/:intake_id` — Get by ID
   - `GET /api/v1/intakes/by-hash/:content_hash` — Get by hash (all statuses)
   - `PUT /api/v1/intakes/:intake_id` — Update (draft only)
   - `POST /api/v1/intakes/:intake_id/activate` — Transition to active
   - `POST /api/v1/intakes/:intake_id/archive` — Transition to archived
   - `POST /api/v1/intakes/:intake_id/fork` — Create new draft from active/archived

2. **Activation logic**
   ```rust
   pub async fn activate_intake(intake_id: &str, actor: &ActorId) -> Result<Intake, Error> {
       // 1. Validate all required fields present
       // 2. Compute canonical JSON (sorted keys, no whitespace)
       // 3. Compute content_hash = sha256(canonical_json)
       // 4. Check for hash collision (same hash already exists -> error)
       // 5. Set status = Active, activated_at = now(), activated_by = actor
       // 6. Emit IntakeActivated event (per C-EVT-1)
   }
   ```

3. **Router updates** (`crates/sr-api/src/main.rs`)

**API Examples:**

```bash
# Create intake
POST /api/v1/intakes
{
  "work_unit_id": "wu:research-rate-limiting",
  "title": "API Rate Limiting Analysis",
  "kind": "research_memo",
  "objective": "Evaluate rate limiting strategies for the public API",
  "audience": "Engineering team",
  "deliverables": [
    {"name": "Analysis", "format": "markdown", "path": "candidate/main.md"}
  ],
  "constraints": ["Maximum 2000 words"],
  "definitions": {"rate_limit": "Maximum requests per time window"},
  "inputs": [],
  "unknowns": ["Acceptable latency?"],
  "completion_criteria": ["All strategies evaluated"]
}

# Response
{
  "intake_id": "intake:01HQXYZ123ABC",
  "status": "draft",
  "version": 1,
  "created_at": "2026-01-16T10:00:00Z",
  "created_by": {"actor_kind": "HUMAN", "actor_id": "oidc_sub:..."}
}

# Activate intake
POST /api/v1/intakes/intake:01HQXYZ123ABC/activate

# Response
{
  "intake_id": "intake:01HQXYZ123ABC",
  "status": "active",
  "content_hash": "sha256:abc123def456...",
  "activated_at": "2026-01-16T10:05:00Z"
}
```

**Verification Checklist:**
- [ ] All CRUD endpoints working
- [ ] Activation computes correct content_hash
- [ ] Hash collision detected and rejected
- [ ] Status transitions enforced (draft→active, active→archived)
- [ ] Events emitted correctly
- [ ] By-hash retrieval returns all statuses

---

### Phase 0c: References API

**Duration:** References browser backend
**Dependencies:** Phase 0a (TypedRef)

**Deliverables:**

1. **References handler** (`crates/sr-api/src/handlers/references.rs`)
   - All endpoints from §2.3
   - Standardized response format
   - Pagination support

2. **Reference aggregation service**
   - Query governed artifacts from `proj.governed_artifacts`
   - Query intakes from `proj.intakes`
   - Query exceptions from `proj.exceptions` (existing)
   - Query iterations from `proj.iterations` (existing)
   - Convert all to TypedRef format

3. **Router updates**

**Verification Checklist:**
- [ ] All endpoints return standardized format
- [ ] Pagination works correctly
- [ ] All RefKind categories represented
- [ ] Agent Definitions and Gating Policies included

---

### Phase 1: UI Structure Reorganization

**Duration:** Frontend restructure
**Dependencies:** Phase 0c

**Deliverables:**

1. **Sidebar navigation update**
   ```typescript
   const items = [
     { to: "/overview", label: "Overview", icon: Home },
     { to: "/agents", label: "Agents", icon: Bot },
     { to: "/protocols", label: "Protocols", icon: FileCode },
     { to: "/oracles", label: "Oracles", icon: Shield },
     { to: "/templates", label: "Templates", icon: Layout },
     { to: "/workflows", label: "Workflows", icon: GitBranch },
     { to: "/loops", label: "Loops", icon: RefreshCw },
     { to: "/intakes", label: "Intakes", icon: FileInput },      // NEW
     { to: "/references", label: "References", icon: Library },  // Renamed
     { to: "/prompts", label: "Prompts", icon: MessageSquare },
     { to: "/artifacts", label: "Artifacts", icon: Package },
     { to: "/approvals", label: "Approvals", icon: CheckCircle },
     { to: "/audit", label: "Audit Log", icon: History },
     { to: "/settings", label: "Settings", icon: Settings },
   ];
   ```

2. **Route updates**
   ```typescript
   const routes = [
     // Intakes (NEW)
     { path: "/intakes", element: <Intakes /> },
     { path: "/intakes/new", element: <IntakeCreate /> },
     { path: "/intakes/:intakeId", element: <IntakeDetail /> },
     { path: "/intakes/:intakeId/edit", element: <IntakeEdit /> },

     // References (renamed from Context)
     { path: "/references", element: <References /> },
     { path: "/references/documents/:documentId", element: <ReferenceDocumentDetail /> },
     { path: "/references/bundles/:bundleId", element: <ReferenceBundleDetail /> },
     { path: "/references/governed-artifacts/:artifactId", element: <GovernedArtifactDetail /> },
   ];
   ```

3. **File renames**
   | Current | New |
   |---------|-----|
   | `pages/Context.tsx` | `pages/References.tsx` |
   | `pages/ContextDocumentDetail.tsx` | `pages/ReferenceDocumentDetail.tsx` |
   | `pages/ContextBundleDetail.tsx` | `pages/ReferenceBundleDetail.tsx` |

**Verification Checklist:**
- [ ] Sidebar shows Intakes as separate item
- [ ] References page loads (even with empty data)
- [ ] Routes work correctly
- [ ] No TypeScript errors: `npm run type-check`
- [ ] UI builds: `npm run build`

---

### Phase 2: Intakes Management UI

**Duration:** Full Intake CRUD UI
**Dependencies:** Phase 0b, Phase 1

**Deliverables:**

1. **Intakes list page** (`pages/Intakes.tsx`)
   - Filter by status, kind
   - Search by title
   - Status badges (Draft=yellow, Active=green, Archived=gray)
   - Pagination

2. **Intake create form** (`pages/IntakeCreate.tsx`)
   - All SR-WORK-SURFACE §3.1 fields
   - Deliverables array editor
   - Constraints list editor
   - Definitions key-value editor
   - Input references selector (links to References browser)
   - Unknowns list editor
   - Completion criteria list editor

3. **Intake detail page** (`pages/IntakeDetail.tsx`)
   - Display all fields
   - Lifecycle actions based on status:
     - Draft: [Edit] [Activate] [Delete]
     - Active: [Fork to New Draft] [Archive] [Create Work Surface]
     - Archived: [Fork to New Draft]
   - Display content hash for Active/Archived

4. **Intake edit form** (`pages/IntakeEdit.tsx`)
   - Same as create, but pre-populated
   - Only available for Draft status

**Verification Checklist:**
- [ ] List page displays intakes
- [ ] Create form validates required fields
- [ ] Activation works from UI
- [ ] Status badges display correctly
- [ ] Archive and Fork work correctly

---

### Phase 3: References Browser

**Duration:** References browsing UI
**Dependencies:** Phase 0c, Phase 1

**Deliverables:**

1. **References page** (`pages/References.tsx`)
   - Category sidebar (per §2.1)
   - Category-specific views
   - Search within category
   - Pagination

2. **Reference detail views**
   - GovernedArtifactDetail
   - CandidateDetail
   - EvidenceBundleDetail
   - IntakeDetail (link to /intakes/:id)
   - ExceptionDetail

3. **Dependency graph visualization** (optional, future)

**Verification Checklist:**
- [ ] All categories display
- [ ] Category counts correct
- [ ] Detail views load
- [ ] Pagination works

---

### Phase 4: Work Surface Composition (Future)

**Deferred.** Depends on Phases 0-3. Will enable binding Intake + Procedure Template + Oracle Suite into Work Surface instances.

---

## 4. Verification Checklist (All Phases)

| Check | Command | Expected |
|-------|---------|----------|
| Rust build | `cargo build` | No errors |
| Rust tests | `cargo test` | All pass |
| Clippy | `cargo clippy` | No warnings |
| TypeScript | `cd ui && npm run type-check` | No errors |
| UI build | `cd ui && npm run build` | Success |
| Migrations | `sqlx migrate run` | Applied cleanly |

---

## 5. Intentional Deviations from SR-* Specifications

| Deviation | Specification | Reason |
|-----------|---------------|--------|
| None | N/A | This plan is fully aligned with SR-* specifications |

---

## 6. Governance Document References

| Document | Relevance | Key Sections |
|----------|-----------|--------------|
| SR-CONTRACT | Binding invariants | §2.8 (Commitment Objects), C-EVT-1, C-CTX-2, C-EVID-6 |
| SR-SPEC | Mechanics | §1.5.3 (TypedRef), §3.2.1.1 (Ref Categories), §1.4 (Actor Identity) |
| SR-TYPES | Type taxonomy | §4.4 (`record.intake`), §3.1 (status enum), §4.3 (platform domain types) |
| SR-WORK-SURFACE | Intake schema | §3 (required fields), §4 (Procedure Template), §5 (Work Surface Instance) |
| SR-PROCEDURE-KIT | Procedure templates | §1-2 (stage structure) |
| SR-SEMANTIC-ORACLE-SPEC | Oracle interface | §2-4 (suite identity, outputs) |

---

## 7. User Preferences (Established)

1. **Intakes are a top-level nav item** — Separate from References
2. **Show all intakes** — Regardless of binding status, with status filter
3. **No backward compatibility needed** — Clean implementation aligned with SR-* specs
4. **Backend-first implementation** — Phase 0 before UI phases
5. **"References"** — Acceptable user-facing term (renamed from "Context")
6. **"Prompts"** — Keep as-is, lower priority
