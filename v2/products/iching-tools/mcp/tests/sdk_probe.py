import asyncio
import json
import os
import sys
from pathlib import Path


def configure():
    mode = sys.argv[1] if len(sys.argv) > 1 else "source"
    probe_root = Path(__file__).resolve().parents[2]
    if mode == "--installed":
        cwd = Path(sys.argv[2]).resolve()
        sys.path[:] = [
            path for path in sys.path
            if not path or not _under(Path(path).resolve(), probe_root)
        ]
        sys.path.insert(0, str(cwd))
        return cwd, ["-m", "iching_mcp", "--sim"]

    for package in ("coverage", "reframe", "statediff", "cl3calc", "xai", "rotor"):
        package_path = str(probe_root / package)
        if package_path not in sys.path:
            sys.path.insert(0, package_path)
    if str(probe_root) not in sys.path:
        sys.path.insert(0, str(probe_root))
    return probe_root, [str(probe_root / "mcp" / "server.py"), "--sim"]


def _under(path, root):
    try:
        path.relative_to(root)
    except ValueError:
        return False
    return True


ROOT, SERVER_ARGS = configure()
from mcp import ClientSession, StdioServerParameters  # noqa: E402
from mcp.client.stdio import stdio_client  # noqa: E402
from iching_mcp.contracts import validate_tool_result  # noqa: E402


TOOL_ARGUMENTS = {
    "coverage_audit": {"task": "launch", "plan": "Build it."},
    "reframe": {"statement": "We should raise prices."},
    "state_diff": {"before": "cache 94% latency 120ms", "after": "cache 99% latency 95ms"},
    "cl3_evaluate": {"ops": [
        {"op": "product", "a": "e2", "b": "e1"},
        {"op": "complement", "state": "kan"},
    ]},
    "interaction_spectrum": {
        "points": [[1, 1], [1, -1], [-1, 1], [-1, -1]],
        "values": [1.0, -1.0, -1.0, 1.0],
    },
    "rotor_transition": {"ops": [{
        "op": "apply",
        "r": [0.7071067811865476, -0.7071067811865476, 0.0, 0.0],
        "blade": "e1",
    }]},
}


async def run():
    params = StdioServerParameters(
        command=sys.executable,
        args=SERVER_ARGS,
        cwd=str(ROOT),
        env=dict(os.environ),
    )
    async with stdio_client(params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            tools = await session.list_tools()
            names = [tool.name for tool in tools.tools]
            expected = list(TOOL_ARGUMENTS)
            assert names == expected, names
            print("NAMES:", json.dumps(names))

            for name in names:
                result = await session.call_tool(name, TOOL_ARGUMENTS[name])
                is_error = getattr(result, "is_error", getattr(result, "isError", None))
                assert is_error is False, name
                assert len(result.content) == 1, name
                text = result.content[0].text
                data = json.loads(text)
                validate_tool_result(name, data)
            print("CALLS:", json.dumps(names))


asyncio.run(run())
