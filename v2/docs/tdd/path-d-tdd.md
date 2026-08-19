# TDD Plan — Path D (The Decisive Experiment)

**Strategy:** Red-green-refactor. `pytest` from `v2/probes/path-d/`. Network REQUIRED only for the real-LLM run (`run_all.py`); unit tests use `SimulatedLLM` (L4: fixtures only for tests). Every AC maps to ≥1 test case.

## Test inventory (AC → cases)

### T-D1 Rubric (`tests/test_rubric.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| D1.1 | parse valid JSON array | 8 floats returned | D1 |
| D1.2 | parse with code fence | stripped, parsed | D1 |
| D1.3 | parse malformed | raises clear RubricError, no partial data | D1 |
| D1.4 | parse wrong length | raises | D1 |
| D1.5 | parse non-numeric | raises | D1 |
| D1.6 | prompt contains 8 roles + output spec | encode_prompt coverage | D1 |

### T-D2 LLM client (`tests/test_llm_client.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| D2.1 | SimulatedLLM deterministic | same call → same output + usage | D2 |
| D2.2 | SimulatedLLM encode → 8 normalized floats | unit norm | D2 |
| D2.3 | real client request built correctly (mocked transport) | auth header, model, JSON body | D2 |
| D2.4 | real client parses usage from response | prompt/completion tokens | D2 |
| D2.5 | retry on failure | fails twice then succeeds → returns | D2 |
| D2.6 | budget cap | exceeding cap raises BudgetError | D2 |

### T-D3 Retrieval + baselines (`tests/test_retrieval_baselines.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| D3.1 | cosine top-K | hand-computed ordering | D3 |
| D3.2 | TF-IDF scores | hand-computed on 3 docs | D3 |
| D3.3 | BM25 scores | hand-computed on 3 docs | D3 |
| D3.4 | random top-K seeded | deterministic | D3 |
| D3.5 | cosine of identical vectors = 1 | sanity | D3 |

### T-D4 Eval (`tests/test_eval.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| D4.1 | R@5/R@10/MRR | hand-computed on toy ranked lists | D4 |
| D4.2 | break-even arithmetic | known tokens → known queries | D5 |
| D4.3 | savings at 10/50/200 | formula check | D5 |
| D4.4 | ledger rows | metric/value/baseline/split/seed/date | D9 |
| D4.5 | verdict rows render | D1/D2/D3 each PASS/FAIL/PENDING | D7 |

### T-D5 Caching + offline (`tests/test_cache_offline.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| D5.1 | encode with cache | second run calls API 0 times (mock counter) | D6 |
| D5.2 | offline mode with cached encodings | baselines + retrieval run, no network module touched | D8 |

### T-D6 End-to-end (`tests/test_run_all.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| D6.1 | run_all with SimulatedLLM injected | exit 0, reports exist, verdicts rendered | D7 |
| D6.2 | run_all without network (offline) | exit 0, status row "PENDING" when no cached encodings | D8 |

## Red-green-refactor order

1. `test_rubric.py` → `rubric.py`
2. `test_llm_client.py` → `llm_client.py` (+ `SimulatedLLM`)
3. `test_retrieval_baselines.py` → `retrieval.py`, `baselines.py`
4. `test_eval.py` → `eval.py`
5. `test_cache_offline.py` → `cache.py` + offline paths
6. `test_run_all.py` → `run_all.py` (SimulatedLLM injection for tests)

## Definition of done

- `pytest tests/ -q` all green (≥ 25 cases)
- `python run_all.py --offline` works with cached encodings (no network)
- `python run_all.py` (real LLM) produces: retrieval_metrics.md, pipeline.md, token_economics.md, gate_summary.md, claims_ledger.csv, with explicit D1/D2/D3 verdicts
- README: honest framing; every number links to a ledger row; single-annotator ground truth + substring matching disclosed
