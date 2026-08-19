# Multi-Agent Orchestration Plan — v2 Portfolio

**Goal:** Close all three probes in parallel with maximum parallelism and tight QA/QC gates. One build cycle, one QA cycle, fix cycle on failure.

## 1. Team & roles

| Role | Count | Responsibility | Agent/owner |
|------|-------|----------------|-------------|
| Builder A | 1 | Implement Path A probe per spec+TDD | subagent (parallel) |
| Builder B | 1 | Implement Path B probe per spec+TDD | subagent (parallel) |
| Builder C | 1 | Implement Path C tool per spec+TDD | subagent (parallel) |
| QA/QC reviewer | 1 | Run all gates across 3 probes, produce gate report | subagent (after builders) |
| Integration owner | 1 | Portfolio docs, claims ledger, kill/go recommendation | orchestrator (me) |

## 2. Parallelization DAG

```
         ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
         │ Builder A   │ │ Builder B   │ │ Builder C   │     (parallel, isolated dirs:
         │ probes/a    │ │ probes/b    │ │ probes/c    │      no shared files)
         └──────┬──────┘ └──────┬──────┘ └──────┬──────┘
                └───────────────┼───────────────┘
                                ▼
                      ┌──────────────────┐
                      │ QA/QC reviewer   │  (gate checklist, cross-probe)
                      └────────┬─────────┘
                               ▼
                      ┌──────────────────┐
                      │ Fix cycle (if    │
                      │ gates fail)      │
                      └────────┬─────────┘
                               ▼
                      Integration owner: kill/go report
```

Isolation contract: builders write ONLY inside `v2/probes/<path>/`; QA writes ONLY inside `v2/qa/`; orchestrator owns everything else. No builder touches another probe's directory.

## 3. QA/QC gates (executable; see `v2/qa/gates.py`)

| Gate | Definition | Where enforced | Failing = |
|------|-----------|----------------|-----------|
| G1 Test suite green | All pytest/node tests pass | per-probe `pytest -q` / `node tests/run_js_tests.js` | Red build → fix |
| G2 Baseline wall | Eval tables include majority, dom(A), cosine kNN, random | `eval.py`/`reports.py` output | Report FAIL |
| G3 Ablation gate | Feature removal changes ≥5% predictions / ≥1pp (A); basis & relation ablations rendered (B) | probe reports | Flag for kill |
| G4 Circularity gate | max(label==dom(A), label==dom(B)) ≤ 60% (A) | `dataset.circularity_report` | Dataset FAIL |
| G5 Split & determinism | 3-fold CV / seeded; re-run identical (1e-12) | `run_all.py` twice | Report FAIL |
| G6 Claims ledger | Every metric row → run artifact + baseline + date | `output/claims_ledger.csv` | Report FAIL |
| G7 Honest framing | README states limitations; no unsupported claim; front page == internal | doc review by QA | Report FAIL |
| G8 Pre-registered verdicts | A: >10pp over dom(A) p<0.05; B: delta vs RotatE computed PASS/FAIL; C: thresholds documented | gate report | Verdict rendered, not hidden |
| G9 No-network / single-file (C) | zero external requests, one file | `verify_offline.py` | Red build → fix |
| G10 Runtime budget | A < 60s, B < 180s (mini-KG), C tests < 60s | timed runs | Report FAIL |

## 4. QA/QC gate report template (`v2/qa/gate-report.md`)

```
# Gate Report — <date> — Run <hash>
## Summary: X/Y gates green
| Probe | Tests | Baseline wall | Ablation | Circularity | Determinism | Ledger | Framing | Verdict |
| A | ✅/❌ | ✅/❌ | ✅/❌ | ✅/❌ | ✅/❌ | ✅/❌ | ✅/❌ | PASS/FAIL |
| B | ... |
| C | ... |
## Kill-criterion verdicts
- A: primary claim PASS/FAIL (delta over dom(A), McNemar p)
- B: delta check PASS/FAIL (+0.01 MRR)
- C: human gate NOT RUN (manual) — thresholds documented
## Blockers (if any)
## Recommendation (orchestrator fills): fund / kill / iterate per probe
```

## 5. Timeline

| Phase | Duration | Gates |
|-------|----------|-------|
| 0. Specs + TDD + orchestration (this dir) | done | doc review |
| 1. Build (3 agents parallel) | ≤ 40 min wall clock | per-probe test suites |
| 2. QA/QC (1 agent) | ≤ 15 min | G1–G10 |
| 3. Fix cycle (targeted agents) | ≤ 20 min | re-run failed gates |
| 4. Integration report | ≤ 10 min | G8 verdicts + recommendation |

## 7. Execution log (2026-08-07)

| Phase | Planned | Actual |
|-------|---------|--------|
| 0. Specs + TDD + orchestration | docs | done — 3 specs, 3 TDD plans, LESSONS ledger, this plan |
| 1. Build (3 parallel agents) | 3 subagents | **2 agent sessions failed silently (Paths A & B — empty results, no files); orchestrator executed A and B directly**; Path C completed via its agent (44 JS + 11 pytest assertions green) |
| 2. QA/QC | 1 agent | run by orchestrator via `v2/qa/gates.py` — **23/23 gates green** |
| 3. Fix cycle | targeted agents | 2 gate issues fixed in place (case-sensitive G7 check; no blockers remained) |
| 4. Integration report | orchestrator | kill/go below |

**Integration verdict (2026-08-07):**
- **A — KILL (claim).** Primary claim FAILED pre-registration (probe ≤ dom(A)); methodology validated (circularity gate, spectrum ablation). Keep: dataset-construction method, gate machinery.
- **B — ITERATE.** Delta check PASSED on CI fixture but the Bagua-axis init ablation is negative and RotatE ≈ random; require the WN18RR public-benchmark run (documented manual step) before any claim.
- **C — PROCEED to human gate.** Build complete and clean; run the pre-registered learner sessions (n≥5, ≥60% improvement, ≥70% ≥4/5) and record in `probes/path-c/output/human-gate-report.md`.

Lesson L11 (new): agent sessions can fail silently — every builder output must be verified by artifact existence + gate execution, never by agent report alone.
