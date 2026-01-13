---
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "SR-GUIDE"
  type: "governance.usage_guide"
  title: "SOLVER-Ralph Usage Guide"
  version: "3.1.0-draft.1"
  status: "draft"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  supersedes:
    - "SR-GUIDE@3.0.0-draft.1"
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
        Guide for understanding and using the SOLVER-Ralph platform for
        controlled agentic work with human approval at critical junctures.
      core_abstraction: "ralph-loop"
      architectural_style: "hexagonal"
---

# SOLVER-Ralph Usage Guide — v3.1.0-draft.1

## Purpose

This document describes SOLVER-Ralph: what it is, how it works, and how to use it.

SOLVER-Ralph is a **software platform** for controlled agentic work. It provides infrastructure for agents to perform long-running tasks with human approval required at critical junctures.

---

## 1. Three Layers

Understanding SOLVER-Ralph requires distinguishing three layers:

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: BOOTSTRAPPING                                         │
│                                                                  │
│  Building SOLVER-Ralph                                           │
│                                                                  │
│  - Agents (Claude, etc.) write code constrained by SR-* docs     │
│  - Humans approve work via conversation                          │
│  - SR-CONTRACT, SR-SPEC define what to build                     │
│  - SR-PLAN defines the build sequence                            │
│                                                                  │
│  This layer is SCAFFOLDING. The SR-* documents guide the build   │
│  but do not become part of the running platform.                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ produces
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 2: THE PLATFORM                                          │
│                                                                  │
│  SOLVER-Ralph as running software                                │
│                                                                  │
│  - Ralph-loops: iteration mechanism for agent work               │
│  - Portals: human approval surfaces                              │
│  - Oracles: deterministic verification                           │
│  - State machine: work unit progression                          │
│  - Event store: auditable state changes                          │
│                                                                  │
│  The specifications (SR-CONTRACT, SR-SPEC) become CODE.          │
│  The platform embodies constraints; it doesn't read documents.   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ enables
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 3: USAGE                                                 │
│                                                                  │
│  Working through SOLVER-Ralph                                    │
│                                                                  │
│  - Users provide: work definitions, instructions, configurations │
│  - Platform provides: infrastructure, constraints, state mgmt    │
│  - Agents are constrained by ARCHITECTURE, not by documents      │
│                                                                  │
│  Users interact with the platform via API.                       │
│  The SR-* documents are not visible at this layer.               │
└─────────────────────────────────────────────────────────────────┘
```

### 1.1 Why This Matters

**The SR-* documents are Layer 1 artifacts.** They specify what to build. They constrain agents during the build. But once the platform is built:

- SR-CONTRACT becomes enforcement code
- SR-SPEC becomes the implementation
- SR-AGENTS becomes agent authority rules in the platform
- SR-GUIDE becomes documentation (this document)

**Users of the platform (Layer 3) don't read SR-CONTRACT.** They interact with the platform, which embodies those constraints.

### 1.2 The Recursion Trap

It's tempting to think: "SOLVER-Ralph should be able to build SOLVER-Ralph."

This is conceptually true but practically confusing:
- You can't use SOLVER-Ralph to build SOLVER-Ralph until SOLVER-Ralph exists
- Layer 1 (bootstrapping) is necessarily different from Layer 3 (usage)
- The bootstrapping scaffolding (SR-PLAN, task instructions, approval conversations) is not part of the platform

**Accept the asymmetry:** Building the platform and using the platform are different activities, even if they share concepts.

---

## 2. What SOLVER-Ralph Is

SOLVER-Ralph is infrastructure for controlled agentic work.

### 2.1 The Core Problem

Agents (LLMs) are powerful but non-deterministic. They can produce useful work, but:
- They cannot reliably verify their own output
- They should not have authority to approve their own work
- Long-running sessions accumulate errors
- Their claims ("tests pass") are not evidence

SOLVER-Ralph addresses this by:
- Breaking work into bounded iterations (ralph-loops)
- Requiring oracle verification (deterministic evidence)
- Requiring human approval at critical junctures (portals)
- Tracking state changes as auditable events

### 2.2 Hexagonal Architecture

SOLVER-Ralph follows hexagonal (ports-and-adapters) architecture:

```
                    ┌─────────────────────────────────────┐
                    │                                     │
   Driving Ports    │         DOMAIN CORE                 │    Driven Ports
   (how you use it) │                                     │    (what it uses)
                    │    ┌─────────────────────┐          │
        ───────────►│    │                     │          │►───────────
   Work Definitions │    │    Ralph-Loop       │          │   Persistence
                    │    │    Engine           │          │
        ───────────►│    │  ┌─────────────┐    │          │►───────────
   Approvals        │    │  │ Work Unit   │    │          │   LLM Adapters
                    │    │  │ State       │    │          │
        ◄───────────│    │  └─────────────┘    │          │►───────────
   Portal Requests  │    │                     │          │   Oracle Runners
                    │    └─────────────────────┘          │
        ◄───────────│                                     │►───────────
   Results/Evidence │                                     │   Event Store
                    │                                     │
                    └─────────────────────────────────────┘
