"""
OpenAI Responses API Client for Chirality Framework

Single wrapper for OpenAI Responses API calls.
Enforces JSON output format and global configuration.
"""

import os
import time
import random
from typing import Dict, Any, List, Tuple, Optional

try:
    from openai import OpenAI
    from openai import RateLimitError, APITimeoutError, APIConnectionError, APIError
except ImportError:
    OpenAI = None
    RateLimitError = Exception
    APITimeoutError = Exception  
    APIConnectionError = Exception
    APIError = Exception

from .config import get_config
from ..api.guards import guard_llm_call, install_all_guards


class LLMClient:
    """
    Wrapper for OpenAI Responses API.

    Enforces use of Responses API (not Chat Completions) and JSON output format.
    Uses global configuration from llm_config.
    """

    def __init__(self, api_key: Optional[str] = None):
        """
        Initialize client.

        Args:
            api_key: OpenAI API key. Uses OPENAI_API_KEY env var if None.
        """
        if OpenAI is None:
            raise ImportError("OpenAI package required. Install with: pip install openai")

        # Load .env if present to populate OPENAI_API_KEY (silent best-effort)
        try:
            from dotenv import load_dotenv
            load_dotenv()
        except Exception:
            pass

        api_key = api_key or os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError("OpenAI API key required")

        self.client = OpenAI(api_key=api_key)
        self._rf_probed = False

    def _probe_response_format_support(self) -> None:
        # No-op: rely on pinned SDK in the environment; no runtime probing
        self._rf_probed = True

    def _call_with_retry(self, api_params: Dict[str, Any], max_retries: int = 3) -> Any:
        """
        P0-5: Resilient API call with exponential backoff and rate limit handling.
        
        Per colleague_1's specification: "SDK pin + resilient adapter (timeouts, 
        retries/backoff, rate-limit handling; usage/latency only in traces)."
        
        Args:
            api_params: Parameters for the Responses API call
            max_retries: Maximum number of retry attempts
            
        Returns:
            OpenAI API response object
            
        Raises:
            Exception: If all retries are exhausted
        """
        last_exception = None
        
        for attempt in range(max_retries + 1):
            try:
                # Set timeout for this attempt (progressively longer)
                # Use extended timeouts for reasoning-capable models (GPT-5, o1 series)
                model_name = api_params.get("model", "")
                is_reasoning_model = self._check_reasoning_capability(model_name)
                
                if is_reasoning_model:
                    # Extended timeouts for reasoning models: 240s, 300s, 360s, 420s
                    timeout = 240.0 + (attempt * 60.0)
                else:
                    # Standard timeouts for other models: 30s, 40s, 50s, 60s
                    timeout = 30.0 + (attempt * 10.0)
                
                # Temporarily set timeout on client
                original_timeout = getattr(self.client, 'timeout', None)
                self.client.timeout = timeout
                
                try:
                    # Log timeout info for reasoning models
                    if attempt == 0 and is_reasoning_model:
                        print(f"🧠 Reasoning model {model_name} using extended timeout: {timeout}s", file=__import__('sys').stderr)
                    elif attempt == 0:
                        print(f"⚡ Standard model {model_name} using timeout: {timeout}s", file=__import__('sys').stderr)
                    
                    # Make the API call
                    response = self.client.responses.create(**api_params)
                    
                    # Success - log latency only in traces (not transcript)
                    if attempt > 0:
                        print(f"📡 API success after {attempt} retries", file=__import__('sys').stderr)
                    
                    return response
                    
                finally:
                    # Restore original timeout
                    if original_timeout is not None:
                        self.client.timeout = original_timeout
                    
            except RateLimitError as e:
                last_exception = e
                if attempt == max_retries:
                    break
                    
                # Exponential backoff with jitter for rate limits
                backoff_time = (2 ** attempt) + random.uniform(0, 1)
                print(f"⏱️  Rate limit hit, retrying in {backoff_time:.1f}s (attempt {attempt + 1}/{max_retries + 1})", 
                      file=__import__('sys').stderr)
                time.sleep(backoff_time)
                
            except Exception as e:
                last_exception = e
                # Allow caller to handle signature-related issues (e.g., unexpected kwargs)
                if isinstance(e, TypeError):
                    raise
                
                # Check if it's a timeout or connection error by name/type
                if "timeout" in str(e).lower() or "connection" in str(e).lower() or \
                   e.__class__.__name__ in ["APITimeoutError", "APIConnectionError"]:
                    if attempt == max_retries:
                        break
                        
                    # Exponential backoff for timeouts/connection errors
                    backoff_time = (2 ** attempt) + random.uniform(0, 0.5)
                    print(f"🔄 Connection/timeout error, retrying in {backoff_time:.1f}s (attempt {attempt + 1}/{max_retries + 1})", 
                          file=__import__('sys').stderr)
                    time.sleep(backoff_time)
                    continue
                    
                # Check if it's an API error (don't retry)
                elif e.__class__.__name__ == "APIError" or "APIError" in str(type(e)):
                    print(f"❌ API error (not retrying): {e}", file=__import__('sys').stderr)
                    break
                    
                # Don't retry on other unexpected errors
                else:
                    print(f"❌ Unexpected error (not retrying): {e}", file=__import__('sys').stderr)
                    break
        
        # All retries exhausted
        raise Exception(f"LLM API call failed after {max_retries + 1} attempts: {last_exception}") from last_exception

    def _normalize_usage_fields(self, response: Any) -> Dict[str, int]:
        """
        Normalize usage fields from OpenAI response to consistent format.

        Provides robust extraction with fallbacks for different response structures.
        Always returns the 4 fields expected by BudgetTracker: prompt_tokens,
        completion_tokens, cached_tokens, total_tokens.

        Args:
            response: OpenAI API response object

        Returns:
            Dict with normalized usage fields, all integers >= 0
        """
        # Default values (safe fallbacks)
        prompt_tokens = 0
        completion_tokens = 0
        cached_tokens = 0
        total_tokens = 0

        # Extract from response.usage if available
        usage = getattr(response, "usage", None)
        if usage:
            # Primary token counts
            prompt_tokens = getattr(usage, "input_tokens", 0)
            if prompt_tokens == 0:  # Fallback naming
                prompt_tokens = getattr(usage, "prompt_tokens", 0)

            completion_tokens = getattr(usage, "output_tokens", 0)
            if completion_tokens == 0:  # Fallback naming
                completion_tokens = getattr(usage, "completion_tokens", 0)

            # Cached tokens from input_token_details
            input_details = getattr(usage, "input_token_details", None)
            if input_details:
                cached_tokens = getattr(input_details, "cached_tokens", 0)

            # Total (prefer provided total, fallback to sum)
            total_tokens = getattr(usage, "total_tokens", 0)
            if total_tokens == 0:
                total_tokens = prompt_tokens + completion_tokens

        # Ensure all values are non-negative integers
        return {
            "prompt_tokens": max(0, int(prompt_tokens)),
            "completion_tokens": max(0, int(completion_tokens)),
            "cached_tokens": max(0, int(cached_tokens)),
            "total_tokens": max(0, int(total_tokens)),
        }

    def _get_contract_mode(self) -> str:
        """Return contract mode: 'TEXT_FORMAT' (default) or 'RESPONSE_FORMAT'."""
        mode = os.getenv("CHIRALITY_CONTRACT") or os.getenv("CONTRACT") or "TEXT_FORMAT"
        mode = str(mode).strip().upper()
        return mode if mode in ("TEXT_FORMAT", "RESPONSE_FORMAT") else "TEXT_FORMAT"

    def call_responses(
        self,
        messages: List[Dict[str, str]],
        temperature: Optional[float] = None,
        max_tokens: Optional[int] = None,
        json_only: bool = True,
        top_p: Optional[float] = None,
        verbosity: Optional[str] = None,
        reasoning_effort: Optional[str] = None,
    ) -> Tuple[Dict[str, Any], Dict[str, Any]]:
        """
        Call OpenAI Responses API with messages.

        CRITICAL: Uses Responses API, not Chat Completions API.

        Args:
            messages: List of message dicts with 'role' and 'content'
            temperature: Optional temperature override
            max_tokens: Optional max_tokens override
            json_only: If True, enforce JSON response format
            top_p: Optional top_p override (note: OpenAI uses top_p, not top_k)
            verbosity: GPT-5 verbosity ("low", "medium", "high")
            reasoning_effort: GPT-5 reasoning effort ("minimal", "medium")

        Returns:
            Tuple of (response_dict, metadata_dict)

        Raises:
            ValueError: If response is not valid JSON
            Exception: For API errors
        """
        # D2-4: Apply guards with updated allow-list per colleague_1's guidance
        guard_llm_call("call_responses", 
                      temperature=temperature, 
                      max_tokens=max_tokens,
                      top_p=top_p,
                      verbosity=verbosity,
                      reasoning_effort=reasoning_effort)
        
        config = get_config()
        start_time = time.time()

        # Use provided values or defaults
        temp = temperature if temperature is not None else config.temperature
        max_tok = max_tokens if max_tokens is not None else config.max_tokens
        top_p_val = top_p if top_p is not None else config.top_p

        try:
            # PROPER: Use Responses API per colleague_1's requirements
            # Convert messages format to instructions + input for Responses API
            system_message = ""
            user_content = ""
            
            for msg in messages:
                if msg["role"] == "system":
                    system_message = msg["content"]
                elif msg["role"] == "user":
                    user_content += f"[USER] {msg['content']}\n"
                elif msg["role"] == "assistant":
                    user_content += f"[ASSISTANT] {msg['content']}\n"
            
            # Clean up trailing newline
            user_content = user_content.rstrip("\n")
            
            # Build Responses API parameters
            api_params = {
                "model": config.model,
                "instructions": system_message,
                "input": user_content,
            }
            
            # Only include temperature if non-None
            if temp is not None:
                api_params["temperature"] = temp
            
            # Only include top_p if non-None
            if top_p_val is not None:
                api_params["top_p"] = top_p_val
            
            # Use max_output_tokens instead of max_tokens for Responses API
            if max_tok is not None:
                api_params["max_output_tokens"] = max_tok
            
            # Only include seed if it's provided (not all APIs support it)
            if config.seed is not None:
                api_params["seed"] = config.seed

            # Add JSON format if requested
            if json_only:
                api_params["response_format"] = {"type": "json_object"}

            # P0-5: Use resilient retry for legacy method too
            response = self._call_with_retry(api_params)

            # Extract JSON content safely from Responses API structure
            try:
                # Standard Responses API response format
                if hasattr(response, "output_text"):
                    content = response.output_text or ""
                elif hasattr(response, "output") and response.output:
                    # Handle structured output format
                    if isinstance(response.output, list) and response.output:
                        content = response.output[0].get("content", {}).get("text", "")
                    else:
                        content = str(response.output)
                else:
                    raise ValueError("Could not extract content from Responses API response")

                # Parse JSON response
                import json

                response_dict = json.loads(content)

            except (json.JSONDecodeError, AttributeError, IndexError) as e:
                # Provide truncated response for debugging
                content_preview = (
                    str(content)[:200] + "..." if len(str(content)) > 200 else str(content)
                )
                raise ValueError(
                    f"Invalid JSON response from Responses API: {e}\\nResponse preview: {content_preview}"
                )

            # Build metadata with normalized usage extraction
            latency_ms = int((time.time() - start_time) * 1000)
            usage_fields = self._normalize_usage_fields(response)

            metadata = {
                "model": getattr(response, "model", config.model),
                "temperature": temp,
                "top_p": top_p_val,
                "max_output_tokens": max_tok,  # Changed from max_tokens to match Responses API
                "json_only": json_only,
                "latency_ms": latency_ms,
                "usage": getattr(response, "usage", None),
                "created_at": int(time.time()),
                "response_format": "json_object" if json_only else "text",
                **usage_fields,  # Merge normalized usage fields
            }

            return response_dict, metadata

        except Exception as e:
            # Add timing info to error metadata
            latency_ms = int((time.time() - start_time) * 1000)
            {
                "error": str(e),
                "latency_ms": latency_ms,
                "created_at": int(time.time()),
            }
            raise Exception(f"LLM API call failed: {e}") from e

    def _extract_response_content(self, response, response_format):
        """
        Centralized JSON extraction per colleague_1's guidance.
        
        Handles different response formats with clear error handling.
        Logs raw output on parse failures for debugging.
        """
        import json
        
        # Extract raw text content
        if hasattr(response, "output_text"):
            output_text = response.output_text or ""
        elif hasattr(response, "output") and response.output:
            # Handle structured output format
            if isinstance(response.output, list) and response.output:
                first = response.output[0]
                # If json_schema, some SDKs return parsed object; surface it
                if response_format and response_format.get("type") == "json_schema":
                    try:
                        return str(first), first
                    except Exception:
                        pass
                # Object-shaped path
                if hasattr(first, "content") and first.content:
                    first_content = first.content[0]
                    if hasattr(first_content, "text") and first_content.text:
                        output_text = first_content.text
                    else:
                        output_text = str(first)
                # Dict-shaped path
                elif isinstance(first, dict):
                    output_text = first.get("content", {}).get("text", "")
                else:
                    output_text = str(first)
            else:
                output_text = str(response.output)
        else:
            output_text = ""
        
        # Parse JSON with clear error handling
        if response_format and response_format.get("type") in ["json_object", "json_schema"]:
            try:
                response_dict = json.loads(output_text) if output_text else {}
            except json.JSONDecodeError as e:
                # Log raw payload for debugging per colleague_1's guidance
                response_dict = {
                    "error": "json_parse_failed", 
                    "parse_error": str(e),
                    "raw_output": output_text[:500] + "..." if len(output_text) > 500 else output_text
                }
        else:
            # Non-JSON response format
            response_dict = {"text": output_text}
        
        return output_text, response_dict

    def _call_responses_raw(self, api_params: Dict[str, Any]):
        """Raw HTTP call to /v1/responses using top-level response_format.
        Minimal, documented fields only; small timeouts and one retry on 5xx.
        """
        import os
        import time as _time
        from types import SimpleNamespace

        payload = {k: v for k, v in api_params.items() if v is not None}
        # Raw path honors contract toggle
        contract_mode = self._get_contract_mode()
        rf = payload.pop("response_format", None)
        if contract_mode == "RESPONSE_FORMAT":
            if rf is not None:
                payload["response_format"] = rf
            else:
                payload.setdefault("response_format", {"type": "json_object"})
        else:
            payload.setdefault("text", {"format": "json_object"})
        headers = {
            "Authorization": f"Bearer {os.getenv('OPENAI_API_KEY','')}",
            "Content-Type": "application/json",
        }

        def _do_post():
            try:
                import httpx
                with httpx.Client(timeout=httpx.Timeout(connect=10.0, read=60.0, write=10.0, pool=10.0)) as client:
                    return client.post("https://api.openai.com/v1/responses", headers=headers, json=payload)
            except ModuleNotFoundError:
                # Fallback to stdlib
                import json as _json
                import urllib.request
                req = urllib.request.Request(
                    url="https://api.openai.com/v1/responses",
                    data=_json.dumps(payload).encode("utf-8"),
                    headers=headers,
                    method="POST",
                )
                with urllib.request.urlopen(req, timeout=70) as resp:
                    class _Resp:
                        status_code = resp.status
                        text = resp.read().decode("utf-8")
                        def json(self):
                            return _json.loads(self.text)
                    return _Resp()

        # One retry on 5xx with jitter
        r = _do_post()
        if getattr(r, "status_code", 200) >= 500:
            _time.sleep(0.5 + random.uniform(0, 0.5))
            r = _do_post()

        if getattr(r, "status_code", 200) >= 400:
            raise RuntimeError(f"Responses API error {r.status_code}: {str(getattr(r, 'text', ''))[:300]}")

        data = r.json()
        output_text = ""
        if isinstance(data, dict):
            if isinstance(data.get("output_text"), str) and data.get("output_text"):
                output_text = data.get("output_text")
            else:
                out = data.get("output")
                if isinstance(out, list) and out:
                    first = out[0]
                    if isinstance(first, dict):
                        content = first.get("content")
                        if isinstance(content, list) and content:
                            text = content[0].get("text")
                            if isinstance(text, str):
                                output_text = text
        return SimpleNamespace(output_text=output_text, usage=None, model=data.get("model"))

    def _check_reasoning_capability(self, model: str) -> bool:
        """
        Check if model supports reasoning parameter.
        
        Per colleague_1's guidance: adapter handles capability checks.
        """
        reasoning_models = {'gpt-5', 'gpt-5-nano', 'o1', 'o1-mini', 'o1-preview'}
        return any(rm in str(model).lower() for rm in reasoning_models)

    def call_responses_new(
        self,
        *,
        instructions: str,
        input: str,
        store: bool = True,
        metadata: Optional[Dict[str, Any]] = None,
        response_format: Optional[Dict[str, Any]] = None,
        text: Optional[Dict[str, Any]] = None,
        expects_json: bool = False,
        temperature: Optional[float] = None,
        top_p: Optional[float] = None,
        max_output_tokens: Optional[int] = None,
        seed: Optional[int] = None,
        verbosity: Optional[str] = None,
        reasoning: Optional[Dict[str, str]] = None,
        **kwargs
    ) -> Dict[str, Any]:
        """
        Call OpenAI Responses API with instructions + input format.
        
        This is the canonical Responses API interface per colleague_1's specification.
        Uses instructions + input instead of messages array to maintain semantic purity.
        
        SELF-POLICING: Raises error if Chat Completions parameters are used.
        
        Args:
            instructions: System instructions (sent explicitly every call)  
            input: Complete input context (transcript + current stage prompt)
            response_format: JSON schema for strict response validation
            store: Whether to store the conversation
            metadata: Additional metadata for the request
            
        Returns:
            Dictionary with {id, output_text, usage, raw}
            
        Raises:
            ValueError: If forbidden Chat Completions parameters are provided
        """
        # Self-policing: Check for forbidden Chat Completions parameters
        forbidden_params = {
            'messages', 'message', 'role', 'content', 'system', 'user', 'assistant',
            'max_tokens', 'max_completion_tokens', 'function_call', 'functions',
            'chat', 'completions', 'stream', 'stream_options'
        }
        
        for param in kwargs:
            if param in forbidden_params:
                raise ValueError(
                    f"Forbidden Chat Completions parameter '{param}' detected in LLMClient. "
                    f"Chirality Framework requires Responses API with instructions+input format only."
                )
        
        if kwargs:
            # Any unexpected parameters should be flagged
            raise ValueError(
                f"Unexpected parameters: {list(kwargs.keys())}. "
                f"LLMClient.call_responses_new only accepts documented Responses API parameters."
            )
        config = get_config()
        start_time = time.time()
        
        # D2-4: Apply guards - purely syntactic validation per colleague_1's refinement
        # Pass only non-None values into guard to avoid spurious unknowns
        _gk = {
            k: v for k, v in {
                "response_format": response_format,
                "text": text,
                "expects_json": expects_json,
                "store": store,
                "metadata": metadata,
                "temperature": temperature,
                "top_p": top_p,
                "max_output_tokens": max_output_tokens,
                "seed": seed,
                "verbosity": verbosity,
                "reasoning": reasoning,
            }.items() if v is not None
        }
        guarded_kwargs = guard_llm_call("call_responses_new", **_gk)
        
        # Use validated parameters from guard
        response_format = guarded_kwargs.get('response_format', response_format)
        text = guarded_kwargs.get('text', text)
        expects_json = guarded_kwargs.get('expects_json', expects_json)
        store = guarded_kwargs.get('store', store)
        metadata = guarded_kwargs.get('metadata', metadata)
        temperature = guarded_kwargs.get('temperature', temperature)
        top_p = guarded_kwargs.get('top_p', top_p)
        max_output_tokens = guarded_kwargs.get('max_output_tokens', max_output_tokens)
        seed = guarded_kwargs.get('seed', seed)
        reasoning = guarded_kwargs.get('reasoning', reasoning)
        
        # Reasoning capability check - adapter handles business logic per colleague_1's refinement
        supports_reasoning = self._check_reasoning_capability(config.model)
        if reasoning and not supports_reasoning:
            # Drop reasoning and log to metadata per colleague_1's guidance
            if not isinstance(metadata, dict):
                metadata = {}
            metadata['reasoning_dropped'] = f"Model {config.model} does not support reasoning parameter"
            reasoning = None
        
        try:
            # PROPER: Use Responses API with instructions + typed message parts input
            api_params = {
                "model": config.model,
                "instructions": instructions,
            }
            
            # Use provided temperature or config default
            temp = temperature if temperature is not None else config.temperature
            if temp is not None:
                api_params["temperature"] = temp
            
            # Use provided top_p or config default (omit for reasoning-capable models that don't support it)
            top_p_val = top_p if top_p is not None else config.top_p
            # Defer supports_reasoning until after model is known
            
            # Handle max_tokens parameter canonicalization per colleague_1's guidance
            # Accept max_tokens for legacy compatibility but translate to max_output_tokens
            max_tok = max_output_tokens if max_output_tokens is not None else config.max_tokens
            if max_tok is not None:
                api_params["max_output_tokens"] = max_tok
            
            # Configure structured outputs per contract toggle
            contract_mode = self._get_contract_mode()
            if contract_mode == "RESPONSE_FORMAT":
                if isinstance(response_format, dict):
                    api_params["response_format"] = response_format
                elif expects_json:
                    api_params["response_format"] = {"type": "json_object"}
            else:
                if expects_json or response_format is not None:
                    api_params["text"] = {"format": "json_object"}
            
            # Add seed if provided
            seed_val = seed if seed is not None else config.seed
            if seed_val is not None:
                api_params["seed"] = seed_val
            
            # Note: verbosity is allowed for internal tracing but not forwarded to SDK
            # per colleague_1's guidance - strip non-SDK params before API call
            
            # Add reasoning if provided (nested structure per OpenAI docs)
            if reasoning:
                api_params["reasoning"] = reasoning
            
            # Add store and metadata for traceability per colleague_1's guidance
            if store:
                api_params["store"] = store
            
            if metadata:
                api_params["metadata"] = metadata
            
            # For reasoning-capable models, drop unsupported sampling params (e.g., top_p)
            try:
                supports_reasoning = self._check_reasoning_capability(config.model)
            except Exception:
                supports_reasoning = False
            if supports_reasoning:
                # Some reasoning models reject temperature and top_p; remove if present
                api_params.pop("top_p", None)
                api_params.pop("temperature", None)

            # Assemble typed input
            # - If caller passes a pre-built typed message list, forward as-is
            # - Otherwise, wrap the provided string as a single user turn
            if isinstance(input, (list, tuple)):
                api_params["input"] = input  # already typed parts
            else:
                api_params["input"] = [{
                    "role": "user",
                    "content": [{"type": "input_text", "text": input}]
                }]

            # P0-5: Call Responses API with resilient retry logic
            try:
                response = self._call_with_retry(api_params)
            except TypeError as e:
                # Minimal raw-POST fallback for parameter mismatches (disabled by default)
                if str(os.getenv("CHIRALITY_RAW_FALLBACK", "")).strip().lower() in ("1", "true", "yes"):
                    response = self._call_responses_raw(api_params)
                else:
                    raise
            
            # Extract content using centralized method per colleague_1's guidance
            output_text, response_dict = self._extract_response_content(response, response_format)
            
            # Build metadata for Responses API
            latency_ms = int((time.time() - start_time) * 1000)
            usage_fields = self._normalize_usage_fields(response)
            
            # Convert to new return format
            return {
                "id": f"resp_{int(time.time())}", 
                "output_text": output_text,
                "usage": getattr(response, "usage", None),
                "raw": {
                    "response": response_dict,
                    "metadata": {
                        "model": getattr(response, "model", config.model),
                        "temperature": config.temperature,
                        "top_p": config.top_p,
                        "max_output_tokens": config.max_tokens,
                        "latency_ms": latency_ms,
                        "created_at": int(time.time()),
                        # "response_format": response_format,
                        **usage_fields
                    },
                    "request": {
                        "instructions": instructions[:100] + "..." if len(instructions) > 100 else instructions,
                    "input": "[role:user, content:text]",
                        # "response_format": response_format,
                        "store": store
                    }
                }
            }
            
        except Exception as e:
            # Return error in new format
            return {
                "id": f"error_{int(time.time())}",
                "output_text": "",
                "usage": None,
                "raw": {
                    "error": str(e),
                    "latency_ms": int((time.time() - start_time) * 1000),
                    "created_at": int(time.time())
                }
            }


