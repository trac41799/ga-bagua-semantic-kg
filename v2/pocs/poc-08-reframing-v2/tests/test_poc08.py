"""POC-08 tests: few-shot prompt, moves re-verify, metrics, runner."""

import os

import pytest

from moves import START_STATE, all_positions, complement
from naming import MOVE_EXEMPLARS, arm_a_prompt, naming_prompt, parse_8
from run_all import diversity


def test_few_shot_exemplars_per_move():
    for move in ["flip", "double_flip", "complement"]:
        assert len(MOVE_EXEMPLARS.get(move, [])) >= 2, move


def test_naming_prompt_contains_examples():
    msgs = naming_prompt("statement", "flip", "Zhen")
    assert "Examples:" in msgs[1]["content"]
    assert "flip" in msgs[1]["content"].lower()


def test_arm_a_prompt_unchanged_format():
    p = arm_a_prompt("s")[0]["content"]
    assert "JSON array of 8" in p


def test_parse_8():
    assert len(parse_8('["a","b","c","d","e","f","g","h"]')) == 8
    with pytest.raises(Exception):
        parse_8('["a","b"]')


def test_positions_8_distinct():
    pos = all_positions(START_STATE)
    assert len(pos) == 8
    assert len({s for _, s in pos}) == 8


def test_complement_identity():
    pairs = {"kan": "li", "gen": "dui", "zhen": "xun", "kun": "qian"}
    for a, b in pairs.items():
        from moves import TRIGRAMS
        ca = complement(next(s for _, s in all_positions(START_STATE)))
        assert isinstance(ca, tuple)  # exact operator works
    # spot: complement of Kun (000) must be Qian (111 = blade 7)
    assert complement((0, +1))[0] == 7


def test_diversity_hand():
    import numpy as np
    v = [np.array([1.0, 0, 0, 0, 0, 0, 0, 0]), np.array([0.0, 1, 0, 0, 0, 0, 0, 0])]
    assert diversity(v) == pytest.approx(1.0)
    v2 = [np.array([1.0, 0, 0, 0, 0, 0, 0, 0]), np.array([1.0, 0, 0, 0, 0, 0, 0, 0])]
    assert diversity(v2) == pytest.approx(0.0)


def test_runner_sim():
    import run_all
    assert run_all.main(real=False) == 0
    assert os.path.exists(os.path.join("output", "verdict.md"))
