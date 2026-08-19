"""POC-04 orchestrator: tag quality, stability, filtering, non-interference.

Modes
- default: real LLM (needs DEEPSEEK_API_KEY or repo-root .env.local); cached.
- --sim: SimulatedLLM — pipeline smoke test ONLY; produces no claim results.
- --offline: cached tags only; fails on any cache miss.

Reports (out_dir): tag_quality.md, stability.md, filtering.md,
noninterference.md, gate_summary.md; claims_ledger.csv is appended each run.
Exit code reflects run completion only; verdicts are informational rows.
"""

import argparse
import csv
import json
import os
import sys
import time

from corpus import CONCEPTS
from filter_queries import FILTER_QUERIES
from filtering import filter_precision, filter_recall
from noninterf import cosine_retrieval, rankings_identical
from reference_tags import REFERENCE_TAGS, verify_freeze
from stability import consistency
from tagger import tag
from tags import dominant_role

QUALITY_GATE = 0.80
STABILITY_GATE = 0.80
PRECISION_GATE = 0.50

REPORT_NAMES = ["tag_quality.md", "stability.md", "filtering.md",
                "noninterference.md", "gate_summary.md"]

ID_TO_ITEM = {cid: (name, domain, desc) for cid, name, domain, desc in CONCEPTS}


def _make_client(mode, cache_dir):
    if mode == "sim":
        from llm_client import SimulatedLLM
        return SimulatedLLM()
    from llm_client import LLMClient, load_api_config
    cfg = load_api_config()
    if cfg is None:
        raise RuntimeError(
            "no API key found (DEEPSEEK_API_KEY / .env.local); use --sim or --offline")
    base, key, model = cfg
    return LLMClient(base, key, model=model, budget_tokens=200_000,
                     max_tokens=256, cache_dir=cache_dir)


def run(mode, out_dir="output", cache_dir="data/cache"):
    os.makedirs(out_dir, exist_ok=True)
    os.makedirs(cache_dir, exist_ok=True)
    verify_freeze()

    client = _make_client(mode, cache_dir)
    offline = mode == "offline"

    ref_items = []
    for item in REFERENCE_TAGS:
        name, domain, desc = ID_TO_ITEM[item["id"]]
        ref_items.append((item["id"], item["dominant"], desc))

    tags_run0 = [tag(desc, client=client, run_id=0, offline=offline,
                     cache_dir=cache_dir) for _i, _d, desc in ref_items]
    tags_run1 = [tag(desc, client=client, run_id=1, offline=offline,
                     cache_dir=cache_dir) for _i, _d, desc in ref_items]

    quality_matches = [
        dominant_role(t0) == human
        for (_i, human, _d), t0 in zip(ref_items, tags_run0)
    ]
    quality = sum(quality_matches) / len(quality_matches)
    stability_val = consistency([tags_run0, tags_run1])

    corpus_tags = [tag(desc, client=client, run_id=0, offline=offline,
                       cache_dir=cache_dir)
                   for _cid, _name, _dom, desc in CONCEPTS]
    items_with_roles = [(cid, t) for (cid, *_rest), t in zip(CONCEPTS, corpus_tags)]

    filter_rows = []
    for q in FILTER_QUERIES:
        prec = filter_precision(q["role"], items_with_roles, q["ground_truth"])
        rec = filter_recall(q["role"], items_with_roles, q["ground_truth"])
        retrieved = [cid for cid, t in items_with_roles
                     if dominant_role(t) == q["role"]]
        filter_rows.append((q, retrieved, prec, rec))
    mean_precision = (sum(r[2] for r in filter_rows) / len(filter_rows))

    with_items = [(cid, name, desc, tags) for (cid, name, _d, desc), tags
                  in zip(CONCEPTS, corpus_tags)]
    without_items = [(cid, name, desc, None) for cid, name, _d, desc in CONCEPTS]
    ni_rows = []
    for q in FILTER_QUERIES:
        fn = lambda items, q=q: cosine_retrieval(q["query"], items)
        ok = rankings_identical(fn, with_items, without_items)
        ni_rows.append((q, ok))
    noninterf_ok = all(ok for _q, ok in ni_rows)

    results = {
        "mode": mode,
        "quality": quality, "quality_matches": quality_matches,
        "stability": stability_val,
        "mean_precision": mean_precision, "filter_rows": filter_rows,
        "noninterf_ok": noninterf_ok, "ni_rows": ni_rows,
        "tags_run0": tags_run0, "tags_run1": tags_run1,
        "corpus_tags": corpus_tags,
    }

    _write_reports(results, out_dir)
    _append_ledger(results, os.path.join(out_dir, "claims_ledger.csv"))
    _print_summary(results)
    return results


