# iching-tools — the validated I-Ching capability suite (v0.2.0)

Three LLM-supportive tools and three exact-math tools, built from **pre-registered, real-LLM-validated** I-Ching applications where applicable, delivered as one cohesive suite: a unified `itools` CLI, six tool packages, and one dependency-free MCP server.

**Positioning:** where every generic prompt-based ideation/planning tool offers *arbitrary* prompts, iching-tools offers **algebra-grounded structure with banked evidence**: exactly 8 explainable reframing moves, an 8-role completeness checklist, and a strict 3-aspect state-diff protocol — each benchmarked against its own validated baseline (B1–B4). The I-Ching structure is a **scaffold for generation — never a source of meaning** (no semantic claims, house rule). Current live status (2026-08-15, `deepseek-v4-flash`): B2 PASS; B1/B3/B4 FAIL — see [Validation](#validation-2026-08-15-current-live-state).

## Tools

| Tool | What it does | Validated evidence |
|------|--------------|--------------------|
| **`coverage_audit`** | Completes a plan against the 8-role checklist (receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative) | Prior-protocol POC-05 Δ+1.15; production Δ+2.65 (B1, deepseek-chat) PASS; v4-flash rerun FAIL (rater protocol, 20 defects) |
| **`reframe`** ⭐ flagship | Exactly 8 algebra-grounded reframes — origin, 3 single-line flips, 3 double-flips, complement (Hodge dual) — **each position explainable** (move description) | DeepSeek POC-08 baseline 0.453/3.75; R-08 gpt-4o-mini replication 0.296/3.10 FAIL; v4-flash B2 rerun 0.538/4.15/0 defects PASS |
| **`state_diff`** | Summarizes a state change as exactly 3 aspect lines `aspect: before -> after` | Historical POC-10 comprehension 1.00 vs 0.90; repaired B3 live rerun (v4-flash): 18/20, coverage 0.90 FAIL; second-model replication pending |
| **`cl3_evaluate`** | Executes a strictly validated batch of exact Cl(3) blade operations | Deterministic algebra; no LLM |
| **`interaction_spectrum`** | Recovers Walsh-Hadamard interaction coefficients and identified masks | Deterministic algebra; no LLM |
| **`rotor_transition`** | Executes exact rotor compose, invert, apply, and distance operations | Deterministic algebra; no LLM |

## Install & usage

```bash
pip install -e .            # or: pip install iching-tools (when published)

# unified CLI (identical flags: --json | --sim | --api-key K | --model M | --provider P)
itools coverage  --task "launch an API product" --plan "Build it." --json
itools reframe   --statement "We should raise prices." --json
itools statediff --before "cache 94%, latency 120ms" --after "cache 99%, latency 95ms" --json
itools --version

# per-package CLIs (same semantics)
python -m iching_coverage --task T --plan P --json
python -m iching_reframe  --statement S --json
python -m iching_statediff --before B --after A --json
```

- API key: env `DEEPSEEK_API_KEY` / `OPENROUTER_API_KEY` or `--api-key`; `--sim` = deterministic simulator (tests/demos only).
- Provider: `--provider deepseek|openrouter` is optional. Without it,
  `DEEPSEEK_API_KEY` wins and `OPENROUTER_API_KEY` is the fallback. An explicit
  `--api-key` keeps the historical DeepSeek default unless
  `--provider openrouter` is also set. Model strings, including
  `openai/gpt-4o-mini`, are passed through unchanged.
- Model override: set `ICHING_MODEL` (env) to run all tools/benches on another
  model, e.g. `$env:ICHING_MODEL='deepseek-v4-flash'`. Benchmarks cache per
  model (`.cache_*.<model>.json`) so responses never mix across models.
- Exit codes: 0 ok, 1 runtime/LLM error, 2 usage/config error.
- Validated model: deepseek-chat (temperature 0); `deepseek-v4-flash` tested
  2026-08-15 (B2 passes, B1/B3/B4 fail — see Validation).

### Coverage guardrail contract

`coverage_audit` uses protocol identifier `coverage_audit.guardrail.v1`. The
audit returns the documented JSON shape only when the model response is a
non-empty string with exactly one marker at the start of a logical line for
each of the eight roles. Markdown headings and numbered list prefixes are
accepted. Missing, duplicated, non-string, empty, or prose-only output raises
`CoverageProtocolError`; the CLI exits `1` and emits no successful JSON result.

`checklist: true` means that this mechanical output contract passed. It is not
a proof of semantic completeness, plan quality, or cross-model reliability.
POC-05, POC-15, and B1 measurements are prior-protocol generation or
benchmark evidence and must not be read as validation of every generated plan
under this guardrail. Cached benchmark consumers must include both the prompt
and `coverage_audit.guardrail.v1` in their cache key; old POC-05 caches do not
automatically satisfy the changed contract.

## MCP server (stdio, stdlib-only)

```bash
iching-mcp            # installed entry point, real mode (API key env)
iching-mcp --sim      # deterministic simulator
python -m iching_mcp --sim
```

The installed package exposes exactly six tools: `coverage_audit`, `reframe`,
`state_diff`, `cl3_evaluate`, `interaction_spectrum`, and `rotor_transition`.
The official MCP SDK is a test dependency only; the runtime server uses the
Python standard library. `mcp/server.py` remains a source-tree compatibility
wrapper and is not included in the wheel. Client config + agent loop pattern:
`AGENT_INTEGRATION.md`.

Missing or extra tool arguments return JSON-RPC `-32602`. Invalid JSON is
`-32700`, an invalid JSON-RPC envelope is `-32600`, an unknown method is
`-32601`, missing provider configuration is `-32002`, and LLM transport or
protocol failures are `-32000`. Provider/client setup is lazy: initialize,
tools/list, and the three deterministic math tools work without API keys;
missing provider configuration is reported only when an LLM-backed tool is
called. JSON-RPC notifications produce no response, and non-finite JSON
numbers are rejected.

The `cl3_evaluate` and `rotor_transition` input schemas are discriminated by
operation and reject missing or extra operation fields. Successful results are
validated independently before they are serialized.

Distribution evidence is mechanical and separate from LLM quality evidence:
the test suite builds a fresh wheel and sdist, checks the six packages and
`iching-mcp` entry point, installs the wheel with repository paths removed,
and drives all six tools through the official MCP SDK in simulator mode. It
does not make real LLM calls.

## Validation (accurate, relevant, reproducible)

All production benchmarks run against the **frozen POC datasets** with the same objective metrics as the validating experiments — real LLM, cached, deterministic (B1–B4 in `output/benchmark_*.md`, ledger `output/claims_ledger.csv`).

### Validation (2026-08-15, current live state)

Latest full G16 run uses `ICHING_MODEL=deepseek-v4-flash` (direct DeepSeek API, temperature 0). Benches exit non-zero on FAIL verdict, and the G16 gate parses the verdict artifacts, not just exit codes.

| Benchmark | Result (v4-flash) | Bar | Verdict |
|-----------|-----|-----|---------|
| B1 coverage Δ missing aspects (prior protocol) | delta +3.33 on 6/20 tasks; **20 rater-parse defects** | ≥ 1.0, 0 defects | **FAIL** |
| B2 reframe diversity / coherence | **0.538 / 4.15, 0 defects** | ≥ 0.403 / ≥ 3.5, 0 defects | **PASS** |
| B3 state_diff compliance / coverage (repaired protocol) | 18/20 / 0.9000 | 20/20 / ≥ 0.95 | **FAIL** |
| B4 real-mode smoke (CLI + MCP) | statediff CLI/MCP strict-parse fail ("expected exactly 3 aspect lines, got 0") | zero defects | **FAIL** |

### Historical baseline (deepseek-chat, prior production runs)

| Benchmark | Result | Bar | Verdict |
|-----------|--------|-----|---------|
| B1 coverage Δ missing aspects (prior protocol) | **+2.65** | ≥ 1.0 | PASS |
| B2 reframe diversity / coherence | **0.441 / 3.75** | ≥ 0.403 / ≥ 3.5 | PASS |
| B3 state_diff compliance / coverage | **20/20 / 1.0000** | 20/20 / ≥ 0.95 | PASS |
| B4 real-mode smoke (CLI + MCP) | **PASS** | zero defects | PASS |
| Regression suite | simulator and deterministic contract tests | zero network calls | PASS |

B1/B3/B4 failing under v4-flash is honest, model-specific evidence: the strict
rater/parser protocols are not yet robust to this model's output format. It does
not retroactively invalidate the deepseek-chat baseline, and it does not promote
v4-flash. `output/benchmark_*.md` holds the per-task detail for both states.

## Docs

- `AGENT_INTEGRATION.md` — MCP wiring + tool-use pattern
- Validation spec/TDD: `v2/docs/specs/iching-tools-validation-spec.md`, `...-suite-v2-spec.md`
- Evidence base: `v2/THESIS.md` (the versioned working thesis)

## Production notes

- No disk caching (stateless calls); retries 2, timeout 120s, 50k-token budget per call; API key never logged.
- **No semantic claims**: the roles/moves are structure and naming; nothing here predicts or classifies.

## Model-dependence caveat

- **coverage_audit / state_diff: validated on deepseek-chat; gpt-4o-mini replication of coverage PASSED (R-05).** Under `deepseek-v4-flash` (2026-08-15) both FAIL: coverage rater-parse defects (20), statediff repaired 18/20 (coverage 0.90) + CLI/MCP smoke strict-parse failure. Model-sensitive; not model-general.
- **reframe: validated on deepseek-chat AND deepseek-v4-flash (B2 PASS both).** On gpt-4o-mini the claim does NOT replicate (diversity 0.296 vs bar 0.403, coherence 3.10 vs 3.5). The naming protocol is model-sensitive on the OpenAI side; use reframe with a DeepSeek model, or treat other-model output as unvalidated.
