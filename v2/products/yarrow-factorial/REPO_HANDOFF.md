# Yarrow — Dedicated Repository Handoff

**Status: ✅ DONE — extracted 2026-08-09 to `D:\TRANSFER DATA\Coding\OpenCode\yarrow-factorial\`** (outside this repository; verified: 16/16 tests green, CLI working, docs + validation + examples + LICENSE + CI in place).
**Decision:** Yarrow is a complete application (standalone math library + CLI + reports; no LLM dependency; Phase-2 web UI planned). It graduates out of the portfolio monorepo into its own repository.

## 1. What ships (source of truth: `v2/products/yarrow-factorial/`)
- `yarrow/` — package (factorial math core from POC-02, effects, reports, CLI): 16/16 verification tests green
- `pyproject.toml` — build config, console script `yarrow`
- `PRODUCT_SPEC.md` — industry-standard PRD (market, personas, requirements, KPIs, risks)
- `PRODUCT_PLAN.md` — phased plan with grow/kill gates

## 2. Repo initialization checklist (in the new repo)
1. Copy `yarrow/`, `pyproject.toml`, `tests/`, `README.md` (write fresh), `LICENSE` (MIT).
2. CI: run `pytest tests/ -q` on push (the verification suite IS the correctness gate — contrasts exact vs brute force, Möbius ≤1e-9). Never release on red.
3. Publish: `pip install yarrow-factorial` (PyPI); version 0.1.0.
4. Roadmap pointer: PRODUCT_PLAN.md phases 2–4 (web UI exit gates, LLM-assisted phase with pre-registration, integrations).
5. House rules carried over: no statistical-inference claims, no Bagua semantic claims (naming only), determinism, claims ledger for every benchmark number.

## 3. Path C addendum (also shelved for a dedicated repo)
`v2/probes/path-c/index.html` is a complete single-file web application (44 JS + 11 pytest assertions green; human gate pending). Extraction: copy `index.html` + `docs/lesson-plan.md` + `output/human-gate-report.md` template; add `README.md` with open instructions + the pre-registered human-gate thresholds (≥60% quiz improvement, ≥70% Likert ≥4/5).

## 4. Out of scope for extraction
Do NOT copy the portfolio's experimental machinery (gates.py, POC docs). The new repos inherit only: the product, its verification suite, and the two house rules above.
