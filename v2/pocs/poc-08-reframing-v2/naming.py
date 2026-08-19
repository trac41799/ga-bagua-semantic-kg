"""POC-08: few-shot naming protocol (v2 of POC-03's zero-shot naming).

2 exemplars per move type so the LLM learns the naming register; the 8 moves
themselves are unchanged exact operators (moves.py, POC-03-verified).
"""

EXEMPLARS = [
    # (move_name, statement, position_state_name, exemplar_name)
    ("flip", "We should make the checkout faster.", "Zhen", "Flipping the initiating line: instead of making checkout faster, trigger the change by reordering how steps start."),
    ("flip", "The team should adopt code reviews.", "Kan", "Flipping the flow line: code reviews as the channel through which knowledge flows, not as a gate."),
    ("double_flip", "We should cut the marketing budget.", "Li", "Flipping two lines: cutting spend while illuminating what it buys — the reframe is about transparency, not austerity."),
    ("double_flip", "Add an offline mode.", "Dui", "Flipping two lines: offline mode as a mirror of the online experience, not a stripped copy."),
    ("complement", "We should raise prices.", "Qian", "Complement (antipode): the reframe is lowering friction instead of raising prices — the opposite position on the cube."),
    ("complement", "We should hire more engineers.", "Kun", "Complement (antipode): the reframe is removing work rather than adding capacity — receiving the opposite pole."),
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


def naming_prompt(statement, move_name, state_name):
    ex = MOVE_EXEMPLARS.get(move_name, [])
    ex_lines = "\n".join(
        f"  statement: {st} | position: {pos} -> reframe: {name}" for st, pos, name in ex)
    return [
        {"role": "system", "content": "You rename a position in a structured reframing system. "
                                      "The position was reached by an exact algebraic move on the "
                                      "statement's cube. Produce ONE reframe sentence (1 line, no "
                                      "explanation) that expresses the statement from that position."},
        {"role": "user", "content": f"Move type: {MOVE_DESC[move_name]}\n"
                                    f"Examples:\n{ex_lines}\n"
                                    f"Statement: {statement}\nPosition state: {state_name}\nReframe:"},
    ]


def arm_a_prompt(statement):
    return [{"role": "system", "content": "Give 8 alternative framings of the statement. "
                                          "Output ONLY a JSON array of 8 strings."},
            {"role": "user", "content": statement}]


def parse_8(text):
    import json
    t = text.strip().strip("`").strip()
    if t.startswith("json"):
        t = t[4:].strip()
    data = json.loads(t)
    if not isinstance(data, list) or len(data) != 8:
        raise ValueError(f"expected 8 reframes, got {len(data) if isinstance(data, list) else 'non-list'}")
    return [str(x) for x in data]


def judge_prompt(statement, reframes):
    joined = "\n".join(f"{i}. {r}" for i, r in enumerate(reframes))
    return [{"role": "system", "content": "Rate COHERENCE of this reframe set: how consistently "
                                          "each reframe stays on the statement's topic, 1-5. "
                                          "Output ONLY the integer."},
            {"role": "user", "content": f"Statement: {statement}\nReframes:\n{joined}"}]
