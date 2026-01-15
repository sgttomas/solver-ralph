"""
Domain Contracts for Phase-2 Safety

Defines stable JSON schemas for tensor operations (M, W, U, N, Solution).
These contracts must remain stable for Phase-2 branch-wise generation.
"""

from typing import Dict, Any, List, Optional
from pydantic import BaseModel, Field
from enum import Enum


class TensorType(str, Enum):
    """Tensor types for Phase-2 operations."""

    M = "M"  # Methods tensor
    W = "W"  # Workflows tensor
    U = "U"  # Utilities tensor
    N = "N"  # Networks tensor


class ComponentType(str, Enum):
    """Component types in the pipeline."""

    C = "C"  # Combination
    D = "D"  # Definition
    F = "F"  # Framing
    X = "X"  # Expansion
    Z = "Z"  # Validation (shifted)
    E = "E"  # Evaluation


class StationType(str, Enum):
    """Station types for semantic context."""

    REQUIREMENTS = "requirements"
    OBJECTIVES = "objectives"
    VERIFICATION = "verification"
    VALIDATION = "validation"
    EVALUATION = "evaluation"


class TensorM(BaseModel):
    """Methods tensor schema for Phase-2."""

    tensor_type: TensorType = TensorType.M
    dimensions: tuple[int, int] = Field(..., description="Tensor dimensions (rows, cols)")
    components: List[ComponentType] = Field(..., description="Component sequence")
    stations: List[StationType] = Field(..., description="Station sequence")
    metadata: Dict[str, Any] = Field(default_factory=dict)


class TensorW(BaseModel):
    """Workflows tensor schema for Phase-2."""

    tensor_type: TensorType = TensorType.W
    dimensions: tuple[int, int] = Field(..., description="Tensor dimensions (rows, cols)")
    workflow_stages: List[str] = Field(..., description="Workflow stage names")
    dependencies: Dict[str, List[str]] = Field(default_factory=dict)
    metadata: Dict[str, Any] = Field(default_factory=dict)


class TensorU(BaseModel):
    """Utilities tensor schema for Phase-2."""

    tensor_type: TensorType = TensorType.U
    dimensions: tuple[int, int] = Field(..., description="Tensor dimensions (rows, cols)")
    utility_functions: List[str] = Field(..., description="Utility function names")
    parameters: Dict[str, Any] = Field(default_factory=dict)
    metadata: Dict[str, Any] = Field(default_factory=dict)


class TensorN(BaseModel):
    """Networks tensor schema for Phase-2."""

    tensor_type: TensorType = TensorType.N
    dimensions: tuple[int, int] = Field(..., description="Tensor dimensions (rows, cols)")
    network_nodes: List[str] = Field(..., description="Network node identifiers")
    connections: List[tuple[str, str]] = Field(..., description="Node connections")
    metadata: Dict[str, Any] = Field(default_factory=dict)


class SolutionStatement(BaseModel):
    """Final solution statement schema."""

    problem_id: str = Field(..., description="Problem identifier")
    solution_text: str = Field(..., description="Solution statement text")
    confidence: float = Field(ge=0.0, le=1.0, description="Solution confidence")
    supporting_tensors: List[TensorType] = Field(..., description="Supporting tensor evidence")
    metadata: Dict[str, Any] = Field(default_factory=dict)


class Phase2Contract(BaseModel):
    """Complete Phase-2 contract schema."""

    tensor_m: Optional[TensorM] = None
    tensor_w: Optional[TensorW] = None
    tensor_u: Optional[TensorU] = None
    tensor_n: Optional[TensorN] = None
    solution: Optional[SolutionStatement] = None
    generation_metadata: Dict[str, Any] = Field(default_factory=dict)


# Export schemas for external use
__all__ = [
    "TensorType",
    "ComponentType",
    "StationType",
    "TensorM",
    "TensorW",
    "TensorU",
    "TensorN",
    "SolutionStatement",
    "Phase2Contract",
]