# Global client instance
_client: Optional[LLMClient] = None


def get_client() -> LLMClient:
    """Get global LLM client instance."""
    global _client
    if _client is None:
        _client = LLMClient()
    return _client


def call_responses_api(
    messages: List[Dict[str, str]],
    temperature: Optional[float] = None,
    max_tokens: Optional[int] = None,
    json_only: bool = True,  # Default to JSON for new architecture
    top_p: Optional[float] = None,
    verbosity: Optional[str] = None,
    reasoning_effort: Optional[str] = None,
) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    """
    DEPRECATED: Legacy API removed - use call_responses instead.
    
    This function is no longer supported. All code must use the Responses API
    with call_responses(instructions=..., input=...) format.

    Raises:
        NotImplementedError: Always - this API is deprecated and removed
    """
    raise NotImplementedError(
        "Legacy Chat Completions API removed - use call_responses(instructions=..., input=...) instead. "
        "The messages array format is no longer supported."
    )


def call_responses(
    *,
    instructions: str,
    input: str, 
    store: bool = True,
    metadata: Optional[Dict[str, Any]] = None,
    expects_json: bool = False,
    **kwargs
) -> Dict[str, Any]:
    """
    Call OpenAI Responses API with instructions + input format.
    
    Per colleague_1's D2-3 specification:
    - instructions = system.md (sent explicitly every call)
    - input = transcript_so_far + current_stage_asset_text
    - response_format = stage's JSON tail schema (strict)
    - store = conversation storage setting
    
    SELF-POLICING: Raises error if Chat Completions parameters are used.
    
    Args:
        instructions: System instructions (typically system.md content)
        input: Complete input context (transcript + current stage prompt)
        response_format: JSON schema for response validation
        store: Whether to store the conversation
        metadata: Additional metadata for the request
        
    Returns:
        Dictionary with {id, output_text, usage, raw}
        
    Raises:
        ValueError: If forbidden Chat Completions parameters are provided
    """
    # Self-policing: Check for forbidden Chat Completions parameters
    forbidden_params = {
        'messages', 'message', 'role', 'content', 'system', 'user', 'assistant',
        'max_tokens', 'max_completion_tokens', 'function_call', 'functions',
        'chat', 'completions', 'stream', 'stream_options'
    }
    
    for param in kwargs:
        if param in forbidden_params:
            raise ValueError(
                f"Forbidden Chat Completions parameter '{param}' detected. "
                f"Chirality Framework requires Responses API with instructions+input format only. "
                f"Use instructions=... and input=... instead of messages array."
            )
    
    # Extract and validate optional parameters from kwargs
    temperature = kwargs.pop('temperature', None)
    top_p = kwargs.pop('top_p', None) 
    max_output_tokens = kwargs.pop('max_output_tokens', None)
    seed = kwargs.pop('seed', None)
    verbosity = kwargs.pop('verbosity', None)
    reasoning_effort = kwargs.pop('reasoning_effort', None)
    response_format = kwargs.pop('response_format', None)
    text = kwargs.pop('text', None)
    
    if kwargs:
        # Log any unexpected parameters for debugging
        raise ValueError(
            f"Unexpected parameters: {list(kwargs.keys())}. "
            f"Only Responses API parameters are allowed: instructions, input, response_format, store, metadata, "
            f"temperature, top_p, max_output_tokens, seed, verbosity, reasoning_effort."
        )
    
    # Build reasoning object if effort is provided
    reasoning = None
    if reasoning_effort:
        reasoning = {"effort": reasoning_effort}
    
    client = get_client()
    # Probe response_format support (no-op if already checked)
    client._probe_response_format_support()
    # Build minimal, SDK-aligned kwargs and only include known, non-None values
    kwargs_out = {
        "instructions": instructions,
        "input": input,
        "store": store,
    }
    if metadata is not None:
        kwargs_out["metadata"] = metadata
    # Map strict outputs: prefer new text=; map legacy response_format if present
        # Set text.format=json_object for structured outputs
    if expects_json or isinstance(response_format, dict):
        kwargs_out["text"] = {"format": "json_object"}
    # Forward optional tuning only when set
    if temperature is not None:
        kwargs_out["temperature"] = temperature
    if top_p is not None:
        kwargs_out["top_p"] = top_p
    if max_output_tokens is not None:
        kwargs_out["max_output_tokens"] = max_output_tokens
    if seed is not None:
        kwargs_out["seed"] = seed
    if verbosity is not None:
        kwargs_out["verbosity"] = verbosity
    if reasoning is not None:
        kwargs_out["reasoning"] = reasoning

    return client.call_responses_new(**kwargs_out)
