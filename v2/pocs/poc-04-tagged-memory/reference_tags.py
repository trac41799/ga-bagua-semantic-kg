"""Human reference tags: 30 corpus items (10 software, 8 business, 6 biology,
6 governance), each with a dominant role + full 8-strength tags.

ANNOTATION RECORD (pre-registration compliance)
- Single annotator: the POC-04 builder (documented limitation, per SDD §1).
- Authored BEFORE any LLM tagging run; no LLM output was consulted.
- Rubric: dominant role = the role whose function the description most serves,
  per the ROLE_GLOSS semantics in tags.py. Strength sign: positive = embodies
  the role, negative = actively suppresses it, 0 = neutral.
- Canonical anchor examples per role: receptive (Cache Layer, Seed Bank,
  National Archive), causal (Event Trigger, Chaos Injector), transmissive
  (Message Queue, Pollinator), constraining (Rate Limiter, Regulator),
  clarifying (Monitoring Dashboard, Auditor), influential (Loyalty Program,
  Policy Think Tank), balancing (Load Balancer, Homeostasis),
  generative (Photosynthesis, Pricing Engine).
- Freeze: reference_tags.sha256 holds the sha256 of the canonical serialization
  of REFERENCE_TAGS; verify_freeze() rejects any drift after authoring.
"""

import hashlib
import json
import os

from tags import ROLES, dominant_role

