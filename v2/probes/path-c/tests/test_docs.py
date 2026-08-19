"""T-C3 documentation and human-gate protocol checks.

Run: python -m pytest tests/test_docs.py -q
"""

import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
README_PATH = os.path.join(ROOT, "README.md")
LESSON_PLAN_PATH = os.path.join(ROOT, "docs", "lesson-plan.md")
GATE_REPORT_PATH = os.path.join(ROOT, "output", "human-gate-report.md")

LIKERT_QUESTIONS = [
    "The visualization helped me understand what a blade is and what its grade means.",
    "The product table made the geometric product (signs and blade results) clear.",
    "The line-flip module helped me see how flipping a line relates to multiplying by a basis vector.",
    "The rotor demo helped me understand the sandwich product a' = R a R~.",
    "Overall, I would recommend this tool to someone learning geometric algebra.",
]


def test_c31_readme_has_open_instructions_human_gate_and_no_claims_statement():
    assert os.path.exists(README_PATH), "README.md missing"
    with open(README_PATH, encoding="utf-8") as f:
        readme = f.read()
    assert "file://" in readme, "README must document file:// open instructions"
    assert "double-click" in readme.lower(), "README must mention double-click opening"
    assert "human-gate" in readme.lower() or "human gate" in readme.lower(), \
        "README must document the human-gate protocol"
    assert "output/human-gate-report.md" in readme, \
        "README must point to the human-gate report template path"
    assert "60%" in readme and "70%" in readme, \
        "README must state the pre-registered thresholds (>=60%, >=70%)"
    assert "no semantic claims" in readme.lower(), \
        "README must carry the explicit no-semantic-claims statement"


def test_c32_lesson_plan_exists_with_session_protocol():
    assert os.path.exists(LESSON_PLAN_PATH), "docs/lesson-plan.md missing"
    with open(LESSON_PLAN_PATH, encoding="utf-8") as f:
        plan = f.read()
    assert "20" in plan, "lesson plan must specify the 20-minute guided session"
    assert "pre-quiz" in plan.lower(), "lesson plan must include the pre-quiz phase"
    assert "post-quiz" in plan.lower(), "lesson plan must include the post-quiz phase"
    assert "likert" in plan.lower(), "lesson plan must include the Likert feedback form"
    for q in LIKERT_QUESTIONS:
        assert q in plan, f"lesson plan must list the verbatim Likert question: {q[:50]}..."
    headings = re.findall(r"^### \d+\.", plan, re.MULTILINE)
    assert len(headings) == 8, f"lesson plan should walk through all 8 modules, found {len(headings)}"


def test_c33_human_gate_template_with_pre_registered_thresholds():
    assert os.path.exists(GATE_REPORT_PATH), "output/human-gate-report.md missing"
    with open(GATE_REPORT_PATH, encoding="utf-8") as f:
        report = f.read()
    assert "60%" in report, "template must state the >=60% improvement threshold"
    assert "70%" in report, "template must state the >=70% Likert threshold"
    assert "4/5" in report, "template must state the Likert >=4/5 criterion"
    assert "pre" in report.lower() and "post" in report.lower(), \
        "template must record pre and post quiz scores"
    assert "session log" in report.lower(), "template must have a session log section"
    assert "free text" in report.lower() or "free-text" in report.lower(), \
        "template must have free-text feedback"
    for q in LIKERT_QUESTIONS:
        assert q in report, f"template must list the verbatim Likert question: {q[:50]}..."
