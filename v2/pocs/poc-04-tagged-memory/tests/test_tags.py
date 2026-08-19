"""T-04.1 Tags: parse_tags, dominant_role, prompt, reference-tag integrity."""

import json

import pytest

import reference_tags
from tags import ROLES, ROLE_GLOSS, TAG_PROMPT, TagError, dominant_role, parse_tags

VALID = json.dumps(
    {"receptive": 0.2, "causal": -0.5, "transmissive": 0.1, "constraining": 0.9,
     "clarifying": 0.0, "influential": -0.3, "balancing": 0.4, "generative": -0.2})


def make_tags(dominant, strength=0.9, others=0.0):
    return {r: (strength if r == dominant else others) for r in ROLES}


def test_0411_parse_tags_valid():
    tags = parse_tags(VALID)
    assert set(tags) == set(ROLES)
    assert len(tags) == 8
    assert all(-1.0 <= v <= 1.0 for v in tags.values())
    assert tags["constraining"] == 0.9


def test_0411_parse_tags_fenced_json_ok():
    tags = parse_tags("```json\n" + VALID + "\n```")
    assert set(tags) == set(ROLES)


def test_0412_parse_tags_malformed_raises_typed_error():
    with pytest.raises(TagError):
        parse_tags("not json at all {{{")
    with pytest.raises(TagError):
        parse_tags("[1, 2, 3]")
    with pytest.raises(TagError):
        parse_tags("")


def test_0412_parse_tags_out_of_range_raises():
    bad = json.loads(VALID)
    bad["balancing"] = 1.5
    with pytest.raises(TagError):
        parse_tags(json.dumps(bad))


def test_0413_parse_tags_missing_role_raises():
    bad = json.loads(VALID)
    del bad["causal"]
    with pytest.raises(TagError):
        parse_tags(json.dumps(bad))


def test_0413_parse_tags_extra_key_raises():
    bad = json.loads(VALID)
    bad["nova"] = 0.1
    with pytest.raises(TagError):
        parse_tags(json.dumps(bad))


def test_0413_parse_tags_non_numeric_value_raises():
    bad = json.loads(VALID)
    bad["causal"] = "strong"
    with pytest.raises(TagError):
        parse_tags(json.dumps(bad))


def test_0414_dominant_role_strong_wins():
    assert dominant_role(make_tags("generative", 0.9)) == "generative"
    assert dominant_role(make_tags("constraining", 0.4)) == "constraining"


def test_0414_dominant_role_tie_goes_first_in_fixed_order():
    tags = {r: 0.5 for r in ROLES}
    assert dominant_role(tags) == ROLES[0] == "receptive"
    tags["transmissive"] = 0.5
    assert dominant_role(tags) == "receptive"
    tags["transmissive"] = 0.7
    assert dominant_role(tags) == "transmissive"


def test_0414_dominant_role_negative_values_pick_least_suppressed():
    tags = {r: -0.4 for r in ROLES}
    tags["balancing"] = -0.1
    assert dominant_role(tags) == "balancing"


def test_tag_prompt_contains_roles_and_description():
    msgs = TAG_PROMPT("Rate Limiter restricts requests")
    assert len(msgs) == 2
    assert msgs[-1]["content"] == "Concept description: Rate Limiter restricts requests"
    text = msgs[0]["content"] + msgs[1]["content"]
    for r in ROLES:
        assert r in text
    assert ROLE_GLOSS["constraining"] in text


def test_tag_prompt_missing_description_raises():
    with pytest.raises(TypeError):
        TAG_PROMPT()


def test_reference_tags_integrity():
    items = reference_tags.REFERENCE_TAGS
    assert len(items) == 30
    counts = reference_tags.validate_reference_tags()
    assert counts == {"software": 10, "business": 8, "biology": 6, "governance": 6}
    assert set(i["dominant"] for i in items) == set(ROLES)
    for item in items:
        parse_tags(json.dumps(item["strengths"]))
        assert dominant_role(item["strengths"]) == item["dominant"]
    assert reference_tags.verify_freeze() is True
