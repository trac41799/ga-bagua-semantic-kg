"""B1 benchmark: production iching_coverage.audit vs frozen POC-05 drafts (real LLM, cached).

Protocol (validation spec B1):
  1. 20 frozen POC-05 tasks; drafts = cached POC-05 Arm-A plans.
  2. Production audit(task, draft, llm) -> audited plan (real LLMClient, temperature 0).
  3. Rate BOTH plans with common.audit_rater (6 aspects, one call each).
  4. missing = 6 - sum(bits); delta = mean(orig_missing) - mean(aud_missing).
  5. Verdict PASS iff delta >= 1.0 (validated 1.15, tolerance +/-0.3). Zero defects required.
  6. output/benchmark_coverage.md (verdict + per-task detail); ledger rows in output/claims_ledger.csv.
  7. Every LLM response cached to bench/.cache_coverage.json keyed by purpose|tid|input-hash;
     re-runs reuse the cache (temperature 0 => deterministic).

Deviation (harness note): common.poc05_draft_plans() key-collides "N_audit_A" over "N_A"
(every key ends with "_A"), so the frozen draft plans are unreachable through it. This script
loads the same frozen cache file with the POC's exact-key semantics (cache["<tid>_A"], as the
POC run_all.py does) and reports that as a harness defect note in the report.
"""

import csv
import hashlib
import json
import os
import re
import sys
import time
from datetime import date

HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.dirname(HERE)
sys.path.insert(0, HERE)
sys.path.insert(0, TOOLS)
sys.path.insert(0, os.path.join(TOOLS, "coverage"))

import bench.common as common  # noqa: E402
from iching_coverage.llm_client import LLMClient, resolve_api_key  # noqa: E402
from iching_coverage import audit  # noqa: E402

def _model_tag():
    model = os.environ.get("ICHING_MODEL", "")
    return re.sub(r"[^A-Za-z0-9_.-]+", "_", model) if model and model != "deepseek-chat" else ""


CACHE_FILE = os.path.join(HERE, f".cache_coverage{('_' + _model_tag()) if _model_tag() else ''}.json")
REPORT = os.path.join(TOOLS, "output", "benchmark_coverage.md")
LEDGER = os.path.join(TOOLS, "output", "claims_ledger.csv")
VALIDATED_DELTA = 1.15
BAR_DELTA = 1.0
N_ASPECTS = 6


def load_env():
    """Load .env.local (repo root) into os.environ if DEEPSEEK_API_KEY is unset."""
    if os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENROUTER_API_KEY"):
        return
    p, cur = None, TOOLS
    for _ in range(4):
        cand = os.path.join(cur, ".env.local")
        if os.path.exists(cand):
            p = cand
            break
        cur = os.path.dirname(cur)
    if p:
        for line in open(p, encoding="utf-8"):
            line = line.strip()
            if line.startswith("DEEPSEEK_API_KEY="):
                os.environ.setdefault("DEEPSEEK_API_KEY",
                                      line.split("=", 1)[1].strip().strip('"').strip("'"))
            elif line.startswith("OPENROUTER_API_KEY="):
                os.environ.setdefault("OPENROUTER_API_KEY",
                                      line.split("=", 1)[1].strip().strip('"').strip("'"))


def frozen_drafts():
    """{tid: draft_plan} from the frozen POC-05 cache using exact '<tid>_A' keys (POC semantics)."""
    pocs_cache = os.path.join(common.POCS, "poc-05-coverage-rubric", "data", "cache", "responses.json")
    data = json.load(open(pocs_cache, encoding="utf-8"))
    out = {}
    for k, v in data.items():
        if re.fullmatch(r"\d+_A", k):
            out[int(k.split("_")[0])] = v
    return out


