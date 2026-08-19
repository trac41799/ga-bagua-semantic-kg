# Yarrow — Interaction Algebra Workbench
## Product Plan (Industry Standard)

**Status:** Plan v1.0 | **Date:** 2026-08-08 | **Source:** PRODUCT_SPEC.md (POC-02 promotion)

---

## 1. Strategy Summary

Take the fully validated POC-02 core (blade algebra = 2^k factorial math; 22/22 exact contrasts; Möbius decomposition ≤1e-9) and productize it as **Yarrow**: a local, deterministic, dependency-light Python library + CLI + report generator. Differentiate on (a) the subset-lattice interaction spectrum, (b) exactness-by-construction with a published proof suite, (c) the Bagua mnemonic naming layer (strictly naming). Grow through the ML-interaction-analysis and DOE segments; monetize later, adopt now.

**Exit criteria (portfolio discipline):** each phase has a pre-registered grow/kill gate. The product is promoted only as the algebra layer; inference and semantics are out of scope by design.

## 2. Phased Delivery

### Phase 1 — MVP: package + CLI + reports (2–3 weeks, 1 engineer)
**Deliverables**
- Package `yarrow` (from validated POC-02 code): designs (k=2..6), contrast signs, effects, Möbius decomposition, Bagua naming, markdown/CSV reports, CLI.
- Verification suite published as the correctness proof page.
- Docs: API reference, three persona tutorials (Ana/Diego/Mara), scope statement (no inference, no semantics).
- Release: `pip install yarrow` (PyPI), MIT.

**Exit gate (grow → Phase 2):**
- Verification suite 100% green on CI for ≥2 weeks of releases.
- ≥3 external users (2 ML, 1 DOE) complete the persona tutorials and report success (structured feedback).
- Task-time reduction ≥30% for the Ana scenario (measured, n≥5).
- **Kill:** no external user completes a tutorial in 6 weeks → reassess positioning before investing in UI.

### Phase 2 — Interactive web UI (4–6 weeks)
**Deliverables**
- Local web app: design explorer (cube/hexagram visualization), clickable contrast tables, interaction spectrum browser, report export.
- No backend beyond static generation; local-only (compliance).
- Usability protocol pre-registered (≥5 users, task completion unassisted).

**Exit gate (grow → Phase 3):**
- Usability study passes (≥5 unassisted completions, ≥4/5 task ease).
- ≥1 reference deployment in a real experimentation workflow.
- **Kill:** usability fails → keep CLI-only product; drop UI investment.

### Phase 3 — LLM-assisted design (4–6 weeks, optional wedge)
**Deliverables**
- Natural language → design spec (factors, levels, aliases) via the validated LLM-judgment mechanism (D3), with strict schema validation.
- Interpretive layer: LLM explains the interaction spectrum in plain language, using Bagua names as vocabulary only.
- **Pre-registered before build:** LLM-generated designs ≥90% schema-valid (few-shot protocol, POC-01 lesson: format enforcement); no token-economics claims (D2 lesson: they stay dead).

**Exit gate (grow → Phase 4):**
- Schema-validity gate passes; ≥2 users report the NL path saves time vs form entry.
- **Kill:** validity <90% → ship without LLM features (product does not depend on them).

### Phase 4 — Integrations & adoption (ongoing)
- Jupyter widget, R bridge (data.frame ↔ yarrow), docs site, changelog discipline, claims ledger.
- Outreach: ML-interaction-analysis tutorials, DOE education materials.

**Exit gate:** ≥100 weekly installs OR ≥3 reference deployments within 12 months. If neither, maintain as OSS with clear scope (still a valid, correct tool).

## 3. Resourcing & Roles
| Phase | Roles | Notes |
|-------|-------|-------|
| 1 | 1 engineer (library/reports), 0.25 PM, 0.1 QA | Core code exists; packaging + docs dominate |
| 2 | 1 engineer (UI), 0.25 designer, 0.1 QA | Local-only static UI |
| 3 | 0.5 engineer (LLM), 0.25 PM | Pre-registration mandatory before build |
| 4 | 0.25 engineer (maintenance) | OSS cadence |

## 4. Milestones & Timeline (from kickoff)
| Week | Milestone |
|------|-----------|
| 1 | Package skeleton + verification suite port; CI green |
| 2 | CLI + reports; docs draft |
| 3 | PyPI release; persona feedback session (exit gate 1) |
| 4–6 | Gate review → Phase 2 decision |
| 7–12 | UI build + usability study (exit gate 2) |
| 13–16 | LLM-assisted phase pre-registration + build (exit gate 3) |
| 17+ | Integrations, adoption, KPIs |

## 5. QA/QC (house discipline, all phases)
- **Correctness gate:** every release must pass the POC-02 verification suite (contrasts exact vs independent brute force; Möbius ≤1e-9) — the published proof is the product's core claim.
- **No-claims gate:** no statistical-inference claims, no semantic claims about Bagua (front page == internal assessment).
- **Determinism gate:** byte-identical outputs on re-run; seeded where randomness exists.
- **Ledger:** every reported number (benchmarks, studies) has a claims-ledger row.
- **Pre-registration:** any user study or LLM feature is pre-registered (hypothesis, metric, baseline, kill criterion) before running.

## 6. Risks & Contingencies
| Risk | Contingency |
|------|-------------|
| Market too small | Education wedge; publish the correctness proof as the differentiator; keep OSS costs near zero |
| R/JMP incumbent lock-in | Integration bridge (R bridge) + interaction-spectrum capability they don't expose |
| Scope creep into inference | Non-goals enforced at every release; inference = integration story only |
| LLM phase fails validity gate | Ship without it; product remains complete (Phase 3 is a wedge, not a dependency) |

## 7. Go-to-Market (lightweight, evidence-based)
- **Channels:** PyPI, GitHub README (with the proof), targeted posts on ML-interaction analysis; DOE community tutorials (Mara persona).
- **Narrative:** "Exact 2^k factorial math, interaction spectrum, and an interpretable naming layer — local, deterministic, verifiable." No cultural mystique in marketing; Bagua is a naming mnemonic, stated plainly.
- **Success evidence:** reference deployments + tutorial completions, tracked in the claims ledger.

## 8. Budget (rough, units)
| Phase | Effort | Notes |
|-------|--------|-------|
| 1 | 2.5–3 staff-weeks | Packaging + docs + study |
| 2 | 4–6 staff-weeks | UI + usability |
| 3 | 3–4 staff-weeks | LLM wedge (pre-registered) |
| 4 | maintenance | OSS cadence |

Total to first grow/kill decision: ~3 staff-weeks.
