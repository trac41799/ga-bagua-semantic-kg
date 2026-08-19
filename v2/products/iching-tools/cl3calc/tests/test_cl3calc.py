"""POC-11 tests (TDD poc-11-cl3-calculator-mcp-tdd.md):

C1  op correctness: 24 flips, 8 complements (Kan->Li etc., blade-index equality),
    64 products vs PROD_TABLE, grades, combine, count_blades — all exact vs the
    verified POC-01 reference loaded by file path.
C2  strict validation: unknown op, unknown state, bad/extra/missing args,
    out-of-range values, and NO partial execution (whole call errors).
C3  100 seeded random op sequences evaluate identically to the reference (1e-12).
C6  import audit: zero imports (hence zero LLM code) in the package.
"""

import ast
import importlib.util
import random
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import iching_cl3calc as cl3c  # noqa: E402
from iching_cl3calc import (  # noqa: E402
    BadArgumentError,
    BLADE_NAMES,
    Cl3CalcError,
    PROD_TABLE,
    UnknownOpError,
    UnknownStateError,
    combine,
    complement,
    count_blades,
    double_flip,
    evaluate,
    flip,
    grade,
    product,
    validate,
)

REF_DIR = ROOT / "reference"

# Every state name accepted by the reference resolver.
ALL_STATES = [
    "1", "e1", "e2", "e3", "e12", "e23", "e31", "e123",
    "-1", "-e1", "-e2", "-e3", "-e12", "-e23", "-e31", "-e123",
    "kun", "gen", "kan", "xun", "zhen", "li", "dui", "qian",
    "000", "001", "010", "011", "100", "101", "110", "111",
]
# Names that resolve INTO a canonical 3-bit trigram state (flippable).
TRIGRAM_STATES = [
    "1", "e1", "e2", "e3", "e12", "e23", "e123",
    "kun", "gen", "kan", "xun", "zhen", "li", "dui", "qian",
    "000", "001", "010", "011", "100", "101", "110", "111",
]
CANONICAL_BITS = ["000", "001", "010", "011", "100", "101", "110", "111"]


# --------------------------------------------------------------------------
# Reference loading (by file path; cl3.py registered as top-level "cl3" so
# calculator.py's `from cl3 import ...` resolves, exactly like the POC-01 tree)
# --------------------------------------------------------------------------

def _load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, str(path))
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


@pytest.fixture(scope="module")
def ref():
    """The verified POC-01 calculator, loaded by file path (ground truth)."""
    _load_module("cl3", REF_DIR / "cl3.py")
    return _load_module("poc01_calculator", REF_DIR / "calculator.py")


def reference_run(calc, ops):
    """Run ops against the reference calculator; returns the per-op result list."""
    results = []
    for op in ops:
        n = op["op"]
        if n == "flip":
            v = calc.flip(calc.resolve(op["state"]), op["line"])
        elif n == "double_flip":
            v = calc.double_flip(calc.resolve(op["state"]), op["lines"])
        elif n == "complement":
            v = calc.complement(calc.resolve(op["state"]))
        elif n == "grade":
            v = calc.grade(calc.resolve(op["state"]))
        elif n == "product":
            v = calc.product(op["a"], op["b"])
        elif n == "combine":
            v = calc.combine(op["upper"], op["lower"])
        elif n == "count_blades":
            v = calc.count_blades(op["grade_k"])
        else:  # pragma: no cover
            raise AssertionError(f"generator produced unknown op {n!r}")
        results.append(v)
    return results


def approx_equal(a, b, tol=1e-12):
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return abs(a - b) < tol
    return a == b


def random_op(rng):
    """A uniformly random VALID op (all ops must succeed -> parity is meaningful)."""
    t = rng.choice(["flip", "double_flip", "complement", "grade",
                    "product", "combine", "count_blades"])
    if t == "flip":
        return {"op": "flip", "state": rng.choice(TRIGRAM_STATES), "line": rng.randint(0, 2)}
    if t == "double_flip":
        return {"op": "double_flip", "state": rng.choice(TRIGRAM_STATES),
                "lines": [rng.randint(0, 2) for _ in range(rng.randint(1, 3))]}
    if t == "complement":
        return {"op": "complement", "state": rng.choice(ALL_STATES)}
    if t == "grade":
        return {"op": "grade", "state": rng.choice(ALL_STATES)}
    if t == "product":
        return {"op": "product", "a": rng.choice(ALL_STATES), "b": rng.choice(ALL_STATES)}
    if t == "combine":
        return {"op": "combine", "upper": rng.choice(ALL_STATES), "lower": rng.choice(ALL_STATES)}
    return {"op": "count_blades", "grade_k": rng.randint(0, 3)}


