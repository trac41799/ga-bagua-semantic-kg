# B4 — real-mode integration smoke (CLI + MCP)

| component | exit 0 | schema valid |
|---|---|---|
| coverage CLI | True | True |
| reframe CLI | True | True |
| statediff CLI | False | False |
| MCP tools (3 real calls) | False | False |

**VERDICT: FAIL**

- statediff defect: rc=False schema=False stderr=error: expected exactly 3 aspect lines, got 0

- MCP defect: error: expected exactly 3 aspect lines, got 0
