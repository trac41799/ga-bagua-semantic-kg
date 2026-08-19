# TDD - Coverage Audit Guardrail Contract

**Strategy:** public `audit()` behavior first, then CLI and MCP error propagation; do not weaken the validator to preserve stale simulator output.

## Task 1 - Validator RED/GREEN

**Files:** `coverage/iching_coverage/__init__.py`, `coverage/tests/test_coverage_guardrail.py`.

RED tests:

```python
def test_all_eight_role_headings_pass(): ...
```

Run:

```powershell
python -m pytest coverage/tests/test_coverage_guardrail.py -q
```

GREEN implementation must call `validate_audited_plan()` before returning the result.

## Task 2 - Simulator and CLI

**Files:** `coverage/iching_coverage/llm_client.py`, `coverage/iching_coverage/cli.py`, `coverage/tests/test_coverage.py`.

Add a simulator response containing all eight markers. Add a test that an invalid simulator response returns exit code `1` and no successful result.

Run:

```powershell
python -m pytest coverage/tests -q
python -m iching_coverage --task t --plan p --sim
```

## Task 3 - MCP propagation

**Files:** `iching_mcp/contracts.py` or the current MCP implementation, `mcp/tests/test_contracts.py`.

Add a test that a coverage protocol exception is serialized as error code `-32000`, and that the message contains no API key.

Run:

```powershell
python -m pytest mcp/tests/test_contracts.py -q
```

## Task 4 - Evidence documentation

Update the coverage skill and product README with `coverage_audit.guardrail.v1`, the fail-closed behavior, and the statement that this is mechanical output validation rather than semantic completeness proof.

## Definition of Done

- All focused coverage tests pass.
- Invalid model output cannot be reported as a successful audit.
- Existing POC-05/POC-15 claims remain labeled as prior-protocol evidence.