# --------------------------------------------------------------------------
# C1 — op correctness (exact vs reference / PROD_TABLE)
# --------------------------------------------------------------------------

@pytest.mark.parametrize("bits", CANONICAL_BITS)
@pytest.mark.parametrize("line", [0, 1, 2])
def test_c1_24_canonical_flips(ref, bits, line):
    mine = flip(bits, line)
    theirs = ref.flip(ref.resolve(bits), line)
    assert mine == theirs
    expected = list(bits)
    expected[line] = "1" if expected[line] == "0" else "0"
    assert mine == ref.BITS_TO_STATE["".join(expected)]


def test_c1_flips_all_trigram_names_match_reference(ref):
    checked = 0
    for name in TRIGRAM_STATES:
        for line in (0, 1, 2):
            mine = flip(name, line)
            theirs = ref.flip(ref.resolve(name), line)
            assert mine == theirs, (name, line, mine, theirs)
            assert flip(mine, line) == ref.resolve(name), "flip must be an involution"
            checked += 1
    assert checked == len(TRIGRAM_STATES) * 3


def test_c1_8_complements_blade_index_equality(ref):
    for bits in CANONICAL_BITS:
        i, _s = ref.resolve(bits)
        expected_idx = ref.PROD_TABLE[i][7][0]  # dual blade index = * e123
        mine = complement(bits)
        theirs = ref.complement(ref.resolve(bits))
        assert mine == theirs
        assert mine[0] == expected_idx, (bits, mine, expected_idx)
        # complement is an involution on canonical trigram states
        assert complement(complement(bits)) == ref.resolve(bits)


def test_c1_complement_known_pairs():
    assert complement("kan") == (6, -1) and complement("li") == (2, 1)  # Kan <-> Li
    assert complement("kun") == (7, 1) and complement("qian") == (0, 1)  # Kun <-> Qian
    assert complement("zhen") == (5, 1) and complement("xun") == (1, 1)  # Zhen <-> Xun
    assert complement("gen") == (4, 1) and complement("dui") == (3, 1)  # Gen <-> Dui


@pytest.mark.parametrize("i", range(8))
@pytest.mark.parametrize("j", range(8))
def test_c1_64_products_vs_prod_table(ref, i, j):
    k, s = PROD_TABLE[i][j]
    expected = ("-" if s < 0 else "") + BLADE_NAMES[k]
    mine = product(BLADE_NAMES[i], BLADE_NAMES[j])
    assert mine == expected
    assert mine == ref.product(BLADE_NAMES[i], BLADE_NAMES[j])
    assert PROD_TABLE == ref.PROD_TABLE, "local PROD_TABLE must equal ground truth"


def test_c1_products_sign_prefixed(ref):
    for a in ALL_STATES:
        for b in ALL_STATES:
            assert product(a, b) == ref.product(a, b)
    # -e1 * e2 = -(e1 * e2) = -e12
    assert product("-e1", "e2") == "-e12"
    assert product("e1", "e3") == "-e31"
    assert product("e2", "e3") == "e23"
    assert product("e123", "e123") == "-1"


def test_c1_grades(ref):
    expected = {"1": 0, "e1": 1, "e2": 1, "e3": 1, "e12": 2, "e23": 2, "e31": 2, "e123": 3}
    for name, g in expected.items():
        assert grade(name) == g
        assert grade(name) == ref.grade(ref.resolve(name))
        assert grade("-" + name) == g


@pytest.mark.parametrize("u", ALL_STATES)
@pytest.mark.parametrize("l", ALL_STATES)
def test_c1_combine_all_pairs_match_reference(ref, u, l):
    mine = combine(u, l)
    theirs = ref.combine(u, l)
    assert mine == theirs
    assert 0 <= mine <= 63


def test_c1_combine_known_codes():
    assert combine("qian", "kun") == 0b111 << 3 | 0b000  # 56
    assert combine("kun", "qian") == 0b000 << 3 | 0b111  # 7
    assert combine("li", "li") == 0b101 << 3 | 0b101  # 45
    assert combine("-e1", "li") == 0b100 << 3 | 0b101  # 37 (sign ignored)


def test_c1_count_blades(ref):
    for k, expected in ((0, 1), (1, 3), (2, 3), (3, 1)):
        assert count_blades(k) == expected
        assert count_blades(k) == ref.count_blades(k)


