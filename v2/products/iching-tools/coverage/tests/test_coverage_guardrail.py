"""Focused tests for the coverage audit output contract."""

import pytest

from iching_coverage import ROLES, SimulatedLLM, audit, validate_audited_plan
from iching_coverage import CoverageProtocolError


def test_all_eight_role_headings_pass():
    audited_plan = "\n".join(f"{role}: covered" for role in ROLES)

    assert validate_audited_plan(audited_plan) is None


def test_markdown_headings_and_numbered_role_lines_pass():
    heading_plan = "\n".join(f"## {role}: covered" for role in ROLES)
    dotted_plan = "\n".join(
        f"{index}. {role}: covered" for index, role in enumerate(ROLES, start=1)
    )
    parenthesized_plan = "\n".join(
        f"{index}) {role}: covered" for index, role in enumerate(ROLES, start=1)
    )

    validate_audited_plan(heading_plan)
    validate_audited_plan(dotted_plan)
    validate_audited_plan(parenthesized_plan)


@pytest.mark.parametrize(
    "audited_plan",
    [
        "",
        "just prose without any role markers",
        "\n".join(f"Details about {role}: covered" for role in ROLES),
        "receptive: covered\ncausal: covered",
        "\n".join(f"{role}: covered" for role in ROLES[:-1]),
        "\n".join(f"receptive: covered" for _ in ROLES),
        "\n".join(f"{role}: covered" for role in ROLES + ["receptive"]),
        None,
        42,
    ],
)
def test_invalid_audited_plan_raises_protocol_error(audited_plan):
    with pytest.raises(CoverageProtocolError):
        validate_audited_plan(audited_plan)


def test_audit_fails_closed_before_returning_success_for_invalid_output():
    class InvalidLLM:
        def complete(self, messages):
            return "The plan is complete, but has no role headings."

    with pytest.raises(CoverageProtocolError):
        audit("task", "plan", InvalidLLM())


def test_simulator_output_satisfies_the_same_validator():
    audited_plan = SimulatedLLM().complete([])

    assert validate_audited_plan(audited_plan) is None
