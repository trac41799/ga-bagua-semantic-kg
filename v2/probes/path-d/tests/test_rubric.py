"""T-D1 rubric tests."""

import numpy as np
import pytest

from rubric import RubricError, encode_prompt, parse_encoding, parse_id_list


def test_parse_valid():
    v = parse_encoding("[0.1, 0.2, -0.3, 0.4, 0.5, -0.6, 0.7, 0.8]")
    assert len(v) == 8
    assert np.linalg.norm(v) == pytest.approx(1.0)


def test_parse_fenced():
    v = parse_encoding("```json\n[1.0,0,0,0,0,0,0,0]\n```")
    assert v[0] == pytest.approx(1.0)


def test_parse_malformed_raises():
    with pytest.raises(RubricError):
        parse_encoding("not json at all")


def test_parse_wrong_length():
    with pytest.raises(RubricError):
        parse_encoding("[0.1, 0.2]")


def test_parse_non_numeric():
    with pytest.raises(RubricError):
        parse_encoding('[0.1, "x", 0, 0, 0, 0, 0, 0]')


def test_parse_zero_vector():
    with pytest.raises(RubricError):
        parse_encoding("[0,0,0,0,0,0,0,0]")


def test_prompt_covers_roles():
    msgs = encode_prompt("a concept")
    system = msgs[0]["content"]
    for role, _ in [("receptive", None), ("constraining", None), ("generative", None)]:
        assert role in system
    assert "JSON array" in msgs[0]["content"] or "JSON" in msgs[0]["content"]


def test_parse_id_list():
    assert parse_id_list("[1, 2, 3]") == [1, 2, 3]
    with pytest.raises(RubricError):
        parse_id_list("nope")
