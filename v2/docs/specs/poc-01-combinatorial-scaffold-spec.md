# SDD — POC-01: Combinatorial Reasoning Scaffold for LLMs

**Status:** Pre-registered POC | **Effort:** 1–2 weeks | **Priority:** 1 of 4
**Falsifiable question:** Does a deterministic Cl(3) blade-algebra calculator — with the Bagua state vocabulary as interface — improve LLM accuracy on exact combinatorial reasoning by ≥20pp over LLM-alone, on a 50-problem benchmark (p<0.05)?

## 1. Pre-registration (before any code)

| Item | Commitment |
|------|------------|
| Primary claim | Scaffold accuracy ≥ LLM-alone accuracy + **20pp** on the 50-problem benchmark, McNemar p<0.05 |
| Benchmark | 50 problems, 10 per category: (1) parity/bit operations, (2) complement & De Morgan, (3) interaction counting (grade/Hamming weight), (4) sign bookkeeping (geometric products), (5) hexagram composition (two-trigram → 6-bit state). Problems authored with exact answer keys |
| Protocol | LLM-alone: direct answer. Scaffold: LLM encodes problem → blade operations (JSON, validated) → calculator computes exactly → LLM interprets. Both on the same model (deepseek-chat, temperature 0), same budget class |
| Baselines | LLM-alone, random-guess floor, calculator-only (no LLM, impossible on natural language — documented) |
| Kill criterion | Δ < 20pp or p ≥ 0.05 → POC-01 dies as a claim (harness survives as an eval substrate) |
| Honesty | Problems authored BEFORE running any model; answer keys frozen; protocol errors counted as scaffold failures (no retries) |

## 2. Scope

**In:** verified Cl(3) core (port from archive `path-a-semantic-index/cl3.py` — K1), calculator API (`flip`, `complement`/`dualize`, `wedge`, `grade`, `product`, `hexagram_combine`), LLM translation protocol (encode problem → JSON op list → validate → execute → interpret), benchmark (50 problems + keys), runner (scaffold vs alone), reports, tests.
**Out:** No new semantics; no WuXing; no retrieval claims; no product beyond the benchmark verdict.

## 3. Architecture

```
problems.py (50 problems, 5 categories, frozen answer keys)
    ├── llm_client (reuse Path D client pattern; SimulatedLLM for tests)
    ├── protocol.py     problem → JSON ops (validate) → execute on calculator → result → interpret
    ├── calculator.py   Cl(3) ops on blade states (bit pattern + sign)
    └── run_all.py      scaffold vs alone vs random → output/ (accuracy table, McNemar, verdict)
```

## 4. Interfaces

| Component | File | Interface |
|-----------|------|-----------|
| Calculator | `calculator.py` | `flip(state, line)`, `complement(state)`, `grade(state)`, `product(a,b)`, `combine(tri_a, tri_b) -> hexagram`; states as (blade_index, sign) |
| Protocol | `protocol.py` | `plan_ops(problem_text, llm) -> op_list` (JSON, validated); `execute(ops) -> result`; `interpret(result, llm) -> answer` |
| Benchmark | `problems.py` | `PROBLEMS: list[(id, category, text, answer)]`; `score(answer_text) -> bool` (exact key match, normalized) |
| Runner | `run_all.py` | runs both arms, McNemar, renders reports + verdict |

## 5. Evaluation protocol

1. All 50 problems frozen before any run; answer keys canonical (normalization: strip, lowercase, numeric compare).
2. LLM-alone: one call per problem, temperature 0.
3. Scaffold: encode → validate (invalid ops = FAIL, no retry) → compute → interpret (one call). Failures at ANY stage count against the scaffold.
4. Same model + temperature for both arms; usage ledger appended.
5. McNemar on per-problem correctness; report per-category breakdown.
6. Determinism: seeds for any sampling; cache of LLM responses for re-runs.

## 6. Acceptance criteria (TDD: `docs/tdd/poc-01-tdd.md`)

- AC-01.1 calculator ops exact on hand cases (flip 24 cases, complement 8, product table 64, grade 8)
- AC-01.2 protocol validates op lists; malformed → typed error, no partial execution
- AC-01.3 protocol plan→execute round-trip on 5 golden problems (deterministic expected op lists)
- AC-01.4 benchmark loads 50 problems / 5 categories / frozen keys; score() correct
- AC-01.5 runner renders accuracy table + McNemar + per-category + verdict row
- AC-01.6 SimulatedLLM tests only; real run cached; budget-capped
- AC-01.7 determinism of reports; claims ledger rows
- AC-01.8 tests green; run_all < 30 min (real LLM, cached)
