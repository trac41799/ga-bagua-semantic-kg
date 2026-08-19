# B3 — state_diff benchmark (real LLM vs frozen POC-06)

- date: 2026-08-13, model: deepseek-chat, temperature: 0, transitions: 20

## Per-transition compliance + coverage

| tid | domain | compliance | coverage | defects |
|---|---|---|---|---|
| 1 | system | PASS | 1.0000 | - |
| 2 | system | PASS | 1.0000 | - |
| 3 | system | PASS | 1.0000 | - |
| 4 | system | PASS | 1.0000 | - |
| 5 | system | PASS | 1.0000 | - |
| 6 | business | PASS | 1.0000 | - |
| 7 | business | PASS | 1.0000 | - |
| 8 | business | PASS | 1.0000 | - |
| 9 | business | PASS | 1.0000 | - |
| 10 | business | PASS | 1.0000 | - |
| 11 | biology | PASS | 1.0000 | - |
| 12 | biology | PASS | 1.0000 | - |
| 13 | biology | PASS | 1.0000 | - |
| 14 | biology | PASS | 1.0000 | - |
| 15 | biology | PASS | 1.0000 | - |
| 16 | governance | PASS | 1.0000 | - |
| 17 | governance | PASS | 1.0000 | - |
| 18 | governance | PASS | 1.0000 | - |
| 19 | governance | PASS | 1.0000 | - |
| 20 | governance | PASS | 1.0000 | - |

**Mean coverage: 1.0000** | compliant 20/20

## Verdicts

| metric | value | bar | verdict |
|---|---|---|---|
| statediff compliance | 20/20 | 20/20 (zero defects) | PASS |
| statediff coverage (mean planted-delta) | 1.0000 | >= 0.95 | PASS |
| statediff defects | 0 | 0 | PASS |

## Defects

None.
