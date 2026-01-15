"""
Fixed canonical matrices for CF14 semantic calculator.

These matrices are constants - they represent the fixed ontological structure
of the Chirality Framework. They are not dynamic or configurable.

This embodies the "semantic calculator" philosophy: we have specific,
unchanging inputs that produce predictable outputs through deterministic
semantic operations.
"""

from ..types import Matrix, Cell


def _create_cell(row: int, col: int, value: str) -> Cell:
    """Helper to create a cell with minimal provenance."""
    return Cell(row=row, col=col, value=value, provenance={"source": "canonical_matrix"})


def _create_matrix_cells(content: list[list[str]]) -> list[list[Cell]]:
    """Convert 2D string array to 2D Cell array."""
    return [
        [_create_cell(row, col, content[row][col]) for col in range(len(content[row]))]
        for row in range(len(content))
    ]


# Fixed canonical Matrix A (3x4)
MATRIX_A = Matrix(
    name="A",
    station="Problem Statement",
    row_labels=["normative", "operative", "iterative"],
    col_labels=["guiding", "applying", "judging", "reflecting"],
    cells=_create_matrix_cells(
        [
            ["objectives", "actions", "benchmarks", "feedback"],
            ["standards", "methods", "criteria", "adaptation"],
            ["developments", "coordination", "evaluation", "refinement"],
        ]
    ),
)

# Fixed canonical Matrix B (4x4)
MATRIX_B = Matrix(
    name="B",
    station="Problem Statement",
    row_labels=["data", "information", "knowledge", "wisdom"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells(
        [
            ["necessary", "sufficient", "complete", "consistent"],
            ["contingent", "actionable", "contextual", "congruent"],
            ["purposeful", "effective", "comprehensive", "coherent"],
            ["constitutive", "optimal", "holistic", "principled"],
        ]
    ),
)

# Fixed canonical Matrix J (3x4) - Truncated B without "wisdom" row
MATRIX_J = Matrix(
    name="J",
    station=None,
    row_labels=["data", "information", "knowledge"],  # No "wisdom" row
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells(
        [
            ["necessary", "sufficient", "complete", "consistent"],
            ["contingent", "actionable", "contextual", "congruent"],
            ["purposeful", "effective", "comprehensive", "coherent"],
        ]
    ),
)

# Matrix C (3x4) - Problem Statement
MATRIX_C = Matrix(
    name="C",
    station="problem statement",
    row_labels=["normative", "operative", "iterative"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - computed from A · B
        ["", "", "", ""],
        ["", "", "", ""],
    ]),
)

# Matrix F (3x4) - Requirements
MATRIX_F = Matrix(
    name="F",
    station="requirements",
    row_labels=["data", "information", "knowledge"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - computed from C ⊙ J
        ["", "", "", ""],
        ["", "", "", ""],
    ]),
)

# Matrix D (3x4) - Objectives
MATRIX_D = Matrix(
    name="D",
    station="objectives",
    row_labels=["normative", "operative", "iterative"],
    col_labels=["guiding", "applying", "judging", "reflecting"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - computed from A + F
        ["", "", "", ""],
        ["", "", "", ""],
    ]),
)

# Matrix K (4x3) - Transpose of D
MATRIX_K = Matrix(
    name="K",
    station=None,
    row_labels=["guiding", "applying", "judging", "reflecting"],
    col_labels=["normative", "operative", "iterative"],
    cells=_create_matrix_cells([
        ["", "", ""],  # Empty cells - transpose(D)
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
    ]),
)

# Matrix X (4x4) - Verification
MATRIX_X = Matrix(
    name="X",
    station="verification",
    row_labels=["guiding", "applying", "judging", "reflecting"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - computed from K · J
        ["", "", "", ""],
        ["", "", "", ""],
        ["", "", "", ""],
    ]),
)

# Matrix Z (4x4) - Validation
MATRIX_Z = Matrix(
    name="Z",
    station="validation",
    row_labels=["guiding", "applying", "judging", "reflecting"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - computed from X with station shift
        ["", "", "", ""],
        ["", "", "", ""],
        ["", "", "", ""],
    ]),
)

# Matrix G (3x4) - First 3 rows of Z
MATRIX_G = Matrix(
    name="G",
    station=None,
    row_labels=["guiding", "applying", "judging"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - Z[0:3, :]
        ["", "", "", ""],
        ["", "", "", ""],
    ]),
)

