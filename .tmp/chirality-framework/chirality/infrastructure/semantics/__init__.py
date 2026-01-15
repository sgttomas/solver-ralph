"""
Infrastructure for semantic operations using LLM resolution.

This module implements the actual LLM-backed semantic operations
defined in the domain layer.
"""

from .resolvers import semantic_multiply_llm, semantic_add_llm, resolve_semantic_expression

__all__ = [
    "semantic_multiply_llm",
    "semantic_add_llm", 
    "resolve_semantic_expression"
]