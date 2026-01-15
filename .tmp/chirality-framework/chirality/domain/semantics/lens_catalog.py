"""
Lens catalog generation for Phase 1.

Generates the complete lens catalog upfront using the Phase 1 system prompt,
creating a canonized set of lenses that can be analyzed for stable "attractors"
over time.
"""

import json
import hashlib
from typing import Dict, List, Callable, Any
from datetime import datetime, timezone

from ...infrastructure.prompts.registry import get_registry

# Station-specific schemas (single source of truth)
STATION_SCHEMAS = {
    "Problem Statement": {"rows": ["normative","operative","iterative"], 
                         "cols": ["necessity (vs contingency)","sufficiency","completeness","consistency"]},
    "Requirements":      {"rows": ["normative","operative","iterative"],
                         "cols": ["necessity (vs contingency)","sufficiency","completeness","consistency"]},
    "Objectives":        {"rows": ["normative","operative","iterative"],
                         "cols": ["guiding","applying","judging","reflecting"]},
    "Verification":      {"rows": ["guiding","applying","judging","reflecting"],
                         "cols": ["necessity (vs contingency)","sufficiency","completeness","consistency"]},
    "Validation":        {"rows": ["guiding","applying","judging","reflecting"],
                         "cols": ["necessity (vs contingency)","sufficiency","completeness","consistency"]},
    "Evaluation":        {"rows": ["guiding","applying","judging"],
                         "cols": ["data","information","knowledge"]},
}


def generate_lens_catalog(
    stations: List[str],
    call_llm: Callable[[str, str, str], Dict[str, Any]]
) -> tuple[Dict[str, List[List[str]]], Dict[str, Any]]:
    """
    Generate complete lens catalog for all stations using Phase 1 system prompt.
    
    This creates a canonized lens catalog that can be reused across runs and
    analyzed for stable semantic "attractors" over time.
    
    Args:
        stations: List of station names 
        call_llm: Function to call LLM with (message, tail, operation_id)
        
    Returns:
        Tuple of (catalog, metadata) where catalog is {station: [[lens_strings]]}
    """
    
    catalog = {}
    for station in stations:
        if station not in STATION_SCHEMAS:
            raise ValueError(f"Unknown station schema: {station}")
        schema = STATION_SCHEMAS[station]
        rows, cols = schema["rows"], schema["cols"]
        catalog[station] = [[None] * len(cols) for _ in rows]
    
    for station in stations:
        # Get station-specific schema
        schema = STATION_SCHEMAS[station]
        station_rows, station_cols = schema["rows"], schema["cols"]
        
        # Generate lenses for one station at a time for reliability
        message = _build_catalog_prompt_for_station(station, station_rows, station_cols)
        tail = _get_lens_catalog_tail(station, station_rows, station_cols)
        
        response = call_llm(message, tail, f"{station.lower().replace(' ', '_')}_lens_catalog")
        
        # Extract lenses from response
        if "lenses" in response and len(response["lenses"]) == len(station_rows):
            lenses = response["lenses"]
            
            # Handle case where LLM returns triple-nested instead of double-nested array
            if (len(lenses) > 0 and isinstance(lenses[0], list) and len(lenses[0]) > 0 
                and isinstance(lenses[0][0], list) and len(lenses[0][0]) > 0
                and isinstance(lenses[0][0][0], str)):
                # Triple-nested: [[[str]]] -> flatten to [[str]]
                lenses = [row[0] if isinstance(row[0], list) else row for row in lenses]
            
            catalog[station] = lenses
        else:
            raise ValueError(f"Invalid lens catalog response for station '{station}': {response}")
    
    # Generate metadata for tracking and canonization
    # Get system prompt hash (requires registry access)
    try:
        from ...infrastructure.prompts.registry import get_registry
        registry = get_registry()
        system_prompt = registry.get_text("system")
        prompt_hash = _hash_text(system_prompt)
    except:
        prompt_hash = "unknown"
    
    meta = {
        "version": "v2",
        "prompt_hash": prompt_hash,
        "stations": stations,
        "schemas": STATION_SCHEMAS,  # Include full schemas for validation
        "generated_at": datetime.now(timezone.utc).isoformat(),
    }
    
    return catalog, meta


def _build_catalog_prompt_for_station(station: str, rows: List[str], cols: List[str]) -> str:
    """Build prompt for generating lenses for one station."""
    return f"""
Generate interpretive lenses for the "{station}" station using the normative formula:
[station_meaning * row_name * column_name]

Station: "{station}"

Row ontologies: {rows}
Column ontologies: {cols}

For each cell position, create a lens by finding the semantic intersection of:
1. The meaning of "{station}" station in knowledge work context
2. The row ontology meaning  
3. The column ontology meaning

Generate exactly {len(rows)} rows × {len(cols)} columns = {len(rows) * len(cols)} distinct lenses.

Each lens should be:
- A concise, semantically rich statement
- Distinct from other lenses (no duplicates)
- Suitable for interpreting content through this combined perspective
- Free of ontological identifiers (just the semantic meaning)

Return the complete {len(rows)}×{len(cols)} lens matrix for the "{station}" station.
"""


def _get_lens_catalog_tail(station: str, rows: List[str], cols: List[str]) -> str:
    """Get JSON tail for lens catalog generation."""
    rows_json = json.dumps(rows)
    cols_json = json.dumps(cols)
    lenses_template = "[[" + ",".join(['"..."'] * len(cols)) + "]]" * len(rows)
    lenses_template = lenses_template.replace("]][[", "],[")
    lenses_template = f"[{lenses_template}]"
    
    return f'''Return JSON only using this contract: {{"artifact":"lens_catalog","station":"{station}","rows":{rows_json},"cols":{cols_json},"lenses":{lenses_template}}}'''


