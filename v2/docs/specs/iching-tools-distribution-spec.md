# SDD - iching-tools Distribution and MCP Contract Hardening

**Status:** Superseding implementation specification  
**Product:** `v2/products/iching-tools`  
**Purpose:** make the bounded tools installable and mechanically trustworthy without promoting unvalidated LLM quality claims.

## 1. Package Layout

Create an installed package named `iching_mcp`:

```text
iching_mcp/__init__.py
iching_mcp/__main__.py
iching_mcp/contracts.py
iching_mcp/server.py
```

Keep `mcp/server.py` as a source compatibility wrapper. Do not add `mcp/__init__.py`; that can shadow the official MCP SDK.

Add console entry point:

```toml
iching-mcp = "iching_mcp.server:main"
```

The official MCP SDK remains a test dependency, not a runtime dependency. The server runtime remains stdlib-only.

## 2. Shared Provider Configuration

Create `iching_tools/providers.py`:

```python
@dataclass(frozen=True)
class ProviderConfig:
    provider: Literal["deepseek", "openrouter"]
    api_key: str
    api_base_url: str
    model: str

def resolve_provider(*, explicit_key=None, explicit_provider=None,
                     model="deepseek-chat", environ=None) -> ProviderConfig | None: ...
```

Rules:

- explicit provider wins;
- `DEEPSEEK_API_KEY` defaults to DeepSeek;
- `OPENROUTER_API_KEY` defaults to OpenRouter when DeepSeek is absent;
- explicit key without provider retains DeepSeek backward compatibility;
- missing provider key is a configuration error;
- secrets never appear in `repr`, logs, provenance, or errors;
- OpenRouter model strings such as `openai/gpt-4o-mini` are passed unchanged.

## 3. Six-Tool Contracts

Create `iching_mcp/contracts.py` with argument and result validators for:

- `coverage_audit`;
- `reframe`;
- `state_diff`;
- `cl3_evaluate`;
- `interaction_spectrum`;
- `rotor_transition`.

Set `additionalProperties: false` in input schemas. Missing, extra, or wrong-typed arguments return `-32602`.

## 4. MCP Error Matrix

| Condition | Code |
|---|---:|
| Invalid JSON | `-32700` |
| Invalid JSON-RPC envelope | `-32600` |
| Unknown method | `-32601` |
| Unknown tool or tool arguments | `-32602` |
| Missing provider key | `-32002` |
| LLM transport/protocol failure | `-32000` |

Successful calls remain one JSON text content item. Every successful tool result must pass its result validator before serialization.

## 5. Clean Distribution

The fresh wheel and sdist must contain:

- all six `iching_*` packages;
- `iching_mcp`;
- the `iching-mcp` entry point;
- README and license metadata.

The wheel must not rely on repository `PYTHONPATH` or the source `mcp/server.py` wrapper.

## 6. Official SDK Verification

One simulator session must initialize, list exactly six tools, call every tool, assert `is_error is False`, parse the returned JSON, and validate the result shape. The installed-wheel test must run with repository paths removed.

## 7. Acceptance

| ID | Acceptance |
|---|---|
| D1 | Provider routing tests cover DeepSeek/OpenRouter precedence |
| D2 | Six tool schemas and result validators are enforced |
| D3 | Raw JSON-RPC errors use the frozen matrix |
| D4 | Fresh wheel installs into a clean environment |
| D5 | Official SDK calls all six tools in simulator mode |
| D6 | Documentation and package version agree |
