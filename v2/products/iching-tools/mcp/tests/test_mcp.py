"""MCP server tests: drive the server subprocess over stdio (M1–M8)."""

import json
import os
import subprocess
import sys

import pytest

HERE = os.path.dirname(os.path.abspath(__file__))
SERVER = os.path.join(os.path.dirname(HERE), "server.py")


def call(lines, timeout=60, sim=True, env=None):
    """Send newline-delimited JSON to the server; return parsed responses."""
    command = [sys.executable, SERVER]
    if sim:
        command.append("--sim")
    proc = subprocess.Popen(command, stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            text=True, env=env)
    out, err = proc.communicate("\n".join(lines) + "\n", timeout=timeout)
    responses = []
    for line in out.strip().splitlines():
        if line.strip():
            responses.append(json.loads(line))
    return responses, err


def test_initialize_handshake():
    responses, err = call([json.dumps({"jsonrpc": "2.0", "id": 1, "method": "initialize",
                                       "params": {"protocolVersion": "2024-11-05",
                                                  "capabilities": {}, "clientInfo": {"name": "t", "version": "1"}}})])
    assert responses, f"empty responses; stderr={err!r}"
    assert responses[0]["result"]["protocolVersion"] == "2024-11-05"
    assert "tools" in responses[0]["result"]["capabilities"]
    assert responses[0]["result"]["serverInfo"]["name"] == "iching-tools"


def test_tools_list_six_tools_with_strict_schemas():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})])
    tools = responses[0]["result"]["tools"]
    assert set(t["name"] for t in tools) == {"coverage_audit", "reframe", "state_diff",
                                             "cl3_evaluate", "interaction_spectrum", "rotor_transition"}
    for t in tools:
        assert "inputSchema" in t and "properties" in t["inputSchema"]
        assert t["inputSchema"]["additionalProperties"] is False


def test_tools_call_coverage():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
                                     "params": {"name": "coverage_audit",
                                                "arguments": {"task": "launch", "plan": "Build it."}}})])
    text = responses[0]["result"]["content"][0]["text"]
    data = json.loads(text)
    assert data["checklist"] is True
    assert data["original_plan"] == "Build it."


def test_tools_call_reframe():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 4, "method": "tools/call",
                                     "params": {"name": "reframe",
                                                "arguments": {"statement": "We should raise prices."}}})])
    data = json.loads(responses[0]["result"]["content"][0]["text"])
    assert len(data["positions"]) == 8
    moves = {p["move"] for p in data["positions"]}
    assert "origin" in moves and "complement" in moves
    assert any(m.startswith("flip") for m in moves)
    assert any(m.startswith("double_flip") for m in moves)


def test_tools_call_statediff():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 5, "method": "tools/call",
                                     "params": {"name": "state_diff",
                                                "arguments": {"before": "a 1, b 2", "after": "a 3, b 4"}}})])
    data = json.loads(responses[0]["result"]["content"][0]["text"])
    assert len(data["aspects"]) == 3


def test_tools_call_cl3():
    responses, _ = call([json.dumps({
        "jsonrpc": "2.0", "id": 10, "method": "tools/call",
        "params": {"name": "cl3_evaluate", "arguments": {
            "ops": [{"op": "product", "a": "e1", "b": "e2"},
                    {"op": "count_blades", "grade_k": 2}],
        }},
    })])
    assert responses[0]["result"]["content"][0]["type"] == "text"
    data = json.loads(responses[0]["result"]["content"][0]["text"])
    assert data["result"] == 3
    assert len(data["steps"]) == 2


def test_tools_call_interaction_spectrum():
    responses, _ = call([json.dumps({
        "jsonrpc": "2.0", "id": 11, "method": "tools/call",
        "params": {"name": "interaction_spectrum", "arguments": {
            "points": [[1, 1], [1, -1], [-1, 1], [-1, -1]],
            "values": [1.0, -1.0, -1.0, 1.0],
        }},
    })])
    data = json.loads(responses[0]["result"]["content"][0]["text"])
    assert data["spectrum"]["3"] == 1.0
    assert data["identified"] == [3]


def test_tools_call_rotor():
    responses, _ = call([json.dumps({
        "jsonrpc": "2.0", "id": 12, "method": "tools/call",
        "params": {"name": "rotor_transition", "arguments": {
            "ops": [{"op": "apply", "r": [1.0, 0.0, 0.0, 0.0], "blade": "e1"},
                    {"op": "distance", "r1": [1.0, 0.0, 0.0, 0.0],
                     "r2": [1.0, 0.0, 0.0, 0.0]}],
        }},
    })])
    data = json.loads(responses[0]["result"]["content"][0]["text"])
    assert data == ["e1", 0.0]


def test_unknown_tool_error():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 6, "method": "tools/call",
                                     "params": {"name": "nope", "arguments": {}}})])
    assert responses[0]["error"]["code"] == -32602


