"""Shared, secret-safe provider resolution for the LLM-backed tools."""

from dataclasses import dataclass, field
import os
from typing import Literal, Mapping


ProviderName = Literal["deepseek", "openrouter"]
DEEPSEEK_BASE_URL = "https://api.deepseek.com"
OPENROUTER_BASE_URL = "https://openrouter.ai/api/v1"

_ENV_KEYS = {
    "deepseek": "DEEPSEEK_API_KEY",
    "openrouter": "OPENROUTER_API_KEY",
}
_BASE_URLS = {
    "deepseek": DEEPSEEK_BASE_URL,
    "openrouter": OPENROUTER_BASE_URL,
}


class ProviderConfigurationError(ValueError):
    """Raised for an explicitly requested provider that cannot be configured."""


@dataclass(frozen=True)
class ProviderConfig:
    """Resolved provider settings with the credential omitted from ``repr``."""

    provider: ProviderName
    api_key: str = field(repr=False)
    api_base_url: str
    model: str


def resolve_provider(
    *,
    explicit_key: str | None = None,
    explicit_provider: str | None = None,
    model: str = "deepseek-chat",
    environ: Mapping[str, str] | None = None,
) -> ProviderConfig | None:
    """Resolve one provider using deterministic flag-then-environment rules.

    An explicit provider selects only its matching environment variable. An
    explicit key without a provider keeps the historical DeepSeek behavior.
    When neither a provider nor a key is requested and no environment key is
    present, ``None`` is returned so CLI callers can choose their exit policy.
    """
    if not isinstance(model, str) or not model:
        raise ProviderConfigurationError("model must be a non-empty string")

    env = os.environ if environ is None else environ
    if env.get("ICHING_MODEL"):
        model = env["ICHING_MODEL"]
        if not isinstance(model, str) or not model:
            raise ProviderConfigurationError("model must be a non-empty string")
    if explicit_provider is not None:
        if explicit_provider not in _ENV_KEYS:
            raise ProviderConfigurationError(
                "unknown provider: expected deepseek or openrouter"
            )
        provider = explicit_provider
        key = explicit_key or env.get(_ENV_KEYS[provider])
        if not key:
            raise ProviderConfigurationError(
                f"no API key for {provider}: set {_ENV_KEYS[provider]}"
            )
    elif explicit_key:
        provider = "deepseek"
        key = explicit_key
    else:
        if env.get(_ENV_KEYS["deepseek"]):
            provider = "deepseek"
            key = env[_ENV_KEYS[provider]]
        elif env.get(_ENV_KEYS["openrouter"]):
            provider = "openrouter"
            key = env[_ENV_KEYS[provider]]
        else:
            return None

    return ProviderConfig(
        provider=provider,
        api_key=key,
        api_base_url=_BASE_URLS[provider],
        model=model,
    )
