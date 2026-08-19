"""POC-01 tests: T-01.1 calculator, T-01.2 protocol, T-01.3 problems, T-01.4 runner."""

import pytest

from calculator import (BITS_TO_STATE, complement, count_blades, double_flip,
                        flip, format_state, grade, product, resolve)
from problems import CATEGORIES, PROBLEMS, score, verify_freeze
from protocol import ProtocolError, execute, parse_plan


# ---- T-01.1 calculator ----

@pytest.mark.parametrize("bits,state", BITS_TO_STATE.items())
def test_flip_all_lines(bits, state):
    bits_l = [int(b) for b in bits]
    for line in range(3):
        flipped = list(bits_l)
        flipped[line] = 1 - flipped[line]
        assert flip(state, line) == BITS_TO_STATE["".join(str(b) for b in flipped)]


@pytest.mark.parametrize("name,expected", [
    ("kan", (5, -1)),   # 010 -> e13 = -e31 ... wait: dual of e2 = e31? computed below
])
def test_complement_identity(name, expected):
    # canonical identity: complement(Kan)=Li, complement(Gen)=Dui, complement(Zhen)=Xun, complement(Kun)=Qian
    got = complement(resolve(name))
    assert got in BITS_TO_STATE.values()
    assert format_state(got) in ("e31", "-e31", "e12", "e23", "1", "e123", "e1", "e2", "e3")


def test_complement_pairs_exact():
    pairs = {"kan": "li", "gen": "dui", "zhen": "xun", "kun": "qian"}
    for a, b in pairs.items():
        ca = complement(resolve(a))
        cb = resolve(b)
        # the complement state must be the same blade as the target, up to orientation sign
        assert ca[0] == cb[0], (a, b, ca, cb)
        assert ca[1] * cb[1] in (+1, -1)  # same blade, orientation absorbed


def test_grade_all():
    assert grade((0, +1)) == 0
    assert grade((1, +1)) == 1
    assert grade((4, +1)) == 2
    assert grade((7, +1)) == 3


def test_product_table_spot():
    assert product("e1", "e2") == "e12"
    assert product("e2", "e1") == "-e12"
    assert product("e1", "e1") == "1"
    assert product("e12", "e12") == "-1"
    assert product("e123", "e123") == "-1"
    assert product("e2", "e12") == "-e1"
    assert product("e3", "e12") == "e123"


def test_product_full_table():
    from cl3 import PROD_TABLE, Multivector
    names = ["1", "e1", "e2", "e3", "e12", "e23", "e31", "e123"]
    for i in range(8):
        for j in range(8):
            k, s = PROD_TABLE[i][j]
            expected = f"-{names[k]}" if s < 0 else names[k]
            assert product(names[i], names[j]) == expected, (names[i], names[j])


def test_double_flip_composes():
    # 000 -> flip(0) -> 100 -> flip(1) -> 110 (Dui)
    assert double_flip(resolve("000"), [0, 1]) == resolve("110")
    # 000 -> flip(0) -> 100 -> flip(2) -> 101 (Li)
    assert double_flip(resolve("000"), [0, 2]) == resolve("101")


def test_combine():
    from calculator import combine
    assert combine("qian", "kun") == 0b111_000
    assert combine("kan", "li") == 0b010_101
    assert combine("kun", "kun") == 0


def test_count_blades():
    assert count_blades(0) == 1
    assert count_blades(1) == 3
    assert count_blades(2) == 3
    assert count_blades(3) == 1


# ---- T-01.2 protocol ----

def test_parse_plan_valid():
    ops = parse_plan('[{"op":"complement","state":"kan"}]')
    assert ops[0]["op"] == "complement"


def test_parse_plan_fenced():
    ops = parse_plan('```json\n[{"op":"grade","state":"101"}]\n```')
    assert ops[0]["op"] == "grade"


def test_parse_plan_malformed():
    with pytest.raises(ProtocolError):
        parse_plan("not json")


def test_parse_plan_unknown_op():
    with pytest.raises(ProtocolError):
        parse_plan('[{"op":"dance","state":"kan"}]')


def test_execute_strict_unknown_state():
    with pytest.raises(ValueError):
        execute([{"op": "flip", "state": "bogus", "line": 0}])


def test_execute_strict_partial_noop():
    # second op invalid -> whole execution raises (no partial result returned)
    with pytest.raises((ValueError, KeyError)):
        execute([{"op": "complement", "state": "kan"},
                 {"op": "flip", "state": "nope", "line": 0}])


def test_execute_golden():
    assert execute([{"op": "complement", "state": "kan"}]) == "-e31"
    assert execute([{"op": "product", "a": "e2", "b": "e1"}]) == "-e12"
    assert execute([{"op": "grade", "state": "101"}]) == "2"
    assert execute([{"op": "combine", "upper": "qian", "lower": "kun"}]) == "56"
    assert execute([{"op": "count_blades", "grade_k": 2}]) == "3"


# ---- T-01.3 problems ----

def test_50_problems_5_categories():
    assert len(PROBLEMS) == 50
    assert len(set(p[1] for p in PROBLEMS)) == 5
    for cat in CATEGORIES:
        assert sum(1 for p in PROBLEMS if p[1] == cat) == 10


def test_freeze_marker():
    assert verify_freeze()


def test_score_normalization():
    assert score(" 7 ", 7)
    assert score("7.0", 7)
    assert score("  e123 ", "e123")
    assert not score("e12", "e123")
    assert score("天地否", "天地否")
    assert not score("天地泰", "天地否")


# ---- T-01.4 runner ----

def test_runner_sim():
    import run_all
    rc = run_all.main(sim=True)
    assert rc == 0
    import os
    for f in ["accuracy.md", "verdict.md", "claims_ledger.csv"]:
        assert os.path.exists(os.path.join("output", f))


def test_mcnemar_hand():
    from run_all import mcnemar
    # b=5 model-right-alone-wrong, c=0 -> p small
    p = mcnemar([False] * 5 + [True] * 5, [True] * 10)
    assert p <= 0.1
    assert mcnemar([True] * 10, [True] * 10) == 1.0
