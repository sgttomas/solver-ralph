"""
Phase 2 Tensor Engine.

Stateless executor for tensor computations using Phase 1 snapshot
as system prompt. Each cell is computed independently without
rolling context.
"""

import json
import hashlib
from typing import Dict, List, Any, Optional, Tuple
from pathlib import Path
import itertools

from ...infrastructure.llm.openai_adapter import call_responses
from ...infrastructure.llm.repair import try_parse_json_or_repair, create_tensor_cell_schema_hint
from ...infrastructure.monitoring.tracer import JSONLTracer
from ...infrastructure.caching import CellCache, ResumableRunner
from ...domain.budgets import BudgetConfig, BudgetTracker
from ...lib.logging import log_info


class TensorEngine:
    """
    Executes Phase 2 tensor computations statelessly.

    Uses Phase 1 snapshot as system prompt and computes
    each tensor cell independently.
    """

    def __init__(
        self,
        snapshot_path: Path,
        phase1_output: Dict[str, Any],
        lens_catalog_path: Optional[Path] = None,
        model: Optional[str] = None,
        temperature: Optional[float] = None,
        top_p: float = 0.9,
        max_repair: int = 1,
        budget_config: Optional[BudgetConfig] = None,
        parallel: int = 8,
        tracer: Optional[JSONLTracer] = None,
        artifacts_dir: Optional[Path] = None,
        cache_enabled: bool = True,
        resume: bool = False,
    ):
        """
        Initialize tensor engine.

        Args:
            snapshot_path: Path to phase1_snapshot.md
            phase1_output: Phase 1 output with matrices
            lens_catalog_path: Optional path to lens catalog
            model: LLM model identifier
            temperature: Sampling temperature
            top_p: Nucleus sampling parameter
            max_repair: Maximum repair attempts
            budget_config: Budget configuration for tracking
            parallel: Number of parallel cell computations
            tracer: Optional JSONL tracer
            artifacts_dir: Directory for caching and resume data
            cache_enabled: Whether to enable cell caching
            resume: Whether to resume from previous run
        """
        self.snapshot = self._load_snapshot(snapshot_path)
        self.phase1_output = phase1_output
        self.lens_catalog = self._load_lens_catalog(lens_catalog_path) if lens_catalog_path else {}

        # Single source of truth: model/temperature/top_p from global config when not provided
        from ...infrastructure.llm.config import get_config
        cfg = get_config()
        self.model = model if model is not None else cfg.model
        # Single-source temperature: use config when not provided
        self.temperature = cfg.temperature if temperature is None else temperature
        self.top_p = top_p
        self.max_repair = max_repair
        self.parallel = parallel
        self.tracer = tracer
        self.artifacts_dir = Path(artifacts_dir) if artifacts_dir else None
        self.resume = resume

        # Budget tracking (single source of truth)
        self.budget_tracker = None
        if budget_config:
            self.budget_tracker = BudgetTracker(budget_config, phase="phase2")

        # Cache and resume infrastructure
        if cache_enabled and self.artifacts_dir:
            cache_dir = self.artifacts_dir / "cache" / "cells"
            self.cache = CellCache(cache_dir, enabled=True)
            self.resumable_runner = ResumableRunner(self.artifacts_dir)
        else:
            self.cache = CellCache(Path("/tmp"), enabled=False)  # Disabled cache
            self.resumable_runner = None

    def compute_tensor(
        self, tensor_spec: Dict[str, Any], pruning_config: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """
        Compute a tensor according to specification.

        Args:
            tensor_spec: Tensor specification with name, op, sources
            pruning_config: Optional pruning configuration

        Returns:
            Computed tensor with all cells
        """
        name = tensor_spec["name"]
        op = tensor_spec["op"]
        sources = tensor_spec["sources"]
        matrix_operand = tensor_spec.get("matrix_operand")

        if op != "cross":
            raise ValueError(f"Unsupported operation: {op}")

        # Get source data
        left_source = self._get_source(sources[list(sources.keys())[0]])
        right_source = self._get_source(sources[list(sources.keys())[1]])

        # Get matrix operand for lens derivation
        matrix_for_lens = self._get_matrix_for_lens(matrix_operand)

        # Compute tensor dimensions
        dims = self._compute_tensor_dims(left_source, right_source)

        # Apply pruning if configured
        if pruning_config:
            cell_indices = self._apply_pruning(dims, pruning_config)
        else:
            cell_indices = list(itertools.product(*[range(d) for d in dims]))

        # Get completed cells for resume
        completed_cells = set()
        if self.resume and self.resumable_runner:
            completed_cells = self.resumable_runner.get_completed_cells(name)

        # Compute cells
        cells = {}
        cells_computed = 0
        cells_from_cache = 0
        cells_from_resume = 0

        for idx in cell_indices:
            # Check resume first (saved cell traces)
            if self.resumable_runner:
                # Use canonical cell key from ResumableRunner
                cell_key_resume = "_".join(map(str, idx))
                if cell_key_resume in completed_cells:
                    resumed_result = self.resumable_runner.load_cell_result(name, idx)
                    if resumed_result:
                        cells[idx] = resumed_result
                        cells_from_resume += 1
                        continue

            # Check cache second (disk/memory cache)
            cache_key = self._compute_cache_key(
                name, idx, left_source, right_source, matrix_for_lens
            )
            cached_result = self.cache.get(cache_key)
            if cached_result:
                cells[idx] = cached_result
                cells_from_cache += 1
                continue

            # Compute cell (last resort)
            cell_value = self._compute_tensor_cell(
                name, idx, left_source, right_source, matrix_for_lens, cache_key
            )

            # Save to cache
            self.cache.put(cache_key, cell_value)

            # Save to resume traces
            if self.resumable_runner:
                self.resumable_runner.save_cell_result(name, idx, cell_value)

            cells[idx] = cell_value
            cells_computed += 1

        # Update progress tracking
        if self.resumable_runner:
            total_cells = len(cell_indices)
            completed = cells_computed + cells_from_cache + cells_from_resume
            self.resumable_runner.update_progress(name, completed, total_cells)

        # Log computation stats
        log_info(f"Tensor {name} computation complete:")
        log_info(f"  Cells computed: {cells_computed}")
        log_info(f"  Cells from cache: {cells_from_cache}")
        log_info(f"  Cells from resume: {cells_from_resume}")
        log_info(f"  Total cells: {len(cell_indices)}")

        return {
            "name": name,
            "op": op,
            "dims": dims,
            "cells": cells,
            "sources": sources,
            "matrix_operand": matrix_operand,
            "stats": {
                "cells_computed": cells_computed,
                "cells_from_cache": cells_from_cache,
                "cells_from_resume": cells_from_resume,
                "total_cells": len(cell_indices),
            },
        }

    def get_budget_status(self) -> Optional[Dict[str, Any]]:
        """Get current budget status."""
        if self.budget_tracker:
            return self.budget_tracker.get_status()
        return None

    def save_budget_status(self):
        """Save budget status to artifacts directory."""
        if self.budget_tracker and self.artifacts_dir:
            self.budget_tracker.save_status(self.artifacts_dir)

    def _load_snapshot(self, snapshot_path: Path) -> str:
        """Load Phase 1 snapshot."""
        return snapshot_path.read_text()

    def _load_lens_catalog(self, catalog_path: Path) -> Dict[str, str]:
        """Load precomputed lens catalog."""
        catalog = {}
        with open(catalog_path, "r") as f:
            for line in f:
                entry = json.loads(line)
                lens_id = entry["lens_id"]
                catalog[lens_id] = entry["text"]
        return catalog

    def _get_source(self, source_spec: Dict[str, str]) -> Any:
        """Get source data based on specification."""
        source_type = list(source_spec.values())[0]
        source_name = list(source_spec.keys())[0]

        if source_type == "array":
            # Array R is special case
            if source_name == "R":
                return self._get_array_r()
            else:
                raise ValueError(f"Unknown array: {source_name}")

        elif source_type == "matrix":
            # Get from Phase 1 output
            return self.phase1_output["matrices"][source_name]

        elif source_type == "tensor":
            # Would be previously computed tensor
            # For now, raise error as we compute in order
            raise ValueError(f"Tensor source not yet implemented: {source_name}")

        else:
            raise ValueError(f"Unknown source type: {source_type}")

    def _get_array_r(self) -> List[str]:
        """Get Array R topics."""
        # These would be defined in tensor spec or configuration
        return [
            "Problem Statement",
            "Requirements",
            "Objectives",
            "Methodology",
            "Analysis",
            "Evaluation",
            "Assessment",
            "Implementation",
            "Integration",
        ]

    def _get_matrix_for_lens(self, matrix_name: str) -> Dict[str, Any]:
        """Get matrix to use for lens derivation."""
        if matrix_name == "H":
            # Special case for H (1×1)
            return {
                "rows": ["reflecting"],
                "cols": ["consistency"],
                "station": "Validation",  # Or from P's station
            }
        else:
            return self.phase1_output["matrices"][matrix_name]

    def _compute_tensor_dims(self, left: Any, right: Any) -> List[int]:
        """Compute tensor dimensions from sources."""
        dims = []

        # Left source dimensions
        if isinstance(left, list):
            dims.append(len(left))
        elif isinstance(left, dict):
            if "dims" in left:  # Tensor
                dims.extend(left["dims"])
            else:  # Matrix
                dims.append(len(left.get("rows", [])))
                dims.append(len(left.get("cols", [])))

        # Right source dimensions
        if isinstance(right, dict):
            if "dims" in right:  # Tensor
                dims.extend(right["dims"])
            else:  # Matrix
                dims.append(len(right.get("rows", [])))
                dims.append(len(right.get("cols", [])))

        return dims

    def _apply_pruning(self, dims: List[int], config: Dict[str, Any]) -> List[Tuple[int, ...]]:
        """Apply pruning to reduce cell count."""
        max_pairs = config.get("max_pairs", 64)
        config.get("top_k", 8)

        # Generate all indices
        all_indices = list(itertools.product(*[range(d) for d in dims]))

        # Apply max_pairs limit
        if len(all_indices) > max_pairs:
            # Could implement smarter selection here
            # For now, just take first max_pairs
            all_indices = all_indices[:max_pairs]

        return all_indices

    def _compute_cache_key(
        self,
        tensor_name: str,
        idx: Tuple[int, ...],
        left_source: Any,
        right_source: Any,
        matrix_for_lens: Dict[str, Any],
    ) -> str:
        """Compute complete cache key for a cell including all dependencies."""
        # Get operand values
        left_value = self._get_operand_value(left_source, idx, "left")
        right_value = self._get_operand_value(right_source, idx, "right")

        # Compute operands hash
        operands = {"left": left_value, "right": right_value}
        operands_hash = self.cache.compute_operands_hash(operands)

        # Get lens ID
        lens_id = self._compute_lens_id(matrix_for_lens, idx)

        # Get stable snapshot hash from Phase 1 output (if available)
        snapshot_hash = self._get_snapshot_hash()

        # Get kernel hash from Phase 1 output (if available)
        kernel_hash = self._get_kernel_hash()

        # Get lens catalog digest
        lens_catalog_digest = self._get_lens_catalog_digest()

        return self.cache.compute_cache_key(
            tensor_name=tensor_name,
            indices=idx,
            operands_hash=operands_hash,
            lens_id=lens_id,
            snapshot_hash=snapshot_hash,
            model=self.model,
            temperature=self.temperature,
            top_p=self.top_p,
            kernel_hash=kernel_hash,
            lens_catalog_digest=lens_catalog_digest,
        )

    def _compute_tensor_cell(
        self,
        tensor_name: str,
        idx: Tuple[int, ...],
        left_source: Any,
        right_source: Any,
        matrix_for_lens: Dict[str, Any],
        cache_key: str,
    ) -> Dict[str, Any]:
        """
        Compute a single tensor cell statelessly.

        Args:
            tensor_name: Name of tensor being computed
            idx: Cell index tuple
            left_source: Left operand data
            right_source: Right operand data
            matrix_for_lens: Matrix to derive lens from

        Returns:
            Cell computation result
        """
        # Build system message (Phase 1 snapshot)
        system_message = self.snapshot

        # Get operand values
        left_value = self._get_operand_value(left_source, idx, "left")
        right_value = self._get_operand_value(right_source, idx, "right")

        # Get lens text
        lens_text = self._get_lens_text(matrix_for_lens, idx)

        # Build user message
        user_message = self._build_cell_prompt(tensor_name, idx, left_value, right_value, lens_text)

        # Add JSON contract
        json_contract = self._get_cell_contract(tensor_name)
        full_message = f"{user_message}\n\n{json_contract}"

        # Parse response with repair if needed
        def adapter_call(instructions=None, input=None):
            """Responses API only - no messages support"""
            if not instructions or not input:
                raise ValueError("Must provide instructions and input for Responses API")
            
            # P0-3: Use strict JSON schema for tensor cell responses
            response_format = {
                "type": "json_schema",
                "json_schema": {
                    "name": f"{tensor_name}_cell_response",
                    "description": f"Response schema for {tensor_name} tensor cell computation",
                    "schema": {
                        "type": "object",
                        "properties": {
                            "result": {"type": "string", "description": "Computed cell value"},
                            "reasoning": {"type": "string", "description": "Optional reasoning for the computation"},
                            "hierarchical_path": {"type": "string", "description": "Cell location in tensor structure"}
                        },
                        "required": ["result"],
                        "additionalProperties": False
                    },
                    "strict": True
                }
            }
            
            response = call_responses(
                instructions=instructions,
                input=input,
                response_format=response_format
            )
            # Convert to expected format for repair mechanism
            return {"content": response.get("output_text", "")}, response.get("raw", {}).get("metadata", {})

        # Create schema hint
        schema_hint = create_tensor_cell_schema_hint(tensor_name)

        # Use Responses API format - no messages array
        result, metadata = try_parse_json_or_repair(
            instructions=system_message,
            input_text=full_message,
            adapter_call=adapter_call,
            schema_hint=schema_hint,
            max_repair_attempts=self.max_repair,
        )

        # Track budget (single source of truth)
        if self.budget_tracker:
            self.budget_tracker.record_usage(metadata, self.model)

        # Trace if configured
        if self.tracer:
            self.tracer.trace_stage(
                stage=f"tensor_cell_{tensor_name}",
                context={
                    "tensor": tensor_name,
                    "index": idx,
                    "lens_id": self._compute_lens_id(matrix_for_lens, idx),
                },
                result=result,
                metadata=metadata,
            )

        return {
            "index": idx,
            "value": result.get("value"),
            "left_operand": left_value,
            "right_operand": right_value,
            "lens_text": lens_text,
            "metadata": metadata,
        }

    def _get_snapshot_hash(self) -> str:
        """Get stable snapshot hash from Phase 1 output, fallback to content hash."""
        # Try to get from Phase 1 metadata
        if hasattr(self, "phase1_output") and self.phase1_output:
            meta = self.phase1_output.get("meta", {})
            if "snapshot_hash" in meta:
                return meta["snapshot_hash"]

        # Fallback to hashing the snapshot content (less stable)
        return hashlib.sha256(self.snapshot.encode()).hexdigest()

    def _get_kernel_hash(self) -> str:
        """Get kernel hash from Phase 1 output."""
        if hasattr(self, "phase1_output") and self.phase1_output:
            meta = self.phase1_output.get("meta", {})
            return meta.get("kernel_hash", "unknown_kernel")
        return "unknown_kernel"

    def _get_lens_catalog_digest(self) -> str:
        """Get digest of lens catalog for cache invalidation."""
        if not self.lens_catalog:
            return "no_lens_catalog"

        # Sort keys for deterministic hash
        catalog_str = json.dumps(self.lens_catalog, sort_keys=True, separators=(",", ":"))
        return hashlib.sha256(catalog_str.encode()).hexdigest()

    def _get_operand_value(self, source: Any, idx: Tuple[int, ...], side: str) -> str:
        """Extract operand value from source at index."""
        if side == "left":
            # For left operand, use appropriate indices
            if isinstance(source, list):
                # Array source
                return source[idx[0]]
            elif isinstance(source, dict):
                # Matrix or tensor source
                if "elements" in source:
                    # Matrix
                    row_idx = idx[0] if len(idx) > 0 else 0
                    col_idx = idx[1] if len(idx) > 1 else 0
                    return source["elements"][row_idx][col_idx]
                elif "cells" in source:
                    # Tensor
                    return source["cells"][idx[: len(source["dims"])]]

        elif side == "right":
            # For right operand, use remaining indices
            if isinstance(source, dict):
                if "elements" in source:
                    # Matrix - use last indices
                    n_left_dims = len(idx) - 2  # Assuming matrix is 2D
                    row_idx = idx[n_left_dims] if n_left_dims < len(idx) else 0
                    col_idx = idx[n_left_dims + 1] if n_left_dims + 1 < len(idx) else 0
                    return source["elements"][row_idx][col_idx]

        return "undefined"

    def _get_lens_text(self, matrix: Dict[str, Any], idx: Tuple[int, ...]) -> str:
        """
        Get lens text for cell.

        CRITICAL: Enforces lens lookup rule - must be precomputed.
        """
        # Compute lens ID
        lens_id = self._compute_lens_id(matrix, idx)

        # CRITICAL: Check catalog - fail loudly if missing
        if lens_id in self.lens_catalog:
            return self.lens_catalog[lens_id]

        # CRITICAL: Don't generate fallback - require precomputed lenses
        rows = matrix.get("rows", [])
        cols = matrix.get("cols", [])
        station = matrix.get("station", "Unknown")

        # Map indices to row/col labels for error message
        row_label = "unknown"
        col_label = "unknown"

        n_dims = 2  # Assuming matrix is 2D
        start_idx = len(idx) - n_dims

        if start_idx >= 0 and start_idx + 1 < len(idx):
            row_idx = idx[start_idx]
            col_idx = idx[start_idx + 1]

            if row_idx < len(rows):
                row_label = rows[row_idx]
            if col_idx < len(cols):
                col_label = cols[col_idx]

        # Raise helpful error with CLI command suggestion
        raise ValueError(
            f"Missing lens for triple ({row_label}, {col_label}, {station}) - lens_id: {lens_id}\n"
            f"Run: chirality lenses build --triples artifacts/lenses_triples.json --out artifacts/lens_catalog.jsonl"
        )

    def _compute_lens_id(self, matrix: Dict[str, Any], idx: Tuple[int, ...]) -> str:
        """Compute lens ID for caching."""
        rows = matrix.get("rows", [])
        cols = matrix.get("cols", [])
        station = matrix.get("station", "")

        # Get row/col from index
        n_dims = len(rows) + len(cols)
        start_idx = len(idx) - n_dims

        if start_idx >= 0 and start_idx < len(idx) - 1:
            row_idx = idx[start_idx]
            col_idx = idx[start_idx + 1]

            if row_idx < len(rows) and col_idx < len(cols):
                row = rows[row_idx]
                col = cols[col_idx]
                lens_str = f"{row}|{col}|{station}"
                return hashlib.sha256(lens_str.encode()).hexdigest()

        return hashlib.sha256(station.encode()).hexdigest()

    def _build_cell_prompt(
        self,
        tensor_name: str,
        idx: Tuple[int, ...],
        left_value: str,
        right_value: str,
        lens_text: str,
    ) -> str:
        """Build prompt for cell computation."""
        return f"""
Compute tensor cell {tensor_name}{list(idx)} using semantic cross product.

Left operand: {left_value}
Right operand: {right_value}

Apply the semantic cross product operation to create a hierarchical
semantic relationship between these operands.

{lens_text}

Consider how the left operand provides context that transforms or
extends the meaning of the right operand in this hierarchical structure.
"""

    def _get_cell_contract(self, tensor_name: str) -> str:
        """Get JSON contract for cell response."""
        return (
            f"""Return JSON only: {{"tensor":"{tensor_name}","value":"...","confidence":0.0-1.0}}"""
        )
