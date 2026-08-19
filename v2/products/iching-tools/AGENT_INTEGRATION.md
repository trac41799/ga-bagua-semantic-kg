# iching-tools — Agent Integration

How to wire the iching-tools capabilities into an LLM agent loop. The suite is
designed as **framework-capable features**: three deterministic-protocol LLM
tools plus three exact-math tools an agent calls through MCP, consumes as JSON,
and uses to structure its own generation.

## 1. MCP wiring (recommended for agents)

Install the wheel, then run the stdio server (real mode needs the API key;
`--sim` for demos and tests):

```bash
pip install iching-tools
iching-mcp
iching-mcp --sim
```

Client config (Claude Desktop / OpenCode `~/.config/opencode/opencode.json` / Cursor):
```json
{ "mcpServers": { "iching-tools": { "command": "iching-mcp", "args": [] } } }
```

Tools exposed: `coverage_audit(task, plan)`, `reframe(statement)`,
`state_diff(before, after)`, `cl3_evaluate(ops)`,
`interaction_spectrum(points, values)`, and `rotor_transition(ops)`. Each
successful call returns one JSON object or list as one text content item.

## 2. Tool-use pattern (agent loop)

Protocol: `tools/call` with `{name, arguments}` over stdio JSON-RPC. Every
argument object is strict (`additionalProperties: false`); every successful
result is validated before serialization. `cl3_evaluate` and
`rotor_transition` use per-operation discriminated schemas, so each operation
has an exact required field set. Notifications are intentionally silent.

```
1. AGENT: "Which plan aspects might I have missed?"            → tools/call coverage_audit {task, plan}
2. TOOL:  {"task": "...", "original_plan": "...", "audited_plan": "...", "checklist": true}
3. AGENT: uses audited_plan as its draft; cites the 8-role checklist in its reasoning.
```

Reframe example (the flagship — each position is explainable):
```json
{"statement": "We should raise prices.",
 "positions": [
   {"move": "flip0", "state": "Zhen (blade e1, grade 1, sign +1)",
    "reframe": "flip the initiating line: ...",
    "description": "flip the bottom line of the trigram (yang <-> yin)"},
   {"move": "complement", "state": "Qian (blade e123, grade 3, sign +1)",
    "reframe": "the antipode: remove friction instead of raising prices ...",
    "description": "complement -- the antipode of the cube via the Hodge dual (.e123)"}
 ]}
```

State-diff example:
```json
{"before": "cache 94%, latency 120ms", "after": "cache 99%, latency 95ms",
 "aspects": [{"aspect": "cache hit ratio", "before": "94%", "after": "99%"}, ...]}
```

## 3. Error handling contract

| Condition | Behavior |
|-----------|----------|
| Missing/invalid args | `-32602` (MCP) / exit 2 (CLI) |
| LLM failure | `-32000` (MCP) / exit 1 (CLI) |
| state_diff protocol violation (≠ 3 aspects) | typed error, counted as defect, never silent |
| No API key | clear error; `--sim` for tests/demos only |

Provider and LLM-client construction is lazy. Initialization and tool listing,
as well as deterministic algebra/XAI calls, do not require an API key; an LLM
tool reports missing provider configuration when it is invoked.

Raw JSON-RPC error codes are frozen as follows:

| Condition | Code |
|-----------|------|
| Invalid JSON | `-32700` |
| Invalid JSON-RPC envelope | `-32600` |
| Unknown method | `-32601` |
| Unknown tool or invalid arguments | `-32602` |
| Missing provider key | `-32002` |
| LLM transport/protocol failure | `-32000` |

The coverage package's public `validate_audited_plan` validator is used when
available. The current statediff package does not expose an equivalent public
result validator, so the MCP contracts include a narrow local compatibility
adapter for the documented three-aspect shape; no validator package source is
modified by the distribution workstream.

Provider selection is shared by all three LLM clients and CLIs:

```bash
iching-reframe --statement "s" --provider openrouter --model openai/gpt-4o-mini --json
```

The provider key is selected from `DEEPSEEK_API_KEY` first, then
`OPENROUTER_API_KEY`; credentials never appear in reprs, errors, or tool
results. The SDK/distribution checks use `--sim` only and make no real LLM
calls. The benchmark evidence below is historical quality evidence, not a
distribution or semantic guarantee.

## 4. Why this works (the evidence)

Each tool is a productionization of a **pre-registered, validated** capability:
- coverage_audit: Δ+1.15 plan-completeness (validated), Δ+2.65 in production benchmark (B1)
- reframe: DeepSeek baseline diversity 0.453 / coherence 3.75; gpt-4o-mini replication fails at 0.296 / 3.10; production 0.441 / 3.75 (B2) is model-scoped
- state_diff: historical objective comprehension 1.00 vs 0.90 (POC-10), clean no-leakage rerun pending; production coverage 1.0000, 20/20 compliance (B3) is mechanical evidence

See `docs/iching-tools-validation-spec.md` for the benchmark protocol and `output/` for results.
