# SDD — iching-tools Validation Benchmarks (production tools vs validated POC baselines)

**Status:** Pre-registered validation | **Date:** 2026-08-08
**Why this exists:** the production tools (coverage_audit, reframe, state_diff) are built and unit-tested with SimulatedLLM, but have NEVER run against the real LLM, and no benchmark ties them to the validated POC effects. This spec defines the factual QA/QC: real-LLM benchmarks against the frozen POC datasets, zero-defect and zero-regression gates.

## 1. Benchmarks (all use the frozen POC inputs — same data, same objective metrics, production code paths)

### B1 — coverage_audit (reproduces POC-05: validated Δ+1.15)
| Item | Value |
|------|-------|
| Inputs | The 20 frozen POC-05 tasks; draft plans = the cached POC-05 Arm-A plans (same drafts the POC audited) |
| Protocol | Production `audit(task, plan)` → audited plan; audit BOTH with the POC-05 6-aspect rater (same prompt, same model) |
| Metric | mean missing aspects: original vs audited |
| **Bar** | **audited ≤ original − 1.0** (no regression vs validated Δ+1.15; tolerance: within ±0.3 of 1.15) |

### B2 — reframe (reproduces POC-08: diversity 0.453, coherence 3.75)
| Item | Value |
|------|-------|
| Inputs | The 20 frozen POC-08 statements |
| Protocol | Production `reframe(statement)` → 8 positions; diversity = mean pairwise 1−cosine on rubric-encoded reframes (same encoder as POC-08); coherence = same 1–5 judge |
| **Bars** | **diversity ≥ 0.403** (POC-08 0.453 − 0.05 regression tolerance) AND **≥ 0.10 delta floor implied**; **coherence ≥ 3.5** |
| Zero-defect | exactly 8 distinct positions on all 20 statements (0 defects) |

### B3 — state_diff (reproduces POC-06/10: compliance 20/20, coverage 1.00)
| Item | Value |
|------|-------|
| Inputs | The 20 frozen POC-06 transitions (before/after + planted deltas) |
| Protocol | Production `summarize(before, after)` → 3 aspects; parse strictly |
| Metrics | (a) parse compliance 20/20 (zero defects); (b) planted-delta coverage = fraction of planted (aspect, before, after) values present in the produced aspects |
| **Bars** | **compliance = 20/20**; **coverage ≥ 0.95** (POC-06 objective: 1.00) |

### B4 — real-mode integration smoke (zero defects)
- MCP server in REAL mode: one `tools/call` per tool (3 calls) — exercises each package's real LLMClient constructor + config + JSON output end-to-end.
- CLIs in real mode: one invocation per tool with `--json` (3 calls) — exit 0, schema-valid output.

## 2. Zero-defect / zero-regression gates
1. **Defects:** any benchmark item that errors, times out, parses wrong, or returns the wrong schema counts as a defect (list with repro); zero defects required.
2. **Regressions:** full workspace pytest (29 cases) green before and after benchmarks; production results within tolerance of the validated POC numbers (B1: Δ≥1.0; B2: ≥0.403/≥3.5; B3: 20/20, ≥0.95). Any below-bar result is a regression to fix, not a "new finding."
3. **Determinism:** benchmark runs cached per call; identical re-run numbers (temperature 0).

## 3. Budget & honesty
- ~60 calls (B1) + ~200 (B2) + ~20 (B3) + 6 (B4) ≈ 290 calls, deepseek-chat, temperature 0, cached.
- All artifacts: `v2/products/iching-tools/output/benchmark_*.md` + `claims_ledger.csv` rows; SimulatedLLM never used in reported numbers (L4).

## 4. ACs (TDD: `../tdd/iching-tools-validation-tdd.md`)
- V-1 benchmark harness loads frozen POC data (hash-verified) and reports missing-data as defects
- V-2 each benchmark writes markdown + ledger rows with bar PASS/FAIL
- V-3 zero-defect list rendered; full pytest suite green
- V-4 real-mode smoke (B4) exit 0 with schema-valid JSON
