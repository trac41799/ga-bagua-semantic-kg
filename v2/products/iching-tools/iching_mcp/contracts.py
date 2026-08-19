"""Runtime contracts for the six iching-tools MCP tools.

The server deliberately uses small stdlib validators instead of a runtime
schema dependency. The input schemas are also exposed to MCP clients.
"""

import math
import numbers
import re


class ContractError(ValueError):
    """Raised when an MCP argument or result violates a tool contract."""


McpContractError = ContractError


_CL3_OPS = {
    "flip": ("state", "line"),
    "double_flip": ("state", "lines"),
    "complement": ("state",),
    "grade": ("state",),
    "product": ("a", "b"),
    "combine": ("upper", "lower"),
    "count_blades": ("grade_k",),
}
_ROTOR_OPS = {
    "compose": ("r1", "r2"),
    "invert": ("r",),
    "apply": ("r", "blade"),
    "distance": ("r1", "r2"),
}

_CL3_ALL_STATES = (
    "1", "e1", "e2", "e3", "e12", "e23", "e31", "e123",
    "-1", "-e1", "-e2", "-e3", "-e12", "-e23", "-e31", "-e123",
    "kun", "gen", "kan", "xun", "zhen", "li", "dui", "qian",
    "000", "001", "010", "011", "100", "101", "110", "111",
)
_CL3_TRIGRAM_STATES = (
    "1", "e1", "e2", "e3", "e12", "e23", "e123", "-e31",
    "kun", "gen", "kan", "xun", "zhen", "li", "dui", "qian",
    "000", "001", "010", "011", "100", "101", "110", "111",
)
_ROTOR_BLADES = tuple(
    blade for blade in (
        "1", "e1", "e2", "e3", "e12", "e23", "e31", "e123",
    ) for blade in (blade, f"+{blade}", f"-{blade}")
)


def _object_schema(properties, required):
    return {
        "type": "object",
        "properties": properties,
        "required": list(required),
        "additionalProperties": False,
    }


def _operation_schema(name, properties, required, description):
    return {
        "type": "object",
        "description": description,
        "properties": {
            "op": {"type": "string", "enum": [name]},
            **properties,
        },
        "required": ["op", *required],
        "additionalProperties": False,
    }


def _rotor_schema(description):
    return {
        "type": "array",
        "description": description,
        "minItems": 4,
        "maxItems": 4,
        "items": {"type": "number"},
    }