def _hash_text(text: str) -> str:
    """Generate SHA256 hash of text for tracking changes."""
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def validate_lens_catalog(catalog: Dict[str, List[List[str]]], stations: List[str]) -> List[str]:
    """
    Validate lens catalog structure and content using station-specific schemas.
    
    Returns:
        List of validation errors (empty if valid)
    """
    
    errors = []
    
    # Check structure
    if set(catalog.keys()) != set(stations):
        errors.append(f"Catalog stations {set(catalog.keys())} don't match expected {set(stations)}")
    
    for station, station_lenses in catalog.items():
        if station not in STATION_SCHEMAS:
            errors.append(f"Unknown station: {station}")
            continue
            
        schema = STATION_SCHEMAS[station]
        expected_rows, expected_cols = schema["rows"], schema["cols"]
        
        if len(station_lenses) != len(expected_rows):
            errors.append(f"Station '{station}': expected {len(expected_rows)} rows, got {len(station_lenses)}")
            continue
            
        # Collect all lenses for this station
        flat_lenses = []
        
        for i, row_lenses in enumerate(station_lenses):
            if len(row_lenses) != len(expected_cols):
                errors.append(f"Station '{station}' row {i}: expected {len(expected_cols)} columns, got {len(row_lenses)}")
                continue
                
            for j, lens in enumerate(row_lenses):
                # Check for empty strings
                if not lens or not lens.strip():
                    errors.append(f"Station '{station}' cell [{i},{j}]: empty lens")
                else:
                    flat_lenses.append(lens.strip())
        
        # Check uniqueness within station
        if flat_lenses:
            unique_count = len(set(flat_lenses))
            expected_count = len(expected_rows) * len(expected_cols)
            
            # Require at least 10 unique lenses per station (or 80% if less than 12 cells)
            min_unique = min(10, int(expected_count * 0.8))
            if unique_count < min_unique:
                errors.append(
                    f"Station '{station}': insufficient uniqueness - only {unique_count} unique lenses "
                    f"(minimum {min_unique} required for {expected_count} cells)"
                )
    
    return errors


# New spec-driven lens generation utilities

def _extract_spec_block(full: str,
                        start: str = "<!-- LENS_GEN:BEGIN -->",
                        end: str = "<!-- LENS_GEN:END -->") -> str:
    """Return the lens-generation block if markers exist; otherwise the whole spec."""
    i, j = full.find(start), full.find(end)
    if i != -1 and j != -1 and j > i:
        return full[i + len(start): j].strip()
    return full


def prompt_hash_for_lenses() -> str:
    """SHA-256 of the normative spec lens block (for provenance)."""
    spec = get_registry().get_text("normative_spec")
    block = _extract_spec_block(spec)
    return hashlib.sha256(block.encode("utf-8")).hexdigest()


def generate_lens_matrix_llm(*,
                             station: str,
                             rows: List[str],
                             cols: List[str],
                             call_json_tail: Callable[[str, str, str], Dict]) -> List[List[str]]:
    """
    Generate a complete lenses matrix for a station using normative_spec.txt.
    The prompt text comes from the spec (with variable substitution).
    """
    spec = get_registry().get_text("normative_spec")
    block = _extract_spec_block(spec)
    templ = (block
             .replace("{{STATION}}", station)
             .replace("{{ROWS_LINE}}", ", ".join(rows))
             .replace("{{COLS_LINE}}", ", ".join(cols)))

    request_json = {
        "artifact": "lens_request",
        "station": station,
        "rows": rows,
        "cols": cols,
        "constraints": {
            "rows_count": len(rows),
            "cols_count": len(cols),
            "max_words_per_lens": 15,
            "distinct": True
        }
    }

    preamble = (
        templ + "\n\n"
        "REQUEST_JSON:\n" + json.dumps(request_json, ensure_ascii=False)
    )

    result = call_json_tail(
        preamble,
        TAIL_LENSES_GENERATE,
        f"lenses_gen_{station.lower().replace(' ', '_')}"
    )
    lenses = result["lenses"]

    # Handle triple-nesting and validate shape + content
    def _flatten_if_needed(lenses_raw):
        """Flatten triple-nested arrays if LLM returns [[[str]]] instead of [[str]]."""
        if not lenses_raw or not isinstance(lenses_raw, list):
            return lenses_raw
            
        # Check if triple-nested: [[[str]]] pattern
        if (len(lenses_raw) > 0 and isinstance(lenses_raw[0], list) 
            and len(lenses_raw[0]) > 0 and isinstance(lenses_raw[0][0], list)
            and len(lenses_raw[0][0]) > 0 and isinstance(lenses_raw[0][0][0], str)):
            # Flatten: [[[str]]] -> [[str]]
            return [row[0] if isinstance(row[0], list) and len(row[0]) > 0 else row for row in lenses_raw]
        
        return lenses_raw
    
    lenses = _flatten_if_needed(lenses)
    
    # Validate shape + content  
    if len(lenses) != len(rows) or any(len(r) != len(cols) for r in lenses):
        print(f"DEBUG: Lens shape issue for {station}:")
        print(f"  Expected: {len(rows)}x{len(cols)}")
        print(f"  Got: {len(lenses)}x{len(lenses[0]) if lenses else 0}")
        print(f"  Raw lenses: {lenses[:2]}...")  # Show first 2 rows for debugging
        raise ValueError(f"Lens shape mismatch for {station}: expected {len(rows)}x{len(cols)}")
    if any((not s) or (not str(s).strip()) for row in lenses for s in row):
        raise ValueError(f"Empty lens string found in {station}")
    return lenses
