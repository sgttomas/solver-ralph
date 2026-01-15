"""
Simplified types for Chirality Framework semantic calculator.

Contains only the essential data structures: Cell, Matrix.
All abstractions removed - this is a fixed algorithm, not a flexible framework.
"""

from typing import Any, Dict, List, Optional
from dataclasses import dataclass, field
from .pricing import get_model_pricing


@dataclass
class RichResult:
    """
    Structured result object containing both text output and associated metadata.

    Used by resolver methods to return comprehensive information about
    LLM operations, including the resolved text and all metadata needed
    for provenance tracking and Neo4j export.

    Attributes:
        text: The final resolved text from the LLM operation
        terms_used: List of input terms that were processed
        warnings: List of any warnings generated during processing
        metadata: Dict containing resolver metadata (modelId, latencyMs, promptHash, etc.)
    """

    text: str
    terms_used: List[str]
    warnings: List[str]
    metadata: Dict[str, Any]


@dataclass
class Cell:
    """
    Fundamental semantic unit in Chirality Framework semantic calculator.

    Stores the result of the 3-stage interpretation pipeline:
    - Stage 1 (Combinatorial): k-products generated mechanically
    - Stage 2 (Semantic): Word pairs resolved to concepts
    - Stage 3 (Lensing): Ontological interpretation applied

    Attributes:
        row: Row position in matrix
        col: Column position in matrix
        value: Final semantic result after 3-stage pipeline
        provenance: Dict storing all intermediate results from each stage
    """

    row: int
    col: int
    value: str
    provenance: Dict[str, Any] = field(default_factory=dict)


@dataclass
class Matrix:
    """
    2D semantic matrix for Chirality Framework semantic calculator.

    Contains cells arranged in a fixed ontological structure where:
    - row_labels define the row ontological axis
    - col_labels define the column ontological axis
    - Each cell represents the semantic intersection of its row/column coordinates

    Attributes:
        name: Matrix identifier (A, B, C, D, F, J)
        station: Valley station where matrix exists
        row_labels: Ontological labels for rows (e.g. ["Normative", "Operative", "Evaluative"])
        col_labels: Ontological labels for columns (e.g. ["Determinacy", "Sufficiency", etc.])
        cells: 2D array of cells [row][col]
    """

    name: str
    station: str
    row_labels: List[str]
    col_labels: List[str]
    cells: List[List[Cell]]

    @property
    def shape(self) -> tuple[int, int]:
        """Get matrix dimensions."""
        return (len(self.row_labels), len(self.col_labels))

    def get_cell(self, row: int, col: int) -> Optional[Cell]:
        """Get cell at specific position."""
        if 0 <= row < len(self.cells) and 0 <= col < len(self.cells[row]):
            return self.cells[row][col]
        return None

    def __getitem__(self, key):
        """
        Support A[i] -> row list and A[i, j] -> single cell.
        Keeps legacy code working that wrote A[i][j].
        """
        if isinstance(key, tuple):
            i, j = key
            return self.cells[i][j]
        return self.cells[key]

    def __len__(self):
        """Return number of rows in matrix."""
        return len(self.cells)

    def transpose(self) -> "Matrix":
        """
        Transposes the matrix by swapping rows and columns.

        Returns a new Matrix instance with transposed dimensions, labels, and cells.
        """
        transposed_cells = []

        for j in range(self.shape[1]):
            new_row = []
            for i in range(self.shape[0]):
                original_cell = self.get_cell(i, j)
                if original_cell:
                    new_row.append(
                        Cell(
                            row=j,
                            col=i,
                            value=original_cell.value,
                            provenance=original_cell.provenance,
                        )
                    )
            transposed_cells.append(new_row)

        return Matrix(
            name=f"{self.name}_transposed",
            station=self.station,
            row_labels=self.col_labels,
            col_labels=self.row_labels,
            cells=transposed_cells,
        )


@dataclass
class Phase1Config:
    """Configuration for Phase 1 operations."""

    token_budget: Optional[int] = None
    cost_budget: Optional[float] = None
    time_budget: Optional[int] = None

    # LLM inference parameters (updated defaults for gpt-5-nano)
    model: str = "gpt-5-nano"
    temperature: float = 1.0
    top_p: float = 0.9  # Note: user requested "top-k 0.9" but OpenAI uses top_p


@dataclass
class Phase2Config:
    """Configuration for Phase 2 operations."""

    token_budget: Optional[int] = None
    cost_budget: Optional[float] = None
    time_budget: Optional[int] = None
    parallel: int = 1
    cache_enabled: bool = True
    resume_enabled: bool = True

    # LLM inference parameters (updated defaults for gpt-5-nano)
    model: str = "gpt-5-nano"
    temperature: float = 1.0
    top_p: float = 0.9  # Note: user requested "top-k 0.9" but OpenAI uses top_p

    # GPT-5 specific parameters
    verbosity: str = "medium"  # "low", "medium", "high"
    reasoning_effort: str = "medium"  # "minimal", "medium"


@dataclass
class ChiralityConfig:
    """Overall Chirality Framework configuration."""

    phase1: Phase1Config = field(default_factory=Phase1Config)
    phase2: Phase2Config = field(default_factory=Phase2Config)
    prompt_version: str = "v1"

    # Model pricing (per-token; centralized from pricing module)
    model_pricing: Dict[str, Dict[str, float]] = field(default_factory=get_model_pricing)