TOOL_SCHEMAS = {
    "coverage_audit": _object_schema(
        {
            "task": {"type": "string", "description": "The task the plan addresses"},
            "plan": {"type": "string", "description": "The draft plan to audit"},
        },
        ("task", "plan"),
    ),
    "reframe": _object_schema(
        {"statement": {"type": "string", "description": "The statement to reframe"}},
        ("statement",),
    ),
    "state_diff": _object_schema(
        {
            "before": {"type": "string", "description": "The prior state"},
            "after": {"type": "string", "description": "The new state"},
        },
        ("before", "after"),
    ),
    "cl3_evaluate": _object_schema(
        {
            "ops": {
                "type": "array",
                "minItems": 1,
                "description": "A non-empty list of exact Cl(3) operations",
                "items": {
                    "oneOf": [
                        _operation_schema(
                            "flip",
                            {
                                "state": {
                                    "type": "string",
                                    "enum": list(_CL3_TRIGRAM_STATES),
                                },
                                "line": {
                                    "type": "integer",
                                    "minimum": 0,
                                    "maximum": 2,
                                },
                            },
                            ("state", "line"),
                            "Flip one line of a trigram state.",
                        ),
                        _operation_schema(
                            "double_flip",
                            {
                                "state": {
                                    "type": "string",
                                    "enum": list(_CL3_TRIGRAM_STATES),
                                },
                                "lines": {
                                    "type": "array",
                                    "items": {
                                        "type": "integer",
                                        "minimum": 0,
                                        "maximum": 2,
                                    },
                                },
                            },
                            ("state", "lines"),
                            "Flip several lines in order.",
                        ),
                        _operation_schema(
                            "complement",
                            {
                                "state": {
                                    "type": "string",
                                    "enum": list(_CL3_ALL_STATES),
                                },
                            },
                            ("state",),
                            "Take the Hodge-dual complement of a state.",
                        ),
                        _operation_schema(
                            "grade",
                            {
                                "state": {
                                    "type": "string",
                                    "enum": list(_CL3_ALL_STATES),
                                },
                            },
                            ("state",),
                            "Return the blade grade of a state.",
                        ),
                        _operation_schema(
                            "product",
                            {
                                "a": {
                                    "type": "string",
                                    "enum": list(_CL3_ALL_STATES),
                                },
                                "b": {
                                    "type": "string",
                                    "enum": list(_CL3_ALL_STATES),
                                },
                            },
                            ("a", "b"),
                            "Compute the geometric product of two states.",
                        ),
                        _operation_schema(
                            "combine",
                            {
                                "upper": {
                                    "type": "string",
                                    "enum": list(_CL3_ALL_STATES),
                                },
                                "lower": {
                                    "type": "string",
                                    "enum": list(_CL3_ALL_STATES),
                                },
                            },
                            ("upper", "lower"),
                            "Combine two trigram states into a hexagram code.",
                        ),
                        _operation_schema(
                            "count_blades",
                            {
                                "grade_k": {
                                    "type": "integer",
                                    "minimum": 0,
                                    "maximum": 3,
                                },
                            },
                            ("grade_k",),
                            "Count blades at a given grade.",
                        ),
                    ]
                },
            }
        },
        ("ops",),
    ),
    "interaction_spectrum": _object_schema(
        {
            "points": {
                "type": "array",
                "minItems": 1,
                "description": "Design points made of numeric +1/-1 levels",
                "items": {
                    "type": "array",
                    "minItems": 1,
                    "items": {"type": "number"},
                },
            },
            "values": {
                "type": "array",
                "minItems": 1,
                "description": "One numeric response per design point",
                "items": {"type": "number"},
            },
        },
        ("points", "values"),
    ),
    "rotor_transition": _object_schema(
        {
            "ops": {
                "type": "array",
                "description": "A list of exact rotor operations",
                "items": {
                    "oneOf": [
                        _operation_schema(
                            "compose",
                            {
                                "r1": _rotor_schema("The first unit rotor."),
                                "r2": _rotor_schema("The second unit rotor."),
                            },
                            ("r1", "r2"),
                            "Compose two unit rotors.",
                        ),
                        _operation_schema(
                            "invert",
                            {"r": _rotor_schema("The unit rotor to invert.")},
                            ("r",),
                            "Invert a unit rotor.",
                        ),
                        _operation_schema(
                            "apply",
                            {
                                "r": _rotor_schema("The unit rotor to apply."),
                                "blade": {
                                    "type": "string",
                                    "enum": list(_ROTOR_BLADES),
                                },
                            },
                            ("r", "blade"),
                            "Apply a rotor to a canonical blade.",
                        ),
                        _operation_schema(
                            "distance",
                            {
                                "r1": _rotor_schema("The first unit rotor."),
                                "r2": _rotor_schema("The second unit rotor."),
                            },
                            ("r1", "r2"),
                            "Compute the Euclidean distance between rotors.",
                        ),
                    ]
                },
            }
        },
        ("ops",),
    ),
}

TOOL_DESCRIPTIONS = {
    "coverage_audit": (
        "Audit and complete a plan against the eight-role completeness "
        "checklist."
    ),
    "reframe": (
        "Generate exactly eight algebra-grounded reframes: origin, three "
        "single-line flips, three double-flips, and a complement."
    ),
    "state_diff": "Summarize a state change as exactly three aspect lines.",
    "cl3_evaluate": "Evaluate a batch of exact Cl(3) blade-algebra operations.",
    "interaction_spectrum": "Recover the exact Walsh-Hadamard interaction spectrum.",
    "rotor_transition": "Evaluate exact rotor compose, invert, apply, and distance operations.",
}

TOOL_NAMES = tuple(TOOL_SCHEMAS)
INPUT_SCHEMAS = TOOL_SCHEMAS


def _is_number(value):
    if not isinstance(value, numbers.Real) or isinstance(value, bool):
        return False
    try:
        return math.isfinite(float(value))
    except (OverflowError, ValueError):
        return False


def _is_int(value):
    return isinstance(value, numbers.Integral) and not isinstance(value, bool)


