"""
JSON Repair Helper.

Provides repair pass functionality for malformed JSON responses.
Used by both dialogue runner and cell runner.
"""

import json
from typing import Dict, Any, List, Callable, Optional, Tuple


def try_parse_json_or_repair(
    messages: List[Dict[str, str]] = None,
    adapter_call: Callable = None,
    schema_hint: Optional[str] = None,
    max_repair_attempts: int = 1,
    instructions: str = None,
    input_text: str = None,
) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    """
    Attempt to parse JSON, with repair pass if needed.

    Args:
        messages: List of chat messages
        adapter_call: Function to call LLM - returns (response_obj, metadata)
                     where response_obj may be:
                     - a parsed dict OR
                     - {"content": "...json string..."} (legacy/raw)
        schema_hint: Optional schema hint for repair prompt
        max_repair_attempts: Maximum repair attempts

    Returns:
        Tuple of (parsed_json, metadata)

    Raises:
        json.JSONDecodeError: If JSON still invalid after repairs
    """
    
    def _basic_validate(obj: Dict[str, Any], hint: Optional[str]) -> Tuple[bool, str]:
        """
        Minimal contract checks per tail type. Fast, non-exhaustive.
        """
        if not isinstance(obj, dict):
            return False, "not a dict"
        
        art = obj.get("artifact")
        if art == "matrix":
            # Matrices need these keys; 'elements' shape is checked later.
            for k in ("name", "station", "rows", "cols", "step", "elements"):
                if k not in obj:
                    return False, f"missing key '{k}'"
            return True, ""
        
        if art == "lenses":
            for k in ("station", "rows", "cols", "lenses"):
                if k not in obj:
                    return False, f"missing key '{k}'"
            return True, ""
        
        if art == "lens_catalog":
            for k in ("station", "rows", "cols", "lenses"):
                if k not in obj:
                    return False, f"missing key '{k}'"
            # For lens_catalog, we could add station-specific validation here if needed
            return True, ""
        
        if art == "aggregated_output":
            for k in ("matrices", "meta"):
                if k not in obj:
                    return False, f"missing key '{k}'"
            return True, ""
        
        # Unknown artifact: require at least something structured
        if art is None:
            return False, "missing 'artifact' key"
        
        return False, f"unknown artifact '{art}'"

    # First attempt - support both formats
    if instructions is not None and input_text is not None:
        # New format: use instructions + input
        response, metadata = adapter_call(instructions=instructions, input=input_text)
    else:
        # Legacy format: use messages
        response, metadata = adapter_call(messages)

    # If adapter already returned parsed JSON, validate before accepting.
    if isinstance(response, dict) and "content" not in response and "text" not in response:
        ok, why = _basic_validate(response, schema_hint)
        if ok:
            return response, metadata
        # Fall through to repair attempts using the same messages + a repair cue.
        print(f"DEBUG: Parsed JSON failed basic validation ({why}); attempting repair...")

    # Try to parse response
    content = response.get("content", response.get("text", ""))

    try:
        parsed = json.loads(content)
        ok, why = _basic_validate(parsed, schema_hint)
        if ok:
            return parsed, metadata
        # If structure wrong, proceed to repair below
        print(f"DEBUG: Parsed JSON failed basic validation ({why}); attempting repair...")
    except json.JSONDecodeError:
        pass
        
    # Original parse failed or validation failed, try repair
    for attempt in range(max_repair_attempts):
        # Create repair message
        repair_content = (
            "The previous output was invalid JSON. "
            "Return valid JSON only"
            + (f" matching this shape: {schema_hint}" if schema_hint else "")
            + "."
        )
        if schema_hint:
            repair_content += f"\nEnsure the JSON contains the required keys for '{schema_hint}'."

        repair_message = {"role": "user", "content": repair_content}

        # P0-4: EPHEMERAL repair - never append to transcript (per colleague_1's specification)
        if instructions is not None and input_text is not None:
            # Use separate ephemeral repair call that doesn't pollute transcript
            response, metadata = _ephemeral_repair_call(
                instructions, input_text, content, repair_content, adapter_call
            )
        else:
            # Legacy format: use messages (also ephemeral)
            response, metadata = _ephemeral_repair_call_legacy(
                messages, content, repair_message, adapter_call
            )
        
        # Handle parsed dict response
        if isinstance(response, dict) and "content" not in response and "text" not in response:
            ok, why = _basic_validate(response, schema_hint)
            if ok:
                return response, metadata
            content = json.dumps(response)  # For error reporting
        else:
            content = response.get("content", response.get("text", ""))
            try:
                parsed = json.loads(content)
                ok, why = _basic_validate(parsed, schema_hint)
                if ok:
                    return parsed, metadata
            except json.JSONDecodeError:
                pass

        if attempt == max_repair_attempts - 1:
            # Last attempt failed, raise with helpful error
            raise json.JSONDecodeError(
                f"JSON repair failed after {max_repair_attempts} attempts. "
                f"Final response: {content[:200]}...",
                content,
                0,
            )

    # Should not reach here, but handle gracefully
    raise json.JSONDecodeError(f"Unexpected error parsing JSON: {content}", content, 0)


