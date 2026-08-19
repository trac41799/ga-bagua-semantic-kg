"""TDD POC-12: X1 exact recovery, X2 strict validation, X5 import audit."""

import ast
import itertools
from pathlib import Path

import pytest

from iching_xai import XAIValidationError, identify, interaction_spectrum

PKG_DIR = Path(__file__).resolve().parent.parent / "iching_xai"

# Planted interactions from POC-07 over 6 inputs:
#   {0,1}:2.5, {3,4}:-1.75, {0,1,2}:0.9  (masks 3, 24, 7)
PLANTED = {3: 2.5, 24: -1.75, 7: 0.9}
N_INPUTS = 6


def design(k):
    """Full 2^k design at +-1 levels."""
    return list(itertools.product((-1.0, 1.0), repeat=k))


def planted_values(points):
    return [2.5 * x[0] * x[1] - 1.75 * x[3] * x[4] + 0.9 * x[0] * x[1] * x[2]
            for x in points]


def planted_spectrum():
    points = design(N_INPUTS)
    return interaction_spectrum(points, planted_values(points))


# ---------------------------------------------------------------------------
# X1: exact recovery on the POC-07 planted function (err <= 1e-9, 3/3, 0 FP)
# ---------------------------------------------------------------------------

def test_x1_spectrum_exact_recovery_on_planted():
    spectrum = planted_spectrum()
    assert len(spectrum) == 2 ** N_INPUTS
    max_err = max(abs(c - PLANTED.get(m, 0.0)) for m, c in spectrum.items())
    assert max_err <= 1e-9
    for m, c in PLANTED.items():
        assert abs(spectrum[m] - c) <= 1e-9


def test_x1_identify_returns_exactly_three_planted():
    assert identify(planted_spectrum()) == [3, 7, 24]


def test_x1_identify_no_false_positives():
    found = set(identify(planted_spectrum()))
    assert found == set(PLANTED)
    assert len(found) == 3


# ---------------------------------------------------------------------------
# X2: strict input validation -> typed errors
# ---------------------------------------------------------------------------

@pytest.mark.parametrize("bad", [0.0, 0.5, 2.0, -2.0])
def test_x2_points_with_non_plusminus_entry_rejected(bad):
    points = [[1.0, 1.0], [1.0, bad], [-1.0, 1.0], [-1.0, -1.0]]
    with pytest.raises(XAIValidationError):
        interaction_spectrum(points, [1.0, 1.0, 1.0, 1.0])


def test_x2_non_numeric_point_entry_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum([[1.0, "x"], [1.0, 1.0]], [1.0, 1.0])


def test_x2_bool_point_entry_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum([[True, 1.0], [1.0, 1.0]], [1.0, 1.0])


def test_x2_mismatched_vector_lengths_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum([[1.0, 1.0], [1.0]], [1.0, 1.0])


def test_x2_values_length_mismatch_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum(design(2), [1.0, 1.0, 1.0])


def test_x2_empty_points_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum([], [])


def test_x2_empty_values_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum(design(2), [])


def test_x2_empty_vector_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum([[], [1.0, 1.0]], [1.0, 1.0])


def test_x2_non_numeric_value_rejected():
    with pytest.raises(XAIValidationError):
        interaction_spectrum(design(2), [1.0, "a", 1.0, 1.0])


def test_x2_identify_invalid_tol_or_spectrum_rejected():
    spectrum = {3: 1.0}
    with pytest.raises(XAIValidationError):
        identify(spectrum, tol=-1.0)
    with pytest.raises(XAIValidationError):
        identify(spectrum, tol="x")
    with pytest.raises(XAIValidationError):
        identify([(3, 1.0)])
    with pytest.raises(XAIValidationError):
        identify({"a": 1.0})
    with pytest.raises(XAIValidationError):
        identify({-1: 1.0})
    with pytest.raises(XAIValidationError):
        identify({3: "a"})


# ---------------------------------------------------------------------------
# X5: import audit — stdlib only, no model/network imports
# ---------------------------------------------------------------------------

def _package_sources():
    return sorted(PKG_DIR.glob("*.py"))


def test_x5_package_imports_stdlib_only():
    allowed = {"__future__", "itertools", "numbers", "math", "typing"}
    for src in _package_sources():
        tree = ast.parse(src.read_text(encoding="utf-8"), filename=str(src))
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                for a in node.names:
                    assert a.name.split(".")[0] in allowed, \
                        f"{src}: non-stdlib import {a.name}"
            elif isinstance(node, ast.ImportFrom):
                assert node.module.split(".")[0] in allowed, \
                    f"{src}: non-stdlib import {node.module}"


def test_x5_package_has_no_llm_or_network_tokens():
    banned = ["llm", "openai", "anthropic", "langchain",
              "requests", "httpx", "urllib", "socket"]
    for src in _package_sources():
        text = src.read_text(encoding="utf-8").lower()
        for token in banned:
            assert token not in text, f"{src}: banned token {token}"


# ---------------------------------------------------------------------------
# Determinism + threshold behaviour
# ---------------------------------------------------------------------------

def test_spectrum_is_deterministic():
    points = design(N_INPUTS)
    values = planted_values(points)
    assert interaction_spectrum(points, values) == interaction_spectrum(points, values)


def test_single_pair_interaction_is_exact():
    points = [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
    values = [1.0, -1.0, -1.0, 1.0]  # y = x0*x1
    assert interaction_spectrum(points, values) == {0: 0.0, 1: 0.0, 2: 0.0, 3: 1.0}


def test_identify_tol_threshold():
    spectrum = {0: 1e-7, 1: 0.0, 3: 1.0}
    assert identify(spectrum, tol=1e-6) == [3]
    assert identify(spectrum, tol=1e-9) == [0, 3]
