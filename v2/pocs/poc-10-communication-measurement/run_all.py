"""POC-10 runner: calibration -> objective QA -> calibrated rater -> verdicts.

Consumes cached POC-06/09 responses (read-only) and regenerates POC-07 explanations
(deterministic). New runs use the versioned no-ground-truth question protocol and
write versioned output/cache files so historical evidence remains immutable.
"""

from decimal import Decimal
from fractions import Fraction
import json
import os
import sys
import time

import numpy as np

from llm_client import LLMClient, SimulatedLLM, load_api_config
from measure import (CALIBRATION, NO_GROUND_TRUTH_QUESTION_PROTOCOL,
                     conveyance_score, freeze_hash, no_ground_truth_questions,
                     qa_match_count)

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")
CLEAN_ARTIFACT_VERSION = "clean-v1"
CLEAN_VERDICT_PATH = os.path.join(OUT, f"verdict-{CLEAN_ARTIFACT_VERSION}.md")
CLEAN_CACHE_PATH = os.path.join(CACHE, f"responses-{CLEAN_ARTIFACT_VERSION}.json")
P06 = os.path.join(HERE, "..", "poc-06-state-diff", "data", "cache", "responses.json")
P09 = os.path.join(HERE, "..", "poc-09-situation-labeling", "data", "cache", "responses.json")
QA06_THRESHOLD = Decimal("0.10")
QA06_THRESHOLD_FRACTION = Fraction(QA06_THRESHOLD)

RATE_SYS = ("Rate on a 1-5 scale. 1 = very poor, 5 = excellent. Output ONLY the integer.")


def load_json(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


_MODE = "real"


def load_cache():
    p = CLEAN_CACHE_PATH
    if not os.path.exists(p):
        return {}
    data = json.load(open(p, encoding="utf-8"))
    if data.get("_mode") != _MODE:
        return {}  # refuse to reuse a cache from another mode (L4)
    return data


def save_cache(data):
    data["_mode"] = _MODE
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(CLEAN_CACHE_PATH, "w", encoding="utf-8"), indent=1)


def decimal_from_fraction(value):
    """Convert an exact fraction to Decimal without going through a float."""
    return Decimal(value.numerator) / Decimal(value.denominator)


def exact_qa_metrics(correct_by_arm, total_by_arm):
    """Return exact arm scores, delta, and verdict for the QA threshold."""
    scores = {arm: Fraction(correct_by_arm[arm], total_by_arm[arm])
              for arm in ("A", "B")}
    delta = scores["B"] - scores["A"]
    verdict = (delta >= QA06_THRESHOLD_FRACTION
               and decimal_from_fraction(delta) >= QA06_THRESHOLD)
    return scores, delta, verdict


def rate(llm, cache, key, sys_prompt, text):
    if key not in cache:
        ans, _ = llm.chat([{"role": "system", "content": sys_prompt},
                           {"role": "user", "content": text}], max_tokens=4)
        try:
            cache[key] = int(ans.strip())
        except ValueError:
            cache[key] = 1
    return cache[key]