# Matrix P (1x4) - Fourth row of Z
MATRIX_P = Matrix(
    name="P",
    station=None,
    row_labels=["reflecting"],
    col_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    cells=_create_matrix_cells([
        ["", "", "", ""],  # Empty cells - Z[3, :]
    ]),
)

# Matrix T (4x3) - Transpose of J
MATRIX_T = Matrix(
    name="T",
    station=None,
    row_labels=["necessity (vs contingency)", "sufficiency", "completeness", "consistency"],
    col_labels=["data", "information", "knowledge"],
    cells=_create_matrix_cells([
        ["", "", ""],  # Empty cells - transpose(J)
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
    ]),
)

# Matrix E (3x3) - Evaluation
MATRIX_E = Matrix(
    name="E",
    station="evaluation",
    row_labels=["guiding", "applying", "judging"],
    col_labels=["data", "information", "knowledge"],
    cells=_create_matrix_cells([
        ["", "", ""],  # Empty cells - computed from G · T
        ["", "", ""],
        ["", "", ""],
    ]),
)

def get_canonical_matrix(name: str) -> Matrix:
    """
    Get a canonical matrix by name.

    Args:
        name: Matrix name (A, B, J, C, F, D, K, X, Z, G, P, T, E)

    Returns:
        The canonical matrix

    Raises:
        ValueError: If matrix name is not recognized
    """
    matrices = {
        "A": MATRIX_A, "B": MATRIX_B, "J": MATRIX_J,
        "C": MATRIX_C, "F": MATRIX_F, "D": MATRIX_D,
        "K": MATRIX_K, "X": MATRIX_X, "Z": MATRIX_Z, 
        "G": MATRIX_G, "P": MATRIX_P, "T": MATRIX_T, "E": MATRIX_E
    }

    if name not in matrices:
        available = ", ".join(sorted(matrices.keys()))
        raise ValueError(f"Unknown canonical matrix: {name}. Available: {available}")

    return matrices[name]


def get_matrix_info(name: str) -> dict:
    """
    Get matrix dimension and label information without creating full Matrix object.
    
    Args:
        name: Matrix name (A, B, J, C, F, D, K, X, Z, G, P, T, E)
        
    Returns:
        Dictionary with matrix_id, rows, cols, dimensions, station, row_labels, col_labels
    """
    matrix = get_canonical_matrix(name)
    return {
        "matrix_id": name,
        "rows": len(matrix.row_labels),
        "cols": len(matrix.col_labels),
        "row_labels": matrix.row_labels,
        "col_labels": matrix.col_labels,
        "dimensions": [len(matrix.row_labels), len(matrix.col_labels)],
        "station": matrix.station
    }


