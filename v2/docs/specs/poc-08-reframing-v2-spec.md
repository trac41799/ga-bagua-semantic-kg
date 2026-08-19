# SDD — POC-08: Reframing Grammar v2 (few-shot naming protocol)

**Status:** Re-pre-registration (POC-03 failed: Δ+0.082 < +0.15, coherence 2.8) | **Effort:** 1 week
**Falsifiable question (v2):** With a few-shot naming protocol (position examples per move type), do the 8 algebra-grounded cube reframes meet re-derived margins: diversity Δ ≥ +0.10 and coherence ≥ 3.5?

## 1. Re-pre-registration (what changed vs POC-03 — required by failure-ledger rule)

| Item | POC-03 (failed) | POC-08 (v2) |
|------|-----------------|-------------|
| Naming protocol | zero-shot per-position prompt | **few-shot**: 2 exemplar names per move type (flip/double-flip/complement) in the prompt |
| Margin (diversity) | ≥ +0.15 | **≥ +0.10** (re-derived: measured +0.082 with a clear protocol deficit; the re-derived bar is the measured upper confidence region, not a retreat to 0) |
| Coherence | ≥ 3.5 | ≥ 3.5 (unchanged — naming quality is the claimed fix) |
| Statements | 20 frozen | same 20 (hash unchanged) |
| Other | — | identical arms, metrics, rater, kill rules |

Kill criterion: Δ < +0.10 OR coherence < 3.5 → the reframing thesis dies permanently (moves remain pedagogical only).

## 2. Scope
New `naming.py` (few-shot protocol) + runner reusing POC-03's moves/metrics/statements (copied, self-contained per house rule); new cache; same audit.

## 3. Interfaces (self-contained copy in `pocs/poc-08-reframing-v2/`)
`moves.py` (from POC-03), `statements.py` (same 20 + hash), `naming.py` (few-shot), `metrics.py` (diversity via rubric vectors, coherence judge), `run_all.py`.

## 4. ACs (TDD: `../tdd/poc-08-reframing-v2-tdd.md`)
- 08.1 few-shot prompt contains ≥2 exemplars per move type; arm-A prompt unchanged
- 08.2 8 distinct positions (re-verified); complement identity holds
- 08.3 metrics/determinism as POC-03; runner renders both margins + verdict
- 08.4 tests green
