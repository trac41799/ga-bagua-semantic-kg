"""Path D evaluation: retrieval metrics, pipeline metrics, token ledger, verdicts, reports."""

import csv
import os
from datetime import date

import numpy as np

from corpus import CONCEPTS, QUERIES

SEED = 20260809


# ---- retrieval metrics ----

def recall_at_k(ranked_ids, gt, k):
    if not gt:
        return 0.0
    return len(set(ranked_ids[:k]) & set(gt)) / len(set(gt))


def retrieval_metrics(ranked_lists, ground_truth):
    """ranked_lists: list of lists of ids (one per query); ground_truth: list of id-sets."""
    r5 = np.mean([recall_at_k(r, g, 5) for r, g in zip(ranked_lists, ground_truth)])
    r10 = np.mean([recall_at_k(r, g, 10) for r, g in zip(ranked_lists, ground_truth)])
    mrr = 0.0
    for r, g in zip(ranked_lists, ground_truth):
        for i, cid in enumerate(r):
            if cid in g:
                mrr += 1.0 / (i + 1)
                break
    mrr /= max(len(ranked_lists), 1)
    return {"R@5": float(r5), "R@10": float(r10), "MRR": float(mrr)}


def selection_metrics(chosen_ids, ground_truth):
    """Pipeline: LLM-verified selection vs ground truth."""
    chosen = set(chosen_ids)
    gt = set(ground_truth)
    if not chosen:
        return {"recall": 0.0, "precision": 0.0, "f1": 0.0}
    recall = len(chosen & gt) / len(gt) if gt else 0.0
    precision = len(chosen & gt) / len(chosen)
    f1 = 2 * precision * recall / (precision + recall) if precision + recall else 0.0
    return {"recall": float(recall), "precision": float(precision), "f1": float(f1)}


def name_match_recall(answer_text, ground_truth_names):
    """Fraction of ground-truth names found in the answer (case-insensitive substring)."""
    al = answer_text.lower()
    found = [1 for n in ground_truth_names if n.lower() in al]
    return len(found) / len(ground_truth_names) if ground_truth_names else 0.0


# ---- token economics ----

def break_even_queries(encode_tokens, full_per_query, verify_per_query):
    per_query_saving = full_per_query - verify_per_query
    if per_query_saving <= 0:
        return None
    return int(math.ceil(encode_tokens / per_query_saving))


def savings_at(encode_tokens, verify_per_query, full_per_query, n_queries):
    d_total = encode_tokens + verify_per_query * n_queries
    base_total = full_per_query * n_queries
    if base_total <= 0:
        return None
    return base_total / d_total


import math  # noqa: E402


# ---- reports ----

