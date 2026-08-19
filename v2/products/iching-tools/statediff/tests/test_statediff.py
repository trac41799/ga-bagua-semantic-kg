"""S1-S5 for iching_statediff (TDD `../tdd/iching-tools-tdd.md`).

SimulatedLLM only -- no network, no disk caching.
"""

import json

import pytest

import iching_statediff.cli as cli_mod
from iching_statediff import ProtocolError, parse_aspects, summarize, validate_aspects
from iching_statediff.llm_client import SimulatedLLM


class FailingSimulatedLLM(SimulatedLLM):
    """Protocol-violating variant: emits only 2 aspect lines."""

    def __init__(self):
        super().__init__(lines=2)


def test_parse_three_non_empty_lines():
    text = "latency: 120ms -> 95ms\ncache: 94% -> 99%\nerrors: 0.2% -> 0.1%"

    assert parse_aspects(text) == [
        {"aspect": "latency", "before": "120ms", "after": "95ms"},
        {"aspect": "cache", "before": "94%", "after": "99%"},
        {"aspect": "errors", "before": "0.2%", "after": "0.1%"},
    ]


@pytest.mark.parametrize("line", [
    "latency: -> 95ms",
    "cache: 94% ->",
    ": 94% -> 99%",
])
def test_parse_rejects_empty_fields(line):
    text = "\n".join([line, "errors: 0.2% -> 0.1%", "requests: 10 -> 12"])

    with pytest.raises(ProtocolError):
        parse_aspects(text)


def test_parse_rejects_duplicate_labels_casefolded():
    text = "Latency: 120ms -> 95ms\nlatency: 3 -> 4\nerrors: 0.2% -> 0.1%"

    with pytest.raises(ProtocolError, match="duplicate"):
        parse_aspects(text)


def test_parse_rejects_extra_prose_and_non_dict_rows():
    with pytest.raises(ProtocolError):
        parse_aspects("a: 1 -> 2\nb: 3 -> 4\nc: 5 -> 6\nSummary complete.")

    with pytest.raises(ProtocolError, match="list"):
        validate_aspects({})
    with pytest.raises(ProtocolError, match="dict"):
        validate_aspects([{"aspect": "a", "before": "1", "after": "2"},
                          "not a row", {"aspect": "c", "before": "5", "after": "6"}])


def test_s1_cli_sim_three_aspects(capsys):
    rc = cli_mod.main(["--before", "cache 94%, latency 120ms",
                       "--after", "cache 99%, latency 95ms", "--sim"])
    out = capsys.readouterr().out
    assert rc == 0
    aspect_lines = [ln for ln in out.splitlines() if " -> " in ln]
    assert len(aspect_lines) == 3

    result = summarize("cache 94%, latency 120ms", "cache 99%, latency 95ms",
                       SimulatedLLM())
    assert len(result["aspects"]) == 3
    for a in result["aspects"]:
        assert set(a) == {"aspect", "before", "after"}


def test_s2_cli_json_schema(capsys):
    rc = cli_mod.main(["--before", "b", "--after", "a", "--json", "--sim"])
    out = capsys.readouterr().out
    assert rc == 0
    doc = json.loads(out)
    assert set(doc) == {"before", "after", "aspects"}
    assert doc["before"] == "b"
    assert doc["after"] == "a"
    assert len(doc["aspects"]) == 3


def test_s3_protocol_violation_exit_1(capsys, monkeypatch):
    with pytest.raises(ProtocolError):
        summarize("b", "a", FailingSimulatedLLM())


def test_s3_cli_protocol_violation_exit_1(capsys, monkeypatch):
    monkeypatch.setattr(cli_mod, "SimulatedLLM", FailingSimulatedLLM)
    rc = cli_mod.main(["--before", "b", "--after", "a", "--sim"])
    captured = capsys.readouterr()
    assert rc == 1
    assert "error" in captured.err.lower()
    assert "3 aspect lines" in captured.err


def test_s4_planted_delta_roundtrip():
    before = "cache 94% latency 120ms"
    after = "cache 99% latency 95ms"
    result = summarize(before, after, SimulatedLLM())
    planted = {"aspect": "aspect2", "before": "94%", "after": "99%"}
    assert planted in result["aspects"]


def test_s5_missing_config_exit_2(capsys, monkeypatch):
    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    monkeypatch.delenv("OPENROUTER_API_KEY", raising=False)
    rc = cli_mod.main(["--before", "b", "--after", "a"])
    captured = capsys.readouterr()
    assert rc == 2
    assert "API key" in captured.err
