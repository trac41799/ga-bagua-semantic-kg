"""Shared fixtures for POC-03 tests. SimulatedLLM ONLY in tests (L4)."""

import argparse

import pytest

import metrics
import naming


class GarbageBackend:
    """Chat backend that always returns invalid JSON (protocol-failure tests)."""

    usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0}

    def chat(self, messages, max_tokens=None, temperature=0.0):
        return "definitely not json {{{", {"prompt_tokens": 5, "completion_tokens": 3,
                                           "total_tokens": 8}


@pytest.fixture
def sim_engine():
    from llm_client import SimulatedLLM
    return SimulatedLLM()


@pytest.fixture
def garbage_engine():
    from llm_client import RealEngine
    return RealEngine(GarbageBackend())


@pytest.fixture(autouse=True)
def _reset_engines():
    yield
    naming.set_engine(None)
    metrics.set_engine(None)


@pytest.fixture
def run_args(tmp_path):
    """argparse-style Namespace for run_all.run() pointing at tmp dirs."""
    args = argparse.Namespace(sim=True, offline=False,
                              cache_dir=str(tmp_path / "cache"),
                              output_dir=str(tmp_path / "out"),
                              run_id=None)
    return args
