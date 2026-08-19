"""POC-15: cross-model replication (POC-05 + POC-08 on a second model via OpenRouter).

Second model: openai/gpt-4o-mini. Same frozen protocols, prompts, and bars.
"""

import json
import os
import time
import urllib.request

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
POCS = os.path.join(os.path.dirname(os.path.dirname(HERE)), "pocs")
OUT = os.path.join(HERE, "output")
CACHE = os.path.join(HERE, "data", "cache")
MODEL = "openai/gpt-4o-mini"


def load_api_key():
    key = os.environ.get("OPENROUTER_API_KEY")
    if key:
        return key
    env = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.abspath(__file__))))), ".env.local")
    if os.path.exists(env):
        for line in open(env, encoding="utf-8"):
            if line.startswith("OPENROUTER_API_KEY="):
                return line.strip().split("=", 1)[1].strip().strip('"').strip("'")
    return None


class OrClient:
    def __init__(self, key, model=MODEL):
        self.key = key
        self.model = model
        self.usage = {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0, "calls": 0}

    def chat(self, messages, max_tokens=128):
        body = json.dumps({"model": self.model, "messages": messages,
                           "max_tokens": max_tokens, "temperature": 0.0}).encode()
        req = urllib.request.Request("https://openrouter.ai/api/v1/chat/completions",
                                     data=body, method="POST",
                                     headers={"Content-Type": "application/json",
                                              "Authorization": "Bearer " + self.key})
        with urllib.request.urlopen(req, timeout=180) as r:
            data = json.loads(r.read())
        u = data.get("usage", {})
        for k in self.usage:
            if k != "calls":
                self.usage[k] += u.get(k, 0)
        self.usage["calls"] += 1
        return data["choices"][0]["message"]["content"]


def load_cache():
    p = os.path.join(CACHE, "responses.json")
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else {}


def save_cache(data):
    os.makedirs(CACHE, exist_ok=True)
    json.dump(data, open(os.path.join(CACHE, "responses.json"), "w", encoding="utf-8"), indent=1)


def cached(llm, cache, key, fn):
    if key not in cache:
        cache[key] = fn()
    return cache[key]


def render(rows, path):
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, path), "w", encoding="utf-8") as f:
        f.write(f"# POC-15 replication — {MODEL}\n\n| metric | value | bar (deepseek-validated) | verdict |\n|---|---|---|---|\n")
        for label, value, bar, verdict in rows:
            f.write(f"| {label} | {value} | {bar} | {verdict} |\n")


