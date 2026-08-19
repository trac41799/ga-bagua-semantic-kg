# POC-10 Verdict — Communication Boundary-Test

## 1. Instrument calibration

| task | good | bad | delta |
|---|---|---|---|
| 06 | 5.00 | 4.67 | +0.33 |
| 09 | 4.83 | 4.33 | +0.50 |
| 07 | 4.17 | 4.17 | +0.00 |

**Instrument: FAIL (rater unfit)**

## 2. Objective comprehension-QA (POC-06, planted ground truth)

- Arm A: 0.900 | Arm B: 1.000 | delta +0.100 (bar ≥ +0.10) -> **FAIL**
## 3. Answer conveyance (POC-09, objective)

- Arm A: 0.597 | Arm B: 0.518 | delta -0.078 (non-inferiority ≥ −0.05) -> **FAIL**
## 4. Calibrated rater on real pairs

| task | A | B | delta | bar | verdict |
|---|---|---|---|---|---|
| 06 | 4.65 | 4.70 | +0.05 | +0.5 | N/A (instrument) |
| 09 | 4.90 | 4.90 | +0.00 | +0.5 | N/A (instrument) |
| 07 | 4.00 | 4.33 | +0.33 | +0.5 | N/A (instrument) |

## Boundary resolution

- **Instrument failure**: the LLM rater cannot discriminate known-good from known-bad outputs. All prior 1–5 rater verdicts (POC-06/09/07-naming) are UNINTERPRETABLE. The objective QA is the only valid measurement here.
