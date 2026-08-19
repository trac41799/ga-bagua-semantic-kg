# TDD Plan — POC-02: Factorial Interaction Design Explorer

**Strategy:** Pure math, no LLM, no network. `pytest` from `v2/pocs/poc-02/`. Correctness is the whole point — tests use an independent brute-force cross-check.

## Test inventory

### T-02.1 Design & blades (`tests/test_design.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 02.1.1 | combinations(3) | 8 bit patterns | 02.1 |
| 02.1.2 | combinations(4) | 16 patterns | 02.1 |
| 02.1.3 | blade_state natural convention | 101→e13, 110→e12, 011→e23 (products of selected vectors) | 02.1 |
| 02.1.4 | grade == Hamming weight | all 8 | 02.1 |

### T-02.2 Contrasts (`tests/test_contrasts.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 02.2.1 | 2³ contrast signs | EXACT match vs brute-force sum-of-products for all 7 effects | 02.2 |
| 02.2.2 | 2⁴ contrast signs | EXACT match, all 15 effects | 02.3 |
| 02.2.3 | main_effect numeric | hand table → known value | 02.4 |
| 02.2.4 | interaction numeric | hand table → known value | 02.4 |
| 02.2.5 | sign via geometric product | product table sign reproduces contrast sign | 02.2 |

### T-02.3 Decomposition (`tests/test_decompose.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 02.3.1 | Möbius vs brute force | 50 random set functions n=3..6, max err ≤ 1e-9 | 02.5 |
| 02.3.2 | blade projection | interaction coefficients recoverable from grade projection | 02.5 |

### T-02.4 Names (`tests/test_names.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 02.4.1 | trigram names | 8/8 canonical | 02.6 |
| 02.4.2 | hexagram names | ≥8 spot-checks canonical King Wen | 02.6 |
| 02.4.3 | naming map consistent with blade_state | bits ↔ trigram ↔ blade | 02.6 |

### T-02.5 Reports & determinism (`tests/test_reports.py`)
| ID | Scenario | Assertion | AC |
|----|----------|-----------|----|
| 02.5.1 | design table renders | 2³ + 2⁴ | 02.7 |
| 02.5.2 | contrast table renders | effects + signs | 02.7 |
| 02.5.3 | decomposition renders | subsets + coefficients | 02.7 |
| 02.5.4 | deterministic | run twice → identical | 02.8 |

## Red-green order
1. design → 2. contrasts → 3. decompose → 4. names → 5. reports

## Definition of done
- `pytest tests/ -q` green (≥ 18 cases)
- `python run_all.py` → reports + correctness verdict (100% agreement)
- README: verdict, usage, limitation (algebra layer only, no inference claims)