def _require_object(arguments, name):
    if not isinstance(name, str) or name not in TOOL_SCHEMAS:
        raise ContractError(f"unknown tool: {name}")
    if not isinstance(arguments, dict):
        raise ContractError(f"{name}: arguments must be an object")
    schema = TOOL_SCHEMAS[name]
    expected = set(schema["properties"])
    actual = set(arguments)
    missing = expected - actual
    extra = actual - expected
    if missing:
        raise ContractError(f"{name}: missing argument(s): {', '.join(sorted(missing))}")
    if extra:
        raise ContractError(f"{name}: unknown argument(s): {', '.join(sorted(extra))}")


def _require_string(value, path):
    if not isinstance(value, str):
        raise ContractError(f"{path} must be a string")


def _require_allowed(value, path, allowed):
    _require_string(value, path)
    if value not in allowed:
        raise ContractError(f"{path} has an unknown value")


def _validate_cl3_ops(ops):
    if not isinstance(ops, list) or not ops:
        raise ContractError("cl3_evaluate.ops must be a non-empty list")
    for index, operation in enumerate(ops):
        path = f"cl3_evaluate.ops[{index}]"
        if not isinstance(operation, dict):
            raise ContractError(f"{path} must be an object")
        name = operation.get("op")
        if not isinstance(name, str) or name not in _CL3_OPS:
            raise ContractError(f"{path}.op is unknown")
        expected = {"op", *_CL3_OPS[name]}
        missing = expected - set(operation)
        extra = set(operation) - expected
        if missing or extra:
            if missing:
                raise ContractError(f"{path} is missing required arguments")
            raise ContractError(f"{path} has unknown arguments")
        if name in {"flip", "double_flip"}:
            _require_allowed(operation["state"], f"{path}.state", _CL3_TRIGRAM_STATES)
        elif name in {"complement", "grade"}:
            _require_allowed(operation["state"], f"{path}.state", _CL3_ALL_STATES)
        if name == "flip" and (not _is_int(operation["line"]) or not 0 <= operation["line"] <= 2):
            raise ContractError(f"{path}.line must be an integer from 0 to 2")
        if name == "double_flip":
            lines = operation["lines"]
            if not isinstance(lines, list) or not all(
                _is_int(line) and 0 <= line <= 2 for line in lines
            ):
                raise ContractError(f"{path}.lines must be a list of integers from 0 to 2")
        if name == "product":
            _require_allowed(operation["a"], f"{path}.a", _CL3_ALL_STATES)
            _require_allowed(operation["b"], f"{path}.b", _CL3_ALL_STATES)
        if name == "combine":
            _require_allowed(operation["upper"], f"{path}.upper", _CL3_ALL_STATES)
            _require_allowed(operation["lower"], f"{path}.lower", _CL3_ALL_STATES)
        if name == "count_blades" and (
            not _is_int(operation["grade_k"]) or not 0 <= operation["grade_k"] <= 3
        ):
            raise ContractError(f"{path}.grade_k must be an integer from 0 to 3")


def _validate_points(points, values):
    if not isinstance(points, list) or not points:
        raise ContractError("interaction_spectrum.points must be a non-empty list")
    width = None
    for index, point in enumerate(points):
        if not isinstance(point, list) or not point:
            raise ContractError(f"interaction_spectrum.points[{index}] must be a non-empty list")
        if width is None:
            width = len(point)
        elif len(point) != width:
            raise ContractError("interaction_spectrum.points must have equal-width vectors")
        if any(not _is_number(value) or abs(float(value)) != 1.0 for value in point):
            raise ContractError("interaction_spectrum.points entries must be numeric +1 or -1")
    if not isinstance(values, list) or len(values) != len(points):
        raise ContractError("interaction_spectrum.values must match points length")
    if any(not _is_number(value) for value in values):
        raise ContractError("interaction_spectrum.values entries must be finite numbers")


def _validate_rotor(value, path):
    if not isinstance(value, list) or len(value) != 4:
        raise ContractError(f"{path} must be a list of four numbers")
    if any(not _is_number(component) for component in value):
        raise ContractError(f"{path} must contain only finite numbers")
    norm = math.sqrt(sum(float(component) ** 2 for component in value))
    if abs(norm - 1.0) > 1e-9:
        raise ContractError(f"{path} must be a unit rotor")


