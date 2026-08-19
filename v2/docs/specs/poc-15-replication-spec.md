# SDD — POC-15: Cross-Model Replication (POC-05 + POC-08 on a second LLM)

**Status:** Pre-registered | **Purpose:** the THESIS replication gate — the two validated generation-protocol claims must survive a second model before any promotion. Second model: `openai/gpt-4o-mini` via OpenRouter (key available), temperature 0, same frozen protocols and bars.

## 1. Pre-registration
| Item | Commitment |
|------|------------|
| R-05 (coverage checklist) | Same 20 frozen POC-05 tasks, same arm-A/arm-B prompts, same 6-aspect audit → bar: B ≤ A − 1.0 missing aspects (validated Δ+1.15 on deepseek) |
| R-08 (reframe) | Same 20 frozen POC-08 statements, same few-shot naming + diversity encoder + judge → bars: diversity ≥ 0.403, coherence ≥ 3.5 (validated 0.453/3.75) |
| Protocol | Identical prompts and metrics as the original POCs; only the model changes; cached; budget-capped |
| Honesty | If a bar fails → the claim is model-dependent (report, do not hide); if both pass → replication achieved |
| Kill | N/A — replication is measurement, not a product claim |

## 2. Interfaces
`v2/pocs/poc-15-replication/`: `bench_coverage.py` (reuses POC-05 protocol + draft plans), `bench_reframe.py` (reuses POC-08 moves/naming/metrics), `run_all.py` (both, verdicts vs original bars, ledger, cache).

## 3. ACs (TDD `poc-15-replication-tdd.md`)
- RP1 frozen data hash-verified (same 20+20 as the originals)
- RP2 prompts byte-identical to the original protocols (diff-checked)
- RP3 both benches render bars + PASS/FAIL vs the original validated numbers
- RP4 determinism on cache; budget caps; tests green
