"""
Mock LLM resolvers for testing.

QUARANTINED: This file contained legacy method references.
All legacy methods have been completely removed pending new architecture implementation.
"""

from typing import List
from ...domain.types import RichResult


class EchoResolver:
    """
    QUARANTINED: Legacy resolver removed due to architecture violations.
    
    This class is no longer functional and raises errors when used.
    Use the CLI interface instead: python3 -m chirality.interfaces.cli
    """

    def __init__(self, *args, **kwargs):
        raise NotImplementedError(
            "EchoResolver has been quarantined due to architecture violations. "
            "Legacy template-based methods have been completely removed. "
            "Use the CLI interface instead: python3 -m chirality.interfaces.cli"
        )

    def resolve(self, prompt: str) -> str:
        """QUARANTINED: Legacy method removed."""
        raise NotImplementedError(
            "EchoResolver.resolve() has been quarantined due to architecture violations. "
            "Legacy template-based methods have been completely removed. "
            "Use the CLI interface instead: python3 -m chirality.interfaces.cli"
        )