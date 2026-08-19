"""POC-03 runner: Arm A (free-form) vs Arm B (algebra-grounded) on 20 statements.

Usage:
    python run_all.py --sim             # SimulatedLLM (tests / verification only)
    python run_all.py                   # real LLM, cached responses (validation phase)
    python run_all.py --offline         # cache only; aborts with exit 1 on any miss

Writes (into output/):
    arm_comparison.md   per-arm means + per-domain breakdown
    verdict.md          proxy claim PASS/FAIL (+0.15 diversity delta, >= 3.5 coherence)
    claims_ledger.csv   per-set rows, appended across runs (run_id distinguishes)

Exit codes: 0 ok, 1 LLM/cache/budget failure, 2 freeze-marker mismatch.
"""

import argparse
import csv
import sys
import time
from pathlib import Path

import moves
import statements
import naming
import metrics
from llm_client import (BudgetError, CacheMissError, CachedClient, LLMClient,
                        RealEngine, SimulatedLLM, load_api_config)

ROOT = Path(__file__).resolve().parent


def build_engine(args):
    if args.sim:
        return SimulatedLLM()
    config = load_api_config()
    if config is None and not args.offline:
        raise RuntimeError("no DEEPSEEK_API_KEY / OPENROUTER_API_KEY found (env or repo "
                           "root .env.local); use --offline for cache-only runs")
    backend = None
    if config is not None:
        base_url, api_key, model = config
        backend = LLMClient(base_url, api_key, model=model)
    cached = CachedClient(backend, cache_dir=args.cache_dir, offline=args.offline,
                          model=config[2] if config else "deepseek-chat")
    return RealEngine(cached)


def _snapshot(engine):
    return dict(engine.failures), dict(engine.usage), engine.cache_hits


def _delta(before, after):
    return {k: after[k] - before[k] for k in after}


def run_arm(engine, stmt, arm):
    f0, u0, c0 = _snapshot(engine)
    if arm == "A":
        reframes = engine.free_form_reframes(stmt["text"])
    else:
        reframes = []
        for move_name, state in moves.all_positions(moves.START_STATE):
            name = naming.name_position(stmt["text"], move_name, state)
            if name:
                reframes.append(name)
    f1, u1, c1 = _snapshot(engine)
    naming_failures = sum(_delta(f0, f1).values())

    div = metrics.diversity(reframes)
    f2, u2, c2 = _snapshot(engine)
    encode_failures = sum(_delta(f1, f2).values())

    coh = metrics.coherence(stmt["text"], reframes)
    f3, u3, c3 = _snapshot(engine)
    judge_failures = sum(_delta(f2, f3).values())

    u_used = _delta(u0, u3)
    return {
        "arm": arm,
        "n_reframes": len(reframes),
        "naming_failures": naming_failures,
        "encode_failures": encode_failures,
        "judge_failures": judge_failures,
        "diversity": div,
        "coherence": coh,
        "tokens": u_used["total_tokens"],
        "calls": u_used["calls"],
        "cache_hits": c3 - c0,
    }


def _fmt(x, spec=".4f"):
    return f"{x:{spec}}"


