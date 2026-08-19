# POC-12 — interaction-spectrum XAI (iching_xai)

Deterministic Walsh–Hadamard / contrast-transform interaction recovery, packaging the
POC-07 validated math as a library. **Zero-LLM**: the math path is pure algebra on the
standard library — no model calls, no network, no external dependencies.

## Contract

- `interaction_spectrum(points, values) -> dict[int, float]`
  - `points`: list of ±1 vectors (a 2^k design); `values`: responses, same length.
  - Returns `{subset_mask: coefficient}`; for each subset S,
    `c_S = mean over runs r of sign(S, r) * y_r` with `sign(S, r) = prod(r[i] for i in S)`
    and `mask = sum(1 << i for i in S)`.
  - For a polynomial response on the full 2^k design the planted coefficients are
    recovered exactly (float error ≈ 1e-15).
- `identify(spectrum, tol=1e-6) -> list[int]` — subset masks with `|c| > tol`, sorted.
- Strict validation, else `XAIValidationError` (a `ValueError` subclass): every point
  entry must be exactly ±1 (bools rejected), all vectors same length, all non-empty,
  `len(values) == len(points)`, values numeric; `identify` also validates the spectrum
  mapping and a non-negative numeric `tol`.

## Usage

```python
import itertools
from iching_xai import interaction_spectrum, identify

k = 3
points = list(itertools.product((-1.0, 1.0), repeat=k))
values = [2.5 * x[0] * x[1] + 0.9 * x[0] * x[1] * x[2] for x in points]

spec = interaction_spectrum(points, values)   # {mask: coefficient}
assert spec[3] == 2.5                          # {0,1}  -> mask 3
assert spec[7] == 0.9                          # {0,1,2} -> mask 7

identified = identify(spec)                    # [3, 7]
```

## Verification

`python -m pytest tests/ -q` — 21 passed (X1 exact recovery ≤1e-9, 3/3 subsets,
0 false positives; X2 strict validation errors; X5 stdlib-only import audit).

Recovered spectrum on the POC-07 planted function (6 inputs; {0,1}:2.5,
{3,4}:−1.75, {0,1,2}:0.9):

```
mask= 3 (bits [0, 1])    c=+2.500000000000000
mask= 7 (bits [0, 1, 2]) c=+0.899999999999999
mask=24 (bits [3, 4])    c=-1.750000000000000
identified masks: [3, 7, 24]
max |err| vs planted: 1.21e-15
```

## Zero-LLM statement

This package imports only the Python standard library (`itertools`, `numbers`).
It contains no model-client code, no network I/O, and no dependency on any external
service. Results are deterministic.
