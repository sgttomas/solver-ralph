"""
Lens override system for Phase 1.

Handles artifacts/lens_overrides.json with precedence resolution
and full provenance tracking for A/B testing and fallback generation.
"""

from pathlib import Path
from typing import Dict, Any, Optional, List
import json
import hashlib
from datetime import datetime

from ...infrastructure.prompts.registry import get_registry
from ...lib.logging import log_info, log_error, log_success, log_progress


class LensOverridesManager:
    """Manages lens overrides with precedence resolution."""
    
    def __init__(self, overrides_path: Path = None):
        self.overrides_path = overrides_path or Path("artifacts/lens_overrides.json")
        self.registry = get_registry()
        self._cached_overrides = None
    
    def _load_normative_context(self) -> str:
        """Load the immutable Phase 1 normative system prompt."""
        normative_path = Path("chirality/normative_system_prompt_Phase1.txt")
        if not normative_path.exists():
            raise FileNotFoundError(f"Normative system prompt not found: {normative_path}")
        return normative_path.read_text()
    
    def _compute_current_hashes(self) -> Dict[str, str]:
        """Compute current SHA hashes for provenance tracking."""
        try:
            system_sha = self.registry.get("system").sha256[:16]
        except KeyError:
            system_sha = "missing"
        
        return {
            "system_sha": system_sha,
            "normative_sha": hashlib.sha256(self._load_normative_context().encode()).hexdigest()[:16],
        }
    
    def load_overrides(self) -> Dict[str, Any]:
        """Load overrides from disk with caching."""
        
        if self._cached_overrides is not None:
            return self._cached_overrides
        
        if not self.overrides_path.exists():
            # Return empty structure if no overrides file
            self._cached_overrides = {"overrides": {}}
            return self._cached_overrides
        
        try:
            with open(self.overrides_path) as f:
                self._cached_overrides = json.load(f)
            return self._cached_overrides
        except (json.JSONDecodeError, IOError) as e:
            log_error(f"Error loading overrides: {e}")
            # Return empty structure on error
            self._cached_overrides = {"overrides": {}}
            return self._cached_overrides
    
    def save_overrides(self, overrides: Dict[str, Any]) -> None:
        """Save overrides to disk and update cache."""
        
        self.overrides_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(self.overrides_path, "w") as f:
            json.dump(overrides, f, indent=2)
        
        # Update cache
        self._cached_overrides = overrides
        log_success(f"Overrides saved: {self.overrides_path}")
    
    def add_override(self, station: str, lenses_data: Dict[str, Any], 
                    source: str = "auto", model: str = "gpt-4o-mini", 
                    generation_params: Dict[str, Any] = None) -> None:
        """Add a lens override for a specific station."""
        
        overrides = self.load_overrides()
        
        # Build full provenance
        current_hashes = self._compute_current_hashes()
        
        override_entry = {
            "station": station,
            "matrix_id": lenses_data.get("matrix_id"),
            "rows": lenses_data.get("rows", []),
            "cols": lenses_data.get("cols", []),
            "lenses": lenses_data.get("lenses", []),
            "meta": {
                "source": source,
                "system_sha": current_hashes["system_sha"],
                "normative_sha": current_hashes["normative_sha"],
                "asset_sha": self._get_asset_sha_for_station(station),
                "model": model,
                "params": generation_params or {},
                "generated_at": datetime.utcnow().isoformat(),
                "override_reason": f"Generated via {source} mode"
            }
        }
        
        # Store in overrides structure
        if "overrides" not in overrides:
            overrides["overrides"] = {}
        
        overrides["overrides"][station] = override_entry
        
        self.save_overrides(overrides)
        log_info(f"Added override for station '{station}' (source: {source})")
    
    def _get_asset_sha_for_station(self, station: str) -> str:
        """Get asset SHA for station's generate_lenses asset."""
        
        # Map station to matrix for asset lookup
        station_matrices = {
            "Problem Statement": "C",
            "Requirements": "F", 
            "Objectives": "D",
            "Verification": "X",
            "Validation": "Z",
            "Evaluation": "E"
        }
        
        matrix_id = station_matrices.get(station)
        if not matrix_id:
            return "unknown"
        
        asset_id = f"phase1_{matrix_id.lower()}_generate_lenses"
        try:
            return self.registry.get(asset_id).sha256[:16]
        except:
            return "missing"
    
    def get_override(self, station: str) -> Optional[Dict[str, Any]]:
        """Get override for a specific station, if any."""
        
        overrides = self.load_overrides()
        return overrides.get("overrides", {}).get(station)
    
    def has_override(self, station: str) -> bool:
        """Check if station has an override."""
        return self.get_override(station) is not None
    
    def remove_override(self, station: str) -> bool:
        """Remove override for a specific station."""
        
        overrides = self.load_overrides()
        
        if station in overrides.get("overrides", {}):
            del overrides["overrides"][station]
            self.save_overrides(overrides)
            log_info(f"Removed override for station '{station}'")
            return True
        
        log_info(f"No override found for station '{station}'")
        return False
    
    def clear_all_overrides(self) -> int:
        """Clear all overrides. Returns count of cleared overrides."""
        
        overrides = self.load_overrides()
        count = len(overrides.get("overrides", {}))
        
        if count > 0:
            overrides["overrides"] = {}
            self.save_overrides(overrides)
            log_info(f"Cleared {count} overrides")
        else:
            log_info("No overrides to clear")
        
        return count
    
    def list_overrides(self) -> List[str]:
        """List all stations with overrides."""
        
        overrides = self.load_overrides()
        return list(overrides.get("overrides", {}).keys())
    
    def show_override_info(self, station: str) -> Dict[str, Any]:
        """Show detailed information for a station override."""
        
        override = self.get_override(station)
        if not override:
            raise ValueError(f"No override found for station '{station}'")
        
        return {
            "station": override["station"],
            "matrix_id": override["matrix_id"],
            "dimensions": f"{len(override['rows'])}×{len(override['cols'])}",
            "rows": override["rows"],
            "cols": override["cols"],
            "lens_count": sum(len(row) for row in override["lenses"]),
            "source": override["meta"]["source"],
            "model": override["meta"]["model"],
            "generated_at": override["meta"]["generated_at"],
            "system_sha": override["meta"]["system_sha"],
            "normative_sha": override["meta"]["normative_sha"],
            "asset_sha": override["meta"]["asset_sha"],
            "override_reason": override["meta"].get("override_reason", "Unknown")
        }
    
    def validate_overrides_schema(self) -> bool:
        """Validate overrides structure and content."""
        
        try:
            overrides = self.load_overrides()
            
            if "overrides" not in overrides:
                log_error("Missing 'overrides' key in overrides file")
                return False
            
            for station, override_data in overrides["overrides"].items():
                # Check required keys
                required_keys = ["station", "matrix_id", "rows", "cols", "lenses", "meta"]
                for key in required_keys:
                    if key not in override_data:
                        log_error(f"Missing key '{key}' in override for station '{station}'")
                        return False
                
                # Check meta structure
                meta_keys = ["source", "system_sha", "normative_sha", "asset_sha", 
                           "model", "params", "generated_at"]
                for key in meta_keys:
                    if key not in override_data["meta"]:
                        log_error(f"Missing meta key '{key}' in override for station '{station}'")
                        return False
                
                # Check lens matrix dimensions
                expected_rows = len(override_data["rows"])
                expected_cols = len(override_data["cols"])
                actual_rows = len(override_data["lenses"])
                
                if actual_rows != expected_rows:
                    log_error(f"Station {station}: expected {expected_rows} rows, got {actual_rows}")
                    return False
                
                for i, row in enumerate(override_data["lenses"]):
                    if len(row) != expected_cols:
                        log_error(f"Station {station}, row {i}: expected {expected_cols} cols, got {len(row)}")
                        return False
            
            log_success("Overrides schema validation passed")
            return True
            
        except Exception as e:
            log_error(f"Overrides validation error: {e}")
            return False