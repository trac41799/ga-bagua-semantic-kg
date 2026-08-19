"""POC-13 runner: rubric-evaluation vs plain review detection of deficient outputs.

Objective metrics: detection = fraction of bad outputs flagged deficient;
specificity = fraction of good outputs NOT flagged. Deficiencies:
- rubric: 8-role coverage < 4 roles addressed (parse-based)
- plain: quality score <= 2 (1-5)
"""

import json
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..",
                                "products", "iching-tools", "coverage"))

from outputs import OUTPUTS, ROLES, freeze_hash, verify_bad_deficient  # noqa: E402
from llm_client import LLMClient, SimulatedLLM, load_api_config  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")

RUBRIC_SYS = (
    "For each of the 8 aspects below, output a JSON object {'0': 0|1, ...} "
    "where 1 = the output addresses the aspect, 0 = missing. "
    "Aspects: 0 receptive (who must accept), 1 causal (what triggers), "
    "2 transmissive (what flows), 3 constraining (limits/budgets), "
    "4 clarifying (what is measured), 5 influential (what changes), "
    "6 balancing (feedback loops), 7 generative (new capabilities). "
    "Output ONLY the JSON object."
)

PLAIN_SYS = "Rate the quality of this plan on a 1-5 scale (5 = complete). Output ONLY the integer."


def load_cache():
    p = os.path.join(CACHE, "responses.json")
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


def save_cache(data):
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(os.path.join(CACHE, "responses.json"), "w", encoding="utf-8"), indent=1)


def parse_bits(text):
    t = text.strip().strip("`").strip()
    if t.startswith("json"):
        t = t[4:].strip()
    data = json.loads(t)
    return [int(data[str(i)]) for i in range(8)]


def main(real=False):
    t0 = time.time()
    assert freeze_hash() == open(os.path.join(HERE, "outputs.sha256")).read().strip()
    ok, oid, n = verify_bad_deficient()
    assert ok, f"bad output {oid} covers {n} roles (construction check failed)"

    cfg = load_api_config()
    llm = LLMClient(*cfg) if (real and cfg) else SimulatedLLM()
    cache = load_cache()

    rubric_det, plain_det = [], []
    rubric_fp, plain_fp = [], []
    for oid, label, text in OUTPUTS:
        rkey = f"rubric_{oid}"
        if rkey not in cache:
            ans, _ = llm.chat([{"role": "system", "content": RUBRIC_SYS},
                               {"role": "user", "content": text}], max_tokens=128)
            try:
                cache[rkey] = parse_bits(ans)
            except Exception:
                cache[rkey] = [0] * 8
        bits = cache[rkey]
        deficient = sum(bits) < 4
        (rubric_det if label == "bad" else rubric_fp).append(1 if deficient else 0)

        pkey = f"plain_{oid}"
        if pkey not in cache:
            ans, _ = llm.chat([{"role": "system", "content": PLAIN_SYS},
                               {"role": "user", "content": text}], max_tokens=4)
            try:
                cache[pkey] = int(ans.strip())
            except ValueError:
                cache[pkey] = 3
        score = cache[pkey]
        (plain_det if label == "bad" else plain_fp).append(1 if score <= 2 else 0)
    if real:
        save_cache(cache)

    rd = sum(rubric_det) / len(rubric_det)
    pd = sum(plain_det) / len(plain_det)
    rfp = sum(rubric_fp) / len(rubric_fp)
    pfp = sum(plain_fp) / len(plain_fp)
    verdict = "PASS" if (rd >= 8 / 12 and rd >= pd + 3 / 12 and rfp <= 2 / 12) else "FAIL"

    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "verdict.md"), "w", encoding="utf-8") as f:
        f.write("# POC-13 Verdict — Coverage-Rubric as Output-Quality Evaluation\n\n")
        f.write(f"- rubric detection (bad flagged): {rd:.2f} ({sum(rubric_det)}/12) | "
                f"plain: {pd:.2f} ({sum(plain_det)}/12) | bar rubric >= 8/12 and >= plain + 3/12\n")
        f.write(f"- rubric false alarms (good flagged): {rfp:.2f} ({sum(rubric_fp)}/12) | "
                f"plain: {pfp:.2f} | bar <= 2/12\n")
        f.write(f"- **VERDICT: {verdict}**\n")
        f.write("\n*Objective parse-based metrics; reviewer blind to good/bad; cached.*\n")

    print("=" * 60)
    print("POC-13 VERDICT")
    print("=" * 60)
    print(f"rubric detection: {rd:.2f} ({sum(rubric_det)}/12) | plain: {pd:.2f} ({sum(plain_det)}/12)")
    print(f"rubric false alarms: {rfp:.2f} ({sum(rubric_fp)}/12) | plain: {pfp:.2f}")
    print(f"VERDICT: {verdict} | elapsed {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--real", action="store_true")
    args = ap.parse_args()
    raise SystemExit(main(real=args.real))
