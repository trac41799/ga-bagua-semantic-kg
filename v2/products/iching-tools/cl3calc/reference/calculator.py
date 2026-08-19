"""POC-01 calculator: blade-state ops over Cl(3) with the Bagua state vocabulary.

State = (blade_index 0..7, sign ±1). Canonical names: 1, e1, e2, e3, e12, e23, e31, e123.
Trigram 3-bit patterns (bottom-to-top as (b1,b2,b3)) under the NATURAL convention:
blade = product of selected basis vectors: 101 -> e13 = -e31, 110 -> e12, 011 -> e23.
"""

from cl3 import Multivector, PROD_TABLE

BLADE_NAMES = ["1", "e1", "e2", "e3", "e12", "e23", "e31", "e123"]

# bit pattern -> (blade_idx, sign)  [natural convention]
BITS_TO_STATE = {
    "000": (0, +1),   # Kun
    "001": (3, +1),   # Gen (e3)
    "010": (2, +1),   # Kan (e2)
    "011": (5, +1),   # Xun (e23)
    "100": (1, +1),   # Zhen (e1)
    "101": (6, -1),   # Li (e13 = -e31)
    "110": (4, +1),   # Dui (e12)
    "111": (7, +1),   # Qian (e123)
}
STATE_TO_BITS = {v: k for k, v in BITS_TO_STATE.items()}
TRIGRAM_PINYIN = {"kun": "000", "gen": "001", "kan": "010", "xun": "011",
                  "zhen": "100", "li": "101", "dui": "110", "qian": "111"}


def resolve(name):
    """Canonical name (blade or trigram pinyin or 3-bit pattern) -> (blade_idx, sign)."""
    s = name.strip().lower()
    if s in BLADE_NAMES:
        return BLADE_NAMES.index(s), +1
    if s.startswith("-") and s[1:] in BLADE_NAMES:
        return BLADE_NAMES.index(s[1:]), -1
    if s in TRIGRAM_PINYIN:
        return BITS_TO_STATE[TRIGRAM_PINYIN[s]]
    if s in BITS_TO_STATE:
        return BITS_TO_STATE[s]
    raise ValueError(f"unknown state name: {name!r}")


def format_state(state):
    idx, sign = state
    name = BLADE_NAMES[idx]
    return f"-{name}" if sign < 0 else name


def flip(state, line):
    """Flip one line of the trigram (multiply by the corresponding basis vector)."""
    idx, sign = state
    bits = list(STATE_TO_BITS[(idx, sign)]) if (idx, sign) in STATE_TO_BITS else None
    if bits is None:
        raise ValueError("flip requires a trigram state (bit pattern)")
    bits = [int(b) for b in bits]
    bits[line] = 1 - bits[line]
    return BITS_TO_STATE["".join(str(b) for b in bits)]


def double_flip(state, lines):
    out = state
    for ln in lines:
        out = flip(out, ln)
    return out


def complement(state):
    """Antipode of the cube = Hodge dual (multiply by e123), normalized to canonical trigram."""
    return _dual_state(*state)


def _blade_multivector(idx, sign):
    c = [0.0] * 8
    c[idx] = sign
    return Multivector(c)


def _dual_state(idx, sign):
    mv = _blade_multivector(idx, sign)
    ps = _blade_multivector(7, 1.0)
    out = mv.geo_product(ps)
    coeffs = out.c
    nonzero = [(i, c) for i, c in enumerate(coeffs) if abs(c) > 1e-12]
    if len(nonzero) != 1:
        raise ValueError(f"dual of blade {idx} is not a single blade: {nonzero}")
    i, _ = nonzero[0]
    # normalize orientation to the canonical trigram state for that blade
    canon = next((st for st in BITS_TO_STATE.values() if st[0] == i), None)
    if canon is None:
        raise ValueError(f"no canonical trigram for blade {i}")
    return canon


def grade(state):
    idx, sign = state
    return {0: 0, 1: 1, 2: 1, 3: 1, 4: 2, 5: 2, 6: 2, 7: 3}[idx]


def product(a, b):
    """Geometric product of two named blades -> canonical result string (e.g., 'e123', '-e1', '1')."""
    ia, sa = resolve(a)
    ib, sb = resolve(b)
    k, s = PROD_TABLE[ia][ib]
    total_sign = sa * sb * s
    name = BLADE_NAMES[k]
    return f"-{name}" if total_sign < 0 else name


def combine(upper, lower):
    """Two trigram states -> 6-bit hexagram code (upper<<3 | lower, binary order)."""
    iu, _ = resolve(upper)
    il, _ = resolve(lower)
    bu = next(k for k, v in BITS_TO_STATE.items() if v == (iu, +1) or v == (iu, -1))
    bl = next(k for k, v in BITS_TO_STATE.items() if v == (il, +1) or v == (il, -1))
    code = (int(bu, 2) << 3) | int(bl, 2)
    return code


def count_blades(grade_k):
    return {0: 1, 1: 3, 2: 3, 3: 1}[grade_k]
