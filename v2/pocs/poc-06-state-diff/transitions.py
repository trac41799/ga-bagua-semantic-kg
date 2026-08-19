"""POC-06: state-diff vocabulary — 20 frozen transitions with planted 3-aspect deltas."""

import hashlib
import json

# (id, domain, before, after, planted_deltas)
# planted_deltas: list of 3 (aspect_label, before_value, after_value) — ground truth for the rater
TRANSITIONS = [
    (1, "system", "cache hit ratio 94%, p99 latency 120ms, error rate 0.2%",
     "cache hit ratio 99%, p99 latency 95ms, error rate 0.1%",
     [("cache hit ratio", "94%", "99%"), ("p99 latency", "120ms", "95ms"), ("error rate", "0.2%", "0.1%")]),
    (2, "system", "service replicas 3, CPU utilization 80%, queue depth 400",
     "service replicas 6, CPU utilization 45%, queue depth 40",
     [("replicas", "3", "6"), ("CPU utilization", "80%", "45%"), ("queue depth", "400", "40")]),
    (3, "system", "database connections 500, read throughput 2k/s, write latency 8ms",
     "database connections 250, read throughput 5k/s, write latency 5ms",
     [("connections", "500", "250"), ("read throughput", "2k/s", "5k/s"), ("write latency", "8ms", "5ms")]),
    (4, "system", "feature flag rollout 10% of users, error budget 60% left, alerts 12/day",
     "feature flag rollout 100% of users, error budget 45% left, alerts 3/day",
     [("rollout", "10%", "100%"), ("error budget", "60%", "45%"), ("alerts", "12/day", "3/day")]),
    (5, "system", "auth tokens issued 1k/hour, revocation list 5k, login failures 2%",
     "auth tokens issued 1.4k/hour, revocation list 12k, login failures 0.4%",
     [("tokens/hour", "1k", "1.4k"), ("revocation list", "5k", "12k"), ("login failures", "2%", "0.4%")]),
    (6, "business", "inventory turns 4/year, stockout rate 7%, order fill time 3 days",
     "inventory turns 5.5/year, stockout rate 2%, order fill time 1.5 days",
     [("inventory turns", "4", "5.5"), ("stockout rate", "7%", "2%"), ("fill time", "3 days", "1.5 days")]),
    (7, "business", "customer churn 3.2%/month, NPS 28, support tickets 900/week",
     "customer churn 2.1%/month, NPS 41, support tickets 600/week",
     [("churn", "3.2%", "2.1%"), ("NPS", "28", "41"), ("tickets/week", "900", "600")]),
    (8, "business", "pipeline conversion 18%, deal size $22k, sales cycle 45 days",
     "pipeline conversion 24%, deal size $19k, sales cycle 38 days",
     [("conversion", "18%", "24%"), ("deal size", "$22k", "$19k"), ("cycle", "45 days", "38 days")]),
    (9, "business", "return rate 6%, repeat purchase 34%, cart abandonment 68%",
     "return rate 4%, repeat purchase 47%, cart abandonment 54%",
     [("return rate", "6%", "4%"), ("repeat purchase", "34%", "47%"), ("abandonment", "68%", "54%")]),
    (10, "business", "marketing spend $80k/month, CAC $45, LTV $210",
     "marketing spend $95k/month, CAC $38, LTV $245",
     [("spend", "$80k", "$95k"), ("CAC", "$45", "$38"), ("LTV", "$210", "$245")]),
    (11, "biology", "herbivore population 400, plant cover 55%, predator count 12",
     "herbivore population 250, plant cover 70%, predator count 9",
     [("herbivore population", "400", "250"), ("plant cover", "55%", "70%"), ("predator count", "12", "9")]),
    (12, "biology", "blood glucose 6.8 mmol/L, insulin 8 uU/mL, heart rate 62 bpm",
     "blood glucose 5.1 mmol/L, insulin 12 uU/mL, heart rate 58 bpm",
     [("glucose", "6.8", "5.1"), ("insulin", "8", "12"), ("heart rate", "62", "58")]),
    (13, "biology", "stream temperature 18C, dissolved oxygen 6 mg/L, fish count 120",
     "stream temperature 22C, dissolved oxygen 4.5 mg/L, fish count 80",
     [("temperature", "18C", "22C"), ("oxygen", "6", "4.5"), ("fish count", "120", "80")]),
    (14, "biology", "pollinator visits 30/hour, seed set 40%, flower density 5/m2",
     "pollinator visits 55/hour, seed set 62%, flower density 9/m2",
     [("visits", "30", "55"), ("seed set", "40%", "62%"), ("flower density", "5", "9")]),
    (15, "biology", "immune cell count 5k/uL, inflammation marker 12 pg/mL, recovery days 7",
     "immune cell count 8k/uL, inflammation marker 4 pg/mL, recovery days 4",
     [("cell count", "5k", "8k"), ("inflammation", "12", "4"), ("recovery days", "7", "4")]),
    (16, "governance", "permits issued 120/month, approval time 14 days, appeals 8/month",
     "permits issued 90/month, approval time 9 days, appeals 3/month",
     [("permits", "120", "90"), ("approval time", "14 days", "9 days"), ("appeals", "8", "3")]),
    (17, "governance", "budget deficit 4.2%, audit findings 15, public complaints 220/month",
     "budget deficit 2.8%, audit findings 6, public complaints 130/month",
     [("deficit", "4.2%", "2.8%"), ("audit findings", "15", "6"), ("complaints", "220", "130")]),
    (18, "governance", "response time to incidents 45 min, coverage area 60%, equipment readiness 70%",
     "response time to incidents 25 min, coverage area 85%, equipment readiness 92%",
     [("response time", "45 min", "25 min"), ("coverage", "60%", "85%"), ("readiness", "70%", "92%")]),
    (19, "governance", "data requests processed 300/month, backlog 2k, accuracy 96%",
     "data requests processed 450/month, backlog 800, accuracy 99%",
     [("requests", "300", "450"), ("backlog", "2k", "800"), ("accuracy", "96%", "99%")]),
    (20, "governance", "inspection rate 12%, violations found 30/month, fines $40k/month",
     "inspection rate 25%, violations found 18/month, fines $25k/month",
     [("inspection rate", "12%", "25%"), ("violations", "30", "18"), ("fines", "$40k", "$25k")]),
]


def deltas_present(summary_text, planted):
    """Automated check: fraction of planted (aspect, before, after) triples whose values appear in text."""
    text = summary_text.lower()
    found = 0
    for aspect, b, a in planted:
        if str(b).lower() in text and str(a).lower() in text:
            found += 1
    return found / len(planted) if planted else 0.0


def freeze_hash():
    return hashlib.sha256(json.dumps(TRANSITIONS, ensure_ascii=False).encode()).hexdigest()
