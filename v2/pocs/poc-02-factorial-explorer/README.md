# POC-02 — Factorial Interaction Design Explorer

**Status: BUILT AND VALIDATED — OVERALL PASS**

## Verdict (2026-08-08)

| Check | Result | |
|-------|--------|---|
| 2³ contrast signs vs independent brute force | 7/7 exact | **PASS** |
| 2⁴ contrast signs vs independent brute force | 15/15 exact | **PASS** |
| Möbius interaction decomposition (50 set functions, n=3..6) | max error **3.91e-14** (≤1e-9) | **PASS** |
| Bagua naming (trigrams + King Wen hexagram spot-checks) | 19/19 correct | **PASS** |

**The blade algebra of Cl(3) IS the algebra of 2^k factorial designs:** blades are interaction terms (grade = interaction order), the geometric product signs reproduce ANOVA contrast signs exactly, and Möbius inversion on the subset lattice gives interaction decomposition. The Bagua layer is the mnemonic interface (a trigram IS a 3-factor interaction term; a hexagram stacks designs).

## Run

```
python -m pytest tests/ -q      # 26 tests
python run_all.py               # correctness verification → output/verdict.md
```

## Scope commitment
Algebra layer only — no statistical-inference claims, no UI (CLI/library). No LLM, no network required.
