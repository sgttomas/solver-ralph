"""
D2-5: Schema validation for stage responses and lens payloads.

Per colleague_1's specification, this module provides:
- Stage response validation against JSON tail schemas
- Lens payload validation for canonical BEGIN/END blocks
- Comprehensive error reporting with specific field validation
"""

import json
import re
from typing import Dict, Any, List, Optional, Tuple
from datetime import datetime

from ...domain.matrices.canonical import get_matrix_info
from ..prompts.json_tails import get_tail


class SchemaValidationError(Exception):
    """Raised when schema validation fails."""
    pass


class StageResponseValidator:
    """
    D2-5: Validates stage responses against expected JSON schemas.
    
    Uses the JSON tail specifications to validate that LLM responses
    conform to the expected structure for each matrix/step combination.
    """
    
    def __init__(self):
        # Extract schema patterns from JSON tails
        self.schema_patterns = self._build_schema_patterns()
    
    def _build_schema_patterns(self) -> Dict[Tuple[str, str], Dict[str, Any]]:
        """Build validation patterns from JSON tail specifications."""
        patterns = {}
        
        # Matrix/step combinations to validate
        matrix_steps = [
            ("C", "mechanical"), ("C", "interpreted"), ("C", "lensed"),
            ("F", "mechanical"), ("F", "interpreted"), ("F", "lensed"),
            ("D", "mechanical"), ("D", "interpreted"), ("D", "lensed"),
            ("X", "mechanical"), ("X", "interpreted"), ("X", "lensed"),
            ("Z", "lensed"), ("E", "mechanical"), ("E", "interpreted"), ("E", "lensed")
        ]
        
        for matrix, step in matrix_steps:
            try:
                tail = get_tail(matrix, step)
                schema = self._extract_schema_from_tail(tail, matrix, step)
                patterns[(matrix, step)] = schema
            except ValueError:
                # Skip if no tail defined
                continue
        
        return patterns
    
    def _extract_schema_from_tail(self, tail: str, matrix: str, step: str) -> Dict[str, Any]:
        """Extract validation schema from JSON tail string."""
        # Parse the JSON contract from the tail - handle the [...] placeholders
        match = re.search(r'\{.*\}', tail)
        if not match:
            raise ValueError(f"No JSON found in tail for {matrix}/{step}")
        
        # Replace [...] placeholders with valid JSON for parsing
        json_str = match.group()
        json_str = re.sub(r'\[\.\.\.\]', '["placeholder"]', json_str)
        json_str = re.sub(r'\[(\[.*?\])(,\[.*?\])*\]', r'[["placeholder"]]', json_str)
        
        try:
            contract = json.loads(json_str)
        except json.JSONDecodeError as e:
            # If still can't parse, create a minimal contract
            contract = {
                "artifact": "matrix",
                "name": matrix,
                "step": step
            }
        
        # Get expected dimensions from canonical matrix
        try:
            matrix_info = get_matrix_info(matrix)
            expected_rows = len(matrix_info["row_labels"])
            expected_cols = len(matrix_info["col_labels"])
        except:
            expected_rows = None
            expected_cols = None
        
        return {
            "contract": contract,
            "expected_rows": expected_rows,
            "expected_cols": expected_cols,
            "matrix": matrix,
            "step": step
        }
    
    def validate_stage_response(self, response: Dict[str, Any], matrix: str, step: str) -> List[str]:
        """
        Validate a stage response against its expected schema.
        
        Args:
            response: The parsed JSON response from LLM
            matrix: Matrix name (e.g., "C")
            step: Step name (e.g., "mechanical")
            
        Returns:
            List of validation errors (empty if valid)
        """
        errors = []
        key = (matrix, step)
        
        if key not in self.schema_patterns:
            errors.append(f"No schema pattern defined for {matrix}/{step}")
            return errors
        
        schema = self.schema_patterns[key]
        contract = schema["contract"]
        
        # Validate required top-level fields
        for field, expected_value in contract.items():
            if field == "elements" or field == "lenses":
                # Skip array validation here, handle separately
                continue
                
            if field not in response:
                errors.append(f"Missing required field: {field}")
                continue
                
            if isinstance(expected_value, str) and expected_value != "...":
                if response[field] != expected_value:
                    errors.append(f"Field {field}: expected '{expected_value}', got '{response[field]}'")
        
        # Validate matrix dimensions if this is a matrix response
        if "elements" in contract:
            errors.extend(self._validate_elements_array(response, schema))
        elif "lenses" in contract:
            errors.extend(self._validate_lenses_array(response, schema))
        
        return errors
    
    def _validate_elements_array(self, response: Dict[str, Any], schema: Dict[str, Any]) -> List[str]:
        """Validate the elements array structure."""
        errors = []
        
        if "elements" not in response:
            errors.append("Missing 'elements' field")
            return errors
        
        elements = response["elements"]
        if not isinstance(elements, list):
            errors.append("'elements' must be a list")
            return errors
        
        expected_rows = schema["expected_rows"]
        expected_cols = schema["expected_cols"]
        
        if expected_rows and len(elements) != expected_rows:
            errors.append(f"elements: expected {expected_rows} rows, got {len(elements)}")
        
        for i, row in enumerate(elements):
            if not isinstance(row, list):
                errors.append(f"elements[{i}]: must be a list")
                continue
                
            if expected_cols and len(row) != expected_cols:
                errors.append(f"elements[{i}]: expected {expected_cols} columns, got {len(row)}")
            
            for j, cell in enumerate(row):
                if not isinstance(cell, str):
                    errors.append(f"elements[{i}][{j}]: must be a string, got {type(cell)}")
        
        return errors
    
    def _validate_lenses_array(self, response: Dict[str, Any], schema: Dict[str, Any]) -> List[str]:
        """Validate the lenses array structure."""
        errors = []
        
        if "lenses" not in response:
            errors.append("Missing 'lenses' field")
            return errors
        
        lenses = response["lenses"]
        if not isinstance(lenses, list):
            errors.append("'lenses' must be a list")
            return errors
        
        expected_rows = schema["expected_rows"]
        expected_cols = schema["expected_cols"]
        
        if expected_rows and len(lenses) != expected_rows:
            errors.append(f"lenses: expected {expected_rows} rows, got {len(lenses)}")
        
        for i, row in enumerate(lenses):
            if not isinstance(row, list):
                errors.append(f"lenses[{i}]: must be a list")
                continue
                
            if expected_cols and len(row) != expected_cols:
                errors.append(f"lenses[{i}]: expected {expected_cols} columns, got {len(row)}")
            
            for j, lens in enumerate(row):
                if not isinstance(lens, str):
                    errors.append(f"lenses[{i}][{j}]: must be a string, got {type(lens)}")
        
        return errors


