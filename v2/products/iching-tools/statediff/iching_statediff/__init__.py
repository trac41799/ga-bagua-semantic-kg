"""iching_statediff -- summarize a state change as exactly 3 aspect lines.

Protocol (validated POC-06 arm B): exactly 3 lines of the form
``aspect: before -> after``, no prose beyond the 3 lines.
"""

import re

ARM_B_PROMPT = (
    "Write a structured summary using EXACTLY 3 aspect lines of the form "
    "'aspect: before -> after'. Cover the three most important changes, one per line. "
    "All three fields on every line must be non-empty and aspect labels must be unique. "
    "No prose beyond the 3 lines."
)

_ASPECT_LINE = re.compile(r"^\s*(.+?)\s*:\s*(.*?)\s*->\s*(.*?)\s*$")


class ProtocolError(Exception):
    """LLM output violates the 3-aspect-line protocol."""


def validate_aspects(aspects):
    """Validate the parsed three-row aspect result and return it unchanged."""
    if not isinstance(aspects, list):
        raise ProtocolError("aspects must be a list")
    if len(aspects) != 3:
        raise ProtocolError(f"expected exactly 3 aspect rows, got {len(aspects)}")

    labels = set()
    for i, row in enumerate(aspects):
        if not isinstance(row, dict):
            raise ProtocolError(f"aspect row {i} must be a dict")
        if set(row) != {"aspect", "before", "after"}:
            raise ProtocolError(
                f"aspect row {i} must contain exactly aspect/before/after fields"
            )
        if any(not isinstance(row[field], str) or not row[field].strip()
               for field in ("aspect", "before", "after")):
            raise ProtocolError(f"aspect row {i} contains an empty field")
        label = row["aspect"].strip().casefold()
        if label in labels:
            raise ProtocolError(f"duplicate aspect label: {row['aspect']!r}")
        labels.add(label)
    return aspects


def parse_aspects(text: str) -> list[dict[str, str]]:
    """Parse exactly three non-empty ``aspect: before -> after`` lines."""
    if not isinstance(text, str):
        raise ProtocolError("LLM output must be text")
    lines = [ln for ln in text.splitlines() if ln.strip()]
    if len(lines) != 3:
        raise ProtocolError(f"expected exactly 3 aspect lines, got {len(lines)}")
    aspects = []
    for ln in lines:
        if ln.count("->") != 1:
            raise ProtocolError(
                f"line does not match 'aspect: before -> after': {ln!r}"
            )
        m = _ASPECT_LINE.match(ln)
        if not m:
            raise ProtocolError(
                f"line does not match 'aspect: before -> after': {ln!r}"
            )
        aspects.append({"aspect": m.group(1).strip(),
                        "before": m.group(2).strip(),
                        "after": m.group(3).strip()})
    return validate_aspects(aspects)


def summarize(before, after, llm):
    """Return {"before", "after", "aspects": [{aspect, before, after} x3]}.

    Raises ProtocolError if the LLM output is not exactly 3 parseable
    'aspect: before -> after' lines.
    """
    messages = [
        {"role": "system", "content": ARM_B_PROMPT},
        {"role": "user", "content": f"Before: {before}\nAfter: {after}"},
    ]
    text, _ = llm.chat(messages, max_tokens=128)
    aspects = parse_aspects(text)
    validate_aspects(aspects)
    return {"before": before, "after": after, "aspects": aspects}
