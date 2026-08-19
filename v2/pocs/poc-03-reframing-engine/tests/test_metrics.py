"""T-03.3 Metrics: diversity (mean pairwise cosine) and coherence (LLM 1-5) (AC-03.4, AC-03.5)."""

import numpy as np
import pytest

import metrics
import statements

E0 = np.array([1.0, 0, 0, 0, 0, 0, 0, 0])
E1 = np.array([0.0, 1, 0, 0, 0, 0, 0, 0])
DIAG = np.array([1.0, 1, 0, 0, 0, 0, 0, 0]) / np.sqrt(2)


def test_03_3_1_diversity_identical_vectors_zero():
    """T-03.3.1: identical vectors -> 0 (distance)."""
    assert metrics.diversity(["a", "b"], vectors=[E0, E0]) == pytest.approx(0.0)


def test_03_3_1_diversity_orthogonal_one():
    """T-03.3.1: orthogonal vectors -> 1."""
    assert metrics.diversity(["a", "b"], vectors=[E0, E1]) == pytest.approx(1.0)


def test_03_3_1_diversity_hand_computed_mean():
    """T-03.3.1: three vectors, hand-computed mean pairwise (1 - cosine)."""
    d01 = 1.0
    d02 = 1.0 - float(np.dot(E0, DIAG))          # 1 - 1/sqrt(2)
    d12 = 1.0 - float(np.dot(E1, DIAG))          # 1 - 1/sqrt(2)
    expected = (d01 + d02 + d12) / 3.0
    got = metrics.diversity(["a", "b", "c"], vectors=[E0, E1, DIAG])
    assert got == pytest.approx(expected, abs=1e-12)


def test_03_3_1_diversity_less_than_two_vectors_zero():
    assert metrics.diversity(["a"], vectors=[E0]) == pytest.approx(0.0)
    assert metrics.diversity([], vectors=[]) == pytest.approx(0.0)


def test_03_3_1_diversity_via_engine_encoder(sim_engine):
    """Encoding path: identical texts -> 0; distinct texts -> in (0, 1]."""
    metrics.set_engine(sim_engine)
    assert metrics.diversity(["same", "same"]) == pytest.approx(0.0)
    d = metrics.diversity(["A very fast battery charger", "A slow wooden table"])
    assert 0.0 < d <= 1.0


def test_03_3_2_coherence_bounded_and_deterministic(sim_engine):
    """T-03.3.2: coherence in [1, 5]; deterministic on (cached) judge responses."""
    metrics.set_engine(sim_engine)
    stmt = statements.STATEMENTS[0]["text"]
    reframes = ["one", "two", "three", "four", "five", "six", "seven", "eight"]
    s1 = metrics.coherence(stmt, reframes)
    s2 = metrics.coherence(stmt, reframes)
    assert s1 == s2
    assert 1.0 <= s1 <= 5.0
    for st in statements.STATEMENTS:
        score = metrics.coherence(st["text"], reframes)
        assert 1.0 <= score <= 5.0


def test_03_3_2_coherence_judge_failure_zero(garbage_engine):
    """Judge protocol failure: 0.0 returned and counted, not retried."""
    metrics.set_engine(garbage_engine)
    assert metrics.coherence("stmt", ["a"] * 8) == 0.0
    assert garbage_engine.failures["judge"] == 1
