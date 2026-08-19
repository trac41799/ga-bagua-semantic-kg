"""POC-09 tests: prompts, framing constraint, compliance check, runner."""

import os

import pytest

from run_all import has_trigram_pair
from scenarios import SCENARIOS, ARM_B_SYSTEM, arm_a_prompt, arm_b_prompt, freeze_hash


def test_arm_b_has_scaffold_constraint():
    assert "scaffold" in ARM_B_SYSTEM
    assert "NO predictive meaning" in ARM_B_SYSTEM or "no predictive" in ARM_B_SYSTEM


def test_arm_b_mentions_trigram_pair_structure():
    assert "upper/lower trigram pair" in ARM_B_SYSTEM
    assert "line-change pattern" in ARM_B_SYSTEM


def test_arm_a_plain():
    a = arm_a_prompt("s", "a")[0]["content"]
    assert "hexagram" not in a and "trigram" not in a


def test_arm_b_differs_from_arm_a():
    a = arm_a_prompt("s", "a")[0]["content"]
    b = arm_b_prompt("s", "a")[0]["content"]
    assert a != b


def test_has_trigram_pair():
    assert has_trigram_pair("坎 over 離 explains the dynamics")
    assert has_trigram_pair("upper 震, lower 兑")
    assert not has_trigram_pair("plain explanation without structure")


def test_20_scenarios_4_domains():
    assert len(SCENARIOS) == 20
    assert len(set(s[1] for s in SCENARIOS)) == 4


def test_freeze():
    p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "scenarios.sha256")
    assert open(p).read().strip() == freeze_hash()


def test_runner_sim():
    import run_all
    assert run_all.main(real=False) == 0
    assert os.path.exists(os.path.join("output", "verdict.md"))
