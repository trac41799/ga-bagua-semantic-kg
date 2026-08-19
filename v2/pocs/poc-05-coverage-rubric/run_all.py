"""POC-05 runner: Arm A (free-form) vs Arm B (8-role coverage checklist), blind 6-aspect audit."""

import json
import os
import time

from llm_client import LLMClient, SimulatedLLM, load_api_config
from protocol import TASKS, arm_a_prompt, arm_b_prompt, audit_prompt, parse_audit

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")


def load_cache(name):
    p = os.path.join(CACHE, name)
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


def save_cache(name, data):
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(os.path.join(CACHE, name), "w", encoding="utf-8"), indent=1)


def main(real=False):
    t0 = time.time()
    cfg = load_api_config()
    llm = LLMClient(*cfg) if (real and cfg) else SimulatedLLM()
    cache = load_cache("responses.json")

    missing = {"A": [], "B": []}
    details = []
    for tid, domain, task in TASKS:
        plans = {}
        audits = {}
        for arm, prompt_fn in (("A", arm_a_prompt), ("B", arm_b_prompt)):
            key = f"{tid}_{arm}"
            if key not in cache:
                plan, _ = llm.chat(prompt_fn(task), max_tokens=256)
                cache[key] = plan
            plan = cache[key]
            plans[arm] = plan
            rkey = f"{tid}_audit_{arm}"
            if rkey not in cache:
                aud, _ = llm.chat(audit_prompt(task, plan), max_tokens=128)
                try:
                    cache[rkey] = parse_audit(aud)
                except Exception:
                    cache[rkey] = [0] * 6  # audit failure counted as ALL aspects missing (honest)
            audits[arm] = cache[rkey]
        missing["A"].append(6 - sum(audits["A"]))
        missing["B"].append(6 - sum(audits["B"]))
        details.append((tid, domain, missing["A"][-1], missing["B"][-1]))
    if real:
        save_cache("responses.json", cache)

    mA = sum(missing["A"]) / len(missing["A"])
    mB = sum(missing["B"]) / len(missing["B"])
    delta = mA - mB
    verdict = "PASS" if delta >= 1.0 else "FAIL"

    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-05 Verdict\n\n")
        f.write(f"- Arm A (free-form) mean missing aspects: {mA:.2f}\n")
        f.write(f"- Arm B (coverage checklist) mean missing aspects: {mB:.2f}\n")
        f.write(f"- delta (A − B): {delta:+.2f} (bar ≥ 1.0) -> **{verdict}**\n")
        f.write("\n| id | domain | A missing | B missing |\n|---|---|---|---|\n")
        for tid, domain, a, b in details:
            f.write(f"| {tid} | {domain} | {a} | {b} |\n")
        f.write("\n*LLM-rater proxy (blind); human gate is a separate manual step.*\n")

    print("=" * 60)
    print("POC-05 VERDICT")
    print("=" * 60)
    print(f"A mean missing: {mA:.2f} | B mean missing: {mB:.2f} | delta {delta:+.2f} -> {verdict}")
    print(f"elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
