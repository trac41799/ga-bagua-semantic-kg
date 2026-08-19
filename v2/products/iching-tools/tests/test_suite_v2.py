"""Suite tests: itools unified CLI, flagship reframe descriptions, docs (TDD suite-v2)."""

import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


def run_itools(args, env_extra=None):
    env = dict(os.environ)
    env["PYTHONPATH"] = os.pathsep.join([
        os.path.join(ROOT, "iching_tools"),
        os.path.join(ROOT, "coverage"),
        os.path.join(ROOT, "reframe"),
        os.path.join(ROOT, "statediff"),
    ] + ([env_extra] if env_extra else []))
    p = subprocess.run([sys.executable, "-m", "iching_tools.cli"] + args,
                       capture_output=True, text=True, timeout=120, env=env)
    return p


def test_version():
    p = run_itools(["--version"])
    assert p.returncode == 0
    assert "0.2.0" in p.stdout


def test_help_lists_commands():
    p = run_itools(["--help"])
    assert p.returncode == 0
    for cmd in ["coverage", "reframe", "statediff"]:
        assert cmd in p.stdout


def test_coverage_sim():
    p = run_itools(["coverage", "--task", "launch", "--plan", "Build it.", "--json", "--sim"])
    assert p.returncode == 0, p.stderr
    data = json.loads(p.stdout.strip())
    assert set(data) == {"task", "original_plan", "audited_plan", "checklist"}


def test_reframe_sim():
    p = run_itools(["reframe", "--statement", "We should raise prices.", "--json", "--sim"])
    assert p.returncode == 0, p.stderr
    data = json.loads(p.stdout.strip())
    assert len(data["positions"]) == 8


def test_statediff_sim():
    p = run_itools(["statediff", "--before", "a 1", "--after", "a 2", "--json", "--sim"])
    assert p.returncode == 0, p.stderr
    data = json.loads(p.stdout.strip())
    assert len(data["aspects"]) == 3


def test_delegation_parity():
    """itools output == package CLI output for identical args (sim mode)."""
    p1 = run_itools(["coverage", "--task", "launch", "--plan", "Build it.", "--json", "--sim"])
    env = dict(os.environ)
    env["PYTHONPATH"] = os.path.join(ROOT, "coverage")
    p2 = subprocess.run([sys.executable, "-m", "iching_coverage",
                         "--task", "launch", "--plan", "Build it.", "--json", "--sim"],
                        capture_output=True, text=True, timeout=120, env=env)
    assert p1.stdout.strip() == p2.stdout.strip()


def test_missing_args_exit_2():
    p = run_itools(["coverage", "--json", "--sim"])
    assert p.returncode == 2


def test_no_key_exit_2():
    env_clean = {k: v for k, v in os.environ.items()
                 if k not in ("DEEPSEEK_API_KEY", "OPENROUTER_API_KEY")}
    env = dict(env_clean)
    env["PYTHONPATH"] = os.pathsep.join([os.path.join(ROOT, "iching_tools"),
                                         os.path.join(ROOT, "coverage"),
                                         os.path.join(ROOT, "reframe"),
                                         os.path.join(ROOT, "statediff")])
    p = subprocess.run([sys.executable, "-m", "iching_tools.cli",
                        "reframe", "--statement", "s"],
                       capture_output=True, text=True, timeout=120, env=env)
    assert p.returncode == 2


def test_unknown_command_exit_2():
    p = run_itools(["bogus"])
    assert p.returncode == 2


def test_reframe_descriptions():
    import sys as _s
    _s.path.insert(0, os.path.join(ROOT, "reframe"))
    from iching_reframe import reframe
    from iching_reframe.llm_client import SimulatedLLM
    result = reframe("We should raise prices.", SimulatedLLM())
    positions = result["positions"]
    assert len(positions) == 8
    for p_ in positions:
        assert p_["description"], p_
        assert p_["description"] != ""
    by_move = {p_["move"]: p_ for p_ in positions}
    assert "flip the bottom line" in by_move["flip0"]["description"]
    assert "Hodge dual" in by_move["complement"]["description"]
    assert "original position" in by_move["origin"]["description"]
    # legacy fields intact
    assert all(set(p_) >= {"move", "state", "reframe"} for p_ in positions)


def test_docs_exist():
    for f in ["AGENT_INTEGRATION.md"]:
        assert os.path.exists(os.path.join(ROOT, f)), f
    readme = open(os.path.join(ROOT, "README.md"), encoding="utf-8").read()
    assert "itools" in readme and "positioning" in readme.lower()