def render_comparison(rows, run_id, engine_info, freeze_ok):
    lines = [
        "# Arm comparison -- POC-03 reframing engine",
        "",
        f"- run_id: `{run_id}`",
        f"- engine: {engine_info}",
        f"- statements: 20 frozen, 4 domains x 5 (freeze marker ok: {freeze_ok})",
        f"- protocol: Arm A = 1 free-form call (8 reframes); "
        f"Arm B = 8 exact cube moves, 1 naming call per position",
        "",
        "## Overall",
        "",
        "| arm | sets | mean diversity | mean coherence | naming failures | "
        "encode failures | judge failures | tokens |",
        "|-----|------|---------------|----------------|-----------------|-----------------|----------------|--------|",
    ]
    for arm in ("A", "B"):
        rs = [r for r in rows if r["arm"] == arm]
        md = sum(r["diversity"] for r in rs) / len(rs) if rs else 0.0
        mc = sum(r["coherence"] for r in rs) / len(rs) if rs else 0.0
        lines.append(
            f"| {arm} | {len(rs)} | {_fmt(md)} | {_fmt(mc)} | "
            f"{sum(r['naming_failures'] for r in rs)} | {sum(r['encode_failures'] for r in rs)} | "
            f"{sum(r['judge_failures'] for r in rs)} | {sum(r['tokens'] for r in rs)} |"
        )
    lines += ["", "## Per domain", "",
              "| domain | arm | sets | mean diversity | mean coherence |", 
              "|--------|-----|------|---------------|----------------|"]
    for domain in statements.DOMAINS:
        for arm in ("A", "B"):
            rs = [r for r in rows if r["arm"] == arm and r["domain"] == domain]
            md = sum(r["diversity"] for r in rs) / len(rs) if rs else 0.0
            mc = sum(r["coherence"] for r in rs) / len(rs) if rs else 0.0
            lines.append(f"| {domain} | {arm} | {len(rs)} | {_fmt(md)} | {_fmt(mc)} |")
    lines.append("")
    return "\n".join(lines)


def render_verdict(rows, run_id, engine_info, tokens_total, failures_total):
    da = [r["diversity"] for r in rows if r["arm"] == "A"]
    db = [r["diversity"] for r in rows if r["arm"] == "B"]
    ca = [r["coherence"] for r in rows if r["arm"] == "A"]
    cb = [r["coherence"] for r in rows if r["arm"] == "B"]
    mean_da = sum(da) / len(da) if da else 0.0
    mean_db = sum(db) / len(db) if db else 0.0
    mean_cb = sum(cb) / len(cb) if cb else 0.0
    delta = mean_db - mean_da
    cond_div = delta >= 0.15
    cond_coh = mean_cb >= 3.5
    verdict = "PASS" if (cond_div and cond_coh) else "FAIL"
    lines = [
        "# Verdict -- POC-03 reframing engine",
        "",
        f"- run_id: `{run_id}`",
        f"- engine: {engine_info}",
        "",
        "## Proxy claim (pre-registered)",
        "",
        "Mean pairwise semantic distance of the 8 algebra-grounded views >= LLM free-form "
        "reframes + **0.15** (cosine on 8-dim rubric vectors, same encoder both arms), "
        "AND mean coherence (LLM-judged, 1-5) >= 3.5.",
        "",
        "| quantity | Arm A | Arm B | delta | threshold | condition |",
        "|----------|-------|-------|-------|-----------|-----------|",
        f"| mean diversity | {_fmt(mean_da)} | {_fmt(mean_db)} | {_fmt(delta)} | +0.15 | "
        f"{'PASS' if cond_div else 'FAIL'} |",
        f"| mean coherence | {_fmt(sum(ca)/len(ca) if ca else 0.0)} | {_fmt(mean_cb)} | "
        f"{_fmt(mean_cb - (sum(ca)/len(ca) if ca else 0.0))} | >= 3.5 | "
        f"{'PASS' if cond_coh else 'FAIL'} |",
        "",
        f"## Proxy claim: **{verdict}**",
        "",
        "Kill criterion: the proxy claim failing kills POC-03 as a claim (the moves remain "
        "useful only pedagogically -> Path C, not a product).",
        "",
        "Human gate (pre-registered, runs after the proxy passes): >=70% of raters (n>=5) "
        "rate the algebra-grounded set 'more systematically complete' than the free-form set. "
        "See SDD: ../../docs/specs/poc-03-reframing-engine-spec.md (section 1, Pre-registration).",
        "",
        f"- total tokens (live calls): {tokens_total}",
        f"- total protocol failures (all arms): {failures_total}",
        "",
    ]
    return "\n".join(lines)


