---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-GUIDE"
  type: "governance.usage_guide"
  title: "SOLVER-Ralph Usage Guide"
  version: "3.0.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-GUIDE@2.0.0-draft.1"
    - "SR-PARADIGM@1.0.0-draft.10"
  created: "2026-01-10"
  updated: "2026-01-12"
  tags:
    - "solver-ralph"
    - "ralph-loop"
    - "usage-guide"
    - "agentic-infrastructure"
    - "hexagonal-architecture"
  ext:
    usage_guide:
      purpose: >
        Guide for using the SOLVER-Ralph platform to execute agentic work with
        controlled state transitions and human approval at critical junctures.
      core_abstraction: "ralph-loop"
      architectural_style: "hexagonal"
---

# SOLVER-Ralph Usage Guide — v3.0.0-draft.1

## Purpose

This document describes how to use SOLVER-Ralph as infrastructure for agentic work.

SOLVER-Ralph is a **software platform** built on hexagonal architecture. It provides:

- **Ralph-loops** — the core iteration mechanism for agent work
- **Portals** — human approval surfaces at critical junctures
- **Oracles** — deterministic verification mechanisms
- **State tracking** — controlled, auditable state transitions

SOLVER-Ralph does **not** prescribe a methodology for problem-solving or knowledge work. Users define their own workflows by configuring and sequencing ralph-loops. The platform provides the constraints and control surfaces; users provide the work definition and direction.

---

## 0. Version Changes

### 3.0.0-draft.1 (2026-01-12)

**Major scope change:** This version removes all process methodology content.

- **Removed**: SAPS, RPS, IPS, INPS concepts (methodology artifacts)
- **Removed**: C0 classification, S0-S14 authoring stages (workflow methodology)
- **Removed**: "Coherence audit as binding gate" (process checkpoint)
- **Removed**: Agent wave model (methodology framing)
- **Removed**: Boot alignment prerequisites as governance categories (methodology)
- **Removed**: Appendix A SAPS template (methodology artifact)
- **Refocused**: Document now describes ralph-loop infrastructure and usage
- **Changed type key**: `governance.usage_guide` (was `governance.authoring_guide`)
- **Changed normative status**: `directional` (was `normative`) — this is a usage guide, not binding process

**Rationale:** SOLVER-Ralph is software infrastructure, not a methodology. The previous document conflated platform specification with process prescription. Users develop their own agentic workflows on top of this infrastructure; the platform provides control and constraint, not workflow.

---

## 1. Architectural Context

### 1.1 Hexagonal Architecture

SOLVER-Ralph follows hexagonal (ports-and-adapters) architecture:

```
                    ┌─────────────────────────────────────┐
                    │                                     │
   Driving Ports    │         DOMAIN CORE                 │    Driven Ports
   (how you use it) │                                     │    (what it uses)
                    │    ┌─────────────────────┐          │
        ───────────►│    │                     │          │►───────────
   Instructions     │    │    Ralph-Loop       │          │   Persistence
                    │    │                     │          │
        ───────────►│    │  ┌─────────────┐    │          │►───────────
   Approvals        │    │  │ Work Unit   │    │          │   LLM Adapters
                    │    │  │ State       │    │          │
        ◄───────────│    │  └─────────────┘    │          │►───────────
   Portal Requests  │    │                     │          │   Oracle Runners
                    │    └─────────────────────┘          │
        ◄───────────│                                     │►───────────
   Evidence/Results │                                     │   Event Store
                    │                                     │
                    └─────────────────────────────────────┘
```

**Key implications:**

- The domain core (ralph-loop logic) is independent of adapters
- Adapters can be replaced without changing core behavior
- The governed set (SR-* documents) constrains the domain core
- Infrastructure choices (which LLM, which database) are adapter concerns

### 1.2 What SOLVER-Ralph Provides

| Component | What it does |
|-----------|--------------|
| **Ralph-loop** | Iterative execution of agent work within defined constraints |
| **Work unit tracking** | State machine for work products (candidate → verified → approved → complete) |
| **Portal surfaces** | Interrupt points for human approval/decision |
| **Oracle integration** | Deterministic verification producing evidence |
| **Evidence binding** | Linkage between work products and verification results |
| **State persistence** | Event-sourced, auditable state transitions |
| **Context compilation** | Deterministic assembly of inputs for each loop iteration |

### 1.3 What SOLVER-Ralph Does Not Provide

| Concern | Where it belongs |
|---------|------------------|
| Problem-solving methodology | User-defined; external to platform |
| Workflow sequencing | User-defined; configured via instructions |
| Decomposition strategy | User-defined; expressed as work unit configuration |
| Approval criteria | User-defined; expressed in portal configuration |
| Verification scope | User-defined; expressed in oracle configuration |

