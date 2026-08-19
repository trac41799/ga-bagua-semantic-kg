---
name: coverage-audit
description: Complete a plan against the 8-role I-Ching completeness checklist (receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative) and validate the output contract. Use when drafting plans, policies, or proposals to reduce missing aspects. Guardrail: coverage_audit.guardrail.v1.
---

# Coverage Audit (8-role checklist)

## When to use
Before finalizing any plan, policy, design, or proposal the agent produces — as a completeness gate.

## Prompt (apply to the draft)

```
Audit and complete the following plan against all 8 aspects:
1. receptive: who/what receives the plan and must accept it?
2. causal: what triggers the plan into action?
3. transmissive: what flows (data, resources, communication) must move?
4. constraining: what limits, budgets, or guardrails apply?
5. clarifying: what must be measured or made visible?
6. influential: what habits or conventions must change?
7. balancing: what feedback loops keep the plan stable?
8. generative: what new capabilities or options does it create?

Plan: <draft>
Rewrite the plan so every aspect is addressed, keeping it concise.
Return exactly one logical line for each role, beginning with its lowercase
marker (for example, `receptive:`); do not omit or duplicate markers.
```

## Tool form (MCP)
`coverage_audit(task, plan)` -> `{task, original_plan, audited_plan, checklist: true}`

Protocol `coverage_audit.guardrail.v1` requires `audited_plan` to be a
non-empty string containing exactly one start-of-logical-line marker for each
role. Markdown headings and numbered list prefixes are accepted. Invalid model
output raises `CoverageProtocolError`; the CLI exits `1` without emitting a
successful JSON result.

## Evidence
- POC-05 (deepseek-chat, 20 frozen tasks): audited missing aspects 2.40 -> 1.25 (Δ+1.15, PASS), prior protocol
- Production benchmark B1: Δ+2.65, 0 defects, prior protocol
- POC-15 replication is also prior-protocol evidence; it does not establish
  semantic completeness under the mechanical guardrail.

## Contract boundary
`checklist: true` confirms only that the output contains the required eight
role markers. It does not prove semantic completeness, plan quality, or
cross-model quality. The eight roles remain a scaffold for generation, with no
semantic or predictive claims.
