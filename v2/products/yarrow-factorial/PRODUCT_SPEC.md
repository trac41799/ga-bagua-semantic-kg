# Yarrow — Interaction Algebra Workbench
## Product Specification (PRD) — Industry Standard

**Status:** Spec v1.0 | **Date:** 2026-08-08 | **Owner:** Product (from POC-02)
**Origin:** POC-02, the only fully validated Clifford/Bagua application (contrast signs 22/22 exact vs independent brute force; Möbius decomposition error 3.91e-14). Working title "Yarrow" — the I-Ching's divination plant: a cultural name over exact mathematics (naming, never semantics).

---

## 1. Executive Summary

Yarrow is a dependency-light Python library, CLI, and report generator for **2^k factorial design analysis built on the blade algebra of Cl(3)**: interaction terms ARE blades (grade = interaction order), the geometric product reproduces ANOVA contrast signs exactly, and Möbius inversion on the subset lattice yields the full interaction spectrum — beyond what standard DOE tooling reports. A Bagua naming layer (trigram ↔ 3-factor interaction term, hexagram ↔ stacked design) provides a mnemonic, teachable interface. Deterministic, local, verifiable.

**One-line value proposition:** *Exact factorial-design mathematics with an interpretable interaction spectrum and shareable reports — in minutes, without heavyweight DOE suites.*

## 2. Problem & Opportunity

### 2.1 Problem
- Teams running 2^k experiments (product A/B tests with multiple factors, ML feature-interaction analysis, manufacturing DOE) get contrast tables and main effects from standard tools (JMP, Minitab, R `FrF2`), but the **interaction spectrum is under-served**: full Möbius interaction decomposition over all subsets is not exposed, and effect structure is presented as opaque tables.
- Ad-hoc Python (`itertools` + hand loops) is error-prone — exactly the sign-bookkeeping errors the blade algebra eliminates (POC-02 proved exactness against independent brute force).
- Heavy DOE suites are overkill for the 2^k case and opaque for newcomers.

### 2.2 Opportunity
- The interaction algebra is **new packaging over classical math**: grade-structured effects, subset-lattice decomposition, and the mnemonic Bagua layer are differentiators.
- Local, deterministic, dependency-light execution is a compliance selling point (data never leaves the machine).
- LLM-assisted design generation is a later-phase wedge (validated mechanism: LLM-judgment encoding > lexical IR, Path D-D3), but the MVP must NOT depend on it (POC-01 protocol lesson).

## 3. Market & Users

### 3.1 Target segments (priority order)
| Segment | Who | Use case | Willingness |
|---------|-----|----------|-------------|
| ML/Data teams | Engineers analyzing feature interactions | Möbius interaction decomposition for model inputs | High (new capability) |
| Product analytics | Experimentation teams running multi-factor A/B tests | 2^k designs with exact contrasts | Medium |
| Manufacturing/quality | Process engineers (DOE) | Design tables + effects + reports | Medium (JMP/Minitab entrenched) |
| Education | Stats/DOE instructors | Teachable, verifiable interaction math with mnemonics | Low (strategic) |

### 3.2 Personas
- **Ana (ML engineer):** wants interaction coefficients over all factor subsets for a 6-factor model-input analysis; today uses ad-hoc scripts; values correctness proof.
- **Diego (experimentation lead):** runs 2^3/2^4 A/B designs; wants exact contrasts and a report he can share with non-statisticians.
- **Mara (process engineer):** used Minitab for a decade; needs the interaction spectrum without a license purchase cycle.

### 3.3 Competitive landscape
| Tool | Strength | Gap Yarrow fills |
|------|----------|------------------|
| JMP / Minitab | Full DOE + inference | Cost, opacity, no subset-spectrum decomposition, no mnemonic layer |
| R FrF2 / DoE.base | Free, statistical inference | Scripting burden; interaction spectrum not first-class |
| pyDOE2 / itertools | Free, simple | Contrast-sign error risk; no decomposition; no reports |
| Statsmodels | Inference | No 2^k design algebra as a product |

**Positioning:** not a replacement for inference tooling — the **algebra layer + interaction spectrum + interpretable naming**, designed to sit alongside R/JMP workflows.

## 4. Product Requirements

