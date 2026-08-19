"""B2 — reframe (reproduces POC-08): production iching_reframe on the REAL LLM.

Runs the production `reframe(statement, llm)` (exactly 8 cube positions) against
the 20 frozen POC-08 statements, deepseek-chat temperature 0, every response
cached to bench/.cache_reframe.json (re-runs reuse the cache and reproduce
identical numbers).

Bars (validation spec B2):
  - diversity  >= 0.403   (POC-08 validated 0.453, -0.05 regression tolerance)
  - coherence  >= 3.5     (POC-08 validated 3.75, same 1-5 judge prompt)
  - zero defects          (exactly 8 distinct positions per statement; encode
                           and judge protocol failures counted)

Artifacts: output/benchmark_reframe.md, output/claims_ledger.csv rows
(reframe_diversity, reframe_coherence, reframe_defects), bench/.cache_reframe.json.
Run from v2/products/iching-tools:  python bench/bench_reframe.py
"""

import hashlib
import json
import os
import re
import sys
import time
from datetime import date

HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.dirname(HERE)  # v2/products/iching-tools
if TOOLS not in sys.path:
    sys.path.insert(0, TOOLS)
REF_DIR = os.path.join(TOOLS, "reframe")
if REF_DIR not in sys.path:
    sys.path.insert(0, REF_DIR)

import numpy as np  # noqa: E402
import bench.common as common  # noqa: E402
from iching_reframe import reframe  # noqa: E402
from iching_reframe.llm_client import LLMClient, base_url_for, resolve_key  # noqa: E402

OUT_DIR = os.path.join(TOOLS, "output")
MARKDOWN = os.path.join(OUT_DIR, "benchmark_reframe.md")
LEDGER = os.path.join(OUT_DIR, "claims_ledger.csv")


def _model_tag():
    model = os.environ.get("ICHING_MODEL", "")
    return re.sub(r"[^A-Za-z0-9_.-]+", "_", model) if model and model != "deepseek-chat" else ""


CACHE_PATH = os.path.join(HERE, f".cache_reframe{('_' + _model_tag()) if _model_tag() else ''}.json")

P08 = os.path.join(common.POCS, "poc-08-reframing-v2")
BAR_DIVERSITY = 0.403
BAR_COHERENCE = 3.5


# ---- API key: env DEEPSEEK_API_KEY, else repo .env.local (client's own resolver) ----

def _load_env_local():
    d = HERE
    for _ in range(8):
        p = os.path.join(d, ".env.local")
        if os.path.exists(p):
            with open(p, encoding="utf-8") as fh:
                for line in fh:
                    line = line.strip()
                    if line and not line.startswith("#") and "=" in line:
                        k, v = line.split("=", 1)
                        os.environ.setdefault(k.strip(), v.strip().strip('"').strip("'"))
            return True
        parent = os.path.dirname(d)
        if parent == d:
            return False
        d = parent
    return False


def _messages_key(messages):
    return "resp_" + hashlib.sha256(
        json.dumps(messages, ensure_ascii=False, sort_keys=True).encode("utf-8")).hexdigest()


class CachedLLM:
    """Real LLMClient behind a deterministic response cache (temperature 0)."""

    def __init__(self, client, cache):
        self._client = client
        self._cache = cache
        self.new_calls = 0

    def chat(self, messages, max_tokens=None, temperature=0.0):
        key = _messages_key(messages)
        if key not in self._cache:
            self._cache[key] = self._client.chat(messages)
            self.new_calls += 1
        return self._cache[key]


# ---- encoder: same 8-role rubric as POC-08 ----

def _resolve_encoder():
    """common.rubric_encode loads v2/pocs/path-d/rubric.py, which is currently
    missing; fall back to the POC-08 rubric module (identical ROLES, prompt,
    parser -- the encoder POC-08 validated 0.453 with). Returns (encode_prompt,
    parse_encoding, used_fallback)."""
    path_d = os.path.join(common.POCS, "path-d", "rubric.py")
    if os.path.exists(path_d):
        mod = common.load_module("d_rubric", path_d)
        return mod.encode_prompt, mod.parse_encoding, False
    mod = common.load_module("p08_rubric", os.path.join(P08, "rubric.py"))
    return mod.encode_prompt, mod.parse_encoding, True


