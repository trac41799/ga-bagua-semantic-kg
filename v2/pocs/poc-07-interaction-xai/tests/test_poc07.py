"""POC-07 tests: math recovery + naming strings + runner."""

import pytest

from run_all import (FACTOR_NAMES, PLANTED, blackbox, explain,
                     identify_planted, interaction_coefficients)


def test_blackbox_planted_structure():
    # x = [1,1,0,0,0,0]: main 1.5 + e01 2.5 + e012 0.9*0 (x2=0 kills 3-way) = 4.0
    x = [1.0, 1.0, 0, 0, 0, 0]
    assert blackbox(x) == pytest.approx(4.0)
    # x = [1,1,1,0,0,0]: 1.5 + 2.5 + 0.9 = 4.9
    x3 = [1.0, 1.0, 1.0, 0, 0, 0]
    assert blackbox(x3) == pytest.approx(4.9)
    x2 = [0, 0, 0, 1.0, 1.0, 0]
    assert blackbox(x2) == pytest.approx(0.7 - 1.75)


def test_spectrum_recovers_planted_exactly():
    coeffs = interaction_coefficients(blackbox, 6)
    for S, c in PLANTED.items():
        assert abs(coeffs[S] - c) <= 1e-9, (S, coeffs[S], c)


def test_identification_exact():
    coeffs = interaction_coefficients(blackbox, 6)
    found, planted = identify_planted(coeffs)
    assert found == planted


def test_no_false_positives():
    coeffs = interaction_coefficients(blackbox, 6)
    found, _ = identify_planted(coeffs)
    assert len(found) == 3
    assert found == set(PLANTED)


def test_naming_differs_only_in_label():
    b = explain(frozenset({0, 1, 2}), FACTOR_NAMES, bagua=True)
    n = explain(frozenset({0, 1, 2}), FACTOR_NAMES, bagua=False)
    assert "2-way" not in b and "3-way" in b
    assert "interaction" in n and "interaction" in b
    # same math facts: both name the factors
    for f in ["latency", "cache_miss", "concurrency"]:
        assert f in b and f in n


def test_runner_sim():
    import run_all
    assert run_all.main() == 0
    import os
    assert os.path.exists(os.path.join("output", "verdict.md"))
