"""POC-05: coverage-rubric prompting — 20 frozen tasks, arm A/B prompts, 6-aspect audit rater."""

import hashlib
import json

import llm_client  # noqa: F401  (module must exist for client import)

# (id, domain, task)
TASKS = [
    (1, "product", "Launch a new API product in 3 months."),
    (2, "product", "Add offline mode to a mobile app."),
    (3, "product", "Migrate the billing system to a new provider."),
    (4, "product", "Ship a recommendation feature for the storefront."),
    (5, "product", "Introduce SSO across all internal tools."),
    (6, "incident", "Respond to a database outage affecting checkout."),
    (7, "incident", "Handle a security breach notification process."),
    (8, "incident", "Recover from a corrupted deployment environment."),
    (9, "incident", "Manage a viral misinformation event about the product."),
    (10, "incident", "Mitigate an API abuse / rate-limit bypass incident."),
    (11, "policy", "Draft a remote-work policy for the company."),
    (12, "policy", "Create a vendor data-processing review process."),
    (13, "policy", "Write an AI-safety usage policy for internal teams."),
    (14, "policy", "Design a carbon-reduction policy for the supply chain."),
    (15, "policy", "Establish an open-source contribution policy."),
    (16, "research", "Plan a 6-week user research study for onboarding."),
    (17, "research", "Plan a benchmarking study of three vector databases."),
    (18, "research", "Design an A/B test for the new checkout flow."),
    (19, "research", "Plan an incident postmortem analysis program."),
    (20, "research", "Plan a model-evaluation campaign for a fraud classifier."),
]

ROLES = ["receptive", "causal", "transmissive", "constraining",
         "clarifying", "influential", "balancing", "generative"]

CHECKLIST = (
    "Before finalizing, verify your plan covers all 8 aspects:\n"
    "1. receptive — who/what receives the plan and must accept it?\n"
    "2. causal — what triggers the plan into action?\n"
    "3. transmissive — what flows (data, resources, communication) must move?\n"
    "4. constraining — what limits, budgets, or guardrails apply?\n"
    "5. clarifying — what must be measured or made visible?\n"
    "6. influential — what habits or conventions must change?\n"
    "7. balancing — what feedback loops keep the plan stable?\n"
    "8. generative — what new capabilities or options does it create?"
)

AUDIT_ASPECTS = [
    "constraint handling (budgets, limits, risks)",
    "resource flow (what moves, who delivers)",
    "stakeholder reception (who must accept)",
    "risk balance (feedback loops, failure modes)",
    "clarity of steps (measurable actions)",
    "initiation triggers (how it starts)",
]


def arm_a_prompt(task):
    return [{"role": "system", "content": "Write a concise, actionable plan (max 8 bullets)."},
            {"role": "user", "content": task}]


def arm_b_prompt(task):
    return [{"role": "system", "content": "Write a concise, actionable plan (max 8 bullets). " + CHECKLIST},
            {"role": "user", "content": task}]


def audit_prompt(task, plan):
    aspects = "\n".join(f"{i}. {a}" for i, a in enumerate(AUDIT_ASPECTS))
    return [{"role": "system", "content": "For each of the 6 aspects below, output a JSON object "
                                          "{'0': 0|1, ...} where 1 = the plan addresses it, 0 = missing. "
                                          "Output ONLY the JSON object."},
            {"role": "user", "content": f"Task: {task}\nPlan:\n{plan}\nAspects:\n{aspects}"}]


def parse_audit(text):
    t = text.strip().strip("`").strip()
    if t.startswith("json"):
        t = t[4:].strip()
    data = json.loads(t)
    return [int(data[str(i)]) for i in range(6)]


def freeze_hash():
    return hashlib.sha256(json.dumps(TASKS, ensure_ascii=False).encode()).hexdigest()
