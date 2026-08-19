# TDD — iching-tools (production CLI + MCP)

**Strategy:** per-package pytest suites; SimulatedLLM only (L4); MCP tested by driving the server subprocess over stdio.

## Coverage tool (`coverage/tests/`)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| C1 | CLI sim happy path | exit 0, audited_plan contains 8-role checklist prompt | 1 |
| C2 | CLI --json schema | parses; keys task/original_plan/audited_plan/checklist | 2 |
| C3 | CLI missing config (no --sim) | exit 2, stderr mentions API key | 1 |
| C4 | CLI --api-key flag | config resolution prefers flag | 1 |
| C5 | audit prompt contains 8 roles | receptive..generative all present | 5 |
| C6 | LLM failure → exit 1 | SimulatedLLM raising → rc 1 | 1 |

## Reframe tool (`reframe/tests/`)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| R1 | CLI sim: 8 positions | exit 0; positions length 8; move names flip0..2, double_flip0..2, complement | 3 |
| R2 | CLI --json schema | statement + positions[{move,state,reframe}] | 2 |
| R3 | distinct states | 8 distinct state labels | 3 |
| R4 | complement identity | complement position state is the Hodge-dual trigram (Kan→Li etc.) | 3 |
| R5 | few-shot prompt | ≥2 exemplars per move type in prompt | 3 |
| R6 | missing config exit 2 | same contract | 1 |

## StateDiff tool (`statediff/tests/`)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| S1 | CLI sim: 3 aspects | exit 0; aspects length 3; each has aspect/before/after | 4 |
| S2 | CLI --json schema | before/after/aspects keys | 2 |
| S3 | protocol violation → error | SimulatedLLM emitting non-3-line output → exit 1 with message | 4 |
| S4 | planted delta roundtrip | given (before,after) with known values, aspects contain them (sim) | 4 |
| S5 | missing config exit 2 | same contract | 1 |

## MCP server (`mcp/tests/`)
| ID | Case | Assertion | AC |
|----|------|-----------|----|
| M1 | initialize handshake | protocolVersion echoed, capabilities.tools, serverInfo | 6 |
| M2 | tools/list | exactly 3 tools with inputSchema containing required props | 6 |
| M3 | tools/call coverage_audit | content text non-empty (sim mode) | 6 |
| M4 | tools/call reframe | 8 positions in text | 6 |
| M5 | tools/call state_diff | 3 aspects in text | 6 |
| M6 | unknown tool | −32602 error response | 6 |
| M7 | unknown method | −32601 | 6 |
| M8 | malformed JSON | −32700 | 6 |

## DoD
- `pytest coverage/tests reframe/tests statediff/tests mcp/tests -q` all green (≥ 24 cases)
- All three CLIs run with `--sim` end-to-end; MCP server starts and answers tools/list via stdio
- README (root): install, CLI usage, MCP config snippet (Claude/OpenCode), model note, no-claims statement
