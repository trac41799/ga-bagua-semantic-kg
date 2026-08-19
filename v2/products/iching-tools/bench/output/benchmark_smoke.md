# B4 — real-mode integration smoke (CLI + MCP)

| component | exit 0 | schema valid |
|---|---|---|
| coverage CLI | False | False |
| reframe CLI | True | True |
| statediff CLI | True | True |
| MCP tools (3 real calls) | True | True |

**VERDICT: FAIL**

- coverage defect: rc=False schema=False stderr=Traceback (most recent call last):
  File "<frozen runpy>", line 198, in _run_module_as_main
  File "<frozen runpy>", line 88, in _run_code
  File "D:
