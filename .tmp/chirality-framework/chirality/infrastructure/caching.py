"""
Caching system for Phase 2 tensor cell computations.

Two-layer caching:
1. In-memory cache for current run
2. On-disk cache for cross-run persistence
"""

import json
import hashlib
import time
import os
import tempfile
from typing import Dict, Any, Optional, Tuple
from pathlib import Path
import threading
import sys


class CellCache:
    """
    Caches Phase 2 tensor cell computation results.

    Cache key includes:
    - Tensor name and cell indices
    - Operands hash (from source matrices/tensors)
    - Lens ID for semantic context
    - Snapshot hash for Phase 1 dependency
    - Model and inference parameters
    """

    def __init__(self, cache_dir: Path, enabled: bool = True):
        """
        Initialize cell cache.

        Args:
            cache_dir: Directory for persistent cache files
            enabled: Whether caching is enabled
        """
        self.cache_dir = Path(cache_dir)
        self.enabled = enabled
        self.cache_dir.mkdir(parents=True, exist_ok=True)

        # In-memory cache for current run
        self._memory_cache: Dict[str, Dict[str, Any]] = {}
        self._lock = threading.Lock()

    def compute_cache_key(
        self,
        tensor_name: str,
        indices: Tuple[int, ...],
        operands_hash: str,
        lens_id: str,
        snapshot_hash: str,
        model: str,
        temperature: float = 0.2,
        top_p: float = 1.0,
        kernel_hash: str = "unknown",
        lens_catalog_digest: str = "none",
        verbosity: Optional[str] = None,
        reasoning_effort: Optional[str] = None,
        max_tokens: Optional[int] = None,
    ) -> str:
        """
        Compute cache key for a tensor cell.

        Args:
            tensor_name: Name of tensor (M, W, U, N)
            indices: Cell indices tuple (i, j, k, ...)
            operands_hash: Hash of input operands
            lens_id: Lens ID for semantic context
            snapshot_hash: Phase 1 snapshot hash
            model: LLM model identifier
            temperature: Sampling temperature
            top_p: Nucleus sampling parameter
            kernel_hash: Kernel hash from prompt assets
            lens_catalog_digest: Hash of lens catalog
            verbosity: GPT-5 verbosity setting
            reasoning_effort: GPT-5 reasoning effort
            max_tokens: Maximum tokens setting

        Returns:
            SHA256 hash as cache key
        """
        indices_str = "_".join(map(str, indices))

        # Core parameters that affect computation
        cache_parts = [
            tensor_name,
            indices_str,
            operands_hash,
            lens_id,
            snapshot_hash,
            kernel_hash,
            lens_catalog_digest,
            model,
            f"temp_{temperature}",
            f"top_p_{top_p}",
        ]

        # Add optional GPT-5 parameters if present
        if verbosity:
            cache_parts.append(f"verbosity_{verbosity}")
        if reasoning_effort:
            cache_parts.append(f"reasoning_{reasoning_effort}")
        if max_tokens:
            cache_parts.append(f"max_tokens_{max_tokens}")

        cache_str = "|".join(cache_parts)
        return hashlib.sha256(cache_str.encode()).hexdigest()

    def get(self, cache_key: str) -> Optional[Dict[str, Any]]:
        """
        Retrieve cached result.

        Args:
            cache_key: Cache key from compute_cache_key

        Returns:
            Cached result or None if not found
        """
        if not self.enabled:
            return None

        # Check in-memory cache first
        with self._lock:
            if cache_key in self._memory_cache:
                return self._memory_cache[cache_key]

        # Check on-disk cache
        cache_file = self.cache_dir / f"{cache_key}.json"
        if cache_file.exists():
            try:
                with open(cache_file, "r") as f:
                    result = json.load(f)

                # Load into memory cache
                with self._lock:
                    self._memory_cache[cache_key] = result

                return result
            except (json.JSONDecodeError, FileNotFoundError):
                # Corrupted cache file, remove it
                cache_file.unlink(missing_ok=True)

        return None

    def put(self, cache_key: str, result: Dict[str, Any]):
        """
        Store result in cache.

        Args:
            cache_key: Cache key from compute_cache_key
            result: Computation result to cache
        """
        if not self.enabled:
            return

        # Store in memory cache
        with self._lock:
            self._memory_cache[cache_key] = result

        # Store on disk
        cache_file = self.cache_dir / f"{cache_key}.json"
        try:
            with open(cache_file, "w") as f:
                json.dump(result, f, indent=2)
        except Exception as e:
            # Log error but don't fail the computation
            print(f"Warning: Failed to write cache file {cache_file}: {e}", file=sys.stderr)

    def compute_operands_hash(self, operands: Dict[str, Any]) -> str:
        """
        Compute hash of operands for cache key.

        Args:
            operands: Dictionary of operand values

        Returns:
            SHA256 hash of operands
        """
        # Sort keys for deterministic hash
        operands_str = json.dumps(operands, sort_keys=True, separators=(",", ":"))
        return hashlib.sha256(operands_str.encode()).hexdigest()

    def clear_memory_cache(self):
        """Clear in-memory cache (keep disk cache)."""
        with self._lock:
            self._memory_cache.clear()

    def clear_all(self):
        """Clear both memory and disk cache."""
        self.clear_memory_cache()

        # Remove all cache files
        for cache_file in self.cache_dir.glob("*.json"):
            cache_file.unlink()

    def get_stats(self) -> Dict[str, Any]:
        """Get cache statistics."""
        disk_files = len(list(self.cache_dir.glob("*.json")))

        with self._lock:
            memory_entries = len(self._memory_cache)

        return {
            "enabled": self.enabled,
            "cache_dir": str(self.cache_dir),
            "memory_entries": memory_entries,
            "disk_files": disk_files,
        }


