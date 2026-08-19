# POC-13 — Coverage-Rubric as Output-Quality Evaluation

**Status: VALIDATED — PASS.** The 8-role checklist works as an EVALUATION/guardrail tool, not just an audit.

## Verdict (real LLM, deepseek-chat, 24 frozen outputs: 12 known-good / 12 known-bad)

| Metric | Rubric (8-role) | Plain review | Bar | |
|--------|-----------------|--------------|-----|---|
| Bad-output detection | **8/12 (0.67)** | 3/12 (0.25) | ≥ 8/12 AND ≥ plain + 3/12 | **PASS** |
| False alarms on good outputs | **0/12** | 0/12 | ≤ 2/12 | **PASS** |

The rubric-evaluation flags deficient outputs **2.7× more often than plain review, with zero false alarms** — the roles as an output-quality checklist catch what generic review misses.

## What this enables
The validated capability as an **agent guardrail skill**: evaluate any generated plan/answer against the 8 roles (reception, trigger, flow, constraint, clarity, influence, balance, generation) before accepting it. Packaged in `products/iching-tools/skills/coverage-audit/SKILL.md` (audit form) — this POC validates the evaluation form.

## Run
```
python run_all.py --sim   # pipeline smoke
python run_all.py --real  # validated run (48 calls, cached)
```
Outputs: `output/verdict.md`. Frozen outputs: `outputs.py` + `outputs.sha256` (construction check: bad outputs verifiably cover < 4 roles).
