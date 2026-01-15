"""
Global LLM Configuration for Chirality Framework

Centralized configuration for OpenAI Responses API calls.
No per-station overrides - single global configuration.
"""

import os
from typing import Optional, Dict, Any
from dataclasses import dataclass, field


@dataclass
class LLMConfig:
    """Global LLM configuration parameters."""

    model: str = "gpt-5-mini"
    temperature: Optional[float] = 1.0  # Single-source default (can be overridden via env/testing)
    top_p: Optional[float] = 0.9
    max_tokens: Optional[int] = None  # Let API determine from context
    seed: Optional[int] = None  # For deterministic testing
    response_format: Dict[str, Any] = field(default_factory=lambda: {"type": "json_object"})


# Global configuration instance
_config: Optional[LLMConfig] = None


def get_config() -> LLMConfig:
    """
    Get global LLM configuration.

    Returns:
        LLMConfig instance with current settings
    """
    global _config
    if _config is None:
        _config = create_default_config()
    return _config


def set_config(config: LLMConfig) -> None:
    """
    Set global LLM configuration.

    Args:
        config: New LLMConfig instance
    """
    global _config
    _config = config


def create_default_config() -> LLMConfig:
    """
    Create default configuration from environment variables.

    Environment variables:
        CHIRALITY_MODEL: Model name (default: gpt-5-mini)
        CHIRALITY_TEMPERATURE: Temperature (default: 1.0)
        CHIRALITY_TOP_P: Top-p value (default: 0.9)
        CHIRALITY_MAX_TOKENS: Max tokens (default: None)
        CHIRALITY_SEED: Random seed (default: None)

    Returns:
        LLMConfig with environment-based or default values
    """
    return LLMConfig(
        model=os.getenv("CHIRALITY_MODEL", "gpt-5-mini"),
        temperature=float(os.getenv("CHIRALITY_TEMPERATURE", "1.0")),
        top_p=float(os.getenv("CHIRALITY_TOP_P", "0.9")),
        max_tokens=_parse_optional_int(os.getenv("CHIRALITY_MAX_TOKENS")),
        seed=_parse_optional_int(os.getenv("CHIRALITY_SEED")),
    )


def _parse_optional_int(value: Optional[str]) -> Optional[int]:
    """Parse optional integer from string."""
    if value is None or value.strip() == "":
        return None
    try:
        return int(value)
    except ValueError:
        return None


def configure_for_testing(seed: int = 42, temperature: float = 0.1) -> LLMConfig:
    """
    Create deterministic configuration for testing.

    Args:
        seed: Random seed for reproducibility
        temperature: Low temperature for deterministic output

    Returns:
        LLMConfig optimized for testing
    """
    config = LLMConfig(model="gpt-5-mini", temperature=temperature, top_p=0.9, seed=seed)
    set_config(config)
    return config
