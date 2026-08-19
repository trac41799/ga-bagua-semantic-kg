"""Rubric encoding: SKILL.md protocol prompt + strict JSON parsing (Path D)."""

import json
import re

import numpy as np

ROLES = [
    ("receptive", "Accepts, follows, grounds; adopts conventions"),
    ("causal", "Triggers, starts a chain reaction; event-driven"),
    ("transmissive", "Channels, flows, transmits; data pipelines"),
    ("constraining", "Limits, bounds, restricts; permissions, capacity"),
    ("clarifying", "Reveals, illuminates, makes visible; introspection"),
    ("influential", "Pervades, gradually affects; convention spreading"),
    ("balancing", "Mirrors, equilibrates, reflects; feedback loops"),
    ("generative", "Introduces, creates, initiates new patterns"),
]

SYSTEM_PROMPT = (
    "You encode concepts into an 8-element vector using the Bagua role rubric.\n"
    "The 8 roles, in fixed order, are:\n"
    + "\n".join(f"{i}. {name}: {desc}" for i, (name, desc) in enumerate(ROLES)) +
    "\nFor each role ask: does this concept exhibit or counter this quality? "
    "Assign a weight in [-1.0, 1.0] per role (magnitude: >0.5 strong, 0.2-0.5 moderate, "
    "0.05-0.2 slight, -0.05..0.05 irrelevant, negative = counters).\n"
    "Output ONLY a JSON array of exactly 8 floats, no prose, no markdown."
)


def encode_prompt(description: str) -> list:
    return [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": f"Concept description: {description}\nOutput the JSON array."},
    ]


class RubricError(ValueError):
    pass


def parse_encoding(text: str) -> np.ndarray:
    """Parse LLM output into a normalized 8-float array. Raises RubricError on any deviation."""
    t = text.strip()
    t = re.sub(r"^```(json)?\s*", "", t)
    t = re.sub(r"\s*```$", "", t)
    try:
        data = json.loads(t)
    except json.JSONDecodeError as e:
        raise RubricError(f"not valid JSON: {e}") from e
    if not isinstance(data, list):
        raise RubricError(f"expected JSON array, got {type(data).__name__}")
    if len(data) != 8:
        raise RubricError(f"expected 8 coefficients, got {len(data)}")
    out = np.zeros(8)
    for i, v in enumerate(data):
        if not isinstance(v, (int, float)) or isinstance(v, bool):
            raise RubricError(f"coefficient {i} is not numeric: {v!r}")
        out[i] = float(v)
    n = np.linalg.norm(out)
    if n < 1e-12:
        raise RubricError("zero vector")
    return out / n


def parse_id_list(text: str) -> list:
    """Parse LLM verifier output: JSON array of concept ids (or names -> ids via caller)."""
    t = re.sub(r"^```(json)?\s*", "", text.strip())
    t = re.sub(r"\s*```$", "", t)
    try:
        data = json.loads(t)
    except json.JSONDecodeError as e:
        raise RubricError(f"verifier output not JSON: {e}") from e
    if not isinstance(data, list):
        raise RubricError("verifier output not a list")
    return data
