"""T-04.2 Metrics: consistency, filter precision/recall, non-interference."""

import pytest

from filtering import filter_precision, filter_recall
from noninterf import cosine_retrieval, rankings_identical
from stability import consistency
from tags import ROLES

ROLES_SET = set(ROLES)


def make_tags(dominant, strength=0.9, others=0.0):
    return {r: (strength if r == dominant else others) for r in ROLES}


def test_0421_consistency_hand_case_8_of_10():
    doms = ["receptive", "causal", "transmissive", "constraining", "clarifying",
            "influential", "balancing", "generative", "receptive", "causal"]
    run_a = [make_tags(r, 0.9) for r in doms]
    run_b = list(run_a)
    run_b[0] = make_tags("generative", 0.9)
    run_b[5] = make_tags("receptive", 0.9)
    assert consistency([run_a, run_b]) == pytest.approx(0.8)


def test_0421_consistency_requires_two_runs():
    run_a = [make_tags("causal")]
    with pytest.raises(ValueError):
        consistency([run_a])
    with pytest.raises(ValueError):
        consistency([])


def test_0422_filter_precision_hand_case_3_of_5():
    items = [(i, make_tags("constraining")) for i in range(5)]
    gt = {0, 2, 4}
    assert filter_precision("constraining", items, gt) == pytest.approx(0.6)


def test_0423_filter_recall_hand_case_3_of_4():
    items = [(i, make_tags("constraining")) for i in range(5)]
    gt = {0, 2, 4, 99}
    assert filter_recall("constraining", items, gt) == pytest.approx(0.75)


def test_0422_precision_empty_retrieval_is_zero():
    items = [(0, make_tags("balancing")), (1, make_tags("generative"))]
    assert filter_precision("constraining", items, {0}) == 0.0


def test_0423_recall_empty_ground_truth_is_zero():
    items = [(0, make_tags("constraining"))]
    assert filter_recall("constraining", items, []) == 0.0


def test_0424_noninterference_identical_rankings_true():
    retrieval = lambda items: [cid for cid, *_ in items]
    with_tags = [(1, "a", "x", {"tags": "whatever"}), (2, "b", "y", None)]
    without_tags = [(1, "a", "x", None), (2, "b", "y", None)]
    assert rankings_identical(retrieval, with_tags, without_tags) is True


def test_0424_noninterference_different_rankings_false():
    retrieval = lambda items: [cid for cid, *_ in items]
    a = [(1, "a", "x", {"t": 1}), (2, "b", "y", {"t": 2})]
    b = [(2, "b", "y", None), (1, "a", "x", None)]
    assert rankings_identical(retrieval, a, b) is False


def test_0424_cosine_retrieval_ignores_tags():
    base = [(0, "rate limiter", "restricts requests within a time window", None),
            (1, "message queue", "relays events between services", None),
            (2, "monitoring dashboard", "visualizes metrics and alerts", None)]
    with_tags = [(cid, n, d, {"receptive": 1.0, "causal": 0.5, "constraining": -1.0})
                 for cid, n, d, _ in base]
    without_tags = [(cid, n, d, None) for cid, n, d, _ in base]
    q = "restricts requests"
    assert cosine_retrieval(q, with_tags) == cosine_retrieval(q, without_tags)


def test_0424_cosine_retrieval_rank_is_sensible():
    items = [(0, "rate limiter", "restricts requests within a time window", None),
             (1, "message queue", "relays events between services", None),
             (2, "cache layer", "stores frequently accessed data in memory", None)]
    ranked = cosine_retrieval("which component restricts requests", items)
    assert ranked[0] == 0
    assert set(ranked) == {0, 1, 2}
