"""
GA-Bagua MCP Client — communicates with ga-semantics-mcp binary via JSON-RPC over stdio.
"""
import subprocess
import json
import os
import time
from typing import Optional

class McpClient:
    """Client for the GA-Bagua MCP server over stdio."""

    def __init__(self, binary_path: Optional[str] = None):
        self.binary_path = binary_path or self._find_binary()
        self.process: Optional[subprocess.Popen] = None
        self._call_counter = 0

    @staticmethod
    def _find_binary() -> str:
        """Find the ga-semantics-mcp binary."""
        candidates = [
            os.path.abspath(os.path.join(os.path.dirname(__file__), "../../target/debug/ga-semantics-mcp.exe")),
            os.path.abspath(os.path.join(os.path.dirname(__file__), "../../target/release/ga-semantics-mcp.exe")),
            os.path.abspath(os.path.join(os.path.dirname(__file__), "../../target/debug/ga-semantics-mcp")),
            os.path.abspath(os.path.join(os.path.dirname(__file__), "../../target/release/ga-semantics-mcp")),
        ]
        for c in candidates:
            if os.path.exists(c):
                return c
        raise FileNotFoundError(f"Cannot find ga-semantics-mcp binary. Tried: {candidates}")

    def start(self):
        """Spawn the MCP server process."""
        if self.process is not None:
            return
        self.process = subprocess.Popen(
            [self.binary_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        self._initialize()

    def stop(self):
        """Kill the MCP server process."""
        if self.process:
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
            self.process = None

    def _send_request(self, request: dict) -> dict:
        """Send a JSON-RPC request and return the response."""
        if self.process is None or self.process.stdin is None or self.process.stdout is None:
            raise RuntimeError("MCP server not started")

        request["jsonrpc"] = "2.0"
        request["id"] = request.get("id", 1)
        request_str = json.dumps(request) + "\n"

        self.process.stdin.write(request_str)
        self.process.stdin.flush()

        response_line = self.process.stdout.readline()
        if not response_line:
            stderr_output = self.process.stderr.read() if self.process.stderr else ""
            raise RuntimeError(f"MCP server returned empty response. Stderr: {stderr_output}")

        return json.loads(response_line)

    def _initialize(self):
        """Send MCP initialize handshake."""
        response = self._send_request({
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "ga-benchmarks", "version": "0.1.0"}
            }
        })
        if "error" in response:
            raise RuntimeError(f"MCP initialize failed: {response['error']}")

    def call_tool(self, name: str, arguments: dict) -> tuple[dict, float]:
        """Call a tool and return (result, latency_seconds)."""
        self._call_counter += 1
        start = time.perf_counter()
        response = self._send_request({
            "id": self._call_counter,
            "method": "tools/call",
            "params": {"name": name, "arguments": arguments}
        })
        latency = time.perf_counter() - start

        if "error" in response:
            raise RuntimeError(f"Tool '{name}' error: {response['error']}")

        return response.get("result", {}), latency

    def list_tools(self) -> dict:
        response = self._send_request({"method": "tools/list", "params": {}})
        return response.get("result", {})

    def llm_encode(self, name: str, coefficients: list[float]) -> tuple[dict, float]:
        return self.call_tool("llm_encode", {"name": name, "coefficients": coefficients})

    def store_llm_concept(self, name: str, coefficients: list[float], text: str) -> tuple[dict, float]:
        return self.call_tool("store_llm_concept", {"name": name, "coefficients": coefficients, "text": text})

    def store_query_similar(self, query: list[float], top_k: int = 10) -> tuple[dict, float]:
        return self.call_tool("store_query_similar", {"query": query, "top_k": top_k})

    def classify_relation(self, a: list[float], b: list[float]) -> tuple[dict, float]:
        return self.call_tool("classify_relation", {"a": a, "b": b})

    def detect_contradiction(self, a: list[float], b: list[float], threshold: float = 0.5) -> tuple[dict, float]:
        return self.call_tool("detect_contradiction", {"a": a, "b": b, "threshold": threshold})

    def semantic_similarity(self, a: list[float], b: list[float]) -> tuple[dict, float]:
        return self.call_tool("semantic_similarity", {"a": a, "b": b})

    def store_list_concepts(self) -> tuple[dict, float]:
        return self.call_tool("store_list_concepts", {})

    def store_export(self) -> tuple[dict, float]:
        return self.call_tool("store_export", {})

    def validate_encoding(self, a: list[float], b: list[float], expected_relation: str) -> tuple[dict, float]:
        return self.call_tool("validate_encoding", {"a": a, "b": b, "expected_relation": expected_relation})

    def multivector_describe(self, coeffs: list[float]) -> tuple[dict, float]:
        return self.call_tool("multivector_describe", {"multivector": coeffs})

    def store_open(self, path: str) -> tuple[dict, float]:
        return self.call_tool("store_open", {"path": path})

    def store_close(self) -> tuple[dict, float]:
        return self.call_tool("store_close", {})

    def __enter__(self):
        self.start()
        return self

    def __exit__(self, *args):
        self.stop()
