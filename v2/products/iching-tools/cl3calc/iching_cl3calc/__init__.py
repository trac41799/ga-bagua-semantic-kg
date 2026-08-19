"""iching_cl3calc — POC-11: Cl3 calculator as a structured-op evaluation tool.

Zero-LLM, deterministic, pure-Python clone of the verified POC-01 Cl3 calculator
(v2/pocs/poc-01-combinatorial-scaffold/calculator.py + cl3.py). Op semantics are
exact copies of the reference; PROD_TABLE below is the ground truth, copied
verbatim from cl3.py.

State vocabulary (same resolution as the reference):
  * blades: 1, e1, e2, e3, e12, e23, e31, e123 (sign-prefixed allowed, e.g. "-e1")
  * trigram pinyin: kun, gen, kan, xun, zhen, li, dui, qian
  * 3-bit patterns (bottom-to-top), e.g. "101" -> Li = (6, -1)

Contract:
  evaluate(ops: list[dict]) -> {"result": <last op value>, "steps": [...]}
  validate(ops: list[dict]) -> None

STRICT: the whole op list is validated BEFORE any op executes. If ANY op fails
(unknown op, unknown state, bad/missing/extra/wrong-typed argument, out-of-range
value) the entire call raises a typed error and NOTHING is executed — there is
no partial execution.
"""

BLADE_NAMES = ["1", "e1", "e2", "e3", "e12", "e23", "e31", "e123"]

# PROD_TABLE[i][j] = (result_index, sign) — copied VERBATIM from POC-01 cl3.py.
# Blade order: 0=1, 1=e1, 2=e2, 3=e3, 4=e12, 5=e23, 6=e31, 7=e123
PROD_TABLE = [
    [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1)],
    [(1, 1), (0, 1), (4, 1), (6, -1), (2, 1), (7, 1), (3, -1), (5, 1)],
    [(2, 1), (4, -1), (0, 1), (5, 1), (1, -1), (3, 1), (7, 1), (6, 1)],
    [(3, 1), (6, 1), (5, -1), (0, 1), (7, 1), (2, -1), (1, 1), (4, 1)],
    [(4, 1), (2, -1), (1, 1), (7, 1), (0, -1), (6, -1), (5, 1), (3, -1)],
    [(5, 1), (7, 1), (3, -1), (2, 1), (6, 1), (0, -1), (4, -1), (1, -1)],
    [(6, 1), (3, 1), (7, 1), (1, -1), (5, -1), (4, 1), (0, -1), (2, -1)],
    [(7, 1), (5, 1), (6, 1), (4, 1), (3, -1), (1, -1), (2, -1), (0, -1)],
]

# bit pattern (bottom-to-top (b1,b2,b3)) -> (blade_index, sign); natural convention
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

__version__ = "1.0.0"

__all__ = [
    "evaluate", "validate", "resolve", "format_state",
    "flip", "double_flip", "complement", "grade",
    "product", "combine", "count_blades",
    "BLADE_NAMES", "PROD_TABLE", "BITS_TO_STATE", "STATE_TO_BITS", "TRIGRAM_PINYIN",
    "Cl3CalcError", "UnknownOpError", "UnknownStateError", "BadArgumentError",
]


# --------------------------------------------------------------------------
# Typed errors
# --------------------------------------------------------------------------

class Cl3CalcError(Exception):
    """Base class for all typed errors raised by this package."""


class UnknownOpError(Cl3CalcError):
    """The op name is not one of the seven supported ops."""


class UnknownStateError(Cl3CalcError):
    """A state name could not be resolved."""


class BadArgumentError(Cl3CalcError):
    """An argument is missing, unknown, wrong-typed, or out of range."""


# --------------------------------------------------------------------------
# State resolution (exact POC-01 semantics)
# --------------------------------------------------------------------------

def resolve(name):
    """State name or (blade_index, sign) tuple -> (blade_index, sign).

    Names: canonical blade, sign-prefixed blade, trigram pinyin, or 3-bit
    pattern (bottom-to-top). Tuples pass through unchanged, so chained ops
    (e.g. flip(flip(...))) compose exactly like the reference implementation.
    """
    if isinstance(name, tuple):
        idx, sign = name
        if isinstance(idx, int) and isinstance(sign, int) and 0 <= idx <= 7 and sign in (-1, 1):
            return idx, sign
        raise ValueError(f"unknown state name: {name!r}")
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
    """(blade_index, sign) -> canonical display name, e.g. (6, -1) -> 'e31'."""
    idx, sign = state
    name = BLADE_NAMES[idx]
    return f"-{name}" if sign < 0 else name


# --------------------------------------------------------------------------
# Ops (exact POC-01 semantics)
# --------------------------------------------------------------------------

def flip(state, line):
    """Flip one line of the trigram (multiply by the corresponding basis vector).

    Only trigram states (resolvable into a canonical 3-bit pattern) may be flipped.
    Returns the canonical (blade_index, sign) of the result.
    """
    idx, sign = resolve(state)
    st = (idx, sign)
    if st not in STATE_TO_BITS:
        raise BadArgumentError(f"flip requires a trigram state, got {state!r}")
    bits = [int(b) for b in STATE_TO_BITS[st]]
    bits[line] = 1 - bits[line]
    return BITS_TO_STATE["".join(str(b) for b in bits)]


def double_flip(state, lines):
    """Flip several lines in order. Returns the canonical (blade_index, sign)."""
    out = state
    for ln in lines:
        out = flip(out, ln)
    return out


