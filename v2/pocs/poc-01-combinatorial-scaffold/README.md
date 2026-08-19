# POC-01 — Combinatorial Reasoning Scaffold for LLMs

**Status: BUILT AND VALIDATED — pre-registered claim FAILED (+2.0pp vs +20pp bar), with a clean failure decomposition**

## Verdict (2026-08-08, real LLM deepseek-chat, 50 frozen problems)

| Metric | Value | Criterion | |
|--------|-------|-----------|---|
| Scaffold accuracy | 0.120 | — | |
| LLM-alone accuracy (exact match) | 0.100 | — | |
| Delta | **+2.0pp** | ≥ +20pp | **FAIL — kill criterion fired** |
| McNemar p | 1.000 | < 0.05 | FAIL |

**Failure decomposition (QA fact-check, diagnose.py):** 37/50 scaffold failures = LLM did not emit the specified JSON op objects (protocol-parse failures); calculator execution failures = **0**. Alone-arm strict 10% vs lenient 40% (answers embedded in prose). The failure is a **protocol-compliance problem, not an algebra problem** — the calculator was exact whenever it ran. v2 would require a new pre-registration (few-shot JSON format + canonical answers for both arms).

## Run
`
python -m pytest tests/ -q        # 28 tests
python run_all.py --sim           # smoke
python run_all.py                 # real run (cached) → output/verdict.md
python diagnose.py                # failure decomposition
`

## What it is## What it is
A deterministic Cl(3) blade calculator (flip ·ei, complement = Hodge dual ·e123, grade, geometric product, hexagram combine) wrapped in an LLM translation protocol: the LLM encodes a combinatorial problem into validated blade operations, the calculator computes exactly, the LLM interprets. The Bagua vocabulary names the states (8 trigrams, 64 hexagrams). No semantics are derived from the algebra — GA computes, Bagua names.

## Pre-registered claim
Scaffold accuracy ≥ LLM-alone + **20pp** on the 50 frozen problems (5 categories × 10: parity, complement/De Morgan, interaction counting, sign bookkeeping, hexagram composition), McNemar p<0.05. Freeze marker: `problems.keys.sha256` (`24ab10dd...`). Protocol failures count as scaffold failures (no retries).

## Run
```
python -m pytest tests/ -q        # 28 tests (SimulatedLLM only)
python run_all.py --sim           # pipeline smoke test
python run_all.py                 # REAL LLM run (cached in data/cache/) → output/verdict.md
```

## Calculator conventions (natural convention)
- Bits ↔ blades: 000→1, 100→e1, 010→e2, 001→e3, 011→e23, 101→e13=−e31, 110→e12, 111→e123
- complement(Kan)=Li, complement(Gen)=Dui, complement(Zhen)=Xun, complement(Kun)=Qian — the coherent identity, verified in tests