REFERENCE_TAGS = [
    # ---- software systems (10) ----
    {"id": 0, "name": "Rate Limiter", "domain": "software", "dominant": "constraining",
     "strengths": {"receptive": 0.0, "causal": -0.2, "transmissive": -0.6,
                   "constraining": 0.95, "clarifying": 0.1, "influential": 0.0,
                   "balancing": 0.2, "generative": -0.3}},
    {"id": 1, "name": "Message Queue", "domain": "software", "dominant": "transmissive",
     "strengths": {"receptive": 0.3, "causal": 0.1, "transmissive": 0.9,
                   "constraining": -0.1, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.5, "generative": -0.2}},
    {"id": 3, "name": "Load Balancer", "domain": "software", "dominant": "balancing",
     "strengths": {"receptive": -0.1, "causal": 0.0, "transmissive": 0.4,
                   "constraining": 0.1, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.9, "generative": -0.3}},
    {"id": 4, "name": "Circuit Breaker", "domain": "software", "dominant": "constraining",
     "strengths": {"receptive": 0.0, "causal": -0.3, "transmissive": -0.7,
                   "constraining": 0.9, "clarifying": 0.2, "influential": 0.0,
                   "balancing": 0.3, "generative": -0.4}},
    {"id": 6, "name": "Monitoring Dashboard", "domain": "software", "dominant": "clarifying",
     "strengths": {"receptive": 0.2, "causal": 0.0, "transmissive": 0.3,
                   "constraining": 0.0, "clarifying": 0.95, "influential": 0.1,
                   "balancing": 0.0, "generative": -0.2}},
    {"id": 8, "name": "Event Trigger", "domain": "software", "dominant": "causal",
     "strengths": {"receptive": 0.2, "causal": 0.95, "transmissive": 0.3,
                   "constraining": -0.2, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.0, "generative": 0.1}},
    {"id": 9, "name": "Cache Layer", "domain": "software", "dominant": "receptive",
     "strengths": {"receptive": 0.9, "causal": 0.0, "transmissive": 0.4,
                   "constraining": 0.1, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.3, "generative": -0.3}},
    {"id": 13, "name": "Data Pipeline", "domain": "software", "dominant": "generative",
     "strengths": {"receptive": 0.2, "causal": 0.2, "transmissive": 0.6,
                   "constraining": -0.1, "clarifying": 0.1, "influential": 0.0,
                   "balancing": 0.1, "generative": 0.85}},
    {"id": 16, "name": "Auth Service", "domain": "software", "dominant": "constraining",
     "strengths": {"receptive": 0.1, "causal": -0.2, "transmissive": -0.1,
                   "constraining": 0.9, "clarifying": 0.4, "influential": 0.0,
                   "balancing": 0.0, "generative": 0.1}},
    {"id": 36, "name": "Chaos Injector", "domain": "software", "dominant": "causal",
     "strengths": {"receptive": 0.0, "causal": 0.85, "transmissive": 0.0,
                   "constraining": -0.2, "clarifying": 0.3, "influential": 0.0,
                   "balancing": -0.4, "generative": 0.2}},
    # ---- business operations (8) ----
    {"id": 40, "name": "Demand Forecast", "domain": "business", "dominant": "clarifying",
     "strengths": {"receptive": 0.3, "causal": 0.1, "transmissive": 0.0,
                   "constraining": 0.0, "clarifying": 0.9, "influential": -0.1,
                   "balancing": 0.0, "generative": 0.2}},
    {"id": 42, "name": "Pricing Engine", "domain": "business", "dominant": "generative",
     "strengths": {"receptive": 0.3, "causal": 0.2, "transmissive": 0.0,
                   "constraining": 0.1, "clarifying": 0.3, "influential": 0.2,
                   "balancing": 0.4, "generative": 0.85}},
    {"id": 46, "name": "Fraud Screen", "domain": "business", "dominant": "constraining",
     "strengths": {"receptive": 0.2, "causal": -0.2, "transmissive": -0.3,
                   "constraining": 0.9, "clarifying": 0.5, "influential": 0.0,
                   "balancing": 0.0, "generative": -0.2}},
    {"id": 50, "name": "Marketing Budget Allocator", "domain": "business",
     "dominant": "balancing",
     "strengths": {"receptive": 0.1, "causal": 0.2, "transmissive": 0.3,
                   "constraining": 0.1, "clarifying": 0.0, "influential": 0.4,
                   "balancing": 0.9, "generative": -0.2}},
    {"id": 52, "name": "Churn Predictor", "domain": "business", "dominant": "clarifying",
     "strengths": {"receptive": 0.3, "causal": 0.1, "transmissive": 0.0,
                   "constraining": 0.0, "clarifying": 0.9, "influential": 0.2,
                   "balancing": 0.0, "generative": 0.2}},
    {"id": 56, "name": "Demand Planner", "domain": "business", "dominant": "balancing",
     "strengths": {"receptive": 0.3, "causal": 0.2, "transmissive": 0.0,
                   "constraining": 0.1, "clarifying": 0.3, "influential": 0.0,
                   "balancing": 0.85, "generative": 0.2}},
    {"id": 59, "name": "Loyalty Program", "domain": "business", "dominant": "influential",
     "strengths": {"receptive": 0.3, "causal": 0.1, "transmissive": -0.1,
                   "constraining": 0.0, "clarifying": 0.0, "influential": 0.9,
                   "balancing": 0.2, "generative": 0.2}},
    {"id": 69, "name": "Margin Guard", "domain": "business", "dominant": "constraining",
     "strengths": {"receptive": 0.1, "causal": -0.2, "transmissive": -0.4,
                   "constraining": 0.95, "clarifying": 0.3, "influential": 0.0,
                   "balancing": 0.2, "generative": -0.3}},
    # ---- biological systems (6) ----
    {"id": 70, "name": "Predator", "domain": "biology", "dominant": "constraining",
     "strengths": {"receptive": -0.1, "causal": 0.5, "transmissive": 0.2,
                   "constraining": 0.8, "clarifying": 0.0, "influential": 0.2,
                   "balancing": 0.3, "generative": 0.1}},
    {"id": 71, "name": "Photosynthesis", "domain": "biology", "dominant": "generative",
     "strengths": {"receptive": 0.7, "causal": 0.1, "transmissive": 0.2,
                   "constraining": -0.2, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.0, "generative": 0.95}},
    {"id": 72, "name": "Homeostasis", "domain": "biology", "dominant": "balancing",
     "strengths": {"receptive": 0.2, "causal": 0.2, "transmissive": 0.0,
                   "constraining": 0.5, "clarifying": 0.2, "influential": 0.0,
                   "balancing": 0.95, "generative": -0.1}},
    {"id": 74, "name": "Pollinator", "domain": "biology", "dominant": "transmissive",
     "strengths": {"receptive": 0.2, "causal": 0.2, "transmissive": 0.9,
                   "constraining": 0.0, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.1, "generative": 0.5}},
    {"id": 75, "name": "Seed Bank", "domain": "biology", "dominant": "receptive",
     "strengths": {"receptive": 0.95, "causal": -0.2, "transmissive": 0.1,
                   "constraining": 0.2, "clarifying": 0.0, "influential": 0.0,
                   "balancing": 0.0, "generative": 0.5}},
    {"id": 76, "name": "Mutation", "domain": "biology", "dominant": "generative",
     "strengths": {"receptive": 0.0, "causal": 0.5, "transmissive": 0.0,
                   "constraining": -0.3, "clarifying": 0.1, "influential": 0.0,
                   "balancing": -0.2, "generative": 0.85}},
    # ---- governance (6) ----
    {"id": 95, "name": "Regulator", "domain": "governance", "dominant": "constraining",
     "strengths": {"receptive": 0.1, "causal": 0.1, "transmissive": 0.0,
                   "constraining": 0.95, "clarifying": 0.2, "influential": 0.2,
                   "balancing": 0.3, "generative": -0.3}},
    {"id": 97, "name": "Auditor", "domain": "governance", "dominant": "clarifying",
     "strengths": {"receptive": 0.2, "causal": 0.0, "transmissive": 0.0,
                   "constraining": 0.4, "clarifying": 0.95, "influential": -0.1,
                   "balancing": 0.0, "generative": 0.1}},
    {"id": 99, "name": "Citizen Assembly", "domain": "governance", "dominant": "receptive",
     "strengths": {"receptive": 0.85, "causal": 0.0, "transmissive": 0.2,
                   "constraining": -0.1, "clarifying": 0.4, "influential": 0.3,
                   "balancing": 0.2, "generative": 0.0}},
    {"id": 101, "name": "Policy Think Tank", "domain": "governance",
     "dominant": "influential",
     "strengths": {"receptive": 0.3, "causal": 0.1, "transmissive": 0.2,
                   "constraining": -0.2, "clarifying": 0.3, "influential": 0.9,
                   "balancing": 0.0, "generative": 0.4}},
    {"id": 106, "name": "Public Broadcaster", "domain": "governance",
     "dominant": "transmissive",
     "strengths": {"receptive": 0.1, "causal": 0.0, "transmissive": 0.9,
                   "constraining": 0.0, "clarifying": 0.3, "influential": 0.4,
                   "balancing": 0.1, "generative": 0.2}},
    {"id": 107, "name": "Election Commission", "domain": "governance",
     "dominant": "clarifying",
     "strengths": {"receptive": 0.2, "causal": 0.1, "transmissive": 0.1,
                   "constraining": 0.5, "clarifying": 0.8, "influential": 0.0,
                   "balancing": 0.3, "generative": 0.0}},
]