```

**Key implications:**

- The domain core (ralph-loop logic, state machines) is independent of adapters
- Adapters (which LLM, which database, which oracle runner) can be replaced
- The platform enforces constraints regardless of which adapters are used
- Users interact through driving ports; the platform uses driven ports

### 2.3 What the Platform Provides

| Component | What it does |
|-----------|--------------|
| **Ralph-loop engine** | Executes bounded iterations of agent work |
| **Work unit state machine** | Tracks work through defined states |
| **Portal system** | Surfaces decisions for human approval |
| **Oracle integration** | Runs deterministic verification, captures evidence |
| **Evidence binding** | Links work products to verification results |
| **Event store** | Records all state changes for audit/replay |
| **Context compiler** | Assembles inputs for each loop iteration |

### 2.4 What the Platform Does Not Provide

| Concern | Where it belongs |
|---------|------------------|
| Workflow methodology | User-defined |
| Decomposition strategy | User-defined |
| Task instructions | User-defined |
| Approval criteria | User-configured |
| Verification scope | User-configured |
| "How to think about problems" | Not the platform's concern |

---

## 3. The Ralph-Loop

### 3.1 Core Concept

A ralph-loop is a **single, bounded iteration of agent work**.

Each loop:
1. Receives compiled context (instructions + state + constraints)
2. Agent produces work output
3. Loop may trigger verification (oracle run)
4. Loop may request human input (portal)
5. State changes are recorded as events
6. Loop terminates

Loops are **short-lived and fresh-context**. This is intentional:
- Long-running agent sessions accumulate errors
- Fresh context prevents "sticky" mistakes
- Bounded iterations are auditable

### 3.2 Loop Inputs

| Input | Description |
|-------|-------------|
| **Instructions** | What this loop should accomplish |
| **Work unit reference** | Which work unit this advances |
| **Context bundle** | Prior state, relevant artifacts |
| **Constraints** | Budget limits, tool restrictions, scope |

### 3.3 Loop Outputs

| Output | Description |
|--------|-------------|
| **Work products** | Files, code, artifacts produced |
| **Candidate** | Content-addressed snapshot (if work submitted) |
| **Portal requests** | Decisions needed from humans |
| **Events** | State changes recorded |

### 3.4 Loop Termination

| Condition | Result |
|-----------|--------|
| Work submitted | Candidate created; may proceed to verification |
| Budget exhausted | Loop stopped; partial progress recorded |
| Portal required | Loop blocked; awaiting human |
| Error | Loop failed; error recorded |
| Explicit stop | Human or system halted |

### 3.5 Loop Constraints (Enforced by Platform)

| Constraint | How enforced |
|------------|--------------|
| **Budget** | Loop terminates when limits exceeded |
| **Scope** | Out-of-scope state changes rejected |
| **Authority** | Agent cannot claim Verified/Approved status |
| **Evidence** | Claims without oracle evidence are proposals only |

---

## 4. Work Units

### 4.1 Work Unit as State Machine

A work unit progresses through states:

```
┌──────────┐     ┌───────────┐     ┌──────────┐     ┌──────────┐
│  DRAFT   │────►│ SUBMITTED │────►│ VERIFIED │────►│ APPROVED │
└──────────┘     └───────────┘     └──────────┘     └──────────┘
                       │                                  │
                       │ (verification failed)            │
                       ▼                                  ▼
                 ┌──────────┐                       ┌──────────┐
                 │ REJECTED │                       │ COMPLETE │
                 └──────────┘                       └──────────┘
