# Phase 1 Prompt Assets Manifest

## Architecture Principle: Conversational Chain

The dialogue history provides context, not template rendering.

## Data Flow

```
Stage 1 (mechanical) → Adds to conversation history
    ↓
Stage 2 (interpreted) → Reads from history (static prompt)
    ↓  
Stage 3 (lensed) → Receives lens + content as minimal params
    ↓
All outputs accumulate in dialogue transcript
```

## Placeholder Usage by Stage

### Stage 1: Mechanical (`*/mechanical.md`)
**May use placeholders for INPUTS only:**
- `{{matrix_a}}` - Matrix A elements (for C, D)
- `{{matrix_b}}` - Matrix B elements (for C)
- `{{matrix_j}}` - Matrix J elements (for F, X)
- `{{station}}` - Current station name
- `{{rows}}` - Row labels as JSON array
- `{{cols}}` - Column labels as JSON array
- `{{json_tail}}` - JSON output contract

**Note**: K, G, T are computed in code (transpose/slice), not via prompts

### Stage 2: Interpreted (`*/interpreted.md`)
**STATIC - NO OUTPUT PLACEHOLDERS:**
- `{{json_tail}}` - JSON output contract only
- Relies entirely on conversation history
- Simple instruction to resolve operations

### Stage 3: Lensed (`*/lensed.md`)
**Minimal data parameters only:**
- `{{lens}}` - The interpretive lens text
- `{{station}}` - Current station name
- `{{json_tail}}` - JSON output contract
- **NO {{content}} placeholders** - relies on conversation history

## Special Cases

### Matrix D - CRITICAL INVARIANT
- **Has mechanical.md**: Per 4-stage requirement (D is 4-stage conversational)
- **Mechanical stage**: Generates concatenation recipe WITH operators
- **Interpreted stage**: Resolves ONLY addition (+), no multiplication

### Matrix Z - CRITICAL INVARIANTS  
- **No interpreted stage**: Skips Stage 2 (explicitly skipped)
- **Lensed stage**: Operates directly on X's **lensed** layer with station shift
- **Principles extraction**: Separate LLM call after lensed stage
- **Build Constraint**: Z.build.combinatorial == X.build.lens_interpreted
- **Dialogue Order**: X/lensed → Z/lensed → Z/principles

### Pure Code Operations
These are NOT prompted, computed directly in Python:
- **Matrix K**: Transpose of D
- **Matrix G**: First 3 rows of Z
- **Array P**: Fourth row of Z  
- **Matrix T**: Transpose of J
- **Scalar H**: Single element from P

## Conversation Continuity

Each matrix computation follows this pattern:

1. **User**: Content from mechanical.md with input placeholders filled
   **Assistant**: Mechanical construction with operators

2. **User**: Static content from interpreted.md
   **Assistant**: Resolved semantic operations

3. **User**: Content from lensed.md with lens/content params
   **Assistant**: Lens-interpreted result

All turns append to the same conversation history, building semantic state progressively.

## JSON Tails

Each prompt file includes `{{json_tail}}` which will be replaced with the appropriate JSON output contract from `infrastructure/prompts/json_tails.py`.

## Authoring Guidelines for User

1. **Mechanical prompts**: Provide clear instructions for constructing formulas
2. **Interpreted prompts**: Keep minimal - just "resolve operations" 
3. **Lensed prompts**: Explain how to apply lens perspective
4. **Maintain conversation flow**: Each stage builds on prior context
5. **No inline semantics**: All semantic logic lives in these assets