---

## 2. The Ralph-Loop

### 2.1 Core Concept

A ralph-loop is a **single iteration of agent work** within controlled boundaries.

Each loop:
1. Receives compiled context (instructions + relevant artifacts + state)
2. Produces work output (proposals, candidates, artifacts)
3. May trigger verification (oracle runs)
4. May request human input (portal invocation)
5. Records state changes (events)
6. Terminates (success, failure, or portal-blocked)

Loops are intentionally **short-lived and fresh-context**. Long-running agent sessions accumulate errors; ralph-loops reset context each iteration.

### 2.2 Loop Inputs

A ralph-loop receives:

| Input | Description |
|-------|-------------|
| **Instructions** | What this loop should accomplish |
| **Work unit reference** | The work unit this loop is advancing |
| **Context bundle** | Compiled from governed artifacts + prior state |
| **Constraints** | Budget limits, allowed tools, scope boundaries |
| **Evidence requirements** | What verification is expected |

Context compilation is deterministic: given the same refs and state, the same context is produced.

### 2.3 Loop Outputs

A ralph-loop produces:

| Output | Description |
|--------|-------------|
| **Work products** | Files, code, documents, artifacts |
| **Candidate** | Content-addressed snapshot of work products |
| **State change requests** | Proposed transitions (e.g., "mark as ready for verification") |
| **Portal requests** | Requests for human decision/approval |
| **Loop summary** | Structured record of what occurred |

### 2.4 Loop Termination

A ralph-loop terminates when:

| Condition | Result |
|-----------|--------|
| Work complete | Candidate produced; ready for verification |
| Budget exhausted | Loop stopped; partial progress recorded |
| Portal required | Loop blocked; awaiting human input |
| Error/exception | Loop failed; error recorded |
| Explicit stop | Human or system halted the loop |

### 2.5 Loop Constraints

The platform enforces constraints on loops:

| Constraint | Enforcement |
|------------|-------------|
| **Budget** | Iteration/token/time limits; loop terminates when exceeded |
| **Scope** | Work unit boundaries; loop cannot modify out-of-scope state |
| **Tools** | Allowed tool set; unauthorized tool calls rejected |
| **Authority** | Agents cannot claim Verified/Approved/Shippable (see SR-AGENTS) |
| **Evidence** | Claims require oracle evidence; narrative claims are proposals only |

---

## 3. Work Units

### 3.1 Work Unit as State Machine

A work unit progresses through states:

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  DRAFT   │────►│ CANDIDATE│────►│ VERIFIED │────►│ APPROVED │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
     │                │                │                │
     │                │                │                ▼
     │                │                │          ┌──────────┐
     └────────────────┴────────────────┴─────────►│ COMPLETE │
                      (any failure)               └──────────┘
