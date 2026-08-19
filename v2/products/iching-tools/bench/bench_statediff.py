"""B3 repaired state_diff measurement benchmark.

Runs production `iching_statediff.summarize` against the 20 frozen POC-06 transitions
with same-record value-pair coverage. Repaired reports and caches use new paths;
the historical B3 artifacts are never overwritten. New evidence remains pending
until a fresh run and second-model replication.

Run from the product root:  python bench/bench_statediff.py
"""

import json
import os
import re
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.dirname(HERE)
REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(HERE))))
OUT = os.path.join(TOOLS, "output")


def _model_tag():
    model = os.environ.get("ICHING_MODEL", "")
    return re.sub(r"[^A-Za-z0-9_.-]+", "_", model) if model and model != "deepseek-chat" else ""


CACHE = os.path.join(HERE, f".cache_statediff_repaired{('_' + _model_tag()) if _model_tag() else ''}.json")
LEDGER = os.path.join(OUT, "claims_ledger.csv")
REPORT = os.path.join(OUT, "benchmark_statediff_repaired.md")

sys.path.insert(0, TOOLS)
sys.path.insert(0, os.path.join(TOOLS, "statediff"))

from bench import common  # noqa: E402
from iching_statediff import ProtocolError, summarize, validate_aspects  # noqa: E402
from iching_statediff.llm_client import LLMClient, resolve_config  # noqa: E402

COVERAGE_BAR = 0.95
MODEL_STATUS = "single_model"


def artifact_metadata(source, model_name="deepseek-chat"):
    """Return provenance metadata for the repaired measurement artifact."""
    return {
        "method": common.STATE_DIFF_MEASUREMENT_METHOD,
        "protocol": common.STATE_DIFF_MEASUREMENT_PROTOCOL,
        "model": model_name,
        "source_hash": common.hash_of(source),
        "protocol_hash": common.hash_of(common.STATE_DIFF_MEASUREMENT_PROTOCOL),
        "model_status": MODEL_STATUS,
    }


