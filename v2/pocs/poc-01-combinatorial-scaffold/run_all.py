"""POC-01 runner: LLM-alone vs scaffold (plan->execute->interpret) vs random floor.

Real LLM (deepseek) with caching; SimulatedLLM for --sim (tests only).
"""

import argparse
import json
import math
import os
import time

from llm_client import LLMClient, SimulatedLLM, load_api_config
from problems import CATEGORIES, PROBLEMS, score, verify_freeze
from protocol import (ProtocolError, execute, interpret_prompt, parse_answer,
                      parse_plan, plan_prompt)

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")

PREDICTED_MARGIN = 20.0  # pre-registered: >= +20pp over LLM-alone


def load_cache(name):
    p = os.path.join(CACHE, name)
    if os.path.exists(p):
        with open(p, encoding="utf-8") as f:
            return json.load(f)
    return {}


def save_cache(name, data):
    os.makedirs(CACHE, exist_ok=True)
    with open(os.path.join(CACHE, name), "w", encoding="utf-8") as f:
        json.dump(data, f, indent=1)


def cached_call(llm, cache, key, fn):
    if key in cache:
        return cache[key]
    val = fn()
    cache[key] = val
    return val


def main(sim=False, offline=False):
    assert verify_freeze(), "problems.keys.sha256 does not match PROBLEMS (freeze broken)"
    t0 = time.time()
    llm = SimulatedLLM() if sim else (LLMClient(*load_api_config()) if load_api_config() else None)
    if llm is None and not sim:
        print("no API key — use --sim or --offline")
        return 2

    plan_cache = load_cache("plans.json")
    interp_cache = load_cache("interp.json")
    alone_cache = load_cache("alone.json")

    results = {"alone": [], "scaffold": [], "random": []}
    for pid, cat, text, key in PROBLEMS:
        # random floor: seeded per problem
        import random as _r
        results["random"].append(score(str(_r.Random(pid).randint(0, 1)), key) or
                                 (isinstance(key, int) and _r.Random(pid + 1).randint(0, 3) == key))

        # LLM-alone arm
        def _alone():
            ans, _ = llm.chat([{"role": "user", "content": text + "\nAnswer exactly."}], max_tokens=32)
            return ans
        ans = cached_call(llm, alone_cache, str(pid), _alone) if not sim else _alone()
        results["alone"].append(score(parse_answer(ans), key))

        # scaffold arm
        correct = False
        try:
            plan_text = cached_call(llm, plan_cache, str(pid), lambda: llm.chat(
                plan_prompt(text), max_tokens=128)[0])
            ops = parse_plan(plan_text)
            result = execute(ops)
            interp = cached_call(llm, interp_cache, str(pid), lambda: llm.chat(
                interpret_prompt(text, result), max_tokens=32)[0])
            correct = score(parse_answer(interp), key)
        except (ProtocolError, ValueError, KeyError, IndexError):
            correct = False
        results["scaffold"].append(correct)

    if not sim:
        save_cache("plans.json", plan_cache)
        save_cache("interp.json", interp_cache)
        save_cache("alone.json", alone_cache)

    acc = {k: sum(v) / len(v) for k, v in results.items()}
    delta = (acc["scaffold"] - acc["alone"]) * 100.0
    p = mcnemar(results["alone"], results["scaffold"])
    verdict = "PASS" if (delta >= PREDICTED_MARGIN and p < 0.05) else "FAIL"

    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "accuracy.md"), "w", encoding="utf-8") as f:
        f.write("# POC-01 Accuracy\n\n| Arm | Accuracy |\n|---|---|\n")
        for k in ["scaffold", "alone", "random"]:
            f.write(f"| {k} | {acc[k]:.3f} |\n")
        f.write("\n## Per-category (scaffold / alone)\n\n| Category | Scaffold | Alone |\n|---|---|---|\n")
        for cat in CATEGORIES:
            idx = [i for i, (_, c, _, _) in enumerate(PROBLEMS) if c == cat]
            s = sum(results["scaffold"][i] for i in idx) / len(idx)
            a = sum(results["alone"][i] for i in idx) / len(idx)
            f.write(f"| {cat} | {s:.3f} | {a:.3f} |\n")

    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-01 Verdict\n\n")
        f.write(f"- scaffold: {acc['scaffold']:.3f} | alone: {acc['alone']:.3f} | random: {acc['random']:.3f}\n")
        f.write(f"- delta: {delta:+.1f}pp (criterion >= +{PREDICTED_MARGIN:.0f}pp), McNemar p={p:.4f}\n")
        f.write(f"- **VERDICT: {verdict}**\n")
        if verdict == "FAIL":
            f.write("- Kill criterion fired: the scaffold does not beat LLM-alone by the pre-registered margin.\n")

    with open(os.path.join(OUT, "claims_ledger.csv"), "w", newline="", encoding="utf-8") as f:
        import csv
        from datetime import date
        w = csv.writer(f)
        w.writerow(["metric", "value", "baseline", "split", "seed", "date"])
        w.writerow(["poc01_scaffold_acc", f"{acc['scaffold']:.4f}", f"alone:{acc['alone']:.4f}", "50 problems", "frozen", str(date.today())])
        w.writerow(["poc01_delta_pp", f"{delta:.2f}", f">={PREDICTED_MARGIN:.0f}", "50 problems", "frozen", str(date.today())])
        w.writerow(["poc01_mcnemar_p", f"{p:.4f}", "<0.05", "50 problems", "frozen", str(date.today())])

    print("=" * 60)
    print("POC-01 VERDICT")
    print("=" * 60)
    for k in ["scaffold", "alone", "random"]:
        print(f"  {k:9s} acc={acc[k]:.3f}")
    print(f"  delta {delta:+.1f}pp  McNemar p={p:.4f}  -> {verdict}")
    print(f"  elapsed {time.time()-t0:.0f}s")
    return 0


def mcnemar(alone_ok, scaffold_ok):
    b = sum(1 for a, s in zip(alone_ok, scaffold_ok) if s and not a)
    c = sum(1 for a, s in zip(alone_ok, scaffold_ok) if not s and a)
    n = b + c
    if n == 0:
        return 1.0
    k = min(b, c)
    return min(2.0 * sum(math.comb(n, i) * 0.5 ** n for i in range(k + 1)), 1.0)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--sim", action="store_true")
    ap.add_argument("--offline", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(sim=args.sim, offline=args.offline))
