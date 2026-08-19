"""Shared provider routing tests.  No network calls belong in this module."""

import pytest

from iching_tools.providers import (
    DEEPSEEK_BASE_URL,
    OPENROUTER_BASE_URL,
    ProviderConfigurationError,
    resolve_provider,
)


def test_deepseek_environment_selects_deepseek():
    config = resolve_provider(
        environ={"DEEPSEEK_API_KEY": "deep-key", "OPENROUTER_API_KEY": "route-key"}
    )

    assert config.provider == "deepseek"
    assert config.api_key == "deep-key"
    assert config.api_base_url == DEEPSEEK_BASE_URL
    assert config.model == "deepseek-chat"


def test_openrouter_is_fallback_when_deepseek_is_absent():
    config = resolve_provider(
        model="openai/gpt-4o-mini",
        environ={"OPENROUTER_API_KEY": "route-key"},
    )

    assert config.provider == "openrouter"
    assert config.api_key == "route-key"
    assert config.api_base_url == OPENROUTER_BASE_URL
    assert config.model == "openai/gpt-4o-mini"


def test_explicit_provider_selects_its_matching_environment_key():
    config = resolve_provider(
        explicit_provider="openrouter",
        model="openai/gpt-4o-mini",
        environ={"DEEPSEEK_API_KEY": "deep-key", "OPENROUTER_API_KEY": "route-key"},
    )

    assert config.provider == "openrouter"
    assert config.api_key == "route-key"
    assert config.model == "openai/gpt-4o-mini"


def test_explicit_key_without_provider_keeps_deepseek_compatibility():
    config = resolve_provider(
        explicit_key="flag-key",
        explicit_provider=None,
        environ={"OPENROUTER_API_KEY": "route-key"},
    )

    assert config.provider == "deepseek"
    assert config.api_key == "flag-key"
    assert config.api_base_url == DEEPSEEK_BASE_URL


def test_explicit_provider_and_key_use_the_explicit_key():
    config = resolve_provider(
        explicit_key="flag-key",
        explicit_provider="openrouter",
        environ={"OPENROUTER_API_KEY": "route-key"},
    )

    assert config.provider == "openrouter"
    assert config.api_key == "flag-key"


def test_explicit_provider_without_matching_key_is_configuration_error():
    with pytest.raises(ProviderConfigurationError, match="OPENROUTER_API_KEY"):
        resolve_provider(
            explicit_provider="openrouter",
            environ={"DEEPSEEK_API_KEY": "deep-key"},
        )


def test_no_provider_key_returns_none_when_no_provider_was_requested():
    assert resolve_provider(environ={}) is None


def test_invalid_provider_is_configuration_error_without_echoing_secrets():
    secret = "never-echo-this-key"
    with pytest.raises(ProviderConfigurationError) as exc_info:
        resolve_provider(explicit_provider="other", explicit_key=secret, environ={})

    assert secret not in str(exc_info.value)


def test_provider_repr_is_secret_safe():
    config = resolve_provider(explicit_key="never-echo-this-key", environ={})

    assert "never-echo-this-key" not in repr(config)
    assert "deepseek" in repr(config)
