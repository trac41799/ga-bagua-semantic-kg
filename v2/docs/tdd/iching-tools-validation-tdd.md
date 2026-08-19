# TDD — iching-tools Validation Benchmarks

**Strategy:** benchmark harness per tool (uses frozen POC data, real LLM, cached); unit tests for harness logic (SimulatedLLM only); QA/QC agent independently re-runs everything.

## Harness tests (`bench/tests/`)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| H1 | POC data loaders return 20 items each with hash check | counts + hash match frozen markers | V-1 |
| H2 | 6-aspect audit rater parses strictly | valid JSON → 6 bits; malformed → counted defect | V-1 |
| H3 | diversity metric hand case | identical → 0, orthogonal → 1 | V-2 |
| H4 | planted-delta coverage hand case | both values present → 1.0 | V-2 |
| H5 | ledger writer appends rows | metric/value/bar/date | V-2 |
| H6 | benchmark md renders PASS/FAIL per bar | content check | V-3 |

## Benchmark programs (real mode; not unit-tested — artifact-verified)
| ID | Program | Outputs |
|----|---------|---------|
| B1 | `bench/bench_coverage.py` | `output/benchmark_coverage.md`, ledger rows (original vs audited missing aspects, Δ, bar verdict) |
| B2 | `bench/bench_reframe.py` | `output/benchmark_reframe.md` (diversity, coherence, 8-position defect count) |
| B3 | `bench/bench_statediff.py` | `output/benchmark_statediff.md` (compliance 20/20, coverage) |
| B4 | `bench/bench_smoke.py` | `output/benchmark_smoke.md` (real-mode MCP + CLI calls, exit codes, schema check) |

## QA/QC checklist (independent agent)
1. Re-run full workspace pytest → 29 green (zero regressions)
2. Re-run each benchmark program → identical PASS/FAIL rows vs first run (determinism), bars met
3. Defect list: every error/schema-mismatch with repro steps; zero defects required
4. Claims ledger rows exist for every reported number
