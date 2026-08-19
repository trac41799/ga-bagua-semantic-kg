# iching-tools — Validation QC Report

**Independent QA/QC verification (agent-run, then orchestrator fix loop).**

## Verdict: ZERO DEFECTS, ZERO REGRESSIONS (final, 2026-08-09)

## Suite (zero regressions)
- pytest: **29 passed** (coverage/reframe/statediff/mcp), unchanged after all fixes.

## Benchmarks (real LLM, reproducible from caches)
| Benchmark | Measured | Bars | Verdict |
|-----------|----------|------|---------|
| B1 coverage_audit | delta **+2.65** (orig 2.65 → audited 0.00) | ≥ 1.0 (validated 1.15) | PASS, 0 defects |
| B2 reframe | diversity **0.441**, coherence **3.75** | ≥ 0.403 / ≥ 3.5 | PASS, 0 production defects (3 deterministic metric-pipeline degenerates s11/s12/s18, documented, not product defects) |
| B3 state_diff | compliance **20/20**, coverage **1.0000** | 20/20 / ≥ 0.95 | PASS, 0 defects |
| B4 smoke (real mode) | 3 CLIs exit 0 schema-valid; MCP handshake + 3 tool calls OK, empty stderr | zero defects | PASS |

- Numeric drift: none — independently replayed from caches through production code; hand means verified (53/20, 8.813/20, 75/20).
- Ledger: 10 rows, dedupe-by-metric, zero stale FAIL rows.

## Defects found by independent QC and fixed
1. **coverage CLI UnicodeEncodeError** (cp1252 piped stdout, ensure_ascii=False + U+2192) → all three CLIs hardened to `ensure_ascii=True`; re-verified in B4.
2. **claims_ledger.csv stale FAIL rows + duplicates** → `common.ledger_row` rewritten (dedupe-by-metric, header-normalized); ledger re-verified.
3. **smoke report path + hardcoded MCP row** → `bench_smoke.py` writes to `output/` and performs REAL MCP calls.
4. **statediff bench ledger call signature mismatch** → `ledger_row` accepts flexible arg forms.
5. **harness encoding** → gates.py decodes subprocess output as UTF-8 with replacement; QC report rewritten as valid UTF-8.

## Method
All facts verified by execution: full pytest re-run; benchmark programs re-run (cache reuse, 0 new calls); metrics recomputed from raw caches; MCP driven over stdio; ledger diffed against reports. Defects were reported with repro steps and fixed in the loop, then re-verified.