class CachedLLM(LLMClient):
    """Real production client + disk cache (purpose|tid|input-hash) + chat() tuple shim."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._cache = {}
        if os.path.exists(CACHE_FILE):
            try:
                self._cache = json.load(open(CACHE_FILE, encoding="utf-8"))
            except Exception:
                self._cache = {}
        self.purpose = "unknown"
        self.tid = "?"

    def _save(self):
        tmp = CACHE_FILE + ".tmp"
        json.dump(self._cache, open(tmp, "w", encoding="utf-8"), indent=1)
        os.replace(tmp, CACHE_FILE)

    def complete(self, messages, max_tokens=None):
        payload = {"messages": messages, "max_tokens": max_tokens}
        h = hashlib.sha256(json.dumps(payload, ensure_ascii=False, sort_keys=True).encode()).hexdigest()
        key = f"{self.purpose}|{self.tid}|{h}"
        if key in self._cache:
            return self._cache[key]
        text = super().complete(messages)
        self._cache[key] = text
        self._save()
        return text

    def chat(self, messages, max_tokens=None):
        return self.complete(messages, max_tokens=max_tokens), {}


def main():
    t0 = time.time()
    load_env()

    tasks = common.poc05_tasks()
    drafts = frozen_drafts()
    llm = CachedLLM(api_key=None)  # production constructor; key from env/.env.local

    defects = []
    rows = []

    for tid, domain, task in tasks:
        if tid not in drafts:
            defects.append({"tid": tid, "kind": "missing_plan", "detail": "no frozen Arm-A draft"})
            continue
        draft = drafts[tid]

        try:
            llm.purpose, llm.tid = "audit", tid
            result = audit(task, draft, llm)
            audited = result["audited_plan"]
        except Exception as exc:
            defects.append({"tid": tid, "kind": "llm_error", "detail": f"audit: {exc!r}"})
            continue

        try:
            llm.purpose, llm.tid = "rater_original", tid
            bits_orig = common.audit_rater(llm, task, draft)
        except Exception as exc:
            defects.append({"tid": tid, "kind": "rater_parse", "detail": f"original: {exc!r}"})
            bits_orig = None

        try:
            llm.purpose, llm.tid = "rater_audited", tid
            bits_aud = common.audit_rater(llm, task, audited)
        except Exception as exc:
            defects.append({"tid": tid, "kind": "rater_parse", "detail": f"audited: {exc!r}"})
            bits_aud = None

        if bits_orig is None or bits_aud is None:
            rows.append((tid, domain, None, None, None, None))
            continue

        m_orig = N_ASPECTS - sum(bits_orig)
        m_aud = N_ASPECTS - sum(bits_aud)
        rows.append((tid, domain, m_orig, m_aud, bits_orig, bits_aud))

    valid = [r for r in rows if r[2] is not None]
    n = len(valid)
    orig_mean = sum(r[2] for r in valid) / n if n else float("nan")
    aud_mean = sum(r[3] for r in valid) / n if n else float("nan")
    delta = orig_mean - aud_mean
    verdict = "PASS" if (n == len(tasks) and delta >= BAR_DELTA and not defects) else "FAIL"
    delta_verdict = "PASS" if delta >= BAR_DELTA else "FAIL"

    os.makedirs(os.path.dirname(REPORT), exist_ok=True)
    common.render_verdict(REPORT, [
        ("coverage_delta (original - audited mean missing)", f"{delta:+.2f}",
         f">= {BAR_DELTA} (validated {VALIDATED_DELTA:+}, tol +/-0.30)", delta_verdict),
        ("coverage_original_missing", f"{orig_mean:.2f}", "POC-05 Arm-A baseline", ""),
        ("coverage_audited_missing", f"{aud_mean:.2f}", "<= original - 1.0", ""),
        ("defects", f"{len(defects)}", "0 required", "PASS" if not defects else "FAIL"),
    ])
    with open(REPORT, "a", encoding="utf-8") as f:
        f.write("\n## B1 coverage_audit detail (real LLM, temperature 0, cached)\n\n")
        f.write(f"- tasks: {n}/{len(tasks)} | original mean missing: {orig_mean:.2f} "
                f"| audited mean missing: {aud_mean:.2f} | delta: {delta:+.2f} "
                f"(validated {VALIDATED_DELTA:+}) | verdict: **{verdict}**\n")
        f.write("- client: `LLMClient(api_key=None, model={llm.model!r}, base_url=None)` "
                f"(production constructor; cache shim adds chat() for the rater)\n")
        f.write(f"- cache: `bench/{os.path.basename(CACHE_FILE)}` "
                f"(key = purpose|tid|input-hash)\n\n")
        f.write("| id | domain | original missing | audited missing | original bits | audited bits |\n")
        f.write("|---|---|---|---|---|---|\n")
        for tid, domain, m_orig, m_aud, bits_orig, bits_aud in rows:
            bo = "".join(map(str, bits_orig)) if bits_orig else "-"
            ba = "".join(map(str, bits_aud)) if bits_aud else "-"
            f.write(f"| {tid} | {domain} | {m_orig if m_orig is not None else '-'} | "
                    f"{m_aud if m_aud is not None else '-'} | {bo} | {ba} |\n")
        f.write("\n## Defects\n\n")
        if defects:
            for d in defects:
                f.write(f"- tid {d['tid']} [{d['kind']}]: {d['detail']}\n")
        else:
            f.write("- none (0 defects)\n")
        f.write("\n## Harness note\n\n")
        f.write("- `common.poc05_draft_plans()` key-collides `N_audit_A` over `N_A` "
                "(both end with `_A`); drafts loaded here with the POC's exact-key "
                "`<tid>_A` semantics from the same frozen cache file (see run_all.py).\n")

    with open(LEDGER, "a", newline="", encoding="utf-8") as lf:
        if os.path.getsize(LEDGER) == 0:
            csv.writer(lf).writerow(["metric", "value", "bar", "date", "extra"])
    common.ledger_row(LEDGER, "coverage_delta", f"{delta:.2f}",
                      f">={BAR_DELTA} (validated {VALIDATED_DELTA:+})",
                      f"verdict {delta_verdict}")
    common.ledger_row(LEDGER, "coverage_original_missing", f"{orig_mean:.2f}", "-", "POC-05 Arm-A")
    common.ledger_row(LEDGER, "coverage_audited_missing", f"{aud_mean:.2f}", "-", "after production audit")

    print("=" * 64)
    print(f"B1 coverage: original mean missing {orig_mean:.2f} | "
          f"audited mean missing {aud_mean:.2f} | delta {delta:+.2f} "
          f"(validated {VALIDATED_DELTA:+}) -> {delta_verdict}")
    print(f"defects: {len(defects)} (required 0) -> {'PASS' if not defects else 'FAIL'}")
    print(f"report: {REPORT} | elapsed {time.time() - t0:.0f}s")
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
