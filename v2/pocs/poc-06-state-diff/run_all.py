"""POC-06 runner: Arm A (free-form summary) vs Arm B (3-aspect structured template), blind rater.

Comprehension rater: 1-5, LLM-blind, judged on coverage of the planted deltas.
"""

import json
import os
import time

from llm_client import LLMClient, SimulatedLLM, load_api_config
from transitions import TRANSITIONS, deltas_present

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")

ARM_A = "Write a short prose summary of what changed between the two states."
ARM_B = ("Write a structured summary using EXACTLY 3 aspect lines of the form "
         "'aspect: before -> after'. Cover the three most important changes, one per line. "
         "No prose beyond the 3 lines.")


def load_cache(name):
    p = os.path.join(CACHE, name)
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


def save_cache(name, data):
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(os.path.join(CACHE, name), "w", encoding="utf-8"), indent=1)


def rater_prompt(before, after, summary):
    return [{"role": "system", "content": "Rate COMPREHENSION: how completely does this summary "
                                          "capture the state change, on a 1-5 scale? Output ONLY the integer."},
            {"role": "user", "content": f"Before: {before}\nAfter: {after}\nSummary:\n{summary}"}]


def main(real=False):
    t0 = time.time()
    cfg = load_api_config()
    llm = LLMClient(*cfg) if (real and cfg) else SimulatedLLM()
    cache = load_cache("responses.json")

    scores = {"A": [], "B": []}
    auto = {"A": [], "B": []}
    details = []
    for tid, domain, before, after, planted in TRANSITIONS:
        summaries = {}
        for arm, sys_prompt in (("A", ARM_A), ("B", ARM_B)):
            key = f"{tid}_{arm}"
            if key not in cache:
                s, _ = llm.chat([{"role": "system", "content": sys_prompt},
                                 {"role": "user", "content": f"Before: {before}\nAfter: {after}"}],
                                max_tokens=128)
                cache[key] = s
            summaries[arm] = cache[key]
            auto[arm].append(deltas_present(summaries[arm], planted))
            rkey = f"{tid}_rate_{arm}"
            if rkey not in cache:
                r, _ = llm.chat(rater_prompt(before, after, summaries[arm]), max_tokens=4)
                try:
                    cache[rkey] = int(r.strip())
                except ValueError:
                    cache[rkey] = 1
            scores[arm].append(cache[rkey])
        details.append((tid, domain, scores["A"][-1], scores["B"][-1], auto["A"][-1], auto["B"][-1]))
    if real:
        save_cache("responses.json", cache)

    mA = sum(scores["A"]) / len(scores["A"])
    mB = sum(scores["B"]) / len(scores["B"])
    delta = mB - mA
    verdict = "PASS" if delta >= 0.5 else "FAIL"

    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-06 Verdict\n\n")
        f.write(f"- Arm A (free-form) mean comprehension: {mA:.2f}\n")
        f.write(f"- Arm B (3-aspect structured) mean comprehension: {mB:.2f}\n")
        f.write(f"- delta (B − A): {delta:+.2f} (bar ≥ +0.5) -> **{verdict}**\n")
        f.write(f"- automated planted-delta coverage: A {sum(auto['A'])/len(auto['A']):.2f} | "
                f"B {sum(auto['B'])/len(auto['B']):.2f}\n")
        f.write("\n| id | domain | A | B | A-auto | B-auto |\n|---|---|---|---|---|---|\n")
        for tid, domain, a, b, aa, ba in details:
            f.write(f"| {tid} | {domain} | {a} | {b} | {aa:.2f} | {ba:.2f} |\n")
        f.write("\n*LLM-rater proxy (blind); human gate is a separate manual step.*\n")

    print("=" * 60)
    print("POC-06 VERDICT")
    print("=" * 60)
    print(f"A: {mA:.2f} | B: {mB:.2f} | delta {delta:+.2f} -> {verdict}")
    print(f"auto coverage A {sum(auto['A'])/len(auto['A']):.2f} | B {sum(auto['B'])/len(auto['B']):.2f}")
    print(f"elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
