# TDD Plan — POC-04: Interpretable-Tag Agent Memory

**Strategy:** Red-green-refactor; `pytest` from `v2/pocs/poc-04/`; SimulatedLLM for tests only; real tag runs cached; reuses Path D corpus + client patterns.

## Test inventory

### T-04.1 Tags (`tests/test_tags.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 04.1.1 | parse_tags valid | 8 roles, values in [-1,1] | 04.1 |
| 04.1.2 | parse_tags malformed | typed TagError | 04.1 |
| 04.1.3 | parse_tags wrong keys | missing role → error | 04.1 |
| 04.1.4 | dominant_role hand cases | strong role wins; ties → first | 04.2 |

### T-04.2 Metrics (`tests/test_metrics.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 04.2.1 | consistency hand case | 8/10 dominant-role matches → 0.8 | 04.3 |
| 04.2.2 | filter precision hand case | 3 relevant of 5 retrieved → 0.6 | 04.4 |
| 04.2.3 | recall included | 3 of 4 relevant → 0.75 | 04.4 |
| 04.2.4 | non-interference hand case | identical rankings → True; different → False | 04.5 |

### T-04.3 Runner (`tests/test_runner.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 04.3.1 | all five reports render | tag_quality, stability, filtering, noninterference, gate_summary | 04.6 |
| 04.3.2 | verdict rows | quality ≥80%, stability ≥80%, precision ≥0.5, non-interference True | 04.6 |
| 04.3.3 | SimulatedLLM runner | exit 0 | 04.7 |
| 04.3.4 | ledger + determinism | rows appended; cached re-run identical | 04.7 |

## Red-green order
1. tags → 2. metrics → 3. runner → 4. real run (cached; human reference tags authored first)

## Definition of done
- `pytest tests/ -q` green (≥ 12 cases)
- `python run_all.py` → five reports + verdicts
- README: verdicts, retrieval-accuracy dependency documented (embeddings), human-tag limitation disclosed
