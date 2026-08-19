"""POC-06 tests: planted deltas, template protocol, rater, runner."""

import os

import pytest

from transitions import TRANSITIONS, deltas_present, freeze_hash


def test_planted_deltas_present_in_pairs():
    for tid, _, before, after, planted in TRANSITIONS:
        for aspect, b, a in planted:
            assert str(b).lower() in before.lower(), (tid, aspect, b)
            assert str(a).lower() in after.lower(), (tid, aspect, a)


def test_each_transition_has_3_deltas():
    for tid, _, _, _, planted in TRANSITIONS:
        assert len(planted) == 3, tid


def test_20_transitions_4_domains():
    assert len(TRANSITIONS) == 20
    assert len(set(t[1] for t in TRANSITIONS)) == 4


def test_deltas_present_hand():
    planted = [("cache hit ratio", "94%", "99%"), ("p99 latency", "120ms", "95ms"), ("error rate", "0.2%", "0.1%")]
    assert deltas_present("cache hit ratio 94% -> 99%, p99 120ms -> 95ms, error 0.2% -> 0.1%", planted) == 1.0
    assert deltas_present("nothing relevant", planted) == 0.0


def test_freeze():
    p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "transitions.sha256")
    assert open(p).read().strip() == freeze_hash()


def test_arm_b_template_strict():
    from run_all import ARM_B
    assert "EXACTLY 3 aspect lines" in ARM_B
    assert "before -> after" in ARM_B


def test_runner_sim():
    import run_all
    assert run_all.main(real=False) == 0
    assert os.path.exists(os.path.join("output", "verdict.md"))
