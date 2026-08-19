"""T-D5/T-D6 cache, offline mode, and end-to-end (SimulatedLLM injection) tests."""

import json
import os

import numpy as np
import pytest

from corpus import CONCEPTS, QUERIES
from run_all import CACHE, main


@pytest.fixture(autouse=True)
def clean_cache(tmp_path, monkeypatch):
    monkeypatch.setattr("run_all.CACHE", str(tmp_path / "cache"))
    monkeypatch.setattr("run_all.HERE", str(tmp_path))
    yield


def test_cache_prevents_second_encode(tmp_path, monkeypatch):
    calls = {"n": 0}

    class CountingLLM:
        def __init__(self):
            self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

        def encode_concept(self, desc):
            calls["n"] += 1
            v = np.random.default_rng(abs(hash(desc)) % 2**32).normal(size=8)
            v /= np.linalg.norm(v)
            self.usage["total_tokens"] += 20
            self.usage["calls"] += 1
            return v, {"total_tokens": 20}

        def verify_candidates(self, query, candidates):
            self.usage["total_tokens"] += 10
            self.usage["calls"] += 1
            return [c[0] for c in candidates[:1]], {"total_tokens": 10}

        def full_context(self, query, descriptions):
            self.usage["total_tokens"] += 500
            self.usage["calls"] += 1
            return " ".join(descriptions)[:50], {"total_tokens": 500}

    llm = CountingLLM()
    monkeypatch.setattr("run_all.load_api_config", lambda: ("http://x", "k", "m"))
    monkeypatch.setattr("run_all.LLMClient", lambda *a, **k: llm)
    import run_all as ra

    # seed cache so no encode calls happen on first run
    cache = {str(cid): list(np.eye(8)[cid % 8]) for cid, *_ in CONCEPTS}
    qcache = {str(qid): list(np.eye(8)[qid % 8]) for qid, *_ in QUERIES}
    os.makedirs(str(tmp_path / "cache"), exist_ok=True)
    json.dump(cache, open(str(tmp_path / "cache" / "encodings.json"), "w"))
    json.dump(qcache, open(str(tmp_path / "cache" / "query_encodings.json"), "w"))
    ra.main(offline=False)
    assert calls["n"] == 0, "cache must prevent re-encoding"


def test_offline_without_cache_is_pending(tmp_path, monkeypatch):
    import run_all as ra
    monkeypatch.setattr("run_all.HERE", str(tmp_path))
    monkeypatch.setattr("run_all.CACHE", str(tmp_path / "cache"))
    vs = ra.main(offline=True)
    assert vs["d1"] is None and vs["d2"] is None and vs["d3"] is None
    with open(os.path.join(str(tmp_path), "output", "gate_summary.md"), encoding="utf-8") as f:
        assert "PENDING" in f.read()


def test_offline_with_cached_encodings_renders_baselines(tmp_path, monkeypatch):
    import run_all as ra
    cache = {str(cid): list(np.eye(8)[cid % 8]) for cid, *_ in CONCEPTS}
    qcache = {str(qid): list(np.eye(8)[qid % 8]) for qid, *_ in QUERIES}
    os.makedirs(str(tmp_path / "cache"), exist_ok=True)
    json.dump(cache, open(str(tmp_path / "cache" / "encodings.json"), "w"))
    json.dump(qcache, open(str(tmp_path / "cache" / "query_encodings.json"), "w"))
    monkeypatch.setattr("run_all.HERE", str(tmp_path))
    monkeypatch.setattr("run_all.CACHE", str(tmp_path / "cache"))
    vs = ra.main(offline=True)
    assert vs["d1"] is not None  # baselines + rubric retrieval render
    with open(os.path.join(str(tmp_path), "output", "retrieval_metrics.md"), encoding="utf-8") as f:
        content = f.read()
    for name in ["rubric", "tfidf", "bm25", "random"]:
        assert name in content
