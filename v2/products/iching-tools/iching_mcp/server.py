"""Dependency-free stdio JSON-RPC MCP server for all six iching-tools."""

import json
import sys

from iching_cl3calc import Cl3CalcError, evaluate as cl3_evaluate
from iching_coverage import audit
from iching_coverage.llm_client import SimulatedLLM as CoverageSimulatedLLM
from iching_reframe import reframe
from iching_reframe.llm_client import SimulatedLLM as ReframeSimulatedLLM
from iching_rotor import evaluate as rotor_evaluate
from iching_statediff import summarize
from iching_statediff.llm_client import SimulatedLLM as StateDiffSimulatedLLM
from iching_tools.providers import ProviderConfigurationError, resolve_provider
from iching_xai import identify as identify_spectrum
from iching_xai import interaction_spectrum

from . import __version__
from .contracts import (
    ContractError,
    TOOL_DESCRIPTIONS,
    TOOL_NAMES,
    TOOL_SCHEMAS,
    validate_tool_arguments,
    validate_tool_result,
)


PROTOCOL_VERSION = "2024-11-05"
SERVER_INFO = {"name": "iching-tools", "version": __version__}
TOOLS = [
    {
        "name": name,
        "description": TOOL_DESCRIPTIONS[name],
        "inputSchema": TOOL_SCHEMAS[name],
    }
    for name in TOOL_NAMES
]


class McpError(Exception):
    """An error that maps directly to a frozen JSON-RPC error code."""

    def __init__(self, code, message):
        super().__init__(message)
        self.code = code
        self.message = message


def _safe_error_message(exc, prefix):
    """Keep exception details useful without ever reflecting credentials."""
    message = str(exc).strip()
    if not message or "api_key" in message.lower() or "authorization" in message.lower():
        return prefix
    return f"{prefix}: {message}"


