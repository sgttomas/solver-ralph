"""
Explicit lens API for Phase 1 semantic operations.

Implements cell-wise lens generation and application using the normative formula:
[station_meaning * row_name * column_name *]

This is the semantic-first approach - we prompt the LLM to perform 
semantic operations rather than implementing them in Python logic.
"""

from typing import List
from ...infrastructure.llm.openai_adapter import call_responses
from ...infrastructure.prompts.registry import get_registry


def generate_lens(station: str, row_name: str, col_name: str) -> str:
    """
    Generate a semantic lens using the normative formula:
    [station_meaning * row_name * column_name *]
    
    This prompts the LLM to create a semantic intersection of the station,
    row ontology, and column ontology to form an interpretive lens.
    
    Args:
        station: Station name (e.g., "Problem Statement", "Requirements", etc.)
        row_name: Row ontology name (e.g., "normative", "data", etc.)
        col_name: Column ontology name (e.g., "necessity (vs contingency)", "guiding", etc.)
    
    Returns:
        A semantic lens as a single, coherent statement
    """
    # Load system context for semantic operations
    registry = get_registry()
    
    # Create the prompt for lens generation
    user_message = f"""
Generate a semantic lens using the formula: [station_meaning * row_name * column_name *]

Station: "{station}"
Row ontology: "{row_name}" 
Column ontology: "{col_name}"

Apply semantic multiplication to find the intersection of these three concepts:
1. The meaning of "{station}" in the context of knowledge work
2. The ontological meaning of "{row_name}"  
3. The ontological meaning of "{col_name}"

Return exactly one interpretive lens statement that captures the essence of this semantic intersection. The lens should be concise, semantically rich, and suitable for interpreting content through this combined perspective.

Do not include ontological identifiers or explanations - just the semantic meaning at the nexus of these aspects.
"""

    # Call LLM to generate lens using Responses API
    instructions = "You are generating semantic lenses for the Chirality Framework using semantic multiplication to find the intersection of ontological meanings."
    
    response = call_responses(
        instructions=instructions,
        input=user_message
    )
    
    # Extract the lens from response
    lens = response.get("output_text", "").strip()
    
    if not lens:
        raise ValueError(f"Failed to generate lens for station='{station}', row='{row_name}', col='{col_name}'")
    
    return lens


def apply_lens(content: str, lens: str) -> str:
    """
    Apply a semantic lens to interpret content through that lens.
    
    This prompts the LLM to reinterpret the given content through the 
    perspective provided by the lens, producing a lensed interpretation.
    
    Args:
        content: The content to be interpreted (semantic result from operations)
        lens: The interpretive lens to apply
        
    Returns:
        The lensed interpretation as a coherent statement
    """
    user_message = f"""
Apply the following interpretive lens to the given content:

Content to interpret: "{content}"

Interpretive lens: "{lens}"

Interpret the content through this lens to produce a lensed interpretation. Your output should be a brief but semantically rich statement that captures the essence of the content when viewed through this lens.

Focus on the semantic meaning that emerges from applying the lens perspective to the content. Do not include lens labels or meta-commentary - just the interpreted meaning.
"""

    # Call LLM to apply lens using Responses API
    instructions = "You are applying semantic lenses in the Chirality Framework to interpret content through specific ontological perspectives."
    
    response = call_responses(
        instructions=instructions,
        input=user_message
    )
    
    # Extract the lensed interpretation
    lensed = response.get("output_text", "").strip()
    
    if not lensed:
        raise ValueError(f"Failed to apply lens to content: '{content[:50]}...'")
    
    return lensed


def generate_matrix_lenses(station: str, row_labels: List[str], col_labels: List[str]) -> List[List[str]]:
    """
    Generate lenses for all cells in a matrix using the explicit lens API.
    
    Args:
        station: Station name for the matrix
        row_labels: List of row ontology labels
        col_labels: List of column ontology labels
        
    Returns:
        2D list of lenses, one for each matrix cell
    """
    lenses = []
    
    for row_name in row_labels:
        row_lenses = []
        for col_name in col_labels:
            lens = generate_lens(station, row_name, col_name)
            row_lenses.append(lens)
        lenses.append(row_lenses)
    
    return lenses


def apply_matrix_lenses(
    content_matrix: List[List[str]], 
    lens_matrix: List[List[str]], 
    station: str,
    rows: List[str],
    cols: List[str],
    tail: str,
    tracer_tag: str,
    call_json_tail: callable
) -> List[List[str]]:
    """
    Apply lenses to all cells in a content matrix using JSON injection approach.
    
    This is a domain wrapper that delegates to infrastructure for structured LLM processing.
    
    Args:
        content_matrix: 2D list of content to be lensed
        lens_matrix: 2D list of corresponding lenses
        station: Station name for context
        rows: Row labels for structure
        cols: Column labels for structure  
        tail: JSON tail contract for response format
        tracer_tag: Tracer tag for logging
        
    Returns:
        2D list of lensed interpretations
    """
    # Import here to avoid circular dependency
    from ...infrastructure.semantics.resolvers import apply_matrix_lenses_llm
    
    result = apply_matrix_lenses_llm(
        content_matrix, lens_matrix, station, rows, cols, tail, tracer_tag, call_json_tail
    )
    return result["elements"]