# SDD — POC-06: State-Diff Vocabulary (3-aspect change summaries for agent introspection)

**Status:** Pre-registered POC | **Effort:** 1 week | **Priority:** 3 in adoption roadmap
**Falsifiable question:** Do structured 3-aspect change summaries ("aspect1: X→Y, aspect2: ..., aspect3: ...") improve comprehension of agent state transitions over free-form prose summaries?

## 1. Pre-registration

| Item | Commitment |
|------|------------|
| Primary claim (proxy) | LLM-rater comprehension (1–5) of Arm B (structured 3-aspect) ≥ Arm A (free-form) + **0.5**, paired over 20 state transitions |
| Data | 20 (before, after) concept-state pairs — e.g., system/business/biological state changes with planted aspect-level deltas — frozen before any run |
| Protocol | Arm A: LLM writes a prose summary of the change. Arm B: LLM fills the 3-aspect template (aspect labels generated per context, deltas exact). Rater: separate LLM call, blind, scores comprehension 1–5 (factual coverage of the planted deltas) |
| Kill criterion | Delta < 0.5 → POC-06 dies as a claim |
| Honesty | Proxy labeled; human gate later |

## 2. Scope
20 frozen transitions (with planted ground-truth deltas), two summarization protocols, blind rater, runner, tests. The 3-line bit structure (V3) is the template source; labels are LLM-generated per context — no forced vocabulary.

## 3. Interfaces
`transitions.py` (20 frozen with planted deltas), `prompts.py`, `summarize.py`, `rater.py`, `run_all.py`.

## 4. ACs (TDD: `../tdd/poc-06-state-diff-tdd.md`)
- 06.1 planted deltas verifiable from (before,after) pairs (automated check)
- 06.2 Arm B template strictly 3 aspects (parse-validated); failures counted
- 06.3 rater 1–5 parse; deterministic on cache
- 06.4 runner renders paired table + delta + verdict
- 06.5 tests green
