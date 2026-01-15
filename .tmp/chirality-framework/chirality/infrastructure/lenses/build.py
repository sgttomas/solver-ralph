"""
Lens Builder System.

Generates lens text from triples using stateless LLM calls.
Each lens is computed independently and cached by lens_id.
"""

import json
from typing import Dict, Any, Optional
from pathlib import Path

# No longer importing call_responses_api - using call_responses directly
from ..prompts.registry import get_registry


class LensBuilder:
    """
    Builds lens text from lens triples statelessly.

    Each lens is generated independently without memory.
    Results are cached by lens_id for reuse.
    """

    def __init__(
        self,
        model: str = None,
        temperature: Optional[float] = None,
        system_prompt: Optional[str] = None,
    ):
        """
        Initialize lens builder.

        Args:
            model: LLM model identifier (uses global config if None)
            temperature: Sampling temperature
            system_prompt: Optional custom system prompt
        """
        if model is None:
            from ..llm.config import get_config
            model = get_config().model
        self.model = model
        if temperature is None:
            try:
                from ..llm.config import get_config
                self.temperature = get_config().temperature
            except Exception:
                self.temperature = 1.0
        else:
            self.temperature = temperature
        self.system_prompt = system_prompt or self._load_system_prompt()

    def build_lens_catalog(self, lens_triples_path: Path, output_path: Path) -> int:
        """
        Build complete lens catalog from triples.

        Args:
            lens_triples_path: Path to lenses_triples.json
            output_path: Path to write lens_catalog.jsonl

        Returns:
            Number of lenses generated
        """
        # Load lens triples
        with open(lens_triples_path, "r") as f:
            triples_data = json.load(f)

        lenses = triples_data["lenses"]

        # Generate lens text for each triple
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

        generated_count = 0
        with open(output_path, "w") as f:
            for lens in lenses:
                lens_text = self._generate_lens_text(lens)

                # Write to JSONL catalog
                catalog_entry = {
                    "lens_id": lens["lens_id"],
                    "row": lens["row"],
                    "col": lens["col"],
                    "station": lens["station"],
                    "text": lens_text,
                    "matrix_source": lens["matrix_source"],
                    "kernel_hash": lens["kernel_hash"],
                    "model": lens["model"],
                    "prompt_version": lens["prompt_version"],
                }

                f.write(json.dumps(catalog_entry) + "\n")
                generated_count += 1

        return generated_count

    def _generate_lens_text(self, lens_spec: Dict[str, Any]) -> str:
        """
        Generate lens text for a single lens specification.

        Args:
            lens_spec: Lens specification dict

        Returns:
            Generated lens text
        """
        row = lens_spec["row"]
        col = lens_spec["col"]
        station = lens_spec["station"]

        # Load lens generation prompt from registry and substitute variables
        registry = get_registry()
        # Use per-matrix generation asset (e.g., phase1_c_generate_lenses)
        matrix_id = lens_spec.get("matrix_source", "").lower() or "c"
        asset_id = f"phase1_{matrix_id}_generate_lenses"
        lens_prompt_template = registry.get_text(asset_id)
        
        # Get matrix_id for template substitution
        matrix_id = lens_spec.get("matrix_source", "Unknown")
        
        # Substitute template variables
        user_prompt = lens_prompt_template.replace("{{matrix_id}}", matrix_id)
        user_prompt = user_prompt.replace("{{station}}", station)
        user_prompt = user_prompt.replace("{{row}}", row)
        user_prompt = user_prompt.replace("{{col}}", col)

        # Call LLM statelessly using Responses API
        from ..llm.openai_adapter import call_responses
        
        response_result = call_responses(
            instructions=self.system_prompt,
            input=user_prompt
        )
        
        # Convert to expected format for compatibility
        response = {"content": response_result.get("output_text", "")}
        metadata = response_result.get("raw", {}).get("metadata", {})

        # Extract lens text from response (may contain reasoning + JSON)
        if isinstance(response, dict):
            # Direct JSON response
            if "text" in response:
                return response["text"]
            elif "lens" in response:
                return response["lens"]
        elif isinstance(response, str):
            # Response contains reasoning + JSON, extract the JSON part
            try:
                import re
                json_match = re.search(r'\{[^}]*"text"[^}]*\}', response)
                if json_match:
                    json_str = json_match.group()
                    json_data = json.loads(json_str)
                    return json_data.get("text", response)
                else:
                    # No JSON found, use the whole response
                    return response
            except (json.JSONDecodeError, AttributeError):
                # JSON parsing failed, use the whole response
                return response

        # Fallback if all parsing fails
        return f"Through the lens of {row} × {col} at {station}"

    def _load_system_prompt(self) -> str:
        """Load Phase 1 system prompt from registry for consistency."""
        try:
            registry = get_registry()
            return registry.get_text("system")
        except Exception:
            # Fallback if registry fails
            return "You are generating interpretive lenses for the Chirality Framework semantic calculator."

    def extract_component_lenses(self, catalog_path: Path, component: str, output_path: Path):
        """
        Extract lenses for a specific component/matrix.

        Args:
            catalog_path: Path to lens_catalog.jsonl
            component: Component name (e.g., 'M', 'E', 'X')
            output_path: Path to write component lenses JSON
        """
        component_lenses = {}

        with open(catalog_path, "r") as f:
            for line in f:
                entry = json.loads(line)
                if entry["matrix_source"] == component:
                    key = f"{entry['row']}×{entry['col']}"
                    component_lenses[key] = {
                        "lens_id": entry["lens_id"],
                        "text": entry["text"],
                        "row": entry["row"],
                        "col": entry["col"],
                        "station": entry["station"],
                    }

        # Save component lenses
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

        output_data = {
            "component": component,
            "lens_count": len(component_lenses),
            "lenses": component_lenses,
        }

        with open(output_path, "w") as f:
            json.dump(output_data, f, indent=2)


def build_lens_catalog(triples_path: Path, output_path: Path, model: str = None) -> int:
    """
    Convenience function to build lens catalog.

    Args:
        triples_path: Path to lenses_triples.json
        output_path: Path to write lens_catalog.jsonl
        model: LLM model identifier (uses global config if None)

    Returns:
        Number of lenses generated
    """
    if model is None:
        from ..llm.config import get_config
        model = get_config().model
        
    builder = LensBuilder(model=model)
    return builder.build_lens_catalog(triples_path, output_path)
