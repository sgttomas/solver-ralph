# SOLVER-Ralph Claude Code Sub-Agent Authoring Specification

## 1. Purpose {#purpose}

This document specifies how to create Claude Code sub-agents (“subagents”) that are compatible with the SOLVER-Ralph governance model and Ralph Loop execution protocol.

Subagents are treated as **work-plane assistants**: they help an Agent Worker perform constrained subtasks (analysis, code review, evidence packaging, etc.) without altering SOLVER-Ralph authority boundaries.

## 2. Scope and non-goals {#scope}

### 2.1 In scope {#scope-in}

- Filesystem-based subagent definition for Claude Code.
- Recommended prompts, tool scopes, and output contracts for subagents used inside a Ralph Loop iteration.
- Version control and change-control guidance so subagent definitions do not become “ghost inputs”.

### 2.2 Out of scope {#scope-out}

- Defining or changing any SOLVER-Ralph governance rules, portals, lifecycle states, or Verified/Approved semantics.
- Agent-worker-to-SR-SPEC API integration details (covered elsewhere).
- Deterministic oracle execution (handled by the oracle runner sandbox and suite definitions).

## 3. Definitions {#definitions}

- **Main agent (Claude Code session):** the primary Claude Code agent instance that can invoke subagents.
- **Subagent:** a specialized Claude agent instance with its own system prompt and (optionally) limited tool access.
- **Ralph Loop iteration:** a fresh-context work cycle whose authoritative semantic inputs are the `IterationStarted` payload + dereferenced `IterationStarted.refs[]`.

## 4. Placement in the SOLVER-Ralph architecture {#placement}

Subagents live entirely in the **work plane** and are therefore **out-of-domain**:

- They MAY assist the Agent Worker in producing Candidates, summaries, or evidence packages.
- They MUST NOT be treated as authorities for governance-relevant claims (verification, approval, exception closure, release).

A subagent is best understood as a **structured internal helper**: it increases throughput and reduces cognitive load, but does not change what is binding.

## 5. Claude Code subagent definition format {#claude-format}

### 5.1 Filesystem layout {#claude-format-layout}

Define each subagent as a Markdown file in:

- `.claude/agents/<agent_name>.md`

### 5.2 YAML frontmatter fields {#claude-format-frontmatter}

Claude Code expects YAML frontmatter at the top of each file with, at minimum:

- `name`
- `description`
- (optional) `tools`
- (optional) `model`

Follow the Claude Code convention that the system prompt is the Markdown body after the frontmatter.

### 5.3 SOLVER-Ralph governed metadata embedding {#claude-format-solver-metadata}

To avoid “unversioned prompt drift” and to support audit provenance when a subagent materially affects outcomes, implementers SHOULD embed SOLVER-Ralph metadata directly in the same YAML block under a `solver_ralph:` key (unknown keys are ignored by Claude Code).

**Recommended pattern:**

- Keep Claude Code keys (`name`, `description`, `tools`, `model`) at the YAML top level.
- Add `solver_ralph` + `ext` sections (as used by other governed artifacts) alongside them.

If an environment cannot tolerate additional YAML keys, implementers MAY keep a separate governed artifact file (e.g., `docs/governance/config/agents/<id>.md`) and reference the Claude Code agent file path from `ext`.

## 6. Normative SOLVER-Ralph compatibility requirements {#normative}

This section is **normative**. Use of MUST/SHOULD/MAY below is binding for SOLVER-Ralph-compatible subagents.

### 6.1 Authority boundaries {#normative-authority}

A subagent:

1. MUST be treated as **non-authoritative** with respect to verification, approval, exception status, and release readiness.
2. MUST NOT claim “Approved” or “Verified” as outcomes; it may only propose actions, cite evidence, and identify gaps.
3. MUST NOT attempt to start an iteration, emit `IterationStarted`, or simulate SYSTEM actions.

### 6.2 Input provenance discipline {#normative-provenance}

A subagent’s semantic inputs:

1. MUST be limited to the **derived iteration context** supplied by the main agent for the current iteration (i.e., content derived deterministically from `IterationStarted` + dereferenced `refs[]`) plus the subagent’s own prompt.
2. MUST treat any additional information (external docs, background assumptions, ad-hoc notes) as **out of scope** unless and until it is ingested as a typed artifact and referenced in the next iteration’s `IterationStarted.refs[]`.
3. SHOULD explicitly state when it is blocked by missing provenance (“ghost input attempt”) and return a structured request for the missing artifact/ref.

### 6.3 Tool access and recursion control {#normative-tools}

1. Subagents MUST NOT spawn additional subagents. Practically, this means the subagent definition MUST NOT include the `Task` tool in its allowed tools list.
2. The main agent environment MUST allow the `Task` tool (so it can invoke subagents), but subagents themselves must be non-recursive.
3. Subagents SHOULD be granted the minimum tool set needed for their role (principle of least privilege).

