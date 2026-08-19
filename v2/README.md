# v2 Portfolio — GA×Bagua, Rebuilt on Evidence

This is the second-generation portfolio for the Bagua + Clifford Algebra idea. v1 (repo root) is **retired**. Failed implementations live in `archive/experimentation/fails/` with their evidence. The surviving principle: **GA computes (flip ·ei, complement ·e123, wedge, grade, product); Bagua names (8 trigram states, 64 hexagram states). Never the reverse.**

## Active evidence & tools

| Item | Status | Evidence |
|------|--------|----------|
| **THESIS v1.0** (`THESIS.md`) | The versioned working thesis with its boundary, evidence map, and replication queue | Supported: exact computation; bounded generation (POC-05 replicated, POC-08 model-dependent); falsified: discrimination; communication replication pending |
| **Path C** — GA×Bagua teaching tool (`probes/path-c/`) | Build complete, all tests green; human gate pending (manual) | 44 JS + 11 pytest assertions, offline verified |
| **Path D** — decisive experiment harness (`probes/path-d/`) | EXECUTED with real LLM; verdicts banked | D1 FAIL (R@10 0.370), D2 FAIL (47% recall, break-even 23), **D3 PASS (+10.6pp over TF-IDF)** |
| **Yarrow** — product MVP (`products/yarrow-factorial/`, standalone at `yarrow-factorial/`) | Built: package + CLI + reports; Phase-1 exit gates pending | 86 tests passing (standalone project); POC-02-validated core; release provenance blocked (no independent Git root, no external evidence) |
| **iching-tools** — deterministic capability suite (`products/iching-tools/`) | Package/MCP/official-SDK mechanics PASS; product status `PENDING` | v0.2.0 wheel/sdist, six-tool MCP surface, SDK source and clean-wheel checks; live G16 (`deepseek-v4-flash`, 2026-08-15): B2 PASS, B1/B3/B4 FAIL — model-specific; host/product fit pending |
| **Applications portfolio** (`applications/README.md`) | Evidence-graded list + adoption-batch results | POC-07 math PASS; POC-05 PASS/REPLICATED; POC-08 MODEL_DEPENDENT (R-08 FAIL) |
| **QA gates** (`qa/gates.py`) | Mechanical gate runner; report in `qa/gate-report.md` | Claim/promotion status is in `qa/promotion-report.md` |

## Foundational POCs (SDD + TDD and implementation evidence)

| POC | Question | Spec | TDD | Structural success |
|-----|----------|------|-----|--------------------|
| **01** Combinatorial reasoning scaffold | Does a blade-algebra calculator improve LLM combinatorial reasoning ≥20pp? | [spec](docs/specs/poc-01-combinatorial-scaffold-spec.md) | [tdd](docs/tdd/poc-01-combinatorial-scaffold-tdd.md) | ~65% |
| **02** Factorial interaction explorer | Does the blade algebra reproduce 2^k factorial math at 100% correctness? | [spec](docs/specs/poc-02-factorial-explorer-spec.md) | [tdd](docs/tdd/poc-02-factorial-explorer-tdd.md) | ~90% |
| **03** Dialectical reframing engine | Are algebra-grounded reframes more diverse + coherent than free-form? | [spec](docs/specs/poc-03-reframing-engine-spec.md) | [tdd](docs/tdd/poc-03-reframing-engine-tdd.md) | ~75% |
| **04** Interpretable-tag agent memory | Do Bagua role tags deliver quality, stability, filtering, non-interference? | [spec](docs/specs/poc-04-tagged-memory-spec.md) | [tdd](docs/tdd/poc-04-tagged-memory-tdd.md) | ~55% |

## Archived failures (do not re-propose without new evidence)

`archive/experimentation/fails/` — Path A (semantic index), Path B (rotor KG embeddings), plus all failed ideas with their evidence and the salvageable pieces.

## Operating rules (enforced, not aspirational)

1. Every metric ships with its baseline wall.
2. Every feature is ablatable; features that change nothing are deleted.
3. Every number has a claims-ledger row.
4. No number without a split and fixed seed.
5. Front page == internal assessment.
6. Kill criteria are pre-registered; failing a probe is a successful experiment.

## How to run everything

```
python -m pytest v2/probes/path-c/tests/test_tool.py v2/probes/path-c/tests/test_docs.py -q
node v2/probes/path-c/tests/run_js_tests.js
python -m pytest v2/probes/path-d/tests -q        # 28 tests (SimulatedLLM only)
python v2/probes/path-d/run_all.py --offline      # cached reports, no network
python v2/probes/path-d/run_all.py                # REAL LLM run (cached) -> output/
python v2/qa/gates.py          # full gate checklist → v2/qa/gate-report.md
```

## Latest gate report

See [v2/qa/gate-report.md](qa/gate-report.md) (generated after each build cycle).
