"""POC-02 tests: T-02.1..T-02.5 (design, contrasts, decompose, names, reports)."""

import os
import random

import pytest

from factorial import (HEXAGRAM_NAMES, TRIGRAM_NAMES, blade_state, combinations,
                       contrast_signs, grade_of, hexagram_name, interaction,
                       main_effect, mobius_coefficients, trigram_name)


def test_combinations_3():
    assert len(combinations(3)) == 8
    assert combinations(3)[0] == (0, 0, 0)
    assert combinations(3)[7] == (1, 1, 1)


def test_combinations_4():
    assert len(combinations(4)) == 16


def test_blade_state_natural_convention():
    assert blade_state((1, 0, 1)) == (6, -1)   # e13 = -e31
    assert blade_state((1, 1, 0)) == (4, +1)   # e12
    assert blade_state((0, 1, 1)) == (5, +1)   # e23
    assert blade_state((0, 0, 0)) == (0, +1)
    assert blade_state((1, 1, 1)) == (7, +1)


def test_grade_is_hamming_weight():
    for bits in combinations(3):
        assert grade_of(bits) == sum(bits)


def test_contrast_signs_2_3_exact_vs_brute():
    from run_all import brute_force_contrast_signs
    alg = contrast_signs(3)
    brute = brute_force_contrast_signs(3)
    assert set(alg) == set(brute)
    for S in alg:
        assert alg[S] == brute[S]


def test_contrast_signs_2_4_exact_vs_brute():
    from run_all import brute_force_contrast_signs
    alg = contrast_signs(4)
    brute = brute_force_contrast_signs(4)
    assert set(alg) == set(brute)
    for S in alg:
        assert alg[S] == brute[S]


def test_effect_counts():
    assert len(contrast_signs(3)) == 7
    assert len(contrast_signs(4)) == 15


def test_main_effect_hand_table():
    # 2^2 table: factor A, B ; responses y = 10 + 4A + 2B (no interaction)
    data = [((0, 0), 10), ((1, 0), 14), ((0, 1), 12), ((1, 1), 16)]
    assert main_effect(data, 0) == pytest.approx(4.0)
    assert main_effect(data, 1) == pytest.approx(2.0)


def test_interaction_hand_table():
    # interaction AB = (y11 - y10 - y01 + y00)/2 = 6  ->  y11 = 24
    data = [((0, 0), 10), ((1, 0), 11), ((0, 1), 11), ((1, 1), 24)]
    assert interaction(data, (0, 1)) == pytest.approx(6.0)


def test_mobius_roundtrip():
    rng = random.Random(3)
    n = 4
    universe = [frozenset(s) for s in _powerset(range(n))]
    values = {s: rng.uniform(-5, 5) for s in universe}
    coeffs = mobius_coefficients(lambda s: values[s], n)
    for S in universe:
        rec = sum(coeffs[T] for T in _subsets_of(S))
        assert abs(rec - values[S]) < 1e-9


def test_mobius_50_functions_max_err():
    from run_all import check_decomposition
    assert check_decomposition(50, 6, seed=20260809) <= 1e-9


def test_trigram_names_all():
    for code, name in enumerate(TRIGRAM_NAMES):
        bits = ((code >> 2) & 1, (code >> 1) & 1, code & 1)
        assert trigram_name(bits) == name


@pytest.mark.parametrize("u,l,expected", [
    ((0, 0, 0), (0, 0, 0), "坤為地"),
    ((1, 1, 1), (1, 1, 1), "乾為天"),
    ((0, 1, 0), (1, 0, 1), "水火既濟"),
    ((1, 0, 1), (0, 1, 0), "火水未濟"),
    ((0, 0, 1), (0, 1, 0), "山水蒙"),
    ((1, 1, 1), (0, 0, 0), "天地否"),
    ((0, 0, 1), (1, 1, 0), "山澤損"),
    ((1, 1, 0), (0, 0, 1), "澤山咸"),
    ((0, 1, 1), (1, 1, 1), "風天小畜"),
    ((1, 0, 0), (1, 1, 0), "雷澤歸妹"),
    ((0, 0, 0), (1, 1, 1), "地天泰"),
])
def test_hexagram_spot_checks(u, l, expected):
    assert hexagram_name(u, l) == expected


def test_hexagram_table_shape():
    assert len(HEXAGRAM_NAMES) == 8
    assert all(len(row) == 8 for row in HEXAGRAM_NAMES)
    assert len(set(name for row in HEXAGRAM_NAMES for name in row)) == 64


def test_reports_render(tmp_path):
    import run_all as ra
    import types
    # run the verification into the tmp dir by calling internals
    k3_effects, k3_bad = ra.check_contrasts(3)
    k4_effects, k4_bad = ra.check_contrasts(4)
    worst = ra.check_decomposition()
    names_checked, names_bad = ra.check_names()
    assert k3_bad == 0 and k4_bad == 0
    assert worst <= 1e-9
    assert names_bad == 0


def test_determinism():
    from run_all import check_decomposition
    a = check_decomposition(50, 6, seed=20260809)
    b = check_decomposition(50, 6, seed=20260809)
    assert a == b


def _powerset(items):
    items = list(items)
    for mask in range(2 ** len(items)):
        yield [items[i] for i in range(len(items)) if (mask >> i) & 1]


def _subsets_of(S):
    items = list(S)
    for mask in range(2 ** len(items)):
        yield frozenset(items[i] for i in range(len(items)) if (mask >> i) & 1)
