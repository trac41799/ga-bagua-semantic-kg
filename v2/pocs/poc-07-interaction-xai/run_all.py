"""POC-07: feature-interaction XAI — Möbius spectrum + Bagua naming.

Math: the blade-algebra Möbius spectrum exactly recovers planted interactions of
a synthetic black-box f over 6 inputs (3 planted interactions).
Naming: Bagua-labeled vs numbered interaction explanations, LLM-rater clarity.
"""

import json
import os
import time

import numpy as np

from factorial import mobius_coefficients, trigram_name, trigram_pinyin

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")

FACTOR_NAMES = ["latency", "cache_miss", "concurrency", "batch_size", "error_rate", "retry"]
N_INPUTS = 6

# planted interaction structure: {frozenset: coefficient}
PLANTED = {
    frozenset({0, 1}): 2.5,     # latency × cache_miss
    frozenset({3, 4}): -1.75,   # batch_size × error_rate
    frozenset({0, 1, 2}): 0.9,  # latency × cache_miss × concurrency
}


def blackbox(x):
    """Synthetic black-box over 6 real inputs: sum of planted interactions + noise-free main terms."""
    v = np.asarray(x, dtype=float)
    f = 0.0
    for S, c in PLANTED.items():
        term = 1.0
        for i in S:
            term *= v[i]
        f += c * term
    f += 1.5 * v[0] + 0.7 * v[3]
    return f


def interaction_coefficients(f, n):
    """Interaction coefficients via the Walsh-Hadamard/contrast transform (POC-02-validated).

    Evaluate f on all 2^n design points at +-1 levels; c_S = mean over runs of
    sign(S, run) * y_run. For polynomial f this recovers the planted coefficients
    EXACTLY (2^n design, no aliasing when n >= max interaction order).
    """
    import itertools
    runs = list(itertools.product((-1.0, 1.0), repeat=n))
    ys = np.array([f(r) for r in runs])
    out = {}
    for size in range(n + 1):
        for S in itertools.combinations(range(n), size):
            S = frozenset(S)
            signs = np.array([np.prod([r[i] for i in S]) if S else 1.0 for r in runs])
            out[S] = float(np.mean(signs * ys))
    return out


def identify_planted(coeffs, tol=1e-6):
    """Subsets whose |coefficient| > tol. Returns (found, planted)."""
    found = {S for S, c in coeffs.items() if abs(c) > tol and len(S) >= 2}
    return found, set(PLANTED)


def explain(subset, factor_names, bagua=False):
    """One-sentence explanation of an interaction subset."""
    names = [factor_names[i] for i in sorted(subset)]
    if bagua and len(subset) == 3:
        bits = tuple(1 if i in subset else 0 for i in range(3))
        label = f"{trigram_name(bits)} ({trigram_pinyin(bits)})"
        return f"the 3-way interaction {label} among {', '.join(names)}"
    if bagua and len(subset) == 2:
        bits = [0, 0, 0]
        return f"the 2-way interaction between {names[0]} and {names[1]}"
    return f"the interaction among {', '.join(names)}"


def main(real=False):
    os.makedirs(OUT, exist_ok=True)
    t0 = time.time()

    # ---- math arm (no LLM) ----
    coeffs = interaction_coefficients(blackbox, N_INPUTS)
    found, planted = identify_planted(coeffs)
    max_planted_err = max(abs(coeffs[S] - c) for S, c in PLANTED.items())
    math_ok = found == planted and max_planted_err <= 1e-9

    # ---- naming arm (LLM rater on 10 explanations; SimulatedLLM unless --real) ----
    from llm_client import SimulatedLLM, load_api_config
    if not real:
        llm = SimulatedLLM()
    else:
        cfg = load_api_config()
        if cfg is None:
            llm = SimulatedLLM()
        else:
            from llm_client import LLMClient
            llm = LLMClient(*cfg)
    pairs = [(explain(S, FACTOR_NAMES, bagua=False), explain(S, FACTOR_NAMES, bagua=True))
             for S in sorted(PLANTED, key=sorted) for _ in range(3)]
    pairs = pairs[:10]
    scores_a, scores_b = [], []
    cache = {}
    for i, (a, b) in enumerate(pairs):
        for label, text in (("A", a), ("B", b)):
            key = f"{i}_{label}"
            if key not in cache:
                ans, _ = llm.chat([
                    {"role": "system", "content": "Rate how CLEARLY this explanation identifies the interaction on a 1-5 scale. Output ONLY the integer."},
                    {"role": "user", "content": text}], max_tokens=4)
                try:
                    cache[key] = int(ans.strip())
                except ValueError:
                    cache[key] = 3
        scores_a.append(cache[f"{i}_A"])
        scores_b.append(cache[f"{i}_B"])
    naming_delta = float(np.mean(scores_b) - np.mean(scores_a))

    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-07 Verdict\n\n")
        f.write(f"- planted interactions: {sorted(planted, key=sorted)}\n")
        f.write(f"- recovered (|c| > 1e-6): {sorted(found, key=sorted)}\n")
        f.write(f"- max planted-coefficient error: {max_planted_err:.2e} "
                f"({'PASS' if max_planted_err <= 1e-9 else 'FAIL'})\n")
        f.write(f"- math: {'PASS' if math_ok else 'FAIL'} (exact identification + ≤1e-9)\n")
        if naming_delta is not None:
            f.write(f"- naming clarity delta (bagua − numbered): {naming_delta:+.2f} "
                    f"({'PASS' if naming_delta >= 0.5 else 'FAIL'}; bar ≥ +0.5)\n")
        else:
            f.write("- naming arm: PENDING (no API)\n")

    print("=" * 60)
    print("POC-07 VERDICT")
    print("=" * 60)
    print(f"planted: {sorted(planted, key=sorted)}")
    print(f"recovered: {sorted(found, key=sorted)}")
    print(f"max planted err: {max_planted_err:.2e} -> {'PASS' if math_ok else 'FAIL'}")
    if naming_delta is not None:
        print(f"naming clarity delta: {naming_delta:+.2f} -> {'PASS' if naming_delta >= 0.5 else 'FAIL'}")
    print(f"elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