def _validate_rotor_ops(ops):
    if not isinstance(ops, list):
        raise ContractError("rotor_transition.ops must be a list")
    for index, operation in enumerate(ops):
        path = f"rotor_transition.ops[{index}]"
        if not isinstance(operation, dict):
            raise ContractError(f"{path} must be an object")
        name = operation.get("op")
        if not isinstance(name, str) or name not in _ROTOR_OPS:
            raise ContractError(f"{path}.op is unknown")
        expected = {"op", *_ROTOR_OPS[name]}
        if set(operation) != expected:
            raise ContractError(f"{path} has missing or unknown arguments")
        for key in _ROTOR_OPS[name]:
            if key.startswith("r"):
                _validate_rotor(operation[key], f"{path}.{key}")
            else:
                _require_allowed(operation[key], f"{path}.{key}", _ROTOR_BLADES)


def validate_tool_arguments(name, arguments):
    """Validate a tool's complete JSON argument object before dispatch."""
    if not isinstance(name, str) or name not in TOOL_SCHEMAS:
        raise ContractError(f"unknown tool: {name}")
    _require_object(arguments, name)
    if name in {"coverage_audit", "reframe", "state_diff"}:
        for key in TOOL_SCHEMAS[name]["required"]:
            _require_string(arguments[key], f"{name}.{key}")
    elif name == "cl3_evaluate":
        _validate_cl3_ops(arguments["ops"])
    elif name == "interaction_spectrum":
        _validate_points(arguments["points"], arguments["values"])
    elif name == "rotor_transition":
        _validate_rotor_ops(arguments["ops"])


def _require_result_object(name, result, keys):
    if not isinstance(result, dict) or set(result) != set(keys):
        raise ContractError(f"{name} result has the wrong object shape")


def _validate_coverage_result(result):
    _require_result_object(
        "coverage_audit", result, ("task", "original_plan", "audited_plan", "checklist")
    )
    for key in ("task", "original_plan", "audited_plan"):
        _require_string(result[key], f"coverage_audit.{key}")
    if result["checklist"] is not True:
        raise ContractError("coverage_audit.checklist must be true")
    _validate_coverage_result_compat(result["audited_plan"])


def _validate_coverage_result_compat(audited_plan):
    roles = {
        "receptive",
        "causal",
        "transmissive",
        "constraining",
        "clarifying",
        "influential",
        "balancing",
        "generative",
    }
    if not isinstance(audited_plan, str) or not audited_plan.strip():
        raise ContractError("coverage_audit.audited_plan must be non-empty")
    marker = re.compile(
        r"^[ \t]*(?:#{1,6}[ \t]+)?(?:\d+[.)][ \t]+)?"
        r"(?P<role>receptive|causal|transmissive|constraining|clarifying|"
        r"influential|balancing|generative)[ \t]*:"
    )
    found = [match.group("role") for line in audited_plan.splitlines()
             if (match := marker.match(line)) is not None]
    if any(found.count(role) != 1 for role in roles):
        raise ContractError("coverage_audit.audited_plan must contain all eight role markers")


def _validate_reframe_result(result):
    _require_result_object("reframe", result, ("statement", "positions"))
    _require_string(result["statement"], "reframe.statement")
    positions = result["positions"]
    expected_moves = [
        "origin",
        "flip0",
        "flip1",
        "flip2",
        "double_flip01",
        "double_flip02",
        "double_flip12",
        "complement",
    ]
    if not isinstance(positions, list) or len(positions) != len(expected_moves):
        raise ContractError("reframe.positions must contain exactly eight positions")
    states = set()
    for expected, position in zip(expected_moves, positions):
        if not isinstance(position, dict) or set(position) != {
            "move", "state", "reframe", "description"
        }:
            raise ContractError("reframe position has the wrong shape")
        if position["move"] != expected:
            raise ContractError("reframe positions are out of order")
        for key in ("move", "state", "reframe", "description"):
            _require_string(position[key], f"reframe.positions.{key}")
            if not position[key].strip():
                raise ContractError(f"reframe.positions.{key} must be non-empty")
        if position["state"] in states:
            raise ContractError("reframe positions must have distinct states")
        states.add(position["state"])