def load_module(name, path):
    import importlib.util
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main(real=False):
    t0 = time.time()
    if real:
        cfg = load_api_config()
        if cfg is None:
            raise SystemExit("FATAL: --real requested but no API config found; refusing to run SimulatedLLM")
        llm = LLMClient(*cfg)
        print(f"[poc-10] mode=real model={cfg[2]} client={type(llm).__name__}")
    else:
        llm = SimulatedLLM()
        print("[poc-10] mode=sim (tests only)")
    cache = load_cache()
    assert freeze_hash() == open(os.path.join(HERE, "calibration.sha256")).read().strip()

    transitions_mod = load_module("p06_transitions", os.path.join(HERE, "..", "poc-06-state-diff", "transitions.py"))
    scenarios_mod = load_module("p09_scenarios", os.path.join(HERE, "..", "poc-09-situation-labeling", "scenarios.py"))
    p07_dir = os.path.join(HERE, "..", "poc-07-interaction-xai")
    sys.path.insert(0, p07_dir)
    try:
        p07_mod = load_module("p07_run_all", os.path.join(p07_dir, "run_all.py"))
    finally:
        sys.path.pop(0)
    TRANSITIONS = transitions_mod.TRANSITIONS
    SCENARIOS = scenarios_mod.SCENARIOS
    FACTOR_NAMES = p07_mod.FACTOR_NAMES
    PLANTED = p07_mod.PLANTED
    explain = p07_mod.explain

    # ---- 1. calibration: rater must discriminate known-good vs known-bad ----
    cal = {}
    cal_verdict = {}
    for task, pairs in CALIBRATION.items():
        gs, bs = [], []
        for i, (good, bad) in enumerate(pairs):
            gs.append(rate(llm, cache, f"cal_{task}_{i}_g", RATE_SYS, good))
            bs.append(rate(llm, cache, f"cal_{task}_{i}_b", RATE_SYS, bad))
        d = float(np.mean(gs) - np.mean(bs))
        cal[task] = (float(np.mean(gs)), float(np.mean(bs)), d)
        cal_verdict[task] = d >= 1.0
    instrument_ok = all(cal_verdict.values())

    # ---- 2. objective comprehension QA (POC-06 cached summaries) ----
    p06 = load_json(P06)
    sys.path.insert(0, os.path.join(HERE, "..", "poc-06-state-diff"))
    from transitions import TRANSITIONS
    qa_correct = {"A": 0, "B": 0}
    qa_total_by_arm = {"A": 0, "B": 0}
    for tid, domain, _before, _after, planted in TRANSITIONS:
        aspect_names = [aspect for aspect, _, _ in planted]
        for arm in ("A", "B"):
            summary = p06.get(f"{tid}_{arm}", "")
            for qi, q in enumerate(no_ground_truth_questions(summary, aspect_names)):
                key = f"qa06_{tid}_{arm}_{qi}"
                if key not in cache:
                    ans, _ = llm.chat([{"role": "system", "content": "Answer from the summary ONLY."},
                                       {"role": "user", "content": f"Summary:\n{summary}\n\nQuestion: {q}"}],
                                       max_tokens=48)
                    cache[key] = ans
                qa_correct[arm] += qa_match_count(cache[key], [planted[qi]])
                qa_total_by_arm[arm] += 1
    qa_total = sum(qa_total_by_arm.values())
    qa06, qa06_delta, qa06_verdict = exact_qa_metrics(qa_correct, qa_total_by_arm)

    # ---- 3. answer conveyance (POC-09 cached explanations) ----
    p09 = load_json(P09)
    sys.path.insert(0, os.path.join(HERE, "..", "poc-09-situation-labeling"))
    from scenarios import SCENARIOS
    conv = {"A": [], "B": []}
    for sid, domain, situation, answer in SCENARIOS:
        for arm in ("A", "B"):
            expl = p09.get(f"{sid}_{arm}", "")
            key = f"conv09_{sid}_{arm}"
            if key not in cache:
                ans, _ = llm.chat([{"role": "system", "content": "What action does this explanation recommend? Answer in one sentence."},
                                   {"role": "user", "content": expl}], max_tokens=48)
                cache[key] = ans
            conv[arm].append(conveyance_score(cache[key], answer))
    conv09 = {k: float(np.mean(v)) for k, v in conv.items()}
    conv09_delta = conv09["B"] - conv09["A"]
    conv09_ok = conv09_delta >= -0.05  # non-inferiority

    # ---- 4. calibrated rater on real pairs (06, 09, 07) ----
    rater_scores = {}
    for tid, domain, before, after, planted in TRANSITIONS:
        for arm in ("A", "B"):
            s = p06.get(f"{tid}_{arm}", "")
            key = f"rate06_{tid}_{arm}"
            rater_scores.setdefault("06", {"A": [], "B": []})
            rater_scores["06"][arm].append(rate(llm, cache, key, RATE_SYS,
                                                 f"Summary of a state change:\n{s}"))
    for sid, domain, situation, answer in SCENARIOS:
        for arm in ("A", "B"):
            e = p09.get(f"{sid}_{arm}", "")
            key = f"rate09_{sid}_{arm}"
            rater_scores.setdefault("09", {"A": [], "B": []})
            rater_scores["09"][arm].append(rate(llm, cache, key, RATE_SYS,
                                                 f"Explanation of a situation:\n{e}"))
    # 07 naming: deterministic explanations (loaded above as p07_mod)
    for i, S in enumerate(sorted(PLANTED, key=sorted)):
        for arm, bagua in (("A", False), ("B", True)):
            e = explain(S, FACTOR_NAMES, bagua=bagua)
            key = f"rate07_{i}_{arm}"
            rater_scores.setdefault("07", {"A": [], "B": []})
            rater_scores["07"][arm].append(rate(llm, cache, key, RATE_SYS, e))

    deltas = {}
    verdicts = {}
    for task in ("06", "09", "07"):
        da = float(np.mean(rater_scores[task]["A"]))
        db = float(np.mean(rater_scores[task]["B"]))
        deltas[task] = db - da
        verdicts[task] = deltas[task] >= 0.5 and instrument_ok

    if real:
        save_cache(cache)

    os.makedirs(OUT, exist_ok=True)
    with open(CLEAN_VERDICT_PATH, "w", encoding="utf-8") as f:
        f.write("# POC-10 Verdict — Communication Boundary-Test (clean-v1)\n\n")
        f.write(f"Question protocol: `{NO_GROUND_TRUTH_QUESTION_PROTOCOL}`\n\n")
        f.write("## 1. Instrument calibration\n\n| task | good | bad | delta |\n|---|---|---|---|\n")
        for task, (g, b, d) in cal.items():
            f.write(f"| {task} | {g:.2f} | {b:.2f} | {d:+.2f} |\n")
        f.write(f"\n**Instrument: {'PASS (rater discriminates)' if instrument_ok else 'FAIL (rater unfit)'}**\n\n")
        f.write("## 2. Objective comprehension-QA (POC-06, evaluator-held ground truth)\n\n")
        f.write(f"- Questions scored: {qa_total} | Arm A correct: {qa_correct['A']} | "
                f"Arm B correct: {qa_correct['B']}\n")
        f.write(f"- Arm A: {float(qa06['A']):.3f} | Arm B: {float(qa06['B']):.3f} | "
                f"delta {float(qa06_delta):+.3f} (bar ≥ +{QA06_THRESHOLD:.2f}) "
                f"-> **{'PASS' if qa06_verdict else 'FAIL'}**\n")
        f.write("## 3. Answer conveyance (POC-09, objective)\n\n")
        f.write(f"- Arm A: {conv09['A']:.3f} | Arm B: {conv09['B']:.3f} | delta {conv09_delta:+.3f} "
                f"(non-inferiority ≥ −0.05) -> **{'PASS' if conv09_ok else 'FAIL'}**\n")
        f.write("## 4. Calibrated rater on real pairs\n\n| task | A | B | delta | bar | verdict |\n|---|---|---|---|---|---|\n")
        for task in ("06", "09", "07"):
            f.write(f"| {task} | {float(np.mean(rater_scores[task]['A'])):.2f} | "
                    f"{float(np.mean(rater_scores[task]['B'])):.2f} | {deltas[task]:+.2f} | +0.5 | "
                    f"{'PASS' if verdicts[task] else 'FAIL' if instrument_ok else 'N/A (instrument)'} |\n")
        f.write("\n## Boundary resolution\n\n")
        if not instrument_ok:
            f.write("- **Instrument failure**: the LLM rater cannot discriminate known-good from "
                    "known-bad outputs. All prior 1–5 rater verdicts (POC-06/09/07-naming) are "
                    "UNINTERPRETABLE. The objective QA is the only valid measurement here.\n")
        elif qa06_verdict:
            f.write("- **Phenomenon real (objective)**: structured summaries measurably improve "
                    "comprehension (QA accuracy vs planted ground truth). Prior 'FAIL' was "
                    "instrument saturation, not phenomenon absence.\n")
        else:
            f.write("- **Phenomenon null (objective)**: structured summaries do not improve "
                    "comprehension beyond free-form on this objective measure.\n")

    print("=" * 60)
    print("POC-10 VERDICT")
    print("=" * 60)
    print(f"calibration: {cal}")
    print(f"instrument: {'PASS' if instrument_ok else 'FAIL'}")
    print(f"QA06: {qa_total} questions | A {qa_correct['A']} ({float(qa06['A']):.3f}) | "
          f"B {qa_correct['B']} ({float(qa06['B']):.3f}) | delta {float(qa06_delta):+.3f} -> "
          f"{'PASS' if qa06_verdict else 'FAIL'}")
    print(f"conv09: A {conv09['A']:.3f} | B {conv09['B']:.3f} | delta {conv09_delta:+.3f}")
    print(f"rater deltas: { {k: round(v, 2) for k, v in deltas.items()} }")
    print(f"elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