def freeze_hash():
    """sha256 of the canonical serialization of REFERENCE_TAGS."""
    canonical = json.dumps(REFERENCE_TAGS, sort_keys=True,
                           separators=(",", ":"), ensure_ascii=True)
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def marker_path():
    return os.path.join(os.path.dirname(os.path.abspath(__file__)),
                        "reference_tags.sha256")


def verify_freeze():
    """Raise if the frozen marker no longer matches REFERENCE_TAGS."""
    with open(marker_path(), encoding="utf-8") as f:
        expected = f.read().strip().split()[0]
    actual = freeze_hash()
    if actual != expected:
        raise RuntimeError(
            f"reference_tags freeze mismatch: marker={expected[:12]}... "
            f"actual={actual[:12]}... — reference tags drifted after authoring")
    return True


def validate_reference_tags():
    """Structural integrity: domains, parsable strengths, dominant consistency."""
    from tags import parse_tags
    counts = {}
    for item in REFERENCE_TAGS:
        counts[item["domain"]] = counts.get(item["domain"], 0) + 1
        parse_tags(json.dumps(item["strengths"]))
        if dominant_role(item["strengths"]) != item["dominant"]:
            raise ValueError(
                f"dominant mismatch for id {item['id']}: "
                f"{item['dominant']} != {dominant_role(item['strengths'])}")
    if set(item["dominant"] for item in REFERENCE_TAGS) != set(ROLES):
        raise ValueError("reference dominants do not cover all 8 roles")
    return counts