# --------------------------------------------------------------------------
# C2 — strict validation (typed errors, no partial execution)
# --------------------------------------------------------------------------

def test_c2_unknown_op_raises():
    with pytest.raises(UnknownOpError):
        evaluate([{"op": "rotate", "state": "kun"}])


def test_c2_unknown_state_raises():
    with pytest.raises(UnknownStateError):
        evaluate([{"op": "product", "a": "zzz", "b": "e1"}])


def test_c2_flip_requires_trigram_state():
    with pytest.raises(BadArgumentError):
        evaluate([{"op": "flip", "state": "-e1", "line": 0}])


def test_c2_line_out_of_range():
    for bad in (3, -1):
        with pytest.raises(BadArgumentError):
            evaluate([{"op": "flip", "state": "kun", "line": bad}])


def test_c2_line_wrong_type():
    for bad in ("0", None, 1.5, True):
        with pytest.raises(BadArgumentError):
            evaluate([{"op": "flip", "state": "kun", "line": bad}])


def test_c2_grade_k_out_of_range():
    for bad in (4, -1, "2", True, None):
        with pytest.raises(BadArgumentError):
            evaluate([{"op": "count_blades", "grade_k": bad}])


def test_c2_double_flip_lines_bad():
    for bad in ("01", [0, 3], [1.5], True, 0):
        with pytest.raises(BadArgumentError):
            evaluate([{"op": "double_flip", "state": "kun", "lines": bad}])


def test_c2_missing_argument():
    with pytest.raises(BadArgumentError):
        evaluate([{"op": "product", "a": "e1"}])
    with pytest.raises(BadArgumentError):
        evaluate([{"op": "flip", "state": "kun"}])


def test_c2_unknown_argument_key():
    with pytest.raises(BadArgumentError):
        evaluate([{"op": "flip", "state": "kun", "line": 0, "extra": 1}])


def test_c2_state_not_string():
    with pytest.raises(BadArgumentError):
        evaluate([{"op": "grade", "state": 5}])


def test_c2_op_not_dict():
    with pytest.raises(BadArgumentError):
        evaluate([["flip", "kun", 0]])
    with pytest.raises(BadArgumentError):
        evaluate(["flip"])


def test_c2_ops_not_a_list():
    with pytest.raises(BadArgumentError):
        evaluate({"op": "flip", "state": "kun", "line": 0})
    with pytest.raises(BadArgumentError):
        evaluate("kun")


def test_c2_empty_ops_rejected():
    with pytest.raises(BadArgumentError):
        evaluate([])


def test_c2_validate_raises_typed_errors_too():
    with pytest.raises(UnknownOpError):
        validate([{"op": "nope"}])
    with pytest.raises(UnknownStateError):
        validate([{"op": "grade", "state": "qwerty"}])
    with pytest.raises(BadArgumentError):
        validate([{"op": "count_blades", "grade_k": 9}])


def test_c2_error_type_hierarchy():
    for exc in (UnknownOpError(), UnknownStateError(), BadArgumentError()):
        assert isinstance(exc, Cl3CalcError)
        assert isinstance(exc, Exception)


def test_c2_no_partial_execution(monkeypatch):
    """A bad op anywhere in the list => the WHOLE call raises; nothing executes."""
    calls = []
    real_flip = cl3c.flip

    def spy(*a, **k):
        calls.append((a, k))
        return real_flip(*a, **k)

    monkeypatch.setattr(cl3c, "flip", spy)

    ops = [
        {"op": "flip", "state": "kun", "line": 0},
        {"op": "product", "a": "e1"},  # missing 'b' -> validation error at op #1
    ]
    with pytest.raises(BadArgumentError) as excinfo:
        cl3c.evaluate(ops)
    assert "op #1" in str(excinfo.value)
    assert calls == [], "op #0 must NOT have executed (validate-before-execute)"

    # a bad op in the middle of a longer chain also fails the whole call
    ops = [
        {"op": "flip", "state": "kun", "line": 0},
        {"op": "grade", "state": "bogus"},
        {"op": "product", "a": "e1", "b": "e2"},
    ]
    with pytest.raises(UnknownStateError) as excinfo:
        cl3c.evaluate(ops)
    assert "op #1" in str(excinfo.value)
    assert calls == []


