"""B4 real-mode CLI smoke: run each CLI once, check exit code + JSON schema."""

import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
ENV = dict(os.environ)
ENV["PYTHONPATH"] = os.pathsep.join([os.path.join(ROOT, "coverage"),
                                     os.path.join(ROOT, "reframe"),
                                     os.path.join(ROOT, "statediff")])
if not (ENV.get("DEEPSEEK_API_KEY") or ENV.get("OPENROUTER_API_KEY")):
    env_path = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))), ".env.local")
    if os.path.exists(env_path):
        for line in open(env_path, encoding="utf-8"):
            line = line.strip()
            if line.startswith("DEEPSEEK_API_KEY="):
                ENV["DEEPSEEK_API_KEY"] = line.split("=", 1)[1].strip().strip('"').strip("'")

CASES = [
    ("coverage", ["-m", "iching_coverage", "--task", "launch an API product",
                  "--plan", "Build it.", "--json"], ["task", "original_plan", "audited_plan", "checklist"]),
    ("reframe", ["-m", "iching_reframe", "--statement", "We should raise prices.", "--json"],
     ["statement", "positions"]),
    ("statediff", ["-m", "iching_statediff", "--before", "cache 94%, latency 120ms",
                   "--after", "cache 99%, latency 95ms", "--json"], ["before", "after", "aspects"]),
]


def main():
    results = []
    for name, args, keys in CASES:
        p = subprocess.run([sys.executable] + args, capture_output=True, text=True,
                           timeout=180, env=ENV, cwd=ROOT)
        ok_rc = p.returncode == 0
        schema_ok = False
        if ok_rc:
            try:
                data = json.loads(p.stdout.strip())
                schema_ok = all(k in data for k in keys)
            except json.JSONDecodeError:
                schema_ok = False
        results.append((name, ok_rc, schema_ok, p.stderr.strip()[:150]))

    # MCP real-mode: handshake + one tools/call per tool
    mcp_results = []
    server = os.path.join(ROOT, "mcp", "server.py")
    proc = subprocess.Popen([sys.executable, server], stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=ENV)
    lines = [
        json.dumps({"jsonrpc": "2.0", "id": 1, "method": "tools/call",
                    "params": {"name": "coverage_audit",
                               "arguments": {"task": "launch an API product", "plan": "Build it."}}}),
        json.dumps({"jsonrpc": "2.0", "id": 2, "method": "tools/call",
                    "params": {"name": "reframe",
                               "arguments": {"statement": "We should raise prices."}}}),
        json.dumps({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
                    "params": {"name": "state_diff",
                               "arguments": {"before": "cache 94%, latency 120ms",
                                             "after": "cache 99%, latency 95ms"}}}),
    ]
    out, err = proc.communicate("\n".join(lines) + "\n", timeout=300)
    for line in out.strip().splitlines():
        d = json.loads(line)
        mcp_results.append(("error" not in d, d.get("id")))
    mcp_ok = len(mcp_results) == 3 and all(ok for ok, _ in mcp_results) and "Traceback" not in err

    ok = all(rc and sc for _, rc, sc, _ in results) and mcp_ok
    out_dir = os.path.join(ROOT, "output")
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, "benchmark_smoke.md"), "w", encoding="utf-8") as f:
        f.write("# B4 — real-mode integration smoke (CLI + MCP)\n\n")
        f.write("| component | exit 0 | schema valid |\n|---|---|---|\n")
        for name, rc, sc, err in results:
            f.write(f"| {name} CLI | {rc} | {sc} |\n")
        f.write(f"| MCP tools (3 real calls) | {mcp_ok} | {mcp_ok} |\n")
        f.write(f"\n**VERDICT: {'PASS (zero defects)' if ok else 'FAIL'}**\n")
        for name, rc, sc, err in results:
            if not (rc and sc):
                f.write(f"\n- {name} defect: rc={rc} schema={sc} stderr={err}\n")
        if not mcp_ok:
            f.write(f"\n- MCP defect: {err[-200:]}\n")
    for name, rc, sc, err in results:
        print(f"{name:10s} rc={rc} schema={sc} {'OK' if rc and sc else err}")
    print(f"MCP 3 calls: {'OK' if mcp_ok else 'FAIL'}")
    print("VERDICT:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
