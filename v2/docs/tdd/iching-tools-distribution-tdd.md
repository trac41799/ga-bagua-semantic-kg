# TDD - iching-tools Distribution and MCP Contract Hardening

**Strategy:** provider resolver, then validators, then raw protocol errors, then installed distribution, then official SDK. Each cycle uses public behavior and a focused test.

## Task 1 - Provider routing

**Files:** create `iching_tools/providers.py`, create `tests/test_provider_routing.py`, modify the three LLM clients and three CLIs.

RED tests:

```python
def test_deepseek_environment_selects_deepseek(): ...
```

Run:

```powershell
python -m pytest tests/test_provider_routing.py -q
```

GREEN implementation: route all clients through one `ProviderConfig` and add `--provider` to the package/unified CLI paths.

## Task 2 - Coverage and state-diff result validators

**Files:** create `iching_mcp/contracts.py`, modify coverage/statediff packages, create `mcp/tests/test_contracts.py`.

RED tests:

```python
def test_coverage_result_requires_all_roles(): ...
```

Run:

```powershell
python -m pytest mcp/tests/test_contracts.py -q
```

## Task 3 - Raw MCP errors

**Files:** create/modify `iching_mcp/server.py`, modify `mcp/tests/test_mcp.py`.

RED tests:

```python
def test_missing_argument_is_invalid_params(): ...
```

Run:

```powershell
python -m pytest mcp/tests/test_mcp.py -q
```

GREEN implementation must validate before indexing `args[...]` and must classify errors explicitly.

## Task 4 - Installed package

**Files:** create `iching_mcp/__init__.py`, `iching_mcp/__main__.py`, `iching_mcp/server.py`; convert `mcp/server.py` to wrapper; modify `pyproject.toml`; create `tests/test_distribution.py`.

RED tests:

```python
def test_wheel_contains_iching_mcp(): ...
```

Run:

```powershell
python -m pytest tests/test_distribution.py -q
```

GREEN build:

```powershell
python -m build --wheel --sdist --outdir .tmp-iching-dist
```

The test must inspect the fresh artifact, not the stale checked-in `dist/` directory.

## Task 5 - Official SDK all-six test

**Files:** modify `mcp/tests/sdk_probe.py`, `mcp/tests/test_debug.py`.

RED test: assert exact six-tool set and successful call/result validation for every tool.

Run:

```powershell
python -m pytest mcp/tests/test_debug.py -q
```

GREEN: source and installed-wheel SDK sessions both pass in simulator mode. No `importorskip` may silently skip the required test in CI.

## Task 6 - CI and documentation

Modify `.github/workflows/ci.yml`, `README.md`, `AGENT_INTEGRATION.md`, and skills. CI must install the MCP SDK test dependency, build a fresh wheel, run all tests, and run the six-tool SDK test.

## Definition of Done

- Full iching-tools suite passes.
- Fresh wheel installs without repository paths.
- Official SDK consumes all six tools.
- Provider routing is deterministic and secret-safe.
- Mechanical compatibility is documented separately from LLM quality evidence.
