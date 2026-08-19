"""POC-01 failure decomposition — QA fact-check.

Pre-registered verdict: FAIL (exact-match, delta +2.0pp < +20pp). This diagnostic
quantifies WHY: format failures vs execution failures vs genuine wrong answers,
and a lenient (extraction) re-score of the alone arm, so the verdict is clean.
"""

import json
import os
import re

from problems import PROBLEMS, score
from protocol import ProtocolError, execute, parse_plan

CACHE = "data/cache"


def load(name):
    p = os.path.join(CACHE, name + ".json")
    if os.path.exists(p):
        return json.load(open(p, encoding="utf-8"))
    return {}


def lenient_score(answer_text, key):
    """Extract the expected value from prose: numbers, 3-bit patterns, blade names, hexagram names."""
    a = answer_text.strip()
    if score(a, key):
        return True
    if isinstance(key, int):
        nums = re.findall(r"-?\d+", a)
        return any(int(n) == key for n in nums)
    if isinstance(key, str) and re.fullmatch(r"[01]{3}", key):
        pats = re.findall(r"[01]{3}", a)
        return any(p == key for p in pats)
    k = key.lower()
    return k in a.lower()


def main():
    alone = load("alone")
    plans = load("plans")
    interp = load("interp")

    stage = {"plan_parse_fail": 0, "execute_fail": 0, "interp_wrong": 0, "scaffold_ok": 0}
    alone_strict = 0
    alone_lenient = 0
    examples = []
    for pid, cat, text, key in PROBLEMS:
        s = str(pid)
        a = alone.get(s, "")
        alone_strict += 1 if score(a, key) else 0
        alone_lenient += 1 if lenient_score(a, key) else 0

        plan_text = plans.get(s, "")
        try:
            ops = parse_plan(plan_text)
            try:
                result = execute(ops)
                i = interp.get(s, "")
                if score(i, key):
                    stage["scaffold_ok"] += 1
                else:
                    stage["interp_wrong"] += 1
                    if len(examples) < 6:
                        examples.append((pid, "interp_wrong", plan_text[:80], i[:60]))
            except (ValueError, KeyError, IndexError) as e:
                stage["execute_fail"] += 1
                if len(examples) < 6:
                    examples.append((pid, "execute_fail", plan_text[:80], str(e)[:40]))
        except ProtocolError as e:
            stage["plan_parse_fail"] += 1
            if len(examples) < 6:
                examples.append((pid, "plan_parse_fail", plan_text[:80], str(e)[:40]))

    n = len(PROBLEMS)
    print("POC-01 FAILURE DECOMPOSITION")
    print("=" * 60)
    print(f"alone strict : {alone_strict}/{n} ({100*alone_strict/n:.0f}%)")
    print(f"alone lenient: {alone_lenient}/{n} ({100*alone_lenient/n:.0f}%)  <- answer embedded in prose")
    print(f"scaffold     : {stage['scaffold_ok']}/{n} ({100*stage['scaffold_ok']/n:.0f}%)")
    print(f"  plan parse fail: {stage['plan_parse_fail']}  (LLM did not emit JSON op objects)")
    print(f"  execute fail   : {stage['execute_fail']}")
    print(f"  interp wrong   : {stage['interp_wrong']}")
    print("examples:")
    for pid, kind, plan, note in examples:
        print(f"  P{pid} [{kind}] plan={plan!r} note={note!r}")


if __name__ == "__main__":
    main()
