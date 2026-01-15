"""
Budget tracking and enforcement for LLM operations.

Provides token, cost, and time guards to prevent runaway expenses
during Phase 1 and Phase 2 operations.
"""

import time
from typing import Optional, Dict, Any
from dataclasses import dataclass
from pathlib import Path
from .pricing import get_model_pricing, calculate_cost


@dataclass
class BudgetConfig:
    """Budget configuration settings."""

    token_budget: Optional[int] = None
    cost_budget: Optional[float] = None  # USD
    time_budget: Optional[int] = None  # seconds

    # Model pricing (tokens per USD)
    model_pricing: Dict[str, Dict[str, float]] = None

    def __post_init__(self):
        if self.model_pricing is None:
            self.model_pricing = get_model_pricing()


class BudgetTracker:
    """
    Tracks and enforces budgets during LLM operations.

    Thread-safe for concurrent operations within a single process.
    """

    def __init__(self, config: BudgetConfig, phase: str = "unknown"):
        """
        Initialize budget tracker.

        Args:
            config: Budget configuration
            phase: Phase identifier (for error messages)
        """
        self.config = config
        self.phase = phase

        # Counters
        self.token_count = 0
        self.input_tokens = 0
        self.output_tokens = 0
        self.cost_spent = 0.0
        self.start_time = time.time()

        # Operation counter for provenance
        self.operation_count = 0

    def record_usage(self, metadata: Dict[str, Any], model: str = "gpt-4"):
        """
        Record usage from LLM call metadata and check budgets.

        Args:
            metadata: LLM response metadata with token counts
            model: Model identifier for cost calculation

        Raises:
            RuntimeError: If any budget is exceeded
        """
        # Extract token counts
        total_tokens = metadata.get("total_tokens", 0)
        prompt_tokens = metadata.get("prompt_tokens", 0)
        completion_tokens = metadata.get("completion_tokens", 0)
        cached_tokens = metadata.get("cached_tokens", 0)  # New cached input tokens

        # Update counters
        self.token_count += total_tokens
        self.input_tokens += prompt_tokens
        self.output_tokens += completion_tokens
        self.operation_count += 1

        # Calculate cost using centralized pricing
        cost = calculate_cost(model, prompt_tokens, completion_tokens, cached_tokens)
        self.cost_spent += cost

        # Check budgets
        self._check_budgets()

    def _check_budgets(self):
        """Check all budget limits and raise if exceeded."""

        # Token budget
        if self.config.token_budget and self.token_count > self.config.token_budget:
            raise RuntimeError(
                f"Token budget exceeded in {self.phase}: "
                f"{self.token_count:,} > {self.config.token_budget:,}. "
                f"Completed {self.operation_count} operations. "
                "Increase --token-budget or reduce scope."
            )

        # Cost budget
        if self.config.cost_budget and self.cost_spent > self.config.cost_budget:
            raise RuntimeError(
                f"Cost budget exceeded in {self.phase}: "
                f"${self.cost_spent:.4f} > ${self.config.cost_budget:.4f}. "
                f"Used {self.token_count:,} tokens in {self.operation_count} operations. "
                "Increase --cost-budget or reduce scope."
            )

        # Time budget
        if self.config.time_budget:
            elapsed = time.time() - self.start_time
            if elapsed > self.config.time_budget:
                raise RuntimeError(
                    f"Time budget exceeded in {self.phase}: "
                    f"{elapsed:.1f}s > {self.config.time_budget}s. "
                    f"Completed {self.operation_count} operations using {self.token_count:,} tokens. "
                    "Increase --time-budget or reduce scope."
                )

    def get_status(self) -> Dict[str, Any]:
        """Get current budget status for logging."""
        elapsed = time.time() - self.start_time

        return {
            "phase": self.phase,
            "operations": self.operation_count,
            "tokens": {
                "total": self.token_count,
                "input": self.input_tokens,
                "output": self.output_tokens,
                "budget": self.config.token_budget,
                "utilization": (
                    self.token_count / self.config.token_budget
                    if self.config.token_budget
                    else None
                ),
            },
            "cost": {
                "spent": self.cost_spent,
                "budget": self.config.cost_budget,
                "utilization": (
                    self.cost_spent / self.config.cost_budget if self.config.cost_budget else None
                ),
            },
            "time": {
                "elapsed": elapsed,
                "budget": self.config.time_budget,
                "utilization": (
                    elapsed / self.config.time_budget if self.config.time_budget else None
                ),
            },
        }

    def save_status(self, output_dir: Path):
        """Save budget status to output directory."""
        status_file = Path(output_dir) / "budget_status.json"
        status_file.parent.mkdir(parents=True, exist_ok=True)

        import json

        with open(status_file, "w") as f:
            json.dump(self.get_status(), f, indent=2)
