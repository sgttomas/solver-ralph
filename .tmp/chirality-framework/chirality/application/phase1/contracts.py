"""
Phase 1 Output Contracts.

Pydantic models for strict validation of Phase 1 outputs.
Ensures consistent schema for Phase 2 and exporters.
"""

from typing import List, Dict, Optional, Literal, Union
from pydantic import BaseModel, Field


Step = Literal[
    "base", "mechanical", "interpreted", "lenses", "lensed", "transpose", "shifted", "constructed"
]


class Meta(BaseModel):
    """Metadata for Phase 1 run."""

    kernel_hash: str
    snapshot_hash: str
    model: str
    timestamp: Optional[str] = None
    token_count: Optional[int] = None


class Matrix(BaseModel):
    """Matrix specification with validation."""

    name: str
    station: str
    rows: List[str]
    cols: List[str]
    elements: List[List[str]]
    step: Step
    op: Optional[str] = None
    # Optional: keep computed lenses for auditing
    lenses: Optional[List[List[str]]] = None


class Principles(BaseModel):
    """Principles extracted from validation."""

    from_: str = Field(alias="from")
    items: List[str]


class MatrixSnapshot(BaseModel):
    """
    Snapshot model matching the exact schema specification.
    
    This is the precise model for CLI output as specified:
    - name: specific matrix names only
    - build: dict with stage keys (combinatorial, interpreted, lenses, lens_interpreted)  
    - principles: only for Matrix Z
    - transform: only for transpose operations (K, T)
    """
    
    name: Literal["A", "B", "C", "J", "F", "D", "K", "X", "Z", "G", "P", "T", "E"]
    station: str
    rows: List[str]
    cols: List[str]
    dependencies: List[str] = []
    build: Dict[str, List[List[str]]]  # keys: combinatorial, interpreted, lenses, lens_interpreted
    principles: Optional[List[str]] = None  # only for Z
    transform: Optional[Literal["transpose"]] = None  # for K/T


class Phase1Snapshot(BaseModel):
    """Complete Phase 1 snapshot with all matrices."""
    
    meta: Meta
    matrices: Dict[str, MatrixSnapshot]


class Phase1Output(BaseModel):
    """Complete Phase 1 output with strict validation."""

    meta: Meta
    matrices: Dict[str, Matrix]
    principles: Principles

    model_config = {"populate_by_name": True}