def write_reports(outdir="output", enc_usage=None, verify_usage=None, full_usage=None,
                  retrieval=None, pipeline=None, full_context=None, status="OK",
                  encode_tokens=0, verify_per_query=0, full_per_query=0):
    os.makedirs(outdir, exist_ok=True)
    gt = [set(q[2]) for q in QUERIES]

    # verdicts
    d1 = retrieval["rubric"]["R@10"] >= 0.60 if retrieval else None
    pipe_recall = np.mean([s["recall"] for s in pipeline["rubric"]]) if pipeline else None
    full_recall = np.mean([s["recall"] for s in full_context]) if full_context else None
    be = break_even_queries(encode_tokens, full_per_query, verify_per_query) if encode_tokens else None
    d2 = None
    if pipe_recall is not None and full_recall is not None and be is not None:
        d2 = pipe_recall >= 0.95 * full_recall and be <= 10
    d3 = None
    if retrieval:
        d3 = (retrieval["rubric"]["R@10"] - retrieval["tfidf"]["R@10"] >= 0.10
              and retrieval["rubric"]["R@10"] - retrieval["bm25"]["R@10"] >= 0.10)

    with open(os.path.join(outdir, "retrieval_metrics.md"), "w", encoding="utf-8") as f:
        f.write("# Retrieval Metrics — Path D\n\n| Method | R@5 | R@10 | MRR |\n|---|---|---|---|\n")
        if retrieval:
            for name, m in retrieval.items():
                f.write(f"| {name} | {m['R@5']:.3f} | {m['R@10']:.3f} | {m['MRR']:.3f} |\n")
        else:
            f.write("| _pending_ | — | — | — |\n")

    with open(os.path.join(outdir, "pipeline.md"), "w", encoding="utf-8") as f:
        f.write("# Pipeline (top-10 + LLM verify) vs Full-Context — Path D\n\n")
        f.write("| Method | mean recall | mean precision | mean F1 |\n|---|---|---|---|\n")
        if pipeline:
            for name, ms in pipeline.items():
                f.write(f"| {name} | {np.mean([s['recall'] for s in ms]):.3f} | "
                        f"{np.mean([s['precision'] for s in ms]):.3f} | "
                        f"{np.mean([s['f1'] for s in ms]):.3f} |\n")
        if full_context:
            f.write(f"| full-context (name match) | {full_recall:.3f} | — | — |\n")

    with open(os.path.join(outdir, "token_economics.md"), "w", encoding="utf-8") as f:
        f.write("# Token Economics — Path D\n\n")
        f.write(f"- encoding (one-time): {encode_tokens} tokens\n")
        f.write(f"- verification per query: {verify_per_query:.1f} tokens\n")
        f.write(f"- full-context per query: {full_per_query:.1f} tokens\n")
        if be:
            f.write(f"- break-even: {be} queries\n")
            f.write(f"- savings at 10 queries: {savings_at(encode_tokens, verify_per_query, full_per_query, 10):.1f}x\n")
            f.write(f"- savings at 50 queries: {savings_at(encode_tokens, verify_per_query, full_per_query, 50):.1f}x\n")
            f.write(f"- savings at 200 queries: {savings_at(encode_tokens, verify_per_query, full_per_query, 200):.1f}x\n")
        else:
            f.write("- break-even: N/A (no per-query saving)\n")

    with open(os.path.join(outdir, "gate_summary.md"), "w", encoding="utf-8") as f:
        f.write("# Gate Summary — Path D\n\n")
        f.write(f"- status: {status}\n")
        fmt = lambda v: "PASS" if v is True else ("FAIL" if v is False else "PENDING")
        if d1 is not None:
            f.write(f"- D1 (rubric R@10 >= 60%): {retrieval['rubric']['R@10']:.1%} -> {fmt(d1)}\n")
        else:
            f.write(f"- D1 (rubric R@10 >= 60%): PENDING\n")
        if d2 is not None:
            f.write(f"- D2 (pipeline >= 95% full-context recall AND break-even <= 10): "
                    f"recall {pipe_recall:.1%} vs {full_recall:.1%}, break-even {be} -> {fmt(d2)}\n")
        else:
            f.write("- D2: PENDING\n")
        if d3 is not None:
            f.write(f"- D3 (rubric >= TF-IDF+10pp AND >= BM25+10pp R@10): "
                    f"rubric {retrieval['rubric']['R@10']:.1%}, tfidf {retrieval['tfidf']['R@10']:.1%}, "
                    f"bm25 {retrieval['bm25']['R@10']:.1%} -> {fmt(d3)}\n")
        else:
            f.write("- D3: PENDING\n")

    rows = [["metric", "value", "baseline", "split", "seed", "date"]]
    if d1 is not None:
        rows.append(["D1_rubric_R10", f"{retrieval['rubric']['R@10']:.4f}", ">=0.60", "24 queries", str(SEED), str(date.today())])
    if d2 is not None:
        rows.append(["D2_pipeline_recall", f"{pipe_recall:.4f}", f"0.95*full:{full_recall:.4f}", "24 queries", str(SEED), str(date.today())])
        rows.append(["D2_break_even", f"{be}", "<=10", "ledger", str(SEED), str(date.today())])
    if d3 is not None:
        rows.append(["D3_rubric_minus_tfidf", f"{retrieval['rubric']['R@10'] - retrieval['tfidf']['R@10']:+.4f}", "+0.10", "24 queries", str(SEED), str(date.today())])
        rows.append(["D3_rubric_minus_bm25", f"{retrieval['rubric']['R@10'] - retrieval['bm25']['R@10']:+.4f}", "+0.10", "24 queries", str(SEED), str(date.today())])
    rows.append(["encode_tokens", f"{encode_tokens}", "one-time", "ledger", str(SEED), str(date.today())])
    rows.append(["verify_per_query", f"{verify_per_query:.1f}", "per-query", "ledger", str(SEED), str(date.today())])
    rows.append(["full_per_query", f"{full_per_query:.1f}", "per-query", "ledger", str(SEED), str(date.today())])
    with open(os.path.join(outdir, "claims_ledger.csv"), "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerows(rows)

    return {"d1": d1, "d2": d2, "d3": d3}