def write_ledger(out_dir, rows):
    path = out_dir / "claims_ledger.csv"
    header = ["run_id", "statement_id", "domain", "arm", "n_reframes", "naming_failures",
              "encode_failures", "judge_failures", "diversity", "coherence", "tokens", "calls",
              "cache_hits"]
    new_file = not path.exists()
    with open(path, "a", newline="", encoding="utf-8") as fh:
        writer = csv.writer(fh)
        if new_file:
            writer.writerow(header)
        for r in rows:
            writer.writerow([r["run_id"], r["statement_id"], r["domain"], r["arm"],
                             r["n_reframes"], r["naming_failures"], r["encode_failures"],
                             r["judge_failures"], _fmt(r["diversity"]), _fmt(r["coherence"]),
                             r["tokens"], r["calls"], r["cache_hits"]])


def run(args):
    """Run the full protocol; returns the process exit code."""
    out_dir = Path(args.output_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    if not statements.verify_frozen():
        print(f"[poc-03] ERROR: statements.sha256 does not match statements.py "
              f"(expected {statements.freeze_marker()}) -- refusing to run")
        return 2

    try:
        engine = build_engine(args)
    except RuntimeError as e:
        print(f"[poc-03] ERROR: {e}")
        return 1

    naming.set_engine(engine)
    metrics.set_engine(engine)
    run_id = args.run_id or time.strftime("%Y%m%dT%H%M%SZ", time.gmtime())

    rows = []
    try:
        for stmt in statements.STATEMENTS:
            for arm in ("A", "B"):
                res = run_arm(engine, stmt, arm)
                res.update({"run_id": run_id, "statement_id": stmt["id"],
                            "domain": stmt["domain"]})
                rows.append(res)
    except CacheMissError as e:
        print(f"[poc-03] ERROR: cache miss in offline mode: {e}")
        print("[poc-03] no live calls were made; nothing was written")
        return 1
    except BudgetError as e:
        print(f"[poc-03] ERROR: budget cap: {e}")
        return 1

    engine_info = "SIMULATED (SimulatedLLM, tests only)" if args.sim else \
        f"REAL (deepseek-chat, cached in {args.cache_dir})"
    comp = render_comparison(rows, run_id, engine_info, True)
    verdict = render_verdict(rows, run_id, engine_info,
                             sum(r["tokens"] for r in rows),
                             sum(r["naming_failures"] + r["encode_failures"] +
                                 r["judge_failures"] for r in rows))
    (out_dir / "arm_comparison.md").write_text(comp, encoding="utf-8")
    (out_dir / "verdict.md").write_text(verdict, encoding="utf-8")
    write_ledger(out_dir, rows)

    da = [r["diversity"] for r in rows if r["arm"] == "A"]
    db = [r["diversity"] for r in rows if r["arm"] == "B"]
    cb = [r["coherence"] for r in rows if r["arm"] == "B"]
    print(f"[poc-03] run {run_id}: Arm A mean diversity {sum(da)/len(da):.4f} | "
          f"Arm B mean diversity {sum(db)/len(db):.4f} | delta {sum(db)/len(db)-sum(da)/len(da):+.4f} | "
          f"Arm B mean coherence {sum(cb)/len(cb):.4f}")
    print(f"[poc-03] reports written to {out_dir}")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description="POC-03 reframing engine: Arm A vs Arm B")
    ap.add_argument("--sim", action="store_true", help="use SimulatedLLM (tests only)")
    ap.add_argument("--offline", action="store_true", help="cache only; abort on cache miss")
    ap.add_argument("--cache-dir", default=str(ROOT / "data" / "cache"),
                    help="LLM response cache directory")
    ap.add_argument("--output-dir", default=str(ROOT / "output"),
                    help="where reports + ledger are written")
    ap.add_argument("--run-id", default=None, help="explicit run id (default: UTC timestamp)")
    return run(ap.parse_args(argv))


if __name__ == "__main__":
    sys.exit(main())
