"""LLM client for iching_statediff: real (OpenAI-compatible) + SimulatedLLM.

The API key is never logged: it is not part of repr, error messages, or any
request dump. Retries 2, timeout 120s, budget cap 50k tokens per call.
"""

import json
import os
import time
import urllib.error
import urllib.request

try:
    from iching_tools.providers import ProviderConfig, resolve_provider
    from iching_tools.providers import (
        DEEPSEEK_BASE_URL,
        OPENROUTER_BASE_URL,
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

DEEPSEEK_URL = DEEPSEEK_BASE_URL + "/chat/completions"
OPENROUTER_URL = OPENROUTER_BASE_URL + "/chat/completions"
RETRIES = 2
TIMEOUT = 120
BUDGET_TOKENS = 50_000


class LLMError(RuntimeError):
    """LLM transport/response error (never contains the API key)."""


def resolve_provider_config(explicit_key=None, explicit_provider=None,
                            model="deepseek-chat"):
    return resolve_provider(
        explicit_key=explicit_key,
        explicit_provider=explicit_provider,
        model=model,
    )


def resolve_config(explicit_key=None, explicit_provider=None, model="deepseek-chat"):
    """Resolve (base_url, api_key) from --api-key, DEEPSEEK_API_KEY, OPENROUTER_API_KEY.

    Priority: explicit key > DEEPSEEK_API_KEY > OPENROUTER_API_KEY.
    Returns None when no key is available.
    """
    config = resolve_provider_config(explicit_key, explicit_provider, model)
    if config is None:
        return None
    return config.api_base_url.rstrip("/") + "/chat/completions", config.api_key


class LLMClient:
    def __init__(self, base_url=None, api_key=None, model="deepseek-chat",
                 retries=RETRIES, timeout=TIMEOUT, budget_tokens=BUDGET_TOKENS,
                 *, provider=None, provider_config: ProviderConfig | None = None):
        config = provider_config or resolve_provider_config(api_key, provider, model)
        if config is None:
            raise ValueError(
                "no API key: set DEEPSEEK_API_KEY or OPENROUTER_API_KEY, or pass api_key"
            )
        self.provider_config = config
        self.provider = config.provider
        self.base_url = base_url or config.api_base_url.rstrip("/") + "/chat/completions"
        self.api_key = config.api_key
        self.model = config.model
        self.retries = retries
        self.timeout = timeout
        self.budget_tokens = budget_tokens
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def __repr__(self):
        return (
            f"LLMClient(provider={self.provider!r}, model={self.model!r}, "
            f"base_url={self.base_url!r})"
        )

    def chat(self, messages, max_tokens=128, temperature=0.0):
        max_tokens = min(max_tokens or 128, self.budget_tokens)
        body = json.dumps({
            "model": self.model,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
        }).encode()
        req = urllib.request.Request(
            self.base_url, data=body, method="POST",
            headers={"Content-Type": "application/json",
                     "Authorization": "Bearer " + self.api_key},
        )
        last = None
        for attempt in range(self.retries + 1):
            try:
                with urllib.request.urlopen(req, timeout=self.timeout) as r:
                    data = json.loads(r.read())
                text = data["choices"][0]["message"]["content"]
                u = data.get("usage", {})
                for k in self.usage:
                    if k != "calls":
                        self.usage[k] += u.get(k, 0)
                self.usage["calls"] += 1
                if self.usage["total_tokens"] > self.budget_tokens:
                    raise LLMError("token budget exceeded")
                return text, u
            except urllib.error.URLError as e:
                last = e
                time.sleep(2 * (attempt + 1))
        raise LLMError(f"LLM call failed after {self.retries + 1} attempt(s)")


class SimulatedLLM:
    """Deterministic stand-in: emits 'aspectN: <token> -> <token>' lines derived
    from the user message. Tests/demos only -- never used in production paths.

    Pair the i-th whitespace token of Before with the i-th token of After.
    """

    def __init__(self, lines=3):
        self.lines = lines
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0):
        self.usage["calls"] += 1
        self.usage["total_tokens"] += 12
        before_tokens = []
        after_tokens = []
        for m in messages:
            if m.get("role") != "user":
                continue
            content = m.get("content", "")
            for line in content.splitlines():
                if line.startswith("Before: "):
                    before_tokens = line[len("Before: "):].split()
                elif line.startswith("After: "):
                    after_tokens = line[len("After: "):].split()
        out = []
        for i in range(self.lines):
            b = before_tokens[i] if i < len(before_tokens) else f"before{i + 1}"
            a = after_tokens[i] if i < len(after_tokens) else f"after{i + 1}"
            out.append(f"aspect{i + 1}: {b} -> {a}")
        return "\n".join(out), {"total_tokens": 12}
