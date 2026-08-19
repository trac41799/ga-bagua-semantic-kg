"""LLM client for iching_coverage: DeepSeek/OpenRouter chat completions + SimulatedLLM.

The API key is never logged: it is not included in any repr, error message,
or exception raised by this module.
"""

import json
import os
import time
import urllib.error
import urllib.request

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

DEFAULT_MODEL = "deepseek-chat"
DEFAULT_BASE_URL = DEEPSEEK_BASE_URL
MAX_RETRIES = 2
TIMEOUT_SECONDS = 120
BUDGET_TOKENS = 50000
TEMPERATURE = 0


def resolve_api_key(api_key=None, provider=None, model=DEFAULT_MODEL):
    """Return the resolved key while keeping the historical helper API."""
    config = resolve_provider(
        explicit_key=api_key,
        explicit_provider=provider,
        model=model,
    )
    return config.api_key if config else None


def resolve_provider_config(api_key=None, provider=None, model=DEFAULT_MODEL):
    return resolve_provider(
        explicit_key=api_key,
        explicit_provider=provider,
        model=model,
    )


def _default_base_url(api_key, provider=None):
    config = resolve_provider_config(api_key=api_key, provider=provider)
    return config.api_base_url if config else DEFAULT_BASE_URL


class LLMClient:
    """Stateless chat-completions client. Retries 2, timeout 120s, budget 50k tokens/call."""

    def __init__(self, api_key=None, model=DEFAULT_MODEL, base_url=None,
                 *, provider=None, provider_config: ProviderConfig | None = None):
        config = provider_config or resolve_provider_config(
            api_key=api_key, provider=provider, model=model
        )
        if config is None:
            raise ValueError(
                "no API key: set DEEPSEEK_API_KEY or OPENROUTER_API_KEY, or pass api_key"
            )
        self.provider_config = config
        self.provider = config.provider
        self.api_key = config.api_key
        self.model = config.model
        self.base_url = (base_url or config.api_base_url).rstrip("/")
        self.max_retries = MAX_RETRIES
        self.timeout = TIMEOUT_SECONDS
        self.budget = BUDGET_TOKENS
        self.temperature = TEMPERATURE

    def __repr__(self):
        return (
            f"LLMClient(provider={self.provider!r}, model={self.model!r}, "
            f"base_url={self.base_url!r})"
        )

    def complete(self, messages):
        """Send a chat-completions request; return the assistant text."""
        estimated = sum((len(str(m.get("content", ""))) + 3) // 4 for m in messages)
        if estimated > self.budget:
            raise RuntimeError(f"prompt exceeds the budget of {self.budget} tokens")
        body = json.dumps(
            {
                "model": self.model,
                "messages": messages,
                "temperature": self.temperature,
                "stream": False,
            }
        ).encode("utf-8")
        last_error = None
        for attempt in range(self.max_retries + 1):
            try:
                request = urllib.request.Request(
                    self.base_url + "/chat/completions",
                    data=body,
                    headers={
                        "Content-Type": "application/json",
                        "Authorization": "Bearer " + self.api_key,
                    },
                    method="POST",
                )
                with urllib.request.urlopen(request, timeout=self.timeout) as response:
                    payload = json.loads(response.read().decode("utf-8"))
                return payload["choices"][0]["message"]["content"]
            except (urllib.error.HTTPError, urllib.error.URLError,
                    KeyError, ValueError) as exc:
                last_error = exc
                if attempt < self.max_retries:
                    time.sleep(2 * (attempt + 1))
        raise RuntimeError(
            f"LLM call failed after {self.max_retries + 1} attempts: {last_error}"
        )


class SimulatedLLM:
    """Deterministic stand-in for tests/demos. No network, no state."""

    def complete(self, messages):
        return (
            "receptive: stakeholders who must accept the plan are identified\n"
            "causal: explicit triggers start each phase of the plan\n"
            "transmissive: data, resources, and communication flows are mapped\n"
            "constraining: budgets, limits, and guardrails are fixed\n"
            "clarifying: metrics and checkpoints make progress visible\n"
            "influential: habits and conventions that must change are named\n"
            "balancing: feedback loops keep the plan stable\n"
            "generative: new capabilities and options the plan creates"
        )
