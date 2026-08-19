"""iching_coverage - audit a plan against the 8-role completeness checklist.

The production tool takes task + plan as runtime input and asks the LLM to
rewrite/complete the plan so it answers all 8 coverage questions. The audit
protocol validates the returned structure only; it does not establish
semantic completeness.
"""

import re

from .llm_client import LLMClient, SimulatedLLM, resolve_api_key

PROTOCOL_ID = "coverage_audit.guardrail.v1"

ROLES = [
    "receptive",
    "causal",
    "transmissive",
    "constraining",
    "clarifying",
    "influential",
    "balancing",
    "generative",
]

CHECKLIST = (
    "Before finalizing, verify your plan covers all 8 aspects:\n"
    "1. receptive: who/what receives the plan and must accept it?\n"
    "2. causal: what triggers the plan into action?\n"
    "3. transmissive: what flows (data, resources, communication) must move?\n"
    "4. constraining: what limits, budgets, or guardrails apply?\n"
    "5. clarifying: what must be measured or made visible?\n"
    "6. influential: what habits or conventions must change?\n"
    "7. balancing: what feedback loops keep the plan stable?\n"
    "8. generative: what new capabilities or options does it create?"
)

_ROLE_MARKER = re.compile(
    r"^[ \t]*(?:#{1,6}[ \t]+)?(?:\d+[.)][ \t]+)?"
    r"(?P<role>receptive|causal|transmissive|constraining|clarifying|"
    r"influential|balancing|generative)[ \t]*:"
)


class CoverageProtocolError(ValueError):
    """Raised when an audited plan does not satisfy the coverage protocol."""


def validate_audited_plan(audited_plan: object) -> None:
    """Require exactly one logical-line marker for every coverage role."""
    if not isinstance(audited_plan, str) or not audited_plan.strip():
        raise CoverageProtocolError(
            "audited plan must be a non-empty string with all eight role markers"
        )

    roles = [
        match.group("role")
        for line in audited_plan.splitlines()
        if (match := _ROLE_MARKER.match(line)) is not None
    ]
    counts = {role: roles.count(role) for role in ROLES}
    missing = [role for role, count in counts.items() if count == 0]
    duplicated = [role for role, count in counts.items() if count > 1]
    if missing or duplicated:
        details = []
        if missing:
            details.append("missing: " + ", ".join(missing))
        if duplicated:
            details.append("duplicated: " + ", ".join(duplicated))
        raise CoverageProtocolError(
            "audited plan must contain exactly one marker for each role ("
            + "; ".join(details)
            + ")"
        )


def audit_prompt(task, plan):
    """Build the chat-completions messages for the coverage audit.

    Contains all 8 roles as coverage questions and asks the LLM to
    rewrite/complete the plan.
    """
    system = (
        "Rewrite or complete the given plan so it answers every question in "
        "the checklist below. Keep it concise and actionable. Return exactly "
        "one logical line for each role, beginning with its lowercase marker "
        "such as `receptive:`; do not omit or duplicate markers.\n\n"
        + CHECKLIST
    )
    return [
        {"role": "system", "content": system},
        {"role": "user", "content": f"Task: {task}\n\nPlan:\n{plan}"},
    ]


def audit(task: str, plan: str, llm) -> dict[str, object]:
    """Audit `plan` for `task` using `llm`, returning the documented shape."""
    audited_plan = llm.complete(audit_prompt(task, plan))
    validate_audited_plan(audited_plan)
    return {
        "task": task,
        "original_plan": plan,
        "audited_plan": audited_plan,
        "checklist": True,
    }


__all__ = [
    "PROTOCOL_ID",
    "ROLES",
    "CHECKLIST",
    "CoverageProtocolError",
    "audit",
    "audit_prompt",
    "validate_audited_plan",
    "LLMClient",
    "SimulatedLLM",
    "resolve_api_key",
]
