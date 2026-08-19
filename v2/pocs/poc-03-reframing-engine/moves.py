"""Exact cube operators over trigram states.

A trigram state is a pair (blade_index, sign):

  * blade_index is the 3-bit line mask of the trigram (bit 0 = bottom line,
    bit 1 = middle line, bit 2 = top line), matched to the repository's
    established convention (Path C tool, v2/probes/path-c/tests/test_tool.py):

        blade 0 (000) = Kun    (scalar "1",  grade 0)
        blade 1 (001) = Zhen   (e1,           grade 1)
        blade 2 (010) = Kan    (e2,           grade 1)
        blade 4 (100) = Gen    (e3,           grade 1)
        blade 3 (011) = Dui    (e12,          grade 2)
        blade 6 (110) = Xun    (e23,          grade 2)
        blade 5 (101) = Li     (-e31 = e13,   grade 2)
        blade 7 (111) = Qian   (e123,         grade 3)

  * sign is the orientation of the blade (+1 / -1). The state's sign component
    absorbs the orientation signs that arise from the geometric product.

The 8 positions of the cube: the eight vertices of the 3-bit cube Q3 reachable
from the start vertex are exactly {origin} U {3 single-line flips} U {3 double
line flips} U {complement}:

  all_positions(state) returns, in order:
    [("origin", state)]                        -- the original position
    + ("flip0", "flip1", "flip2")              -- single-line flips
    + ("double_flip01", "double_flip02",
       "double_flip12")                        -- two-line flips
    + [("complement", complement(state))]      -- antipode

complement is the Hodge dual under the right contraction with the unit
pseudoscalar e123 ("antipode via Hodge dual .e123"):

    dual(B) = B . e123            (right contraction / right product)

With the blade orientations above the dual acts on the index as bitwise
complement (blade ^ 0b111) and multiplies the sign by

    dual_sign(blade) = +1 if bit 1 of blade == 0 else -1

yielding exactly the required natural-convention identities (blade-wise):

    complement(Kan)  = Li     (dual of e2  is -e13 -> Li,  sign -1)
    complement(Gen)  = Dui    (dual of e3  is +e12 -> Dui, sign +1)
    complement(Zhen) = Xun    (dual of e1  is +e23 -> Xun, sign +1)
    complement(Kun)  = Qian   (dual of 1   is +e123,       sign +1)

complement is a sign-reversing involution: complement(complement(s)) == (blade, -sign);
complement^4 == identity.
"""

TRIGRAMS = {0: "Kun", 1: "Zhen", 2: "Kan", 4: "Gen", 3: "Dui", 6: "Xun", 5: "Li", 7: "Qian"}

# Repository blade labels (Path C): Li is written "-e31" (= e13).
BLADE_LABELS = {0: "1", 1: "e1", 2: "e2", 4: "e3", 3: "e12", 5: "-e31", 6: "e23", 7: "e123"}

MOVE_NAMES = [
    "origin",
    "flip0", "flip1", "flip2",
    "double_flip01", "double_flip02", "double_flip12",
    "complement",
]

MOVE_DESCRIPTIONS = {
    "origin": "no move -- the original position of the statement",
    "flip0": "flip the bottom line of the trigram (yang <-> yin)",
    "flip1": "flip the middle line of the trigram (yang <-> yin)",
    "flip2": "flip the top line of the trigram (yang <-> yin)",
    "double_flip01": "flip the bottom and middle lines (two-line flip)",
    "double_flip02": "flip the bottom and top lines (two-line flip)",
    "double_flip12": "flip the middle and top lines (two-line flip)",
    "complement": "complement -- the antipode of the cube via the Hodge dual (.e123)",
}

# Protocol choice (pre-registration was silent on the start state): the 8
# positions are the orbit of the cube from the neutral/empty trigram Kun.
START_STATE = (0, +1)


def _check_state(state):
    if not isinstance(state, (tuple, list)) or len(state) != 2:
        raise ValueError(f"state must be (blade_index, sign), got {state!r}")
    blade, sign = state
    if blade not in BLADE_LABELS:
        raise ValueError(f"blade_index must be in 0..7, got {blade!r}")
    if sign not in (+1, -1):
        raise ValueError(f"sign must be +1 or -1, got {sign!r}")
    return int(blade), int(sign)


def flip(state, line):
    """Flip a single line of the trigram: blade ^ (1 << line), sign unchanged."""
    blade, sign = _check_state(state)
    if line not in (0, 1, 2):
        raise ValueError(f"line must be 0, 1 or 2, got {line!r}")
    return (blade ^ (1 << line), sign)


def double_flip(state, a, b):
    """Flip two distinct lines; composition of two single flips (commutative)."""
    if a not in (0, 1, 2) or b not in (0, 1, 2):
        raise ValueError(f"lines must be in 0..2, got {a!r}, {b!r}")
    if a == b:
        raise ValueError(f"double flip needs two distinct lines, got {a!r}, {b!r}")
    return flip(flip(state, a), b)


def _dual_sign(blade):
    """Sign of the Hodge dual of a canonical blade under the .e123 convention.

    Derived from the geometric products:
      dual(e1) = +e23, dual(e2) = -e13, dual(e3) = +e12,
      dual(e12) = -e3, dual(e13) = +e2, dual(e23) = -e1,
      dual(1) = +e123, dual(e123) = -1
    which equals +1 exactly when bit 1 (the middle line) of the blade index is 0.
    """
    return +1 if (blade & 0b010) == 0 else -1


def complement(state):
    """Antipode of the cube: Hodge dual via right contraction with e123.

    Blade index maps to its bitwise complement; the sign is multiplied by the
    dual sign (bit-level, exact)."""
    blade, sign = _check_state(state)
    return (blade ^ 0b111, sign * _dual_sign(blade))


def all_positions(state):
    """The 8 positions of the cube for a start state.

    Returns a list of exactly 8 (move_name, state) entries -- origin first,
    then the 3 single-line flips, the 3 double-line flips, and the complement.
    Raises ValueError on degenerate (non-distinct) output.
    """
    blade, sign = _check_state(state)
    positions = [
        ("origin", (blade, sign)),
        ("flip0", flip(state, 0)),
        ("flip1", flip(state, 1)),
        ("flip2", flip(state, 2)),
        ("double_flip01", double_flip(state, 0, 1)),
        ("double_flip02", double_flip(state, 0, 2)),
        ("double_flip12", double_flip(state, 1, 2)),
        ("complement", complement(state)),
    ]
    states = [s for _, s in positions]
    if len(states) != 8 or len(set(states)) != 8:
        raise ValueError(f"degenerate move set for {state!r}: {positions}")
    return positions


def describe_state(state):
    """Human-readable description of a state for LLM naming prompts."""
    blade, sign = _check_state(state)
    return (f"{TRIGRAMS[blade]} (blade {BLADE_LABELS[blade]}, grade {bin(blade).count('1')}, "
            f"sign {sign:+d})")
