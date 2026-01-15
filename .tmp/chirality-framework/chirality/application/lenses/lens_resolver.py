"""
Unified lens resolution system with precedence handling.

Implements the lookup order: overrides → catalog → auto generation → error
Provides the integration point for Phase D orchestrator.
"""

from pathlib import Path
from typing import Dict, Any, Optional, Literal
import json

from ...domain.matrices.canonical import get_canonical_matrix, get_matrix_info
from ...infrastructure.prompts.registry import get_registry
from ...infrastructure.llm.mock_resolvers import EchoResolver
from ...lib.logging import log_info, log_error, log_progress, log_success
from .catalog_manager import LensCatalogManager
from .overrides_manager import LensOverridesManager


class LensResolver:
    """Unified lens resolution with precedence and mode support."""
    
    def __init__(self, 
                 lens_mode: Literal["catalog", "auto"] = "catalog",
                 catalog_path: Path = None,
                 overrides_path: Path = None,
                 model: Optional[str] = None):
        
        self.lens_mode = lens_mode
        # Single source of truth: pull model from global LLM config if not provided
        if model is None:
            from ...infrastructure.llm.config import get_config
            model = get_config().model
        self.model = model
        self.registry = get_registry()
        self.resolver = None  # QUARANTINED: Was EchoResolver()
        
        # Initialize managers
        self.catalog_manager = LensCatalogManager(catalog_path)
        self.overrides_manager = LensOverridesManager(overrides_path)
        
        # Station to matrix mapping using canonical matrix definitions
        self.station_matrices = {
            "Problem Statement": get_matrix_info("C"),
            "Requirements": get_matrix_info("F"), 
            "Objectives": get_matrix_info("D"),
            "Verification": get_matrix_info("X"),
            "Validation": get_matrix_info("Z"),
            "Evaluation": get_matrix_info("E")
        }
    
    def resolve_lenses(self, station: str) -> Dict[str, Any]:
        """
        Resolve lenses for a station using precedence order.
        
        Order: overrides → catalog → auto generation (if allowed) → error
        """
        
        log_progress(f"Resolving lenses for station '{station}'")
        
        # Step 1: Check overrides
        override = self.overrides_manager.get_override(station)
        if override:
            log_info(f"  - Using override (source: {override['meta']['source']})")
            return self._format_lens_result(override, source="override")
        
        # Step 2: Check catalog
        try:
            catalog_lenses = self.catalog_manager.get_station_lenses(station)
            log_info("  - Using catalog")
            return self._format_lens_result(catalog_lenses, source="catalog")
            
        except (FileNotFoundError, ValueError) as e:
            log_info(f"  - Catalog unavailable: {e}")
        
        # Step 3: Auto generation (if allowed)
        if self.lens_mode == "auto":
            log_info("  - Generating lenses on-the-fly")
            generated_lenses = self._generate_lenses_for_station(station)
            
            # Persist to overrides
            self.overrides_manager.add_override(
                station=station,
                lenses_data=generated_lenses,
                source="auto",
                model=self.model
            )
            
            return self._format_lens_result(generated_lenses, source="auto")
        
        # Step 4: Error (catalog mode with no catalog)
        raise ValueError(
            f"No lenses available for station '{station}' in catalog mode. "
            f"Run 'chirality lenses ensure' to create catalog or use --lens-mode=auto"
        )
    
    def _format_lens_result(self, lenses_data: Dict[str, Any], source: str) -> Dict[str, Any]:
        """Format lens result with consistent structure."""
        return {
            "station": lenses_data["station"],
            "matrix_id": lenses_data.get("matrix_id"),
            "rows": lenses_data.get("rows", []),
            "cols": lenses_data.get("cols", []),
            "lenses": lenses_data.get("lenses", []),
            "source": source,
            "meta": lenses_data.get("meta", {})
        }
    
    def _generate_lenses_for_station(self, station: str) -> Dict[str, Any]:
        """Generate lenses on-the-fly for a specific station."""
        
        # Get matrix info
        matrix_info = self.station_matrices.get(station)
        if not matrix_info:
            raise ValueError(f"Unknown station: {station}")
        
        matrix_id = matrix_info["matrix_id"]
        
        # Load and render the generation asset
        asset_id = f"phase1_{matrix_id.lower()}_generate_lenses"
        try:
            asset_text = self.registry.get_text(asset_id)
        except KeyError:
            raise ValueError(f"Generation asset not found: {asset_id}")
        
        # Replace placeholders (no {{context}} allowed here)
        rendered_prompt = asset_text.replace("{{station}}", station)
        rendered_prompt = rendered_prompt.replace("{{matrix_id}}", matrix_id)
        rendered_prompt = rendered_prompt.replace("{{rows}}", str(matrix_info["rows"]))
        rendered_prompt = rendered_prompt.replace("{{cols}}", str(matrix_info["cols"]))
        rendered_prompt = rendered_prompt.replace("{{row_labels}}", str(matrix_info["row_labels"]))
        rendered_prompt = rendered_prompt.replace("{{col_labels}}", str(matrix_info["col_labels"]))
        
        # Make LLM call
        # response = self.resolver.resolve(rendered_prompt) # QUARANTINED
        response = {} # Dummy response
        
        # Parse response (in production, this would be proper JSON from LLM)
        # For now, generate placeholder structure
        rows, cols = matrix_info["rows"], matrix_info["cols"]
        placeholder_lenses = [
            [f"auto_{station.lower().replace(' ', '_')}_lens_{r}_{c}" for c in range(cols)]
            for r in range(rows)
        ]
        
        return {
            "station": station,
            "matrix_id": matrix_id,
            "rows": matrix_info["row_labels"],
            "cols": matrix_info["col_labels"],
            "lenses": placeholder_lenses,
            "meta": {
                "generated_at": "auto-generated",
                "model": self.model,
                "source": "auto"
            }
        }
    
    def get_lens_source(self, station: str) -> str:
        """Get the source that would be used for a station (without resolving)."""
        
        # Check overrides first
        if self.overrides_manager.has_override(station):
            override = self.overrides_manager.get_override(station)
            return f"override (source: {override['meta']['source']})"
        
        # Check catalog
        try:
            self.catalog_manager.get_station_lenses(station)
            return "catalog"
        except (FileNotFoundError, ValueError):
            pass
        
        # Would use auto generation if allowed
        if self.lens_mode == "auto":
            return "auto (would generate)"
        
        return "error (no source available)"
    
    def clear_station_override(self, station: str) -> bool:
        """Clear override for a specific station."""
        return self.overrides_manager.remove_override(station)
    
    def clear_all_overrides(self) -> int:
        """Clear all overrides."""
        return self.overrides_manager.clear_all_overrides()
    
    def list_override_stations(self) -> list[str]:
        """List stations with overrides."""
        return self.overrides_manager.list_overrides()
    
    def validate_lens_system(self) -> Dict[str, bool]:
        """Validate the entire lens system."""
        
        results = {}
        
        # Validate catalog if it exists
        try:
            results["catalog"] = self.catalog_manager.validate_catalog_schema()
        except FileNotFoundError:
            results["catalog"] = None  # No catalog is OK
        
        # Validate overrides
        results["overrides"] = self.overrides_manager.validate_overrides_schema()
        
        # Check that all stations can be resolved
        all_stations_ok = True
        station_results = {}
        
        for station in self.station_matrices.keys():
            try:
                source = self.get_lens_source(station)
                station_results[station] = f"OK ({source})"
                if "error" in source:
                    all_stations_ok = False
            except Exception as e:
                station_results[station] = f"ERROR: {e}"
                all_stations_ok = False
        
        results["stations"] = all_stations_ok
        results["station_details"] = station_results
        
        return results
