Below is a semantically-consistent, implementation-agnostic definition of a Ralph-loop for your system (SOLVER-Ralph), written as design instructions an agent can implement in any language.

⸻

1) What a Ralph-loop is (in this system)

A Ralph Loop is a bounded workflow instance (“work unit”) that progresses a single deliverable toward an evidence-backed, auditable completion state, using fresh-context iterations, typed references, oracle-produced evidence bundles, and (when required) human portal approvals.  ￼

A Ralph-loop is not “just a shell while true.” It is a governed state machine that must enforce:
	•	Bounded iteration budgets and a hard stop (extension/termination requires a human decision when budgets are exhausted).  ￼
	•	Fresh-context iterations with controlled memory, where the system records what was attempted, what candidates/evidence were produced, stop triggers, and a summary.  ￼
	•	Context provenance and “no ghost inputs”: the iteration’s effective context must be derivable only from the IterationStarted event payload + dereferenced refs (no hidden prompt scraps, no implicit repo state, no unrecorded docs).  ￼
	•	A strict separation between proposals (non-authoritative drafts) and commitment objects (durable, content-addressed objects that can be referenced by downstream work). Binding claims must be derivable only from commitment objects.  ￼

⸻

2) Authority model (what the loop may claim)

The loop’s agent is a stochastic generator and has no authority for binding claims (e.g., it can’t “declare” Verified/Approved/Shippable by narrative).  ￼

The loop may only advance binding states through recorded commitment objects:
	•	Verification is oracle evidence (non-binding); Approval is a binding, attributable human decision at a Portal.  ￼
	•	A Candidate may be marked Verified only when an Evidence Bundle exists for a Run against that Candidate, with integrity checks and required oracle results (and no unresolved integrity conditions).  ￼

⸻

3) Inputs that govern task selection (what the loop chooses from)

A Ralph-loop selects “what to do next” from eligible deliverables defined by your governed artifacts:
	•	SR-PLAN provides the decomposition into deliverables and their depends_on relationships; only dependency edges are binding ordering.  ￼
	•	SR-DIRECTIVE owns execution policy per deliverable: gates, oracle suite, evidence kinds, budgets, stop triggers, and required portals/approvals.  ￼
	•	SR-CONTRACT and SR-SPEC constrain what implementations may do and how state/evidence is represented.  ￼

Eligibility rule (minimum): a deliverable is eligible if all its depends_on deliverables are complete (and not stale), and the deliverable is not blocked by a stop trigger, unresolved integrity condition, or missing required portal decision.

⸻

4) The loop’s core invariant: “context must be an artifact”

Each iteration must produce/attach a deterministic context artifact so that an auditor can answer: “What exactly did the agent see and rely on?”

4.1 IterationStarted is authoritative provenance

IterationStarted must include refs[] constituting authoritative provenance and must be emitted by SYSTEM.  ￼

4.2 No ghost inputs

The iteration context must be derivable solely from IterationStarted + dereferenced refs; unrepresented inputs must not influence work.  ￼

4.3 refs[] must be typed, hashed, and semantically classified

Refs must be typed and sufficient for dependency/audit queries. meta.content_hash is required for dereferenceable objects; and the system must distinguish:
	•	rel=depends_on for semantic dependencies (participates in staleness traversal; blocking by default)
	•	rel=supported_by for audit provenance (non-blocking by default)  ￼

Design instruction: your “context artifact” is a commitment object that:
	•	lists all refs used, with meta.content_hash and (ideally) meta.selector for stable slices, and
	•	is itself content-addressed and referenced from IterationStarted.

⸻

5) Canonical iteration lifecycle (the Ralph-loop algorithm)

Step 0 — Compute eligible work
	1.	Build the candidate set from SR-PLAN deliverables (“one loop per deliverable” is intended).  ￼
	2.	Filter to those whose depends_on prerequisites are complete and not stale.
	3.	For each eligible deliverable, load its execution requirements from SR-DIRECTIVE (gates, suite, evidence kind(s), budgets, portals).  ￼
	4.	Select the next deliverable by priority policy (your choice), but record the rationale as a non-binding record (never as binding state).

Step 1 — Start a fresh iteration (bounded + governed)
	1.	Create a new iteration_id.
	2.	Compile the iteration’s context bundle deterministically:
	•	required governed artifacts (at least SR-CONTRACT/SR-SPEC/SR-DIRECTIVE and the deliverable spec from SR-PLAN),
	•	any task-local instructions,
	•	pinned oracle suite identity and environment fingerprint (as required by your implementation of “no ghost inputs” and oracle integrity).
	3.	Emit IterationStarted with refs to that context bundle and all other authoritative inputs.  ￼

Step 2 — Do work → produce Candidate(s)
	1.	The agent produces outputs as proposals until they are materialized into a Candidate (content-addressed snapshot) and recorded in the event substrate (i.e., they become a commitment object).  ￼
	2.	Ensure Candidate identity is stable and immutable once recorded.  ￼

Step 3 — Run oracles → produce Evidence Bundle(s)
	1.	Execute the required oracle suite for the deliverable (as configured by SR-DIRECTIVE).
	2.	Record outputs as an Evidence Bundle with integrity checks, suite identity/hash, and required oracle results.  ￼
	3.	Enforce “no silent oracle weakening” and stop on integrity problems (tamper, gaps, env mismatch, flake).  ￼

Step 4 — Evaluate gates (without pretending they’re approvals)
	1.	Gate evaluation is derived from evidence + policy.
	2.	If a required oracle is missing, flaky, mismatched, or tampered: trigger stop-the-line.  ￼
	3.	If a gate FAIL would be waived: it must be an explicit exception/waiver record; waivers may waive oracle FAIL outcomes but not integrity conditions.  ￼

Step 5 — Portal escalation (only when required)

If SR-DIRECTIVE requires a portal for this deliverable (e.g., engineering acceptance, release approval), the loop must:
	1.	Package the relevant candidate + evidence references.
	2.	Request the portal decision.
	3.	Treat the portal output as the binding Approval record (a commitment object).  ￼

Step 6 — Record the iteration summary (Loop Record)

Regardless of success/failure, record a Loop Record that includes:
	•	the chosen deliverable,
	•	the full context refs (or ref to the context artifact),
	•	candidates produced,
	•	evidence bundles produced,
	•	gates evaluated and their outcomes,
	•	stop triggers fired,
	•	remaining budgets,
	•	next-step recommendation (non-binding).  ￼

⸻

6) Stop conditions and “done” criteria

6.1 Mandatory stop triggers

The system must implement stop triggers at least for:
	•	ORACLE_TAMPER, ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE,
	•	REPEATED_FAILURE (N≥3),
	•	BUDGET_EXHAUSTED.  ￼

6.2 What “complete” means (per deliverable)

A deliverable is complete when SR-DIRECTIVE’s required gates are satisfied by recorded evidence and any required portals/approvals are present—not when an agent says “done.”

If a higher-level state like “Shippable” is in scope, it additionally requires Verified + human approval + a finalized Freeze Record with governed artifacts, active exceptions, evidence refs, approval ref, and candidate identity.  ￼

⸻

7) Practical implication for your “task tracking when work is complex”

You don’t need to linearize PLAN into an ad-hoc runtime checklist to stay oriented. Your trustworthy tracking substrate is:
	•	the event log + typed ref graph (dependency + provenance), and
	•	projections derived from it (reconstructible from the audit trail).  ￼

SR-PLAN is explicit that dependencies are the only binding ordering and SR-DIRECTIVE owns scheduling/execution policy.  ￼
So the loop’s job is to:
	•	compute eligibility from depends_on,
	•	emit context as refs (no ghost inputs),
	•	produce candidates/evidence/approvals as commitment objects,
	•	and let projections answer “what’s complete” from the recorded substrate.

⸻

If you want, paste the existing “instructions prompt” you plan to hand the agent each iteration, and I’ll rewrite it so it forces: (1) deterministic context compilation into an artifact, (2) typed refs + hashes, (3) explicit stop triggers, (4) no fake ‘done’ claims unless the substrate supports it.