def write_repaired_report(path, rows, metadata):
    """Write the repaired report without touching the historical B3 artifact."""
    n = len(rows)
    compliant = sum(1 for row in rows if row["ok"])
    mean_coverage = sum(row["coverage"] for row in rows) / n if n else 0.0
    defect_list = [row for row in rows if not row["ok"]]
    c_pass = n > 0 and compliant == n
    k_pass = mean_coverage >= COVERAGE_BAR
    fresh_real_run = any(str(row.get("source", "")).startswith("live") for row in rows)
    evidence_status = (
        "PENDING: second-model replication required"
        if fresh_real_run else
        "PENDING: fresh real-model rerun and second-model replication required"
    )

    directory = os.path.dirname(path)
    if directory:
        os.makedirs(directory, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write("# B3 — state_diff repaired measurement benchmark\n\n")
        f.write(f"- method: `{metadata['method']}`\n")
        f.write(f"- protocol: `{metadata['protocol']}`\n")
        f.write(f"- source hash: `{metadata['source_hash']}`\n")
        f.write(f"- protocol hash: `{metadata['protocol_hash']}`\n")
        f.write(f"- model: `{metadata['model']}`\n")
        f.write(f"- model status: `{metadata['model_status']}`\n")
        f.write(f"- evidence status: **{evidence_status}**\n\n")
        f.write("## Per-transition compliance + same-record matches\n\n")
        f.write("| tid | domain | compliance | matched | coverage | defects |\n"
                "|---|---|---|---|---|---|\n")
        for row in rows:
            matched = json.dumps(row.get("matched", []), ensure_ascii=True)
            defects = "; ".join(row["defects"]) if row["defects"] else "-"
            f.write(f"| {row['tid']} | {row['domain']} | "
                    f"{'PASS' if row['ok'] else 'FAIL'} | {matched} | "
                    f"{row['coverage']:.4f} | {defects} |\n")
        f.write(f"\n**Mean coverage: {mean_coverage:.4f}** | compliant "
                f"{compliant}/{n}\n\n")
        f.write("## Verdicts\n\n")
        f.write("| metric | value | bar | verdict |\n|---|---|---|---|\n")
        f.write(f"| statediff repaired compliance | {compliant}/{n} | 20/20 (zero defects) | "
                f"{'PASS' if c_pass else 'FAIL'} |\n")
        f.write(f"| statediff repaired coverage | {mean_coverage:.4f} | >= {COVERAGE_BAR} | "
                f"{'PASS' if k_pass else 'FAIL'} |\n")
        f.write(f"| statediff repaired defects | {len(defect_list)} | 0 | "
                f"{'PASS' if not defect_list else 'FAIL'} |\n\n")
        f.write("## Defects\n\n")
        if defect_list:
            for row in defect_list:
                f.write(f"- tid {row['tid']} ({row['domain']}): "
                        f"{', '.join(row['defects'])}\n")
        else:
            f.write("None.\n")


def load_dotenv_local():
    """Load repo-root .env.local into os.environ when the key is missing (client resolver reads env)."""
    if os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENROUTER_API_KEY"):
        return
    path = os.path.join(REPO, ".env.local")
    if not os.path.exists(path):
        return
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            k, _, v = line.partition("=")
            k, v = k.strip(), v.strip()
            if k and k not in os.environ:
                os.environ[k] = v


class _CaptureLLM:
    """Delegates to the real client and keeps the raw response for the cache."""

    def __init__(self, client):
        self._client = client
        self.raw = None

    def chat(self, messages, max_tokens=128, temperature=0.0):
        text, usage = self._client.chat(messages, max_tokens=max_tokens, temperature=temperature)
        self.raw = text
        return text, usage


class _ReplayLLM:
    """Feeds a cached raw response back through the production summarize path."""

    def __init__(self, raw):
        self.raw = raw

    def chat(self, messages, max_tokens=128, temperature=0.0):
        return self.raw, {}


def load_cache():
    if os.path.exists(CACHE):
        with open(CACHE, encoding="utf-8") as f:
            return json.load(f)
    return {}


def save_cache(cache, metadata=None):
    if metadata is not None:
        cache["_metadata"] = metadata
    os.makedirs(HERE, exist_ok=True)
    tmp = CACHE + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump(cache, f, ensure_ascii=False, indent=2)
    os.replace(tmp, CACHE)


def check_defects(summary):
    """Zero-defect checks: exactly 3 aspects, each a dict with aspect/before/after keys."""
    issues = []
    if not isinstance(summary, dict) or not isinstance(summary.get("aspects"), list):
        return ["summary is not a dict with an 'aspects' list"]
    aspects = summary["aspects"]
    if len(aspects) != 3:
        issues.append(f"expected exactly 3 aspects, got {len(aspects)}")
    for i, a in enumerate(aspects):
        if not isinstance(a, dict) or not all(k in a for k in ("aspect", "before", "after")):
            issues.append(f"aspect {i} missing aspect/before/after keys: {a!r}")
    if not issues:
        try:
            validate_aspects(aspects)
        except ProtocolError as exc:
            issues.append(str(exc))
    return issues


def run_one(tid, domain, before, after, planted, llm, cache):
    cached = cache.get(str(tid))
    aspects, defects, source = None, [], "live"
    if cached is not None:
        if cached.get("raw") is not None:
            try:
                summary = summarize(before, after, _ReplayLLM(cached["raw"]))
                aspects = summary["aspects"]
                defects = check_defects(summary)
            except Exception as e:  # cached replay through the production parser
                defects = [f"{type(e).__name__}: {e}"]
        else:
            defects = [cached.get("error", "cached failure")]
        source = "cache"
        return aspects, defects, source
    cap = _CaptureLLM(llm)
    start = time.time()
    try:
        summary = summarize(before, after, cap)
        aspects = summary["aspects"]
        defects = check_defects(summary)
        cache[str(tid)] = {"raw": cap.raw, "error": None}
    except Exception as e:
        defects = [f"{type(e).__name__}: {e}"]
        cache[str(tid)] = {"raw": None, "error": defects[0]}
    save_cache(cache)
    source = f"live ({time.time() - start:.1f}s)"
    return aspects, defects, source


def main():
    load_dotenv_local()
    cfg = resolve_config()
    if cfg is None:
        print("error: no API key (set DEEPSEEK_API_KEY or OPENROUTER_API_KEY, or repo .env.local)", file=sys.stderr)
        return 1
    base_url, api_key = cfg
    llm = LLMClient(base_url, api_key, model=os.environ.get("ICHING_MODEL", "deepseek-chat"))

    transitions = common.poc06_transitions()
    metadata = artifact_metadata(transitions, model_name=llm.model)
    cache = load_cache()

    rows = []
    for tid, domain, before, after, planted in transitions:
        aspects, defects, source = run_one(tid, domain, before, after, planted, llm, cache)
        measurement = (
            common.measure_planted_delta_coverage(aspects, planted)
            if aspects is not None else {
                "method": common.STATE_DIFF_MEASUREMENT_METHOD,
                "matched": [False] * len(planted),
                "covered": 0,
                "total": len(planted),
                "coverage": 0.0,
            }
        )
        rows.append({"tid": tid, "domain": domain, "ok": not defects,
                     "coverage": measurement["coverage"],
                     "matched": measurement["matched"],
                     "measurement": measurement, "defects": defects,
                     "source": source})

    n = len(rows)
    compliant = sum(1 for r in rows if r["ok"])
    mean_coverage = sum(r["coverage"] for r in rows) / n if n else 0.0
    defect_list = [r for r in rows if not r["ok"]]

    c_pass = n > 0 and compliant == n
    k_pass = mean_coverage >= COVERAGE_BAR
    verdicts = [
        ("statediff repaired compliance", f"{compliant}/{n}",
         "20/20 (zero defects)", "PASS" if c_pass else "FAIL"),
        ("statediff repaired coverage", f"{mean_coverage:.4f}",
         f">= {COVERAGE_BAR}", "PASS" if k_pass else "FAIL"),
        ("statediff repaired defects", str(len(defect_list)),
         "0", "PASS" if not defect_list else "FAIL"),
    ]

    write_repaired_report(REPORT, rows, metadata)
    save_cache(cache, metadata)

    exists = os.path.exists(LEDGER)
    if not exists:
        with open(LEDGER, "w", newline="", encoding="utf-8") as f:
            f.write("metric,value,bar,date,note\n")
    new_rows = [
        ("statediff_repaired_compliance", f"{compliant}/{n}", "20/20",
         f"method={metadata['method']}; protocol={metadata['protocol']}; "
         f"source_hash={metadata['source_hash']}; protocol_hash={metadata['protocol_hash']}; "
         f"model_status={metadata['model_status']}"),
        ("statediff_repaired_coverage", f"{mean_coverage:.4f}", f">={COVERAGE_BAR}",
         f"method={metadata['method']}; protocol={metadata['protocol']}; "
         f"source_hash={metadata['source_hash']}; protocol_hash={metadata['protocol_hash']}; "
         f"model_status={metadata['model_status']}"),
        ("statediff_repaired_defects", str(len(defect_list)), "0",
         f"method={metadata['method']}; protocol={metadata['protocol']}; "
         f"source_hash={metadata['source_hash']}; protocol_hash={metadata['protocol_hash']}; "
         f"model_status={metadata['model_status']}"),
    ]
    for row in new_rows:
        common.ledger_row(LEDGER, *row)

    print(f"compliance: {compliant}/{n} {'PASS' if c_pass else 'FAIL'}")
    print(f"mean coverage: {mean_coverage:.4f} {'PASS' if k_pass else 'FAIL'}")
    print(f"verdicts: {verdicts}")
    print(f"defect count: {len(defect_list)}")
    print(f"method: {metadata['method']} | protocol: {metadata['protocol']}")
    print(f"model status: {metadata['model_status']} | evidence remains pending")
    for r in defect_list:
        print(f"  defect tid {r['tid']}: {r['defects']}")
    print(f"files: {REPORT}, {LEDGER}, {CACHE}")
    return 0 if all(v == "PASS" for _, _, _, v in verdicts) else 1


if __name__ == "__main__":
    sys.exit(main())
