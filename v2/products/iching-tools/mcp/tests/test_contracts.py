"""Strict six-tool input and result contract tests."""

import pytest
from jsonschema import ValidationError, validate

from iching_mcp.contracts import (
    TOOL_SCHEMAS,
    ContractError,
    validate_tool_arguments,
    validate_tool_result,
)


TOOLS = {
    "coverage_audit",
    "reframe",
    "state_diff",
    "cl3_evaluate",
    "interaction_spectrum",
    "rotor_transition",
}


def coverage_result():
    roles = (
        "receptive",
        "causal",
        "transmissive",
        "constraining",
        "clarifying",
        "influential",
        "balancing",
        "generative",
    )
    return {
        "task": "launch",
        "original_plan": "Build it.",
        "audited_plan": "\n".join(f"{role}: covered" for role in roles),
        "checklist": True,
    }


def reframe_result():
    moves = [
        "origin",
        "flip0",
        "flip1",
        "flip2",
        "double_flip01",
        "double_flip02",
        "double_flip12",
        "complement",
    ]
    return {
        "statement": "s",
        "positions": [
            {
                "move": move,
                "state": f"state {index}",
                "reframe": f"reframe {move}",
                "description": f"description {move}",
            }
            for index, move in enumerate(moves)
        ],
    }


def state_diff_result():
    return {
        "before": "b",
        "after": "a",
        "aspects": [
            {"aspect": f"aspect{i}", "before": "b", "after": "a"}
            for i in range(1, 4)
        ],
    }


def test_all_six_schemas_are_strict_objects():
    assert set(TOOL_SCHEMAS) == TOOLS
    for schema in TOOL_SCHEMAS.values():
        assert schema["type"] == "object"
        assert schema["additionalProperties"] is False


def test_algebra_schemas_describe_each_operation_shape():
    representatives = {
        "cl3_evaluate": [
            {"op": "flip", "state": "kan", "line": 1},
            {"op": "double_flip", "state": "kan", "lines": [0, 2]},
            {"op": "complement", "state": "kan"},
            {"op": "grade", "state": "e12"},
            {"op": "product", "a": "e1", "b": "e2"},
            {"op": "combine", "upper": "qian", "lower": "kun"},
            {"op": "count_blades", "grade_k": 2},
        ],
        "rotor_transition": [
            {"op": "compose", "r1": [1.0, 0.0, 0.0, 0.0],
             "r2": [1.0, 0.0, 0.0, 0.0]},
            {"op": "invert", "r": [1.0, 0.0, 0.0, 0.0]},
            {"op": "apply", "r": [1.0, 0.0, 0.0, 0.0], "blade": "e1"},
            {"op": "distance", "r1": [1.0, 0.0, 0.0, 0.0],
             "r2": [1.0, 0.0, 0.0, 0.0]},
        ],
    }

    for name, operations in representatives.items():
        schema = TOOL_SCHEMAS[name]
        assert "oneOf" in schema["properties"]["ops"]["items"]
        for operation in operations:
            validate({"ops": [operation]}, schema)


@pytest.mark.parametrize(
    ("name", "operation"),
    [
        ("cl3_evaluate", {"op": "product", "a": "e1"}),
        ("cl3_evaluate", {"op": "product", "a": "e1", "b": "e2", "extra": 1}),
        ("rotor_transition", {"op": "invert"}),
        ("rotor_transition", {"op": "invert", "r": [1.0, 0.0, 0.0, 0.0], "extra": 1}),
    ],
)
def test_algebra_schemas_reject_missing_and_extra_operation_fields(name, operation):
    with pytest.raises(ValidationError):
        validate({"ops": [operation]}, TOOL_SCHEMAS[name])


@pytest.mark.parametrize(
    ("name", "arguments"),
    [
        ("coverage_audit", {"task": "t"}),
        ("reframe", {"statement": 42}),
        ("state_diff", {"before": "b", "after": "a", "extra": True}),
        ("cl3_evaluate", {"ops": {}}),
        ("interaction_spectrum", {"points": [[1, 1]], "values": [True]}),
        ("rotor_transition", {"ops": [{"op": "invert"}]}),
    ],
)
def test_invalid_arguments_raise_contract_error(name, arguments):
    with pytest.raises(ContractError):
        validate_tool_arguments(name, arguments)


def test_coverage_result_requires_all_roles():
    result = coverage_result()
    validate_tool_result("coverage_audit", result)

    result["audited_plan"] = "receptive: covered"
    with pytest.raises(ContractError):
        validate_tool_result("coverage_audit", result)


def test_state_diff_result_requires_exactly_three_aspects():
    result = state_diff_result()
    validate_tool_result("state_diff", result)

    result["aspects"].pop()
    with pytest.raises(ContractError):
        validate_tool_result("state_diff", result)


def test_all_six_result_shapes_validate():
    results = {
        "coverage_audit": coverage_result(),
        "reframe": reframe_result(),
        "state_diff": state_diff_result(),
        "cl3_evaluate": {
            "result": "e12",
            "steps": [{"op": "product", "args": {"a": "e1", "b": "e2"}, "result": "e12"}],
        },
        "interaction_spectrum": {
            "spectrum": {0: 1.0, 1: 0.0},
            "identified": [0],
        },
        "rotor_transition": ["e2"],
    }

    for name, result in results.items():
        validate_tool_result(name, result)


@pytest.mark.parametrize(
    ("name", "result"),
    [
        (
            "cl3_evaluate",
            {"result": "e1", "steps": []},
        ),
        (
            "cl3_evaluate",
            {
                "result": "e1",
                "steps": [{"op": "not-an-op", "args": {}, "result": "e1"}],
            },
        ),
        ("rotor_transition", [{"malformed": True}]),
        ("rotor_transition", ["not-a-blade"]),
        ("rotor_transition", [[float("nan"), 0.0, 0.0, 0.0]]),
        (
            "interaction_spectrum",
            {"spectrum": {"0": 1.0, "2": 0.0}, "identified": [0]},
        ),
        (
            "state_diff",
            {
                "before": "b",
                "after": "a",
                "aspects": [
                    {"aspect": "same", "before": "1", "after": "2"},
                    {"aspect": "SAME", "before": "2", "after": "3"},
                    {"aspect": "other", "before": "3", "after": "4"},
                ],
            },
        ),
    ],
)
def test_result_validators_reject_malformed_values_independently(name, result):
    with pytest.raises(ContractError):
        validate_tool_result(name, result)
