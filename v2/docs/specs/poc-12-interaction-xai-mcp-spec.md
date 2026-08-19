# SDD — POC-12: Interaction-Spectrum MCP Tool (POC-07 math as a tool)

**Status:** Pre-registered | **Purpose:** expose the validated interaction-recovery math (POC-07: exact, 2.2e-16) as an MCP tool for ML-agent feature-interaction analysis. Deterministic; no LLM in the math path.

## 1. Pre-registration
| Item | Commitment |
|------|------------|
| Tool contract | `interaction_spectrum(points, values)` — points: list of ±1 level vectors (2^k design), values: responses; returns {subset_mask: coefficient} via the Walsh–Hadamard/contrast transform; `identify(spectrum, tol)` → dominant interaction subsets |
| Correctness | Planted interactions recovered exactly (≤1e-9), zero false positives — the POC-07 bar |
| Input contract | Strict: points must be ±1 vectors, all same length, len(values)==len(points); else typed error |
| Zero hallucination | Pure algebra; programmatic exactness proof |
| Kill | Recovery failure on the planted test → dies |

## 2. Interfaces
`iching_xai/__init__.py`: `interaction_spectrum(points, values) -> dict`, `identify(spectrum, tol=1e-6) -> list`. MCP tool registration.

## 3. ACs (TDD `poc-12-interaction-xai-mcp-tdd.md`)
- X1 exact recovery on the POC-07 planted function (err ≤1e-9, 3/3 subsets, 0 FPs)
- X2 strict input validation (non-±1, length mismatch → typed errors)
- X3 MCP tools/list + tools/call (sim) returns the spectrum JSON
- X4 MCP-SDK client call works
- X5 tests green; zero LLM calls
