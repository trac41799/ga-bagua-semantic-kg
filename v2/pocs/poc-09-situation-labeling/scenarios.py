"""POC-09: situation-labeling as generated interpretation — 20 frozen scenarios."""

import hashlib
import json

# (id, domain, situation, factual_answer)
SCENARIOS = [
    (1, "product", "The checkout page is failing for 12% of mobile users on a major release day.", "Roll back to the previous checkout build and notify affected users."),
    (2, "product", "A new pricing tier was announced but existing customers keep emailing support about grandfathering.", "Publish a clear grandfathering policy and auto-apply it to current subscribers."),
    (3, "product", "The recommendation engine started recommending competitors' products after a data migration.", "Validate the entity mapping after migration and quarantine the affected user segments."),
    (4, "product", "Feature adoption for the new dashboard is 8% after three weeks, well below the 30% target.", "Run onboarding prompts and measure the drop-off step before iterating on the dashboard."),
    (5, "product", "The mobile app crashes on cold start for devices running the latest OS update.", "Reproduce on the affected OS, ship a hotfix build, and add a device-family regression test."),
    (6, "business", "Two key suppliers both raised prices 15% within the same quarter.", "Renegotiate annual contracts and model the impact of switching to a third supplier."),
    (7, "business", "Sales pipeline conversion dropped 6 points after the CRM migration.", "Audit the migration mapping for lost stage data before assuming a process problem."),
    (8, "business", "A viral competitor launch is pulling our most active segment.", "Identify the segment's switching triggers with interviews, then run a targeted retention offer."),
    (9, "business", "Return rates spiked 3x after a packaging redesign.", "Compare return reasons pre/post redesign and test the previous packaging on a subset."),
    (10, "business", "The subscription team missed its renewal target for the third quarter in a row.", "Diagnose the renewal funnel stage losses, then redesign the renewal touchpoint sequence."),
    (11, "biology", "A lake's oxygen level dropped sharply after an unusually hot week.", "Monitor algae bloom indicators and reduce nutrient inflow from upstream farms."),
    (12, "biology", "A pollinator species declined 40% in two seasons while a neighboring crop expanded.", "Assess pesticide timing against bloom windows and create hedgerow corridors."),
    (13, "biology", "Hospital-acquired infection rates rose after a new cleaning protocol was introduced.", "Compare protocols at affected wards and audit dilution ratios before reverting."),
    (14, "biology", "A reef's fish census shows a collapse in herbivore species.", "Check for overfishing pressure and introduce a temporary herbivore-protection zone."),
    (15, "biology", "Migratory bird arrival shifted two weeks earlier than the historical band.", "Update habitat management calendars and monitor mismatch with food availability."),
    (16, "governance", "Public transport ridership fell 18% after a fare increase, more than forecast.", "Review the fare elasticity model and pilot a discounted off-peak pass."),
    (17, "governance", "A data-privacy complaint backlog doubled after new regulations took effect.", "Triage the backlog by risk class and automate routine acknowledgment responses."),
    (18, "governance", "An emergency response drill exposed a 40-minute delay in inter-agency handoff.", "Redesign the handoff protocol and run a second drill within 60 days."),
    (19, "governance", "A procurement audit found 22% of contracts were awarded without competitive bidding.", "Publish a waiver-approval workflow and re-audit the current quarter."),
    (20, "governance", "Voter registration errors affected 3% of new applicants last cycle.", "Introduce pre-submission validation and reconcile against the registry before the next cycle."),
]

ARM_B_SYSTEM = (
    "Frame your explanation using the I-Ching hexagram structure as a WRITING SCAFFOLD ONLY:\n"
    "1. Name an upper/lower trigram pair that captures the situation's dynamics (e.g., 坎 over 離).\n"
    "2. Describe the line-change pattern (which of the three lines is 'moving').\n"
    "3. Then give the factual explanation in plain prose.\n"
    "The structure is a scaffold for organizing your writing. It has NO predictive meaning; "
    "do not claim it predicts outcomes. Stay factual."
)


def arm_a_prompt(situation, answer):
    return [{"role": "system", "content": "Explain this situation factually and give the recommended action."},
            {"role": "user", "content": f"Situation: {situation}\nFactual answer to convey: {answer}"}]


def arm_b_prompt(situation, answer):
    return [{"role": "system", "content": ARM_B_SYSTEM},
            {"role": "user", "content": f"Situation: {situation}\nFactual answer to convey: {answer}"}]


def rater_prompt(situation, explanation):
    return [{"role": "system", "content": "Rate COMPREHENSION and TRUST of this explanation of the "
                                          "situation on a 1-5 scale (5 = fully clear and trustworthy). "
                                          "Output ONLY the integer."},
            {"role": "user", "content": f"Situation: {situation}\nExplanation:\n{explanation}"}]


def freeze_hash():
    return hashlib.sha256(json.dumps(SCENARIOS, ensure_ascii=False).encode()).hexdigest()
