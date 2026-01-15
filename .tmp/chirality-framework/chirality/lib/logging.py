"""
Production-ready logging utilities for Chirality Framework.

Provides channel separation for CI/CD integration:
- Logs → stderr for monitoring
- Data → stdout for pipeline consumption
"""

import sys
import json
from typing import Any, Dict, Optional


def log_info(message: str) -> None:
    """Log informational message to stderr."""
    print(f"🔄 {message}", file=sys.stderr)


def log_success(message: str) -> None:
    """Log success message to stderr."""
    print(f"✓ {message}", file=sys.stderr)


def log_error(message: str) -> None:
    """Log error message to stderr."""
    print(f"❌ {message}", file=sys.stderr)


def log_progress(message: str) -> None:
    """Log progress update to stderr."""
    print(f"🔄 {message}", file=sys.stderr)


def output_data(data: Any) -> None:
    """Output data to stdout for pipeline consumption."""
    if isinstance(data, (dict, list)):
        print(json.dumps(data), file=sys.stdout)
    else:
        print(data, file=sys.stdout)


def log_stats(
    stats: Dict[str, Any], title: Optional[str] = None, prefix: Optional[str] = None
) -> None:
    """Log statistics to stderr with proper formatting."""
    if title:
        print(f"📊 {title}", file=sys.stderr)

    if prefix:
        # Custom prefix format
        for key, value in stats.items():
            print(f"{prefix} {key}: {value}", file=sys.stderr)
    else:
        # Default format with dash
        for key, value in stats.items():
            print(f"  - {key}: {value}", file=sys.stderr)


def log_with_prefix(prefix: str, message: str) -> None:
    """Log message with custom prefix to stderr."""
    print(f"{prefix} {message}", file=sys.stderr)
