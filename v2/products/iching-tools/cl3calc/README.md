# iching_cl3calc — POC-11 Cl3 Calculator (structured-op evaluation)

The POC-01 fix: the Cl3 calculator was proven exact (0 execution failures in
POC-01); the failure was free-form plan parsing. This tool exposes the same
exact algebra as a **structured op contract**: the LLM (or any caller) constructs
a JSON list of typed ops; the tool validates the whole list strictly and
executes deterministic, pure algebra. No LLM code anywhere in the tool path.

Spec: `v2/docs/specs/poc-11-cl3-calculator-mcp-spec.md`
TDD:  `v2/docs/tdd/poc-11-cl3-calculator-mcp-tdd.md`

---

## Contract

```python
from iching_cl3calc import evaluate, validate

evaluate(ops: list[dict]) -> {"result": <value of last op>, "steps": [{"op", "args", "result"}, ...]}
validate(ops: list[dict]) -> None   # raises a typed error on the first violation
```

**STRICT**: the ENTIRE op list is validated before any op executes. If ANY op
fails (unknown op, unknown state name, missing/extra/wrong-typed/out-of-range
argument), the whole call raises a typed error — there is **no partial
execution** (proven in tests by a call-spy that never fires).

## Ops

| op            | args                          | result type / example                       |
|---------------|-------------------------------|---------------------------------------------|
| `flip`        | `state: str`, `line: 0..2`    | `(blade_index, sign)` e.g. `(3, 1)`         |
| `double_flip` | `state: str`, `lines: [0..2]` | `(blade_index, sign)` (sequential flips)    |
| `complement`  | `state: str`                  | `(blade_index, sign)` — Hodge dual (×e123), normalized to the canonical trigram (Kan↔Li, Kun↔Qian, …) |
| `grade`       | `state: str`                  | `int` 0..3                                  |
| `product`     | `a: str`, `b: str`            | canonical string: `"e123"`, `"-e1"`, `"1"`  |
| `combine`     | `upper: str`, `lower: str`    | `int` 0..63 (`upper<<3 | lower`)            |
| `count_blades`| `grade_k: 0..3`               | `int` `{0:1, 1:3, 2:3, 3:1}`               |

## State names

Same resolution as the verified POC-01 calculator:

* blades `1, e1, e2, e3, e12, e23, e31, e123` — sign-prefixed allowed (`-e1`)
* trigram pinyin `kun, gen, kan, xun, zhen, li, dui, qian`
* 3-bit patterns (bottom-to-top): `000`…`111`, e.g. `"101"` → Li = `(6, -1)`

`flip`/`double_flip` require a trigram state (one that resolves into a canonical
3-bit pattern); everything else accepts any resolvable state.

## Errors (typed)

* `Cl3CalcError` — base class (subclasses `Exception`)
* `UnknownOpError` — op name not in the seven above
* `UnknownStateError` — state name does not resolve
* `BadArgumentError` — missing/unknown key, wrong type, out-of-range value,
  non-trigram state passed to `flip`

Error messages carry the failing op index (e.g. `op #1: …`).

## Usage

```python
from iching_cl3calc import evaluate

ops = [
    {"op": "flip", "state": "kan", "line": 1},      # Kan -> ? 
    {"op": "complement", "state": "e1"},            # Zhen -> Xun
    {"op": "product", "a": "-e31", "b": "e2"},      # (-e31)·e2 = e1
    {"op": "combine", "upper": "qian", "lower": "kun"},  # 56
    {"op": "count_blades", "grade_k": 2},           # 3
]
print(evaluate(ops))
# {'result': 3, 'steps': [{'op': 'flip', 'args': {'state': 'kan', 'line': 1}, 'result': (4, 1)}, ...]}
```

## Zero-LLM statement

`iching_cl3calc/` is pure deterministic algebra: **zero imports** (no LLM
client, no network, no third-party libraries — not even numpy), and the
import audit test (`test_c6_import_audit`) enforces that the package source
contains no import statements and no LLM/network/dependency tokens at all.

## Ground truth & parity

* `reference/` holds **verbatim copies** of the verified POC-01 implementation
  (`calculator.py`, `cl3.py` — `PROD_TABLE` is ground truth). Tests load it **by
  file path**; the package never imports across directories.
* Parity: 100 seeded random op sequences (10 ops each, all 7 op types) must
  evaluate identically to the reference (numeric tolerance 1e-12).

## Testing

```bash
cd v2/products/iching-tools/cl3calc
python -m pytest tests/ -q          # C1–C3 + C6, ~130+ cases
python -m pytest tests/test_cl3calc.py::test_c3_sample_10op_comparison -s   # sample parity trace
```
