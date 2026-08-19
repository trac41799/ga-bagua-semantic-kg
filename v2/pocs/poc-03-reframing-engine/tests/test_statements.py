"""Benchmark freeze: 20 statements, 5 per domain, sha256 marker (pre-registration)."""

from collections import Counter

import statements


def test_frozen_benchmark_shape():
    assert len(statements.STATEMENTS) == 20
    assert len({s["id"] for s in statements.STATEMENTS}) == 20
    counts = Counter(s["domain"] for s in statements.STATEMENTS)
    assert counts == {d: 5 for d in statements.DOMAINS}
    for s in statements.STATEMENTS:
        assert isinstance(s["text"], str) and len(s["text"]) > 20


def test_freeze_marker_matches_file():
    marker = statements._FREEZE_PATH.read_text(encoding="utf-8").strip()
    assert marker == statements.freeze_marker()


def test_canonical_json_is_stable():
    assert statements.canonical_json() == statements.canonical_json()