def _validate_state_diff_result(result):
    _require_result_object("state_diff", result, ("before", "after", "aspects"))
    _require_string(result["before"], "state_diff.before")
    _require_string(result["after"], "state_diff.after")
    aspects = result["aspects"]
    if not isinstance(aspects, list) or len(aspects) != 3:
        raise ContractError("state_diff.aspects must contain exactly three aspects")
    labels = set()
    for aspect in aspects:
        if not isinstance(aspect, dict) or set(aspect) != {"aspect", "before", "after"}:
            raise ContractError("state_diff aspect has the wrong shape")
        for key in ("aspect", "before", "after"):
            _require_string(aspect[key], f"state_diff.aspects.{key}")
            if not aspect[key].strip():
                raise ContractError(f"state_diff.aspects.{key} must be non-empty")
        label = aspect["aspect"].strip().casefold()
        if label in labels:
            raise ContractError("state_diff aspect labels must be unique")
        labels.add(label)


def _is_json_value(value):
    if value is None or isinstance(value, (str, bool, int)):
        return True
    if isinstance(value, float):
        return math.isfinite(value)
    if isinstance(value, (list, tuple)):
        return all(_is_json_value(item) for item in value)
    if isinstance(value, dict):
        return all(isinstance(key, (str, int)) and _is_json_value(item) for key, item in value.items())
    return False


_CL3_RESULT_BLADE_NAMES = {
    name for name in (
        "1", "e1", "e2", "e3", "e12", "e23", "e31", "e123",
    ) for name in (name, f"-{name}")
}


def _validate_cl3_state_result(value, path):
    if not isinstance(value, (list, tuple)) or len(value) != 2:
        raise ContractError(f"{path} must be a blade index/sign pair")
    if not _is_int(value[0]) or not 0 <= value[0] <= 7:
        raise ContractError(f"{path} has an invalid blade index")
    if not _is_int(value[1]) or value[1] not in (-1, 1):
        raise ContractError(f"{path} has an invalid blade sign")


def _validate_cl3_step_result(op, args, value, path):
    if op in {"flip", "double_flip", "complement"}:
        _validate_cl3_state_result(value, path)
    elif op == "grade":
        if not _is_int(value) or not 0 <= value <= 3:
            raise ContractError(f"{path} must be a blade grade")
    elif op == "product":
        if not isinstance(value, str) or value not in _CL3_RESULT_BLADE_NAMES:
            raise ContractError(f"{path} must be a canonical blade name")
    elif op == "combine":
        if not _is_int(value) or not 0 <= value <= 63:
            raise ContractError(f"{path} must be a hexagram code")
    elif op == "count_blades":
        expected = {0: 1, 1: 3, 2: 3, 3: 1}[args["grade_k"]]
        if not _is_int(value) or value != expected:
            raise ContractError(f"{path} does not match count_blades")


def _normalized_json_value(value):
    if isinstance(value, tuple):
        return [_normalized_json_value(item) for item in value]
    if isinstance(value, list):
        return [_normalized_json_value(item) for item in value]
    if isinstance(value, dict):
        return {key: _normalized_json_value(item) for key, item in value.items()}
    return value


def _validate_cl3_result(result):
    _require_result_object("cl3_evaluate", result, ("result", "steps"))
    if not isinstance(result["steps"], list) or not result["steps"]:
        raise ContractError("cl3_evaluate.steps must be a non-empty list")
    for step in result["steps"]:
        if not isinstance(step, dict) or set(step) != {"op", "args", "result"}:
            raise ContractError("cl3_evaluate step has the wrong shape")
        if not isinstance(step["op"], str) or not isinstance(step["args"], dict):
            raise ContractError("cl3_evaluate step has invalid values")
        if "op" in step["args"]:
            raise ContractError("cl3_evaluate step args must not contain op")
        operation = {"op": step["op"], **step["args"]}
        try:
            _validate_cl3_ops([operation])
        except (ContractError, TypeError) as exc:
            raise ContractError(f"cl3_evaluate step is invalid: {exc}") from None
        _validate_cl3_step_result(
            step["op"], step["args"], step["result"], "cl3_evaluate.step.result"
        )
    if _normalized_json_value(result["result"]) != _normalized_json_value(
        result["steps"][-1]["result"]
    ):
        raise ContractError("cl3_evaluate.result must equal the final step result")


