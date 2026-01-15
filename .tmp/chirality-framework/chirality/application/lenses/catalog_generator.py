"""
Lens catalog generation system for Phase 1.

Implements the meta-pipeline for building artifacts/lens_catalog.json
with proper invalidation and provenance tracking.
"""

from pathlib import Path
from typing import Dict, Any, List, Optional
import json
import hashlib
from datetime import datetime


from ...domain.matrices.canonical import get_canonical_matrix, get_matrix_info
from ...infrastructure.prompts.registry import get_registry
from ...infrastructure.llm.openai_adapter import call_responses
from ...lib.logging import log_info, log_progress, log_success


class LensCatalogGenerator:
    """Generates lens catalogs for Phase 1 stations."""
    
    def __init__(self, model: Optional[str] = None):
        # Single source of truth: pull model from global LLM config if not provided
        if model is None:
            from ...infrastructure.llm.config import get_config
            model = get_config().model
        self.model = model
        self.registry = get_registry()
        # No resolver instance here; we call the Responses API in the method that needs it.
        
        # Station to matrix mapping using canonical matrix definitions
        self.station_matrices = {
            "Problem Statement": get_matrix_info("C"),
            "Requirements": get_matrix_info("F"), 
            "Objectives": get_matrix_info("D"),
            "Verification": get_matrix_info("X"),
            "Validation": get_matrix_info("Z"),
            "Evaluation": get_matrix_info("E")
        }

    def _render_template_strict(self, template: str, variables: Dict[str, str]) -> str:
        import re
        rendered = template
        for var_name, var_value in variables.items():
            placeholder = f"{{{{{var_name}}}}}"
            rendered = rendered.replace(placeholder, var_value)
        leftover_matches = re.findall(r'\{\{[^}]+\}\}', rendered)
        if leftover_matches:
            raise ValueError(
                f"Template rendering failed: unresolved placeholders {leftover_matches}. "
                f"Available variables: {list(variables.keys())}"
            )
        return rendered
    
    def _load_normative_context(self) -> str:
        """Load the immutable Phase 1 normative system prompt."""
        normative_path = Path("chirality/normative_system_prompt_Phase1.txt")
        if not normative_path.exists():
            raise FileNotFoundError(f"Normative system prompt not found: {normative_path}")
        return normative_path.read_text()
    
    def _compute_context_hash(self, context: str) -> str:
        """Compute SHA256 hash of the context for invalidation."""
        return hashlib.sha256(context.encode()).hexdigest()[:16]
    
    def _build_station_context(self, station: str) -> str:
        """Build the context for a specific station."""
        normative_text = self._load_normative_context()
        
        # Get matrix info from station mapping
        matrix_info = self.station_matrices[station]
        matrix_id = matrix_info["matrix_id"]
        matrix_def = f"Matrix {matrix_id} ({matrix_info['rows']}×{matrix_info['cols']})"
        
        # Build context per colleague_1's specification
        context = f"""## Normative Framework (Phase 1)

{normative_text}

## Station Context

Station: {station}
{matrix_def}
Row labels: {matrix_info['row_labels']}  
Column labels: {matrix_info['col_labels']}

## Semantic Valley Progression

Station progression: Problem Statement → Requirements → Objectives → Verification → Validation → Evaluation
Current station: {station}

Generate interpretive lenses for all matrix positions that capture the semantic intersection of row×column ontologies synthesized with the station meaning."""
        
        return context
    
    def _generate_lenses_for_station(self, station: str) -> Dict[str, Any]:
        """Generate lenses for a specific station using the meta-pipeline."""
        
        # Get matrix info
        matrix_info = self.station_matrices[station]
        matrix_id = matrix_info["matrix_id"]
        
        # Build context
        context = self._build_station_context(station)
        
        # Load and render the catalog generation asset
        asset_text = self.registry.get_text("phase1_lens_catalog_generation")
        
        # Replace placeholders
        template_vars = {
            "context": context,
            "station": station,
            "matrix_id": matrix_id,
            "n_rows": str(matrix_info["rows"]),
            "n_cols": str(matrix_info["cols"]),
            "rows_json": json.dumps(matrix_info["row_labels"]),
            "cols_json": json.dumps(matrix_info["col_labels"]),
        }

        rendered_prompt = self._render_template_strict(asset_text, template_vars)
        
        log_progress(f"Generating lenses for {station} (Matrix {matrix_id})")
        
        # Call the Responses API (no model kwarg; respect adapter's contract)
        resp = call_responses(
            instructions=context,
            input=rendered_prompt,
            expects_json=True,
            store=True,
            metadata={"op": "lens_catalog_generate", "station": station, "matrix": matrix_id},
        )
        output_text = resp.get("output_text", "")
        try:
            parsed = json.loads(output_text) if output_text else {}
        except json.JSONDecodeError:
            parsed = {"error": "invalid_json", "raw_output": output_text}

        # If parsed contains the definitive lenses list, use it; otherwise fall back to a stub.
        lenses = parsed.get("lenses") if isinstance(parsed, dict) else None
        if not isinstance(lenses, list):
            # Fallback structure
            rows, cols = matrix_info["rows"], matrix_info["cols"]
            lenses = [
                [f"{station.lower().replace(' ', '_')}_lens_{r}_{c}" for c in range(cols)]
                for r in range(rows)
            ]
        
        return {
            "station": station,
            "matrix_id": matrix_id,
            "rows": matrix_info["row_labels"],
            "cols": matrix_info["col_labels"],
            "lenses": lenses,
            "meta": {
                "generated_at": datetime.utcnow().isoformat(),
                "context_hash": self._compute_context_hash(context),
                "model": self.model
            }
        }
    
    def generate_catalog(self, output_path: Path) -> Dict[str, Any]:
        """Generate complete lens catalog for all stations."""
        
        log_info("Generating lens catalog...")
        
        catalog = {}
        try:
            system_sha = self.registry.get("system").sha256[:16]
        except KeyError:
            system_sha = "missing"
        
        try:
            asset_sha = self.registry.get("phase1_lens_catalog_generation").sha256[:16]
        except KeyError:
            asset_sha = "missing"
        
        meta = {
            "system_sha": system_sha,
            "normative_sha": hashlib.sha256(self._load_normative_context().encode()).hexdigest()[:16],
            "asset_sha": asset_sha,
            "model": self.model,
            "stations": list(self.station_matrices.keys()),
            "generated_at": datetime.utcnow().isoformat()
        }
        
        # Generate lenses for each station
        for station in self.station_matrices.keys():
            station_lenses = self._generate_lenses_for_station(station)
            catalog[station] = station_lenses
        
        # Build final catalog structure
        full_catalog = {
            "meta": meta,
            "catalog": catalog
        }
        
        # Write to file
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            json.dump(full_catalog, f, indent=2)
        
        log_success(f"Lens catalog generated: {output_path}")
        log_info(f"  - Stations: {len(catalog)}")
        log_info(f"  - System SHA: {meta['system_sha']}")
        log_info(f"  - Normative SHA: {meta['normative_sha']}")
        
        return full_catalog