```

| State | Meaning |
|-------|---------|
| **DRAFT** | Work in progress |
| **SUBMITTED** | Candidate submitted; awaiting verification |
| **VERIFIED** | Oracle evidence confirms conformance |
| **APPROVED** | Human approved at portal |
| **COMPLETE** | Work unit closed |
| **REJECTED** | Verification or approval failed |

### 4.2 State Transitions

Transitions are **deterministic given inputs**:
- Given current state and action, validity is computable
- Valid transitions produce deterministic next state
- Invalid transitions are rejected
- All transitions recorded as events

### 4.3 Work Unit Definition (User-Provided)

Users define work units with:

| Field | Description |
|-------|-------------|
| `id` | Stable identifier |
| `instructions` | What to produce |
| `constraints` | Scope, budget, tools |
| `verification` | Oracle configuration |
| `portal` | Approval requirements |
| `depends_on` | Dependencies on other work units |

The platform is agnostic to *what* the work is. It enforces *how* work progresses.

---

## 5. Portals

### 5.1 Portal Concept

A portal is a point where the platform **requires human decision** to proceed.

Portals exist because:
- Some decisions cannot be automated
- High-stakes transitions need human accountability
- Authority to approve belongs to humans, not agents

### 5.2 When Portals Are Invoked

| Trigger | Example |
|---------|---------|
| State transition requires approval | Verified → Approved |
| Verification result needs interpretation | Ambiguous oracle output |
| Exception requested | Need to proceed despite failed verification |
| Escalation | Budget exceeded, scope question |

### 5.3 Portal Interaction

**Portal presents:**
- Subject (what needs decision)
- Context (evidence, state, history)
- Options (approve, reject, defer, etc.)

**Human provides:**
- Decision
- Rationale (optional but recommended)

**Platform records:**
- Portal Decision (binding record)
- Authorizes state transition (if approved)

---

## 6. Oracles and Evidence

### 6.1 Oracle Concept

An oracle is a **deterministic verification mechanism**.

Oracles:
- Take defined inputs
- Produce pass/fail result
- Are deterministic (same inputs → same result)
- Generate evidence (recorded proof of execution)

### 6.2 Evidence Bundles

Oracle runs produce evidence bundles:

| Field | Description |
|-------|-------------|
| `oracle_id` | Which oracle ran |
| `subject_ref` | What was verified (content-addressed) |
| `result` | PASS / FAIL / ERROR |
| `details` | Structured output |
| `environment` | Execution environment fingerprint |
| `timestamp` | When oracle ran |

Evidence bundles are immutable and content-addressed.

### 6.3 Verified ≠ Correct

**Verified** means: specified oracles passed for declared scope.

**Verified does NOT mean:**
- The work is correct in all respects
- All edge cases are covered
- The oracle suite is complete

Verification is bounded and honest. It's evidence about specific checks.

---

## 7. State and Events

### 7.1 Event Sourcing

All state changes are recorded as events:
- Events are immutable and ordered
- Current state is computed from event history
- Any state can be reconstructed by replay
- Audit trail is complete

### 7.2 Event Categories

| Category | Examples |
|----------|----------|
| **Loop events** | LoopStarted, LoopCompleted, LoopFailed |
| **Work unit events** | Submitted, VerificationCompleted, Approved |
| **Portal events** | PortalInvoked, DecisionRecorded |
| **Evidence events** | OracleRun, EvidenceBundleCreated |

### 7.3 Deterministic State

State transitions are deterministic:
- Same events in same order → same state
- Enables replay and audit
- Invalid transitions rejected regardless of who/what attempts them

---

## 8. Using SOLVER-Ralph

### 8.1 Basic Pattern

1. **Define work units** — what needs to be done, with what constraints
2. **Configure oracles** — what verification is required
3. **Configure portals** — where human approval is needed
4. **Start loops** — agents execute within constraints
5. **Respond to portals** — make decisions when requested
6. **Track progress** — monitor state via events

### 8.2 The Platform is Agnostic

SOLVER-Ralph doesn't know or care:
- What kind of work you're doing
- How you decompose problems
- What your methodology is
- What your approval criteria are

It provides:
- Bounded iteration (ralph-loops)
- State tracking (work units)
- Verification infrastructure (oracles)
- Approval infrastructure (portals)
- Audit trail (events)

You provide the work definition and make the decisions.

### 8.3 Integration

| Driving (inbound) | Driven (outbound) |
|-------------------|-------------------|
| API for work definitions | LLM providers |
| API for approvals | Persistence (event store) |
| Webhooks / callbacks | Oracle runners |
| CLI tools | Notification systems |

---

## 9. Relationship to SR-* Documents

### 9.1 The Documents Are Layer 1 Scaffolding

The SR-* documents (SR-CONTRACT, SR-SPEC, SR-AGENTS, etc.) are specifications for building SOLVER-Ralph.

| Document | Role in Building | Role in Platform |
|----------|------------------|------------------|
| SR-CONTRACT | Defines invariants to implement | Becomes enforcement code |
| SR-SPEC | Defines technical mechanics | Becomes the implementation |
| SR-AGENTS | Defines agent constraints | Becomes authority rules |
| SR-GUIDE | Documents the platform | This document |
| SR-INTENT | Design rationale | Informs design; not load-bearing |
| SR-TYPES | Type system for build artifacts | Platform has its own domain types |
| SR-PLAN | Build plan | Not part of the platform |
| SR-DIRECTIVE | Build execution governance | Not part of the platform |
| SR-CHANGE | Specification change control | Not part of the platform |

### 9.2 Users Don't See SR-* Documents

At Layer 3 (usage), users interact with the **platform**, not the specifications:

- They call APIs, not read SR-SPEC
- They experience portal UIs, not read SR-CONTRACT
- They configure work units, not read SR-TYPES

The specifications guided the build. The platform embodies them.

### 9.3 Maintaining vs Using

**Maintaining SOLVER-Ralph** (fixing bugs, adding features) may involve:
- Reading SR-CONTRACT to understand invariants
- Reading SR-SPEC to understand mechanics
- Updating specifications and code together

**Using SOLVER-Ralph** involves:
- Defining work
- Responding to portals
- Reviewing evidence

Different activities, different concerns.

---

## 10. What This Document Is Not

- It is not a problem-solving methodology
- It is not a project management framework
- It is not binding specification (see SR-CONTRACT, SR-SPEC)
- It is not a tutorial (that may exist separately)

It is a guide to understanding what SOLVER-Ralph is and how it works.
