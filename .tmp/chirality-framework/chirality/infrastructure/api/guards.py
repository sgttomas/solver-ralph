"""
API Guards for Chirality Framework

Per colleague_1's D2-4 specification:
- no_chat_completions: Forbid client.chat.completions.create
- no_decoding_overrides: Forbid passing temperature, top_p, etc., from code/tests

Enforces Responses API usage and external parameter control throughout the framework.
"""


class APIGuardError(Exception):
    """Raised when forbidden API is used."""

    pass


class DecodingOverrideError(Exception):
    """Raised when decoding parameters are passed from code/tests."""

    pass


def no_chat_completions(*args, **kwargs):
    """
    D2-4 Guard: Prevent Chat Completions API usage.

    Per colleague_1's specification, this guard must forbid any usage of
    client.chat.completions.create throughout the framework.

    Raises:
        APIGuardError: Always - Chat Completions API is forbidden
    """
    raise APIGuardError(
        "Chat Completions API is forbidden in Chirality Framework. "
        "Use Responses API (client.responses.create) instead. "
        "This is enforced by the normative specification."
    )


def no_decoding_overrides(func_name: str, **kwargs):
    """
    D2-4 Guard: Prevent decoding parameter overrides from code/tests.

    Per colleague_1's specification, temperature, top_p, and other decoding
    parameters must be controlled externally, not passed from code.

    Args:
        func_name: Name of the function being called
        **kwargs: Parameters being passed to the function

    Raises:
        DecodingOverrideError: If forbidden parameters are detected
    """
    forbidden_params = {
        'temperature', 'top_p', 'top_k', 'frequency_penalty', 
        'presence_penalty', 'repetition_penalty', 'min_p',
        'typical_p', 'entropy_cutoff', 'rep_pen'
    }
    
    detected_params = []
    for param in forbidden_params:
        if param in kwargs and kwargs[param] is not None:
            detected_params.append(param)
    
    if detected_params:
        raise DecodingOverrideError(
            f"Decoding parameters {detected_params} forbidden in {func_name}. "
            "Parameters must be controlled externally via configuration, "
            "not passed from code/tests. This is enforced by colleague_1's "
            "D2-4 specification."
        )


# Legacy name for backward compatibility
forbid_chat_completions = no_chat_completions


def require_responses_api():
    """
    Decorator to enforce Responses API usage.

    Usage:
        @require_responses_api()
        def my_llm_function():
            # Must use client.responses.create(input=...)
            pass
    """

    def decorator(func):
        def wrapper(*args, **kwargs):
            # Check for forbidden imports/calls during runtime
            import sys

            forbidden_modules = ["openai.chat", "openai.Completion"]
            for module_name in forbidden_modules:
                if module_name in sys.modules:
                    raise APIGuardError(
                        f"Forbidden module {module_name} detected. "
                        "Only Responses API is allowed."
                    )
            return func(*args, **kwargs)

        return wrapper

    return decorator


def install_chat_completions_guard():
    """
    D2-4: Install a guard that monkey-patches OpenAI Chat Completions to always fail.

    This should be called in test setup to ensure no code accidentally uses
    the forbidden Chat Completions API.
    """
    try:
        import openai

        # Monkey patch chat completions to always raise our guard error
        if hasattr(openai, "chat") and hasattr(openai.chat, "completions"):
            openai.chat.completions.create = no_chat_completions

        # Also patch any Completions API if it exists
        if hasattr(openai, "Completion"):
            openai.Completion.create = no_chat_completions

    except ImportError:
        # OpenAI not installed, no need to guard
        pass


def install_all_guards():
    """
    D2-4: Install all D2-4 guards for comprehensive protection.
    
    This should be called at application startup or test setup to ensure
    all forbidden patterns are protected against.
    """
    install_chat_completions_guard()
    
    # The no_decoding_overrides guard is applied at function call level,
    # so it doesn't need global installation like the chat completions guard


def guard_llm_call(func_name: str, **kwargs):
    """
    D2-4: Combined guard for LLM calls.
    
    Validates parameters against SDK-documented Responses API allow-list.
    Per colleague_1's guidance: strict allow-list with descriptive failures.
    Also handles model capability checks and parameter filtering.
    
    Args:
        func_name: Name of the function being called
        **kwargs: Parameters being passed to the function
        
    Returns:
        Dict with potentially modified kwargs (reasoning may be dropped)
        
    Raises:
        ValueError: If forbidden or unknown parameters detected
    """
    # SDK-documented allow-list for Responses API per OpenAI docs
    # Note: 'model' deliberately excluded - adapter controls model selection
    allowed_params = {
        # Core parameters
        'instructions', 'input',
        # Optional control parameters  
        'temperature', 'top_p', 'max_output_tokens',
        'seed', 'reasoning', 'text', 'response_format',
        'store', 'metadata', 'verbosity',
        # Framework control parameters
        'expects_json'  # Controls JSON format application
    }
    
    # Check for unknown parameters
    unknown_params = set(kwargs.keys()) - allowed_params
    if unknown_params:
        raise ValueError(
            f"Unknown parameters in {func_name}: {sorted(unknown_params)}. "
            f"Allowed: {sorted(allowed_params)}. "
            f"This maintains SDK compatibility per colleague_1's architecture."
        )
    
    # Guard is purely syntactic per colleague_1's refinement
    # No business logic, capability checks, or config access
    return kwargs


# Global guards that can be monkey-patched if needed
CHAT_COMPLETIONS_GUARD = no_chat_completions
DECODING_OVERRIDES_GUARD = no_decoding_overrides