def main(which="all"):
    key = load_api_key()
    if not key:
        print("FATAL: no OPENROUTER_API_KEY")
        return 2
    llm = OrClient(key)
    cache = load_cache()
    results = []

    # ---- R-05: coverage checklist ----
    if which in ("all", "coverage"):
        import importlib.util
        spec = importlib.util.spec_from_file_location(
            "p05", os.path.join(POCS, "poc-05-coverage-rubric", "protocol.py"))
        mod = importlib.util.module_from_spec(spec)
        sys.path.insert(0, os.path.join(POCS, "poc-05-coverage-rubric"))
        try:
            spec.loader.exec_module(mod)
        finally:
            sys.path.pop(0)
        missing = {"A": [], "B": []}
        for tid, domain, task in mod.TASKS:
            for arm, fn in (("A", mod.arm_a_prompt), ("B", mod.arm_b_prompt)):
                plan = cached(llm, cache, f"r05_{tid}_{arm}_plan",
                              lambda: llm.chat(fn(task), max_tokens=256))
                bits = cached(llm, cache, f"r05_{tid}_{arm}_audit",
                              lambda: _audit(llm, mod, task, plan))
                missing[arm].append(6 - sum(bits))
        mA = sum(missing["A"]) / len(missing["A"])
        mB = sum(missing["B"]) / len(missing["B"])
        delta = mA - mB
        verdict = "PASS" if delta >= 1.0 else "FAIL"
        results.append(("R-05 coverage delta (A-B missing)", f"{delta:+.2f}", ">= +1.0", verdict))
        results.append(("R-05 audited mean missing", f"{mB:.2f}", "<= 1.25", "PASS" if mB <= 1.25 else "FAIL"))
        render(results, "replication_coverage.md")

    # ---- R-08: reframe ----
    if which in ("all", "reframe"):
        p08 = os.path.join(POCS, "poc-08-reframing-v2")
        sys.path.insert(0, p08)
        try:
            from moves import START_STATE, all_positions, describe_state
            from naming import naming_prompt, judge_prompt
            import statements as st
            import rubric as rb
        finally:
            sys.path.pop(0)
        statements = [s["text"] if isinstance(s, dict) else s for s in st.STATEMENTS]
        divs, cohs = [], []
        for i, statement in enumerate(statements):
            positions = list(all_positions(START_STATE))
            reframes = []
            for move, state in positions:
                base = move.rstrip("0123456789")
                r = cached(llm, cache, f"r08_{i}_{move}",
                           lambda: llm.chat(naming_prompt(statement, base, describe_state(state)),
                                            max_tokens=64))
                reframes.append(str(r).strip())
            vecs = []
            for j, r in enumerate(reframes):
                v = cached(llm, cache, f"r08_{i}_enc_{j}",
                           lambda: _enc_list(llm, rb, r))
                vecs.append(v)
            divs.append(_diversity(vecs))
            score = cached(llm, cache, f"r08_{i}_judge",
                           lambda: _judge(llm, judge_prompt, statement, reframes))
            cohs.append(score)
        mean_div = float(np.mean(divs))
        mean_coh = float(np.mean([c for c in cohs if c > 0]))
        results.append(("R-08 reframe diversity", f"{mean_div:.3f}", ">= 0.403", "PASS" if mean_div >= 0.403 else "FAIL"))
        results.append(("R-08 reframe coherence", f"{mean_coh:.2f}", ">= 3.5", "PASS" if mean_coh >= 3.5 else "FAIL"))
        render(results, "replication_reframe.md")

    save_cache(cache)
    print("=" * 60)
    print("POC-15 REPLICATION")
    print("=" * 60)
    for label, value, bar, verdict in results:
        print(f"  {label}: {value} (bar {bar}) -> {verdict}")
    print(f"  usage: {llm.usage['total_tokens']} tokens, {llm.usage['calls']} calls")
    return 0


def _audit(llm, mod, task, plan):
    ans = llm.chat(mod.audit_prompt(task, plan), max_tokens=128)
    t = ans.strip().strip("`").strip()
    if t.startswith("json"):
        t = t[4:].strip()
    data = json.loads(t)
    return [int(data[str(i)]) for i in range(6)]


def _enc_list(llm, rb, text):
    v = _encode(llm, rb, text)
    return v.tolist() if v is not None else None


def _encode(llm, rb, text):
    ans = llm.chat(rb.encode_prompt(text), max_tokens=128)
    try:
        return rb.parse_encoding(ans)
    except Exception:
        return None


def _judge(llm, judge_prompt, statement, reframes):
    ans = llm.chat(judge_prompt(statement, reframes), max_tokens=4)
    try:
        return int(str(ans).strip())
    except ValueError:
        return 0


def _diversity(vectors):
    vs = [np.asarray(v, dtype=float) for v in vectors if v is not None]
    if len(vs) < 2:
        return 0.0
    d, n = 0.0, 0
    for i in range(len(vs)):
        for j in range(i + 1, len(vs)):
            d += 1.0 - float(vs[i] @ vs[j])
            n += 1
    return d / n if n else 0.0


import sys  # noqa: E402


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--which", default="all", choices=["all", "coverage", "reframe"])
    args = ap.parse_args()
    raise SystemExit(main(which=args.which))
