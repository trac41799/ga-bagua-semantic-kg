# POC-14 — Rotor Transition Algebra (`iching_rotor`)

Exact, composable, invertible state-transition algebra for agents (Tier-3
candidate), built on the verified Cl(3) geometric-algebra core
(`cl3.py`, copied verbatim from
`v2/pocs/poc-01-combinatorial-scaffold/`; `PROD_TABLE` is ground truth).
Rotors act as transitions: composition = rotor product, inverse = reverse,
distance = norm of difference. Semantics are assigned by the LLM, never
derived.

Spec: `v2/docs/specs/poc-14-rotor-state-algebra-spec.md`
TDD:  `v2/docs/tdd/poc-14-rotor-state-algebra-tdd.md`

## Contract

A rotor is a unit even-grade multivector `[s, b12, b23, b31]` of Cl(3),
`s^2 + b12^2 + b23^2 + b31^2 = 1` (unit-norm check tolerance `1e-9`).

| Op | Signature | Meaning |
|----|-----------|---------|
| `compose(r1, r2)` | `([4], [4]) -> [4]` | rotor product `R1 R2` via Cl(3) geometric product, renormalized |
| `invert(r)` | `([4]) -> [4]` | reverse of the even multivector (`R~ == R^-1` for unit rotors) |
| `apply(r, blade_name)` | `([4], str) -> str` | sandwich `a' = R a R~`; canonical name with sign, e.g. `"e2"`, `"-e1"`; accepts signed names (`"-e2"`); exact-or-error (see below) |
| `distance(r1, r2)` | `([4], [4]) -> float` | norm of the 4-vector difference |
| `evaluate(ops)` | `([dict]) -> list` | strict batch executor; validate all ops first, then execute (no partial execution) |
| `rotor(theta, plane)` | `(float, str) -> [4]` | helper: `[cos(theta/2), -sin(theta/2) in slot]` for plane `e12`/`e23`/`e31` |

Blade names: `1`, `e1`, `e2`, `e3`, `e12`, `e23`, `e31`, `e123`.

## Usage

```python
from iching_rotor import apply, compose, invert, distance, evaluate, rotor

R = rotor(3.141592653589793 / 2, "e12")
apply(R, "e1")                    # "e2"
apply(rotor(3.141592653589793, "e12"), "e1")   # "-e1"

R2 = compose(rotor(1.0, "e23"), rotor(0.5, "e31"))
apply(invert(R2), apply(R2, "e3"))   # "e3"

evaluate([
    {"op": "compose", "r1": list(R), "r2": list(R)},
    {"op": "invert",  "r": list(R)},
    {"op": "apply",   "r": list(R), "blade": "e1"},
    {"op": "distance", "r1": list(R), "r2": list(R)},
])  # [4-vector, 4-vector, str, 0.0]
```

Strict validation: rotors must be lists of 4 finite real numbers with unit
norm (`|norm - 1| <= 1e-9`); unknown op, unknown blade, missing/extra keys,
or malformed rotors raise `ValueError`. `evaluate` is all-or-nothing: it
completes a full validation pass, including exact-or-error `apply` checks,
before executing any operation; any error means no results are returned.

`apply` is **exact-or-error**: it returns a canonical name only when the
sandwich result is a single blade to within `1e-12` (e.g. `theta = pi/2`
or `pi` in one plane, and the `1`/`e123` invariants, which hold for every
rotor). A general rotation maps a blade to a mixed multivector, which
cannot be represented by one name; `apply` raises `ValueError` instead of
naming the dominant component.

## Tests

```bash
python -m pytest tests/ -q
```

Covers TDD R1 (hand cases: π/2 e12 maps e1→e2, π maps e1→−e1, invert
round-trip, distance identity = 0), R2 (100 seeded random rotor chains,
depth ≤ 10: unit norm ≤ 1e-12, inverse round-trip ≤ 1e-12, associativity,
all-or-nothing `evaluate` pipelines), R3 (strict validation, no partial
execution, `apply` exact-or-error), R6 (import audit: no LLM or network
imports; `PROD_TABLE` matches the `cl3.py` ground truth; README zero-LLM
statement).

## Zero-LLM statement

This package makes **zero LLM calls**: it is deterministic floating-point
math over the verified `PROD_TABLE` multiplication table, with no network
access, no HTTP, no model imports, and no hidden side effects. Every
operation is reproducible to within `1e-12` (validation tolerance `1e-9`).
