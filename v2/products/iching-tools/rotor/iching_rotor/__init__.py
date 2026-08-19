"""POC-14: rotor transition algebra over Cl(3).

A rotor is a unit even-grade multivector of Cl(3) written as a 4-vector
``[s, b12, b23, b31]`` with s**2 + b12**2 + b23**2 + b31**2 == 1.

Blade order (ground truth, verbatim from ``cl3.PROD_TABLE``):
0=1, 1=e1, 2=e2, 3=e3, 4=e12, 5=e23, 6=e31, 7=e123.

Zero-LLM: pure deterministic floating-point math; no network, no LLM
imports, no external services. Semantics are assigned by the caller.

Operations
    compose(r1, r2)   rotor product: R1 R2 (geometric product), renormalized
    invert(r)         reverse of the even multivector (R~ = R^-1 for unit r)
    apply(r, blade)   sandwich a' = R a R~, returns canonical name ("e2", "-e1")
    distance(r1, r2)  norm of the 4-vector difference
    evaluate(ops)     strict batch executor (validate all, then run all)
    rotor(theta, plane) helper: rotation by theta in plane e12/e23/e31
"""

import math

# PROD_TABLE[i][j] = (result_index, sign) -- copied EXACTLY from
# v2/pocs/poc-01-combinatorial-scaffold/cl3.py (verified against v1 multivector.rs).
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

BLADE_NAMES = ["1", "e1", "e2", "e3", "e12", "e23", "e31", "e123"]

# even blade indices inside the 8-dim multivector
_EVEN = (0, 4, 5, 6)  # 1, e12, e23, e31  ->  rotor slots [s, b12, b23, b31]

# plane -> rotor slot for the -sin(theta/2) bivector component
_PLANE_SLOT = {"e12": 1, "e23": 2, "e31": 3}

_UNIT_TOL = 1e-9
_ROTOR_LEN = 4


def _geo(a8, b8):
    """Full 8-dim geometric product via PROD_TABLE (pure Python)."""
    out = [0.0] * 8
    for i in range(8):
        ai = a8[i]
        if ai == 0.0:
            continue
        row = PROD_TABLE[i]
        for j in range(8):
            bj = b8[j]
            if bj == 0.0:
                continue
            k, s = row[j]
            out[k] += ai * bj * s
    return out


def _to_8(r):
    """Embed rotor 4-vector into the 8-dim blade basis: [s, 0,0,0, b12, b23, b31, 0]."""
    return [r[0], 0.0, 0.0, 0.0, r[1], r[2], r[3], 0.0]


def _from_8(a8):
    """Extract rotor 4-vector [s, b12, b23, b31] from an 8-dim multivector."""
    return (a8[0], a8[4], a8[5], a8[6])


def _norm_sq(r):
    return r[0] * r[0] + r[1] * r[1] + r[2] * r[2] + r[3] * r[3]


def _renormalize(r):
    n = math.sqrt(_norm_sq(r))
    if n <= 1e-15:
        raise ValueError("rotor product collapsed to zero; cannot renormalize")
    return (r[0] / n, r[1] / n, r[2] / n, r[3] / n)


def compose(r1, r2):
    """Rotor product r1 * r2 (geometric product), renormalized."""
    r1 = _validate_rotor(r1, "compose.r1")
    r2 = _validate_rotor(r2, "compose.r2")
    return _renormalize(_from_8(_geo(_to_8(r1), _to_8(r2))))


def invert(r):
    """Reverse of the even multivector; for unit rotors R~ == R^-1."""
    r = _validate_rotor(r, "invert.r")
    return (r[0], -r[1], -r[2], -r[3])


def _parse_blade(blade_name):
    """Canonicalize a blade name (optionally signed) to (sign, name)."""
    if not isinstance(blade_name, str):
        return None
    sign = 1.0
    name = blade_name
    if name[:1] in ("+", "-"):
        sign = -1.0 if name[0] == "-" else 1.0
        name = name[1:]
    if name not in BLADE_NAMES:
        return None
    return (sign, name)


def apply(r, blade_name):
    """Sandwich a' = R a R~; returns canonical blade name with sign, e.g. "e2", "-e1".

    ``blade_name`` may be a canonical name (``e1``) or a signed name
    (``-e1``, ``+e1``) as returned by this function.

    Exact-or-error: the answer is a canonical name only when the sandwich
    result is a single blade to within 1e-12 (true for axis-aligned cases
    such as theta = pi/2 or pi in one plane). A general rotation maps a
    blade to a mixed multivector; that is not representable as one name, so
    ValueError is raised instead of silently naming the dominant component.
    """
    r = _validate_rotor(r, "apply.r")
    return _apply_validated(r, blade_name)


def _apply_validated(r, blade_name):
    """Apply a rotor after its rotor value has already been validated."""
    parsed = _parse_blade(blade_name)
    if parsed is None:
        raise ValueError("apply.blade: unknown blade %r" % (blade_name,))
    sign, name = parsed
    i = BLADE_NAMES.index(name)
    a8 = [0.0] * 8
    a8[i] = sign
    R = _to_8(r)
    Rt = list(R)
    Rt[4] = -Rt[4]
    Rt[5] = -Rt[5]
    Rt[6] = -Rt[6]
    Rt[7] = -Rt[7]
    out = _geo(_geo(R, a8), Rt)
    idx = max(range(8), key=lambda k: abs(out[k]))
    residual = max((abs(x) for x in out if x is not out[idx]), default=0.0)
    if residual > 1e-12:
        raise ValueError("apply: sandwich result is not a pure blade "
                         "(dominant %s, residual %.3e)" % (BLADE_NAMES[idx], residual))
    sign = "-" if out[idx] < 0.0 else ""
    return sign + BLADE_NAMES[idx]


