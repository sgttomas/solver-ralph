"""
Budget-aware LLM resolver wrapper.

QUARANTINED: This file contained legacy method references.
All legacy methods have been completely removed pending new architecture implementation.
"""

from typing import List
from ...domain.budgets import BudgetTracker  
from ...domain.types import RichResult


class BudgetResolver:
    """
    QUARANTINED: Legacy resolver removed due to architecture violations.
    
    This class is no longer functional and raises errors when used.
    Use the CLI interface instead: python3 -m chirality.interfaces.cli
    """

    def __init__(self, wrapped_resolver=None, budget_tracker: BudgetTracker = None):
        """QUARANTINED: Legacy constructor removed."""
        raise NotImplementedError(
            "BudgetResolver has been quarantined due to architecture violations. "
            "Legacy template-based methods have been completely removed. "
            "Use the CLI interface instead: python3 -m chirality.interfaces.cli"
        )