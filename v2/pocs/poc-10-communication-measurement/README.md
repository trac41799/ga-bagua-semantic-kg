# POC-10 — Communication Boundary-Test (measurement of claim d)

**Status: FRESH CLEAN EVIDENCE PENDING RERUN. Historical evidence is preserved below; the clean protocol is now wired into new runs.**

## Clean-Protocol Status

New runs use `state_diff.comprehension.no_ground_truth_in_question_v1`: each question names only an aspect, while the generated summary is supplied separately as reader context. Before/after/planted values remain evaluator-held and are not interpolated into question text.

The clean runner writes `output/verdict-clean-v1.md` and `data/cache/responses-clean-v1.json`. It never overwrites the historical `output/verdict.md` or `data/cache/responses.json`. QA results are computed from exact correct/total counts; the `+0.10` bar is compared as an exact `Fraction` derived from `Decimal("0.10")`, not from the displayed rounded delta.

**Fresh clean-protocol LLM evidence: PENDING RERUN.** The simulated runner used by tests is not evidence and does not call a real LLM.

## Historical Verdict (2026-08-08, real LLM, 267s, 283 calls)

### 1. Instrument calibration — FAIL (the decisive finding)
| task | known-good | known-bad | delta | bar |
|------|-----------|-----------|-------|-----|
| 06 summaries | 5.00 | 4.67 | +0.33 | ≥1.0 |
| 09 explanations | 4.83 | 4.33 | +0.50 | ≥1.0 |
| 07 naming | 4.17 | 4.17 | +0.00 | ≥1.0 |

**The LLM 1–5 rater cannot discriminate expert-authored known-good from known-bad outputs.**
Per pre-registration, **all prior 1–5 rater verdicts (POC-06 Δ+0.10, POC-09 Δ+0.00, POC-07 naming +0.33) are UNINTERPRETABLE — instrument failure, not phenomenon absence.**

### 2. Historical objective comprehension-QA (POC-06, planted ground truth) — INCONCLUSIVE pending clean rerun
| Arm | QA accuracy |
|-----|-------------|
| A free-form | 0.900 |
| B structured (3-aspect) | **1.000** |
| delta | **+0.100 (bar ≥ +0.10)** |

The historical run recorded perfect comprehension vs 90% for free-form, but it used planted ground truth and is not sufficient for current promotion. The calibration failure is retained; the clean no-leakage rerun and replication remain pending.

### 3. Answer conveyance (POC-09, objective) — FAIL (true null)
A 0.597 vs B 0.518, delta **−0.078** (non-inferiority bar −0.05). Hexagram-framed explanations slightly REDUCE factual conveyance. **The POC-09 phenomenon is genuinely null/slightly negative** — 20/20 framing compliance with no benefit and a small factual cost.

### 4. Calibrated rater deltas — N/A (instrument failed)
06: +0.05, 09: +0.00, 07: +0.33 — meaningless without calibration.

## Provisional boundary reading (clean rerun pending)
- **Not** "generation vs communication" — **objective vs subjective instruments**.
- POC-06: historical objective signal, invisible to raters; promotion remains pending until the clean protocol reruns.
- POC-09: genuine null → prior FAIL was correct but for unmeasurable reasons.
- Subjective clarity/trust claims (POC-06/09/07-naming) remain **human-gate-pending**; LLM raters are proven unfit (calibration).

## Run
```
python -m pytest tests/ -q      # focused POC-10 tests; simulated only
python run_all.py --real        # fresh clean run; writes versioned artifact/cache
```
Reports: `output/verdict-clean-v1.md`. The historical report remains `output/verdict.md`.
