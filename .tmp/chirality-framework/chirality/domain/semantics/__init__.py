"""
Semantic operations for the Chirality Framework.

This module contains the semantic-first implementations for Phase 1 operations,
including lens generation/application and semantic arithmetic.
"""

from .lens import generate_lens, apply_lens, generate_matrix_lenses, apply_matrix_lenses
from .operations import semantic_multiply, semantic_add, create_k_products, create_addition_sentence

__all__ = [
    "generate_lens", 
    "apply_lens", 
    "generate_matrix_lenses", 
    "apply_matrix_lenses",
    "semantic_multiply",
    "semantic_add",
    "create_k_products", 
    "create_addition_sentence"
]