def _validate_interaction_result(result):
    _require_result_object("interaction_spectrum", result, ("spectrum", "identified"))
    if not isinstance(result["spectrum"], dict) or not isinstance(result["identified"], list):
        raise ContractError("interaction_spectrum result has invalid values")
    if not result["spectrum"]:
        raise ContractError("interaction_spectrum spectrum must not be empty")
    masks = set()
    for mask, coefficient in result["spectrum"].items():
        json_mask = isinstance(mask, str) and mask.isdigit()
        if json_mask and str(int(mask)) != mask:
            json_mask = False
        if not ((_is_int(mask) and mask >= 0) or json_mask) or not _is_number(coefficient):
            raise ContractError("interaction_spectrum spectrum has invalid values")
        masks.add(int(mask))
    size = len(masks)
    if size < 2 or size & (size - 1) or masks != set(range(size)):
        raise ContractError("interaction_spectrum spectrum masks must be contiguous")
    identified = result["identified"]
    if (not all(_is_int(mask) and mask >= 0 for mask in identified)
            or identified != sorted(set(identified))
            or any(mask not in masks for mask in identified)):
        raise ContractError("interaction_spectrum identified has invalid values")
    expected = sorted(
        int(mask) for mask, coefficient in result["spectrum"].items()
        if abs(float(coefficient)) > 1e-6
    )
    if expected != identified:
        raise ContractError("interaction_spectrum identified masks are inconsistent")


_ROTOR_RESULT_BLADES = {
    name for name in (
        "1", "e1", "e2", "e3", "e12", "e23", "e31", "e123",
    ) for name in (name, f"-{name}")
}


def _validate_rotor_output(value, path):
    if isinstance(value, str):
        if value not in _ROTOR_RESULT_BLADES:
            raise ContractError(f"{path} is not a canonical blade result")
        return
    if _is_number(value):
        if float(value) < 0.0:
            raise ContractError(f"{path} distance must be non-negative")
        return
    if isinstance(value, (list, tuple)) and len(value) == 4:
        if any(not _is_number(component) for component in value):
            raise ContractError(f"{path} rotor has invalid components")
        norm = math.sqrt(sum(float(component) ** 2 for component in value))
        if abs(norm - 1.0) > 1e-9:
            raise ContractError(f"{path} rotor must be unit length")
        return
    raise ContractError(f"{path} has an invalid rotor operation result")


def _validate_rotor_result(result):
    if not isinstance(result, list):
        raise ContractError("rotor_transition result must be a list")
    for index, value in enumerate(result):
        _validate_rotor_output(value, f"rotor_transition[{index}]")


_RESULT_VALIDATORS = {
    "coverage_audit": _validate_coverage_result,
    "reframe": _validate_reframe_result,
    "state_diff": _validate_state_diff_result,
    "cl3_evaluate": _validate_cl3_result,
    "interaction_spectrum": _validate_interaction_result,
    "rotor_transition": _validate_rotor_result,
}
RESULT_VALIDATORS = _RESULT_VALIDATORS


def validate_coverage_result(result):
    return _validate_coverage_result(result)


def validate_reframe_result(result):
    return _validate_reframe_result(result)


def validate_state_diff_result(result):
    return _validate_state_diff_result(result)


def validate_cl3_result(result):
    return _validate_cl3_result(result)


def validate_interaction_spectrum_result(result):
    return _validate_interaction_result(result)


def validate_rotor_transition_result(result):
    return _validate_rotor_result(result)


def validate_tool_result(name, result):
    """Validate a successful tool result before it is serialized to JSON."""
    if not isinstance(name, str):
        raise ContractError(f"unknown tool: {name}")
    try:
        validator = _RESULT_VALIDATORS[name]
    except KeyError as exc:
        raise ContractError(f"unknown tool: {name}") from exc
    validator(result)


validate_arguments = validate_tool_arguments
validate_result = validate_tool_result


__all__ = [
    "ContractError",
    "INPUT_SCHEMAS",
    "McpContractError",
    "RESULT_VALIDATORS",
    "TOOL_DESCRIPTIONS",
    "TOOL_NAMES",
    "TOOL_SCHEMAS",
    "validate_arguments",
    "validate_cl3_result",
    "validate_coverage_result",
    "validate_interaction_spectrum_result",
    "validate_reframe_result",
    "validate_result",
    "validate_rotor_transition_result",
    "validate_state_diff_result",
    "validate_tool_arguments",
    "validate_tool_result",
]
