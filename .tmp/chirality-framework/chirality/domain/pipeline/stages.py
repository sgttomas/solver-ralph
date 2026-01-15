"""
Domain logic for the 3-stage pipeline.

Defines the canonical stages without any implementation details.
Pure business logic that describes WHAT happens at each stage.
"""

from typing import Dict, List, Any, Optional
from enum import Enum
from dataclasses import dataclass

from ..semantics.operations import SemanticOperationType


class PipelineStage(Enum):
    """The three canonical stages of the Chirality Framework pipeline."""

    STAGE_1_CONSTRUCT = "stage_1_construct"
    STAGE_2_SEMANTIC = "stage_2_semantic"
    STAGE_3_COMBINED_LENSED = "stage_3_combined_lensed"


@dataclass
class StageDefinition:
    """Defines what happens at a pipeline stage."""

    stage: PipelineStage
    description: str
    is_llm_required: bool
    operation_type: Optional[SemanticOperationType]
    expected_inputs: List[str]
    expected_outputs: List[str]


# Domain rules for each stage
STAGE_DEFINITIONS = {
    PipelineStage.STAGE_1_CONSTRUCT: StageDefinition(
        stage=PipelineStage.STAGE_1_CONSTRUCT,
        description="Mechanical generation of k-products or direct pairs",
        is_llm_required=False,
        operation_type=None,  # Mechanical only
        expected_inputs=["source_matrices", "row_index", "col_index"],
        expected_outputs=["texts", "metadata", "terms_used", "warnings"],
    ),
    PipelineStage.STAGE_2_SEMANTIC: StageDefinition(
        stage=PipelineStage.STAGE_2_SEMANTIC,
        description="LLM resolves concepts via operation-specific strategies",
        is_llm_required=True,
        operation_type=None,  # Varies by matrix type
        expected_inputs=["stage_1_output", "component_id"],
        expected_outputs=["text", "metadata", "terms_used", "warnings"],
    ),
    PipelineStage.STAGE_3_COMBINED_LENSED: StageDefinition(
        stage=PipelineStage.STAGE_3_COMBINED_LENSED,
        description="Single unified semantic operation combining row × column × station perspectives",
        is_llm_required=True,
        operation_type=SemanticOperationType.LENSING,
        expected_inputs=["stage_2_output", "row_label", "col_label", "station_context"],
        expected_outputs=["text", "metadata", "terms_used", "warnings"],
    ),
}


def get_stage_definition(stage: PipelineStage) -> StageDefinition:
    """Get the domain definition for a pipeline stage."""
    return STAGE_DEFINITIONS[stage]


def validate_stage_inputs(stage: PipelineStage, inputs: Dict[str, Any]) -> List[str]:
    """
    Validate inputs for a pipeline stage according to domain rules.

    Args:
        stage: Pipeline stage
        inputs: Input data to validate

    Returns:
        List of validation errors (empty if valid)
    """
    definition = get_stage_definition(stage)
    errors = []

    # Check required inputs are present
    for required_input in definition.expected_inputs:
        if required_input not in inputs:
            errors.append(f"Missing required input '{required_input}' for {stage.value}")

    return errors


def get_matrix_pipeline_stages(component_id: str) -> List[PipelineStage]:
    """
    Get the pipeline stages for a matrix component according to domain rules.

    Args:
        component_id: Matrix component ('C', 'D', 'F', 'X', 'Z', 'E')

    Returns:
        List of stages in execution order
    """
    # All matrices follow the 3-stage pipeline
    # (except Z which has special handling in Stage 3)
    return [
        PipelineStage.STAGE_1_CONSTRUCT,
        PipelineStage.STAGE_2_SEMANTIC,
        PipelineStage.STAGE_3_COMBINED_LENSED,
    ]


def is_mechanical_matrix(component_id: str) -> bool:
    """Check if a matrix uses mechanical operations (no LLM in Stage 2)."""
    # Domain rule: Only D uses mechanical addition in Stage 2
    return component_id == "D"


def uses_special_lensing(component_id: str) -> bool:
    """Check if a matrix uses special lensing instead of combined lensing."""
    # Domain rule: Only Z uses station shift instead of combined lensing
    return component_id == "Z"