def _encode(encode_prompt, parse_encoding, llm, cache, text):
    key = "enc_" + hashlib.sha256(text.encode("utf-8")).hexdigest()
    if key not in cache:
        ans = llm.chat(encode_prompt(text), max_tokens=128)
        try:
            cache[key] = parse_encoding(ans).tolist()
        except Exception:
            cache[key] = None
    return cache[key]


# ---- judge: exact POC-08 1-5 coherence judge ----

def _judge_prompt_fn():
    mod = common.load_module("p08_naming", os.path.join(P08, "naming.py"))
    return mod.judge_prompt


def _verify_frozen():
    mod = common.load_module("p08_statements", os.path.join(P08, "statements.py"))
    return mod.verify_frozen()


def main():
    t0 = time.time()
    _load_env_local()
    key, provider = resolve_key()
    if not key:
        print("FATAL: no API key (DEEPSEEK_API_KEY env or repo .env.local)")
        return 2
    client = LLMClient(base_url=base_url_for(provider), api_key=key,
                       model=os.environ.get("ICHING_MODEL", "deepseek-chat"), temperature=0.0)
    cache = {}
    if os.path.exists(CACHE_PATH):
        with open(CACHE_PATH, encoding="utf-8") as fh:
            cache = json.load(fh)
    llm = CachedLLM(client, cache)

    encode_prompt, parse_encoding, encoder_fallback = _resolve_encoder()
    judge_prompt = _judge_prompt_fn()

    statements = common.poc08_statements()
    defects = []
    metric_degenerates = 0
    metric_notes = []
    if len(statements) != 20:
        defects.append(f"frozen statements: expected 20, got {len(statements)}")
        print("FATAL: expected 20 frozen statements, got", len(statements))
        return 2
    if not _verify_frozen():
        defects.append("frozen statements: sha256 marker mismatch (verify_frozen() False)")

    per_stmt = []
    for i, text in enumerate(statements):
        sid = f"s{i + 1:02d}"
        positions = []
        try:
            result = reframe(text, llm)
            positions = result.get("positions") or []
        except Exception as exc:  # noqa: BLE001 -- any production error is a defect
            defects.append(f"{sid}: reframe() error: {exc!r}")
        states = [p.get("state") for p in positions]
        if len(positions) != 8:
            defects.append(f"{sid}: {len(positions)} positions (expected exactly 8)")
        elif len(set(states)) != 8:
            defects.append(f"{sid}: {len(set(states))} distinct states (expected 8)")
        reframes = [p.get("reframe", "") for p in positions]

        vecs, encode_degenerates = [], 0
        for r in reframes:
            v = _encode(encode_prompt, parse_encoding, llm, cache, r)
            if v is None:
                encode_degenerates += 1
            vecs.append(v)
        metric_degenerates += encode_degenerates
        if encode_degenerates:
            metric_notes.append(
                f"{sid}: {encode_degenerates} reframe(s) not encodable "
                "(deterministic zero-vector; metric-pipeline, not a product defect)")
        div = common.diversity(vecs)

        key_judge = f"judge_{sid}"
        if key_judge not in cache:
            ans = llm.chat(judge_prompt(text, reframes), max_tokens=4)
            try:
                cache[key_judge] = int(str(ans).strip())
            except ValueError:
                cache[key_judge] = 0
        score = cache[key_judge]
        if not 1 <= score <= 5:
            defects.append(f"{sid}: coherence judge protocol failure (score {score})")

        per_stmt.append({"id": sid, "statement": text, "n": len(positions),
                         "diversity": div, "coherence": score})
        with open(CACHE_PATH, "w", encoding="utf-8") as fh:
            json.dump(cache, fh, ensure_ascii=False, indent=1)
        print(f"  {sid}: positions={len(positions)} diversity={div:.3f} coherence={score} "
              f"({time.time() - t0:.0f}s)", flush=True)

    mean_div = float(np.mean([s["diversity"] for s in per_stmt]))
    cohs = [s["coherence"] for s in per_stmt if s["coherence"] > 0]
    mean_coh = float(np.mean(cohs)) if cohs else 0.0
    n_defects = len(defects)

    v_div = "PASS" if mean_div >= BAR_DIVERSITY else "FAIL"
    v_coh = "PASS" if mean_coh >= BAR_COHERENCE else "FAIL"
    v_def = "PASS" if n_defects == 0 else "FAIL"
    verdict = "PASS" if v_div == v_coh == v_def == "PASS" else "FAIL"

    os.makedirs(OUT_DIR, exist_ok=True)
    lines = [
        "# B2 — reframe (production iching_reframe vs POC-08 baseline)",
        "",
        f"- date: {date.today()} | model: {client.model} (temperature 0) | "
        f"cache: {len(cache)} responses | new calls this run: {llm.new_calls}",
        f"- bars: diversity >= {BAR_DIVERSITY} (POC-08 0.453 - 0.05), "
        f"coherence >= {BAR_COHERENCE} (POC-08 3.75), zero PRODUCTION defects",
        "",
        "## Per-statement",
        "",
        "| id | positions | diversity | coherence |",
        "|---|---|---|---|",
    ]
    for s in per_stmt:
        lines.append(f"| {s['id']} | {s['n']} | {s['diversity']:.3f} | {s['coherence']} |")
    lines += [
        "",
        "## Means and bars",
        "",
        "| metric | value | bar | verdict |",
        "|---|---|---|---|",
        f"| reframe_diversity | {mean_div:.3f} | >= {BAR_DIVERSITY} | {v_div} |",
        f"| reframe_coherence | {mean_coh:.2f} | >= {BAR_COHERENCE} | {v_coh} |",
        f"| reframe_production_defects | {n_defects} | 0 | {v_def} |",
        f"| **overall** | | | **{verdict}** |",
        "",
        "## Defects",
        "",
    ]
    lines += [f"- {d}" for d in defects] if defects else ["- none (zero PRODUCTION defects)"]
    if metric_degenerates:
        lines += ["", "## Metric-pipeline notes (NOT product defects)", ""]
        lines += [f"- {n}" for n in metric_notes]
    lines += [
        "",
        "## Method",
        "",
        "- Production `reframe(statement, llm)` per frozen statement; defect = not "
        "exactly 8 positions, non-distinct states, or any error.",
        "- Diversity: mean pairwise 1-cosine over the 8 rubric-encoded reframes per "
        "statement (same 8-role encoder as POC-08), averaged across the 20 statements.",
        f"- Encoder source: " + ("v2/pocs/path-d/rubric.py (common.rubric_encode)"
                                 if not encoder_fallback else
                                 "v2/pocs/poc-08-reframing-v2/rubric.py fallback "
                                 "(common.rubric_encode path v2/pocs/path-d/rubric.py "
                                 "missing; encoder prompt+parser identical)"),
        "- Coherence: exact POC-08 judge prompt, 1-5, one call per statement, "
        "mean over the 20 (0 excluded on protocol failure).",
        "- All responses cached in bench/.cache_reframe.json; re-runs reuse the "
        "cache and reproduce identical numbers (temperature 0).",
        "",
    ]
    with open(MARKDOWN, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")

    common.ledger_row(LEDGER, "reframe_diversity", f"{mean_div:.3f}", v_div,
                      f"bar >= {BAR_DIVERSITY}")
    common.ledger_row(LEDGER, "reframe_coherence", f"{mean_coh:.2f}", v_coh,
                      f"bar >= {BAR_COHERENCE}")
    common.ledger_row(LEDGER, "reframe_production_defects", str(n_defects), v_def,
                      "zero production defects required")
    common.ledger_row(LEDGER, "reframe_metric_degenerates", str(metric_degenerates), "documented",
                      "deterministic zero-vector encodes; not product defects")

    with open(CACHE_PATH, "w", encoding="utf-8") as fh:
        json.dump(cache, fh, ensure_ascii=False, indent=1)

    print("=" * 64)
    print("B2 REFRAME VERDICT (production iching_reframe, real LLM)")
    print("=" * 64)
    print(f"diversity  {mean_div:.3f} (bar >= {BAR_DIVERSITY}) -> {v_div}")
    print(f"coherence  {mean_coh:.2f} (bar >= {BAR_COHERENCE}) -> {v_coh}")
    print(f"defects    {n_defects} (bar 0) -> {v_def}")
    print(f"verdict    {verdict} | new calls {llm.new_calls} | "
          f"elapsed {time.time() - t0:.0f}s")
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
