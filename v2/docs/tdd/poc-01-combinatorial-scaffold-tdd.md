# TDD Plan — POC-01: Combinatorial Reasoning Scaffold

**Strategy:** Red-green-refactor; `pytest` from `v2/pocs/poc-01/`; SimulatedLLM for tests only (L4); real run cached. Every AC → ≥1 case.

## Test inventory

### T-01.1 Calculator (`tests/test_calculator.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 01.1.1 | flip all 8 trigrams × 3 lines | bit XOR, 24 cases | 01.1 |
| 01.1.2 | complement all 8 | natural-convention dual (Kan→Li, Gen→Dui, Zhen→Xun, Kun→Qian) | 01.1 |
| 01.1.3 | grade of all 8 | Hamming weight of bits | 01.1 |
| 01.1.4 | product table 8×8 | matches verified PROD_TABLE (64 entries) | 01.1 |
| 01.1.5 | combine(upper, lower) | 6-bit state; 8 spot hexagrams correct | 01.1 |
| 01.1.6 | double-flip | two single flips compose | 01.1 |

### T-01.2 Protocol (`tests/test_protocol.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 01.2.1 | plan_ops returns JSON op list | parseable, ops known | 01.3 |
| 01.2.2 | invalid op raises ProtocolError | no partial execution | 01.2 |
| 01.2.3 | out-of-range args raise | validated | 01.2 |
| 01.2.4 | golden round-trip ×5 | plan→execute matches expected results | 01.3 |
| 01.2.5 | interpret returns text | non-empty | 01.3 |

### T-01.3 Benchmark (`tests/test_problems.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 01.3.1 | 50 problems, 5 categories × 10 | structure | 01.4 |
| 01.3.2 | answer keys frozen | hash file committed | 01.4 |
| 01.3.3 | score() normalization | "7" vs "7.0" vs "seven" handling per key type | 01.4 |

### T-01.4 Runner (`tests/test_runner.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 01.4.1 | accuracy table renders | rows: scaffold, alone, random | 01.5 |
| 01.4.2 | McNemar computed | hand case | 01.5 |
| 01.4.3 | per-category breakdown | 5 rows | 01.5 |
| 01.4.4 | verdict row | PASS/FAIL vs +20pp | 01.5 |
| 01.4.5 | ledger rows | appended | 01.7 |
| 01.4.6 | SimulatedLLM runner | exit 0, reports exist | 01.8 |

## Red-green order
1. calculator → 2. protocol → 3. problems → 4. runner → 5. real run (cached)

## Definition of done
- `pytest tests/ -q` green (≥ 20 cases)
- `python run_all.py` (real LLM, cached) → accuracy + McNemar + per-category + verdict
- README: verdict, per-category table, ledger link, frozen-key hash
