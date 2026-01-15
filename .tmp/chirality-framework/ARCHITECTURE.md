# Chirality Framework Architecture

## 1. Core Philosophy: "Conversation as Program"

The Chirality Framework is not a traditional Python application. Its architecture is designed to serve a single purpose: to execute a **"Conversation as Program."**

- **The Program Is the Transcript:** The *actual program* is the sequence of version-controlled prompt assets and the resulting dialogue transcript. The final output is an artifact of this structured conversation.
- **The Code Is the Control-Plane:** The Python codebase is a deterministic **Orchestrator** or **Control-Plane**. Its only job is to run the "real" program (the conversation) reliably and verifiably.
- **The LLM Is the Runtime:** The Large Language Model is the runtime environment where the semantic program executes.

This philosophy mandates a strict separation of concerns, which is implemented using a Domain-Driven Design (DDD) structure.

## 2. The "Pure History" Model & Data Flow

The framework operates on a "Pure History" model to maintain semantic integrity.

- **The Transcript is Sacred:** The conversational history sent to the LLM contains only pure semantic content (instructions, matrices, lenses). It **must not** contain any framework metadata (e.g., `system_sha`, `source: catalog`, `mode: cross`).
- **Prompts are Static Assets:** Each `.md` file is a complete, static user turn. The orchestrator's job is to sequence them.
- **No `{{content}}` Placeholders:** The orchestrator never injects the output of a previous LLM turn into the prompt for the next turn. The LLM is expected to derive this context from the conversation history.
- **Metadata Placeholders are Allowed:** Prompts *can* contain placeholders for out-of-band data needed for an operation, such as `{{lens}}` (from the lens catalog) or `{{row_name}}` (metadata for the instruction).
- **Provenance is External:** All metadata (SHAs, timestamps, lens sources, etc.) is captured exclusively in the Python trace objects and is never exposed to the LLM.

