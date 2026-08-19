"""MCP-SDK compatibility: our stdio server consumed by the OFFICIAL mcp client.

Factual verification via subprocess probes (the probe runs the real SDK's
stdio_client + ClientSession against our server and prints the results).
"""

import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROBE = os.path.abspath(os.path.join(HERE, "sdk_probe.py"))


def probe(scenario="source"):
    p = subprocess.run([sys.executable, PROBE, scenario], capture_output=True,
                       text=True, timeout=180, cwd=ROOT)
    assert p.returncode == 0, (f"probe {scenario} failed: rc={p.returncode} "
                               f"out={p.stdout[-200:]!r} err={p.stderr[-600:]!r}")
    return p.stdout


def test_sdk_simulator_initializes_lists_and_calls_all_six_tools():
    out = probe("source")
    names = json.loads(out.split("NAMES:", 1)[1].splitlines()[0])
    assert names == ["coverage_audit", "reframe", "state_diff",
                     "cl3_evaluate", "interaction_spectrum", "rotor_transition"]
    calls = json.loads(out.split("CALLS:", 1)[1].splitlines()[0])
    assert calls == names
