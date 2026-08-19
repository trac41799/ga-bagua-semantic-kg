"""Shared benchmark harness — frozen POC data loaders, metrics, ledger (validation spec)."""

import csv
import hashlib
import importlib.util
import json
import os
import re
import sys
from datetime import date

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.dirname(HERE)
POCS = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(HERE))), "pocs")


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    parent = os.path.dirname(path)
    if parent not in sys.path:
        sys.path.insert(0, parent)
    try:
        spec.loader.exec_module(mod)
    finally:
        sys.path.pop(0)
    return mod


# ---- frozen POC data (read-only) ----

def poc05_tasks():
    mod = load_module("p05_protocol", os.path.join(POCS, "poc-05-coverage-rubric", "protocol.py"))
    return list(mod.TASKS)


def poc05_draft_plans():
    """Cached POC-05 Arm-A plans: {tid: plan_text} — the same drafts the POC audited.
    Keys are exactly '<tid>_A' (audit keys '<tid>_audit_A' must not collide)."""
    cache = os.path.join(POCS, "poc-05-coverage-rubric", "data", "cache", "responses.json")
    data = json.load(open(cache, encoding="utf-8"))
    return {int(k.split("_")[0]): v for k, v in data.items()
            if re.fullmatch(r"\d+_A", k)}


def poc08_statements():
    mod = load_module("p08_statements", os.path.join(POCS, "poc-08-reframing-v2", "statements.py"))
    return [s["text"] if isinstance(s, dict) else s for s in mod.STATEMENTS]


def poc06_transitions():
    mod = load_module("p06_transitions", os.path.join(POCS, "poc-06-state-diff", "transitions.py"))
    return list(mod.TRANSITIONS)


def hash_of(obj):
    return hashlib.sha256(json.dumps(obj, ensure_ascii=False, sort_keys=True).encode()).hexdigest()


# ---- metrics ----

STATE_DIFF_MEASUREMENT_METHOD = "same_record_value_pair_v1"
STATE_DIFF_MEASUREMENT_PROTOCOL = (
    "state_diff.measurement.same_record_value_pair_v1"
)

def audit_rater(llm, task, plan):
    """POC-05 6-aspect audit; returns list of 6 bits (1 = aspect present)."""
    aspects = "\n".join([
        "0. constraint handling (budgets, limits, risks)",
        "1. resource flow (what moves, who delivers)",
        "2. stakeholder reception (who must accept)",
        "3. risk balance (feedback loops, failure modes)",
        "4. clarity of steps (measurable actions)",
        "5. initiation triggers (how it starts)",
    ])
    sys_prompt = ("For each of the 6 aspects below, output a JSON object "
                  "{'0': 0|1, ...} where 1 = the plan addresses it, 0 = missing. "
                  "Output ONLY the JSON object.")
    ans, _ = llm.chat([{"role": "system", "content": sys_prompt},
                       {"role": "user", "content": f"Task: {task}\nPlan:\n{plan}\nAspects:\n{aspects}"}],
                      max_tokens=128)
    t = ans.strip().strip("`").strip()
    if t.startswith("json"):
        t = t[4:].strip()
    data = json.loads(t)
    return [int(data[str(i)]) for i in range(6)]


def diversity(vectors):
    vs = [np.array(v) for v in vectors if v is not None]
    if len(vs) < 2:
        return 0.0
    d, n = 0.0, 0
    for i in range(len(vs)):
        for j in range(i + 1, len(vs)):
            d += 1.0 - float(vs[i] @ vs[j])
            n += 1
    return d / n if n else 0.0


def rubric_encode(llm, text):
    """8-dim rubric vector via the SKILL.md encoder (POC-08 diversity metric)."""
    mod = load_module("d_rubric", os.path.join(POCS, "path-d", "rubric.py"))
    ans, _ = llm.chat(mod.encode_prompt(text), max_tokens=128)
    try:
        return mod.parse_encoding(ans)
    except Exception:
        return None


def measure_planted_delta_coverage(aspects, planted):
    """Measure planted before/after value pairs retained in one aspect row.

    Aspect labels are intentionally ignored: this is a value-pair retention
    metric, not a semantic label-accuracy metric.
    """
    rows = aspects if isinstance(aspects, list) else []
    planted = list(planted or [])

    def value_in_field(value, field):
        if value is None or field is None:
            return False
        value_text = str(value)
        field_text = str(field)
        return bool(value_text) and value_text.casefold() in field_text.casefold()

    matched = []
    for planted_row in planted:
        if isinstance(planted_row, dict):
            before_value = planted_row.get("before")
            after_value = planted_row.get("after")
        else:
            try:
                _, before_value, after_value = planted_row
            except (TypeError, ValueError):
                matched.append(False)
                continue
        matched.append(any(
            isinstance(row, dict)
            and value_in_field(before_value, row.get("before"))
            and value_in_field(after_value, row.get("after"))
            for row in rows
        ))

    covered = sum(matched)
    total = len(matched)
    return {
        "method": STATE_DIFF_MEASUREMENT_METHOD,
        "matched": matched,
        "covered": covered,
        "total": total,
        "coverage": covered / total if total else 0.0,
    }


def statediff_coverage(aspects, planted):
    """Compatibility scalar for callers that only need repaired coverage."""
    return measure_planted_delta_coverage(aspects, planted)["coverage"]


def parse_aspect_lines(text):
    """Strict 3-line parse: 'label: v1 -> v2' x3. Returns list of dicts or raises."""
    lines = [l.strip() for l in text.strip().splitlines() if l.strip()]
    if len(lines) != 3:
        raise ValueError(f"expected exactly 3 aspect lines, got {len(lines)}")
    out = []
    for l in lines:
        m = re.match(r"^(.+?):\s*(.+?)\s*->\s*(.+)$", l)
        if not m:
            raise ValueError(f"unparseable aspect line: {l!r}")
        out.append({"aspect": m.group(1).strip(), "before": m.group(2).strip(),
                    "after": m.group(3).strip()})
    return out


# ---- ledger + verdict ----

def ledger_row(path, *args):
    """Append a ledger row, replacing any existing row for the same metric (no stale/dupe rows).
    Accepts ledger_row(path, metric, value, bar, extra) or ledger_row(path, [m, v, b, e])."""
    if len(args) == 1 and isinstance(args[0], (list, tuple)):
        row = list(args[0])
    else:
        row = list(args)
    metric = row[0]
    while len(row) < 4:
        row.append("")
    rows = []
    if os.path.exists(path):
        with open(path, newline="", encoding="utf-8") as f:
            rows = list(csv.reader(f))
    if not rows or rows[0][:1] != ["metric"]:
        rows = [["metric", "value", "bar", "date", "note"]]
    rows = [r for r in rows[1:] if r and r[0] != metric]  # drop stale rows for this metric
    rows.append([row[0], f"{row[1]}", row[2], str(date.today()), row[3]])
    with open(path, "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["metric", "value", "bar", "date", "note"])
        w.writerows(rows)


def render_verdict(path, rows):
    """rows: list of (label, value, bar, verdict). Writes markdown verdict table."""
    with open(path, "w", encoding="utf-8") as f:
        f.write("# iching-tools validation benchmark\n\n")
        f.write("| metric | value | bar | verdict |\n|---|---|---|---|\n")
        for label, value, bar, verdict in rows:
            f.write(f"| {label} | {value} | {bar} | {verdict} |\n")
