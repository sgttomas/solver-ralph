"""
Pure domain logic for semantic operations.

This module contains the business rules for semantic multiplication, addition, etc.
It has no external dependencies - only pure functions that define what operations
mean in the context of the Chirality Framework.
"""

from typing import List, Tuple, Dict, Any
from enum import Enum


class SemanticOperationType(Enum):
    """Types of semantic operations in the framework."""

    MULTIPLY = "multiply"
    ELEMENTWISE = "elementwise"
    ADDITION = "addition"
    LENSING = "lensing"
    SHIFT = "shift"


class SemanticOperation:
    """
    Represents a semantic operation with its inputs and expected output structure.

    This is pure domain logic - it defines WHAT should happen, not HOW it's executed.
    """

    def __init__(
        self, operation_type: SemanticOperationType, inputs: List[str], context: Dict[str, Any]
    ):
        self.operation_type = operation_type
        self.inputs = inputs
        self.context = context

    def validate(self) -> List[str]:
        """
        Validate operation inputs according to domain rules.

        Returns:
            List of validation errors (empty if valid)
        """
        errors = []

        if self.operation_type == SemanticOperationType.MULTIPLY:
            if len(self.inputs) < 2:
                errors.append("Multiply operation requires at least 2 inputs")

        elif self.operation_type == SemanticOperationType.ELEMENTWISE:
            if len(self.inputs) != 2:
                errors.append("Elementwise operation requires exactly 2 inputs")

        elif self.operation_type == SemanticOperationType.ADDITION:
            if len(self.inputs) != 2:
                errors.append("Addition operation requires exactly 2 parts")

        return errors

    def get_expected_output_structure(self) -> Dict[str, str]:
        """
        Define the expected structure of the operation result.

        Returns:
            Dictionary describing expected output fields and types
        """
        return {
            "text": "str",
            "terms_used": "List[str]",
            "warnings": "List[str]",
            "metadata": "Dict[str, Any]",
        }


def create_k_products(row_terms: List[str], col_terms: List[str]) -> List[Tuple[str, str]]:
    """
    Create k-products for semantic multiplication (pure function).

    This implements the combinatorial logic for Stage 1 of matrix operations.

    Args:
        row_terms: Terms from matrix row
        col_terms: Terms from matrix column

    Returns:
        List of (row_term, col_term) pairs for semantic resolution
    """
    return [(row_term, col_term) for row_term in row_terms for col_term in col_terms]


def create_addition_sentence(part_a: str, part_b: str) -> str:
    """
    Create mechanical addition sentence for Matrix D (pure function).

    This implements the canonical D matrix formula:
    A(i,j) + " applied to frame the problem; " + F(i,j) + " to resolve the problem."

    Args:
        part_a: First part (from matrix A)
        part_b: Second part (from matrix F)

    Returns:
        Mechanically constructed sentence
    """
    return f"{part_a} applied to frame the problem; {part_b} to resolve the problem."


def validate_semantic_input(text: str) -> List[str]:
    """
    Validate semantic input according to domain rules.

    Args:
        text: Input text to validate

    Returns:
        List of validation errors (empty if valid)
    """
    errors = []

    if not text.strip():
        errors.append("Semantic input cannot be empty")

    if len(text) > 10000:  # Reasonable limit
        errors.append("Semantic input too long (>10000 chars)")

    return errors


def semantic_multiply(term_a: str, term_b: str) -> str:
    """
    Perform semantic multiplication: find semantic intersection of two terms.
    
    This is a pure domain function that defines what semantic multiplication means.
    The actual LLM resolution would be handled by infrastructure layers.
    
    Examples from normative spec:
    - "sufficient" * "reason" = "justification"  
    - "precision" * "durability" = "reliability"
    - "probability" * "consequence" = "risk"
    
    Args:
        term_a: First term
        term_b: Second term
        
    Returns:
        Semantic intersection as coherent word/statement
    """
    # This is the domain definition - infrastructure will implement LLM resolution
    return f"{term_a}_MULTIPLY_{term_b}"


def semantic_add(parts: List[str]) -> str:
    """
    Perform semantic addition: concatenate terms/fragments into statement.
    
    This is a pure domain function for semantic addition.
    
    Example from normative spec:
    - "faisal" + "has" + "seven" + "balloons" = "faisal has seven balloons"
    
    Args:
        parts: List of terms/fragments to concatenate
        
    Returns:
        Concatenated statement with proper spacing
    """
    return " ".join(parts)
