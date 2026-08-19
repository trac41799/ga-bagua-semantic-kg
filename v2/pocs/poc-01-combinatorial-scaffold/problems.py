"""POC-01 benchmark: 50 frozen problems (5 categories × 10) with exact answer keys.

Freeze marker: problems.keys.sha256 (hash of this module's PROBLEMS list).
Answers are canonical: numbers as ints, blades like 'e123'/'-e1', 3-bit patterns, hexagram names.
"""

import hashlib
import json

# (id, category, text, answer)
PROBLEMS = [
    # ---- A. parity / bit ops ----
    (1, "parity", "How many yang (1) lines does the trigram 巽 (Xun) have?", 2),
    (2, "parity", "What is the parity (sum of bits mod 2) of the trigram 110?", 0),
    (3, "parity", "How many yin (0) lines does the trigram 乾 (Qian) have?", 0),
    (4, "parity", "What is the sum of the bits of the trigram 坎 (010)?", 1),
    (5, "parity", "What 3-bit pattern has yang lines in positions 1 and 3 and yin in position 2?", "101"),
    (6, "parity", "How many lines differ between the trigrams 100 and 011?", 3),
    (7, "parity", "What is the bitwise AND of 110 and 011 (3-bit pattern)?", "010"),
    (8, "parity", "What is the bitwise XOR of 101 and 011 (3-bit pattern)?", "110"),
    (9, "parity", "What is the bitwise OR of 100 and 010 (3-bit pattern)?", "110"),
    (10, "parity", "How many of the 8 trigrams have an odd number of yang lines?", 4),
    # ---- B. complement & De Morgan ----
    (11, "complement", "What is the complement trigram of 坎 (010) as a 3-bit pattern?", "101"),
    (12, "complement", "What is the complement trigram of 乾 (111) as a 3-bit pattern?", "000"),
    (13, "complement", "What is the complement trigram of 艮 (001) as a 3-bit pattern?", "110"),
    (14, "complement", "The complement of the complement of 震 (100) is which 3-bit pattern?", "100"),
    (15, "complement", "Using De Morgan, express the complement of (line1 AND line2) as a 3-bit pattern with line3 = 0.", "110"),
    (16, "complement", "Using De Morgan, express the complement of (line1 OR line3) as a 3-bit pattern with line2 = 0.", "010"),
    (17, "complement", "How many of the 8 trigrams are self-complementary?", 0),
    (18, "complement", "Which 3-bit pattern is the complement of 011?", "100"),
    (19, "complement", "The complement of a trigram with grade 2 has what grade?", 1),
    (20, "complement", "What is the complement trigram of 離 (101) as a 3-bit pattern?", "010"),
    # ---- C. interaction counting / grade ----
    (21, "interaction", "How many 2-way interactions exist among 3 factors?", 3),
    (22, "interaction", "How many interaction terms of order 2 or higher exist in a 3-factor design?", 4),
    (23, "interaction", "How many effects (main effects plus all interactions) exist in a 2^3 design?", 7),
    (24, "interaction", "What is the grade (number of yang lines) of the trigram 101?", 2),
    (25, "interaction", "What is the grade of the blade e123?", 3),
    (26, "interaction", "What is the grade of the blade e31?", 2),
    (27, "interaction", "How many basis blades of grade 1 exist in Cl(3)?", 3),
    (28, "interaction", "How many basis blades of grade 2 exist in Cl(3)?", 3),
    (29, "interaction", "How many basis blades of grade 3 exist in Cl(3)?", 1),
    (30, "interaction", "How many basis blades in total exist in Cl(3) (all grades)?", 8),
    # ---- D. sign bookkeeping ----
    (31, "signs", "What is the coefficient sign of e12 in the product e2·e1?", -1),
    (32, "signs", "What is the result of e1·e1?", "1"),
    (33, "signs", "What is the result of e12·e12?", "-1"),
    (34, "signs", "What is the result of e123·e123?", "-1"),
    (35, "signs", "What is the result of e1·e2·e3 (blade name)?", "e123"),
    (36, "signs", "What is the result of e1·e23 (blade name)?", "e123"),
    (37, "signs", "What is the result of e2·e12 (blade name with sign)?", "-e1"),
    (38, "signs", "What is the result of e3·e12 (blade name)?", "e123"),
    (39, "signs", "How many of the 64 products of two basis blades give a positive scalar?", 4),
    (40, "signs", "What is the reverse of e23 (blade name with sign)?", "-e23"),
    # ---- E. hexagram composition ----
    (41, "hexagram", "Combine upper 乾 (111) and lower 坤 (000). What hexagram?", "天地否"),
    (42, "hexagram", "Combine upper 坎 (010) and lower 離 (101). What hexagram?", "水火既濟"),
    (43, "hexagram", "Combine upper 離 (101) and lower 坎 (010). What hexagram?", "火水未濟"),
    (44, "hexagram", "Combine upper 艮 (001) and lower 坎 (010). What hexagram?", "山水蒙"),
    (45, "hexagram", "Combine upper 坤 (000) and lower 乾 (111). What hexagram?", "地天泰"),
    (46, "hexagram", "Combine upper 乾 (111) and lower 乾 (111). What hexagram?", "乾為天"),
    (47, "hexagram", "Combine upper 坤 (000) and lower 坤 (000). What hexagram?", "坤為地"),
    (48, "hexagram", "How many distinct hexagrams exist?", 64),
    (49, "hexagram", "Combine upper 震 (100) and lower 兌 (110). What hexagram?", "雷澤歸妹"),
    (50, "hexagram", "Combine upper 巽 (011) and lower 乾 (111). What hexagram?", "風天小畜"),
]

CATEGORIES = ["parity", "complement", "interaction", "signs", "hexagram"]


def freeze_hash():
    payload = json.dumps(PROBLEMS, ensure_ascii=False, sort_keys=True)
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def verify_freeze():
    import os
    p = os.path.join(os.path.dirname(os.path.abspath(__file__)), "problems.keys.sha256")
    with open(p, encoding="utf-8") as f:
        stored = f.read().strip()
    return stored == freeze_hash()


def score(answer_text, key):
    """Exact-match scoring after normalization."""
    a = str(answer_text).strip().lower().replace(" ", "")
    if isinstance(key, int):
        try:
            return int(float(a)) == key
        except ValueError:
            return a == str(key)
    k = str(key).strip().lower().replace(" ", "")
    return a == k
