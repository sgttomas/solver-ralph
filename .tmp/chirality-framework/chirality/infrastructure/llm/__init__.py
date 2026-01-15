"""
LLM Infrastructure Module

Contains LLM client adapters, mock resolvers, and configuration
for external language model integrations.
"""

from .openai_adapter import LLMClient
from .mock_resolvers import EchoResolver
from .config import get_config, LLMConfig

__all__ = [
    "LLMClient",
    "EchoResolver",
    "get_config",
    "LLMConfig",
]