def validate_canonical_matrices() -> None:
    """
    Validate that the canonical matrices have the expected structure.

    This is a sanity check to ensure the fixed matrices are properly defined.
    """
    # Validate Matrix A (3x4)
    assert MATRIX_A.shape == (3, 4), f"Matrix A should be 3x4, got {MATRIX_A.shape}"
    assert len(MATRIX_A.row_labels) == 3, "Matrix A should have 3 row labels"
    assert len(MATRIX_A.col_labels) == 4, "Matrix A should have 4 column labels"

    # Validate Matrix B (4x4)
    assert MATRIX_B.shape == (4, 4), f"Matrix B should be 4x4, got {MATRIX_B.shape}"
    assert len(MATRIX_B.row_labels) == 4, "Matrix B should have 4 row labels"
    assert len(MATRIX_B.col_labels) == 4, "Matrix B should have 4 column labels"

    # Validate Matrix J (3x4) - truncated B
    assert MATRIX_J.shape == (3, 4), f"Matrix J should be 3x4, got {MATRIX_J.shape}"
    assert len(MATRIX_J.row_labels) == 3, "Matrix J should have 3 row labels"
    assert len(MATRIX_J.col_labels) == 4, "Matrix J should have 4 column labels"

    # Ensure J is properly truncated B (first 3 rows)
    for i in range(3):
        for j in range(4):
            assert (
                MATRIX_B.cells[i][j].value == MATRIX_J.cells[i][j].value
            ), f"Matrix J cell [{i}][{j}] should match Matrix B"

    # Validate Matrix C (3x4) - Problem Statement
    assert MATRIX_C.shape == (3, 4), f"Matrix C should be 3x4, got {MATRIX_C.shape}"
    assert MATRIX_C.station == "problem statement", f"Matrix C station should be 'problem statement'"
    
    # Validate Matrix F (3x4) - Requirements  
    assert MATRIX_F.shape == (3, 4), f"Matrix F should be 3x4, got {MATRIX_F.shape}"
    assert MATRIX_F.station == "requirements", f"Matrix F station should be 'requirements'"
    
    # Validate Matrix D (3x4) - Objectives
    assert MATRIX_D.shape == (3, 4), f"Matrix D should be 3x4, got {MATRIX_D.shape}"
    assert MATRIX_D.station == "objectives", f"Matrix D station should be 'objectives'"
    
    # Validate Matrix K (4x3) - Transpose of D
    assert MATRIX_K.shape == (4, 3), f"Matrix K should be 4x3, got {MATRIX_K.shape}"
    assert MATRIX_K.station is None, f"Matrix K should not have a station (transpose)"
    
    # Validate Matrix X (4x4) - Verification
    assert MATRIX_X.shape == (4, 4), f"Matrix X should be 4x4, got {MATRIX_X.shape}"
    assert MATRIX_X.station == "verification", f"Matrix X station should be 'verification'"
    
    # Validate Matrix Z (4x4) - Validation
    assert MATRIX_Z.shape == (4, 4), f"Matrix Z should be 4x4, got {MATRIX_Z.shape}"
    assert MATRIX_Z.station == "validation", f"Matrix Z station should be 'validation'"
    
    # Validate Matrix G (3x4) - First 3 rows of Z
    assert MATRIX_G.shape == (3, 4), f"Matrix G should be 3x4, got {MATRIX_G.shape}"
    assert MATRIX_G.station is None, f"Matrix G should not have a station (extraction)"
    
    # Validate Matrix P (1x4) - Fourth row of Z
    assert MATRIX_P.shape == (1, 4), f"Matrix P should be 1x4, got {MATRIX_P.shape}"
    assert MATRIX_P.station is None, f"Matrix P should not have a station (extraction)"
    
    # Validate Matrix T (4x3) - Transpose of J
    assert MATRIX_T.shape == (4, 3), f"Matrix T should be 4x3, got {MATRIX_T.shape}"
    assert MATRIX_T.station is None, f"Matrix T should not have a station (transpose)"
    
    # Validate Matrix E (3x3) - Evaluation
    assert MATRIX_E.shape == (3, 3), f"Matrix E should be 3x3, got {MATRIX_E.shape}"
    assert MATRIX_E.station == "evaluation", f"Matrix E station should be 'evaluation'"


if __name__ == "__main__":
    # Run validation if script is executed directly
    validate_canonical_matrices()
    print("✓ All canonical matrices validated successfully")
    print(f"✓ Matrix A: {MATRIX_A.shape} - {MATRIX_A.station}")
    print(f"✓ Matrix B: {MATRIX_B.shape} - {MATRIX_B.station}")
    print(f"✓ Matrix J: {MATRIX_J.shape} - {MATRIX_J.station}")
    print(f"✓ Matrix C: {MATRIX_C.shape} - {MATRIX_C.station}")
    print(f"✓ Matrix F: {MATRIX_F.shape} - {MATRIX_F.station}")
    print(f"✓ Matrix D: {MATRIX_D.shape} - {MATRIX_D.station}")
    print(f"✓ Matrix K: {MATRIX_K.shape} - {MATRIX_K.station}")
    print(f"✓ Matrix X: {MATRIX_X.shape} - {MATRIX_X.station}")
    print(f"✓ Matrix Z: {MATRIX_Z.shape} - {MATRIX_Z.station}")
    print(f"✓ Matrix G: {MATRIX_G.shape} - {MATRIX_G.station}")
    print(f"✓ Matrix P: {MATRIX_P.shape} - {MATRIX_P.station}")
    print(f"✓ Matrix T: {MATRIX_T.shape} - {MATRIX_T.station}")
    print(f"✓ Matrix E: {MATRIX_E.shape} - {MATRIX_E.station}")