def create_schema_hint(expected_fields: Dict[str, str]) -> str:
    """
    Create schema hint string from expected fields.

    Args:
        expected_fields: Dict mapping field names to types

    Returns:
        Schema hint string
    """
    field_hints = []
    for field, field_type in expected_fields.items():
        if field_type.startswith("list"):
            field_hints.append(f'"{field}": [...]')
        elif field_type.startswith("dict"):
            field_hints.append(f'"{field}": {{...}}')
        else:
            field_hints.append(f'"{field}": "..."')

    return "{" + ", ".join(field_hints) + "}"


def create_matrix_schema_hint(matrix_name: str, step: str) -> str:
    """
    Create schema hint for matrix responses.

    Args:
        matrix_name: Matrix name (C, F, D, etc.)
        step: Step type (mechanical, interpreted, lensed, etc.)

    Returns:
        Schema hint string
    """
    base_hint = {
        "artifact": "matrix",
        "name": matrix_name,
        "station": "...",
        "rows": "[...]",
        "cols": "[...]",
        "step": step,
    }

    if step == "lenses":
        base_hint["lenses"] = "[[...]]"
    else:
        base_hint["elements"] = "[[...]]"
        if step not in ["base", "transpose"]:
            base_hint["op"] = "..."

    return create_schema_hint({k: str(v) for k, v in base_hint.items()})


def create_tensor_cell_schema_hint(tensor_name: str) -> str:
    """
    Create schema hint for tensor cell responses.

    Args:
        tensor_name: Tensor name (M, W, U, N)

    Returns:
        Schema hint string
    """
    return create_schema_hint({"tensor": tensor_name, "value": "string", "confidence": "0.0-1.0"})


def _ephemeral_repair_call(
    instructions: str, 
    original_input: str, 
    failed_content: str, 
    repair_content: str, 
    adapter_call: Callable
) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    """
    P0-4: Make an ephemeral repair call that doesn't pollute the transcript.
    
    Per colleague_1's specification: "Keep JSON repair prompts ephemeral (not appended 
    to history), and confirm via a test that the transcript remains metadata-free 
    even when repair is triggered."
    
    This creates a completely separate, temporary conversation for repair that never
    affects the main transcript.
    
    Args:
        instructions: System instructions for repair
        original_input: Original input that failed
        failed_content: The content that failed to parse
        repair_content: Repair instructions  
        adapter_call: Function to call LLM
        
    Returns:
        Tuple of (response, metadata) from repair attempt
    """
    # Create ephemeral repair context (separate from main transcript)
    ephemeral_input = (
        f"PREVIOUS ATTEMPT:\n{failed_content}\n\n"
        f"REPAIR INSTRUCTIONS:\n{repair_content}\n\n"
        f"ORIGINAL CONTEXT (for reference only):\n{original_input}"
    )
    
    # Make ephemeral call with clear repair instructions
    repair_instructions = (
        f"{instructions}\n\n"
        f"REPAIR MODE: The previous response was invalid JSON. "
        f"Provide ONLY valid JSON that meets the requirements. "
        f"Do not include any explanations or markdown."
    )
    
    return adapter_call(instructions=repair_instructions, input=ephemeral_input)


def _ephemeral_repair_call_legacy(
    original_messages: List[Dict[str, str]], 
    failed_content: str, 
    repair_message: Dict[str, str], 
    adapter_call: Callable
) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    """
    P0-4: Legacy ephemeral repair call for messages format.
    
    Creates a temporary repair conversation without affecting original messages.
    
    Args:
        original_messages: Original message history
        failed_content: Content that failed to parse
        repair_message: Repair instruction message
        adapter_call: Function to call LLM
        
    Returns:
        Tuple of (response, metadata) from repair attempt
    """
    # Create ephemeral messages list (separate from original)
    ephemeral_messages = [
        {"role": "system", "content": "You are a JSON repair assistant. Provide only valid JSON."},
        {"role": "user", "content": f"Previous invalid response: {failed_content}"},
        repair_message,
        {"role": "user", "content": "Return ONLY valid JSON, no explanations."}
    ]
    
    return adapter_call(ephemeral_messages)
