# SR-MODEL

Conceptual foundation for agentic development workflow.

This document establishes the mental model that agents must understand before reading any other workflow document. It explains what we are building, how the documents relate, and what transformation we are trying to achieve.

---

## The Three Layers

### Layer 1: Building

Layer 1 is where we are now. We are building SOLVER-Ralph.

The five specification documents exist as documents — artifacts that constrain and inform the work of writing code. Agents read SR-CONTRACT and implement what it says. The spec is input; the platform is output.

The workflow documents (SR-PLAN, SR-DIRECTIVE, SR-AGENTS, SR-CHANGE, and this document) are operational aids for this effort. They help us organize the work but they are not part of what we are building.

When Layer 1 is complete, we will have produced a working platform.

### Layer 2: The Platform

Layer 2 is the platform itself.

The specification documents no longer exist as documents that anyone reads at runtime. They have become code. The invariants in SR-CONTRACT are enforcement logic in the domain core. The schemas in SR-SPEC are Rust structs and PostgreSQL tables. The event model is a real event store.

The platform does not consult the spec — it embodies the spec. If SR-CONTRACT says agents cannot create approvals, the platform makes that structurally impossible.

The documents were the blueprint; the platform is the building.

### Layer 3: Usage

Layer 3 is usage.

People use the running platform to do their own work. They create projects, define work units, run loops, produce candidates, gather evidence, obtain approvals. They might write their own equivalents of SR-PLAN and SR-DIRECTIVE for their projects — decomposing their goals into deliverables, configuring their gates and portals.

The platform is agnostic to what they build. It provides the machinery; they provide the purpose. Users experience the specification as platform behavior, not as documents they read.

### The Transformation

The transformation across layers is the key insight.

At Layer 1, SR-CONTRACT is a document that tells us what to build. At Layer 2, those same ideas exist as running code that enforces invariants. At Layer 3, users encounter those invariants as "how the platform works" without ever seeing the document that specified them.

---

## Two Specifications

This effort involves two separate specifications for two different domains.

### Platform Specification

Defines what SOLVER-Ralph is.

| Document | Role |
|----------|------|
| SR-GUIDE | Usage guidance for governed artifacts |
| SR-TYPES | Vocabulary and type system |
| SR-INTENT | Rationale and design decisions |
| SR-CONTRACT | Binding invariants |
| SR-SPEC | Technical mechanics |

These five documents define the platform completely. Hand them to any competent team; they can build a conforming implementation.

Location: `docs/spec/`

### Workflow Specification

Defines how agents work on codebases.

| Document | Role |
|----------|------|
| SR-MODEL | This document; conceptual foundation |
| SR-AGENTS | Agent constraints and operating model |
| SR-CHANGE | How specification documents are maintained |
| SR-PLAN | Build decomposition and deliverables |
| SR-DIRECTIVE | Execution configuration and gating |

These documents specify the agentic development process. They are authoritative instructions for agents performing work.

Location: `docs/workflow/`

### Relationship

The two specifications are related but distinct.

SOLVER-Ralph is a platform for agentic work. We are using agentic work to build it. But the platform specification describes a product. The workflow specification describes a process. They are not the same thing.

The platform specification is the input to Layer 1 work. The workflow specification governs how that work is conducted.

---

## Document Dependencies

### Platform Specification

```
SR-GUIDE ← SR-TYPES ← SR-INTENT ← SR-CONTRACT ← SR-SPEC
```

Each document may reference those to its left. SR-SPEC references all four. SR-GUIDE references nothing.

### Workflow Specification

```
SR-MODEL ← SR-AGENTS
SR-MODEL ← SR-CHANGE
SR-MODEL ← SR-PLAN ← SR-DIRECTIVE
```

All workflow documents assume understanding of SR-MODEL. SR-DIRECTIVE operationalizes SR-PLAN.

### Cross-Specification

Workflow documents reference the platform specification as their subject matter — what is being built. They do not modify or extend the platform specification's semantics.

---

## For Agents

When you receive a task:

1. Understand which layer you are operating in. During this build, you are in Layer 1.

2. Understand which specification governs your work. The platform specification tells you what to build. The workflow specification tells you how to work.

3. The platform specification is the authority on what SOLVER-Ralph is. Do not invent requirements. Do not infer capabilities. Build what the spec says.

4. The workflow specification is the authority on how you operate. Follow SR-AGENTS for your constraints. Follow SR-DIRECTIVE for execution policy. Follow SR-PLAN for sequencing.

5. Your outputs are proposals until they become commitment objects through the appropriate verification and approval mechanisms.
