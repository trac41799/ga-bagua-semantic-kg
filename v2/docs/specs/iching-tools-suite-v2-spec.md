# SDD — iching-tools Suite v2 (cohesion + flagship reframe)

**Status:** Pre-registered | **Purpose:** make the three validated tools shine as ONE cohesive suite (the competitiveness recommendation: bundle as a framework-capable suite, not three loose CLIs), and elevate `reframe` as the flagship with explainable output.

## 1. Suite cohesion (v2 changes)

| Change | Rationale |
|--------|-----------|
| **Unified `itools` CLI** — one entry point, subcommands `coverage`, `reframe`, `statediff`; identical flag conventions everywhere (`--json`, `--sim`, `--api-key`, `--model`) | One install, one mental model — the suite is a product, not a folder of scripts |
| **Suite versioning** — single `iching-tools` version 0.2.0; `itools --version` | Cohesive release identity |
| **Flagship reframe** — each position gains a human-readable `description` (the exact algebraic move, e.g., "flip the bottom line of the trigram") | Explainable output is the defensible mechanism vs prompt-based reframers; makes reframe the standout tool |
| **Agent integration doc** — `AGENT_INTEGRATION.md`: MCP wiring into agent loops (tool-use pattern, JSON consumption, error handling) | Operationalizes "bundled feature of an agent framework" |
| **Suite README v2** — value story, positioning vs peers, one-page usage | Distribution-ready docs |

## 2. Interfaces

### 2.1 `itools` CLI (workspace root, console script `itools`)
```
itools coverage --task T --plan P [--json|--sim|--api-key K|--model M]
itools reframe  --statement S [--json|--sim|--api-key K|--model M]
itools statediff --before B --after A [--json|--sim|--api-key K|--model M]
itools --version | --help
```
- Delegates to the three package CLIs with identical semantics (exit 0/1/2; JSON output schema per tool unchanged).
- `--sim` simulator mode; `--api-key` overrides env (`DEEPSEEK_API_KEY`/`OPENROUTER_API_KEY`); `--model` default deepseek-chat.

### 2.2 reframe flagship
`reframe(statement, llm)` output: `{"statement": ..., "positions": [{"move", "state", "reframe", "description"} x8]}` — `description` from MOVE_DESCRIPTIONS (base move name: strip trailing digits).

## 3. Non-goals
No new LLM calls; no protocol changes (validated protocols stay byte-identical); no new semantics claims; no UI in this iteration.

## 4. ACs (TDD: `../tdd/iching-tools-suite-v2-tdd.md`)
- AC-1 `itools --version` prints 0.2.0; `itools --help` lists 3 subcommands
- AC-2 each subcommand mirrors its package CLI (same flags, same JSON schema, same exit codes) — verified by delegating and diffing outputs (sim mode)
- AC-3 reframe positions each carry `description`; descriptions match MOVE_DESCRIPTIONS (strip-digit lookup); legacy fields unchanged
- AC-4 AGENT_INTEGRATION.md exists with a working MCP tool-use example (agent → tool call → JSON consumption)
- AC-5 suite README v2 exists with positioning + usage
- AC-6 all prior suites green (29 tests) — zero regressions
