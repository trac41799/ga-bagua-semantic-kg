# TDD — iching-tools Suite v2

**Strategy:** new root-level tests (itools CLI + flagship reframe); all prior suites must stay green.

## Suite CLI (`tests/test_itools.py` at workspace root)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| S1 | `itools --version` | prints 0.2.0, exit 0 | 1 |
| S2 | `itools --help` | lists coverage/reframe/statediff | 1 |
| S3 | `itools coverage ... --sim` | exit 0, JSON schema keys task/original_plan/audited_plan/checklist | 2 |
| S4 | `itools reframe ... --sim` | exit 0, 8 positions | 2 |
| S5 | `itools statediff ... --sim` | exit 0, aspects length 3 | 2 |
| S6 | delegation parity | `itools coverage` output == package CLI output (sim, same args) | 2 |
| S7 | missing required args | exit 2 with usage error | 2 |
| S8 | no key without --sim | exit 2 | 2 |

## Flagship reframe (`reframe/tests/test_flagship.py`)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| R1 | positions have description | all 8 have non-empty `description` | 3 |
| R2 | description correctness | flip0 → "flip the bottom line"; complement → contains "Hodge dual"; origin → "original position" | 3 |
| R3 | legacy fields intact | move/state/reframe present; count 8 | 3 |

## Docs (`tests/test_docs.py` at workspace root)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| D1 | AGENT_INTEGRATION.md | contains MCP config + a tool-use JSON example | 4 |
| D2 | README v2 | contains positioning vs peers + usage for all three tools + suite install | 5 |

## DoD
- New suites green (≥ 14 cases) AND prior 29 green (zero regressions)
- `itools` verified end-to-end in sim mode; gates extended (G17)