class LensPayloadValidator:
    """
    D2-5: Validates lens payloads in canonical BEGIN/END blocks.
    
    Validates the clean lens injection format (metadata-free):
    <<<BEGIN LENS MATRIX>>>
    rows: ["normative", "operative", "iterative"]
    cols: ["necessity (vs contingency)", "sufficiency", "completeness", "consistency"]
    lenses_json: [[...], [...], [...]]
    <<<END LENS MATRIX>>>
    """
    
    def validate_lens_block(self, lens_block: str) -> List[str]:
        """
        Validate a complete lens block string.
        
        Args:
            lens_block: The complete lens block text
            
        Returns:
            List of validation errors (empty if valid)
        """
        errors = []
        
        # Check for canonical markers
        if not lens_block.startswith("<<<BEGIN LENS MATRIX>>>"):
            errors.append("Lens block must start with '<<<BEGIN LENS MATRIX>>>'")
        
        if not lens_block.endswith("<<<END LENS MATRIX>>>"):
            errors.append("Lens block must end with '<<<END LENS MATRIX>>>'")
        
        # Extract content between markers
        content_match = re.search(
            r"<<<BEGIN LENS MATRIX>>>\s*(.*?)\s*<<<END LENS MATRIX>>>", 
            lens_block, 
            re.DOTALL
        )
        
        if not content_match:
            errors.append("Could not extract content from lens block")
            return errors
        
        content = content_match.group(1)
        
        # Parse the lens block content
        try:
            lens_data = self._parse_lens_content(content)
            errors.extend(self._validate_lens_data(lens_data))
        except Exception as e:
            errors.append(f"Failed to parse lens block content: {e}")
        
        return errors
    
    def _parse_lens_content(self, content: str) -> Dict[str, Any]:
        """Parse the lens block content into structured data."""
        lines = content.strip().split('\n')
        data = {}
        meta_section = False
        meta_data = {}
        
        for line in lines:
            line = line.strip()
            if not line:
                continue
                
            if line == "meta:":
                meta_section = True
                continue
            
            if meta_section:
                if line.startswith("  "):
                    # Meta field
                    key, value = line.strip().split(":", 1)
                    meta_data[key.strip()] = value.strip()
                else:
                    # End of meta section
                    meta_section = False
                    data["meta"] = meta_data
                    # Continue processing this line as normal field
            
            if not meta_section and ":" in line:
                key, value = line.split(":", 1)
                key = key.strip()
                value = value.strip()
                
                # Parse JSON values
                if key in ["rows", "cols", "lenses_json"]:
                    try:
                        data[key] = json.loads(value)
                    except json.JSONDecodeError:
                        data[key] = value  # Keep as string if not valid JSON
                else:
                    data[key] = value
        
        # Add final meta if we were still in meta section
        if meta_section:
            data["meta"] = meta_data
        
        return data
    
    def _validate_lens_data(self, data: Dict[str, Any]) -> List[str]:
        """Validate the parsed lens data structure."""
        errors = []
        
        # Required top-level fields (clean format - no metadata)
        required_fields = ["rows", "cols", "lenses_json"]
        for field in required_fields:
            if field not in data:
                errors.append(f"Missing required field: {field}")
        
        # Validate matrix name
        if "matrix" in data:
            matrix = data["matrix"]
            try:
                get_matrix_info(matrix)
            except ValueError:
                errors.append(f"Unknown matrix: {matrix}")
        
        # Validate rows/cols are lists
        for field in ["rows", "cols"]:
            if field in data:
                value = data[field]
                if not isinstance(value, list):
                    errors.append(f"{field} must be a list")
                elif not all(isinstance(item, str) for item in value):
                    errors.append(f"{field} must be a list of strings")
        
        # Validate lenses_json structure
        if "lenses_json" in data:
            lenses = data["lenses_json"]
            if not isinstance(lenses, list):
                errors.append("lenses_json must be a list")
            else:
                for i, row in enumerate(lenses):
                    if not isinstance(row, list):
                        errors.append(f"lenses_json[{i}] must be a list")
                    else:
                        for j, lens in enumerate(row):
                            if not isinstance(lens, str):
                                errors.append(f"lenses_json[{i}][{j}] must be a string")
        
        # Validate dimensions consistency
        if all(field in data for field in ["rows", "cols", "lenses_json"]):
            expected_rows = len(data["rows"])
            expected_cols = len(data["cols"])
            actual_rows = len(data["lenses_json"])
            
            if actual_rows != expected_rows:
                errors.append(f"lenses_json rows mismatch: expected {expected_rows}, got {actual_rows}")
            
            for i, row in enumerate(data["lenses_json"]):
                if isinstance(row, list) and len(row) != expected_cols:
                    errors.append(f"lenses_json[{i}] cols mismatch: expected {expected_cols}, got {len(row)}")
        
        # Validate source
        if "source" in data:
            valid_sources = ["catalog", "auto", "override"]
            if data["source"] not in valid_sources:
                errors.append(f"Invalid source: {data['source']}. Must be one of {valid_sources}")
        
        # Validate meta section (optional but if present, should be well-formed)
        if "meta" in data:
            meta = data["meta"]
            if not isinstance(meta, dict):
                errors.append("meta must be a dictionary")
            else:
                # Check that meta fields are strings if present
                for field, value in meta.items():
                    if not isinstance(value, str):
                        errors.append(f"meta.{field} must be a string")
        
        return errors


def validate_stage_response(response: Dict[str, Any], matrix: str, step: str) -> List[str]:
    """
    D2-5: Convenience function to validate a stage response.
    
    Args:
        response: Parsed JSON response from LLM
        matrix: Matrix name
        step: Step name
        
    Returns:
        List of validation errors
    """
    validator = StageResponseValidator()
    return validator.validate_stage_response(response, matrix, step)


def validate_lens_payload(lens_block: str) -> List[str]:
    """
    D2-5: Convenience function to validate a lens payload.
    
    Args:
        lens_block: Complete lens block string
        
    Returns:
        List of validation errors
    """
    validator = LensPayloadValidator()
    return validator.validate_lens_block(lens_block)