class ResumableRunner:
    """
    Provides resume capability for Phase 2 tensor computations.

    Tracks completed cells via:
    - run_manifest.json: Overall progress and metadata
    - cell_traces/: Individual cell results
    """

    def __init__(self, artifacts_dir: Path):
        """
        Initialize resumable runner.

        Args:
            artifacts_dir: Artifacts directory for the run
        """
        self.artifacts_dir = Path(artifacts_dir)
        self.manifest_path = self.artifacts_dir / "run_manifest.json"
        self.cell_traces_dir = self.artifacts_dir / "cell_traces"

        self.artifacts_dir.mkdir(parents=True, exist_ok=True)
        self.cell_traces_dir.mkdir(parents=True, exist_ok=True)

    def load_manifest(self) -> Dict[str, Any]:
        """Load run manifest or create empty one."""
        if self.manifest_path.exists():
            try:
                with open(self.manifest_path, "r") as f:
                    return json.load(f)
            except (json.JSONDecodeError, FileNotFoundError):
                pass

        # Create empty manifest
        return {
            "run_id": None,
            "timestamp": None,
            "phase1_complete": False,
            "phase2_complete": False,
            "snapshot_hash": None,
            "model": None,
            "cells_planned": 0,
            "cells_completed": 0,
            "tensors": {},
        }

    def save_manifest(self, manifest: Dict[str, Any]):
        """Save run manifest atomically."""
        # Atomic write using temporary file + rename
        with tempfile.NamedTemporaryFile(
            mode="w",
            dir=self.artifacts_dir,
            prefix=f".{self.manifest_path.name}_",
            suffix=".tmp",
            delete=False,
        ) as tmp_file:
            json.dump(manifest, tmp_file, indent=2)
            tmp_file.flush()
            os.fsync(tmp_file.fileno())  # Force write to disk
            tmp_path = tmp_file.name

        # Atomic replace
        os.replace(tmp_path, self.manifest_path)

    def get_completed_cells(self, tensor_name: str) -> set:
        """
        Get set of completed cell keys for a tensor.

        Args:
            tensor_name: Name of tensor (M, W, U, N)

        Returns:
            Set of completed cell keys like "0_1_2"
        """
        completed = set()
        tensor_dir = self.cell_traces_dir / tensor_name

        if tensor_dir.exists():
            for cell_file in tensor_dir.glob("*.json"):
                # Skip temporary files from atomic writes
                if cell_file.name.startswith(".") and ".tmp" in cell_file.name:
                    continue

                # Validate file is readable before including
                try:
                    with open(cell_file, "r") as f:
                        json.load(f)
                    # Extract cell key from filename
                    cell_key = cell_file.stem
                    completed.add(cell_key)
                except (json.JSONDecodeError, FileNotFoundError, OSError):
                    # Corrupted file, clean it up
                    try:
                        cell_file.unlink()
                    except OSError:
                        pass

        return completed

    def compute_cell_path(self, tensor_name: str, indices: Tuple[int, ...]) -> Path:
        """
        Canonical function to compute cell file path.

        Args:
            tensor_name: Name of tensor
            indices: Cell indices

        Returns:
            Path to cell file
        """
        tensor_dir = self.cell_traces_dir / tensor_name
        cell_key = "_".join(map(str, indices))
        return tensor_dir / f"{cell_key}.json"

    def save_cell_result(self, tensor_name: str, indices: Tuple[int, ...], result: Dict[str, Any]):
        """
        Save cell computation result atomically for resume capability.

        Uses atomic file replacement to avoid partial writes during crashes.

        Args:
            tensor_name: Name of tensor
            indices: Cell indices
            result: Computation result
        """
        # Use canonical path computation
        cell_file = self.compute_cell_path(tensor_name, indices)
        tensor_dir = cell_file.parent
        tensor_dir.mkdir(parents=True, exist_ok=True)

        # Add metadata
        cell_data = {
            "tensor_name": tensor_name,
            "indices": list(indices),
            "timestamp": time.time(),
            "result": result,
        }

        # Atomic write using temporary file + rename
        with tempfile.NamedTemporaryFile(
            mode="w", dir=tensor_dir, prefix=f".{cell_file.name}_", suffix=".tmp", delete=False
        ) as tmp_file:
            json.dump(cell_data, tmp_file, indent=2)
            tmp_file.flush()
            os.fsync(tmp_file.fileno())  # Force write to disk
            tmp_path = tmp_file.name

        # Atomic replace
        os.replace(tmp_path, cell_file)

    def load_cell_result(
        self, tensor_name: str, indices: Tuple[int, ...]
    ) -> Optional[Dict[str, Any]]:
        """
        Load saved cell result.

        Args:
            tensor_name: Name of tensor
            indices: Cell indices

        Returns:
            Saved result or None if not found
        """
        # Use canonical path computation
        cell_file = self.compute_cell_path(tensor_name, indices)

        if cell_file.exists():
            try:
                with open(cell_file, "r") as f:
                    cell_data = json.load(f)
                return cell_data.get("result")
            except (json.JSONDecodeError, FileNotFoundError):
                # Corrupted file, remove it to prevent repeated failures
                try:
                    cell_file.unlink()
                except OSError:
                    pass

        return None

    def update_progress(self, tensor_name: str, cells_completed: int, total_cells: int):
        """
        Update progress tracking in manifest.

        Args:
            tensor_name: Name of tensor being computed
            cells_completed: Number of cells completed so far
            total_cells: Total number of cells in tensor
        """
        manifest = self.load_manifest()

        # Update tensor-specific progress
        if "tensors" not in manifest:
            manifest["tensors"] = {}

        manifest["tensors"][tensor_name] = {
            "cells_completed": cells_completed,
            "total_cells": total_cells,
            "progress": cells_completed / total_cells if total_cells > 0 else 0,
        }

        # Update overall progress
        total_completed = sum(t.get("cells_completed", 0) for t in manifest["tensors"].values())
        total_planned = sum(t.get("total_cells", 0) for t in manifest["tensors"].values())

        manifest["cells_completed"] = total_completed
        manifest["cells_planned"] = total_planned

        # Mark Phase 2 complete if all cells done
        if total_planned > 0 and total_completed >= total_planned:
            manifest["phase2_complete"] = True

        self.save_manifest(manifest)
