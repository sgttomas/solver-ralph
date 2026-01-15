"""
LLM-backed semantic operation resolvers.

These functions implement the actual semantic operations using LLM prompting,
following the semantic-first approach for Phase 1.
"""

import re
import hashlib
from typing import List, Dict, Any
from ..llm.openai_adapter import call_responses


def _current_prompt_hash() -> str:
    """
    Generate a hash of the current normative prompt context.
    
    This creates a deterministic hash for lens catalog canonization
    and audit trails. Currently uses a basic implementation - could be
    enhanced to hash actual loaded system prompt + lens generation assets.
    
    Returns:
        SHA-256 hash as hex string
    """
    # For now, use a deterministic seed based on normative spec version
    # TODO: Hash actual loaded system prompt + lens generation asset content
    normative_context = "chirality_framework_v19.3.0_phase1_lens_generation"
    return hashlib.sha256(normative_context.encode('utf-8')).hexdigest()[:16]


def semantic_multiply_llm(term_a: str, term_b: str) -> str:
    """
    Resolve semantic multiplication using LLM.
    
    Implements semantic multiplication per normative spec:
    "combining the meaning of words into a coherent word or statement that 
    represents the semantic intersection of those words"
    
    Examples:
    - "sufficient" * "reason" = "justification"
    - "precision" * "durability" = "reliability"
    - "probability" * "consequence" = "risk"
    
    Args:
        term_a: First term
        term_b: Second term
        
    Returns:
        Semantic intersection as coherent word/statement
    """
    user_message = f"""
Perform semantic multiplication: "{term_a}" * "{term_b}"

Semantic multiplication means combining the meaning of words into a coherent word or statement that represents the semantic intersection of those words (the meaning when combined together, not just adjoining the terms).

Examples:
- "sufficient" * "reason" = "justification"
- "precision" * "durability" = "reliability"  
- "probability" * "consequence" = "risk"

Find the semantic intersection of "{term_a}" and "{term_b}". Return only the resulting word or short phrase that captures their combined meaning.
"""

    instructions = "You are performing semantic multiplication for the Chirality Framework. Find the semantic intersection of terms, returning only the result word or phrase."
    
    response = call_responses(
        instructions=instructions,
        input=user_message
    )
    result = response.get("output_text", "").strip()
    
    if not result:
        raise ValueError(f"Failed to resolve semantic multiplication: '{term_a}' * '{term_b}'")
    
    return result


def semantic_add_llm(parts: List[str]) -> str:
    """
    Resolve semantic addition using simple concatenation.
    
    Per normative spec: "simply concatenating words or sentence fragments 
    together to form a longer statement"
    
    Example: "faisal" + "has" + "seven" + "balloons" = "faisal has seven balloons"
    
    Args:
        parts: List of parts to concatenate
        
    Returns:
        Concatenated statement
    """
    # Semantic addition is just concatenation - no LLM needed
    return " ".join(parts)


def resolve_semantic_expression(expression: str) -> str:
    """
    Resolve a complete semantic expression with operators.
    
    Handles expressions like: "term1 * term2 + term3 * term4"
    Following order of operations: multiplication first, then addition
    
    Args:
        expression: Expression with * and + operators
        
    Returns:
        Fully resolved semantic result
    """
    # First resolve all multiplication operations
    result = expression
    
    # Find all multiplication patterns
    multiply_pattern = r'"([^"]*?)"\s*\*\s*"([^"]*?)"'
    
    while re.search(multiply_pattern, result):
        match = re.search(multiply_pattern, result)
        if match:
            term_a, term_b = match.groups()
            # Resolve the multiplication
            resolved = semantic_multiply_llm(term_a, term_b)
            # Replace in the expression
            result = result.replace(match.group(0), f'"{resolved}"')
    
    # Now handle addition - split by + and concatenate
    if '+' in result:
        # Extract all quoted terms
        terms = re.findall(r'"([^"]*?)"', result)
        result = semantic_add_llm(terms)
    else:
        # Just remove quotes if single term
        result = re.sub(r'"([^"]*?)"', r'\1', result)
    
    return result.strip()


