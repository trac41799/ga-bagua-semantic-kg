# POC-06 Verdict

- Arm A (free-form) mean comprehension: 4.90
- Arm B (3-aspect structured) mean comprehension: 5.00
- delta (B − A): +0.10 (bar ≥ +0.5) -> **FAIL**
- automated planted-delta coverage: A 0.85 | B 1.00

| id | domain | A | B | A-auto | B-auto |
|---|---|---|---|---|---|
| 1 | system | 5 | 5 | 1.00 | 1.00 |
| 2 | system | 5 | 5 | 1.00 | 1.00 |
| 3 | system | 5 | 5 | 1.00 | 1.00 |
| 4 | system | 5 | 5 | 0.67 | 1.00 |
| 5 | system | 4 | 5 | 0.33 | 1.00 |
| 6 | business | 5 | 5 | 1.00 | 1.00 |
| 7 | business | 5 | 5 | 1.00 | 1.00 |
| 8 | business | 4 | 5 | 0.33 | 1.00 |
| 9 | business | 5 | 5 | 1.00 | 1.00 |
| 10 | business | 5 | 5 | 1.00 | 1.00 |
| 11 | biology | 5 | 5 | 1.00 | 1.00 |
| 12 | biology | 5 | 5 | 1.00 | 1.00 |
| 13 | biology | 5 | 5 | 0.67 | 1.00 |
| 14 | biology | 5 | 5 | 1.00 | 1.00 |
| 15 | biology | 5 | 5 | 0.67 | 1.00 |
| 16 | governance | 5 | 5 | 1.00 | 1.00 |
| 17 | governance | 5 | 5 | 1.00 | 1.00 |
| 18 | governance | 5 | 5 | 1.00 | 1.00 |
| 19 | governance | 5 | 5 | 0.67 | 1.00 |
| 20 | governance | 5 | 5 | 0.67 | 1.00 |

*LLM-rater proxy (blind); human gate is a separate manual step.*
