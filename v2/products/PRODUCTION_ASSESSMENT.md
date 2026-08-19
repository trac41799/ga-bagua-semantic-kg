# Production Assessment — Successful POCs

**Date:** 2026-08-15 | **Decision:** 1 complete application (shelved for dedicated repo), plus the `iching-tools` six-capability suite with mechanical package/MCP/SDK evidence; host/product fit remains pending

| POC | Validated result | Nature | Verdict |
|-----|------------------|--------|---------|
| **Yarrow** (02 + 07 math) | Contrasts 22/22 exact; interaction recovery 2.2e-16 | **Complete application**: standalone math library + CLI + reports; no LLM dependency; product spec/plan written; Phase-2 web UI planned | **SHELVE → dedicated repo** (`products/yarrow-factorial/REPO_HANDOFF.md`); not an LLM-supportive tool |
| **POC-05** Coverage checklist | Missing aspects 2.40→1.25 (Δ+1.15); R-05 replicated | **LLM-supportive capability**: takes a plan+task, returns the checklist-audited plan | **PASS mechanically; product PENDING for host/product fit** |
| **POC-08** Reframing grammar | DeepSeek baseline diversity Δ+0.290/coherence 3.75; R-08 replication 0.296/3.10 FAIL | **Model-dependent LLM-supportive capability**: statement → 8 algebra-grounded reframes | **MODEL_DEPENDENT; product PENDING; not model-general** |
| **POC-06/POC-10** State-diff communication | Historical objective QA 1.000 vs 0.900; rater calibration FAIL; clean no-leakage rerun pending | **LLM-supportive capability under measurement review**: (before, after) → structured 3-aspect summary | **INCONCLUSIVE; product PENDING** |
| **POC-11** Cl3 calculator | Exact structured operations, reference parity, strict validation, and deterministic tests | **Deterministic exact-math MCP capability**: structured Cl(3) evaluation | **PASS mechanically; product PENDING pending host/product fit** |
| **POC-12** Interaction spectrum | 3/3 planted interactions, error ≤1e-9, zero false positives | **Deterministic exact-math MCP capability**: Walsh-Hadamard interaction recovery | **PASS mechanically; product PENDING pending host/product fit** |
| **POC-14** Rotor transition algebra | Hand cases plus 100-chain closure/inverse/associativity tests | **Deterministic exact-math MCP capability**: composable rotor transitions | **PASS mechanically; product PENDING pending host/product fit** |
| **ICHING-TOOLS** suite | Package v0.2.0, six-tool MCP surface, official SDK source/clean-wheel checks, fresh wheel/sdist | **One distributable suite**: three protocol tools + three exact-math tools | **Mechanical evidence PASS; product PENDING for host/product fit** |
| Path C | All gates green, human gate pending | Complete application (single-file web tool) | **SHELVE → dedicated repo** (addendum to Yarrow handoff; single self-contained file) |

**Rationale for the tool/application split:** an LLM-supportive tool is *called by or alongside an LLM agent* (MCP) or a human in a terminal (CLI) and produces a protocol output the LLM consumes. A complete application stands alone with its own UI/domain workflow and its own product lifecycle. Yarrow and Path C qualify as applications; `iching-tools` is a distributable suite of three protocol capabilities and three deterministic math capabilities, not a host-validated product yet.

**Production form:** one workspace `products/iching-tools/` — three protocol Python packages, three deterministic math packages, one unified CLI, and one stdio MCP server exposing all six tools. Package v0.2.0, fresh wheel/sdist, official SDK source/clean-wheel calls, and three skills are mechanically evidenced; LLM quality remains model-scoped and host/product-fit evidence is absent. `SimulatedLLM` is for tests only (L4).

## Shelving procedure (Yarrow, Path C)
1. `REPO_HANDOFF.md` in `products/yarrow-factorial/` — extraction instructions: repo layout, CI, license, publish, roadmap pointer.
2. Path C handoff appended to the same doc (single-file app; copy `index.html` + docs).
3. Both remain in-place as the source of truth until the dedicated repos are initialized; the handoff doc is the contract.
