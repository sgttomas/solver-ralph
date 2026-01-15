"""
Canonical Provenance Schema for Chirality Framework v16.0.0

Defines the standard structure for cell provenance tracking
to ensure consistency between operations and validation.
"""

from typing import Dict, Any, List, Optional
from datetime import datetime, timezone


def create_provenance(
    operation: str,
    coordinates: str,
    stage_data: Dict[str, Any],
    sources: Optional[List[str]] = None,
    traced: bool = False,
    **extras,
) -> Dict[str, Any]:
    """
    Create a canonical provenance dictionary for a cell.

    Args:
        operation: Operation name (e.g., "compute_C", "compute_F", "compute_D")
        coordinates: String representation of cell position (e.g., "(Normative, Determinacy)")
        stage_data: Dictionary containing stage-specific data (stage_1_*, stage_2_*, etc.)
        sources: List of source matrix names used in computation
        traced: Whether this operation was traced
        **extras: Additional provenance fields (e.g., "problem" for D synthesis)

    Returns:
        Canonical provenance dictionary
    """
    provenance = {
        # Required fields for validation
        "operation": operation,
        "sources": sources or [],
        "timestamp": datetime.now(timezone.utc).isoformat(),
        # Core tracking fields
        "coordinates": coordinates,
        "traced": traced,
        # Stage-specific data
        **stage_data,
        # Additional fields
        **extras,
    }

    return provenance


# Canonical provenance field definitions
REQUIRED_FIELDS = ["operation", "sources", "timestamp"]
CORE_FIELDS = ["coordinates", "traced"]
# Universal provenance structure for all matrices
STAGE_FIELDS = {
    "compute_C": [
        "stage_1_construct",
        "stage_2_semantic",
        "stage_3_column_lensed",
        "stage_4_row_lensed",
        "stage_5_final_synthesis",
    ],
    "compute_F": [
        "stage_1_construct",
        "stage_2_semantic",
        "stage_3_column_lensed",
        "stage_4_row_lensed",
        "stage_5_final_synthesis",
    ],
    "compute_D": [
        "stage_1_construct",
        "stage_2_semantic",
        "stage_3_column_lensed",
        "stage_4_row_lensed",
        "stage_5_final_synthesis",
    ],
    "compute_X": [
        "stage_1_construct",
        "stage_2_semantic",
        "stage_3_column_lensed",
        "stage_4_row_lensed",
        "stage_5_final_synthesis",
    ],
    "compute_E": [
        "stage_1_construct",
        "stage_2_semantic",
        "stage_3_column_lensed",
        "stage_4_row_lensed",
        "stage_5_final_synthesis",
    ],
}
