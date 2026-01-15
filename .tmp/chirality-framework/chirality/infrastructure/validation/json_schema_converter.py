#!/usr/bin/env python3
"""
P0-3: Strict Structured Outputs with JSON Schema.

Per colleague_1's specification: "Wherever a stage has a schema, pass 
response_format={"type":"json_schema", "json_schema": <tail>} and validate 
the model's output against that schema in-code."

Converts JSON tail contracts to proper OpenAI JSON Schema format.
"""

import json
import re
from typing import Dict, Any, Optional
from ..prompts.json_tails import get_tail


def convert_contract_to_json_schema(matrix: str, step: str) -> Dict[str, Any]:
    """
    Convert a JSON tail contract to OpenAI JSON Schema format.
    
    Args:
        matrix: Matrix name (e.g., "C")
        step: Step name (e.g., "mechanical")
        
    Returns:
        JSON Schema dict compatible with OpenAI's json_schema format
        
    Raises:
        ValueError: If no contract found for matrix/step
    """
    try:
        tail = get_tail(matrix, step)
    except ValueError as e:
        raise ValueError(f"No JSON tail contract found for {matrix}/{step}: {e}")
    
    # Extract the JSON contract from the tail
    contract_match = re.search(r'\{.*\}', tail)
    if not contract_match:
        raise ValueError(f"No JSON contract found in tail for {matrix}/{step}")
    
    contract_str = contract_match.group()
    
    # Parse the contract to understand its structure
    try:
        # Replace [...] placeholders with null for parsing
        parseable_contract = re.sub(r'\[\.\.\.\]', 'null', contract_str)
        # Replace array patterns like [["..."], ["..."]] with null
        parseable_contract = re.sub(r'\[\[.*?\]\]', 'null', parseable_contract)
        
        contract = json.loads(parseable_contract)
    except json.JSONDecodeError as e:
        raise ValueError(f"Invalid JSON in contract for {matrix}/{step}: {e}")
    
    # Build JSON Schema based on contract structure
    schema = {
        "type": "object",
        "properties": {},
        "required": [],
        "additionalProperties": False
    }
    
    for field, value in contract.items():
        if field == "artifact":
            schema["properties"][field] = {
                "type": "string",
                "enum": ["matrix", "lenses"]
            }
            schema["required"].append(field)
            
        elif field == "name":
            schema["properties"][field] = {
                "type": "string",
                "enum": [matrix]  # Must match expected matrix
            }
            schema["required"].append(field)
            
        elif field == "station":
            schema["properties"][field] = {
                "type": "string"
            }
            schema["required"].append(field)
            
        elif field in ["rows", "cols"]:
            # Extract expected labels from contract
            if isinstance(value, list):
                expected_labels = value
            else:
                # This should be a list in the contract
                expected_labels = []
                
            schema["properties"][field] = {
                "type": "array",
                "items": {"type": "string"},
                "minItems": len(expected_labels) if expected_labels else 1
            }
            
            if expected_labels:
                # Enforce exact labels if specified
                schema["properties"][field]["items"] = {
                    "type": "string",
                    "enum": expected_labels
                }
                schema["properties"][field]["minItems"] = len(expected_labels)
                schema["properties"][field]["maxItems"] = len(expected_labels)
                
            schema["required"].append(field)
            
        elif field == "step":
            schema["properties"][field] = {
                "type": "string",
                "enum": [step]  # Must match expected step
            }
            schema["required"].append(field)
            
        elif field == "op":
            if isinstance(value, str):
                schema["properties"][field] = {
                    "type": "string",
                    "enum": [value]
                }
                schema["required"].append(field)
                
        elif field == "elements":
            # 2D array of strings for matrix elements
            schema["properties"][field] = {
                "type": "array",
                "items": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            }
            schema["required"].append(field)
            
        elif field == "lenses":
            # 2D array of strings for lens elements
            schema["properties"][field] = {
                "type": "array",
                "items": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            }
            schema["required"].append(field)
            
        else:
            # Generic string field
            schema["properties"][field] = {"type": "string"}
            if value != "..." and value is not None:
                schema["required"].append(field)
    
    return schema


