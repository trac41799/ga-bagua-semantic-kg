"""Tag protocol: 8 Bagua roles (fixed order), prompt, strict parser, dominant role.

Roles are TAGS on top of retrieval. They are never the embedding or retrieval
vector (see noninterf.py). Strength semantics: positive = the concept embodies
the role; negative = it actively suppresses/opposes the role; 0 = neutral.
"""

import json

ROLES = [
    "receptive",
    "causal",
    "transmissive",
    "constraining",
    "clarifying",
    "influential",
    "balancing",
    "generative",
]

ROLE_GLOSS = {
    "receptive": "absorbs, contains, stores, or hosts inputs without acting on them",
    "causal": "triggers, initiates, or sets change in motion",
    "transmissive": "moves, relays, carries, or delivers things between parties",
    "constraining": "restricts, limits, bounds, or enforces rules/thresholds",
    "clarifying": "reveals, measures, verifies, or removes ambiguity",
    "influential": "shapes behavior or outcomes through influence, not direct control",
    "balancing": "equalizes, distributes, maintains equilibrium, or compensates",
    "generative": "transforms inputs into new outputs, creates, or produces",
}


class TagError(ValueError):
    """Typed error for any deviation from the strict tag contract."""


def TAG_PROMPT(description):
    """Return the message list (system + user) asking the LLM for role tags.

    The LLM must reply with ONLY a JSON object of the 8 roles -> strength in
    [-1, 1]. Output is validated downstream by parse_tags (strict).
    """
    schema = json.dumps({r: 0.0 for r in ROLES}, separators=(",", ":"))
    gloss = "\n".join(f"- {r}: {ROLE_GLOSS[r]}" for r in ROLES)
    system = (
        "You assign interpretable role tags to a concept description using exactly 8 fixed roles.\n"
        "Role meanings:\n"
        f"{gloss}\n"
        "Strength semantics: positive = the concept embodies the role; "
        "negative = it actively suppresses or opposes the role; 0 = neutral.\n"
        "Reply with ONLY a compact JSON object with exactly these 8 keys, "
        "one numeric value in [-1, 1] (up to 2 decimals) per key, no prose:\n"
        f"{schema}"
    )
    return [
        {"role": "system", "content": system},
        {"role": "user", "content": f"Concept description: {description}"},
    ]


def parse_tags(text):
    """Strictly parse an LLM tag response into {role: float} in [-1, 1].

    Raises TagError on any deviation: non-JSON, non-object, missing role,
    extra key, non-numeric value, or value outside [-1, 1].
    """
    if not isinstance(text, str):
        raise TagError(f"expected str, got {type(text).__name__}")
    s = text.strip()
    if s.startswith("```"):
        lines = s.splitlines()
        if lines and lines[0].strip().startswith("```"):
            lines = lines[1:]
        if lines and lines[-1].strip().startswith("```"):
            lines = lines[:-1]
        s = "\n".join(lines).strip()
    try:
        obj = json.loads(s)
    except json.JSONDecodeError as e:
        raise TagError(f"invalid JSON: {e.msg}") from e
    if not isinstance(obj, dict):
        raise TagError(f"expected JSON object, got {type(obj).__name__}")
    missing = [r for r in ROLES if r not in obj]
    if missing:
        raise TagError(f"missing roles: {missing}")
    extra = [k for k in obj if k not in ROLES]
    if extra:
        raise TagError(f"unexpected keys: {extra}")
    out = {}
    for r in ROLES:
        v = obj[r]
        if isinstance(v, bool) or not isinstance(v, (int, float)):
            raise TagError(f"role {r!r}: value {v!r} is not numeric")
        v = float(v)
        if not (-1.0 <= v <= 1.0):
            raise TagError(f"role {r!r}: value {v} out of range [-1, 1]")
        out[r] = v
    return out


def dominant_role(tags):
    """Return the role with the highest strength; ties resolve to fixed ROLE order."""
    if not isinstance(tags, dict):
        raise TagError(f"expected dict, got {type(tags).__name__}")
    missing = [r for r in ROLES if r not in tags]
    if missing:
        raise TagError(f"missing roles: {missing}")
    return max(ROLES, key=lambda r: tags[r])
