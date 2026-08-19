# SDD — POC-13: Coverage-Rubric as Output-Quality Evaluation

**Status:** Pre-registered | **Purpose:** the Tier-2 extension — the validated 8-role checklist (Δ+1.15 as an audit) applied as an EVALUATION rubric: does a review conducted WITH the 8-role rubric catch known-bad outputs better than plain review? This makes the capability usable as an agent quality gate / guardrail skill.

## 1. Pre-registration
| Item | Commitment |
|------|------------|
| Primary claim (objective) | On 12 known-bad outputs (deficient plans/explanations, expert-authored, frozen), a reviewer WITH the 8-role rubric flags ≥ **8/12** as deficient (deficient = rubric coverage < threshold); plain review flags fewer — **rubric ≥ plain + 3** flagged |
| Secondary (specificity) | On 12 known-good outputs, rubric flags ≤ **2/12** as deficient (no false alarms) |
| Data | 24 outputs (12 good / 12 bad) derived from the POC-10 calibration families, re-authored as standalone outputs (frozen) |
| Metrics | Objective parse-based: rubric coverage = fraction of 8 roles addressed; plain review = 1–5 quality score with a pre-registered deficient threshold (≤2) |
| Protocol | Same LLM (deepseek-chat, temp 0); reviewer blind to good/bad; cached |
| Kill | Rubric detection ≤ plain detection, or specificity fails → dies as a guardrail claim |

## 2. Interfaces
`v2/pocs/poc-13-coverage-eval/`: `outputs.py` (24 frozen outputs + labels), `rubric.py` (8-role evaluation prompt + parse), `plain.py` (plain review prompt), `run_all.py` (paired detection rates, verdicts, cache, ledger).

## 3. ACs (TDD `poc-13-coverage-eval-tdd.md`)
- E1 24 outputs frozen (hash), 12/12 good/bad labels verifiable by construction (bad drop ≥2 of the 8 roles)
- E2 rubric parse: 8 bits strictly; failures counted
- E3 detection-rate math correct (hand case)
- E4 runner renders rubric vs plain detection + specificity + verdicts
- E5 determinism on cache; tests green
