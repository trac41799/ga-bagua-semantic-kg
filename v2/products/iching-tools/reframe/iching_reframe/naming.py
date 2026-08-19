"""Few-shot naming protocol (adapted from validated POC-08 naming.py).

Self-contained copy: the 8 moves themselves are exact operators (moves.py);
this module supplies the naming register -- >=2 exemplars per move type so the
LLM learns the naming register. Production version: the statement is runtime
input, and move names have their trailing digits stripped before prompt lookup
("flip0" -> "flip", "double_flip01" -> "double_flip").
"""

import re

EXEMPLARS = [
    # (move_type, statement, position_state, exemplar_name)
    ("flip", "We should make the checkout faster.", "Zhen", "Flipping the initiating line: instead of making checkout faster, trigger the change by reordering how steps start."),
    ("flip", "The team should adopt code reviews.", "Kan", "Flipping the flow line: code reviews as the channel through which knowledge flows, not as a gate."),
    ("double_flip", "We should cut the marketing budget.", "Li", "Flipping two lines: cutting spend while illuminating what it buys — the reframe is about transparency, not austerity."),
    ("double_flip", "Add an offline mode.", "Dui", "Flipping two lines: offline mode as a mirror of the online experience, not a stripped copy."),
    ("complement", "We should raise prices.", "Qian", "Complement (antipode): the reframe is lowering friction instead of raising prices — the opposite position on the cube."),
    ("complement", "We should hire more engineers.", "Kun", "Complement (antipode): the reframe is removing work rather than adding capacity — receiving the opposite pole."),
    ("origin", "We should add more tests.", "Kun", "Origin: the starting position holds — the reframe is a restatement of the same intent, no move applied."),
    ("origin", "We should launch the feature now.", "Kun", "Origin: reaffirm the statement from its own starting position — the framing stands as it is."),
]

MOVE_EXEMPLARS = {}
for move, st, pos, name in EXEMPLARS:
    MOVE_EXEMPLARS.setdefault(move, []).append((st, pos, name))

MOVE_DESC = {
    "flip": "flip (change ONE aspect)",
    "double_flip": "double_flip (change TWO aspects)",
    "complement": "complement (stand at the opposite pole)",
    "origin": "origin (the starting position itself)",
}


def move_type(move_name):
    """Strip trailing digits: 'flip0' -> 'flip', 'double_flip01' -> 'double_flip'."""
    return re.sub(r"\d+$", "", move_name)


def naming_prompt(statement, move_name, state_name):
    kind = move_type(move_name)
    ex = MOVE_EXEMPLARS.get(kind, [])
    ex_lines = "\n".join(
        f"  statement: {st} | position: {pos} -> reframe: {name}" for st, pos, name in ex)
    return [
        {"role": "system", "content": "You rename a position in a structured reframing system. "
                                      "The position was reached by an exact algebraic move on the "
                                      "statement's cube. Produce ONE reframe sentence (1 line, no "
                                      "explanation) that expresses the statement from that position."},
        {"role": "user", "content": f"Move type: {MOVE_DESC[kind]}\n"
                                    f"Examples:\n{ex_lines}\n"
                                    f"Statement: {statement}\nPosition state: {state_name}\nReframe:"},
    ]