def distance(r1, r2):
    """Euclidean norm of the difference of the two rotor 4-vectors."""
    r1 = _validate_rotor(r1, "distance.r1")
    r2 = _validate_rotor(r2, "distance.r2")
    return math.sqrt((r1[0] - r2[0]) ** 2 + (r1[1] - r2[1]) ** 2
                     + (r1[2] - r2[2]) ** 2 + (r1[3] - r2[3]) ** 2)


def _validate_rotor(v, path):
    """Strict rotor check: list/tuple of 4 finite floats, unit norm within 1e-9."""
    if not isinstance(v, (list, tuple)):
        raise ValueError("%s: rotor must be a list of 4 numbers, got %r" % (path, type(v).__name__))
    if len(v) != _ROTOR_LEN:
        raise ValueError("%s: rotor must have exactly 4 components, got %d" % (path, len(v)))
    out = []
    for k, x in enumerate(v):
        if isinstance(x, bool) or not isinstance(x, (int, float)):
            raise ValueError("%s[%d]: component %r is not a real number" % (path, k, x))
        f = float(x)
        if not math.isfinite(f):
            raise ValueError("%s[%d]: component %r is not finite" % (path, k, x))
        out.append(f)
    n = math.sqrt(_norm_sq(out))
    if abs(n - 1.0) > _UNIT_TOL:
        raise ValueError("%s: not a unit rotor (norm %.9g)" % (path, n))
    return tuple(out)


def _validate_operation(op, idx, allowed_keys):
    """Validate one batch operation and return normalized execution arguments."""
    path = "ops[%d]" % idx
    if not isinstance(op, dict):
        raise ValueError("%s: op must be a dict, got %r" % (path, type(op).__name__))
    kind = op.get("op")
    if kind in allowed_keys:
        extra = [k for k in op if k not in allowed_keys[kind]]
        if extra:
            raise ValueError("%s: unexpected key(s) %r for op %r" % (path, extra, kind))
    if kind == "compose":
        r1 = _validate_rotor(op.get("r1"), path + ".r1")
        r2 = _validate_rotor(op.get("r2"), path + ".r2")
        return kind, (r1, r2)
    if kind == "invert":
        return kind, (_validate_rotor(op.get("r"), path + ".r"),)
    if kind == "apply":
        r = _validate_rotor(op.get("r"), path + ".r")
        blade = op.get("blade")
        if _parse_blade(blade) is None:
            raise ValueError("%s.blade: unknown blade %r" % (path, blade))
        # Check the exact-or-error result before any batch operation runs.
        _apply_validated(r, blade)
        return kind, (r, blade)
    if kind == "distance":
        r1 = _validate_rotor(op.get("r1"), path + ".r1")
        r2 = _validate_rotor(op.get("r2"), path + ".r2")
        return kind, (r1, r2)
    raise ValueError("%s.op: unknown op %r (expected compose/invert/apply/distance)"
                     % (path, kind))


def _execute_operation(kind, args):
    """Execute one operation after the complete batch has been validated."""
    if kind == "compose":
        return compose(*args)
    if kind == "invert":
        return invert(*args)
    if kind == "apply":
        return apply(*args)
    return distance(*args)


def evaluate(ops):
    """Strict batch execution.

    ops: list of {"op": ...} dicts with ops
        compose(r1, r2), invert(r), apply(r, blade), distance(r1, r2).
    All-or-nothing: every op is validated (and, for apply, checked for a
    pure-blade result) before results are returned; any error raises
    ValueError and no partial results are ever produced.
    """
    if not isinstance(ops, list):
        raise ValueError("ops: must be a list of op dicts, got %r" % type(ops).__name__)

    allowed_keys = {"compose": ("op", "r1", "r2"),
                    "invert": ("op", "r"),
                    "apply": ("op", "r", "blade"),
                    "distance": ("op", "r1", "r2")}
    validated_ops = [_validate_operation(op, idx, allowed_keys)
                     for idx, op in enumerate(ops)]
    return [_execute_operation(kind, args) for kind, args in validated_ops]


def rotor(theta, plane):
    """Helper: unit rotor for rotation by ``theta`` radians in plane e12/e23/e31.

    Returns ``[cos(theta/2), -sin(theta/2) in the matching slot]``,
    i.e. R = cos(theta/2) - sin(theta/2) * plane-bivector.
    """
    if plane not in _PLANE_SLOT:
        raise ValueError("rotor: unknown plane %r (expected e12/e23/e31)" % (plane,))
    slot = _PLANE_SLOT[plane]
    r = [0.0, 0.0, 0.0, 0.0]
    r[0] = math.cos(theta / 2.0)
    r[slot] = -math.sin(theta / 2.0)
    return tuple(r)


IDENTITY = (1.0, 0.0, 0.0, 0.0)

__all__ = [
    "compose", "invert", "apply", "distance", "evaluate", "rotor",
    "PROD_TABLE", "BLADE_NAMES", "IDENTITY",
]
