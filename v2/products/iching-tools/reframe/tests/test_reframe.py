"""TDD cases R1-R6 for iching_reframe (SimulatedLLM only; no network)."""

import json
import os
import subprocess
import sys
from pathlib import Path

from iching_reframe import reframe
from iching_reframe.llm_client import SimulatedLLM
from iching_reframe.moves import TRIGRAMS, complement
from iching_reframe.naming import MOVE_DESC, MOVE_EXEMPLARS, move_type, naming_prompt

ROOT = Path(__file__).resolve().parent.parent

EXPECTED_MOVES = [
    "origin", "flip0", "flip1", "flip2",
    "double_flip01", "double_flip02", "double_flip12", "complement",
]
ALL_TRIGRAMS = {"Kun", "Zhen", "Kan", "Gen", "Dui", "Xun", "Li", "Qian"}


def run_cli(*args):
    env = {k: v for k, v in os.environ.items()
           if k not in ("DEEPSEEK_API_KEY", "OPENROUTER_API_KEY")}
    env["PYTHONPATH"] = str(ROOT)
    return subprocess.run(
        [sys.executable, "-m", "iching_reframe", *args],
        capture_output=True, text=True, env=env, cwd=ROOT)


def trigram_of(state_label):
    return state_label.split(" (")[0]


def test_r1_cli_sim_8_positions():
    proc = run_cli("--statement", "We should raise prices.", "--sim")
    assert proc.returncode == 0, proc.stderr
    lines = [ln for ln in proc.stdout.splitlines() if ln.strip()]
    assert len(lines) == 8

    json_proc = run_cli("--statement", "We should raise prices.", "--json", "--sim")
    assert json_proc.returncode == 0, json_proc.stderr
    result = json.loads(json_proc.stdout)
    assert [p["move"] for p in result["positions"]] == EXPECTED_MOVES


def test_r2_cli_json_schema():
    proc = run_cli("--statement", "s", "--json", "--sim")
    assert proc.returncode == 0, proc.stderr
    result = json.loads(proc.stdout)
    assert set(result) == {"statement", "positions"}
    assert result["statement"] == "s"
    assert len(result["positions"]) == 8
    for p in result["positions"]:
        assert set(p) >= {"move", "state", "reframe", "description"}
        assert isinstance(p["move"], str) and p["move"]
        assert isinstance(p["state"], str) and p["state"]
        assert isinstance(p["reframe"], str) and p["reframe"]
        assert isinstance(p["description"], str) and p["description"]


def test_r3_distinct_states():
    result = reframe("We should raise prices.", SimulatedLLM())
    assert len(result["positions"]) == 8
    states = [p["state"] for p in result["positions"]]
    assert len(set(states)) == 8
    assert set(trigram_of(s) for s in states) == ALL_TRIGRAMS
    assert [p["move"] for p in result["positions"]] == EXPECTED_MOVES
    for p in result["positions"]:
        assert p["move"] in p["reframe"]  # SimulatedLLM embeds the move name


def test_r4_complement_identity():
    # blade-level Hodge-dual identities: Kan->Li, Gen->Dui, Zhen->Xun, Kun->Qian
    expect = {2: 5, 4: 3, 1: 6, 0: 7}
    for blade, dual in expect.items():
        assert complement((blade, +1))[0] == dual
    assert complement((2, +1)) == (5, -1)  # dual of e2 is -e13 -> Li, sign -1
    assert complement((0, +1)) == (7, +1)  # dual of 1 is +e123 -> Qian, sign +1

    # through the tool: complement position state is the dual trigram
    result = reframe("We should raise prices.", SimulatedLLM())
    by_move = {p["move"]: p for p in result["positions"]}
    origin_trigram = trigram_of(by_move["origin"]["state"])
    origin_blade = next(b for b, t in TRIGRAMS.items() if t == origin_trigram)
    assert trigram_of(by_move["complement"]["state"]) == TRIGRAMS[origin_blade ^ 0b111]


def test_r5_few_shot_prompt():
    for kind in ("flip", "double_flip", "complement", "origin"):
        assert len(MOVE_EXEMPLARS[kind]) >= 2, kind
        assert kind in MOVE_DESC
    for move in EXPECTED_MOVES:
        kind = move_type(move)
        assert kind in MOVE_DESC
        messages = naming_prompt("We should raise prices.", move,
                                 "Zhen (blade e1, grade 1, sign +1)")
        assert len(messages) == 2
        user = messages[-1]["content"]
        assert user.count("-> reframe:") >= 2, move
        assert f"Move type: {MOVE_DESC[kind]}" in user, move
    assert move_type("flip0") == "flip"
    assert move_type("double_flip02") == "double_flip"
    assert move_type("complement") == "complement"
    assert move_type("origin") == "origin"


def test_r6_missing_config_exit_2():
    proc = run_cli("--statement", "s")
    assert proc.returncode == 2
    assert "api key" in proc.stderr.lower()
