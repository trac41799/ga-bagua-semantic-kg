# SDD — iching-tools: production CLI + MCP server for the 3 validated capabilities

**Status:** Production spec | **Source:** PRODUCTION_ASSESSMENT.md | **Validated bases:** POC-05 (coverage checklist Δ+1.15), POC-08 (reframe Δ+0.290/coh 3.75), POC-06 (state-diff comprehension 1.000)

## 1. Product definition
Three LLM-supportive tools, one workspace `v2/products/iching-tools/`:
- **`iching_coverage`** — audit/improve a plan against the 8-role completeness checklist (validated POC-05 arm-B protocol).
- **`iching_reframe`** — generate exactly 8 algebra-grounded reframes (3 single-flips, 3 double-flips, 1 complement via Hodge dual) with the validated few-shot naming protocol (POC-08).
- **`iching_statediff`** — summarize a state change as exactly 3 aspect lines `aspect: before -> after` (validated POC-06/10 protocol).
- **MCP server** (stdio, dependency-free JSON-RPC) exposing `coverage_audit`, `reframe`, `state_diff`.

## 2. Interface contract (CLI, common to all three)
```
python -m iching_coverage --task "..." --plan "..." [--json] [--api-key K] [--model M] [--sim]
python -m iching_reframe --statement "..." [--json] [--api-key K] [--model M] [--sim]
python -m iching_statediff --before "..." --after "..." [--json] [--api-key K] [--model M] [--sim]
```
- Inputs via flags; `--json` → single JSON object on stdout; plain mode → human text.
- Exit codes: 0 success, 1 runtime/LLM error, 2 usage/config error (missing API key without --sim).
- Env config: `DEEPSEEK_API_KEY` or `OPENROUTER_API_KEY`; `--api-key` overrides; `--model` defaults deepseek-chat.
- `--sim` uses SimulatedLLM (documented: tests/demos only).
- No disk caching in production tools (stateless); retries 2, timeout 120s, budget cap 50k tokens/call.

## 3. MCP contract (stdio JSON-RPC, stdlib only)
Methods: `initialize` (protocolVersion 2024-11-05, capabilities.tools), `notifications/initialized`, `tools/list` (3 tools with inputSchema), `tools/call` (text content result). Errors: parse error −32700, invalid params −32602, method not found −32601, tool error −32000. One message per line (newline-delimited JSON).

## 4. Tool semantics (JSON output shapes)
- `coverage_audit(task, plan)` → `{"task":..., "original_plan":..., "audited_plan":..., "checklist": true}`
- `reframe(statement)` → `{"statement":..., "positions":[{"move":"flip0","state":"Zhen","reframe":"..."} x8]}`
- `state_diff(before, after)` → `{"before":..., "after":..., "aspects":[{"aspect":"...","before":"...","after":"..."} x3]}`

## 5. Security & ops
API key never logged; read-only calls; no network beyond the configured LLM endpoint; MIT license; single validated model documented (deepseek-chat); the 8 roles/moves are naming/structure only — no semantic claims anywhere.

## 6. ACs (TDD: `../tdd/iching-tools-tdd.md`)
- AC-1 CLI: each tool exits 0 with correct output (sim), 2 on missing config, 1 on LLM failure
- AC-2 `--json` output parses and matches the documented schema exactly
- AC-3 reframe returns exactly 8 distinct positions with the 8 move names; complement identity holds
- AC-4 statediff returns exactly 3 aspects; protocol violations counted as errors
- AC-5 coverage applies the 8-role checklist (arm-B prompt contains all 8 roles)
- AC-6 MCP: initialize handshake, tools/list (3 tools + schemas), tools/call per tool, error handling (malformed JSON, unknown tool/method)
- AC-7 no network in tests (SimulatedLLM only); tests green
