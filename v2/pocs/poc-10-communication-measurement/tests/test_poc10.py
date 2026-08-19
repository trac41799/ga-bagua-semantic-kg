"""POC-10 tests: calibration sets, QA scoring, conveyance scoring, runner."""

import os
from decimal import Decimal
from fractions import Fraction

import pytest

from measure import (CALIBRATION, conveyance_score, freeze_hash, qa_questions,
                     qa_score)


def test_calibration_sets_frozen():
    p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "calibration.sha256")
    assert open(p).read().strip() == freeze_hash()


def test_calibration_6_pairs_per_task():
    assert set(CALIBRATION) == {"06", "09", "07"}
    for task, pairs in CALIBRATION.items():
        assert len(pairs) == 6, task
        for good, bad in pairs:
            assert good != bad and good and bad


def test_bad_versions_verifiably_worse_06():
    """Bad 06 summaries must drop planted deltas vs good (contain fewer value pairs)."""
    good = CALIBRATION["06"][0][0]
    bad = CALIBRATION["06"][0][1]
    assert "94%" in good and "99%" in good
    assert not ("94%" in bad and "99%" in bad)


def test_qa_questions_generated():
    before = "a 1, b 2, c 3"
    after = "a 4, b 5, c 6"
    planted = [("a", "1", "4"), ("b", "2", "5"), ("c", "3", "6")]
    qs = qa_questions(before, after, planted)
    assert len(qs) == 3
    assert "a" in qs[0] and "before" in qs[0].lower()


def test_qa_score_hand():
    planted = [("a", "1", "4"), ("b", "2", "5"), ("c", "3", "6")]
    assert qa_score("before: 1, after: 4; before: 2, after: 5; before: 3, after: 6", planted) == 1.0
    assert qa_score("before: 1, after: 4", planted) == pytest.approx(1 / 3)
    assert qa_score("nothing", planted) == 0.0


def test_conveyance_score_hand():
    assert conveyance_score("We should roll back the checkout build and notify affected users.",
                            "Roll back to the previous checkout build and notify affected users.") >= 0.5
    assert conveyance_score("The system is generally healthy.", "Roll back the checkout build.") < 0.5


def test_runner_sim(monkeypatch, tmp_path):
    import run_all
    from pathlib import Path

    clean_verdict = tmp_path / "verdict-clean-v1.md"
    monkeypatch.setattr(run_all, "CLEAN_VERDICT_PATH", str(clean_verdict))
    monkeypatch.setattr(run_all, "CLEAN_CACHE_PATH",
                        str(tmp_path / "responses-clean-v1.json"))
    historical_verdict = Path("output") / "verdict.md"
    historical_cache = Path("data") / "cache" / "responses.json"
    historical_verdict_before = historical_verdict.read_bytes()
    historical_cache_before = historical_cache.read_bytes()

    assert run_all.main(real=False) == 0
    assert clean_verdict.exists()
    assert historical_verdict.read_bytes() == historical_verdict_before
    assert historical_cache.read_bytes() == historical_cache_before


def test_clean_qa_threshold_is_exact_at_displayed_boundary():
    import run_all

    scores, delta, verdict = run_all.exact_qa_metrics(
        {"A": 54, "B": 60}, {"A": 60, "B": 60}
    )
    assert scores == {"A": Fraction(9, 10), "B": Fraction(1, 1)}
    assert delta == Fraction(1, 10)
    assert run_all.decimal_from_fraction(delta) == Decimal("0.10")
    assert f"{float(delta):+.3f}" == "+0.100"
    assert verdict is True
