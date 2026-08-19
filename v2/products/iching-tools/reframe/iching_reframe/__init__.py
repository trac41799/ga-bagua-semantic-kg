"""iching_reframe: exactly 8 algebra-grounded reframes of a statement.

The 8 positions are the exact cube moves -- origin, 3 single-line flips,
3 double-line flips and the complement (Hodge dual). State labels come from
describe_state; reframes come from the validated few-shot naming protocol.
"""

from .moves import MOVE_DESCRIPTIONS, START_STATE, all_positions, describe_state
from .naming import naming_prompt


def _base_move(move):
    return move.rstrip("0123456789")


def reframe(statement, llm):
    """Return {"statement": ..., "positions": [{"move","state","reframe","description"} x8]}.

    statement is runtime input; llm must expose chat(messages) -> str.
    Exactly 8 distinct positions, ordered origin, flip0..2, double_flip01..12,
    complement. Each position carries a human-readable description of the
    exact algebraic move (flagship explainability).
    """
    positions = []
    for move, state in all_positions(START_STATE):
        state_label = describe_state(state)
        text = llm.chat(naming_prompt(statement, move, state_label))
        positions.append({
            "move": move,
            "state": state_label,
            "reframe": str(text).strip(),
            "description": MOVE_DESCRIPTIONS.get(move) or MOVE_DESCRIPTIONS.get(_base_move(move), ""),
        })
    return {"statement": statement, "positions": positions}


__all__ = ["reframe"]