def get_response_format_for_stage(matrix: str, step: str) -> Dict[str, Any]:
    """
    Get the response_format parameter for OpenAI API calls.
    
    Per colleague_1's P0-3: Use {"type":"json_schema", "json_schema": <schema>}
    
    Args:
        matrix: Matrix name
        step: Step name
        
    Returns:
        OpenAI response_format dict with strict JSON schema
    """
    try:
        schema = convert_contract_to_json_schema(matrix, step)
        
        return {
            "type": "json_schema",
            "json_schema": {
                "name": f"{matrix}_{step}_response",
                "description": f"Response schema for {matrix} matrix {step} step",
                "schema": schema,
                "strict": True  # Enable strict mode for better compliance
            }
        }
    except ValueError:
        # Fallback to basic JSON object if no specific schema
        return {"type": "json_object"}


def validate_response_against_schema(
    response: Dict[str, Any], 
    matrix: str, 
    step: str
) -> tuple[bool, list[str]]:
    """
    Validate an LLM response against its expected JSON schema.
    
    Args:
        response: Parsed JSON response from LLM
        matrix: Matrix name
        step: Step name
        
    Returns:
        Tuple of (is_valid, list_of_errors)
    """
    try:
        schema = convert_contract_to_json_schema(matrix, step)
        errors = _validate_json_against_schema(response, schema)
        return len(errors) == 0, errors
    except ValueError as e:
        return False, [f"Schema validation failed: {e}"]


def _validate_json_against_schema(data: Dict[str, Any], schema: Dict[str, Any]) -> list[str]:
    """
    Simple JSON Schema validation (subset implementation).
    
    This implements the critical validations without requiring jsonschema library.
    """
    errors = []
    
    if schema.get("type") != "object":
        return ["Schema must be object type"]
    
    # Check required fields
    required = schema.get("required", [])
    for field in required:
        if field not in data:
            errors.append(f"Missing required field: {field}")
    
    # Check additional properties
    if not schema.get("additionalProperties", True):
        allowed_props = set(schema.get("properties", {}).keys())
        actual_props = set(data.keys())
        extra_props = actual_props - allowed_props
        if extra_props:
            errors.append(f"Additional properties not allowed: {list(extra_props)}")
    
    # Validate each property
    properties = schema.get("properties", {})
    for prop, prop_schema in properties.items():
        if prop in data:
            value = data[prop]
            prop_errors = _validate_property(value, prop_schema, prop)
            errors.extend(prop_errors)
    
    return errors


def _validate_property(value: Any, prop_schema: Dict[str, Any], prop_name: str) -> list[str]:
    """Validate a single property against its schema."""
    errors = []
    
    expected_type = prop_schema.get("type")
    
    if expected_type == "string":
        if not isinstance(value, str):
            errors.append(f"{prop_name}: expected string, got {type(value).__name__}")
        elif "enum" in prop_schema:
            if value not in prop_schema["enum"]:
                errors.append(f"{prop_name}: must be one of {prop_schema['enum']}, got '{value}'")
                
    elif expected_type == "array":
        if not isinstance(value, list):
            errors.append(f"{prop_name}: expected array, got {type(value).__name__}")
        else:
            # Validate array items
            items_schema = prop_schema.get("items", {})
            for i, item in enumerate(value):
                item_errors = _validate_property(item, items_schema, f"{prop_name}[{i}]")
                errors.extend(item_errors)
            
            # Validate array length constraints
            min_items = prop_schema.get("minItems")
            max_items = prop_schema.get("maxItems")
            
            if min_items is not None and len(value) < min_items:
                errors.append(f"{prop_name}: must have at least {min_items} items, got {len(value)}")
            
            if max_items is not None and len(value) > max_items:
                errors.append(f"{prop_name}: must have at most {max_items} items, got {len(value)}")
    
    return errors


# Convenience functions for common operations

def get_strict_response_format(matrix: str, step: str) -> Dict[str, Any]:
    """
    Get strict JSON schema response format for a matrix/step combination.
    
    This is the main function to use in API calls per colleague_1's P0-3.
    """
    return get_response_format_for_stage(matrix, step)


def validate_stage_response_strict(
    response: Dict[str, Any], 
    matrix: str, 
    step: str
) -> tuple[bool, list[str]]:
    """
    Validate a stage response with strict JSON schema validation.
    
    This is the main validation function to use per colleague_1's P0-3.
    """
    return validate_response_against_schema(response, matrix, step)