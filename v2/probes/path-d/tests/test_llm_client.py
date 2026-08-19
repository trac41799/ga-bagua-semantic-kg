"""T-D2 LLM client tests (SimulatedLLM + mocked real client)."""

import numpy as np
import pytest

from llm_client import BudgetError, LLMClient, SimulatedLLM


def test_simulated_deterministic():
    s1, s2 = SimulatedLLM(), SimulatedLLM()
    a, _ = s1.encode_concept("Rate Limiter restricts requests")
    b, _ = s2.encode_concept("Rate Limiter restricts requests")
    assert np.allclose(a, b, atol=1e-12)


def test_simulated_encode_normalized():
    s = SimulatedLLM()
    v, usage = s.encode_concept("Message Queue relays events")
    assert np.linalg.norm(v) == pytest.approx(1.0)
    assert usage["total_tokens"] > 0


def test_simulated_verify_returns_ids():
    s = SimulatedLLM()
    chosen, _ = s.verify_candidates("which components restrict access", [(0, "Access Guard"), (1, "Message Queue")])
    assert 0 in chosen
    assert 1 not in chosen


def test_mock_transport_real_client():
    class FakeResp:
        def __enter__(self):
            return self

        def __exit__(self, *a):
            return False

        def read(self):
            import json
            return json.dumps({"choices": [{"message": {"content": "[0.5,0,0,0,0,0,0,0.5]"}}],
                               "usage": {"prompt_tokens": 10, "completion_tokens": 5,
                                         "total_tokens": 15}}).encode()

    import urllib.request
    orig = urllib.request.urlopen
    urllib.request.urlopen = lambda req, timeout: FakeResp()
    try:
        c = LLMClient("http://x", "key")
        text, usage = c.chat([{"role": "user", "content": "hi"}])
        assert usage["total_tokens"] == 15
        assert c.usage["total_tokens"] == 15
        assert c.usage["calls"] == 1
        assert "[0.5" in text
    finally:
        urllib.request.urlopen = orig


def test_budget_cap():
    class BigResp:
        def __enter__(self): return self
        def __exit__(self, *a): return False
        def read(self):
            import json
            return json.dumps({"choices": [{"message": {"content": "x"}}],
                               "usage": {"prompt_tokens": 10_000_000, "completion_tokens": 0,
                                         "total_tokens": 10_000_000}}).encode()

    import urllib.request
    orig = urllib.request.urlopen
    urllib.request.urlopen = lambda req, timeout: BigResp()
    try:
        c = LLMClient("http://x", "key", budget_tokens=1000)
        with pytest.raises(BudgetError):
            c.chat([{"role": "user", "content": "hi"}])
    finally:
        urllib.request.urlopen = orig
