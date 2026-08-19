"""C1-C6 for the iching_coverage tool (SimulatedLLM only, no network)."""

import json
import os
import subprocess
import sys

import pytest

import iching_coverage as ic
from iching_coverage import SimulatedLLM, audit, audit_prompt, resolve_api_key
from iching_coverage import cli

ROLES = [
    "receptive",
    "causal",
    "transmissive",
    "constraining",
    "clarifying",
    "influential",
    "balancing",
    "generative",
]

COVERAGE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def run_cli(args, env_extra=None):
    env = {
        k: v
        for k, v in os.environ.items()
        if k not in ("DEEPSEEK_API_KEY", "OPENROUTER_API_KEY")
    }
    env.setdefault("PYTHONIOENCODING", "utf-8")
    if env_extra:
        env.update(env_extra)
    return subprocess.run(
        [sys.executable, "-m", "iching_coverage", *args],
        cwd=COVERAGE_DIR,
        env=env,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )


class TestC1CliSimHappyPath:
    def test_exit_0_and_8_role_checklist_prompt(self):
        r = run_cli(["--task", "launch an API product", "--plan", "Build it.", "--sim"])
        assert r.returncode == 0
        assert all(role in r.stdout for role in ROLES)


class TestC2CliJsonSchema:
    def test_json_parses_with_documented_keys(self):
        r = run_cli(["--task", "t", "--plan", "p", "--json", "--sim"])
        assert r.returncode == 0
        data = json.loads(r.stdout)
        assert set(data.keys()) == {
            "task",
            "original_plan",
            "audited_plan",
            "checklist",
        }
        assert data["task"] == "t"
        assert data["original_plan"] == "p"
        assert data["checklist"] is True
        assert isinstance(data["audited_plan"], str) and data["audited_plan"]


class TestC3CliMissingConfig:
    def test_exit_2_stderr_mentions_api_key(self):
        r = run_cli(["--task", "t", "--plan", "p"])
        assert r.returncode == 2
        assert "API key" in r.stderr


class TestC4ApiKeyFlagPreferred:
    def test_resolve_api_key_prefers_flag(self, monkeypatch):
        monkeypatch.setenv("DEEPSEEK_API_KEY", "envkey")
        assert resolve_api_key(api_key="flagkey") == "flagkey"

    def test_resolve_api_key_env_fallback(self, monkeypatch):
        monkeypatch.setenv("DEEPSEEK_API_KEY", "envkey")
        assert resolve_api_key() == "envkey"
        monkeypatch.delenv("DEEPSEEK_API_KEY")
        monkeypatch.setenv("OPENROUTER_API_KEY", "orkey")
        assert resolve_api_key() == "orkey"
        monkeypatch.delenv("OPENROUTER_API_KEY")
        assert resolve_api_key() is None

    def test_cli_passes_flag_key_to_client(self, monkeypatch):
        captured = {}

        class Recorder(SimulatedLLM):
            def __init__(self, api_key=None, model=None):
                captured["api_key"] = api_key
                captured["model"] = model

            def complete(self, messages):
                return "\n".join(f"{role}: covered" for role in ROLES)

        monkeypatch.setattr(cli, "LLMClient", Recorder)
        monkeypatch.setenv("DEEPSEEK_API_KEY", "envkey")
        rc = cli.main(["--task", "t", "--plan", "p", "--api-key", "flagkey"])
        assert rc == 0
        assert captured["api_key"] == "flagkey"
        assert captured["model"] == "deepseek-chat"

    def test_legacy_client_fallback_preserves_provider_base_url(self, monkeypatch):
        captured = {}

        class LegacyClient:
            def __init__(self, api_key=None, model=None, base_url=None):
                captured.update(api_key=api_key, model=model, base_url=base_url)

            def complete(self, messages):
                return "\n".join(f"{role}: covered" for role in ROLES)

        monkeypatch.setattr(cli, "LLMClient", LegacyClient)
        monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
        monkeypatch.setenv("OPENROUTER_API_KEY", "route-key")
        rc = cli.main([
            "--task", "t", "--plan", "p", "--provider", "openrouter", "--json",
        ])

        assert rc == 0
        assert captured["api_key"] == "route-key"
        assert captured["base_url"] == "https://openrouter.ai/api/v1"


class TestC7CliProtocolFailure:
    def test_invalid_audit_output_exits_1_without_success_json(self, monkeypatch, capsys):
        class InvalidLLM:
            def __init__(self, api_key=None, model=None):
                pass

            def complete(self, messages):
                return "prose without role markers"

        monkeypatch.setattr(cli, "LLMClient", InvalidLLM)
        monkeypatch.setenv("DEEPSEEK_API_KEY", "envkey")

        rc = cli.main(["--task", "t", "--plan", "p", "--json"])
        captured = capsys.readouterr()

        assert rc == 1
        assert captured.out == ""
        assert "audit protocol failure" in captured.err
        assert "audited_plan" not in captured.err


class TestC5PromptContains8Roles:
    def test_audit_prompt_contains_all_roles(self):
        messages = audit_prompt("task", "plan")
        text = "\n".join(str(m["content"]) for m in messages)
        for role in ROLES:
            assert role in text
        assert "rewrite" in text or "complete" in text

    def test_audit_result_schema_and_roles(self):
        result = audit("t", "p", SimulatedLLM())
        assert set(result.keys()) == {
            "task",
            "original_plan",
            "audited_plan",
            "checklist",
        }
        assert result["checklist"] is True
        for role in ROLES:
            assert role in result["audited_plan"]


class TestC6LlmFailure:
    def test_exit_1(self, monkeypatch):
        class FailingSimulatedLLM(SimulatedLLM):
            def complete(self, messages):
                raise RuntimeError("llm down")

        monkeypatch.setattr(cli, "SimulatedLLM", FailingSimulatedLLM)
        rc = cli.main(["--task", "t", "--plan", "p", "--sim"])
        assert rc == 1
