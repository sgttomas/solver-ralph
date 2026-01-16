# SR-PLAN-V2: Intakes and References Implementation Plan

**Status:** Draft - Pending V3 Revision
**Created:** 2026-01-16
**Purpose:** Implementation plan for Intakes UI/API and References browser, aligned with SR-* governance framework.

---

## Executive Summary

This plan implements the UI and API infrastructure for **Intakes** and **References** in SOLVER-Ralph, ensuring semantic consistency with the governed document set (SR-CONTRACT, SR-SPEC, SR-TYPES, SR-WORK-SURFACE, SR-PROCEDURE-KIT, SR-SEMANTIC-ORACLE-SPEC).

**Key architectural decisions:**
1. Intakes are **Commitment Objects** when activated (content-addressed, immutable)
2. References are **Typed Refs** per SR-SPEC §1.5.3 and §3.2.1.1
3. Work Surface = Intake + Procedure Template + Oracle Profile (bound together for iterations)
4. Backend-first implementation (Phase 0) to avoid UI calling non-existent endpoints

---

## Terminology Alignment

| UI Term | SR-* Term | Type Key | Schema Source |
|---------|-----------|----------|---------------|
| Intake | Intake | `record.intake` | SR-WORK-SURFACE §3 |
| References | Typed Refs / `refs[]` | Various | SR-SPEC §1.5.3, §3.2.1.1 |
| Procedure Template | Procedure Template | `config.procedure_template` | SR-WORK-SURFACE §4, SR-PROCEDURE-KIT |
| Oracle Suite | Oracle Suite | N/A (config) | SR-SEMANTIC-ORACLE-SPEC |
| Work Surface | Work Surface | `domain.work_surface` | SR-WORK-SURFACE §5 |

---

## Current State Analysis

### Codebase Exploration Findings

**Frontend:**
- `ui/src/pages/Context.tsx` - Exists, has tabs for Documents/Intakes/Bundles
- `ui/src/pages/IntakeDetail.tsx` - Exists, read-only detail view
- `ui/src/pages/ContextDocumentDetail.tsx` - Exists
- `ui/src/pages/ContextBundleDetail.tsx` - Exists
- `ui/src/layout/Sidebar.tsx` - Shows "Prompts" not "Intakes" as nav item

**Backend:**
- **NO `/api/v1/context` endpoint exists** - Context.tsx calls non-existent API
- **NO `handlers/intakes.rs`** - No intake CRUD backend
- `handlers/templates.rs` - Exists, may handle procedure templates
- `handlers/prompt_loop.rs` - Exists, handles prompt execution

### Key Gap
The UI has intake/context browsing components but no backend API support. All data displayed is either mocked or fails silently.

---

## Phase 0: Backend API Foundation

**Goal:** Create the missing backend endpoints with schemas aligned to SR-* specifications.

### 0.1 Intake API (`handlers/intakes.rs`)

