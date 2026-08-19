# SDD — POC-07: Feature-Interaction XAI (Möbius spectrum + Bagua naming for black-box inputs)

**Status:** Pre-registered POC | **Effort:** 2 weeks | **Priority:** 4 in adoption roadmap
**Falsifiable question:** (math) Does the blade-algebra Möbius spectrum exactly recover planted interaction structure of a black-box input function? (naming) Are Bagua-named interaction explanations rated clearer than numbered ones?

## 1. Pre-registration

| Item | Commitment |
|------|------------|
| Math claim | For a synthetic black-box f over 6 inputs with 3 planted interactions (2-way e12, 2-way e45, 3-way e123), the Möbius spectrum recovers the interaction coefficients with max error ≤ 1e-9 and identifies exactly the planted interaction subsets (no false positives at tolerance 1e-6) |
| Naming claim (proxy) | LLM-rater clarity (1–5) of Bagua-named interaction explanations ("the e12 interaction between factor A and B") ≥ numbered ("interaction #2") + **0.5**, over 10 explanations |
| Kill criteria | Math recovery fails → product claim dead. Naming delta < 0.5 → naming claim dead (math still ships) |
| Honesty | Synthetic function (planted ground truth) documented as such; naming proxy labeled |

## 2. Scope
`blackbox.py` (planted function + oracle), `spectrum.py` (Möbius over subsets via POC-02 core), `naming.py` (Bagua labels for 2^3 structure + LLM explanation sentences), `runner` + tests. No inference claims.

## 3. Interfaces
`blackbox.py`: `f(x) -> float` (6 inputs), `planted_interactions`; `spectrum.py`: `mobius_spectrum(f, n=6) -> {subset: coeff}`; `naming.py`: `explain(subset, factor_names, bagua=True/False) -> str`; `run_all.py`.

## 4. ACs (TDD: `../tdd/poc-07-interaction-xai-tdd.md`)
- 07.1 spectrum recovers planted coefficients ≤1e-9; planted subsets identified exactly
- 07.2 naming: bagua vs numbered strings differ only in label wording (same math content)
- 07.3 rater parse; deterministic
- 07.4 runner renders math verdict + naming verdict
- 07.5 tests green