def _write_reports(results, out_dir):
    quality, matches = results["quality"], results["quality_matches"]
    tags_run0, tags_run1 = results["tags_run0"], results["tags_run1"]
    lines = ["# Tag quality (LLM dominant role vs human, 30 items)",
             "",
             "Single annotator (builder); human tags frozen at reference_tags.sha256.",
             "",
             "| id | name | human dominant | LLM dominant | match |",
             "|----|------|----------------|--------------|-------|"]
    for item, t0, ok in zip(REFERENCE_TAGS, tags_run0, matches):
        name, _domain, _desc = ID_TO_ITEM[item["id"]]
        lines.append(f"| {item['id']} | {name} | {item['dominant']} | "
                     f"{dominant_role(t0)} | {'yes' if ok else 'no'} |")
    lines.append("")
    lines.append(f"**Tag quality: {sum(matches)}/30 = {quality:.1%} "
                 f"(gate >= 80%)**")
    with open(os.path.join(out_dir, "tag_quality.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

    lines = ["# Tag stability (dominant role across 2 LLM runs, temperature 0)",
             "",
             "| id | name | run1 dominant | run2 dominant | match |",
             "|----|------|---------------|---------------|-------|"]
    for item, t0, t1 in zip(REFERENCE_TAGS, tags_run0, tags_run1):
        name, _domain, _desc = ID_TO_ITEM[item["id"]]
        ok = dominant_role(t0) == dominant_role(t1)
        lines.append(f"| {item['id']} | {name} | {dominant_role(t0)} | "
                     f"{dominant_role(t1)} | {'yes' if ok else 'no'} |")
    lines.append("")
    lines.append(f"**Stability: {results['stability']:.1%} (gate >= 80%)**")
    with open(os.path.join(out_dir, "stability.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

    lines = ["# Filtering (role filter vs ground truth, 10 queries)",
             "",
             "Retrieved = items whose dominant role equals the query role. "
             "Precision = relevant/retrieved.",
             "",
             "| qid | role | retrieved | precision | recall |",
             "|-----|------|-----------|-----------|--------|"]
    for q, retrieved, prec, rec in results["filter_rows"]:
        lines.append(f"| {q['id']} | {q['role']} | {len(retrieved)} | "
                     f"{prec:.2f} | {rec:.2f} |")
    lines.append("")
    lines.append(f"**Mean precision: {results['mean_precision']:.2f} "
                 f"(gate >= 0.50)**")
    with open(os.path.join(out_dir, "filtering.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

    lines = ["# Non-interference (tagged vs untagged ranking)",
             "",
             "Retrieval stand-in: TF-IDF cosine over (name, description) only. "
             "The 8 roles are tags, never the embedding.",
             "",
             "| qid | query | rankings identical |",
             "|-----|-------|--------------------|"]
    for q, ok in results["ni_rows"]:
        lines.append(f"| {q['id']} | {q['query']} | {ok} |")
    lines.append("")
    lines.append(f"**Non-interference: {results['noninterf_ok']} "
                 f"(gate: True)**")
    with open(os.path.join(out_dir, "noninterference.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

    rows = [
        ("Tag quality (LLM vs human dominant role)", "quality >= 80%",
         f"{results['quality']:.1%}", _verdict(results["quality"] >= QUALITY_GATE)),
        ("Tag stability (2 runs, temperature 0)", "stability >= 80%",
         f"{results['stability']:.1%}", _verdict(results["stability"] >= STABILITY_GATE)),
        ("Filtering precision (10 queries)", "mean precision >= 0.5",
         f"{results['mean_precision']:.2f}", _verdict(results["mean_precision"] >= PRECISION_GATE)),
        ("Non-interference (tagged vs untagged)", "rankings identical",
         str(results["noninterf_ok"]), _verdict(results["noninterf_ok"])),
    ]
    lines = ["# Gate summary — POC-04 interpretable-tag agent memory",
             "",
             "Mode: " + results["mode"] +
             (" (SIMULATED — pipeline smoke test, not a claim)" if results["mode"] == "sim" else ""),
             "",
             "| Claim | Criterion | Measured | Verdict |",
             "|-------|-----------|----------|---------|"]
    for name, criterion, measured, verdict in rows:
        lines.append(f"| {name} | {criterion} | {measured} | {verdict} |")
    overall = all(v == "PASS" for _n, _c, _m, v in rows)
    lines.append("")
    lines.append(f"**Overall gate: {'PASS' if overall else 'FAIL'}**")
    lines.append("")
    lines.append("Notes: retrieval accuracy is OUT of scope for POC-04 (real "
                 "dense embeddings are a documented dependency); the 8 roles "
                 "are tags on top of retrieval, never the embedding/retrieval "
                 "vector; no token-economics claims are made.")
    with open(os.path.join(out_dir, "gate_summary.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def _verdict(ok):
    return "PASS" if ok else "FAIL"


def _append_ledger(results, path):
    stamp = time.strftime("%Y-%m-%dT%H:%M:%S")
    rows = [
        (stamp, results["mode"], "tag_quality", "30 reference items",
         results["quality"], QUALITY_GATE, _verdict(results["quality"] >= QUALITY_GATE)),
        (stamp, results["mode"], "stability", "30 reference items x2 runs",
         results["stability"], STABILITY_GATE, _verdict(results["stability"] >= STABILITY_GATE)),
        (stamp, results["mode"], "filtering_precision_mean", "10 filter queries",
         results["mean_precision"], PRECISION_GATE, _verdict(results["mean_precision"] >= PRECISION_GATE)),
        (stamp, results["mode"], "noninterference", "10 queries, tagged vs untagged",
         1.0 if results["noninterf_ok"] else 0.0, 1.0, _verdict(results["noninterf_ok"])),
        (stamp, results["mode"], "reference_freeze", "reference_tags.sha256",
         1.0, 1.0, "PASS"),
    ]
    new = not os.path.exists(path)
    with open(path, "a", encoding="utf-8", newline="") as f:
        w = csv.writer(f)
        if new:
            w.writerow(["run_timestamp", "mode", "phase", "scope",
                        "metric_value", "gate", "verdict"])
        w.writerows(rows)


def _print_summary(results):
    print(f"POC-04 run [mode={results['mode']}]")
    print(f"  tag quality    : {results['quality']:.1%} (gate >= 80%)")
    print(f"  stability      : {results['stability']:.1%} (gate >= 80%)")
    print(f"  filter precision: {results['mean_precision']:.2f} (gate >= 0.50)")
    print(f"  non-interference: {results['noninterf_ok']} (gate: True)")


def main(argv=None):
    p = argparse.ArgumentParser(description="POC-04 interpretable-tag agent memory")
    p.add_argument("--offline", action="store_true",
                   help="use cached tags only; fail on cache miss")
    p.add_argument("--sim", action="store_true",
                   help="SimulatedLLM smoke run (not a claim measurement)")
    p.add_argument("--out-dir", default="output")
    p.add_argument("--cache-dir", default="data/cache")
    args = p.parse_args(argv)
    mode = "sim" if args.sim else ("offline" if args.offline else "real")
    try:
        run(mode, out_dir=args.out_dir, cache_dir=args.cache_dir)
    except Exception as e:  # noqa: BLE001 — CLI boundary
        print(f"ERROR: {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
