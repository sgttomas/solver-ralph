"""
Lens Derivation System.

Derives lens triples from Phase 1 output and Phase 2 tensor specifications.
Creates the (row, col, station) combinations that need lens text generation.
"""

import json
import hashlib
from typing import Dict, List, Any
from pathlib import Path


class LensDeriver:
    """
    Derives all lens triples needed for Phase 1 and Phase 2 operations.

    Phase 1: All (row × col × station) triples for each matrix used in lensing
    Phase 2: Per tensor, use the matrix operand to derive triples
    """

    def __init__(self, kernel_hash: str, model: str, prompt_version: str = "v1"):
        """
        Initialize lens deriver.

        Args:
            kernel_hash: Hash of prompt assets for provenance
            model: LLM model identifier
            prompt_version: Version of prompt templates
        """
        self.kernel_hash = kernel_hash
        self.model = model
        self.prompt_version = prompt_version

    def derive_phase1_lenses(self, phase1_output: Dict[str, Any]) -> List[Dict[str, Any]]:
        """
        Derive lens triples needed for Phase 1 operations.

        Args:
            phase1_output: Phase 1 aggregated output

        Returns:
            List of lens triple specifications
        """
        lenses = []
        matrices = phase1_output.get("matrices", {})

        # Matrices that use lensing in Phase 1
        lensed_matrices = ["C", "F", "D", "X", "E"]

        for matrix_name in lensed_matrices:
            if matrix_name in matrices:
                matrix = matrices[matrix_name]
                matrix_lenses = self._derive_matrix_lenses(matrix_name, matrix)
                lenses.extend(matrix_lenses)

        return lenses

    def derive_phase2_lenses(
        self, phase1_output: Dict[str, Any], tensor_spec: Dict[str, Any]
    ) -> List[Dict[str, Any]]:
        """
        Derive lens triples needed for Phase 2 tensor operations.

        Args:
            phase1_output: Phase 1 aggregated output
            tensor_spec: Phase 2 tensor specification

        Returns:
            List of lens triple specifications
        """
        lenses = []
        matrices = phase1_output.get("matrices", {})

        # Get tensor specifications
        tensors = tensor_spec.get("tensors", [])

        for tensor in tensors:
            matrix_operand = tensor.get("matrix_operand")
            if matrix_operand:
                if matrix_operand == "H":
                    # Special case for H (1×1)
                    h_matrix = {
                        "rows": ["reflecting"],
                        "cols": ["consistency"],
                        "station": "Validation",  # Or derive from P
                    }
                    matrix_lenses = self._derive_matrix_lenses("H", h_matrix)
                else:
                    # Use matrix from Phase 1
                    if matrix_operand in matrices:
                        matrix = matrices[matrix_operand]
                        matrix_lenses = self._derive_matrix_lenses(matrix_operand, matrix)
                    else:
                        continue

                lenses.extend(matrix_lenses)

        return lenses

    def _derive_matrix_lenses(
        self, matrix_name: str, matrix: Dict[str, Any]
    ) -> List[Dict[str, Any]]:
        """
        Derive all lens triples for a matrix.

        Args:
            matrix_name: Name of the matrix
            matrix: Matrix specification dict

        Returns:
            List of lens specifications
        """
        lenses = []
        rows = matrix.get("rows", [])
        cols = matrix.get("cols", [])
        station = matrix.get("station", "Unknown")

        # Generate all row × col × station combinations
        for i, row in enumerate(rows):
            for j, col in enumerate(cols):
                lens_id = self._compute_lens_id(row, col, station)

                lens_spec = {
                    "lens_id": lens_id,
                    "row": row,
                    "col": col,
                    "station": station,
                    "matrix_source": matrix_name,
                    "index": [i, j],
                    "kernel_hash": self.kernel_hash,
                    "model": self.model,
                    "prompt_version": self.prompt_version,
                }

                lenses.append(lens_spec)

        return lenses

    def _compute_lens_id(self, row: str, col: str, station: str) -> str:
        """
        Compute unique lens ID for caching.

        Args:
            row: Row ontology label
            col: Column ontology label
            station: Station context

        Returns:
            SHA256 hash as lens ID
        """
        lens_str = f"{row}|{col}|{station}|{self.kernel_hash}|{self.model}|{self.prompt_version}"
        return hashlib.sha256(lens_str.encode()).hexdigest()

    def deduplicate_lenses(self, lenses: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """
        Remove duplicate lens specifications.

        Args:
            lenses: List of lens specifications

        Returns:
            Deduplicated list
        """
        seen = set()
        unique_lenses = []

        for lens in lenses:
            lens_id = lens["lens_id"]
            if lens_id not in seen:
                seen.add(lens_id)
                unique_lenses.append(lens)

        return unique_lenses

    def save_lens_triples(self, lenses: List[Dict[str, Any]], output_path: Path):
        """
        Save lens triple specifications to JSON file.

        Args:
            lenses: List of lens specifications
            output_path: Path to write JSON file
        """
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

        output_data = {
            "lens_count": len(lenses),
            "kernel_hash": self.kernel_hash,
            "model": self.model,
            "prompt_version": self.prompt_version,
            "generated_at": "2025-09-08T12:00:00Z",  # Would use actual timestamp
            "lenses": lenses,
        }

        with open(output_path, "w") as f:
            json.dump(output_data, f, indent=2)


def derive_all_lenses(
    phase1_output_path: Path,
    tensor_spec_path: Path,
    kernel_hash: str,
    model: str,
    output_path: Path,
) -> int:
    """
    Convenience function to derive all lenses for Phase 1 and Phase 2.

    Args:
        phase1_output_path: Path to phase1_output.json
        tensor_spec_path: Path to tensor_spec.json
        kernel_hash: Kernel hash for provenance
        model: LLM model identifier
        output_path: Path to write lenses_triples.json

    Returns:
        Total number of unique lenses derived
    """
    # Load inputs
    with open(phase1_output_path, "r") as f:
        phase1_output = json.load(f)

    with open(tensor_spec_path, "r") as f:
        tensor_spec = json.load(f)

    # Derive lenses
    deriver = LensDeriver(kernel_hash, model)

    phase1_lenses = deriver.derive_phase1_lenses(phase1_output)
    phase2_lenses = deriver.derive_phase2_lenses(phase1_output, tensor_spec)

    # Combine and deduplicate
    all_lenses = phase1_lenses + phase2_lenses
    unique_lenses = deriver.deduplicate_lenses(all_lenses)

    # Save
    deriver.save_lens_triples(unique_lenses, output_path)

    return len(unique_lenses)
