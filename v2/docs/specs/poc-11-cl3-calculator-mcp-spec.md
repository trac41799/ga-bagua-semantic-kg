# SDD — POC-11: Cl3 Calculator MCP Tool (structured-op evaluation)

**Status:** Pre-registered | **Purpose:** the POC-01 fix — the Cl3 calculator was proven exact (0 execution failures); the failure was free-form plan parsing. A structured tool contract (LLM constructs JSON ops, the tool validates strictly) makes exact combinatorial reasoning reliably available to agents.

## 1. Pre-registration
| Item | Commitment |
|------|------------|
| Tool contract | `cl3_evaluate(ops)` — ops: JSON list of {op, args}; ops: flip/double_flip/complement/grade/product/combine/count_blades; strict validation (unknown op/arg → error, NO partial execution) |
| Correctness | All ops EXACT vs the verified PROD_TABLE and bit semantics (0 tolerance) — deterministic, no LLM in the tool |
| Zero hallucination | The tool's outputs are pure algebra; tests prove exactness programmatically |
| Parity | Identical results to the POC-01 verified calculator on 100 random op sequences |
| Kill | Any mismatch → dies (the tool would be wrong) |

## 2. Interfaces
`iching_cl3calc/__init__.py`: `evaluate(ops: list[dict]) -> dict{result, steps}`; `validate(ops) -> None` (raises typed errors). MCP tool registration in `mcp/server.py`.

## 3. ACs (TDD `poc-11-cl3-calculator-mcp-tdd.md`)
- C1 op correctness: 24 flips, 8 complements, 64 products, grades, combines (exact)
- C2 strict validation: unknown op / bad arg / partial-execution guard
- C3 parity: 100 random sequences == reference implementation
- C4 MCP: tools/list includes cl3_evaluate with inputSchema; tools/call returns exact result (sim)
- C5 MCP-SDK: official `mcp.client.stdio` client can call it (real compatibility)
- C6 tests green; zero LLM calls in the tool path
