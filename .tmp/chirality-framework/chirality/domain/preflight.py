"""
Shared preflight validation utilities for matrix operations.

These utilities perform validation checks before semantic operations to ensure
matrices are compatible for the intended operation type:
- preflight_hadamard: Element-wise operations (⊙) require identical shape and labels
- preflight_dot: Matrix multiplication (·) requires conformable dimensions
- preflight_addition: Addition (+) requires identical shapes but labels can differ
"""

from typing import Dict, List, Any


class PreflightError(ValueError):
    """Raised when preflight validation fails."""
    pass


def validate_matrix_structure(matrix: Dict[str, Any], expected_name: str) -> None:
    """
    Validate basic matrix structure has required fields.
    
    Args:
        matrix: Matrix data structure to validate
        expected_name: Expected name for error messages
        
    Raises:
        PreflightError: If matrix structure is invalid
    """
    required_fields = ["name"]
    for field in required_fields:
        if field not in matrix:
            raise PreflightError(f"Matrix {expected_name} missing required field: {field}")
    
    # Must have either rows/cols or elements structure
    has_labels = "rows" in matrix and "cols" in matrix
    has_elements = "elements" in matrix
    
    if not has_labels and not has_elements:
        raise PreflightError(f"Matrix {expected_name} must have either rows/cols or elements structure")


def extract_matrix_info(matrix: Dict[str, Any]) -> Dict[str, Any]:
    """
    Extract row and column information from matrix structure.
    
    Handles both direct rows/cols labels and inferred from elements.
    
    Args:
        matrix: Matrix data structure
        
    Returns:
        Dict with 'rows', 'cols', 'row_count', 'col_count'
    """
    if "rows" in matrix and "cols" in matrix:
        rows = matrix["rows"]
        cols = matrix["cols"]
    elif "elements" in matrix:
        elements = matrix["elements"]
        if not elements or not elements[0]:
            raise PreflightError(f"Matrix {matrix.get('name', 'unknown')} has empty elements")
        rows = [f"row_{i}" for i in range(len(elements))]
        cols = [f"col_{j}" for j in range(len(elements[0]))]
    else:
        raise PreflightError(f"Matrix {matrix.get('name', 'unknown')} missing row/col information")
    
    return {
        "rows": rows,
        "cols": cols, 
        "row_count": len(rows),
        "col_count": len(cols)
    }


def preflight_hadamard(matrix_a: Dict[str, Any], matrix_b: Dict[str, Any]) -> None:
    """
    Validates strict label+shape equality for element-wise operations.
    
    Element-wise operations (⊙) like Matrix F = C ⊙ J require:
    - Identical dimensions (row count and column count)
    - Identical row labels in same order
    - Identical column labels in same order
    
    Args:
        matrix_a: First matrix for element-wise operation
        matrix_b: Second matrix for element-wise operation
        
    Raises:
        PreflightError: If matrices are not compatible for element-wise operation
    """
    # Validate basic structure
    validate_matrix_structure(matrix_a, matrix_a.get("name", "A"))
    validate_matrix_structure(matrix_b, matrix_b.get("name", "B"))
    
    # Extract matrix information
    info_a = extract_matrix_info(matrix_a)
    info_b = extract_matrix_info(matrix_b)
    
    # Check dimensions
    if info_a["row_count"] != info_b["row_count"]:
        raise PreflightError(
            f"Element-wise operation requires equal row count: "
            f"{matrix_a['name']} has {info_a['row_count']}, "
            f"{matrix_b['name']} has {info_b['row_count']}"
        )
    
    if info_a["col_count"] != info_b["col_count"]:
        raise PreflightError(
            f"Element-wise operation requires equal column count: "
            f"{matrix_a['name']} has {info_a['col_count']}, "
            f"{matrix_b['name']} has {info_b['col_count']}"
        )
    
    # Check row labels match exactly
    if info_a["rows"] != info_b["rows"]:
        raise PreflightError(
            f"Element-wise operation requires identical row labels: "
            f"{matrix_a['name']} has {info_a['rows']}, "
            f"{matrix_b['name']} has {info_b['rows']}"
        )
    
    # Check column labels match exactly  
    if info_a["cols"] != info_b["cols"]:
        raise PreflightError(
            f"Element-wise operation requires identical column labels: "
            f"{matrix_a['name']} has {info_a['cols']}, "
            f"{matrix_b['name']} has {info_b['cols']}"
        )


def preflight_dot(left_matrix: Dict[str, Any], right_matrix: Dict[str, Any]) -> None:
    """
    Validates conformability for matrix multiplication.
    
    Matrix multiplication requires that the number of columns in the left matrix
    equals the number of rows in the right matrix. Label matching is not required
    for mathematical conformability.
    
    Args:
        left_matrix: Left operand matrix
        right_matrix: Right operand matrix
        
    Raises:
        PreflightError: If matrices are not conformable for multiplication
    """
    # Validate basic structure
    validate_matrix_structure(left_matrix, left_matrix.get("name", "Left"))
    validate_matrix_structure(right_matrix, right_matrix.get("name", "Right"))
    
    # Extract matrix information
    left_info = extract_matrix_info(left_matrix)
    right_info = extract_matrix_info(right_matrix)
    
    # Check conformable dimensions
    if left_info["col_count"] != right_info["row_count"]:
        raise PreflightError(
            f"Matrix multiplication requires conformable dimensions: "
            f"{left_matrix['name']} has {left_info['col_count']} columns, "
            f"{right_matrix['name']} has {right_info['row_count']} rows"
        )


def preflight_addition(matrix_a: Dict[str, Any], matrix_b: Dict[str, Any]) -> None:
    """
    Validates shape compatibility for semantic addition.
    
    Matrix addition (+) like Matrix D = A + F requires:
    - Identical dimensions (row count and column count)
    - Row/column labels can differ (unlike element-wise product)
    
    Args:
        matrix_a: First matrix for addition
        matrix_b: Second matrix for addition
        
    Raises:
        PreflightError: If matrices have incompatible shapes for addition
    """
    # Validate basic structure
    validate_matrix_structure(matrix_a, matrix_a.get("name", "A"))
    validate_matrix_structure(matrix_b, matrix_b.get("name", "B"))
    
    # Extract matrix information
    info_a = extract_matrix_info(matrix_a)
    info_b = extract_matrix_info(matrix_b)
    
    # Check dimensions (labels can differ for addition)
    if info_a["row_count"] != info_b["row_count"]:
        raise PreflightError(
            f"Matrix addition requires equal row count: "
            f"{matrix_a['name']} has {info_a['row_count']}, "
            f"{matrix_b['name']} has {info_b['row_count']}"
        )
    
    if info_a["col_count"] != info_b["col_count"]:
        raise PreflightError(
            f"Matrix addition requires equal column count: "
            f"{matrix_a['name']} has {info_a['col_count']}, "
            f"{matrix_b['name']} has {info_b['col_count']}"
        )