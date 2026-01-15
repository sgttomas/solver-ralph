"""
Lens catalog management with invalidation and persistence.

Handles loading, caching, and invalidation of lens catalogs.
"""

from pathlib import Path
from typing import Dict, Any, Optional, List
import json
import hashlib
from datetime import datetime

from ...infrastructure.prompts.registry import get_registry
from ...lib.logging import log_info, log_error, log_success
from .catalog_generator import LensCatalogGenerator


class LensCatalogManager:
    """Manages lens catalog lifecycle with proper invalidation."""
    
    def __init__(self, catalog_path: Path = None):
        self.catalog_path = catalog_path or Path("artifacts/lens_catalog.json")
        self.registry = get_registry()
        self._cached_catalog = None
    
    def _load_normative_context(self) -> str:
        """Load the immutable Phase 1 normative system prompt."""
        normative_path = Path("chirality/normative_system_prompt_Phase1.txt")
        if not normative_path.exists():
            raise FileNotFoundError(f"Normative system prompt not found: {normative_path}")
        return normative_path.read_text()
    
    def _compute_current_hashes(self) -> Dict[str, str]:
        """Compute current SHA hashes for invalidation check."""
        try:
            system_sha = self.registry.get("system").sha256[:16]
        except KeyError:
            system_sha = "missing"
        
        try:
            asset_sha = self.registry.get("phase1_lens_catalog_generation").sha256[:16]
        except KeyError:
            asset_sha = "missing"
        
        return {
            "system_sha": system_sha,
            "normative_sha": hashlib.sha256(self._load_normative_context().encode()).hexdigest()[:16],
            "asset_sha": asset_sha
        }
    
    def _is_catalog_valid(self) -> bool:
        """Check if existing catalog is still valid."""
        if not self.catalog_path.exists():
            return False
        
        try:
            with open(self.catalog_path) as f:
                catalog = json.load(f)
            
            if "meta" not in catalog:
                return False
            
            # Check SHA hashes for invalidation
            current_hashes = self._compute_current_hashes()
            catalog_meta = catalog["meta"]
            
            for key in ["system_sha", "normative_sha", "asset_sha"]:
                if catalog_meta.get(key) != current_hashes.get(key):
                    log_info(f"Catalog invalid: {key} changed ({catalog_meta.get(key)} → {current_hashes.get(key)})")
                    return False
            
            return True
            
        except (json.JSONDecodeError, KeyError, FileNotFoundError) as e:
            log_error(f"Catalog validation error: {e}")
            return False
    
    def ensure_catalog(self, force_refresh: bool = False) -> Dict[str, Any]:
        """Ensure catalog exists and is valid (idempotent)."""
        
        if not force_refresh and self._is_catalog_valid():
            log_info("Lens catalog is up to date")
            return self.load_catalog()
        
        log_info("Regenerating lens catalog...")
        generator = LensCatalogGenerator()
        catalog = generator.generate_catalog(self.catalog_path)
        
        # Clear cache
        self._cached_catalog = None
        
        return catalog
    
    def load_catalog(self) -> Dict[str, Any]:
        """Load catalog from disk with caching."""
        
        if self._cached_catalog is not None:
            return self._cached_catalog
        
        if not self.catalog_path.exists():
            raise FileNotFoundError(f"Lens catalog not found: {self.catalog_path}")
        
        with open(self.catalog_path) as f:
            self._cached_catalog = json.load(f)
        
        return self._cached_catalog
    
    def get_station_lenses(self, station: str) -> Dict[str, Any]:
        """Get lenses for a specific station."""
        catalog = self.load_catalog()
        
        if station not in catalog["catalog"]:
            available = list(catalog["catalog"].keys())
            raise ValueError(f"Station '{station}' not found. Available: {available}")
        
        return catalog["catalog"][station]
    
    def get_catalog_meta(self) -> Dict[str, Any]:
        """Get catalog metadata."""
        catalog = self.load_catalog()
        return catalog["meta"]
    
    def list_stations(self) -> List[str]:
        """List available stations in catalog."""
        catalog = self.load_catalog()
        return list(catalog["catalog"].keys())
    
    def show_station_info(self, station: str) -> Dict[str, Any]:
        """Show detailed information for a station."""
        station_data = self.get_station_lenses(station)
        
        return {
            "station": station_data["station"],
            "matrix_id": station_data["matrix_id"],
            "dimensions": f"{len(station_data['rows'])}×{len(station_data['cols'])}",
            "rows": station_data["rows"],
            "cols": station_data["cols"],
            "lens_count": sum(len(row) for row in station_data["lenses"]),
            "generated_at": station_data["meta"]["generated_at"],
            "context_hash": station_data["meta"]["context_hash"]
        }
    
    def validate_catalog_schema(self) -> bool:
        """Validate catalog structure and content."""
        try:
            catalog = self.load_catalog()
            
            # Check top-level structure
            required_keys = ["meta", "catalog"]
            for key in required_keys:
                if key not in catalog:
                    log_error(f"Missing top-level key: {key}")
                    return False
            
            # Check meta structure
            meta_keys = ["system_sha", "normative_sha", "asset_sha", "model", "stations", "generated_at"]
            for key in meta_keys:
                if key not in catalog["meta"]:
                    log_error(f"Missing meta key: {key}")
                    return False
            
            # Check each station
            for station, station_data in catalog["catalog"].items():
                station_keys = ["station", "matrix_id", "rows", "cols", "lenses", "meta"]
                for key in station_keys:
                    if key not in station_data:
                        log_error(f"Missing station key in {station}: {key}")
                        return False
                
                # Check lens matrix dimensions
                expected_rows = len(station_data["rows"])
                expected_cols = len(station_data["cols"])
                actual_rows = len(station_data["lenses"])
                
                if actual_rows != expected_rows:
                    log_error(f"Station {station}: expected {expected_rows} rows, got {actual_rows}")
                    return False
                
                for i, row in enumerate(station_data["lenses"]):
                    if len(row) != expected_cols:
                        log_error(f"Station {station}, row {i}: expected {expected_cols} cols, got {len(row)}")
                        return False
                
                # Check for empty lenses
                for i, row in enumerate(station_data["lenses"]):
                    for j, lens in enumerate(row):
                        if not lens or lens.strip() == "":
                            log_error(f"Station {station}: empty lens at [{i}][{j}]")
                            return False
            
            log_success("Catalog schema validation passed")
            return True
            
        except Exception as e:
            log_error(f"Catalog validation error: {e}")
            return False