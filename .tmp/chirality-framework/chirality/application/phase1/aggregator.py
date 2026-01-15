"""
Phase 1 Aggregator.

Validates and writes the final Phase 1 output with strict schema validation.
"""

from pathlib import Path
from typing import Dict, Any

from .contracts import Phase1Output


def validate_and_write_agg(agg_dict: Dict[str, Any], out_dir: str) -> Phase1Output:
    """
    Validate aggregator dict and write phase1_output.json.

    Args:
        agg_dict: Raw aggregator dictionary from dialogue
        out_dir: Output directory path

    Returns:
        Validated Phase1Output model

    Raises:
        ValidationError: If aggregator doesn't match schema
    """
    # Validate with Pydantic
    validated_output = Phase1Output.model_validate(agg_dict)

    # Ensure output directory exists
    Path(out_dir).mkdir(parents=True, exist_ok=True)

    # Write validated JSON
    output_path = Path(out_dir) / "phase1_output.json"
    with open(output_path, "w") as f:
        f.write(validated_output.model_dump_json(indent=2))

    return validated_output


def create_aggregator_schema_hint() -> str:
    """
    Create schema hint for aggregator repair prompts.

    Returns:
        JSON schema hint string
    """
    return """{
    "matrices": {
        "C": {"name": "C", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "lensed", "op": "dot"},
        "J": {"name": "J", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "base"},
        "F": {"name": "F", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "lensed", "op": "hadamard"},
        "D": {"name": "D", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "lensed", "op": "add"},
        "K": {"name": "K", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "transpose", "op": "transpose"},
        "X": {"name": "X", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "lensed", "op": "dot"},
        "Z": {"name": "Z", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "shifted", "op": "shift"},
        "G": {"name": "G", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "base"},
        "P": {"name": "P", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "base"},
        "T": {"name": "T", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "transpose", "op": "transpose"},
        "E": {"name": "E", "station": "...", "rows": [...], "cols": [...], "elements": [[...]], "step": "lensed", "op": "dot"}
    },
    "principles": {
        "from": "Z",
        "items": [...]
    }
}"""
