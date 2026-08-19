# SDD — POC-05: Coverage-Rubric Prompting (I-Ching roles as a generative completeness checklist)

**Status:** Pre-registered POC | **Effort:** 1 week | **Priority:** 2 in adoption roadmap
**Falsifiable question:** Do LLM plans drafted under an 8-role coverage checklist (receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative) miss fewer critical aspects than free-form plans?

## 1. Pre-registration

| Item | Commitment |
|------|------------|
| Primary claim (proxy) | Mean missing-aspect count on a fixed 6-aspect audit rubric: Arm B (checklist) ≤ Arm A (free-form) − **1.0** missing aspects, paired over 20 planning tasks, LLM-rater proxy |
| Tasks | 20 planning tasks (5 per domain: product feature, incident response, policy, research plan) — frozen before any run |
| Audit rubric | 6 aspects (constraint handling, resource flow, stakeholder reception, risk balance, clarity of steps, initiation triggers) — same for both arms; rater blind to arm |
| Protocol | Arm A: "write a plan". Arm B: same prompt + the 8 roles as coverage questions ("does your plan address X for each role..."). Both same model (deepseek-chat, temp 0). Rater: separate LLM call scoring each plan on the 6-aspect rubric (1 = aspect missing, 0 = present), shuffled order |
| Kill criterion | Delta < 1.0 missing aspects → POC-05 dies as a claim |
| Honesty | Human gate (n≥5 raters) later, after proxy passes; proxy explicitly labeled |

## 2. Scope
Prompt protocols (A/B), 20 frozen tasks, audit rubric + blind rater, runner with cache/ledger, tests. No product; no token claims.

## 3. Interfaces
`tasks.py` (20 frozen), `prompts.py` (arm_a, arm_b, audit_prompt), `rater.py` (score plan → 6 bits), `run_all.py` (paired deltas, verdict).

## 4. ACs (TDD: `../tdd/poc-05-coverage-rubric-tdd.md`)
- 05.1 prompts differ only by checklist section (arm B ⊇ arm A)
- 05.2 rater parses 6-bit JSON strictly; failure counted, no retry
- 05.3 20 tasks frozen + hash
- 05.4 runner renders paired table + delta + verdict; deterministic on cache
- 05.5 tests green; SimulatedLLM only in tests
