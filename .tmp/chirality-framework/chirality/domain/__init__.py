"""
Domain Layer - Pure Business Logic

Contains the core business rules, data structures, and domain logic
for the Chirality Framework semantic calculator.

No dependencies on infrastructure or application layers.
"""

from .types import Cell, Matrix, RichResult
from .matrices.canonical import MATRIX_A, MATRIX_B, MATRIX_J
from .validation import FrameworkValidationError, validate_matrix, validate_cell

__all__ = [
    # Core types
    "Cell",
    "Matrix",
    "RichResult",
    # Canonical matrices
    "MATRIX_A",
    "MATRIX_B",
    "MATRIX_J",
    # Validation
    "FrameworkValidationError",
    "validate_matrix",
    "validate_cell",
]
