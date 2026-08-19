# SDD — POC-10: Communication Boundary-Test (the thesis's prescribed measurement)

**Status:** Pre-registered | **Purpose:** Measure claim (d) — communication/comprehension — with instruments that can actually detect effects, resolving the thesis's open boundary question: *is the boundary generation-vs-communication or objective-vs-subjective metrics?*

## 1. Pre-registration

| Item | Commitment |
|------|------------|
| Instrument check (calibration) | The LLM rater must discriminate expert-authored KNOWN-GOOD vs KNOWN-BAD outputs by **≥ 1.0 mean** on 6 frozen pairs per task (3 tasks: 06 summaries, 09 explanations, 07 naming). If calibration FAILS → the instrument is unfit; prior 1–5 FAILs (POC-06/09/07-naming) are declared **uninterpretable (instrument failure)**, not phenomenon failures |
| Primary claim (objective comprehension, 06) | Reader answers comprehension questions from the summary ALONE; accuracy vs planted ground truth. Arm B (structured) ≥ Arm A (free-form) + **0.10** |
| Primary claim (09, calibrated rater) | Arm B (hexagram-framed) ≥ Arm A (plain) + **0.5** on the CALIBRATED rater |
| Primary claim (07 naming, calibrated rater) | Bagua-named ≥ numbered + **0.5** clarity on the CALIBRATED rater |
| Secondary (09, objective) | Answer-conveyance: framed explanations must not lose factual content vs plain (non-inferiority ≥ −0.05) |
| Kill/verdict rules | Calibration FAIL → report instrument failure, no phenomenon verdicts. Calibration PASS + claims fail → phenomenon failures (measured, not assumed) |
| Data | Reuses cached POC-06/09 responses (read-only); POC-07 explanations regenerated (deterministic strings) |

## 2. Scope
Calibration sets (6 per task, expert-authored, frozen), comprehension-QA reader (06: 3 questions/transition × 20 × 2 arms; 09: 1 question × 20 × 2 arms), calibrated rater (3 tasks), verdicts, tests. No new phenomena; this is a measurement POC.

## 3. Interfaces
`calibration.py` (frozen pairs + rater), `qa.py` (question generation + reader + exact scoring), `run_all.py` (orchestrates, renders `output/verdict.md`), `llm_client.py` (pattern), tests.

## 4. ACs (TDD: `../tdd/poc-10-communication-measurement-tdd.md`)
- 10.1 calibration sets frozen (hash), 6 per task, good/bad verifiable (bad versions drop ≥2 planted facts)
- 10.2 QA questions generated from planted deltas (automated check: question contains aspect, values known)
- 10.3 reader scoring exact (contains-check on planted before/after values)
- 10.4 runner renders: calibration verdict, QA table+delta, calibrated-rater table+deltas, final verdicts
- 10.5 determinism on cache; tests green
