# TDD Plan — POC-03: Dialectical Reframing Engine

**Strategy:** Red-green-refactor; `pytest` from `v2/pocs/poc-03/`; SimulatedLLM for tests only; real run cached; 20 frozen statements.

## Test inventory

### T-03.1 Moves (`tests/test_moves.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 03.1.1 | all_positions for every trigram | exactly 8 DISTINCT states, ordered (3 flips, 3 double-flips, 1 complement) | 03.1 |
| 03.1.2 | flip correctness | 24 bit-level cases | 03.1 |
| 03.1.3 | complement = Hodge dual | Kan→Li, Gen→Dui, Zhen→Xun, Kun→Qian (natural convention) | 03.2 |
| 03.1.4 | double-flip = two flips | compose | 03.1 |
| 03.1.5 | no move returns the identity state | distinctness guard | 03.1 |

### T-03.2 Naming (`tests/test_naming.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 03.2.1 | name_position returns text | non-empty | 03.3 |
| 03.2.2 | protocol failures countable | bad output → recorded, not retried | 03.3 |

### T-03.3 Metrics (`tests/test_metrics.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 03.3.1 | diversity hand case | identical vectors → 0; orthogonal → 1 | 03.4 |
| 03.3.2 | coherence in [1,5] | bounded, deterministic on cached judge responses | 03.5 |

### T-03.4 Runner (`tests/test_runner.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 03.4.1 | arm comparison renders | Arm A vs Arm B rows: diversity, coherence | 03.6 |
| 03.4.2 | verdict rows | proxy claim PASS/FAIL vs +0.15 / ≥3.5 | 03.6 |
| 03.4.3 | SimulatedLLM runner | exit 0, reports exist | 03.7 |
| 03.4.4 | ledger + determinism | rows appended; re-run identical | 03.7 |

## Red-green order
1. moves → 2. naming → 3. metrics → 4. runner → 5. real run (cached)

## Definition of done
- `pytest tests/ -q` green (≥ 15 cases)
- `python run_all.py` → arm comparison + verdict
- README: verdict, per-domain breakdown, human-gate link