### 6.4 Output contract {#normative-output}

A SOLVER-Ralph-compatible subagent output MUST be:

- **Structured** (machine-parseable sections) and
- **Audit-friendly** (explicit references to any artifacts it used, and explicit file paths/hashes for artifacts it created).

At minimum, subagent outputs MUST include:

- `summary` (what was done / found)
- `status` in `{ok, needs_human, blocked}`
- `inputs_used` (list of artifact identifiers/paths supplied to it)
- `artifacts_created` (paths + intended hashes, if any)
- `recommended_next_steps`
- `open_risks` (if any)

Subagent outputs SHOULD avoid long narrative; prefer lists, checklists, and precise actionable items.

## 7. Interaction with /ralph-loop in Claude Code {#ralph-loop}

This specification assumes the following runtime pattern:

1. The main agent runs the iteration protocol via the native `/ralph-loop` plugin.
2. When a specialized task is needed, the main agent invokes a subagent using the Task tool and supplies:
   - the relevant slice of the derived `ContextBundle`, and
   - a narrowly scoped instruction (what to check / produce).
3. The main agent incorporates the subagent’s results into:
   - working files,
   - candidate packaging,
   - evidence manifests, and/or
   - iteration summaries,
   while preserving provenance rules.

## 8. Recommended SOLVER-Ralph subagent roles {#roles}

This section is **informative**. These are suggested starting points that map cleanly to SOLVER-Ralph membranes/harnesses.

- **governance-navigator**: quickly locate and quote the applicable clauses from governed artifacts for the current task.
- **compliance-checker**: run checklist-style checks against SR-CONTRACT/SR-SPEC invariants and produce a pass/fail + gaps list.
- **evidence-librarian**: package outputs into an evidence-style manifest (hashes, paths, relationships) for later ingestion.
- **code-reviewer**: focus on correctness, safety, and “oracle-ready” testability; does not approve, only flags.
- **change-scout**: detect when work crosses into change-management territory (normative meaning changes, exceptions needed) and route to change-manager.

## 9. Version control and change control {#change-control}

### 9.1 Version control {#change-control-vcs}

Subagent definitions SHOULD be committed in the same repository as SOLVER-Ralph implementation code.

### 9.2 Change control {#change-control-process}

When a subagent definition is referenced as an iteration input (directly or indirectly), changes to that subagent MAY affect downstream trust.

Therefore:

- If a subagent definition is treated as a typed artifact referenced in `IterationStarted.refs[]`, changes to it SHOULD follow SR-CHANGE.
- At minimum, subagent definitions SHOULD be content-addressed (hash recorded) when used for governance-relevant work.

## 10. Templates {#templates}

### 10.1 Minimal Claude Code subagent (non-recursive) {#templates-minimal}

```yaml
---
name: evidence-librarian
description: Package produced files into an audit-friendly manifest (paths + hashes) and summarize evidence gaps.
tools: Read, Grep, Glob, Write
model: sonnet
solver_ralph:
  schema: "solver-ralph.artifact-metadata/v1"
  id: "AG-EVIDENCE-LIBRARIAN"
  type: "config.agent_definition"
  title: "Claude Code Subagent: Evidence Librarian"
  version: "1.0.0"
  status: "draft"
  normative_status: "directional"
  authority_kind: "content"
  governed_by: ["SR-CHANGE"]
  created: "2026-01-10"
  updated: "2026-01-10"
ext:
  agent_definition:
    agent_role: "evidence_librarian"
    constraints:
      - "Non-authoritative; do not claim Verified/Approved."
      - "No Task tool; do not spawn subagents."
      - "Operate only on inputs provided; request typed refs for missing inputs."
---
You are the Evidence Librarian subagent for SOLVER-Ralph.

You MUST:
- Produce a concise evidence manifest draft (JSON-like in Markdown) listing files, intended sha256 hashes (if computed by parent), and what each file supports.
- Identify missing evidence required to substantiate claims.
- Never claim “Verified” or “Approved”.
- Never invent inputs; if you need context not provided, output a `blocked` status and specify exactly what artifact/ref is missing.
```

### 10.2 Structured result format {#templates-result}

Subagents SHOULD return a final section that the main agent can copy verbatim into an Iteration summary:

```markdown
## SubAgentResult
status: ok|needs_human|blocked
summary:
- ...
inputs_used:
- ...
artifacts_created:
- path: ...
  intended_sha256: ...
recommended_next_steps:
- ...
open_risks:
- ...
```

---

## 11. Appendix: Tool access profiles (informative) {#appendix-tools}

Suggested starting profiles:

- **Read-only analysis**: `Read`, `Grep`, `Glob`
- **Test execution**: `Bash`, `Read`, `Grep`, `Glob`
- **Code modification**: `Read`, `Edit`, `Write`, `Grep`, `Glob`

Do not include `Task` in subagent tools.

