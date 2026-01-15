"""
Validation rules for Chirality Framework v16.0.0 semantic structures.

Enforces dimensional constraints and operation sequencing.
"""

from typing import List
from .types import Cell, Matrix


class FrameworkValidationError(ValueError):
    """Raised when framework validation rules are violated."""

    pass


def validate_cell(cell: Cell) -> List[str]:
    """
    Validate a cell structure for the framework's simplified types.

    Checks only fields that actually exist on Cell:
    - row/col are non-negative integers
    - value is a non-empty string
    - provenance is a dict (may be empty)
    """
    errors: List[str] = []

    # row/col
    if not isinstance(cell.row, int) or cell.row < 0:
        errors.append(f"Invalid row position: {cell.row}")
    if not isinstance(cell.col, int) or cell.col < 0:
        errors.append(f"Invalid column position: {cell.col}")

    # value
    if not isinstance(cell.value, str) or not cell.value.strip():
        errors.append("Cell value must be a non-empty string")

    # provenance
    if not isinstance(cell.provenance, dict):
        errors.append("Cell provenance must be a dict")

    return errors


def validate_matrix(matrix: Matrix) -> List[str]:
    """
    Validate a matrix structure for the framework's simplified types.

    Checks presence and coherence of:
    - name (str), station (str)
    - row_labels/col_labels length matches cells grid
    - cells 2D list shape consistency
    """
    errors: List[str] = []

    # Basic identity
    if not isinstance(matrix.name, str) or not matrix.name:
        errors.append("Matrix missing name")
    if not isinstance(matrix.station, str) or not matrix.station:
        errors.append("Matrix missing station")

    # Labels
    if not isinstance(matrix.row_labels, list) or not all(
        isinstance(x, str) for x in matrix.row_labels
    ):
        errors.append("row_labels must be list[str]")
    if not isinstance(matrix.col_labels, list) or not all(
        isinstance(x, str) for x in matrix.col_labels
    ):
        errors.append("col_labels must be list[str]")

    rows, cols = matrix.shape
    if rows <= 0 or cols <= 0:
        errors.append(f"Invalid dimensions: {matrix.shape}")

    # Cells grid
    if not isinstance(matrix.cells, list) or len(matrix.cells) != rows:
        errors.append("cells must be a 2D list with len == number of rows")
    else:
        for r, row in enumerate(matrix.cells):
            if not isinstance(row, list) or len(row) != cols:
                errors.append(
                    f"row {r} length mismatch: expected {cols}, got {len(row) if isinstance(row, list) else 'not a list'}"
                )
                break

    # Label vs shape coherence
    if isinstance(matrix.row_labels, list) and len(matrix.row_labels) != rows:
        errors.append("row_labels length does not match number of rows")
    if isinstance(matrix.col_labels, list) and len(matrix.col_labels) != cols:
        errors.append("col_labels length does not match number of columns")

    # Validate cells in 2D structure
    cell_positions = set()
    for i, row in enumerate(matrix.cells):
        if not isinstance(row, list):
            errors.append(f"Row {i} is not a list")
            continue
        for j, cell in enumerate(row):
            # Validate individual cell
            cell_errors = validate_cell(cell)
            errors.extend([f"Cell ({i},{j}): {e}" for e in cell_errors])

            # Check that cell position matches its array indices
            if cell.row != i or cell.col != j:
                errors.append(
                    f"Cell at [{i}][{j}] has mismatched position: ({cell.row}, {cell.col})"
                )

            # Check bounds (redundant but thorough)
            if cell.row >= rows or cell.col >= cols:
                errors.append(f"Cell ({i},{j}) out of bounds: ({cell.row}, {cell.col})")

            # Check duplicates
            pos = (cell.row, cell.col)
            if pos in cell_positions:
                errors.append(f"Duplicate cell at position {pos}")
            cell_positions.add(pos)

    return errors


def validate_provenance(cell: Cell) -> List[str]:
    """
    Validate provenance tracking for a cell.

    Validates strictly against the canonical provenance schema.

    Args:
        cell: Cell to validate

    Returns:
        List of validation errors
    """
    errors = []

    if not cell.provenance:
        errors.append("Cell missing provenance")
        return errors

    # Check required provenance fields (canonical schema)
    if "operation" not in cell.provenance:
        errors.append("Provenance missing operation type")

    if "sources" not in cell.provenance:
        errors.append("Provenance missing sources list")
    elif not isinstance(cell.provenance["sources"], list):
        errors.append("Provenance sources must be a list")

    if "timestamp" not in cell.provenance:
        errors.append("Provenance missing timestamp")

    # Validate operation-specific fields if operation is known
    operation = cell.provenance.get("operation")
    # Combined lensing provenance validation for all matrices (new architecture)
    if operation in ["compute_C", "compute_F", "compute_D", "compute_X", "compute_E"]:
        required_fields = ["stage_1_construct", "stage_2_semantic", "stage_3_combined_lensed"]
        for field in required_fields:
            if field not in cell.provenance:
                errors.append(f"{operation} provenance missing {field}")
    elif operation == "compute_Z":
        # Matrix Z uses lean 2-stage provenance structure
        required_fields = ["stage_1_construct", "stage_2_semantic", "stage_3_combined_lensed"]
        for field in required_fields:
            if field not in cell.provenance:
                errors.append(f"{operation} provenance missing {field}")
        # Z should have empty stage_3_combined_lensed
        if cell.provenance.get("stage_3_combined_lensed") != {}:
            errors.append("compute_Z should have empty stage_3_combined_lensed")

    return errors
