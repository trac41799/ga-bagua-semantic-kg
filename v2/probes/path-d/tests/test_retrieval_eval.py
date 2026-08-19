"""T-D3 retrieval/baselines + T-D4/T-D5 eval, ledger, cache tests."""

import os

import numpy as np
import pytest

from retrieval import BM25, TFIDF, cosine_topk, random_topk, topk_from_scores

DOCS = ["the cat sat on the mat", "the dog chased the cat", "birds fly in the sky"]
ENC = np.array([
    [1.0, 0, 0, 0, 0, 0, 0, 0],
    [0.5, 0.5, 0, 0, 0, 0, 0, 0],
    [0.0, 1.0, 0, 0, 0, 0, 0, 0],
])


def test_cosine_topk_ordering():
    q = np.array([1.0, 0, 0, 0, 0, 0, 0, 0])
    res = cosine_topk(ENC, q, 3)
    assert res[0][0] == 0  # identical
    assert res[1][0] == 1  # 0.5/0.707 > 0


def test_tfidf_hand_computed():
    t = TFIDF(DOCS)
    s = t.scores("cat")
    # doc1: 1 cat in 5 words (tf 0.2); doc0: 1 cat in 6 words (tf 0.167); doc2: none
    assert s[1] > s[0] > 0.0
    assert s[2] == 0.0


def test_tfidf_zero_query():
    t = TFIDF(DOCS)
    assert np.allclose(t.scores("zzzzz"), 0.0)


def test_bm25_hand_computed():
    b = BM25(DOCS)
    s = b.scores("cat")
    assert s[0] > 0 and s[2] == 0.0


def test_topk_from_scores():
    assert topk_from_scores(np.array([0.1, 0.9, 0.5]), 2) == [1, 2]


def test_random_topk_seeded():
    a = random_topk(100, 10, seed=7)
    b = random_topk(100, 10, seed=7)
    assert a == b


# ---- eval ----

def test_recall_and_mrr_hand():
    from eval import retrieval_metrics
    ranked = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [5, 6, 7, 8, 9, 0, 1, 2, 3, 4]]
    gt = [{0, 1}, {5}]
    m = retrieval_metrics(ranked, gt)
    assert m["R@5"] == pytest.approx(1.0)  # q1: 0,1 in top5 (2/2); q2: 5 at rank1 (1/1)
    assert m["R@10"] == pytest.approx(1.0)
    assert m["MRR"] == pytest.approx(1.0)


def test_break_even():
    from eval import break_even_queries, savings_at
    assert break_even_queries(1000, 100, 50) == 20   # saving 50/q
    assert break_even_queries(1000, 50, 100) is None  # no saving
    assert savings_at(1000, 50, 100, 50) == pytest.approx(5000 / 3500)


def test_selection_metrics():
    from eval import selection_metrics
    m = selection_metrics([1, 2, 3], {1, 4})
    assert m["recall"] == pytest.approx(0.5)
    assert m["precision"] == pytest.approx(1 / 3)
    assert m["f1"] == pytest.approx(2 * 0.5 * (1 / 3) / (0.5 + 1 / 3))


def test_name_match_recall():
    from eval import name_match_recall
    assert name_match_recall("Rate Limiter and the Message Queue matter", ["Rate Limiter", "Message Queue"]) == 1.0
    assert name_match_recall("nothing relevant", ["Rate Limiter"]) == 0.0


def test_ledger_rows(tmp_path):
    from eval import write_reports
    r = {"rubric": {"R@5": 0.5, "R@10": 0.6, "MRR": 0.4}, "tfidf": {"R@5": 0.2, "R@10": 0.3, "MRR": 0.2},
         "bm25": {"R@5": 0.2, "R@10": 0.3, "MRR": 0.2}, "random": {"R@5": 0.1, "R@10": 0.15, "MRR": 0.1}}
    p = [{"recall": 0.5, "precision": 0.4, "f1": 0.44}] * 24
    f = [{"recall": 0.8, "precision": 0.0, "f1": 0.0}] * 24
    write_reports(str(tmp_path), retrieval=r, pipeline={"rubric": p}, full_context=f,
                  encode_tokens=5000, verify_per_query=60, full_per_query=200)
    with open(os.path.join(str(tmp_path), "claims_ledger.csv"), encoding="utf-8") as fh:
        lines = fh.readlines()
    assert lines[0].startswith("metric,value,baseline")
    assert any("encode_tokens" in l for l in lines)


def test_verdicts_render(tmp_path):
    from eval import write_reports
    r = {"rubric": {"R@5": 0.7, "R@10": 0.8, "MRR": 0.6}, "tfidf": {"R@5": 0.3, "R@10": 0.4, "MRR": 0.2},
         "bm25": {"R@5": 0.3, "R@10": 0.4, "MRR": 0.2}, "random": {"R@5": 0.1, "R@10": 0.15, "MRR": 0.1}}
    p = [{"recall": 0.9, "precision": 0.5, "f1": 0.64}] * 24
    f = [{"recall": 0.9, "precision": 0.0, "f1": 0.0}] * 24
    vs = write_reports(str(tmp_path), retrieval=r, pipeline={"rubric": p}, full_context=f,
                       encode_tokens=400, verify_per_query=50, full_per_query=100)
    with open(os.path.join(str(tmp_path), "gate_summary.md"), encoding="utf-8") as fh:
        content = fh.read()
    assert "D1" in content and "D2" in content and "D3" in content
    assert vs["d1"] is True and vs["d2"] is True and vs["d3"] is True
