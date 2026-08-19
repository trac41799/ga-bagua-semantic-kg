# SDD — POC-09: Situation-Labeling as Generated Interpretation (hexagram-framed explanations)

**Status:** Pre-registered POC | **Effort:** 2 weeks | **Priority:** 6 in adoption roadmap
**Falsifiable question:** Do LLM explanations framed by a hexagram structure (upper/lower trigram + line-dynamics, with the LLM composing all interpretation prose) achieve higher comprehension/trust than plain explanations — with the framework used ONLY as a writing scaffold, never as a forced tag?

## 1. Pre-registration

| Item | Commitment |
|------|------------|
| Primary claim (proxy) | LLM-rater comprehension+trust (1–5, blended) of Arm B (hexagram-framed) ≥ Arm A (plain) + **0.5**, paired over 20 scenarios |
| Scenarios | 20 (situation, factual answer) pairs across 4 domains — frozen; each with a canonical plain explanation authored by the LLM in Arm A and a framed explanation in Arm B |
| Framing protocol | Arm B prompt: "Frame your explanation using the I-Ching hexagram structure: name an upper/lower trigram pair that captures the situation's dynamics, describe the line-change pattern, then explain in plain prose. The structure is a scaffold for writing; do not claim predictive meaning." |
| Rater | Separate LLM call, blind, 1–5 per scenario (comprehension + trust blended); shuffled order |
| Kill criterion | Δ < 0.5 → dies as a claim (framework stays pedagogical) |
| Honesty | Proxy labeled; human gate later. The prompt explicitly forbids predictive claims (structure as scaffold only) |

## 2. Scope
20 frozen scenarios, two explanation protocols, blind rater, runner, tests. No classification, no tags, no retrieval.

## 3. Interfaces
`scenarios.py` (20 frozen + factual answers), `prompts.py` (arm_a, arm_b, rater), `explain.py`, `rater.py`, `run_all.py`.

## 4. ACs (TDD: `../tdd/poc-09-situation-labeling-tdd.md`)
- 09.1 arm-B prompt contains the "scaffold, no predictive claims" constraint
- 09.2 arm-B outputs contain a trigram pair reference (parseable, failure counted not retried)
- 09.3 rater 1–5 parse; deterministic on cache
- 09.4 runner renders paired table + delta + verdict
- 09.5 tests green