```

| State | Meaning |
|-------|---------|
| **DRAFT** | Work in progress; not yet submitted |
| **CANDIDATE** | Content-addressed snapshot submitted for verification |
| **VERIFIED** | Oracle evidence confirms conformance |
| **APPROVED** | Human has approved at portal |
| **COMPLETE** | Work unit closed |

### 3.2 Work Unit Configuration

Users define work units with:

| Field | Description |
|-------|-------------|
| `id` | Stable identifier |
| `type_key` | Expected output type (from SR-TYPES) |
| `instructions` | What the work unit should produce |
| `constraints` | Scope, budget, tool restrictions |
| `evidence_requirements` | What oracles must pass |
| `portal_requirements` | What human approvals are needed |
| `depends_on` | Other work units that must complete first |

### 3.3 Work Unit Sequencing

SOLVER-Ralph tracks dependencies but does **not** prescribe sequencing strategy.

Users may:
- Run work units sequentially
- Run independent work units concurrently
- Define custom sequencing logic via instructions

The platform enforces:
- Dependency ordering (a unit cannot start if dependencies incomplete)
- State validity (transitions follow the state machine)
- Evidence requirements (verification before approval)

---

## 4. Portals

### 4.1 Portal Concept

A portal is a **human decision surface** where the system requires human input to proceed.

Portals exist because:
- Some decisions cannot be reduced to deterministic oracle checks
- High-stakes transitions require human accountability
- Authority claims require human authorization

### 4.2 Portal Invocation

The system invokes a portal when:

| Trigger | Example |
|---------|---------|
| State transition requires approval | Candidate → Approved |
| Oracle result requires interpretation | Ambiguous pass/fail |
| Exception requested | Deviation, deferral, waiver |
| Escalation triggered | Budget exceeded, scope question |
| Configuration requires it | User-defined portal points |

### 4.3 Portal Inputs and Outputs

**Portal receives:**
- Subject (what is being decided)
- Context (relevant evidence, state, history)
- Decision options (approve, reject, defer, etc.)

**Portal produces:**
- Decision record (binding)
- Rationale (optional but recommended)
- State transition authorization

### 4.4 Portal Configuration

Users configure portals by specifying:

| Field | Description |
|-------|-------------|
| `portal_id` | Identifier for this portal type |
| `trigger_conditions` | When this portal is invoked |
| `required_evidence` | What must be present before portal |
| `decision_options` | What choices the human has |
| `timeout_behavior` | What happens if no response |

---

## 5. Oracles and Evidence

### 5.1 Oracle Concept

An oracle is a **deterministic verification mechanism** that produces pass/fail evidence.

Oracles are:
- Deterministic (same inputs → same outputs)
- Evidence-producing (results are recorded)
- Scoped (they verify specific claims, not "correctness")

### 5.2 Oracle Types

| Type | What it verifies |
|------|------------------|
| **Test suite** | Code behavior against specifications |
| **Linter/formatter** | Code style and structure |
| **Schema validator** | Data/document structure |
| **Build check** | Compilation/assembly success |
| **Custom oracle** | User-defined verification logic |

### 5.3 Evidence Bundles

Oracle runs produce evidence bundles:

| Field | Description |
|-------|-------------|
| `oracle_id` | Which oracle ran |
| `candidate_ref` | What was verified (content-addressed) |
| `result` | PASS / FAIL / ERROR |
| `details` | Structured output from oracle |
| `environment` | Execution environment fingerprint |
| `timestamp` | When the oracle ran |

Evidence bundles are content-addressed and immutable.

### 5.4 Verification vs Truth

**Verified** means: "The specified oracles passed for the declared scope."

**Verified does NOT mean:**
- The system is correct
- All edge cases are covered
- The oracle suite is complete

Verification is bounded and honest. It produces evidence about specific checks, not claims of completeness.

---

## 6. State and Events

### 6.1 Event-Sourced State

SOLVER-Ralph uses event sourcing:

- All state changes are recorded as events
- Current state is computed from event history
- Events are immutable and ordered
- Any state can be reconstructed by replaying events

### 6.2 Event Types

| Category | Examples |
|----------|----------|
| **Loop events** | LoopStarted, LoopCompleted, LoopFailed |
| **Work unit events** | CandidateSubmitted, VerificationCompleted, ApprovalGranted |
| **Portal events** | PortalInvoked, DecisionRecorded |
| **System events** | OracleRun, ContextCompiled, StateTransition |

### 6.3 State Transitions

State transitions are **deterministic given inputs**:

- Given a state and an action, whether the transition is valid is computable
- If valid, the resulting state is deterministic
- Invalid transitions are rejected

This enables:
- Audit (any path can be verified)
- Replay (same inputs → same state)
- Enforcement (rules cannot be bypassed)

---

## 7. Using SOLVER-Ralph

### 7.1 Basic Usage Pattern

1. **Define work units** — what needs to be produced, with what constraints
2. **Configure oracles** — what verification is required
3. **Configure portals** — where human approval is needed
4. **Start loops** — provide instructions and let agents work
5. **Respond to portals** — make decisions when requested
6. **Track progress** — monitor state transitions and evidence

### 7.2 Developing Agentic Workflows

Users develop workflows by:

- Defining work unit structures and dependencies
- Writing instructions that guide agent behavior
- Configuring verification requirements
- Setting up portal points for human oversight

The platform provides the infrastructure; users provide the workflow definition.

### 7.3 Integration Points

| Driving (inbound) | Driven (outbound) |
|-------------------|-------------------|
| CLI / API for instructions | LLM providers |
| Portal UI for approvals | Persistence (event store) |
| Webhook triggers | Oracle runners |
| Programmatic control | Notification systems |

---

## 8. Relationship to Other Documents

| Document | Relationship |
|----------|--------------|
| **SR-CONTRACT** | Defines binding invariants the platform enforces |
| **SR-SPEC** | Defines technical mechanics (events, APIs, schemas) |
| **SR-AGENTS** | Defines agent constraints the platform enforces |
| **SR-DIRECTIVE** | Defines execution policies (may reference platform capabilities) |
| **SR-TYPES** | Defines artifact types work units can produce |
| **SR-ETT** | Explains trust boundaries the platform implements |

This guide describes how to **use** the platform. The other documents **specify** the platform.

---

## 9. What This Document Is Not

- It is not a problem-solving methodology
- It is not a workflow prescription
- It is not a project management framework
- It is not binding requirements (see SR-CONTRACT, SR-SPEC)

It is a guide to using SOLVER-Ralph as infrastructure for controlled agentic work.