def apply_lens_llm(text: str, lens: str, station: str) -> str:
    """
    Apply a single lens to text using LLM.
    
    Args:
        text: Content to interpret
        lens: Interpretive lens to apply
        station: Station context
        
    Returns:
        Lensed interpretation
    """
    user_message = f"""
Apply the interpretive lens to the content for the "{station}" station.

Content: "{text}"
Lens: "{lens}"

Interpret the content through this lens to produce a lensed interpretation.
Return a brief but semantically rich statement capturing the essence when viewed through this lens.
Focus on the semantic meaning that emerges from applying the lens perspective to the content.
"""

    instructions = f"You are applying semantic lenses for the {station} station in the Chirality Framework."
    
    response = call_responses(
        instructions=instructions,
        input=user_message
    )
    result = response.get("output_text", "").strip()
    
    if not result:
        raise ValueError(f"Failed to apply lens at station '{station}'")
    
    return result


def apply_matrix_lenses_llm(
    interpreted_matrix: List[List[str]], 
    lenses: List[List[str]], 
    station: str,
    rows: List[str],
    cols: List[str],
    tail: str,
    tracer_tag: str,
    call_json_tail: callable
) -> Dict[str, Any]:
    """
    Apply lenses to all cells in a matrix using JSON injection approach.
    
    Uses colleague_1's approach: inject structured lens + interpreted data,
    let the model apply lenses to corresponding cells and return via tail contract.
    
    Args:
        interpreted_matrix: 2D list of interpreted content from Stage 2
        lenses: 2D list of lenses from catalog
        station: Station name for context
        rows: Row labels for structure
        cols: Column labels for structure  
        tail: JSON tail contract for response format
        tracer_tag: Tracer tag for logging
        
    Returns:
        Dictionary with "elements" key containing lens-interpreted matrix
    """
    import json
    from datetime import datetime, timezone
    
    if len(interpreted_matrix) != len(lenses):
        raise ValueError(f"Matrix and lenses dimensions mismatch: {len(interpreted_matrix)} vs {len(lenses)}")
    
    # Build lens injection JSON
    injection = {
        "artifact": "lenses",
        "station": station,
        "rows": rows,
        "cols": cols,
        "lenses": lenses,
        "catalog_meta": {
            "prompt_hash": _current_prompt_hash(),
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "version": "v1"
        }
    }
    
    # Build interpreted matrix JSON  
    interpreted_json = {
        "rows": rows,
        "cols": cols, 
        "elements": interpreted_matrix
    }
    
    # Create prompt with JSON injection
    preamble = (
        "Apply each lens to the content at the same [row, col]. "
        "Do not alter lens wording. Do not swap lenses across cells. "
        "Return JSON only per the contract.\n\n"
        "LENSES_JSON:\n" + json.dumps(injection, ensure_ascii=False, indent=2)
        + "\n\nINTERPRETED_JSON:\n" + json.dumps(interpreted_json, ensure_ascii=False, indent=2)
    )
    
    # Use the orchestrator's JSON-tail call path for consistent handling
    try:
        result = call_json_tail(preamble, tail, tracer_tag)
        if isinstance(result, dict) and "elements" in result:
            return result
        raise ValueError("Invalid lens application response format")
    except Exception as e:
        # Fallback to cell-by-cell if JSON injection fails
        print(f"Warning: JSON injection failed ({e}), falling back to cell-by-cell")
        fallback_result = []
        for i in range(len(interpreted_matrix)):
            row_result = []
            for j in range(len(interpreted_matrix[i])):
                lensed = apply_lens_llm(interpreted_matrix[i][j], lenses[i][j], station)
                row_result.append(lensed)
            fallback_result.append(row_result)
        return {"elements": fallback_result}