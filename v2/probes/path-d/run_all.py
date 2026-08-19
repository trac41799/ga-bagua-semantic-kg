"""Path D runner: real-LLM end-to-end with caching + offline mode.

Usage:
    python run_all.py             # real LLM (encodes, verifies, full-context, ledger)
    python run_all.py --offline   # cached encodings only; no network (PENDING if no cache)
"""

import argparse
import json
import os
import time

import numpy as np

from corpus import CONCEPTS, QUERIES
from eval import (break_even_queries, name_match_recall, retrieval_metrics,
                  selection_metrics, write_reports)
from llm_client import LLMClient, SimulatedLLM, load_api_config
from retrieval import BM25, TFIDF, cosine_topk, random_topk, topk_from_scores
from rubric import RubricError

HERE = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(HERE, "data", "cache")


def load_cache(name):
    p = os.path.join(CACHE, name)
    if os.path.exists(p):
        with open(p, encoding="utf-8") as f:
            return json.load(f)
    return {}


def save_cache(name, data):
    os.makedirs(CACHE, exist_ok=True)
    with open(os.path.join(CACHE, name), "w", encoding="utf-8") as f:
        json.dump(data, f, indent=1)


def main(offline=False):
    t0 = time.time()
    n = len(CONCEPTS)
    names = [c[1] for c in CONCEPTS]
    descriptions = [c[3] for c in CONCEPTS]
    id_to_name = {c[0]: c[1] for c in CONCEPTS}
    ground_truth = [set(q[2]) for q in QUERIES]
    gt_names = [[id_to_name[i] for i in q[2]] for q in QUERIES]

    enc_cache = load_cache("encodings.json")
    real = None
    if not offline:
        cfg = load_api_config()
        if cfg is None:
            print("no API key found — switching to offline (PENDING)")
            offline = True
        else:
            real = LLMClient(*cfg)

    status = "OK"
    encodings = np.zeros((n, 8))
    encode_tokens = 0
    for cid, name, domain, desc in CONCEPTS:
        key = str(cid)
        if key in enc_cache:
            encodings[cid] = np.array(enc_cache[key])
            continue
        if offline:
            status = "PENDING" if encodings[cid].sum() == 0 else status
            continue
        try:
            v, usage = real.encode_concept(desc)
            encodings[cid] = v
            enc_cache[key] = v.tolist()
            encode_tokens += usage.get("total_tokens", 0)
        except (RubricError, RuntimeError) as e:
            print(f"  encode failed [{name}]: {e}")
            status = "PARTIAL"
    if not offline:
        save_cache("encodings.json", enc_cache)

    # query encodings (one-time setup cost)
    q_enc_cache = load_cache("query_encodings.json")
    query_vecs = []
    for qid, text, _ in QUERIES:
        key = str(qid)
        if key in q_enc_cache:
            query_vecs.append(np.array(q_enc_cache[key]))
        elif offline:
            query_vecs.append(None)
        else:
            try:
                v, usage = real.encode_concept("QUERY: " + text)
                query_vecs.append(v)
                q_enc_cache[key] = v.tolist()
                encode_tokens += usage.get("total_tokens", 0)
            except Exception as e:
                print(f"  query encode failed [{qid}]: {e}")
                query_vecs.append(None)
    if not offline:
        save_cache("query_encodings.json", q_enc_cache)

    # ---- retrieval (no LLM needed) ----
    tfidf = TFIDF(descriptions)
    bm25 = BM25(descriptions)
    ranked = {"rubric": [], "tfidf": [], "bm25": [], "random": []}
    for qi, (qid, text, _) in enumerate(QUERIES):
        if query_vecs[qi] is not None:
            ranked["rubric"].append([i for i, _ in cosine_topk(encodings, query_vecs[qi], 10)])
        else:
            ranked["rubric"].append([])
        ranked["tfidf"].append(topk_from_scores(tfidf.scores(text), 10))
        ranked["bm25"].append(topk_from_scores(bm25.scores(text), 10))
        ranked["random"].append(random_topk(n, 10, SEED + qi))
    retrieval = {k: retrieval_metrics(v, ground_truth) for k, v in ranked.items()}
    if status == "PENDING":
        retrieval = None  # no rubric encodings -> no verdicts (PENDING, not FAIL)

    # ---- pipeline + full-context (LLM) ----
    pipeline = {"rubric": []}
    full_ctx = []
    verify_per_query = 0.0
    full_per_query = 0.0
    v_cache = load_cache("verify.json")
    f_cache = load_cache("full_context.json")
    for qi, (qid, text, _) in enumerate(QUERIES):
        candidates = [(cid, names[cid]) for cid in ranked["rubric"][qi]]
        key = str(qid)
        if key in v_cache and not offline:
            chosen = v_cache[key]
        else:
            if offline:
                chosen = []
            else:
                chosen, usage = real.verify_candidates(text, candidates)
                v_cache[key] = chosen
                verify_per_query += usage.get("total_tokens", 0)
        pipeline["rubric"].append(selection_metrics(chosen, ground_truth[qi]))

        if key in f_cache and not offline:
            ans = f_cache[key]
        else:
            if offline:
                ans = ""
            else:
                ans, usage = real.full_context(text, [f"{c[0]}: {c[1]}: {c[3]}" for c in CONCEPTS])
                f_cache[key] = ans
                full_per_query += usage.get("total_tokens", 0)
        full_ctx.append({"recall": name_match_recall(ans, gt_names[qi]),
                         "precision": 0.0, "f1": 0.0})
    if not offline:
        save_cache("verify.json", v_cache)
        save_cache("full_context.json", f_cache)

    verify_per_query /= len(QUERIES)
    full_per_query /= len(QUERIES)

    verdicts = write_reports(
        outdir=os.path.join(HERE, "output"),
        retrieval=retrieval, pipeline=pipeline, full_context=full_ctx, status=status,
        encode_tokens=encode_tokens, verify_per_query=verify_per_query,
        full_per_query=full_per_query)

    print("=" * 64)
    print("PATH D GATE SUMMARY")
    print("=" * 64)
    print(f"status: {status}  | elapsed {time.time()-t0:.0f}s")
    if retrieval:
        for name, m in retrieval.items():
            print(f"  {name:8s} R@5={m['R@5']:.3f} R@10={m['R@10']:.3f} MRR={m['MRR']:.3f}")
    else:
        print("  (retrieval PENDING — no rubric encodings)")
    pipe_r = np.mean([s["recall"] for s in pipeline["rubric"]])
    full_r = np.mean([s["recall"] for s in full_ctx])
    print(f"  pipeline recall (rubric top-10 + verify): {pipe_r:.3f}  | full-context recall: {full_r:.3f}")
    print(f"  tokens: encode={encode_tokens} verify/q={verify_per_query:.1f} full/q={full_per_query:.1f} "
          f"break-even={break_even_queries(encode_tokens, full_per_query, verify_per_query)}")
    for k, v in verdicts.items():
        print(f"  {k}: {v}")
    return verdicts


SEED = 20260809


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--offline", action="store_true")
    args = ap.parse_args()
    main(offline=args.offline)