class Server:
    def __init__(self, sim=False, provider_config=None):
        self.sim = sim
        self.provider_config = provider_config
        self.clients = {}

    def _get_client(self, name):
        if name in self.clients:
            return self.clients[name]

        if self.sim:
            simulated = {
                "coverage_audit": CoverageSimulatedLLM,
                "reframe": ReframeSimulatedLLM,
                "state_diff": StateDiffSimulatedLLM,
            }
            self.clients[name] = simulated[name]()
            return self.clients[name]

        try:
            config = self.provider_config
            if config is None:
                config = resolve_provider()
        except ProviderConfigurationError as exc:
            raise McpError(-32002, str(exc)) from None
        if config is None:
            raise McpError(
                -32002,
                "no API key: set DEEPSEEK_API_KEY or OPENROUTER_API_KEY (or use --sim)",
            )

        if name == "coverage_audit":
            from iching_coverage.llm_client import LLMClient
        elif name == "reframe":
            from iching_reframe.llm_client import LLMClient
        else:
            from iching_statediff.llm_client import LLMClient
        self.clients[name] = LLMClient(provider_config=config)
        return self.clients[name]

    def handle(self, message):
        response = self._handle(message)
        if isinstance(message, dict) and "id" not in message:
            return None
        return response

    def _handle(self, message):
        if not isinstance(message, dict) or message.get("jsonrpc") != "2.0":
            raise McpError(-32600, "invalid request")
        if "method" not in message or not isinstance(message["method"], str):
            raise McpError(-32600, "invalid request")
        if "id" in message and (
            isinstance(message["id"], (dict, list, bool))
        ):
            raise McpError(-32600, "invalid request")

        method = message["method"]
        request_id = message.get("id")
        if method == "initialize":
            params = message.get("params", {})
            if not isinstance(params, dict):
                raise McpError(-32602, "initialize params must be an object")
            return {
                "jsonrpc": "2.0",
                "id": request_id,
                "result": {
                    "protocolVersion": PROTOCOL_VERSION,
                    "capabilities": {"tools": {}},
                    "serverInfo": SERVER_INFO,
                },
            }
        if method == "notifications/initialized":
            return None
        if method == "tools/list":
            params = message.get("params", {})
            if not isinstance(params, dict):
                raise McpError(-32602, "tools/list params must be an object")
            return {"jsonrpc": "2.0", "id": request_id, "result": {"tools": TOOLS}}
        if method == "tools/call":
            params = message.get("params")
            if not isinstance(params, dict):
                raise McpError(-32602, "tools/call params must be an object")
            name = params.get("name")
            if not isinstance(name, str) or name not in TOOL_SCHEMAS:
                raise McpError(-32602, f"unknown tool: {name}")
            if "arguments" not in params:
                arguments = {}
            else:
                arguments = params["arguments"]
            try:
                validate_tool_arguments(name, arguments)
            except ContractError as exc:
                raise McpError(-32602, str(exc)) from None
            text = self._call(name, arguments)
            return {
                "jsonrpc": "2.0",
                "id": request_id,
                "result": {"content": [{"type": "text", "text": text}]},
            }
        raise McpError(-32601, f"method not found: {method}")

    def _call(self, name, arguments):
        try:
            validate_tool_arguments(name, arguments)
        except ContractError as exc:
            raise McpError(-32602, str(exc)) from None
        try:
            if name == "cl3_evaluate":
                result = cl3_evaluate(arguments["ops"])
            elif name == "interaction_spectrum":
                spectrum = interaction_spectrum(arguments["points"], arguments["values"])
                result = {"spectrum": spectrum, "identified": identify_spectrum(spectrum)}
            elif name == "rotor_transition":
                result = rotor_evaluate(arguments["ops"])
            elif name == "coverage_audit":
                result = audit(arguments["task"], arguments["plan"], self._get_client(name))
            elif name == "reframe":
                result = reframe(arguments["statement"], self._get_client(name))
            elif name == "state_diff":
                result = summarize(arguments["before"], arguments["after"], self._get_client(name))
            else:
                raise McpError(-32602, f"unknown tool: {name}")
        except McpError:
            raise
        except Cl3CalcError as exc:
            raise McpError(-32602, _safe_error_message(exc, "invalid tool arguments")) from None
        except (TypeError, ValueError, KeyError, IndexError) as exc:
            if name in {"cl3_evaluate", "interaction_spectrum", "rotor_transition"}:
                raise McpError(-32602, _safe_error_message(exc, "invalid tool arguments")) from None
            raise McpError(-32000, _safe_error_message(exc, "tool execution failed")) from None
        except Exception as exc:  # noqa: BLE001 - classify all transport/protocol failures
            raise McpError(-32000, _safe_error_message(exc, "tool execution failed")) from None

        try:
            validate_tool_result(name, result)
        except ContractError:
            raise McpError(-32000, "tool result violates its output contract") from None
        try:
            return json.dumps(result, ensure_ascii=False, allow_nan=False)
        except (TypeError, ValueError):
            raise McpError(-32000, "tool result is not JSON serializable") from None


def _error_response(message, error):
    request_id = message.get("id") if isinstance(message, dict) else None
    if request_id is not None and (
        isinstance(request_id, (dict, list, bool))
    ):
        request_id = None
    return {
        "jsonrpc": "2.0",
        "id": request_id,
        "error": {"code": error.code, "message": error.message},
    }


def _reject_non_finite_json(value):
    raise ValueError(f"non-finite JSON number: {value}")


def _is_notification(message):
    return isinstance(message, dict) and "id" not in message


def main(sim=None, argv=None):
    args = sys.argv[1:] if argv is None else list(argv)
    if sim is None:
        sim = "--sim" in args
    try:
        server = Server(sim=sim)
        initialization_error = None
    except McpError as exc:
        server = None
        initialization_error = exc

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        message = None
        try:
            message = json.loads(line, parse_constant=_reject_non_finite_json)
        except (json.JSONDecodeError, ValueError, TypeError):
            output = {
                "jsonrpc": "2.0",
                "id": None,
                "error": {"code": -32700, "message": "parse error"},
            }
        else:
            if initialization_error is not None:
                output = _error_response(message, initialization_error)
            else:
                try:
                    output = server.handle(message)
                except McpError as exc:
                    output = _error_response(message, exc)
                except Exception:
                    output = _error_response(message, McpError(-32000, "internal server error"))
        if _is_notification(message):
            output = None
        if output is not None:
            sys.stdout.write(json.dumps(output, ensure_ascii=True, allow_nan=False) + "\n")
            sys.stdout.flush()


if __name__ == "__main__":
    main()
