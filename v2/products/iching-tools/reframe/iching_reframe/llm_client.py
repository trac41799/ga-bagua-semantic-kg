"""LLM access for iching_reframe.

LLMClient talks to an OpenAI-compatible /chat/completions endpoint
(retries 2, timeout 120s, budget 50k tokens/call, temperature 0).

SimulatedLLM is a deterministic stand-in for tests/demos ONLY: its reframes
always contain the move name, so tests can verify all 8 positions without any
network access. The API key is never logged.
"""

import json
import os
import re
import time
import urllib.error
import urllib.request

from .moves import START_STATE, all_positions, describe_state

try:
    from iching_tools.providers import (
        DEEPSEEK_BASE_URL,
        OPENROUTER_BASE_URL,
        ProviderConfig,
        resolve_provider,
    )
except ModuleNotFoundError as exc:  # source-tree package CLI compatibility
    if exc.name != "iching_tools":
        raise
    import sys
    from pathlib import Path

    sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
    from iching_tools.providers import (  # type: ignore[no-redef]
        DEEPSEEK_BASE_URL,
        OPENROUTER_BASE_URL,
        ProviderConfig,
        resolve_provider,
    )

DEFAULT_BASE_URL = DEEPSEEK_BASE_URL
DEFAULT_MODEL = "deepseek-chat"
RETRIES = 2
TIMEOUT = 120
BUDGET_TOKENS = 50_000
TEMPERATURE = 0.0


class LLMError(Exception):
    """Raised when the LLM endpoint cannot be reached or returns garbage."""


def resolve_key(api_key=None, provider=None, model=DEFAULT_MODEL):
    """Resolve the API key: --api-key wins, then env vars. Never logs the key.

    Returns (key_or_None, provider) with provider in
    {"deepseek", "openrouter", None}.
    """
    config = resolve_provider(
        explicit_key=api_key,
        explicit_provider=provider,
        model=model,
    )
    return (config.api_key, config.provider) if config else (None, None)


def base_url_for(provider):
    if provider == "deepseek":
        return DEEPSEEK_BASE_URL
    if provider == "openrouter":
        return OPENROUTER_BASE_URL
    raise ValueError("unknown provider: expected deepseek or openrouter")


def resolve_provider_config(api_key=None, provider=None, model=DEFAULT_MODEL):
    return resolve_provider(
        explicit_key=api_key,
        explicit_provider=provider,
        model=model,
    )


class LLMClient:
    """OpenAI-compatible chat completions client (stateless; no disk caching)."""

    def __init__(self, base_url=None, api_key=None, model=DEFAULT_MODEL,
                 retries=RETRIES, timeout=TIMEOUT, budget_tokens=BUDGET_TOKENS,
                 temperature=TEMPERATURE, *, provider=None,
                 provider_config: ProviderConfig | None = None):
        config = provider_config or resolve_provider_config(
            api_key=api_key, provider=provider, model=model
        )
        if config is None:
            raise ValueError(
                "no API key: set DEEPSEEK_API_KEY or OPENROUTER_API_KEY, or pass api_key"
            )
        self.provider_config = config
        self.provider = config.provider
        self.base_url = (base_url or config.api_base_url).rstrip("/")
        self.api_key = config.api_key
        self.model = config.model
        self.retries = retries
        self.timeout = timeout
        self.budget_tokens = budget_tokens
        self.temperature = temperature
        self._url = self.base_url + "/chat/completions"

    def __repr__(self):
        return (
            f"LLMClient(provider={self.provider!r}, model={self.model!r}, "
            f"base_url={self.base_url!r})"
        )

    def chat(self, messages):
        """Send a message list; return the assistant text."""
        payload = {
            "model": self.model,
            "messages": messages,
            "temperature": self.temperature,
            "max_tokens": self.budget_tokens,
        }
        body = json.dumps(payload).encode("utf-8")
        headers = {
            "Content-Type": "application/json",
            "Authorization": f"Bearer {self.api_key}",
        }
        last_error = None
        for attempt in range(self.retries + 1):
            try:
                request = urllib.request.Request(self._url, data=body, headers=headers,
                                                 method="POST")
                with urllib.request.urlopen(request, timeout=self.timeout) as response:
                    data = json.loads(response.read().decode("utf-8"))
                return data["choices"][0]["message"]["content"]
            except (urllib.error.URLError, TimeoutError, OSError,
                    json.JSONDecodeError, KeyError, IndexError) as exc:
                last_error = exc
                if attempt < self.retries:
                    time.sleep(1.0 + attempt)
        raise LLMError(
            f"LLM request failed after {self.retries + 1} attempts: {last_error}")


class SimulatedLLM:
    """Deterministic stand-in (tests/demos only): reframes contain the move name.

    Parses the naming prompt (statement + position state) and maps the position
    state back to its exact cube move, so every returned reframe string is
    deterministic and contains e.g. "flip0" / "double_flip01" / "complement".
    """

    _STATEMENT_RE = re.compile(r"Statement: (.+?)\nPosition state:", re.DOTALL)
    _STATE_RE = re.compile(r"Position state: (.+?)\nReframe:")

    def __init__(self):
        self._by_state = {describe_state(s): move for move, s in all_positions(START_STATE)}

    def chat(self, messages):
        content = messages[-1]["content"]
        stm = self._STATEMENT_RE.search(content)
        stt = self._STATE_RE.search(content)
        if not stm or not stt:
            raise LLMError("SimulatedLLM: cannot parse naming prompt")
        statement = stm.group(1)
        state_label = stt.group(1)
        move = self._by_state.get(state_label)
        if move is None:
            raise LLMError(f"SimulatedLLM: unknown position state {state_label!r}")
        return f"[sim:{move}] {statement} -- reframed from {state_label}"
