"""POC-02 runner: verify blade algebra against independent brute-force math; render reports."""

import json
import os
import random

from factorial import (HEXAGRAM_NAMES, TRIGRAM_NAMES, blade_state, combinations,
                       contrast_signs, grade_of, hexagram_name, interaction,
                       main_effect, mobius_coefficients, trigram_name)

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")


def brute_force_contrast_signs(k):
    """Independent cross-check: explicit sum-of-products per effect (no algebra)."""
    runs = combinations(k)
    out = {}
    for size in range(1, k + 1):
        import itertools
        for S in itertools.combinations(range(k), size):
            signs = []
            for row in runs:
                s = 1
                for i in S:
                    s *= 1 if row[i] else -1
                signs.append(s)
            out[S] = signs
    return out


def check_contrasts(k):
    alg = contrast_signs(k)
    brute = brute_force_contrast_signs(k)
    assert set(alg) == set(brute)
    mismatches = 0
    for S in alg:
        if alg[S] != brute[S]:
            mismatches += 1
    return len(alg), mismatches


def check_decomposition(n_trials=50, max_n=6, seed=20260809):
    rng = random.Random(seed)
    worst = 0.0
    for _ in range(n_trials):
        n = rng.randint(3, max_n)
        universe = [frozenset(s) for s in _powerset(range(n))]
        values = {s: rng.uniform(-10, 10) for s in universe}
        coeffs = mobius_coefficients(lambda s: values[s], n)
        # verify by reconstruction: f(S) = sum_{T subset S} coeff[T]
        for S in universe:
            rec = sum(coeffs[T] for T in _subsets_of(S))
            err = abs(rec - values[S])
            worst = max(worst, err)
    return worst


def _powerset(items):
    items = list(items)
    for mask in range(2 ** len(items)):
        yield [items[i] for i in range(len(items)) if (mask >> i) & 1]


def _subsets_of(S):
    items = list(S)
    for mask in range(2 ** len(items)):
        yield frozenset(items[i] for i in range(len(items)) if (mask >> i) & 1)


def check_names():
    # spot checks: (upper_bits, lower_bits, expected)
    cases = [
        ((0, 0, 0), (0, 0, 0), "坤為地"),
        ((1, 1, 1), (1, 1, 1), "乾為天"),
        ((0, 1, 0), (1, 0, 1), "水火既濟"),   # upper Kan, lower Li
        ((1, 0, 1), (0, 1, 0), "火水未濟"),   # upper Li, lower Kan
        ((0, 0, 1), (0, 1, 0), "山水蒙"),     # upper Gen, lower Kan
        ((1, 1, 1), (0, 0, 0), "天地否"),     # upper Qian, lower Kun
        ((0, 0, 1), (1, 1, 0), "山澤損"),     # upper Gen, lower Dui -> 損
        ((1, 1, 0), (0, 0, 1), "澤山咸"),     # upper Dui, lower Gen -> 咸
        ((0, 1, 1), (1, 1, 1), "風天小畜"),   # upper Xun, lower Qian
        ((1, 0, 0), (1, 1, 0), "雷澤歸妹"),   # upper Zhen, lower Dui
        ((0, 0, 0), (1, 1, 1), "地天泰"),     # upper Kun, lower Qian
    ]
    bad = []
    for u, l, expected in cases:
        got = hexagram_name(u, l)
        if got != expected:
            bad.append((u, l, expected, got))
    # trigram names
    for code, name in enumerate(TRIGRAM_NAMES):
        bits = ((code >> 2) & 1, (code >> 1) & 1, code & 1)
        if trigram_name(bits) != name:
            bad.append(("trigram", code, name, trigram_name(bits)))
    return len(cases) + 8, len(bad)


def main():
    os.makedirs(OUT, exist_ok=True)
    k3_effects, k3_bad = check_contrasts(3)
    k4_effects, k4_bad = check_contrasts(4)
    worst = check_decomposition()
    names_checked, names_bad = check_names()

    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-02 Correctness Verdict\n\n")
        f.write(f"- 2³ contrasts: {k3_effects - k3_bad}/{k3_effects} exact "
                f"({'PASS' if k3_bad == 0 else 'FAIL'})\n")
        f.write(f"- 2⁴ contrasts: {k4_effects - k4_bad}/{k4_effects} exact "
                f"({'PASS' if k4_bad == 0 else 'FAIL'})\n")
        f.write(f"- Möbius decomposition (50 set functions, n=3..6): max error {worst:.2e} "
                f"({'PASS' if worst <= 1e-9 else 'FAIL'})\n")
        f.write(f"- Bagua naming: {names_checked - names_bad}/{names_checked} correct "
                f"({'PASS' if names_bad == 0 else 'FAIL'})\n")
        ok = k3_bad == 0 and k4_bad == 0 and worst <= 1e-9 and names_bad == 0
        f.write(f"\n**OVERALL: {'PASS' if ok else 'FAIL'}**\n")

    with open(os.path.join(OUT, "contrasts.md"), "w", encoding="utf-8") as f:
        f.write("# Contrast Signs (2³)\n\n| Effect | Signs |\n|---|---|\n")
        for S, signs in sorted(contrast_signs(3).items(), key=lambda kv: (len(kv[0]), kv[0])):
            f.write(f"| {S} | {''.join('+' if s > 0 else '-' for s in signs)} |\n")

    with open(os.path.join(OUT, "design_k3.md"), "w", encoding="utf-8") as f:
        f.write("# 2³ Design with Bagua Labels\n\n| Run | Bits | Trigram | Grade |\n|---|---|---|---|\n")
        for bits in combinations(3):
            blade, sign = blade_state(bits)
            f.write(f"| {sum((b << (2 - i)) for i, b in enumerate(bits))} | {''.join(str(b) for b in bits)} "
                    f"| {trigram_name(bits)} | {grade_of(bits)} |\n")

    print("=" * 60)
    print("POC-02 VERDICT")
    print("=" * 60)
    print(f"2^3 contrasts: {k3_effects - k3_bad}/{k3_effects} exact")
    print(f"2^4 contrasts: {k4_effects - k4_bad}/{k4_effects} exact")
    print(f"decomposition max err: {worst:.2e}")
    print(f"names: {names_checked - names_bad}/{names_checked}")
    ok = k3_bad == 0 and k4_bad == 0 and worst <= 1e-9 and names_bad == 0
    print(f"OVERALL: {'PASS' if ok else 'FAIL'}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