#### Intake Schema (per SR-WORK-SURFACE §3.1)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intake {
    // Identity
    pub intake_id: String,           // format: "intake:<ULID>"
    pub work_unit_id: String,        // format: "wu:<identifier>"
    pub content_hash: Option<String>, // sha256:<hex> - computed on activation

    // Required fields per SR-WORK-SURFACE §3.1
    pub title: String,
    pub kind: WorkKind,              // enum: research_memo, decision_record, ontology_build, etc.
    pub objective: String,           // ONE sentence
    pub audience: String,
    pub deliverables: Vec<Deliverable>,
    pub constraints: Vec<String>,
    pub definitions: HashMap<String, String>,
    pub inputs: Vec<InputRef>,       // typed refs to input artifacts
    pub unknowns: Vec<String>,
    pub completion_criteria: Vec<String>,

    // Lifecycle
    pub status: IntakeStatus,        // draft | active | archived
    pub version: u32,                // increments on fork
    pub supersedes: Option<String>,  // intake_id of prior version (if forked)

    // Attribution (per C-EVT-1)
    pub created_at: DateTime<Utc>,
    pub created_by: ActorId,
    pub activated_at: Option<DateTime<Utc>>,
    pub activated_by: Option<ActorId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    pub name: String,
    pub format: String,              // e.g., "markdown", "json", "pdf"
    pub path: String,                // conventional output path
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRef {
    pub kind: String,                // per SR-SPEC §1.5.3
    pub id: String,
    pub rel: String,                 // depends_on, supported_by, etc.
    pub description: Option<String>,
    pub meta: RefMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefMeta {
    pub content_hash: Option<String>,
    pub version: Option<String>,
    pub selector: Option<String>,    // document anchor, JSON pointer, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntakeStatus {
    Draft,    // Proposal - editable
    Active,   // Commitment Object - immutable, content-addressed
    Archived, // Superseded - historical
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkKind {
    ResearchMemo,
    DecisionRecord,
    OntologyBuild,
    AnalysisReport,
    DesignDocument,
    ReviewResponse,
}
```

> **[REVIEW NOTE - Issue #1]:** This defines `InputRef` separately from `TypedRef` used in References API. Per SR-SPEC §1.5.3, there should be ONE canonical ref schema. Resolution: Use single `TypedRef` throughout, including for `Intake.inputs[]`.

> **[REVIEW NOTE - Issue #3]:** No PostgreSQL schema defined. Must add `proj.intakes` table definition.

> **[REVIEW NOTE - Issue #4]:** Status uses `Draft | Active | Archived` but SR-TYPES §3.1 uses `draft | governed | superseded | deprecated | archived`. Need alignment or documented deviation.

#### Endpoints

| Method | Path | Description | Constraints |
|--------|------|-------------|-------------|
| POST | `/api/v1/intakes` | Create draft intake | Returns `intake_id` |
| GET | `/api/v1/intakes` | List intakes | Filterable by status, kind, work_unit_id |
| GET | `/api/v1/intakes/:intake_id` | Get intake by ID | |
| GET | `/api/v1/intakes/by-hash/:content_hash` | Get by content hash | For dereferencing refs |
| PUT | `/api/v1/intakes/:intake_id` | Update intake | Only if `status = Draft` |
| POST | `/api/v1/intakes/:intake_id/activate` | Transition to Active | Computes `content_hash`, sets immutable |
| POST | `/api/v1/intakes/:intake_id/archive` | Transition to Archived | Only if `status = Active` |
| POST | `/api/v1/intakes/:intake_id/fork` | Create new Draft from Active | Sets `supersedes` |

#### Activation Logic (Proposal -> Commitment Object)

1. Validate all required fields present
2. Compute canonical JSON (sorted keys, no whitespace)
3. Compute `content_hash = sha256(canonical_json)`
4. Check for hash collision (same hash already exists -> error)
5. Set `status = Active`, `activated_at = now()`, `activated_by = actor`
6. Emit `IntakeActivated` event (per C-EVT-1)

> **[REVIEW NOTE - Issue #2]:** Events mentioned but not specified. Must define full event model: `IntakeCreated`, `IntakeUpdated`, `IntakeActivated`, `IntakeArchived`, `IntakeForked` with payload schemas.

> **[REVIEW NOTE - Issue #5]:** Clarify by-hash retrieval semantics for archived intakes. Per C-EVID-6, must remain retrievable.

---

### 0.2 References API (`handlers/references.rs`)

#### TypedRef Schema (per SR-SPEC §1.5.3)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedRef {
    pub kind: RefKind,
    pub id: String,
    pub rel: RefRelation,
    pub meta: RefMeta,
    pub label: Option<String>,       // human-readable label
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefKind {
    GovernedArtifact,
    Candidate,
    OracleSuite,
    EvidenceBundle,
    Approval,
    Decision,
    Deviation,
    Deferral,
    Waiver,
    Iteration,
    Loop,
    Intake,
    ProcedureTemplate,
    Record,  // with meta.type_key for subtyping
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
```

> **[REVIEW NOTE - Issue #7]:** Verify RefRelation enum includes ALL relations from SR-SPEC §1.5.3.

#### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/references` | List all refs (paginated, filterable by kind) |
| GET | `/api/v1/references/governed-artifacts` | List governed artifacts |
| GET | `/api/v1/references/governed-artifacts/:id` | Get governed artifact detail |
| GET | `/api/v1/references/candidates` | List candidates |
| GET | `/api/v1/references/candidates/:id` | Get candidate detail |
| GET | `/api/v1/references/evidence-bundles` | List evidence bundles |
| GET | `/api/v1/references/evidence-bundles/:hash` | Get evidence bundle by hash |
| GET | `/api/v1/references/oracle-suites` | List oracle suites |
| GET | `/api/v1/references/procedure-templates` | List procedure templates |
| GET | `/api/v1/references/exceptions` | List active exceptions |
| GET | `/api/v1/references/iteration-summaries` | List iteration summaries |
| POST | `/api/v1/references/documents` | Upload a document |
| GET | `/api/v1/references/documents/:id` | Get document detail |

> **[REVIEW NOTE - Issue #6]:** Standardize all responses to `{ refs: TypedRef[], total, page, page_size }`.

> **[REVIEW NOTE - Issue #10]:** Missing Agent Definitions and Gating Policies endpoints per SR-SPEC §3.2.1.1 ref categories.

#### Response Format

```json
{
  "refs": [
    {
      "kind": "GovernedArtifact",
      "id": "SR-CONTRACT",
      "rel": "depends_on",
      "meta": {
        "version": "1.0.0",
        "content_hash": "sha256:abc123..."
      },
      "label": "Architectural Contract",
      "created_at": "2026-01-15T10:00:00Z"
    }
  ],
  "total": 15,
  "page": 1,
  "page_size": 20
}
```

---

### 0.3 Procedure Templates API (`handlers/procedure_templates.rs`)

> **[REVIEW NOTE - Issue #9]:** Review existing `handlers/templates.rs` before creating new handler. May only need extension.

#### Schema (per SR-WORK-SURFACE §4, SR-PROCEDURE-KIT)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureTemplate {
    pub procedure_template_id: String,  // format: "proc:<NAME>"
    pub name: String,
    pub description: String,
    pub supported_kinds: Vec<WorkKind>,
    pub stages: Vec<ProcedureStage>,
    pub terminal_stage_id: String,
    pub version: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureStage {
    pub stage_id: String,            // format: "stage:<NAME>"
    pub stage_name: String,
    pub purpose: String,
    pub required_outputs: Vec<RequiredOutput>,
    pub required_oracle_suites: Vec<String>,
    pub gate_rule: GateRule,
    pub transition_on_pass: Option<String>,
}
```

#### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/procedure-templates` | List procedure templates |
| GET | `/api/v1/procedure-templates/:id` | Get template detail |
| POST | `/api/v1/procedure-templates` | Create template (human-approved) |

---

### 0.4 Router Updates (`main.rs`)

```rust
let app = Router::new()
    // Intakes (per SR-WORK-SURFACE)
    .route("/api/v1/intakes", get(intakes::list).post(intakes::create))
    .route("/api/v1/intakes/:intake_id", get(intakes::get).put(intakes::update))
    .route("/api/v1/intakes/by-hash/:content_hash", get(intakes::get_by_hash))
    .route("/api/v1/intakes/:intake_id/activate", post(intakes::activate))
    .route("/api/v1/intakes/:intake_id/archive", post(intakes::archive))
    .route("/api/v1/intakes/:intake_id/fork", post(intakes::fork))

    // References (typed refs per SR-SPEC)
    .route("/api/v1/references", get(references::list))
    .route("/api/v1/references/governed-artifacts", get(references::list_governed_artifacts))
    // ... remaining reference routes ...

    // Procedure Templates (per SR-PROCEDURE-KIT)
    .route("/api/v1/procedure-templates", get(procedure_templates::list).post(procedure_templates::create))
    .route("/api/v1/procedure-templates/:id", get(procedure_templates::get));
```

> **[REVIEW NOTE - Issue #8]:** Phase 0 scope is large. Consider splitting into 0a (Intakes), 0b (existing refs), 0c (new refs).

---

## Phase 1: UI Structure Reorganization

**Goal:** Align UI navigation and routing with spec semantics.

### 1.1 Sidebar Navigation Update

```typescript
const items = [
  { to: "/overview", label: "Overview", icon: Home },
  { to: "/agents", label: "Agents", icon: Bot },
  { to: "/protocols", label: "Protocols", icon: FileCode },
  { to: "/oracles", label: "Oracles", icon: Shield },
  { to: "/templates", label: "Templates", icon: Layout },
  { to: "/workflows", label: "Workflows", icon: GitBranch },
  { to: "/loops", label: "Loops", icon: RefreshCw },
  { to: "/intakes", label: "Intakes", icon: FileInput },      // NEW - top-level
  { to: "/references", label: "References", icon: Library },  // Renamed from Context
  { to: "/prompts", label: "Prompts", icon: MessageSquare },  // Keep (lower priority)
  { to: "/artifacts", label: "Artifacts", icon: Package },
  { to: "/approvals", label: "Approvals", icon: CheckCircle },
  { to: "/audit", label: "Audit Log", icon: History },
  { to: "/settings", label: "Settings", icon: Settings },
];
```

### 1.2 Route Updates

```typescript
const routes = [
  // Intakes (NEW - top-level per Work Surface)
  { path: "/intakes", element: <Intakes /> },
  { path: "/intakes/new", element: <IntakeCreate /> },
  { path: "/intakes/:intakeId", element: <IntakeDetail /> },
  { path: "/intakes/:intakeId/edit", element: <IntakeEdit /> },

  // References (renamed from Context)
  { path: "/references", element: <References /> },
  { path: "/references/documents/:documentId", element: <ReferenceDocumentDetail /> },
  { path: "/references/bundles/:bundleId", element: <ReferenceBundleDetail /> },
  { path: "/references/governed-artifacts/:artifactId", element: <GovernedArtifactDetail /> },

  // Keep prompts
  { path: "/prompts", element: <PromptLoop /> },
];
```

### 1.3 File Renames

| Current | New | Notes |
|---------|-----|-------|
| `pages/Context.tsx` | `pages/References.tsx` | Remove Intakes tab |
| `pages/ContextDocumentDetail.tsx` | `pages/ReferenceDocumentDetail.tsx` | |
| `pages/ContextBundleDetail.tsx` | `pages/ReferenceBundleDetail.tsx` | |
| N/A | `pages/Intakes.tsx` | NEW |
| N/A | `pages/IntakeCreate.tsx` | NEW |
| N/A | `pages/IntakeEdit.tsx` | NEW |

---

## Phase 2: Intakes Management UI

**Goal:** Full CRUD for Intake records per SR-WORK-SURFACE §3.

### 2.1 Intakes List Page (`pages/Intakes.tsx`)

```
+---------------------------------------------------------------------+
| Intakes                                            [+ New Intake]   |
+---------------------------------------------------------------------+
| Filter: [All Statuses v] [All Kinds v] [Search...]                  |
+---------------------------------------------------------------------+
| ID          Title           Kind       Status    Created            |
|---------------------------------------------------------------------|
| intake:001  API Design...   decision   Active    2026-01-10         |
| intake:002  Research on...  research   Draft     2026-01-15         |
| intake:003  Ontology for... ontology   Archived  2026-01-05         |
+---------------------------------------------------------------------+
| Showing 1-10 of 23                          [< Prev] [Next >]       |
+---------------------------------------------------------------------+
```

**Status badges:**
- Draft: Yellow outline, "Proposal" tooltip
- Active: Green solid, "Commitment Object" tooltip
- Archived: Gray, "Superseded" tooltip

### 2.2 Intake Create/Edit Form

Form fields per SR-WORK-SURFACE §3.1:

- Work Unit ID (required)
- Title (required)
- Kind (dropdown, required)
- Objective (required, one sentence)
- Audience (required)
- Deliverables[] (name, format, path, description)
- Constraints[] (string list)
- Definitions{} (key-value pairs)
- Input References[] (kind, id, rel, meta)
- Unknowns[] (string list)
- Completion Criteria[] (string list)

### 2.3 Intake Detail Page

Enhanced with lifecycle actions:
- **Draft:** [Edit] [Activate] [Delete]
- **Active:** [Fork to New Draft] [Archive] [Create Work Surface]
- **Archived:** [Fork to New Draft] (read-only otherwise)

Display content hash for Active/Archived intakes.

### 2.4 Intake Lifecycle State Machine

```
       +------------------+
       |                  |
   +---v---+         +----+----+
   | DRAFT |---------| ACTIVE  |
   +-------+ activate+----+----+
       ^                  | archive
       | fork             v
       |             +---------+
       +-------------+ARCHIVED |
                     +---------+
```

---

## Phase 3: References Browser

**Goal:** Browsing typed refs per SR-SPEC §3.2.1.1.

### 3.1 References Page Categories

Per SR-SPEC §3.2.1.1, display these ref categories:

| Category | Icon | Kind | Source |
|----------|------|------|--------|
| Governing Artifacts | doc | `GovernedArtifact` | SR-* docs |
| Procedure Templates | list | `ProcedureTemplate` | proc:* |
| Oracle Suites | shield | `OracleSuite` | suite:* |
| Uploaded Documents | paperclip | `GovernedArtifact` | User uploads |
| Evidence Bundles | package | `EvidenceBundle` | Run outputs |
| Iteration Summaries | refresh | `Iteration` | Completed iterations |
| Candidates | file | `Candidate` | Materialized snapshots |
| Active Exceptions | warning | `Deviation|Deferral|Waiver` | Exception records |
| Intervention Notes | message | `Record` | Human notes |

### 3.2 Reference Detail Views

Each ref kind gets a detail view showing:
- Full metadata (kind, id, rel, meta)
- Content preview (if applicable)
- Dependency graph (what depends on this, what this depends on)
- Usage history (which iterations referenced this)

---

## Phase 4: Work Surface Composition (Future)

**Goal:** Enable binding Intake + Procedure Template + Oracle Suite into Work Surface instances.

Deferred to future planning cycle. Depends on Phases 0-3.

---

## Verification Checklist

After each phase:

| Check | Command | Expected |
|-------|---------|----------|
| Rust build | `cargo build` | No errors |
| Rust tests | `cargo test` | All pass |
| TypeScript | `cd ui && npm run type-check` | No errors |
| UI build | `cd ui && npm run build` | Success |

---

## Review Issues Summary

### High Priority (Semantic/Ontological)

| # | Issue | Section | Resolution Required |
|---|-------|---------|---------------------|
| 1 | Separate `InputRef` and `TypedRef` schemas | §0.1 | Unify to single `TypedRef` |
| 2 | Events mentioned but not specified | §0.1 | Define full event model with payloads |
| 3 | No PostgreSQL schema for intakes | §0.1 | Add `proj.intakes` table definition |
| 4 | Status terminology mismatch | §0.1 | Align with SR-TYPES or document deviation |

### Medium Priority (Epistemological)

| # | Issue | Section | Resolution Required |
|---|-------|---------|---------------------|
| 5 | By-hash retrieval semantics unclear | §0.1 | Clarify behavior for all statuses |
| 6 | References API response format inconsistent | §0.2 | Standardize to `{ refs, total, page, page_size }` |
| 7 | RefRelation enum may be incomplete | §0.2 | Verify against SR-SPEC §1.5.3 |

### Lower Priority (Implementation Practicality)

| # | Issue | Section | Resolution Required |
|---|-------|---------|---------------------|
| 8 | Phase 0 scope too large | §0 | Split into sub-phases 0a, 0b, 0c |
| 9 | May duplicate existing templates.rs | §0.3 | Review before creating new handler |
| 10 | Missing ref categories | §0.2 | Add Agent Definitions, Gating Policies |

---

## Governance Document References

| Document | Relevance | Key Sections |
|----------|-----------|--------------|
| SR-CONTRACT | Binding invariants | §2.8 (Commitment Objects), C-EVT-1, C-CTX-2 |
| SR-SPEC | Mechanics | §1.5.3 (TypedRef), §3.2.1.1 (Ref Categories) |
| SR-TYPES | Type taxonomy | §4.4 (`record.intake`), §3.1 (status enum) |
| SR-WORK-SURFACE | Intake schema | §3 (required fields), §4 (Procedure Template) |
| SR-PROCEDURE-KIT | Procedure templates | §1-2 (stage structure) |
| SR-SEMANTIC-ORACLE-SPEC | Oracle interface | §2-4 (suite identity, outputs) |

---

## User Preferences (Established)

1. **Intakes are a top-level nav item** - Separate from References
2. **Show all intakes** - Regardless of binding status, with status filter
3. **No backward compatibility needed** - Clean implementation aligned with SR-* specs
4. **Backend-first implementation** - Phase 0 before UI phases
5. **"References"** - Acceptable user-facing term (renamed from "Context")
6. **"Prompts"** - Keep as-is, lower priority
