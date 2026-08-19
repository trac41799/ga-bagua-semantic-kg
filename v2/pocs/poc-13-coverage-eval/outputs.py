"""POC-13: coverage-eval — 24 frozen outputs (12 good / 12 bad) + rubric/plain evaluation.

Bad outputs verifiably drop >= 2 of the 8 roles (receptive, causal, transmissive,
constraining, clarifying, influential, balancing, generative) — automated check.
"""

import hashlib
import json

ROLES = ["receptive", "causal", "transmissive", "constraining",
         "clarifying", "influential", "balancing", "generative"]

ROLE_KEYWORDS = {
    "receptive": ["accept", "approve", "sign-off", "signoff", "consult", "input", "reception", "endorse"],
    "causal": ["trigger", "kickoff", "initiate", "starts", "start", "event", "fire"],
    "transmissive": ["flow", "route", "move", "pipeline", "through", "channel", "dual-write"],
    "constraining": ["cap", "limit", "budget", "threshold", "ceiling", "enforce", "least-privilege", "contained", "time-box"],
    "clarifying": ["track", "measure", "metric", "dashboard", "visible", "report", "publish", "monitor", "log", "timeline"],
    "influential": ["adopt", "cadence", "nudge", "champion", "playbook", "ritual", "template", "matrix", "workflow"],
    "balancing": ["rebalance", "loop", "rollback", "dual-run", "check-in", "calibrat", "re-certif", "review", "canary"],
    "generative": ["new", "enable", "unlock", "seed", "portal", "option", "create", "pool", "ambassador", "grant", "fund"],
}

# (id, label good|bad, text)
OUTPUTS = [
    # ---- good: address >= 6 roles ----
    (1, "good", "Launch the API in 3 months: get engineering and product sign-off (reception); trigger work via the kickoff and milestone gates; move data through the existing pipeline; hold a 3-month budget cap; track launch metrics weekly; get the team into a release cadence; add a rollback loop; and create the partner portal as the new capability."),
    (2, "good", "Rollout plan: inform stakeholders and collect their constraints; start with the pilot cohort as the trigger; route traffic gradually; enforce the 20% rollback threshold; publish status dashboards; run adoption nudges; rebalance on drift; and enable the new self-serve flow."),
    (3, "good", "Migration plan: stakeholders accept the cutover date; the go-live event is the trigger; data flows through dual-write; spend stays under the migration budget; progress is visible on the tracker; the team adopts the new playbook; weekly check-ins keep it balanced; and we ship the new reporting feature."),
    (4, "good", "Policy rollout: employees are consulted up front; the effective date triggers enforcement; guidance flows through the handbook; exceptions require approval (a limit); compliance metrics are published; champions model the behavior; quarterly reviews adjust it; and the policy enables new remote-work options."),
    (5, "good", "Research plan: the study sponsor approves scope; recruiting starts the clock; insights flow into a shared repo; the sample size is capped; findings are logged weekly; the team shifts its rituals around results; a calibration step keeps scoring consistent; and we seed the follow-up study."),
    (6, "good", "Incident response: the on-call engineer accepts the page; the alert triggers the runbook; status flows to the public page; the blast radius is contained by the kill switch; a timeline is published; the org adopts the postmortem template; load is rebalanced after recovery; and the runbook becomes reusable."),
    (7, "good", "Feature launch: PM and design review the spec (reception); the flag flip is the trigger; usage flows through analytics; the experiment is capped at 10%; metrics are surfaced on the dashboard; the team adopts the new workflow; canary groups balance risk; and the flag system enables future experiments."),
    (8, "good", "Budget cycle: finance signs off on assumptions; the cycle starts with the planning kickoff; numbers flow through the model; a hard ceiling applies to requests; variance is reported monthly; the org adopts the new template; mid-year reviews rebalance; and the process creates an investment pool."),
    (9, "good", "Onboarding revamp: new hires give input (reception); the first day triggers the flow; content flows through the LMS; time-box each module; completion is tracked; managers adopt the checklist; mentors balance workload; and we create a peer-ambassador role."),
    (10, "good", "Vendor change: the security team approves the vendor; the contract start triggers migration; traffic flows through the new provider; a transition budget caps spend; SLA dashboards are public; teams adopt the new runbooks; dual-running rebalances risk; and the integration unlocks new tooling."),
    (11, "good", "Access-control update: the owner approves the policy; the change is triggered by the audit finding; permissions flow through the directory; least-privilege is enforced; audit logs are reviewed weekly; admins adopt the new matrix; periodic re-certification rebalances; and the system enables temporary access grants."),
    (12, "good", "Sustainability program: the board endorses targets; the annual cycle triggers planning; data flows from the meters; a carbon budget caps emissions; progress is published; sites adopt the playbook; offsets rebalance the remainder; and the program seeds an innovation fund."),
    # ---- bad: drop >= 2 roles ----
    (13, "bad", "We should launch the API in 3 months. The team will build it. That is the plan."),
    (14, "bad", "Rollout plan: flip the flag to 100% and monitor. If something breaks, flip it back."),
    (15, "bad", "Migration plan: move the data on Saturday night. Done."),
    (16, "bad", "Policy: employees must follow the new policy. Effective immediately."),
    (17, "bad", "Research plan: interview ten users and write up the findings."),
    (18, "bad", "Incident response: someone should fix the outage quickly and tell people when it is back."),
    (19, "bad", "Feature launch: ship it. Users will figure it out."),
    (20, "bad", "Budget: we need more money this year. Approve the request."),
    (21, "bad", "Onboarding: give new hires the handbook on day one."),
    (22, "bad", "Vendor change: switch providers next month."),
    (23, "bad", "Access control: tighten permissions."),
    (24, "bad", "Sustainability: reduce emissions."),
]


def verify_roles(text):
    """Count of roles covered, via per-role semantic keywords (construction check only)."""
    t = text.lower()
    present = 0
    for role in ROLES:
        if any(kw in t for kw in ROLE_KEYWORDS[role]):
            present += 1
    return present


def freeze_hash():
    return hashlib.sha256(json.dumps(OUTPUTS, ensure_ascii=False).encode()).hexdigest()


def verify_bad_deficient():
    """Automated check: every bad output covers < 4 roles; every good covers >= 4."""
    for oid, label, text in OUTPUTS:
        n = verify_roles(text)
        if label == "bad" and n >= 4:
            return False, oid, n
        if label == "good" and n < 4:
            return False, oid, n
    return True, None, None
