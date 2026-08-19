"""POC-09 runner: Arm A (plain) vs Arm B (hexagram-framed scaffold), blind rater."""

import json
import os
import time

from llm_client import LLMClient, SimulatedLLM, load_api_config
from scenarios import SCENARIOS, arm_a_prompt, arm_b_prompt, rater_prompt

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")


def load_cache(name):
    p = os.path.join(CACHE, name)
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


def save_cache(name, data):
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(os.path.join(CACHE, name), "w", encoding="utf-8"), indent=1)


def has_trigram_pair(text):
    """Arm-B protocol check: output references a trigram pair (e.g., '坎 over 離' or 'over')."""
    t = text.lower()
    return ("over" in t) or ("upper" in t and "lower" in t)


def main(real=False):
    t0 = time.time()
    cfg = load_api_config()
    llm = LLMClient(*cfg) if (real and cfg) else SimulatedLLM()
    cache = load_cache("responses.json")

    scores = {"A": [], "B": []}
    framed_ok = 0
    details = []
    for sid, domain, situation, answer in SCENARIOS:
        expl = {}
        for arm, prompt_fn in (("A", arm_a_prompt), ("B", arm_b_prompt)):
            key = f"{sid}_{arm}"
            if key not in cache:
                e, _ = llm.chat(prompt_fn(situation, answer), max_tokens=256)
                cache[key] = e
            expl[arm] = cache[key]
            if arm == "B" and has_trigram_pair(expl[arm]):
                framed_ok += 1
            rkey = f"{sid}_rate_{arm}"
            if rkey not in cache:
                r, _ = llm.chat(rater_prompt(situation, expl[arm]), max_tokens=4)
                try:
                    cache[rkey] = int(r.strip())
                except ValueError:
                    cache[rkey] = 1
            scores[arm].append(cache[rkey])
        details.append((sid, domain, scores["A"][-1], scores["B"][-1]))
    if real:
        save_cache("responses.json", cache)

    mA = sum(scores["A"]) / len(scores["A"])
    mB = sum(scores["B"]) / len(scores["B"])
    delta = mB - mA
    verdict = "PASS" if delta >= 0.5 else "FAIL"

    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-09 Verdict\n\n")
        f.write(f"- Arm A (plain) mean comprehension+trust: {mA:.2f}\n")
        f.write(f"- Arm B (hexagram-framed scaffold) mean: {mB:.2f}\n")
        f.write(f"- delta (B − A): {delta:+.2f} (bar ≥ +0.5) -> **{verdict}**\n")
        f.write(f"- Arm B framing compliance: {framed_ok}/20\n")
        f.write("\n| id | domain | A | B |\n|---|---|---|---|\n")
        for sid, domain, a, b in details:
            f.write(f"| {sid} | {domain} | {a} | {b} |\n")
        f.write("\n*LLM-rater proxy (blind); human gate separate. Structure used as writing scaffold only.*\n")

    print("=" * 60)
    print("POC-09 VERDICT")
    print("=" * 60)
    print(f"A {mA:.2f} | B {mB:.2f} | delta {delta:+.2f} -> {verdict} | framing {framed_ok}/20")
    print(f"elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