def _dual_state(idx, sign):
    """Hodge dual of a blade state (product by e123), normalized to the canonical trigram."""
    k, s = PROD_TABLE[idx][7]
    canon = next((st for st in BITS_TO_STATE.values() if st[0] == k), None)
    if canon is None:
        raise BadArgumentError(f"no canonical trigram for blade {k}")
    return canon


def complement(state):
    """Antipode of the cube = Hodge dual (multiply by e123), normalized to canonical trigram."""
    idx, sign = resolve(state)
    return _dual_state(idx, sign)


def grade(state):
    """Blade grade 0..3 of the resolved state."""
    idx, _sign = resolve(state)
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
    return (int(bu, 2) << 3) | int(bl, 2)


def count_blades(grade_k):
    """Number of blades of grade k: {0: 1, 1: 3, 2: 3, 3: 1}."""
    counts = {0: 1, 1: 3, 2: 3, 3: 1}
    if grade_k not in counts:
        raise BadArgumentError(f"grade_k must be an int in 0..3, got {grade_k!r}")
    return counts[grade_k]


# --------------------------------------------------------------------------
# Op contract table: op name -> required argument keys
# --------------------------------------------------------------------------

_OPS = {
    "flip": ("state", "line"),
    "double_flip": ("state", "lines"),
    "complement": ("state",),
    "grade": ("state",),
    "product": ("a", "b"),
    "combine": ("upper", "lower"),
    "count_blades": ("grade_k",),
}


def _is_int_not_bool(v):
    return isinstance(v, int) and not isinstance(v, bool)


def _check_state_arg(i, op_name, value):
    if not isinstance(value, str):
        raise BadArgumentError(
            f"op #{i}: {op_name!r}: argument 'state' must be a string, got {type(value).__name__}"
        )
    try:
        return resolve(value)
    except ValueError:
        raise UnknownStateError(f"op #{i}: unknown state name {value!r}") from None


def _check_trigram_state(i, op_name, value):
    st = _check_state_arg(i, op_name, value)
    if st not in STATE_TO_BITS:
        raise BadArgumentError(
            f"op #{i}: {op_name!r}: {value!r} is not a trigram state (flip/double_flip require one)"
        )
    return st


def validate(ops):
    """Strictly validate the ENTIRE op list; raise a typed error on the first violation."""
    if not isinstance(ops, list):
        raise BadArgumentError(f"ops must be a list, got {type(ops).__name__}")
    if not ops:
        raise BadArgumentError("ops must be a non-empty list")

    for i, op in enumerate(ops):
        if not isinstance(op, dict):
            raise BadArgumentError(f"op #{i}: expected a dict, got {type(op).__name__}")
        name = op.get("op")
        if name not in _OPS:
            raise UnknownOpError(f"op #{i}: unknown op {name!r} (expected one of {sorted(_OPS)})")

        required = _OPS[name]
        unknown = set(op) - {"op"} - set(required)
        if unknown:
            raise BadArgumentError(
                f"op #{i}: unknown argument(s) {sorted(unknown)} for op {name!r}"
            )
        missing = set(required) - set(op)
        if missing:
            raise BadArgumentError(
                f"op #{i}: missing argument(s) {sorted(missing)} for op {name!r}"
            )

        if name in ("flip", "double_flip"):
            _check_trigram_state(i, name, op["state"])
        elif name in ("complement", "grade"):
            _check_state_arg(i, name, op["state"])

        if name == "flip":
            line = op["line"]
            if not _is_int_not_bool(line) or not 0 <= line <= 2:
                raise BadArgumentError(
                    f"op #{i}: 'line' must be an int in 0..2, got {line!r}"
                )
        elif name == "double_flip":
            lines = op["lines"]
            if not isinstance(lines, list) or not all(
                _is_int_not_bool(ln) and 0 <= ln <= 2 for ln in lines
            ):
                raise BadArgumentError(
                    f"op #{i}: 'lines' must be a list of ints in 0..2, got {lines!r}"
                )
        elif name == "product":
            _check_state_arg(i, name, op["a"])
            _check_state_arg(i, name, op["b"])
        elif name == "combine":
            _check_state_arg(i, name, op["upper"])
            _check_state_arg(i, name, op["lower"])
        elif name == "count_blades":
            k = op["grade_k"]
            if not _is_int_not_bool(k) or not 0 <= k <= 3:
                raise BadArgumentError(
                    f"op #{i}: 'grade_k' must be an int in 0..3, got {k!r}"
                )


def evaluate(ops):
    """Validate the whole op list, then execute ops in order.

    Returns {"result": <value of the last op>, "steps": [{"op", "args", "result"}, ...]}.
    If ANY op fails validation, the whole call raises a typed error — no partial
    execution ever occurs.
    """
    validate(ops)
    steps = []
    result = None
    for i, op in enumerate(ops):
        name = op["op"]
        args = {k: v for k, v in op.items() if k != "op"}
        if name == "flip":
            value = flip(op["state"], op["line"])
        elif name == "double_flip":
            value = double_flip(op["state"], op["lines"])
        elif name == "complement":
            value = complement(op["state"])
        elif name == "grade":
            value = grade(op["state"])
        elif name == "product":
            value = product(op["a"], op["b"])
        elif name == "combine":
            value = combine(op["upper"], op["lower"])
        elif name == "count_blades":
            value = count_blades(op["grade_k"])
        else:  # unreachable after validate()
            raise UnknownOpError(f"op #{i}: unknown op {name!r}")
        steps.append({"op": name, "args": args, "result": value})
        result = value
    return {"result": result, "steps": steps}
