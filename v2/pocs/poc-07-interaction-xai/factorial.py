"""POC-02: factorial design algebra — combinations, blades, contrasts, decomposition, names."""

import itertools
import math
from functools import lru_cache

# trigram names in binary order (000..111): Kun, Gen, Kan, Xun, Zhen, Li, Dui, Qian
TRIGRAM_NAMES = ["坤", "艮", "坎", "巽", "震", "離", "兑", "乾"]
TRIGRAM_PINYIN = ["Kun", "Gen", "Kan", "Xun", "Zhen", "Li", "Dui", "Qian"]

# natural-convention blade index per 3-bit pattern: blade = product of selected basis vectors
# e1=1, e2=2, e3=3, e12=4, e13=-e31 → canonical blade 6 sign -1, e23=5, e123=7
BITS_TO_BLADE = {
    0b000: (0, +1),  # 1
    0b001: (3, +1),  # e3
    0b010: (2, +1),  # e2
    0b011: (5, +1),  # e23
    0b100: (1, +1),  # e1
    0b101: (6, -1),  # e13 = -e31
    0b110: (4, +1),  # e12
    0b111: (7, +1),  # e123
}

# King Wen hexagram names: table[upper_binary][lower_binary]
HEXAGRAM_NAMES = [
    # upper: Kun(000)
    ["坤為地", "地山謙", "地水師", "地風升", "地雷復", "地火明夷", "地澤臨", "地天泰"],
    # upper: Gen(001)
    ["山地剝", "艮為山", "山水蒙", "山風蠱", "山雷頤", "山火賁", "山澤損", "山天大畜"],
    # upper: Kan(010)
    ["水地比", "水山蹇", "坎為水", "水風井", "水雷屯", "水火既濟", "水澤節", "水天需"],
    # upper: Xun(011)
    ["風地觀", "風山漸", "風水渙", "巽為風", "風雷益", "風火家人", "風澤中孚", "風天小畜"],
    # upper: Zhen(100)
    ["雷地豫", "雷山小過", "雷水解", "雷風恆", "震為雷", "雷火豐", "雷澤歸妹", "雷天大壯"],
    # upper: Li(101)
    ["火地晉", "火山旅", "火水未濟", "火風鼎", "火雷噬嗑", "離為火", "火澤睽", "火天大有"],
    # upper: Dui(110)
    ["澤地萃", "澤山咸", "澤水困", "澤風大過", "澤雷隨", "澤火革", "兌為澤", "澤天夬"],
    # upper: Qian(111)
    ["天地否", "天山遯", "天水訟", "天風姤", "天雷無妄", "天火同人", "天澤履", "乾為天"],
]


# ---- design.py ----

def combinations(k):
    return [tuple((v >> i) & 1 for i in range(k)) for v in range(2 ** k)]


def blade_state(bits):
    """bits: 3-tuple (b1,b2,b3). Returns (blade_idx, sign) under the natural convention."""
    if len(bits) != 3:
        raise ValueError("blade_state requires 3 bits")
    code = (bits[0] << 2) | (bits[1] << 1) | bits[2]
    return BITS_TO_BLADE[code]


def grade_of(bits):
    return sum(bits)


# ---- contrasts.py ----

def contrast_signs(k):
    """{effect: [(row_idx, sign)]} for all effects of a 2^k design.

    sign(effect S, run r) = product over i in S of r_i  (bit 1 -> +1, bit 0 -> -1).
    """
    runs = combinations(k)
    effects = []
    for size in range(1, k + 1):
        for S in itertools.combinations(range(k), size):
            effects.append(S)
    out = {}
    for S in effects:
        signs = []
        for row in runs:
            s = 1
            for i in S:
                s *= 1 if row[i] else -1
            signs.append(s)
        out[S] = signs
    return out


def main_effect(data, factor):
    """data: list of (row_bits_tuple, response). Returns the standard 2^k effect."""
    n = len(data)
    c = 0.0
    for row, y in data:
        s = 1 if row[factor] else -1
        c += s * y
    return c / (n / 2)


def interaction(data, factors):
    n = len(data)
    c = 0.0
    for row, y in data:
        s = 1
        for f in factors:
            s *= 1 if row[f] else -1
        c += s * y
    return c / (n / 2)


# ---- decompose.py ----

def mobius_coefficients(f, n):
    """Möbius inversion on the subset lattice of {0..n-1}.

    f: callable(subset: frozenset) -> float. Returns {frozenset: coefficient}.
    """
    out = {}
    universe = frozenset(range(n))
    for size in range(n + 1):
        for S in itertools.combinations(range(n), size):
            S = frozenset(S)
            acc = 0.0
            for T in _subsets_of(S):
                acc += ((-1) ** (len(S) - len(T))) * f(T)
            out[S] = acc
    return out


def _subsets_of(S):
    items = list(S)
    for mask in range(2 ** len(items)):
        yield frozenset(items[i] for i in range(len(items)) if (mask >> i) & 1)


def blade_projection(f, n):
    """Grade-projection route; must agree with mobius_coefficients (tested)."""
    return mobius_coefficients(f, n)


# ---- bagua_names.py ----

def trigram_name(bits3):
    code = (bits3[0] << 2) | (bits3[1] << 1) | bits3[2]
    return TRIGRAM_NAMES[code]


def trigram_pinyin(bits3):
    code = (bits3[0] << 2) | (bits3[1] << 1) | bits3[2]
    return TRIGRAM_PINYIN[code]


def hexagram_name(upper_bits3, lower_bits3):
    u = (upper_bits3[0] << 2) | (upper_bits3[1] << 1) | upper_bits3[2]
    l = (lower_bits3[0] << 2) | (lower_bits3[1] << 1) | lower_bits3[2]
    return HEXAGRAM_NAMES[u][l]
