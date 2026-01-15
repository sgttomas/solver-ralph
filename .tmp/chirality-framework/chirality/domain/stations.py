"""
Canonical Station Definitions for Chirality Framework v15.0.1

Defines the 11-station semantic valley with proper station names,
operations, and matrix assignments.
"""

from typing import Dict, List, Tuple, Optional

# Canonical Station Names
STATION_PROBLEM_STATEMENT = "Problem Statement"
STATION_PROBLEM_REQUIREMENTS = "Problem Requirements"
STATION_SOLUTION_OBJECTIVES = "Solution Objectives"
STATION_VERIFICATION_FRAMEWORK = "Verification Framework"
STATION_VALIDATION = "Validation"
STATION_EVALUATION = "Evaluation"
STATION_ASSESSMENT = "Assessment"
STATION_IMPLEMENTATION = "Implementation"
STATION_INTEGRATION = "Integration"
STATION_REFLECTION = "Reflection"
STATION_RESOLUTION = "Resolution"

# Station to Matrix Mapping
STATION_MATRICES: Dict[str, List[str]] = {
    STATION_PROBLEM_STATEMENT: ["A", "B"],
    STATION_PROBLEM_REQUIREMENTS: ["C"],  # C = A * B
    STATION_SOLUTION_OBJECTIVES: ["D", "F"],  # D = synthesis(A, F), F = J ⊙ C
    STATION_VERIFICATION_FRAMEWORK: ["X", "K", "J"],  # X = K * J
    STATION_VALIDATION: ["Z"],  # Z from X
    STATION_EVALUATION: ["E", "G", "T"],  # E = G * T
    STATION_ASSESSMENT: ["M", "R"],  # M = R × E
    STATION_IMPLEMENTATION: ["W"],  # W = M × X
    STATION_INTEGRATION: ["U", "P"],  # U = W × P
    STATION_REFLECTION: ["N", "H"],  # N = U × H
    STATION_RESOLUTION: ["N"],  # Final synthesis
}

# Complete 11-Station Definition
STATIONS: List[Tuple[int, str, str, str]] = [
    (1, STATION_PROBLEM_STATEMENT, "[A], [B]", "Establish problem axioms and decision basis"),
    (
        2,
        STATION_PROBLEM_REQUIREMENTS,
        "[A] * [B] = [C]",
        "Generate requirements through semantic multiplication",
    ),
    (
        3,
        STATION_SOLUTION_OBJECTIVES,
        "[A] + [F] = [D]",
        "Synthesize objectives combining axioms and functions",
    ),
    (
        4,
        STATION_VERIFICATION_FRAMEWORK,
        "[K] * [J] = [X]",
        "Establish verification criteria and methods",
    ),
    (5, STATION_VALIDATION, "[X] -> [Z]", "Transform verification into validation context"),
    (
        6,
        STATION_EVALUATION,
        "[G] * [T] = [E]",
        "Evaluate against data/information/knowledge criteria",
    ),
    (7, STATION_ASSESSMENT, "[R] × [E] = [M]", "Assess deliverables through evaluation framework"),
    (
        8,
        STATION_IMPLEMENTATION,
        "[M] × [X] = [W]",
        "Apply verification to assessment for implementation",
    ),
    (
        9,
        STATION_INTEGRATION,
        "[W] × [P] = [U]",
        "Integrate implementation with validity parameters",
    ),
    (10, STATION_REFLECTION, "[U] × [H] = [N]", "Apply consistency check through reflection"),
    (11, STATION_RESOLUTION, "Final [N]", "Complete knowledge generation cycle"),
]


def get_station_for_matrix(matrix_name: str) -> str:
    """
    Get the canonical station name for a given matrix.

    Args:
        matrix_name: Name of the matrix (e.g., "A", "C", "F")

    Returns:
        The station name where this matrix belongs

    Raises:
        ValueError: If matrix is not found in any station
    """
    for station, matrices in STATION_MATRICES.items():
        if matrix_name in matrices:
            return station
    raise ValueError(f"Matrix {matrix_name} not found in any station")


def get_station_index(station_name: str) -> int:
    """
    Get the 1-based index of a station in the semantic valley.

    Args:
        station_name: Name of the station

    Returns:
        1-based station index (1-11)

    Raises:
        ValueError: If station name is not recognized
    """
    for idx, name, _, _ in STATIONS:
        if name == station_name:
            return idx
    raise ValueError(f"Unknown station: {station_name}")


def format_valley_summary(current_station: Optional[str] = None) -> str:
    """
    Format the semantic valley summary with optional current station highlighting.

    Args:
        current_station: Name of the current station to highlight with brackets

    Returns:
        Formatted valley summary string
    """
    names = []
    for _, name, _, _ in STATIONS[:4]:  # Show first 4 stations for brevity
        if name == current_station:
            names.append(f"[{name}]")
        else:
            names.append(name)

    # Add ellipsis if showing partial valley
    if len(STATIONS) > 4:
        names.append("...")

    return " → ".join(names)
