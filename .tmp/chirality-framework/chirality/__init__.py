"""
Chirality Framework - A Semantic Calculator

This package provides a direct implementation of the Chirality Framework, a fixed,
canonical algorithm for structured problem-solving. It functions as a "semantic
calculator" that executes a specific sequence of semantic and structural matrix
operations to traverse a "semantic valley" from problem to evaluation.

The primary user interface is the command-line tool, which provides orchestration
for computing the entire pipeline and visualizing the results.
"""

# Read version from VERSION.md (single source of truth)
from pathlib import Path

try:
    version_path = Path(__file__).parent.parent / "VERSION.md"
    with open(version_path, "r") as f:
        __version__ = f.readline().split("—")[0].strip()
except Exception:
    __version__ = "0.0.0"  # Fallback version
__author__ = "Chirality Framework Team"

from .domain.types import Cell, Matrix
from .domain.matrices.canonical import MATRIX_A, MATRIX_B, MATRIX_J
from .domain.validation import FrameworkValidationError, validate_matrix, validate_cell
from .infrastructure.monitoring.tracer import JSONLTracer

# QUARANTINED: EchoResolver and other legacy resolvers are no longer exported
# Use the CLI interface instead: python3 -m chirality.interfaces.cli

__all__ = [
    # Core types
    "Cell",
    "Matrix",
    # Canonical matrices
    "MATRIX_A",
    "MATRIX_B",
    "MATRIX_J",
    # Validation
    "FrameworkValidationError",
    "validate_matrix",
    "validate_cell",
    # Tracing
    "JSONLTracer",
    # Note: Legacy resolvers (EchoResolver, etc.) have been quarantined
    # Use CLI interface instead: python3 -m chirality.interfaces.cli
]
