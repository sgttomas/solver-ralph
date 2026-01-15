"""
Cell computation service for matrix operations.

QUARANTINED: This file contained legacy method references.
All legacy methods have been completely removed pending new architecture implementation.
"""

from typing import Protocol
from ...domain.types import RichResult


class LLMResolver(Protocol):
    """
    QUARANTINED: Legacy interface removed due to architecture violations.
    
    This interface is no longer functional and raises errors when used.
    Use the CLI interface instead: python3 -m chirality.interfaces.cli
    """
    
    def __init__(self):
        raise NotImplementedError(
            "LLMResolver has been quarantined due to architecture violations. "
            "Legacy template-based methods have been completely removed. "
            "Use the CLI interface instead: python3 -m chirality.interfaces.cli"
        )


class CellComputationService:
    """
    QUARANTINED: Legacy service removed due to architecture violations.
    
    This class is no longer functional and raises errors when used.
    Use the CLI interface instead: python3 -m chirality.interfaces.cli
    """

    def __init__(self, llm_resolver=None):
        """QUARANTINED: Legacy constructor removed."""
        raise NotImplementedError(
            "CellComputationService has been quarantined due to architecture violations. "
            "Legacy template-based methods have been completely removed. "
            "Use the CLI interface instead: python3 -m chirality.interfaces.cli"
        )