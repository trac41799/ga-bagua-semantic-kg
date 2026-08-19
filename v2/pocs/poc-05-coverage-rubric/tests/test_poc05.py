"""POC-05 tests: prompts, rater parse, freeze, runner."""

import json
import os

import pytest

from protocol import (AUDIT_ASPECTS, TASKS, arm_a_prompt, arm_b_prompt,
                      audit_prompt, freeze_hash, parse_audit)


def test_prompts_differ_only_by_checklist():
    a = arm_a_prompt("task")[0]["content"]
    b = arm_b_prompt("task")[0]["content"]
    assert a in b  # arm B ⊇ arm A
    assert "receptive" in b and "generative" in b


def test_checklist_has_8_roles():
    b = arm_b_prompt("task")[0]["content"]
    for role in ["receptive", "causal", "transmissive", "constraining",
                 "clarifying", "influential", "balancing", "generative"]:
        assert role in b


def test_audit_prompt_has_6_aspects():
    msgs = audit_prompt("t", "plan")
    assert "6 aspects" in msgs[0]["content"]
    assert "constraint handling" in msgs[1]["content"]
    assert "initiation triggers" in msgs[1]["content"]


def test_parse_audit_valid():
    assert parse_audit('{"0": 1, "1": 0, "2": 1, "3": 0, "4": 1, "5": 0}') == [1, 0, 1, 0, 1, 0]


def test_parse_audit_fenced():
    assert parse_audit('```json\n{"0": 0, "1": 0, "2": 0, "3": 0, "4": 0, "5": 0}\n```') == [0] * 6


def test_parse_audit_malformed_raises():
    with pytest.raises(Exception):
        parse_audit("not json")


def test_tasks_frozen():
    p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "tasks.sha256")
    assert open(p).read().strip() == freeze_hash()


def test_20_tasks_4_domains():
    assert len(TASKS) == 20
    assert len(set(t[1] for t in TASKS)) == 4


def test_runner_sim():
    import run_all
    assert run_all.main(real=False) == 0
    assert os.path.exists(os.path.join("output", "verdict.md"))