def test_missing_argument_is_invalid_params():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 8, "method": "tools/call",
                                     "params": {"name": "coverage_audit",
                                                "arguments": {"task": "launch"}}})])
    assert responses[0]["error"]["code"] == -32602


def test_extra_argument_is_invalid_params():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 9, "method": "tools/call",
                                     "params": {"name": "reframe",
                                                "arguments": {"statement": "s", "extra": 1}}})])
    assert responses[0]["error"]["code"] == -32602


@pytest.mark.parametrize(
    ("name", "arguments"),
    [
        ("coverage_audit", {"task": "launch"}),
        ("reframe", {"statement": "s", "extra": True}),
        ("state_diff", {"before": "b", "after": 1}),
        ("cl3_evaluate", {"ops": [{"op": "product", "a": "e1"}]}),
        ("interaction_spectrum", {"points": [[1, 1]], "values": []}),
        ("rotor_transition", {"ops": [{"op": "invert"}]}),
    ],
)
def test_each_tool_returns_invalid_params_for_invalid_arguments(name, arguments):
    responses, _ = call([json.dumps({
        "jsonrpc": "2.0", "id": 20, "method": "tools/call",
        "params": {"name": name, "arguments": arguments},
    })])
    assert responses[0]["error"]["code"] == -32602


def test_cl3_typed_validation_error_is_invalid_params():
    responses, _ = call([json.dumps({
        "jsonrpc": "2.0", "id": 21, "method": "tools/call",
        "params": {"name": "cl3_evaluate", "arguments": {
            "ops": [{"op": "grade", "state": "not-a-state"}],
        }},
    })])
    assert responses[0]["error"]["code"] == -32602


def test_invalid_json_rpc_envelope_is_invalid_request():
    responses, _ = call([json.dumps({"jsonrpc": "1.0", "id": 10, "method": "tools/list"})])
    assert responses[0]["error"]["code"] == -32600


def test_unknown_method_error():
    responses, _ = call([json.dumps({"jsonrpc": "2.0", "id": 7, "method": "bogus"})])
    assert responses[0]["error"]["code"] == -32601


def test_malformed_json_error():
    responses, _ = call(["this is not json"])
    assert responses[0]["error"]["code"] == -32700


def test_non_finite_json_input_is_parse_error():
    responses, _ = call([
        '{"jsonrpc":"2.0","id":30,"method":"tools/list","params":{"x":NaN}}',
        '{"jsonrpc":"2.0","id":31,"method":"tools/list","params":{"x":Infinity}}',
    ])
    assert [response["error"]["code"] for response in responses] == [-32700, -32700]


def test_notifications_never_produce_responses():
    responses, _ = call([
        '{"jsonrpc":"2.0","method":"initialize","params":{}}',
        '{"jsonrpc":"2.0","method":"tools/list"}',
        '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"rotor_transition","arguments":{"ops":[]}}}',
        '{"jsonrpc":"2.0","method":"unknown"}',
        '{"jsonrpc":"1.0","method":"tools/list"}',
        '{"jsonrpc":"2.0","method":"notifications/initialized"}',
    ])
    assert responses == []


def test_no_key_does_not_mask_protocol_errors(monkeypatch):
    env = {key: value for key, value in os.environ.items()
           if key not in ("DEEPSEEK_API_KEY", "OPENROUTER_API_KEY")}
    lines = [
        json.dumps({"jsonrpc": "2.0", "id": 40, "method": "initialize", "params": {}}),
        json.dumps({"jsonrpc": "2.0", "id": 41, "method": "tools/list"}),
        json.dumps({"jsonrpc": "1.0", "id": 42, "method": "tools/list"}),
        json.dumps({"jsonrpc": "2.0", "id": 43, "method": "unknown"}),
        json.dumps({"jsonrpc": "2.0", "id": 44, "method": "tools/call",
                    "params": {"name": "coverage_audit",
                                "arguments": {"task": "t", "plan": "p"}}}),
    ]
    responses, _ = call(lines, sim=False, env=env)
    assert responses[0]["result"]["serverInfo"]["name"] == "iching-tools"
    assert len(responses[1]["result"]["tools"]) == 6
    assert responses[2]["error"]["code"] == -32600
    assert responses[3]["error"]["code"] == -32601
    assert responses[4]["error"]["code"] == -32002


def test_missing_provider_key_is_lazy(monkeypatch):
    from iching_mcp.server import McpError, Server

    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    monkeypatch.delenv("OPENROUTER_API_KEY", raising=False)
    server = Server(sim=False)
    with pytest.raises(McpError) as exc_info:
        server.handle({
            "jsonrpc": "2.0", "id": 50, "method": "tools/call",
            "params": {"name": "coverage_audit",
                        "arguments": {"task": "t", "plan": "p"}},
        })
    assert exc_info.value.code == -32002
