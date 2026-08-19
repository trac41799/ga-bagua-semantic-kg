"""POC-08 runner: Arm A (free-form) vs Arm B (few-shot-named 8 cube positions)."""

import json
import os
import time

import numpy as np

from llm_client import LLMClient, SimulatedLLM, load_api_config
from moves import START_STATE, all_positions, describe_state
from naming import arm_a_prompt, judge_prompt, naming_prompt, parse_8
from rubric import encode_prompt, parse_encoding
from statements import STATEMENTS

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")


def load_cache(name):
    p = os.path.join(CACHE, name)
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


def save_cache(name, data):
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(os.path.join(CACHE, name), "w", encoding="utf-8"), indent=1)


def encode_text(llm, cache, key, text):
    if key not in cache:
        ans, _ = llm.chat(encode_prompt(text), max_tokens=128)
        try:
            cache[key] = parse_encoding(ans).tolist()
        except Exception:
            cache[key] = None
    return cache[key]


def diversity(vectors):
    vs = [np.array(v) for v in vectors if v is not None]
    if len(vs) < 2:
        return 0.0
    d = 0.0
    n = 0
    for i in range(len(vs)):
        for j in range(i + 1, len(vs)):
            c = float(vs[i] @ vs[j])
            d += 1.0 - c
            n += 1
    return d / n if n else 0.0


def main(real=False):
    t0 = time.time()
    cfg = load_api_config()
    llm = LLMClient(*cfg) if (real and cfg) else SimulatedLLM()
    cache = load_cache("responses.json")

    div_a, div_b, coh_b = [], [], []
    positions = all_positions(START_STATE)
    for sid, statement in enumerate(STATEMENTS):
        statement = statement["text"] if isinstance(statement, dict) else statement
        # Arm A
        key = f"A_{sid}"
        if key not in cache:
            ans, _ = llm.chat(arm_a_prompt(statement), max_tokens=512)
            try:
                cache[key] = parse_8(ans)
            except Exception:
                cache[key] = ["(protocol failure)"] * 8
        reframes_a = cache[key]
        vecs_a = [encode_text(llm, cache, f"encA_{sid}_{i}", r) for i, r in enumerate(reframes_a)]
        div_a.append(diversity(vecs_a))

        # Arm B
        reframes_b = []
        for move_name, state in positions:
            base = move_name.rstrip("0123456789")
            key = f"B_{sid}_{move_name}"
            if key not in cache:
                ans, _ = llm.chat(naming_prompt(statement, base, describe_state(state)),
                                  max_tokens=64)
                cache[key] = ans.strip()
            reframes_b.append(cache[key])
        vecs_b = [encode_text(llm, cache, f"encB_{sid}_{i}", r) for i, r in enumerate(reframes_b)]
        div_b.append(diversity(vecs_b))

        key = f"judge_{sid}"
        if key not in cache:
            ans, _ = llm.chat(judge_prompt(statement, reframes_b), max_tokens=4)
            try:
                cache[key] = int(ans.strip())
            except ValueError:
                cache[key] = 0
        coh_b.append(cache[key])
    if real:
        save_cache("responses.json", cache)

    da = float(np.mean(div_a))
    db = float(np.mean(div_b))
    cb = float(np.mean([c for c in coh_b if c > 0])) if any(c > 0 for c in coh_b) else 0.0
    delta = db - da
    verdict = "PASS" if (delta >= 0.10 and cb >= 3.5) else "FAIL"

    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-08 Verdict (reframing v2, few-shot naming)\n\n")
        f.write(f"- Arm A diversity: {da:.3f} | Arm B diversity: {db:.3f} | delta {delta:+.3f} (bar ≥ +0.10)\n")
        f.write(f"- Arm B coherence: {cb:.2f} (bar ≥ 3.5)\n")
        f.write(f"- **VERDICT: {verdict}**\n")
        f.write("\n*Re-pre-registration of POC-03: few-shot naming protocol, re-derived margins.*\n")

    print("=" * 60)
    print("POC-08 VERDICT (v2, few-shot naming)")
    print("=" * 60)
    print(f"A {da:.3f} | B {db:.3f} | delta {delta:+.3f} | coherence {cb:.2f} -> {verdict}")
    print(f"elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
