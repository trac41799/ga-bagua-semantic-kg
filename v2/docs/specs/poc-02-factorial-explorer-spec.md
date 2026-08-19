# SDD — POC-02: Factorial Interaction Design Explorer

**Status:** Pre-registered POC | **Effort:** 1 week | **Priority:** 2 of 4
**Falsifiable question:** Does the blade algebra of Cl(3) reproduce textbook 2^k factorial-design mathematics — contrast signs and interaction decomposition — with 100% correctness on standard design tables?

## 1. Pre-registration (before any code)

| Item | Commitment |
|------|------------|
| Primary claim | Contrast signs for all 2³ and 2⁴ factorial designs (main effects + interactions) EXACTLY match the textbook sign tables (e.g., Montgomery "Design and Analysis of Experiments") |
| Secondary claim | Möbius interaction decomposition: for any set function f over n≤6 factors, the interaction coefficients recovered via blade projection reproduce brute-force Möbius inversion to 1e-9 |
| Bagua layer | Trigram ↔ factor-combination naming consistent: factor combo with bits (b1,b2,b3) maps to the trigram with the same bits; hexagram = stacked design (upper = main-effect block, lower = interaction block) |
| Kill criterion | Any mismatch in the 2³/2⁴ contrast tables → POC-02 dies as a correctness claim (the tool would be wrong, full stop) |

## 2. Scope

**In:** design generator (2^k factor combinations), contrast-sign computation via the geometric product (grade = interaction order, sign = product table), interaction decomposition (Möbius/blade projection), Bagua naming map (trigram ↔ combination, hexagram ↔ stacked design), report generator (design table + interaction effects + Bagua labels), tests.
**Out:** No UI (CLI/library only in POC phase); no statistical inference claims (this is the *algebra* layer of design analysis); no LLM.

## 3. Architecture

```
design.py        factor combinations (2^k bit patterns) → blade states
contrasts.py     main effects + interactions via geometric product signs
decompose.py     Möbius inversion via grade projection on the subset lattice
bagua_names.py   trigram/hexagram naming for combinations
report.py        markdown reports: design table, contrast table, effects, names
run_all.py       verifies against embedded textbook tables → verdict
```

## 4. Interfaces

| Component | File | Interface |
|-----------|------|-----------|
| Design | `design.py` | `combinations(k) -> list[bits]`; `blade_state(bits) -> (blade_idx, sign)` |
| Contrasts | `contrasts.py` | `contrast_signs(k) -> {effect: [(rows, sign)]}`; `main_effect(data, factor)`, `interaction(data, factors)` |
| Decompose | `decompose.py` | `mobius_coefficients(f: set_fn, n) -> {subset: coeff}` |
| Names | `bagua_names.py` | `trigram_name(bits)`, `hexagram_name(upper_bits, lower_bits)` |
| Reports | `report.py` | renders `output/design_k3.md`, `output/contrasts.md`, `output/decomposition.md` |

## 5. Evaluation protocol

1. Embedded truth tables: 2³ and 2⁴ contrast signs computed by an independent brute-force method (sum of products over rows) — cross-checked in tests.
2. Contrast agreement = exact equality (no tolerance) on all 2³ (7 effects) and 2⁴ (15 effects).
3. Decomposition: 50 random set functions over n=3..6; max abs error vs brute-force Möbius ≤ 1e-9.
4. Bagua naming: 8 trigram names and 8 spot-checked hexagram names match the canonical King Wen table.
5. Determinism: seeded RNG for random set functions; identical reports on re-run.

## 6. Acceptance criteria (TDD: `docs/tdd/poc-02-tdd.md`)

- AC-02.1 blade_state bit→blade mapping natural convention (e13/e12 per product of selected vectors)
- AC-02.2 contrast signs 2³ exact vs brute force (7 effects × row signs)
- AC-02.3 contrast signs 2⁴ exact vs brute force (15 effects)
- AC-02.4 main_effect/interaction compute correct numeric effects on a hand table
- AC-02.5 Möbius decomposition error ≤ 1e-9 on 50 random set functions (n=3..6)
- AC-02.6 trigram names: 8/8; hexagram spot-checks ≥ 8 correct
- AC-02.7 reports render (design table + contrasts + decomposition + names)
- AC-02.8 deterministic; tests green; no LLM/network required
