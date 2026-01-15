"""
Model pricing definitions for budget calculations.

Single source of truth for per-token pricing across all OpenAI models.
"""

from typing import Dict

# Current OpenAI pricing (per-token; derived from per-1M list)
MODEL_PRICING: Dict[str, Dict[str, float]] = {
    # GPT-5 models
    "gpt-5": {
        "input": 1.25 / 1_000_000,  # $1.25 per 1M tokens
        "cached_input": 0.125 / 1_000_000,  # $0.125 per 1M tokens
        "output": 10.00 / 1_000_000,  # $10.00 per 1M tokens
    },
    "gpt-5-mini": {
        "input": 0.25 / 1_000_000,  # $0.25 per 1M tokens
        "cached_input": 0.025 / 1_000_000,  # $0.025 per 1M tokens
        "output": 2.00 / 1_000_000,  # $2.00 per 1M tokens
    },
    "gpt-5-nano": {
        "input": 0.05 / 1_000_000,  # $0.05 per 1M tokens
        "cached_input": 0.005 / 1_000_000,  # $0.005 per 1M tokens
        "output": 0.40 / 1_000_000,  # $0.40 per 1M tokens
    },
    "gpt-5-chat-latest": {
        "input": 1.25 / 1_000_000,  # $1.25 per 1M tokens
        "cached_input": 0.125 / 1_000_000,  # $0.125 per 1M tokens
        "output": 10.00 / 1_000_000,  # $10.00 per 1M tokens
    },
    # GPT-4.1 models
    "gpt-4.1": {
        "input": 2.00 / 1_000_000,  # $2.00 per 1M tokens
        "cached_input": 0.50 / 1_000_000,  # $0.50 per 1M tokens
        "output": 8.00 / 1_000_000,  # $8.00 per 1M tokens
    },
    "gpt-4.1-mini": {
        "input": 0.40 / 1_000_000,  # $0.40 per 1M tokens
        "cached_input": 0.10 / 1_000_000,  # $0.10 per 1M tokens
        "output": 1.60 / 1_000_000,  # $1.60 per 1M tokens
    },
    "gpt-4.1-nano": {
        "input": 0.10 / 1_000_000,  # $0.10 per 1M tokens
        "cached_input": 0.025 / 1_000_000,  # $0.025 per 1M tokens
        "output": 0.40 / 1_000_000,  # $0.40 per 1M tokens
    },
    # GPT-4o models
    "gpt-4o": {
        "input": 2.50 / 1_000_000,  # $2.50 per 1M tokens
        "cached_input": 1.25 / 1_000_000,  # $1.25 per 1M tokens
        "output": 10.00 / 1_000_000,  # $10.00 per 1M tokens
    },
    "gpt-4o-mini": {
        "input": 0.15 / 1_000_000,  # $0.15 per 1M tokens
        "cached_input": 0.075 / 1_000_000,  # $0.075 per 1M tokens
        "output": 0.60 / 1_000_000,  # $0.60 per 1M tokens
    },
    # Legacy models (normalize to per-token)
    "gpt-4": {
        "input": 30.00 / 1_000_000,  # $30.00 per 1M tokens
        "output": 60.00 / 1_000_000,  # $60.00 per 1M tokens
    },
    "gpt-4-turbo": {
        "input": 10.00 / 1_000_000,  # $10.00 per 1M tokens
        "output": 30.00 / 1_000_000,  # $30.00 per 1M tokens
    },
    "gpt-3.5-turbo": {
        "input": 1.00 / 1_000_000,  # $1.00 per 1M tokens
        "output": 2.00 / 1_000_000,  # $2.00 per 1M tokens
    },
}


def get_model_pricing() -> Dict[str, Dict[str, float]]:
    """
    Get the current model pricing table.

    Returns:
        Dictionary mapping model names to pricing structures
    """
    return MODEL_PRICING.copy()


def get_model_price(model: str, token_type: str) -> float:
    """
    Get price per token for a specific model and token type.

    Args:
        model: Model identifier (e.g., "gpt-5-nano")
        token_type: Type of token ("input", "cached_input", "output")

    Returns:
        Price per token in USD

    Raises:
        KeyError: If model or token type not found
    """
    if model not in MODEL_PRICING:
        raise KeyError(f"Unknown model: {model}")

    pricing = MODEL_PRICING[model]
    if token_type not in pricing:
        # Fall back to regular input price for cached_input if not available
        if token_type == "cached_input" and "input" in pricing:
            return pricing["input"]
        raise KeyError(f"Unknown token type '{token_type}' for model '{model}'")

    return pricing[token_type]


def calculate_cost(
    model: str, prompt_tokens: int = 0, completion_tokens: int = 0, cached_tokens: int = 0
) -> float:
    """
    Calculate total cost for a model call.

    Args:
        model: Model identifier
        prompt_tokens: Number of input tokens (including cached)
        completion_tokens: Number of output tokens
        cached_tokens: Number of cached input tokens (subset of prompt_tokens)

    Returns:
        Total cost in USD
    """
    if model not in MODEL_PRICING:
        return 0.0

    pricing = MODEL_PRICING[model]

    # Calculate cost considering cached vs regular input tokens
    regular_input_tokens = prompt_tokens - cached_tokens
    cost = (
        regular_input_tokens * pricing.get("input", 0)
        + cached_tokens * pricing.get("cached_input", pricing.get("input", 0))
        + completion_tokens * pricing.get("output", 0)
    )

    return cost
