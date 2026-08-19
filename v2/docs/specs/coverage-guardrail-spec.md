# SDD - Coverage Audit Guardrail Contract

**Status:** Superseding implementation specification  
**Product:** `v2/products/iching-tools/coverage`  
**Evidence boundary:** POC-05 and POC-15 validate a generation checklist effect; they do not prove semantic correctness of every generated plan.

## 1. Goal

Make `coverage_audit` fail closed when the configured LLM does not return an auditable eight-role plan. The tool remains a bounded generation scaffold, not a semantic classifier or reliability guarantee.

## 2. Public Contract

```python
class CoverageProtocolError(ValueError):
    pass

def validate_audited_plan(audited_plan: object) -> None:
    """Require all eight role markers at the start of logical lines."""

def audit(task: str, plan: str, llm) -> dict[str, object]:
    """Return the existing result shape only after validation."""
```

Successful output remains:

```json
{
  "task": "...",
  "original_plan": "...",
  "audited_plan": "...",
  "checklist": true
}
```

## 3. Validation Rules

The output must be a non-empty string containing one logical-line marker for each role:

```text
receptive:
causal:
transmissive:
constraining:
clarifying:
influential:
balancing:
generative:
```

Markdown headings and numbered list variants are accepted when the role name is at the start of the logical line. Missing, duplicated-only, empty, non-string, or prose-without-role output raises `CoverageProtocolError`.

The protocol identifier is `coverage_audit.guardrail.v1`. Benchmarks must hash the prompt and protocol before using cached responses. Old POC-05 caches do not automatically validate this changed contract.

## 4. CLI and MCP Behavior

- Simulator output must conform to the new role-heading format.
- CLI exits `1` for protocol failure and emits no successful JSON result.
- MCP maps protocol failure to `-32000` with no secret leakage.
- The result's `checklist: true` means contract validation passed, not that semantic completeness was proven.

## 5. Acceptance

| ID | Acceptance |
|---|---|
| C1 | Valid eight-role output passes |
| C2 | Missing/empty/non-string output fails closed |
| C3 | Simulator and CLI exercise the same validator |
| C4 | MCP exposes the same failure as `-32000` |
| C5 | Documentation separates contract validity from semantic quality evidence |
