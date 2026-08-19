# B2 — reframe (production iching_reframe vs POC-08 baseline)

- date: 2026-08-20 | model: deepseek-v4-flash (temperature 0) | cache: 520 responses | new calls this run: 0
- bars: diversity >= 0.403 (POC-08 0.453 - 0.05), coherence >= 3.5 (POC-08 3.75), zero PRODUCTION defects

## Per-statement

| id | positions | diversity | coherence |
|---|---|---|---|
| s01 | 8 | 0.686 | 4 |
| s02 | 8 | 0.591 | 2 |
| s03 | 8 | 0.582 | 3 |
| s04 | 8 | 0.662 | 4 |
| s05 | 8 | 0.504 | 4 |
| s06 | 8 | 0.320 | 5 |
| s07 | 8 | 0.385 | 4 |
| s08 | 8 | 0.413 | 3 |
| s09 | 8 | 0.406 | 5 |
| s10 | 8 | 0.476 | 5 |
| s11 | 8 | 0.800 | 4 |
| s12 | 8 | 0.407 | 4 |
| s13 | 8 | 0.454 | 5 |
| s14 | 8 | 0.637 | 5 |
| s15 | 8 | 0.351 | 4 |
| s16 | 8 | 0.508 | 4 |
| s17 | 8 | 0.679 | 5 |
| s18 | 8 | 0.622 | 4 |
| s19 | 8 | 0.597 | 4 |
| s20 | 8 | 0.684 | 5 |

## Means and bars

| metric | value | bar | verdict |
|---|---|---|---|
| reframe_diversity | 0.538 | >= 0.403 | PASS |
| reframe_coherence | 4.15 | >= 3.5 | PASS |
| reframe_production_defects | 0 | 0 | PASS |
| **overall** | | | **PASS** |

## Defects

- none (zero PRODUCTION defects)

## Method

- Production `reframe(statement, llm)` per frozen statement; defect = not exactly 8 positions, non-distinct states, or any error.
- Diversity: mean pairwise 1-cosine over the 8 rubric-encoded reframes per statement (same 8-role encoder as POC-08), averaged across the 20 statements.
- Encoder source: v2/pocs/poc-08-reframing-v2/rubric.py fallback (common.rubric_encode path v2/pocs/path-d/rubric.py missing; encoder prompt+parser identical)
- Coherence: exact POC-08 judge prompt, 1-5, one call per statement, mean over the 20 (0 excluded on protocol failure).
- All responses cached in bench/.cache_reframe.json; re-runs reuse the cache and reproduce identical numbers (temperature 0).

