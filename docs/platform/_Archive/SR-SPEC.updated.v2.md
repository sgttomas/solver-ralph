---
doc_id: SR-SPEC
doc_kind: governance.platform_spec
layer: platform
status: draft
refs:
  - rel: governed_by
    to: SR-CHANGE
  - rel: depends_on
    to: SR-CONTRACT
  - rel: depends_on
    to: SR-TYPES
---

# SOLVER-Ralph Technical Specification 

**Purpose:** Define the technical implementation for SOLVER-Ralph as a governance-first, event-sourced, evidence-backed platform for controlled agentic work.

**Normative Status:** **Normative (binding)** within the scope of the SOLVER-Ralph governed set.

**Satisfies Contracts:** C-ARCH-1..C-ARCH-3; C-TB-1..C-TB-7; C-LOOP-1..C-LOOP-4; C-CTX-1..C-CTX-2; C-EVT-1..C-EVT-7; C-EVID-1..C-EVID-6; C-OR-1..C-OR-7; C-VER-1..C-VER-4; C-EXC-1..C-EXC-5; C-SHIP-1; C-DEC-1; C-META-1..C-META-3.

---


## 1. Data Model

This specification defines **Layer 2** of SOLVER-Ralph: the mechanics that implement the invariants in SR-CONTRACT.

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: Building SOLVER-Ralph                                 │
│                                                                  │
│  This specification constrains agents building the platform.    │
│  At this layer, SR-SPEC is a document to be implemented.        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ becomes
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 2: The Platform (what this specification defines)        │
│                                                                  │
│  The mechanics in this specification become CODE:               │
│  - Event schemas → Rust structs                                 │
│  - State machines → Domain core logic                           │
│  - API endpoints → Axum handlers                                │
│  - Projections → PostgreSQL tables                              │
│  - Evidence storage → MinIO adapters                            │
│                                                                  │
│  The running platform embodies these mechanics.                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ enables
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 3: Usage                                                 │
│                                                                  │
│  Users interact with the platform via APIs and UI.              │
│  They experience these mechanics as platform behavior.          │
└─────────────────────────────────────────────────────────────────┘
```

**Implication:** Every schema, state machine, and API endpoint defined here must be implemented. If a mechanic cannot be implemented as specified, it requires a change to this specification via SR-CHANGE.


### 1.1 Normative keywords

The words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative and interpreted as in RFC 2119.

### 1.2 Terminology (normative)

This specification uses the SOLVER-Ralph Contract terms as normative:

- **Candidate:** a content-addressable snapshot of work products; unit of verification and approval.
- **Run:** an execution of an oracle suite against a specific Candidate, producing an Evidence Bundle.
- **Evidence Bundle:** structured evidence artifact produced by a Run (canonical type: `domain.evidence_bundle (canonical type key for Evidence Bundle)`).
- **Oracle Suite:** a named set of oracle definitions with stable identity and deterministic hash; includes environment constraints.
- **Ralph Loop:** bounded workflow instance with goal, budgets, stop-the-line triggers, and controlled memory.
- **Iteration:** a fresh-context cycle within a Loop.
- **Portal:** a Gate where human arbitration is required; trust boundary.
- **Approval:** binding, attributable human decision at a Portal.
- **Deviation / Deferral / Gate Waiver:** binding exception records.

#### 1.2.1 Proposal vs Commitment Object (normative)

Per SR-CONTRACT, the system MUST distinguish:

- **Proposal:** any draft, statement, or work product whose meaning is not yet stabilized as a durable, attributable object in the system. Proposals are non-authoritative and MUST NOT be the substrate for binding state.
- **Commitment Object:** a durable, content-addressed, referenceable object that downstream work can cite without relying on implicit trust. Commitment objects include governed artifacts, Candidates, Evidence Bundles, Approvals, Freeze Records, Decision Records, and Exception Records.

**Constraint:** binding claims (Verified, Approved, Shippable) MUST be derivable only from commitment objects referenced in the event/graph substrate.

#### 1.2.2 Verification, Evaluation, Validation, Assessment (normative)

Per SR-CONTRACT, the system MUST distinguish:

| Term | Actor | Output | Binding authority? |
|------|-------|--------|-------------------|
| **Verification** | Agentic (oracle) | Evidence about conformance | No |
| **Evaluation** | Human | Interpretation of verification evidence | No |
| **Validation** | Agentic (oracle) | Evidence about fitness in context | No |
| **Assessment** | Human | Interpretation of validation evidence | No |
| **Approval** | Human at Portal | Authorization to proceed | **Yes** |

**Constraint:** Evaluation and Assessment records MUST NOT be treated as substitutes for Portal Approval.

#### 1.2.3 Platform Domain Types (alignment with SR-TYPES §4.3)

The platform tracks the following domain entities at runtime. These correspond to SR-TYPES §4.3 platform domain types:

| SR-TYPES Type Key | SR-SPEC Entity | Primary Representation |
|-------------------|----------------|----------------------|
| `domain.work_unit` | Work Unit (Loop) | Loop/WorkUnit state machine + projections |
| `domain.work_surface` | Work Surface | `WorkSurfaceRecorded` event + projection (references Intake + Procedure Template + stage) |
| `domain.candidate` | Candidate | `CandidateMaterialized` event + projection |
| `domain.evidence_bundle` | Evidence Bundle | `EvidenceBundleRecorded` event + blob store |
| `domain.portal_decision` | Approval | `ApprovalRecorded` event + projection |
| `domain.loop_record` | Iteration Summary | `IterationSummaryRecorded` event + projection |
| `domain.event` | Event | Event store record |

The schemas defined in this specification (§1.5 Event model, §1.6 Postgres schema, §1.7 projections, §1.8 graph) are the normative definitions for these platform domain types.

#### 1.2.4 Work Surface and stage-gated procedures (normative)

For Semantic Ralph Loops, the platform MUST treat the following as first-class, referenceable artifacts:

- **Intake:** the structured statement of a work unit’s objective, scope, constraints, definitions, and required outputs. Intake is a commitment object when it is used as binding context for a loop.
- **Procedure Template:** a stage-gated procedure that specifies the required intermediate artifacts, required oracle suites, and gate criteria for each stage.
- **Procedure Stage (`stage_id`):** the named gate within a Procedure Template that the iteration is currently targeting. Stage identity participates in verification scope and evidence binding.
- **Work Surface:** the set of governed references that define what work is being done and how it is evaluated: (Intake + Procedure Template + current stage + selected oracle profile/suites + any stage parameters).

#### 1.2.5 Semantic oracles and semantic sets (normative)

A **semantic oracle** is an oracle whose result record may include structured semantic measurements (e.g., residual vectors, coverage metrics, constraint violations) derived from an ontology / meaning-matrix / semantic set definition.

Normative requirements:

- A semantic oracle MUST emit a machine-readable result record.
- If a PASS/FAIL outcome is used for gates, it MUST be computable from the result record using declared decision rules.
- The oracle suite identity/hash MUST incorporate any semantic set / meaning-matrix definitions that materially affect evaluation.

#### 1.2.6 Semantic Ralph Loop (normative)

A **Semantic Ralph Loop** is a Ralph Loop whose candidates are primarily semantic artifacts (documents, structured analyses, decision records, ontological structures). It relies on stage-gated procedures and semantic oracle suites to produce evidence, rather than assuming a compiler/test harness exists.

### 1.3 Canonical identifiers and hashes

#### 1.3.1 ULID identifiers

The system MUST use ULIDs for high-cardinality internal identifiers to preserve temporal ordering without relying on wall-clock timestamps.

**Format (normative):**

- `loop_id`: `loop_<ULID>`
- `iteration_id`: `iter_<ULID>`
- `run_id`: `run_<ULID>`
- `approval_id`: `appr_<ULID>`
- `decision_id`: `dec_<ULID>`
- `exception_id`: `exc_<ULID>`
- `event_id`: `evt_<ULID>`
- `freeze_id`: `freeze_<ULID>`
- `stale_id`: `stale_<ULID>`

#### 1.3.2 Content hashes

All immutable content-addressed artifacts MUST use `sha256` digests over the canonical bytes of the stored object.

**Format:**
- `sha256:<64-hex>`

**Canonicalization rules (normative):**

- For **binary** content: hash raw bytes.
- For **JSON** manifests: hash UTF-8 bytes of **canonical JSON** (sorted keys, no insignificant whitespace).
- For **text** (e.g., Markdown evidence attachments): hash UTF-8 bytes with LF line endings.


#### 1.3.3 Candidate identity (git + manifest + sha256)

A 
**Stages / Work Surface**
- `WorkSurfaceRecorded`
- `ProcedureTemplateSelected`
- `StageEntered`
- `StageCompleted`
- `SemanticOracleEvaluated`

**Candidate** is a content-addressable snapshot. Candidate identity MUST be stable and immutable for the scope claimed.

**Normative policy:**

- `candidate_id` MUST include a cryptographic digest component: `sha256:<digest>`.
- `candidate_id` SHOULD include a VCS pointer component when applicable: `git:<commit_sha>`.
- When a Candidate cannot be represented as a single VCS commit (common for knowledge-work artifacts), the system MUST materialize a **Candidate Manifest** (a canonical, sorted listing of included artifacts and their content hashes) and the `sha256:<digest>` component MUST be computed over the canonical manifest representation.
- The system MUST NOT blindly trust agent-declared candidate identity. Candidate identity MUST be computed or verified by deterministic platform code. A mismatch is an integrity condition that blocks Verified/Shippable claims.

### 1.4 Actor identity model

#### 1.4.1 Actor kinds

`actor_kind` MUST be one of:

- `HUMAN`
- `AGENT`
- `SYSTEM`

#### 1.4.2 Stable actor identity

`actor_id` MUST be stable, auditable, and verifiable for governance-relevant actions.

**Normative identity formats:**

- Humans: `oidc_sub:<issuer_hash>:<subject>`  
  Where `<issuer_hash>` is `sha256` of the OIDC issuer URL, and `<subject>` is the OIDC `sub` claim from Zitadel.
- System services: `svc:<service_name>:<key_id>`
- Agents: `agent:<deployment_id>:<agent_instance_id>`

The system MUST record `actor_kind` + `actor_id` on every governance-relevant event and every evidence attribution record.

### 1.5 Event model

#### 1.5.1 Event store as source of truth

The append-only event log is the **sole source of truth** for governance-relevant state. Derived/query stores MUST be reconstructible solely from the event log.

#### 1.5.2 Event envelope (v1)

Every persisted event MUST conform to the following envelope:

```jsonc
{
  "event_id": "evt_01J...ULID",
  "stream_id": "loop:loop_01J...",
  "stream_kind": "LOOP|ITERATION|CANDIDATE|RUN|APPROVAL|DECISION|GOVERNANCE|EXCEPTION|ORACLE_SUITE|FREEZE",
  "stream_seq": 42,
  "global_seq": 184467,              // REQUIRED in this SR-SPEC (PostgreSQL reference implementation)
  "event_type": "LoopCreated",        // namespaced in code (see Appendix A)
  "occurred_at": "2026-01-09T12:34:56Z",
  "actor_kind": "SYSTEM",
  "actor_id": "svc:loop-governor:key_01",
  "correlation_id": "corr_01J...",    // optional
  "causation_id": "evt_01J...",       // optional
  "supersedes": ["evt_01J..."],       // optional; explicit correction linkage
  "refs": [                           // typed references (dependency + audit)
    {"kind": "Candidate", "id": "git:abcd...|sha256:...", "rel": "about", "meta": {"content_hash": "sha256:..."}},
    {"kind": "EvidenceBundle", "id": "sha256:...", "rel": "produces", "meta": {"content_hash": "sha256:..."}}
  ],
  "payload": { /* event-specific */ },
  "envelope_hash": "sha256:..."       // hash of canonical JSON excluding this field
}
```

Portability note: some deployments may omit `global_seq`. In this SR-SPEC, the PostgreSQL reference implementation uses `global_seq` as the authoritative total order for replay.

**Normative requirements:**

- `stream_seq` MUST be strictly increasing **per stream** and is the ordering authority for that stream.
- The event store MUST reject writes that would create duplicate `(stream_id, stream_seq)` pairs.
- Events MUST NOT be updated or deleted. Corrections MUST be represented as new events with `supersedes` populated.
- `occurred_at` MUST be recorded but MUST NOT be used as the primary ordering mechanism (sequence-first ordering).

#### 1.5.3 Typed references (refs)

Every governance-relevant event MAY carry `refs[]` to other domain nodes. `refs[]` serve two distinct purposes:

1) **Dependency semantics** (staleness propagation, Shippable gating) via `rel=depends_on`  
2) **Audit provenance** (what was consulted / used) via `rel=supported_by`

The `refs` array MUST use the following shape:

```jsonc
{
  "kind": "GovernedArtifact|Candidate|OracleSuite|EvidenceBundle|Approval|Record|Decision|Deviation|Deferral|Waiver|Loop|Iteration|Run|Freeze",
  "id": "string",
  "rel": "about|depends_on|supported_by|produces|verifies|approved_by|acknowledges|supersedes|releases|governed_by|in_scope_of|affects|stale|root_cause|relates_to",
  "meta": { "key": "value" }
}
```

Deprecated tokens: emitters MUST NOT use `rel=produced` or `rel=approves`; use `rel=produces` and `rel=approved_by`.

Typed references MUST be sufficient to answer dependency and audit questions without requiring unstructured log scraping.

##### 1.5.3.1 `meta` requirements (normative)

`meta` is REQUIRED (use `{}` if no keys apply). For any dereferenceable reference (where the system can fetch bytes/content), `meta.content_hash` is REQUIRED.

- **`meta.content_hash` (required for dereferenceable refs):** `sha256:<64-hex>`
  - REQUIRED for `kind` in: `GovernedArtifact`, `Candidate`, `OracleSuite`, `EvidenceBundle`, `Record`
  - MUST match the canonical bytes of the referenced object (as defined for that object type).
- **`meta.version` (required for governed artifacts):** SemVer string
  - REQUIRED when `kind=GovernedArtifact` (unless the version is fully encoded into `id`).
- **`meta.type_key` (required for record refs):** SR-TYPES type key (e.g., `record.intervention_note`)
  - REQUIRED when `kind=Record`.
  - MUST match a defined SR-TYPES type key so record subtyping is explicit and auditable.
- **`meta.selector` (optional, strongly recommended):** stable semantic slice selector
  - Examples: document anchor/section id, JSON pointer, byte-range. This enables replayable audits without embedding entire documents in events.

##### 1.5.3.2 Relationship semantics (normative)

**Contract alignment note:** This section implements SR-CONTRACT C-EVT-5 and C-EVT-6 by distinguishing
**semantic dependency** vs **audit provenance** and constraining default staleness traversal
to semantic dependencies.

- **`rel=depends_on`** → semantic dependency
  - participates in staleness traversal and Shippable gating (blocking by default)
  - use for inputs that, if changed, should require re-evaluation (governed artifacts in force, base candidates, oracle suites, active exceptions, carried-forward iteration summaries, etc.)
- **`rel=supported_by`** → audit-only provenance
  - does **not** participate in staleness traversal by default (non-blocking)
  - use for inputs that explain work but should not invalidate downstream artifacts if they change later (agent definitions, prompt templates, non-binding reviews, etc.)

**Constraint:** `rel=governed_by` is a label for governance relationships and MUST NOT be relied upon for staleness propagation. If a reference is intended to participate in dependency semantics, use `rel=depends_on`.

### 1.6 PostgreSQL event store schema

#### 1.6.1 Schema overview

All authoritative persistence is in schema `es` (event store). Projections live in schema `proj`. The graph projection lives in schema `graph`.

#### 1.6.2 Tables (normative)

**Streams**

```sql
CREATE TABLE es.streams (
  stream_id        text PRIMARY KEY,
  stream_kind      text NOT NULL,
  stream_version   bigint NOT NULL DEFAULT 0,
  created_at       timestamptz NOT NULL DEFAULT now()
);
```

**Events (append-only)**

```sql
CREATE TABLE es.events (
  global_seq       bigserial PRIMARY KEY,
  event_id         text NOT NULL UNIQUE,
  stream_id        text NOT NULL REFERENCES es.streams(stream_id),
  stream_seq       bigint NOT NULL,
  occurred_at      timestamptz NOT NULL,
  actor_kind       text NOT NULL,
  actor_id         text NOT NULL,
  event_type       text NOT NULL,
  correlation_id   text,
  causation_id     text,
  supersedes       text[],
  refs             jsonb NOT NULL DEFAULT '[]'::jsonb,
  payload          jsonb NOT NULL,
  envelope_hash    text NOT NULL,
  inserted_at      timestamptz NOT NULL DEFAULT now(),
  UNIQUE (stream_id, stream_seq)
);
```

**Outbox (for NATS publication)**

```sql
CREATE TABLE es.outbox (
  outbox_id        bigserial PRIMARY KEY,
  global_seq       bigint NOT NULL,              -- FK to es.events.global_seq (logical)
  published_at     timestamptz,
  topic            text NOT NULL,
  message          jsonb NOT NULL,
  message_hash     text NOT NULL                 -- sha256 over canonical JSON(message)
);
CREATE INDEX ON es.outbox (published_at) WHERE published_at IS NULL;
```

#### 1.6.3 Event append transaction (normative)

To append events to a stream, the adapter MUST:

1. `INSERT ... ON CONFLICT DO NOTHING` into `es.streams` (or create stream ahead of time).
2. `SELECT stream_version FROM es.streams WHERE stream_id = $1 FOR UPDATE`.
3. Validate expected version (optimistic concurrency).
4. Insert N events with sequential `stream_seq = stream_version + 1 .. stream_version + N`.
5. Update `es.streams.stream_version = stream_version + N`.
6. Insert corresponding outbox rows in the **same transaction**.

### 1.7 Projection model

#### 1.7.1 Rebuildability guarantee

Every projection MUST be rebuildable from `es.events` without additional sources of truth.

#### 1.7.2 Projection patterns

Two patterns are permitted:

- **Synchronous projection:** updated in the same DB transaction as event append (simple, strong consistency).
- **Asynchronous projection:** projector consumes the outbox/NATS stream and updates projections (eventual consistency, higher throughput).

The initial build implementation SHOULD use synchronous projections for `proj.*` tables and asynchronous projection for `graph.*` if needed.



#### 1.7.3 Event Manager and eligibility projection (normative)

The platform MUST provide a deterministic **event manager / projection builder** capable of computing:

- current **work unit status** (e.g., TODO / IN_PROGRESS / BLOCKED / COMPLETE; and, for stage-gated procedures, the current `stage_id` and stage completion status),
- the **eligible set** of work units based on SR-PLAN `depends_on` relationships and recorded completion events,
- stop-trigger derived blocking states (e.g., REPEATED_FAILURE, ORACLE_GAP, BUDGET_EXHAUSTED).

These projections MUST be rebuildable from the ordered event stream alone (no hidden state), consistent with §1.7.1.

Implementation details MAY vary (synchronous or asynchronous projection), but the computed results MUST be deterministic functions of the event stream + governed inputs (SR-PLAN instance and SR-DIRECTIVE policy). See SR-EVENT-MANAGER for the normative “what” of these projections.

### 1.8 Dependency graph model (PostgreSQL)

#### 1.8.1 Node types

The graph MUST support, at minimum, the following semantic node categories:

- `GovernedArtifact`
- `WorkSurface` (Intake + Procedure Template + stage context)
- `Intake`
- `ProcedureTemplate`
- `ProcedureStage`
- `SemanticSet` (meaning matrix / semantic set definition used by semantic oracles)
- `Candidate`
- `OracleSuite`
- `EvidenceBundle`
- `Approval`
- `Decision`
- `Deviation`
- `Deferral`
- `Waiver`
- `Loop`

#### 1.8.2 Tables (normative)

```sql
CREATE TABLE graph.nodes (
  node_id     text PRIMARY KEY,
  node_type   text NOT NULL,
  label       text,
  created_at  timestamptz NOT NULL DEFAULT now(),
  meta        jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE graph.edges (
  edge_id     bigserial PRIMARY KEY,
  src_id      text NOT NULL REFERENCES graph.nodes(node_id),
  dst_id      text NOT NULL REFERENCES graph.nodes(node_id),
  edge_type   text NOT NULL, -- depends_on, produces, verifies, approves, acknowledges, supersedes, etc.
  created_at  timestamptz NOT NULL DEFAULT now(),
  meta        jsonb NOT NULL DEFAULT '{}'::jsonb
);
CREATE INDEX ON graph.edges (src_id, edge_type);
CREATE INDEX ON graph.edges (dst_id, edge_type);
```

#### 1.8.3 Recursive dependency query example (normative)

To compute transitive dependencies (depth-limited):

```sql
WITH RECURSIVE walk AS (
  SELECT src_id, dst_id, edge_type, 1 AS depth
  FROM graph.edges
  WHERE src_id = $1

  UNION ALL

  SELECT e.src_id, e.dst_id, e.edge_type, w.depth + 1
  FROM graph.edges e
  JOIN walk w ON e.src_id = w.dst_id
  WHERE w.depth < $2
)
SELECT * FROM walk;
```

### 1.9 Evidence bundle model

#### 1.9.1 Evidence bundle manifest (v1)

The canonical evidence artifact type is `domain.evidence_bundle`. Its manifest MUST include at minimum:

```jsonc
{
  "artifact_type": "domain.evidence_bundle",
  "artifact_version": "v1",
  "candidate_id": "git:abcd...|sha256:...",
  "run_id": "run_01J...",
  "oracle_suite_id": "suite:SR-SUITE-CORE",
  "oracle_suite_hash": "sha256:...",
  // Semantic Ralph Loops: bind evidence to the procedure context that was evaluated.
  "procedure_template_id": "proc:...",
  "stage_id": "stage:...",
  "results": [
    {
      "oracle_id": "oracle:unit_tests",
      "oracle_name": "Unit Tests",
      "classification": "required",
      "status": "PASS",
      "started_at": "2026-01-09T12:00:00Z",
      "finished_at": "2026-01-09T12:01:10Z",
      "artifacts": [
        {"path": "logs/unit_tests.log", "media_type": "text/plain", "sha256": "..."},
        {"path": "reports/unit_tests.json", "media_type": "application/json", "sha256": "..."}
      ]
    },
    {
      // Example of a semantic oracle result: structured measurements + derived PASS/FAIL.
      "oracle_id": "oracle:semantic_stage_eval",
      "oracle_name": "Semantic Stage Evaluation",
      "classification": "required",
      "status": "PASS",
      "measurements": {
        "residual_vector_ref": {"path": "reports/semantic/residual.json", "sha256": "..."},
        "coverage_ref": {"path": "reports/semantic/coverage.json", "sha256": "..."},
        "violations_ref": {"path": "reports/semantic/violations.json", "sha256": "..."}
      }
    }
  ],
  "context": {
    "environment_fingerprint": {
      "os": "...",
      "container_image": "...",
      "tool_versions": {"sr-oracles": "...", "sr-runner": "..."}
    },
    "governed_refs": [
      {"id": "SR-CONTRACT", "version": "", "content_hash": "sha256:..."},
      {"id": "SR-SPEC", "version": "", "content_hash": "sha256:..."},
      {"id": "SR-DIRECTIVE", "version": "", "content_hash": "sha256:..."}
    ],
    "loop_id": "loop_01J...",
    "iteration_id": "iter_01J..."
  },
  "produced_at": "2026-01-09T12:35:10Z"
}
```

The manifest MAY reference separate content-addressed blobs (logs, reports). The manifest itself is the minimum structured record.

#### 1.9.2 Evidence immutability (MinIO)

Evidence Bundles MUST be stored in content-addressed storage:

- Object key MUST be derived from the content hash (e.g., `sha256/<hash>`).
- Storage MUST prevent overwriting an existing object at the same key.
- The evidence store SHOULD enable object lock / retention where available.

#### 1.9.3 Evidence containing secrets

If evidence contains secrets:

- The **original** evidence MUST be stored in a restricted store keyed by `original_content_hash` (H1).
- A **redacted** copy MAY be stored in the general evidence store keyed by `redacted_content_hash` (H2).
- A `redaction_manifest` MUST list redacted regions/fields.

This spec implements the “restricted vault” as:

- a dedicated MinIO bucket `evidence-restricted` with stricter IAM, and
- envelope encryption where the data key is managed in Infisical.

---


#### 1.9.4 Evidence availability and retrievability (C-EVID-6)

Evidence referenced by binding claims MUST remain retrievable for as long as those claims are active.

**Normative requirements:**

1. **Retrievability check:** before the system MAY (a) mark a Candidate Verified, (b) accept/record a Portal Approval that references evidence, (c) finalize a Freeze Record, or (d) compute Shippable, it MUST be able to retrieve each referenced Evidence Bundle and each referenced content-addressed blob by digest.

2. **`EVIDENCE_MISSING` integrity condition:** if any referenced evidence cannot be retrieved, the system MUST:
   - record the `EVIDENCE_MISSING` integrity condition,
   - emit `EvidenceMissingDetected` (Appendix A),
   - block progression for the affected claim(s) until resolved,
   - route escalation to an appropriate Portal (typically Governance Change and/or Release Approval).

3. **Resolution paths:** implementations MUST provide at least one of:
   - re-run verification to produce new evidence, and/or
   - record a binding correction/retraction that invalidates or supersedes the affected claim.

4. **Non-waivable:** `EVIDENCE_MISSING` MUST NOT be bypassed via Gate Waiver (see §1.14).

Implementation note: storage backends SHOULD enforce retention/immutability controls (e.g., object lock) for referenced evidence.


### 1.10 Governed artifact metadata and registry (C-META-1..C-META-3)

SOLVER-Ralph MUST maintain machine-indexable metadata for each **governed artifact version** in scope, sufficient to:

- identify what the artifact is (type, authority class, normative status),
- determine which version is “current” for default selection,
- reconstruct which versions were in force for any Freeze / baseline,
- drive dependency and staleness analysis.

#### 1.10.1 Metadata carrier and minimum fields

Governance-relevant Markdown artifacts MUST begin with YAML frontmatter conforming to `solver-ralph.artifact-metadata/v1`.

At minimum, the frontmatter MUST include:

- `solver_ralph.schema`
- `solver_ralph.id`
- `solver_ralph.type`
- `solver_ralph.title`
- `solver_ralph.version`
- `solver_ralph.status`
- `solver_ralph.normative_status`
- `solver_ralph.authority_kind`
- `solver_ralph.governed_by`

Type-specific extension fields MUST appear only under:

- `solver_ralph.ext.<type_key>`

The system MUST reject (or quarantine as **non-governed**) any artifact that:

- lacks YAML frontmatter,
- omits required fields,
- places type-specific fields outside `solver_ralph.ext.*`.

#### 1.10.2 Stable identity, versioning, and “current” selection

- `solver_ralph.id` is the stable artifact identity (e.g., `SR-CONTRACT`, `SR-SPEC`).
- `solver_ralph.version` MUST be a SemVer-compatible string for versioned governance artifacts.
- For each stable `artifact_id`, the registry MUST enforce **at most one** `is_current = true`.

**Important:** `is_current` is a *selection pointer* and MUST NOT be treated as equivalent to “in a Freeze” or “approved for release.” The Freeze Record is the binding baseline snapshot.

**Governance constraint (trust boundary):** For any artifact where `normative_status = normative`, changing which version is `is_current = true` MUST be treated as a governance change and MUST be performed by a HUMAN actor and linked to an Approval at the Governance Change Portal (portal identifier defined in the Development Directive).

#### 1.10.3 Registry projection table (normative)

The system MUST maintain a projection for governed artifacts:

```sql
CREATE TABLE proj.governed_artifacts (
  artifact_id        text NOT NULL,
  artifact_type      text NOT NULL,
  version            text NOT NULL,
  content_hash       text NOT NULL,   -- sha256:<hex>
  status             text NOT NULL,   -- draft|governed|superseded|deprecated|archived
  normative_status   text NOT NULL,   -- normative|directional|index|record|evidence
  authority_kind     text NOT NULL,   -- content|process|record|config|index
  governed_by        text[] NOT NULL, -- e.g., {SR-CHANGE}
  tags               text[] NOT NULL DEFAULT '{}',
  supersedes         text[] NOT NULL DEFAULT '{}', -- prior versions (optional)
  is_current         boolean NOT NULL DEFAULT false,
  recorded_at        timestamptz NOT NULL,
  recorded_by_kind   text NOT NULL,   -- HUMAN|SYSTEM
  recorded_by_id     text NOT NULL,   -- per §1.4
  last_event_id      text NOT NULL,
  last_global_seq    bigint NOT NULL,
  PRIMARY KEY (artifact_id, version)
);

CREATE UNIQUE INDEX uniq_governed_artifacts_current
  ON proj.governed_artifacts (artifact_id)
  WHERE is_current = true;
```

#### 1.10.4 Registry event (normative)

The system MUST emit a `GovernedArtifactVersionRecorded` event whenever it ingests (or updates the “current” pointer for) a governed artifact version.

**Event type:** `GovernedArtifactVersionRecorded`  
**Stream kind:** `GOVERNANCE`  
**Stream id:** `gov:artifacts`

**Payload (v1):**

```jsonc
{
  "artifact_id": "SR-CONTRACT",
  "artifact_type": "governance.arch_contract",
  "content_hash": "sha256:<hex>",
  "status": "draft",
  "normative_status": "normative",
  "authority_kind": "content",
  "governed_by": ["SR-CHANGE"],
  "tags": ["solver-ralph", "contract"],
  "supersedes": ["SR-CONTRACT"],
  "is_current": true
}
```

**Reference requirements:**

- The event MUST include `refs[]` entries of `kind=GovernedArtifact` for the artifact it is about (`rel=about`).
icable.
- If `is_current=true` and `normative_status=normative`, the event MUST include an `Approval` reference (`rel=approved_by`) to the Governance Change Portal approval record.

---

### 1.11 Decision record model (C-DEC-1)

A **Decision** is a binding human judgment that resolves ambiguity/tradeoffs or authorizes continuation after a stop-the-line trigger.

#### 1.11.1 Decision record minimum fields (normative)

A Decision record MUST include, at minimum:

- `decision_id` (`dec_<ULID>`)
- `trigger` (what caused the decision; e.g., stop trigger code or portal escalation)
- `scope` (what the decision applies to; MUST be specific and bounded)
- `decision` (the resolved choice / authorization)
- `rationale` (human-readable, auditable)
- `decided_by` (stable actor identity per §1.4; MUST be HUMAN)
- `decided_at` (ISO 8601 / timestamptz)
- `subject_refs[]` (typed references to affected entities)
- `evidence_refs[]` (evidence bundles reviewed; MAY be empty)
- `exceptions_acknowledged[]` (active exceptions acknowledged; MUST be explicit even if empty)

#### 1.11.2 Decision event (normative)

**Event type:** `DecisionRecorded`  
**Stream kind:** `DECISION`  
**Stream id:** `dec:{decision_id}`

**Actor constraint:** `actor_kind` MUST be `HUMAN`.

**Payload (v1):**

```jsonc
{
  "decision_id": "dec_01J...ULID",
  "trigger": "STOP_TRIGGER:ORACLE_FLAKE",
  "scope": {
    "loop_id": "loop_01J...ULID",
    "iteration_id": "iter_01J...ULID",
    "candidate_id": "git:<sha>|sha256:<hex>|cand_<ULID>"
  },
  "decision": "Proceed with re-run using repaired oracle suite; do not accept current evidence.",
  "rationale": "Flake indicates non-determinism; integrity condition must be resolved before verification.",
  "evidence_refs": ["sha256:<hex>"],
  "exceptions_acknowledged": [],
  "decided_by": { "actor_kind": "HUMAN", "actor_id": "oidc_sub:<issuer_hash>:<subject>" },
  "decided_at": "2026-01-09T12:34:56Z",
  "is_precedent": false
}
```

**Reference requirements (normative):**

- `refs[]` MUST include at least one `Loop` or `Iteration` reference (`rel=in_scope_of`) when the decision is loop/iteration-scoped.
- `refs[]` MUST include `rel=affects` references for any Candidate, Run, OracleSuite, or Exception materially affected by the decision.

#### 1.11.3 Decisions projection table (normative)

```sql
CREATE TABLE proj.decisions (
  decision_id           text PRIMARY KEY, -- dec_<ULID>
  trigger               text NOT NULL,
  scope                 jsonb NOT NULL,
  decision              text NOT NULL,
  rationale             text NOT NULL,
  is_precedent          boolean NOT NULL DEFAULT false,
  applicability         text,
  evidence_refs         text[] NOT NULL DEFAULT '{}',     -- sha256:...
  exceptions_acknowledged jsonb NOT NULL DEFAULT '[]'::jsonb,
  decided_by_kind       text NOT NULL, -- HUMAN
  decided_by_id         text NOT NULL,
  decided_at            timestamptz NOT NULL,
  last_event_id         text NOT NULL,
  last_global_seq       bigint NOT NULL
);
```

---


#### 1.11.4 Human judgment records projection table (normative)

This table supports auditable, explicitly **non-binding** human interpretation artifacts (Evaluation and Assessment notes) without allowing authority leakage into binding state.

```sql
CREATE TABLE proj.human_judgment_records (
  record_id           text PRIMARY KEY,            -- rec_...
  record_type         text NOT NULL,               -- record.evaluation_note | record.assessment_note
  subject_refs        jsonb NOT NULL,              -- typed refs to subjects evaluated/assessed
  evidence_refs       text[] NOT NULL,             -- evidence bundle digests reviewed
  content             text NOT NULL,
  severity            text,                        -- for evaluation notes
  fitness_judgment    text,                        -- for assessment notes
  recommendations     text,
  is_binding          boolean NOT NULL DEFAULT false, -- MUST be false for eval/assessment
  recorded_by_kind    text NOT NULL,               -- HUMAN
  recorded_by_id      text NOT NULL,
  recorded_at         timestamptz NOT NULL,
  last_event_id       text NOT NULL,
  last_global_seq     bigint NOT NULL
);
```

**Constraint:** `is_binding` MUST remain `false` for `record.evaluation_note` and `record.assessment_note`.


### 1.12 Freeze record and Shippable determination (C-SHIP-1)

A **Freeze Record** is the binding baseline snapshot required for declaring a Candidate **Shippable**.

#### 1.12.1 Freeze record minimum fields (normative)

A Freeze Record MUST enumerate, at minimum:

- `freeze_id` (`freeze_<ULID>`)
- `baseline_id` (human meaningful baseline name, e.g., `baseline:2026-01-09.release-1`)
- `candidate_id` (the Candidate identity being released)
- `verification` (mode, suite id/hash, evidence bundle refs, and any waiver refs if With-Exceptions)
- `release_approval_id` (Approval record at the Release Approval Portal)
- `artifact_manifest[]` (governed artifacts in force: `{ artifact_id, version, content_hash }`)
- `active_exceptions[]` (Deviations, Deferrals, Waivers applicable to the baseline/release)
- `frozen_by` (stable human actor identity) and `frozen_at`

#### 1.12.2 Freeze record event (normative)

**Event type:** `FreezeRecordCreated`  
**Stream kind:** `FREEZE`  
**Stream id:** `freeze:{freeze_id}`

**Actor constraint:** `actor_kind` MUST be `HUMAN`.

**Payload (v1):**

```jsonc
{
  "freeze_id": "freeze_01J...ULID",
  "baseline_id": "baseline:2026-01-09.release-1",
  "candidate_id": "git:<sha>|sha256:<hex>|cand_<ULID>",
  "verification": {
    "mode": "STRICT|WITH_EXCEPTIONS",
    "oracle_suite_id": "suite:<id>|suite_<ULID>",
    "oracle_suite_hash": "sha256:<hex>",
    "evidence_bundle_refs": ["sha256:<hex>"],
    "waiver_refs": ["exc_01J...ULID"]
  },
  "release_approval_id": "appr_01J...ULID",
  "artifact_manifest": [
    { "artifact_id": "SR-CONTRACT", "version": "", "content_hash": "sha256:<hex>" },
    { "artifact_id": "SR-SPEC", "version": "1.0.0", "content_hash": "sha256:<hex>" }
  ],
  "active_exceptions": [
    { "exception_id": "exc_01J...ULID", "kind": "WAIVER", "status": "ACTIVE" }
  ],
  "frozen_by": { "actor_kind": "HUMAN", "actor_id": "oidc_sub:<issuer_hash>:<subject>" },
  "frozen_at": "2026-01-09T12:45:00Z"
}
```

**Reference requirements (normative):**

- `refs[]` MUST include:
  - the Candidate (`rel=releases`),
  - the Release Approval (`rel=approved_by`),
  - each Evidence Bundle (`rel=supported_by`),
  - each active exception (`rel=acknowledges`),
  - each governed artifact version in `artifact_manifest` (`rel=depends_on`).

#### 1.12.3 Release Approval exception-acknowledgement constraint (normative)

The system MUST reject `FreezeRecordCreated` if the referenced Release Approval record does not explicitly acknowledge the **active exceptions** applicable to the release.

Concretely:

- `ApprovalRecorded.exceptions_acknowledged[]` MUST be present and explicit (including an explicit empty list when no exceptions apply).
- If `FreezeRecordCreated.active_exceptions[]` is non-empty, then each such exception MUST appear in the Approval’s `exceptions_acknowledged[]`.

#### 1.12.4 Shippable determination rule (normative)

A Candidate is **Shippable** if and only if:

1) Candidate is `Verified (Strict)` or `Verified-with-Exceptions` (per §3.3 and Contract C-VER-*), AND  
2) A Release Approval exists (ApprovalRecorded; human), AND  
3) A Freeze Record exists and is complete (this section), AND  
4) There are **no unresolved staleness markers** (see §1.13) affecting any of:
   - the Candidate,
   - the oracle suite used for verification,
   - any governed artifact listed in the Freeze Record’s `artifact_manifest`.

#### 1.12.5 Shippable projection table (normative)

```sql
CREATE TABLE proj.shippable_status (
  candidate_id         text PRIMARY KEY,
  is_verified          boolean NOT NULL,
  verification_mode    text,
  latest_evidence_hash text,
  release_approval_id  text,
  freeze_id            text,
  has_unresolved_staleness boolean NOT NULL DEFAULT false,
  computed_at          timestamptz NOT NULL,
  last_event_id        text NOT NULL,
  last_global_seq      bigint NOT NULL
);
```

---

### 1.13 Staleness marking and re-evaluation routing (C-EVT-6)

A **staleness marker** is a first-class signal that downstream verification, approval, or freeze artifacts may no longer be trustworthy because an upstream dependency changed.

#### 1.13.1 Staleness marker semantics (normative)

- Staleness MUST be recorded as events (append-only).
- A staleness marker MUST be attributable (`actor_kind`, `actor_id`) and time-stamped.
- Staleness markers MUST be queryable by:
  - root cause node,
  - impacted dependent nodes,
  - resolution state.

#### 1.13.2 Staleness projection table (normative)

```sql
CREATE TABLE graph.stale_nodes (
  stale_id             text PRIMARY KEY, -- stale_<ULID>
  root_kind            text NOT NULL,
  root_id              text NOT NULL,
  dependent_kind       text NOT NULL,
  dependent_id         text NOT NULL,
  reason_code          text NOT NULL,      -- e.g., GOVERNED_ARTIFACT_CHANGED
  reason_detail        text,
  marked_at            timestamptz NOT NULL,
  marked_by_kind       text NOT NULL,
  marked_by_id         text NOT NULL,
  resolved_at          timestamptz,
  resolution_event_id  text
);

CREATE INDEX idx_stale_root ON graph.stale_nodes(root_kind, root_id) WHERE resolved_at IS NULL;
CREATE INDEX idx_stale_dependent ON graph.stale_nodes(dependent_kind, dependent_id) WHERE resolved_at IS NULL;
```

#### 1.13.3 Staleness events (normative)

##### NodeMarkedStale

**Event type:** `NodeMarkedStale`  
**Stream kind:** `GOVERNANCE`  
**Stream id:** `gov:staleness`

**Payload (v1):**

```jsonc
{
  "stale_id": "stale_01J...ULID",
  "root": { "kind": "GovernedArtifact", "id": "SR-CONTRACT" },
  "reason_code": "GOVERNED_ARTIFACT_CHANGED",
  "reason_detail": "SR-CONTRACT version changed from  to ",
  "dependents": [
    { "kind": "Candidate", "id": "git:<sha>|sha256:<hex>|cand_<ULID>" },
    { "kind": "Freeze", "id": "freeze_01J...ULID" }
  ]
}
```

**Reference requirements:**

- `refs[]` MUST include:
  - `root` (`rel=root_cause`)
  - each dependent (`rel=stale`)

##### ReEvaluationTriggered

**Event type:** `ReEvaluationTriggered`  
**Stream kind:** `GOVERNANCE`  
**Stream id:** `gov:staleness`

**Payload (v1):**

```jsonc
{
  "stale_id": "stale_01J...ULID",
  "triggered_for": { "kind": "Candidate", "id": "git:<sha>|sha256:<hex>|cand_<ULID>" },
  "action": "START_LOOP|REQUEST_PORTAL_REVIEW",
  "triggered_loop_id": "loop_01J...ULID",
  "triggered_portal_id": "ReleaseApprovalPortal",
  "reason": "Candidate depends on updated governed artifact SR-CONTRACT"
}
```


##### StalenessResolved

**Event type:** `StalenessResolved`

**Meaning:** Marks a staleness marker as resolved. Resolution does not retroactively change history; it records that the system (or a human decision) considers the prior staleness satisfied.

```jsonc
{
  "stale_id": "stale_01J...ULID",
  "resolution_kind": "MECHANICAL|DECISION",
  "resolution_note": "string (optional)",
  "resolution_refs": [
    { "kind": "Run", "id": "run_01J...ULID" },
    { "kind": "Decision", "id": "dec_01J...ULID" }
  ]
}
```

**Reference requirements:**
- `refs[]` MUST include:
  - `root` (`rel=root_cause`)
  - each dependent being resolved (`rel=stale`)
- If `resolution_kind=DECISION`, `refs[]` MUST include the binding Decision (or Approval) that authorizes the resolution (`rel=relates_to` or `rel=approved_by`, as appropriate).


#### 1.13.4 Staleness propagation rule (normative)

When a root node is changed (e.g., a new governed artifact version becomes current, an oracle suite is rebased, or a critical exception is activated), the system MUST:

1) compute impacted dependents by traversing the dependency graph, and
2) record staleness markers for each impacted dependent.

The dependency traversal MUST be bounded (depth limit) and MUST be reproducible from the event store.

**Relation participation constraint (normative):**

- Only dependency edges created from `rel=depends_on` participate in staleness propagation (blocking) by default.
- Audit-only provenance edges created from `rel=supported_by` MUST NOT generate staleness markers by default.


**Normative traversal direction:**

- For `edge_type=depends_on`, edges MUST be oriented `dependent → dependency`.
- A dependent impact walk therefore traverses **reverse** edges (`dst_id = root_id`).

Example query for reverse traversal (depth-limited):

```sql
WITH RECURSIVE dependents AS (
  SELECT e.src_id AS dependent_id, e.dst_id AS dependency_id, e.edge_type, 1 AS depth
  FROM graph.edges e
  WHERE e.dst_id = $1

  UNION ALL

  SELECT e2.src_id, e2.dst_id, e2.edge_type, d.depth + 1
  FROM graph.edges e2
  JOIN dependents d ON e2.dst_id = d.dependent_id
  WHERE d.depth < $2
)
SELECT * FROM dependents;
```

#### 1.13.5 Staleness and Shippable gating (normative)

- The system MUST NOT declare a Candidate Shippable if there exists any unresolved staleness marker impacting that Candidate, its verification suite, or any governed artifact in its Freeze Record.
- When staleness is detected for a released baseline, the system SHOULD route to:
  - a re-verification Run (if purely oracle-resolvable), or
  - a Portal review (if acceptability/risk judgment is required).


### 1.14 Gate Waiver scope and constraints (normative)

Per SR-CONTRACT, Gate Waivers have constrained applicability.

#### 1.14.1 Waiver applicability

Gate Waivers apply **only** to explicit `FAIL` outcomes from required oracles. They MUST NOT be used to bypass:

- **Integrity conditions:** `ORACLE_TAMPER`, `ORACLE_GAP`, `ORACLE_ENV_MISMATCH`, `ORACLE_FLAKE`, `EVIDENCE_MISSING`
- **Missing evidence:** a waiver cannot substitute for absent evidence.
- **Missing approvals:** a waiver cannot substitute for Portal approval.

**Constraint:** the domain core MUST reject waiver creation or activation for any target that is not an explicit required-oracle `FAIL` status.

#### 1.14.2 Verified-with-Exceptions computation guardrails

When computing `Verified-with-Exceptions` status:

1. Each required oracle `FAIL` MUST have a corresponding active waiver.
2. Each waiver MUST reference the specific oracle and failure being waived.
3. The waiver scope MUST include the Candidate being verified.
4. The system MUST reject any claim where a waiver references an integrity condition rather than an explicit `FAIL`.


## 2. API Specification (HTTP v1)

### 2.1 Authentication

- The API MUST validate OIDC JWTs issued by Zitadel.
- The API MUST derive `actor_kind` and `actor_id` from the authenticated principal:
  - Human user token → `actor_kind=HUMAN`, `actor_id=oidc_sub:<issuer_hash>:<sub>`
  - Service token → `actor_kind=SYSTEM`, `actor_id=svc:<service>:<key_id>`
  - Agent token → `actor_kind=AGENT`, `actor_id=agent:<deployment>:<instance>`

### 2.2 Authorization (trust boundary enforcement)

Endpoints that create binding governance artifacts MUST require `actor_kind=HUMAN` and appropriate role claims:

- Create/approve/reject Approvals
- Create/activate/resolve Deviations/Deferrals/Waivers
- Register or modify an oracle suite used for governed verification
- Finalize Freeze Records / baselines

Additionally, creation of human judgment records MUST require `actor_kind=HUMAN` (authenticated human identity):

- `record.evaluation_note` (human evaluation of verification evidence)
- `record.assessment_note` (human assessment of validation evidence)


Iteration creation/start is a control-plane action and MUST be SYSTEM-mediated:

- `POST /loops/{loop_id}/iterations` MUST be callable only by a SYSTEM service.
- The emitted `IterationStarted` event MUST have `actor_kind=SYSTEM`.
- Any UI or agent request to “start an iteration” MUST be mediated by a SYSTEM service that emits `IterationStarted` as `actor_kind=SYSTEM`. The initiator MAY be captured for audit via typed references, but MUST NOT change the event’s actor.

The API MUST record authorization outcomes as events (success path) and MUST NOT “simulate” approvals.


#### 2.2.1 Evaluation and Assessment are non-binding (C-TB-7)

The system MUST ensure:

1. **State transition constraint:** only Portal Approvals and binding Decisions MAY create or change binding authority state. The domain core MUST reject any state transition that would advance a Candidate to Approved or Shippable based solely on evaluation/assessment records.

2. **Record typing enforcement:** if evaluation or assessment notes are persisted:
   - they MUST be typed as `record.evaluation_note` or `record.assessment_note` (per SR-TYPES),
   - they MUST NOT be typed as Approval or Decision records,
   - they MUST NOT be accepted as evidence of Verified status.

3. **API enforcement:** endpoints that accept evaluation/assessment notes MUST NOT emit approval-equivalent events and MUST NOT modify verification/approval status projections.


### 2.3 Core endpoints

All endpoints are under `/api/v1`.

#### 2.3.1 Loops

- `POST /loops` → Create a loop  
  Body: `{ goal, work_unit, budgets, directive_ref }`  
  Emits: `LoopCreated`
  Notes: `work_unit` SHOULD be stable for the loop lifetime; it is used to select the default `config.gating_policy` unless explicitly referenced in `IterationStarted.refs[]`.

- `GET /loops/{loop_id}` → Query loop state (projection)

- `POST /loops/{loop_id}/iterations` → Start an iteration (**SYSTEM-only**)  
  Body: `{ refs: Ref[] }` (the Iteration Context Ref Set; see §3.2.1.1)  
  Emits: `IterationStarted` (actor_kind=SYSTEM)

- `POST /loops/{loop_id}/iterations/{iteration_id}/complete` → Complete iteration  
  Body includes the **iteration summary** artifact (structured; see §3.2.2)  
  Emits: `IterationCompleted`, `IterationSummaryRecorded`, plus any stop-trigger events.

#### 2.3.2 Candidates

- `POST /candidates` → Register a candidate snapshot  
  Body: `{ candidate_id, repo_ref, content_hashes[], produced_by_iteration_id }`  
  Emits: `CandidateMaterialized`

  **Candidate identity policy (normative):**
  - `candidate_id` SHOULD embed both a VCS pointer and a cryptographic digest (e.g., `git:<sha>|sha256:<manifest_hash>|cand_<ulid>`).
  - The system SHOULD verify the declared digest against a canonical computation before treating the identity as authoritative (see §1.3.3).

- `GET /candidates/{candidate_id}` → Query candidate status

#### 2.3.3 Runs and evidence

- `POST /runs` → Start a run (oracle suite against candidate)  
  Body: `{ candidate_id, oracle_suite_id }`  
  Emits: `RunStarted`, then `RunCompleted` + `EvidenceBundleRecorded` on completion.

- `GET /runs/{run_id}` → Query run state

- `GET /evidence/{content_hash}` → Get evidence manifest (redirect to MinIO presigned URL)

- `GET /evidence/{content_hash}/status` → Check evidence availability
  Returns: `{ content_hash, available: boolean, last_verified_at?, error_code? }`
  If `available=false`, `error_code` SHOULD be `EVIDENCE_MISSING`.

#### 2.3.4 Approvals (Portals)

- `POST /approvals` → Create a Portal approval record (human only)  
  Body: `{ portal_id, decision, subject_refs[], evidence_refs[], exceptions_acknowledged[], attestations? }`  
  Emits: `ApprovalRecorded`

  **Attestations (recommended; informational only):**
  ```jsonc
  {
    "attestations": {
      "verification_reviewed": true,
      "validation_assessed": true,
      "exceptions_understood": true,
      "assessment_note_refs": ["rec_..."] // optional refs to non-binding assessment notes
    }
  }
  ```

  **Constraint:** `attestations` are auditable metadata and MUST NOT replace the binding `decision` field.

- `GET /approvals/{approval_id}` → Query approval

#### 2.3.5 Exceptions

- `POST /exceptions/waivers` → Create waiver (human only)  
  Body MUST include required waiver fields + scope constraints  
  Emits: `WaiverCreated` (and `WaiverActivated` if immediately active)

- `POST /exceptions/{exception_id}/resolve` → Resolve exception (human only)  
  Emits: `ExceptionResolved`


#### 2.3.6 Governed artifacts

- `POST /governed-artifacts/register` → Register a governed artifact version (registry ingestion)  
  Body: `{ artifact_id, artifact_type, version, content_hash, status, normative_status, authority_kind, governed_by[], tags[], supersedes[], is_current }`  
  Emits: `GovernedArtifactVersionRecorded`

**Constraints:**
- Registry ingestion MAY be performed by `actor_kind=SYSTEM` when `is_current=false`.
- Setting `is_current=true` for a `normative_status=normative` governed artifact MUST be performed by `actor_kind=HUMAN` and MUST include a typed reference to the Governance Change Portal Approval (`rel=approved_by`).

- `GET /governed-artifacts/{artifact_id}` → List known versions + current pointer

#### 2.3.7 Decisions

- `POST /decisions` → Record a binding Decision (human only)  
  Body: `{ trigger, scope, decision, rationale, subject_refs[], evidence_refs[], exceptions_acknowledged[], is_precedent?, applicability? }`  
  Emits: `DecisionRecorded`

- `GET /decisions/{decision_id}` → Query decision

#### 2.3.8 Freeze records and Shippable

- `POST /freeze-records` → Create a Freeze Record for a Candidate (human only)  
  Body: `{ baseline_id, candidate_id, verification, release_approval_id, artifact_manifest[], active_exceptions[] }`  
  Emits: `FreezeRecordCreated`

- `GET /freeze-records/{freeze_id}` → Query freeze record

- `GET /candidates/{candidate_id}/shippable` → Compute shippable status (projection)  
  Returns: `{ candidate_id, is_shippable, reasons[], freeze_id?, release_approval_id?, has_unresolved_staleness }`

#### 2.3.9 Staleness

- `POST /staleness/mark` → Mark a root node stale and fan-out to dependents  
  Body: `{ root_ref, reason_code, reason_detail, max_depth }`  
  Emits: `NodeMarkedStale` (and optionally `ReEvaluationTriggered`)

- `GET /staleness/dependents?root_kind=...&root_id=...` → List unresolved stale dependents


- `POST /staleness/{stale_id}/resolve` → Resolve a staleness marker  
  Body: `{ resolution_kind, resolution_note?, resolution_refs[]? }`  
  Emits: `StalenessResolved`  
  **Authorization:** `actor_kind=SYSTEM` is allowed for purely mechanical resolution; `actor_kind=HUMAN` + binding Decision/Approval reference is REQUIRED when resolution is discretionary.

#### 2.3.10 Human judgment records (non-binding)

- `POST /records/evaluation-notes` → Record human evaluation of verification evidence (human only)  
  Body: `{ subject_refs[], evidence_refs[], content, severity?, recommendations? }`  
  Emits: `RecordCreated` with `record_type=record.evaluation_note`  
  **Constraint:** MUST NOT emit `ApprovalRecorded` and MUST NOT modify Verified/Approved/Shippable status.

- `POST /records/assessment-notes` → Record human assessment of validation evidence (human only)  
  Body: `{ subject_refs[], evidence_refs[], content, fitness_judgment?, context? }`  
  Emits: `RecordCreated` with `record_type=record.assessment_note`  
  **Constraint:** MUST NOT emit `ApprovalRecorded` and MUST NOT modify Verified/Approved/Shippable status.

- `GET /records/{record_id}` → Query record


### 2.4 API response conventions

- All write endpoints return `{ accepted: true, correlation_id, emitted_event_ids[] }`.
- Read endpoints return projection views that include:
  - current computed state,
  - the latest relevant `event_id` + `global_seq`,
  - typed references for navigation.

---

## 3. State Machines

### 3.1 Ralph Loop lifecycle

#### 3.1.1 States

- `CREATED`
- `ACTIVE`
- `PAUSED` (stop-the-line)
- `CLOSED`

#### 3.1.2 Transitions (normative)

- `LoopCreated` → `CREATED`
- `LoopActivated` → `ACTIVE`
- `StopTriggered` → `PAUSED`
- `DecisionRecorded` + `LoopResumed` → `ACTIVE`
- `LoopClosed` → `CLOSED`

A Loop MUST NOT transition from `PAUSED` to `ACTIVE` without a recorded human **Decision** when the pause was caused by a stop-the-line trigger that requires arbitration.

### 3.2 Iteration lifecycle and memory discipline

#### 3.2.1 Fresh-context execution (iteration memory)

Each Iteration MUST run with a freshly constructed context derived from governed, typed inputs. By contract, iteration memory is **controlled**: summaries and typed references are the default.

The system MUST NOT use raw, unbounded conversation history as iteration memory.

##### 3.2.1.1 Iteration Context Ref Set (normative)

**Authoritative provenance rule:** The authoritative set of inputs for an Iteration MUST be recorded as typed references on the `IterationStarted` event:

- `IterationStarted` MUST be emitted with `actor_kind=SYSTEM`.
- `IterationStarted.refs[]` is the authoritative **Iteration Context Ref Set**.
- **No ghost inputs:** the agent’s context semantics MUST be derivable solely from the `IterationStarted` payload + dereferenced `IterationStarted.refs[]`.

**Minimum required ref categories (canonical checklist):** Every `IterationStarted.refs[]` MUST include, at minimum:

1) **Loop**  
   - `kind=Loop`, `rel=in_scope_of`, `id=<loop_id>`

2) **Governing artifacts in force** (binding semantics)  
   - `kind=GovernedArtifact`, `rel=depends_on`  
   - MUST include references to: `SR-TYPES`, `SR-CONTRACT`, `SR-SPEC`, `SR-DIRECTIVE`  
   - each MUST carry `meta.version` and `meta.content_hash` (and SHOULD carry `meta.selector` where a slice is intended)

3) **Prior Iteration summaries carried forward** (controlled memory)  
   - `kind=Iteration`, `rel=depends_on`  
   - MAY be empty for the first iteration of a loop

4) **Base Candidate** (when iterating on an existing snapshot)  
   - `kind=Candidate`, `rel=depends_on`  
   - OPTIONAL (but REQUIRED when the work is explicitly incremental on a prior Candidate)

5) **Oracle suite intended for verification**  
   - `kind=OracleSuite`, `rel=depends_on`

6) **Active exceptions in scope** (binding modifiers)  
   - `kind=Deviation|Deferral|Waiver`, `rel=depends_on`  
   - MAY be empty if no active exceptions apply (absence means “no exceptions referenced”)

7) **Human intervention notes (non-binding, semantic input)**
   - `kind=Record`, `meta.type_key=record.intervention_note`, `rel=depends_on`
   - MUST carry `meta.content_hash` (and SHOULD carry `meta.selector` where a slice is intended)
   - NOTE: this does NOT create Portal approval semantics; it is an input to the next iteration’s context.

   **Human judgment notes (non-binding, semantic input when present/required):**
   - `kind=Record`, `meta.type_key=record.evaluation_note`, `rel=depends_on` (**when required by gating policy**)
   - `kind=Record`, `meta.type_key=record.assessment_note`, `rel=depends_on` (**when required by gating policy**)
   - These records are auditable but MUST NOT be carried forward as iteration memory by default; they are referenced only when needed (typically the next iteration).


8) **Agent definition / worker configuration** (audit-only by default)  
   - `kind=GovernedArtifact` (typically `type=config.agent_definition` per SR-TYPES), `rel=supported_by`  
   - MUST carry `meta.version` and `meta.content_hash`

9) **Gating policy in force** (audit-only by default)
   - `kind=GovernedArtifact` (typically `type=config.gating_policy` per SR-TYPES), `rel=supported_by`
   - MUST carry `meta.version` and `meta.content_hash`

**Meta requirements:** For dereferenceable references in this set (especially `GovernedArtifact`, `Candidate`, `OracleSuite`, `EvidenceBundle`), `meta.content_hash` is REQUIRED (see §1.5.3.1). Use `meta.selector` to make “what was actually used” replayable without embedding entire documents in events.

##### 3.2.1.2 Context semantics and ContextCompiler (normative)

A **ContextCompiler** is the deterministic function that maps:

- inputs: the `IterationStarted` event payload + dereferenced `IterationStarted.refs[]`  
- output: a `ContextBundle` (the semantic context used by the Agent Worker for that iteration)

**Determinism constraint:** The ContextCompiler MUST be deterministic and replayable from the event store.

**Default execution location:** By default, the **Agent Worker** compiles the `ContextBundle` deterministically from `IterationStarted` + dereferenced refs. The SYSTEM MAY also compile and deliver a bundle, but the semantics MUST remain definable and replayable from the refs + payload.

**Precedence ordering (normative):** When multiple referenced artifacts constrain meaning, the compiled context MUST respect the binding precedence defined in SR-TYPES (e.g., Architectural Contract > Technical Spec > Development Directive > Design Intent > README). If conflicts exist among binding artifacts, the worker MUST stop and escalate rather than silently choosing.

**Hard no-ghost-inputs enforcement (normative):** During an Iteration, Agent Workers MUST NOT treat any content outside `IterationStarted` + dereferenced `IterationStarted.refs[]` as a semantic input. If additional external information is required, it MUST be ingested as a typed artifact/evidence and referenced in the *next* Iteration’s `IterationStarted.refs[]`.

**Transcript prohibition (normative):** Raw chat transcripts MUST NOT be included as memory by default. Controlled memory MUST flow through typed Iteration summaries and referenced artifacts/evidence only.


##### 3.2.1.3 Human Judgment Hooks (gating policy) (normative)

SOLVER-Ralph permits non-linear agent workflow while enforcing **deterministic hooks** that increase trust. Hooks are configured per **work unit** via `config.gating_policy` and enforced by SYSTEM at iteration start.

**Hook classes (canonical):**
- `plan_review` — non-binding plan review discipline (NOT a Portal approval)
- `evaluation_on_verification` — human evaluation of verification evidence (e.g., oracle results)
- `assessment_on_validation` — human assessment of validation evidence in context
- `closeout` — final closeout via existing Portal approvals (e.g., ReleaseApprovalPortal)

**Gating modes (canonical):**
- **Soft:** Iterations MAY proceed without the required human judgment record, but SYSTEM MUST surface the deficit (e.g., via a pending-hook indicator and/or an open risk recorded in the Iteration summary) until satisfied.
- **Hard:** SYSTEM MUST NOT start a “progressing” iteration that passes the hook unless the required human judgment record is present and referenced in `IterationStarted.refs[]`.
- **Hybrid (recommended default):** Behaves as **Soft** by default and escalates to **Hard** when deterministic triggers fire (as configured in `config.gating_policy`).

**Deterministic trigger names (canonical examples):**
- `EXCEPTIONS_ACTIVE`
- `OPEN_RISK_HIGH`
- `REPEATED_FAILURE`
- `BUDGET_NEAR_EXHAUSTED`
- `GOVERNANCE_TOUCH`
- `CLOSEOUT_PENDING`

**Non-goals / prohibitions (normative):**
- Hooks MUST NOT introduce new Portals (especially no plan-approval portals).
- Hooks MUST NOT redefine Verified/Approved semantics.
- Hooks MUST NOT introduce new lifecycle states.

#### 3.2.2 Iteration summary schema (v1)

At iteration completion, the system MUST record a typed summary with at least:

- `iteration_id`, `loop_id`
- `intent` (what was attempted)
- `actions[]` (what was changed, **structured**)
- `artifacts_touched[]` (file paths or artifact refs)
- `candidates_produced[]` (candidate refs)
- `runs_executed[]` (run refs)
- `outcomes` (oracle results summary, **structured**)
- `next_steps[]` (**structured**, machine-schedulable)
- `open_risks[]` (**structured**)

##### Minimal shape constraints (normative)

The following constraints are intentionally minimal; they exist to prevent “summary drift” into free-text narrative.

- `actions[]` MUST be an array of objects. Each action MUST include:
  - `kind` (string; e.g., `"code_change"`, `"doc_change"`, `"config_change"`, `"analysis"`, `"run_oracles"`)
  - `summary` (string)
  - `artifacts` (array of strings or refs; MAY be empty)

- `outcomes` MUST be an object. At minimum it MUST include:
  - `oracle_results[]`: array of `{ run_id, oracle_suite_id, status, evidence_refs[] }`
  - `stop_triggers_fired[]`: array of stop-trigger ids (MAY be empty)

- `next_steps[]` MUST be an array of objects. Each next step MUST include:
  - `kind` (string; e.g., `"implement"`, `"verify"`, `"escalate"`, `"ingest_input"`)
  - `description` (string)
  - `blocking` (boolean)

- `open_risks[]` MUST be an array of objects. Each risk MUST include:
  - `severity` (string; e.g., `"low"|"medium"|"high"|"critical"`)
  - `description` (string)
  - `mitigation` (string; MAY be empty)

##### Optional extensions (non-binding)

The summary MAY include an `ext` object for additional, non-binding operational notes. In particular:

- `ext.non_binding_reviews[]` MAY record internal plan/semantic reviews as audit artifacts (NOT approvals).  
  These reviews MUST NOT be treated as portal approvals or as evidence of “Verified”.

The summary MAY reference supporting evidence bundles; it MUST NOT embed raw full chat transcripts.

### 3.3 Candidate verification status

Candidate status is computed from events and evidence according to verification rules:

- `Verified (Strict)` iff evidence exists and all required oracle results are PASS and no unresolved integrity conditions.
- `Verified-with-Exceptions` iff evidence exists, some required oracles FAIL, and each FAIL is covered by an active, human-approved Waiver scoped appropriately.
- Otherwise: `Unverified`.

A Candidate MUST NOT be marked Verified without an evidence-backed run.

### 3.3.1 Candidate release readiness (Shippable)

A Candidate’s **Shippable** readiness is a computed status derived from:

- Candidate verification status (Strict vs With-Exceptions),
- Release Approval (Portal approval),
- Freeze Record completeness,
- Staleness markers (C-EVT-6).

**Normative rule:** The system MUST compute `is_shippable=true` if and only if all conditions in §1.12.4 hold.

If `is_shippable=false`, the system MUST return one or more reasons, including at least one of:

- `NOT_VERIFIED`
- `MISSING_RELEASE_APPROVAL`
- `MISSING_FREEZE_RECORD`
- `FREEZE_INCOMPLETE`
- `UNRESOLVED_STALENESS`


### 3.4 Stop-the-line trigger evaluation

After each iteration and after each run, the loop governor MUST evaluate stop triggers. The minimum mandatory stop triggers are:

**Oracle integrity (stop-the-line, non-waivable):**
- `ORACLE_TAMPER`
- `ORACLE_GAP`
- `ORACLE_ENV_MISMATCH`
- `ORACLE_FLAKE`

**Evidence integrity (blocks progression, non-waivable):**
- `EVIDENCE_MISSING`

**Loop budget/progress:**
- `REPEATED_FAILURE` (N ≥ 3; N defined in the Development Directive)
- `BUDGET_EXHAUSTED`

When a stop trigger fires, the system MUST emit a `StopTriggered` event, transition the loop to `PAUSED`, and require an explicit Decision to proceed.

**`EVIDENCE_MISSING` handling:** Unlike oracle integrity conditions (which are detected during Runs), `EVIDENCE_MISSING` may be detected during verification computation, Shippable computation, or audit. When detected, the system MUST emit `EvidenceMissingDetected` and MUST block affected claims until resolved.

### 3.5 Oracle suite pinning and environment constraints

- At `RunStarted`, the system MUST pin `oracle_suite_id` + `oracle_suite_hash` used.
- Oracle suites used for governed verification MUST declare environment constraints sufficient for determinism of required oracles.
- If a run violates declared constraints, the system MUST record `ORACLE_ENV_MISMATCH` and MUST NOT treat the run as valid evidence for a Verified claim.


### 3.6 Candidate Release Readiness (Shippable)

This state machine is normative for release gating per C-SHIP-1.

    ┌───────────────┐
    │   Unverified  │
    └───────┬───────┘
            │ CandidateVerificationComputed (Verified)
            ▼
    ┌─────────────────┐
    │ Verified        │
    │ (Strict or      │
    │  WithExceptions)│
    └────────┬────────┘
             │
             ▼
    ┌───────────────┐
    │ AwaitApproval │  (Release Approval Portal)
    └──────┬────────┘
           │ ApprovalRecorded(approved)
           ▼
    ┌───────────────┐
    │   Approved    │
    └──────┬────────┘
           │ FreezeRecordCreated (FROZEN)
           ▼
    ┌───────────────┐
    │   Shippable   │
    └───────────────┘

---

## 4. Code Patterns

### 4.1 Hexagonal architecture boundaries

The codebase MUST enforce these module boundaries:

- **Domain Core (Rust):**
  - Defines commands, events, state machines, verification logic, stop triggers.
  - Defines ports (traits) for: EventStore, EvidenceStore, OracleRunner, MessageBus, IdentityProvider, Clock.
  - MUST NOT import DB clients, HTTP frameworks, container runtimes, or LLM SDKs.

- **Adapters (Rust / TypeScript where appropriate):**
  - Implement ports and translate between infrastructure and domain types.

### 4.2 Event sourcing patterns

- Command handlers are pure functions: `(state, command) -> Result<Vec<Event>, DomainError>`.
- State is derived only by folding events: `apply(state, event) -> state`.
- Invariants (e.g., “approval requires human”) are enforced in domain command validation and reinforced in API authorization.

### 4.3 Projection and rebuild tooling

The system MUST provide a rebuild procedure:

- `rebuild projections`:
  - truncates `proj.*` and `graph.*`,
  - replays events from `es.events` in `global_seq` order,
  - re-materializes projections and graph.

Rebuild MUST NOT require reading any non-event “source of truth”.

### 4.4 Evidence capture pattern

The OracleRunner adapter MUST:

1. Materialize a read-only workspace for the candidate.
2. Execute each oracle in the pinned environment (container digest + runtime constraints).
3. Capture stdout/stderr and structured outputs.
4. Compute content hashes for each output artifact.
5. Construct the evidence manifest (canonical JSON), compute its hash.
6. Store blobs and manifest via EvidenceStore (MinIO).
7. Emit `EvidenceBundleRecorded` referencing the manifest hash.

### 4.5 Oracle runtime pattern (Podman + gVisor)

Oracle execution MUST be sandboxed with:

- OCI container image pinned by digest for the oracle suite.
- `runsc` runtime (gVisor) as the OCI runtime.
- Network disabled by default for required oracles (unless suite constraints explicitly allow).
- Read-only mount of candidate workspace.
- Write-only scratch volume for outputs.

The runner MUST record an `environment_fingerprint` including:

- container image digest
- runtime name/version (runsc)
- OS/arch
- critical tool versions (as declared by suite)
- network mode

### 4.6 Message bus pattern (NATS)

- The outbox publisher MUST publish each committed event to NATS subjects:
  - `sr.events.<stream_kind>.<event_type>`
- Work distribution uses JetStream durable consumers (for long-running oracle jobs) when enabled.

The bus is an adapter. No domain invariant depends on NATS being present; the system MUST remain correct with event store + synchronous execution only.


### 4.7 Agent Worker integration (sandboxed worker services)

Agent orchestration MUST live outside the hexagonal **domain core**. The domain core exposes ports, state machines, and event emission rules; Agent Workers are replaceable adapters/services that consume events and call SR-SPEC APIs.

**Normative integration pattern (v1):**

- Agent Workers subscribe to `IterationStarted` events from the message bus (or poll the event store/projection).
- For each `IterationStarted`, the worker MUST:
  1) dereference `IterationStarted.refs[]`, and  
  2) compile the iteration `ContextBundle` deterministically (see §3.2.1.2).
- The worker performs work in a sandboxed environment and may:
  - register Candidate snapshots via `POST /candidates`,
  - request oracle runs via `POST /runs`,
  - append evidence bundles via the Run completion flow,
  - complete the iteration by submitting the structured IterationSummary via `POST /loops/{loop_id}/iterations/{iteration_id}/complete`.
- **Constraint:** Agent Workers MUST NOT emit `IterationStarted`. Only SYSTEM services may start iterations and emit `IterationStarted` events.

**Non-binding internal discipline:** Workers MAY record non-binding plan/semantic reviews in `IterationSummary.ext.non_binding_reviews[]`, but these reviews MUST NOT be treated as approvals or as evidence of “Verified”.


---

## 5. Configuration

### 5.1 Configuration artifacts (normative)

The implementation MUST maintain the following configuration sources as governed artifacts (versioned, hashable):

- `governance/oracle_suites/*.yaml` — oracle suite definitions (including environment constraints)
- `governance/directive/*.md` — Development Directive (budgets, stop trigger N, portal policies)
- `governance/specs/SR-SPEC.md` — this technical specification
- `governance/contracts/SR-CONTRACT.md` — architectural contract

### 5.2 Service configuration

The system MUST support self-hosted deployment with configuration for:

- PostgreSQL connection and role separation:
  - `sr_app` role: INSERT-only on `es.events` and `es.outbox`, SELECT on projections, NO UPDATE/DELETE on event tables.
- MinIO endpoints + bucket names:
  - `evidence-public` (general)
  - `evidence-restricted` (restricted)
- NATS endpoints + credentials (optional in Phase 0)
- Zitadel issuer + client configuration
- Infisical connection for secrets and envelope keys

### 5.3 Governance-grade immutability controls

To protect the integrity of governance artifacts and evidence:

- PostgreSQL MUST enforce append-only behavior for `es.events` via privileges and/or triggers.
- MinIO buckets used for evidence SHOULD have object lock / retention enabled.
- Deletion of evidence objects MUST be restricted to a dedicated human-approved administrative procedure and MUST emit a corresponding governance event if performed.

---

## 6. Appendices

### Appendix A: Canonical event types (non-exhaustive v1)

**Loop**
- `LoopCreated`
- `LoopActivated`
- `IterationStarted`
- `IterationCompleted`
- `IterationSummaryRecorded`
- `StopTriggered`
- `LoopPaused`
- `LoopResumed`
- `LoopClosed`

**Candidate**
- `CandidateMaterialized`
- `CandidateVerificationComputed`

**Run / Evidence**
- `RunStarted`
- `RunCompleted`
- `EvidenceBundleRecorded`

**Oracle suites**
- `OracleSuiteRegistered`
- `OracleSuiteUpdated`
- `OracleSuitePinned`
- `OracleSuiteRebased` (human decision)

**Governed artifacts**
- `GovernedArtifactVersionRecorded`

**Freeze / release**
- `FreezeRecordCreated`

**Staleness / re-evaluation**
- `NodeMarkedStale`
- `ReEvaluationTriggered`
- `StalenessResolved`

**Portals / approvals**
- `ApprovalRecorded`

**Evidence integrity**
- `EvidenceMissingDetected`

**Records (non-binding human judgment)**
- `RecordCreated`
- `RecordSuperseded`

**Exceptions**
- `DeviationCreated`
- `DeferralCreated`
- `WaiverCreated`
- `ExceptionActivated`
- `ExceptionResolved`
- `ExceptionExpired`

**Decisions**
- `DecisionRecorded`
### Appendix B: Graph edge types (v1)

- `about`
- `depends_on`
- `produces`
- `verifies`
- `approved_by`
- `acknowledges`
- `supersedes`
- `releases`
- `supported_by`
- `governed_by`
- `in_scope_of`
- `affects`
- `stale`
- `root_cause`
- `relates_to`

### Appendix C: Integrity conditions (minimum set)

**Oracle integrity conditions:**
- `ORACLE_TAMPER` — suite definition changed during Run
- `ORACLE_GAP` — required oracle result missing
- `ORACLE_ENV_MISMATCH` — Run violated declared environment constraints
- `ORACLE_FLAKE` — required oracle non-deterministic for identical inputs

**Evidence integrity conditions:**
- `EVIDENCE_MISSING` — referenced evidence cannot be retrieved

**Non-waivable:** all integrity conditions listed above are non-waivable per SR-CONTRACT. They require resolution (re-run, rebase, or governance change), not waiver.

(Additional conditions MAY be added by the Development Directive.)