def test_c2_execution_phase_error_still_aborts(monkeypatch):
    """Even an unexpected runtime failure aborts the whole call (no partial result)."""

    def boom(*a, **k):
        raise RuntimeError("unexpected")

    monkeypatch.setattr(cl3c, "flip", boom)
    with pytest.raises(RuntimeError):
        evaluate([{"op": "flip", "state": "kun", "line": 0},
                  {"op": "grade", "state": "kun"}])


# --------------------------------------------------------------------------
# C3 — parity: 100 seeded random op sequences == reference calculator (1e-12)
# --------------------------------------------------------------------------

def test_c3_100_random_sequences_parity(ref):
    rng = random.Random(20260812)
    for seq in range(100):
        ops = [random_op(rng) for _ in range(10)]
        expected = reference_run(ref, ops)
        mine = evaluate(ops)

        assert len(mine["steps"]) == len(ops) == 10
        for k, (op, ref_val) in enumerate(zip(ops, expected)):
            step = mine["steps"][k]
            assert step["op"] == op["op"]
            assert approx_equal(step["result"], ref_val), \
                f"seq {seq} step {k}: {op} -> {step['result']} != ref {ref_val}"
        assert approx_equal(mine["result"], expected[-1]), f"seq {seq} final result"


def test_c3_sample_10op_comparison(ref):
    """The verification sample: one 10-op random sequence, printed side by side."""
    rng = random.Random(7)
    ops = [random_op(rng) for _ in range(10)]
    mine = evaluate(ops)
    expected = reference_run(ref, ops)

    print("\n=== 10-op sample comparison (random.Random(7)) ===")
    for i, (op, ref_val) in enumerate(zip(ops, expected)):
        step = mine["steps"][i]["result"]
        status = "OK" if approx_equal(step, ref_val) else "MISMATCH"
        print(f"  [{i}] {op}")
        print(f"      mine={step!r}  ref={ref_val!r}  {status}")
        assert approx_equal(step, ref_val)
    print(f"  final: mine={mine['result']!r} ref={expected[-1]!r}  "
          f"{'OK' if approx_equal(mine['result'], expected[-1]) else 'MISMATCH'}")
    assert approx_equal(mine["result"], expected[-1])


# --------------------------------------------------------------------------
# C6 — import audit: zero LLM code / zero imports in the tool path
# --------------------------------------------------------------------------

def test_c6_import_audit():
    pkg = ROOT / "iching_cl3calc"
    py_files = sorted(pkg.rglob("*.py"))
    assert py_files, "package directory contains no python files"

    forbidden = {
        "llm", "openai", "anthropic", "together", "replicate", "mistral",
        "transformers", "torch", "tensorflow", "keras", "numpy", "scipy",
        "requests", "httpx", "urllib", "aiohttp", "socket", "websocket",
        "flask", "fastapi", "grpc", "sqlalchemy",
    }
    for py in py_files:
        tree = ast.parse(py.read_text(encoding="utf-8"), filename=str(py))
        for node in ast.walk(tree):
            # zero imports at all: no LLM client, no network, no deps possible
            assert not isinstance(node, (ast.Import, ast.ImportFrom)), \
                f"{py.name}: tool path must be import-free (zero LLM / network / deps)"
            # no forbidden identifier may appear in the code (docstrings excluded:
            # they are ast.Constant strings, not identifiers)
            if isinstance(node, ast.Name):
                assert node.id.lower() not in forbidden, \
                    f"{py.name}: forbidden identifier {node.id!r} in tool path"
            elif isinstance(node, ast.Attribute):
                assert node.attr.lower() not in forbidden, \
                    f"{py.name}: forbidden attribute {node.attr!r} in tool path"


def test_c6_evaluate_is_deterministic():
    ops = [
        {"op": "flip", "state": "kan", "line": 1},
        {"op": "complement", "state": "e1"},
        {"op": "product", "a": "-e31", "b": "e2"},
        {"op": "combine", "upper": "qian", "lower": "kun"},
        {"op": "count_blades", "grade_k": 2},
    ]
    first = evaluate(ops)
    for _ in range(5):
        assert evaluate(ops) == first


def test_c6_result_shape():
    ops = [{"op": "flip", "state": "kun", "line": 0},
           {"op": "grade", "state": "li"}]
    out = evaluate(ops)
    assert set(out) == {"result", "steps"}
    assert len(out["steps"]) == 2
    assert out["steps"][0] == {"op": "flip", "args": {"state": "kun", "line": 0}, "result": (1, 1)}
    assert out["steps"][1] == {"op": "grade", "args": {"state": "li"}, "result": 2}
    assert out["result"] == 2