### 4.1 MVP (Phase 1) — functional requirements
| ID | Requirement | Acceptance |
|----|-------------|------------|
| FR-1 | 2^k design generation (k=2..6) | All bit patterns; deterministic |
| FR-2 | Contrast signs for all effects, geometric-product derived | EXACT match vs independent brute force (regression suite) |
| FR-3 | Main effects + interaction effects from response data | Standard 2^k effect formulas |
| FR-4 | Möbius interaction decomposition over all subsets | Error ≤ 1e-9 vs brute force |
| FR-5 | Bagua naming: trigram ↔ combination, hexagram ↔ stacked design | Canonical names; verified table |
| FR-6 | Report generator: design table, contrasts, effects, decomposition, names | Markdown + CSV export |
| FR-7 | CLI: `yarrow design -k 3 -o report.md` | Documented commands |
| FR-8 | Python API: `design(k)`, `contrast_signs(k)`, `effects(data)`, `decompose(f, n)` | Stable, typed signatures |
| FR-9 | Determinism | Same input → byte-identical output |
| FR-10 | Zero runtime dependencies beyond numpy | `pip install yarrow` |

### 4.2 Non-functional requirements
- **Correctness:** every release gated by the POC-02 verification suite (contrasts vs brute force, decomposition ≤1e-9) — a published, re-runnable proof.
- **Performance:** 2^6 designs compute in < 1s; decomposition n≤6 in < 1s.
- **Portability:** Python ≥3.10, numpy; no network at runtime; no build step.
- **Compliance:** local-only execution (data never leaves the machine) — documented in security section.
- **Documentation:** API reference, tutorial (with Ana/Diego/Mara scenarios), correctness-proof page.

### 4.3 Non-goals (MVP)
- Statistical inference (p-values, variance estimates) — explicitly OUT; position as the algebra layer, integrate with R/statsmodels for inference.
- Non-orthogonal / fractional designs, response surfaces, randomized blocks.
- UI (Phase 2), LLM integration (Phase 3).
- Any semantic claim about Bagua beyond naming.

### 4.4 Later phases (with exit criteria — gate discipline per portfolio rules)
| Phase | Scope | Exit criterion (kill/grow) |
|-------|-------|----------------------------|
| 2 | Interactive web UI: cube/design visualization, clickable contrast explorer | ≥5 external users complete a design task unassisted (usability study) |
| 3 | LLM-assisted: natural language → design spec + interpretation (D3 mechanism) | Pre-registered: LLM-generated designs ≥90% valid vs schema; no token-economics claims |
| 4 | Integrations: Jupyter widget, R bridge, docs site, `pip` release process | Adoption KPI: ≥100 weekly installs OR 3 reference deployments |

## 5. Architecture (MVP)

```
yarrow/                     # Python package (core = validated POC-02 code)
  factorial.py              # designs, blades, contrasts, decomposition, names (existing, tested)
  effects.py                # main/interaction effects from response data
  report.py                 # markdown + CSV renderers
  cli.py                    # yarrow CLI (argparse)
tests/                      # verification suite (contrasts vs brute force, Möbius ≤1e-9)
```

- No service layer in MVP; Phase 4 may add an optional local API.
- Reproducibility: pinned numpy, published verification results per release (claims ledger per house rules).

## 6. Security, Privacy, Compliance
- All computation is local; no telemetry without opt-in; no data transmission (compliance-friendly for regulated process engineering).
- No secrets, no network endpoints in MVP.
- Licensing: MIT (consistent with the portfolio).

## 7. Pricing & Packaging (hypotheses, to validate)
- Open source (MIT) library + CLI — free, adoption driver.
- Revenue candidates (later): managed report service, team support, integration maintenance. Not revenue-gated for MVP.

## 8. KPIs & Success Metrics
| KPI | Target (12 mo) | Measure |
|-----|----------------|---------|
| Correctness regression | 100% every release | CI suite |
| Adoption | ≥100 weekly installs or 3 reference deployments | PyPI + outreach |
| Task time | ≥30% reduction vs ad-hoc scripts (Ana scenario) | User study (n≥5) |
| Report usage | ≥50% of users export reports weekly | Telemetry (opt-in) |

## 9. Risks & Mitigations
| Risk | Severity | Mitigation |
|------|----------|------------|
| Niche market / free R competitors | Med | Differentiate on interaction spectrum + naming + correctness proof; education segment as wedge |
| Correctness liability (statistics domain) | High | Never ship inference claims; publish verification; clear scope statement |
| Over-reach into inference tooling | Med | Explicit non-goals; integration story with R/statsmodels |
| Bagua layer perceived as pseudo-science | Low-Med | Frame strictly as mnemonic naming; no semantic claims anywhere (house rule) |

## 10. Open Questions (tracked)
1. Does the Möbius interaction spectrum drive real decisions better than standard effects? (needs a field user study — pre-registered, Phase 2)
2. Is the education segment a viable wedge for adoption? (test with 2 instructors)
3. Which inference integration (R vs statsmodels) is the first bridge?

## 11. Release definition of done (each release)
- Verification suite green (contrasts exact, decomposition ≤1e-9)
- CLI + API documented; changelog; claims ledger updated
- Non-goals respected (no inference claims, no semantics claims)
- `pip install yarrow` works from a clean environment
