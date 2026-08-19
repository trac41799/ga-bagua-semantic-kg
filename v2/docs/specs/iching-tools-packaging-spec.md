# SDD — iching-tools Packaging, CI, MCP-SDK Compatibility, Skill Deployment

**Status:** Pre-registered | **Purpose:** close the distribution gap (the competitiveness blocker): build-verified packaging, CI workflow, a REAL MCP-SDK compatibility test (official `mcp.client.stdio` client), and SKILL.md deployment files for the three validated capabilities.

## 1. Deliverables
| Item | Contract |
|------|----------|
| **Build verification** | `python -m build` produces a wheel + sdist; `pip install` the wheel into a clean venv; `itools --version` works from the installed console script; all 3 subcommands run in sim mode |
| **CI workflow** | `.github/workflows/ci.yml` for iching-tools: pytest all suites (40), bench smoke (sim), build check — on push/PR |
| **MCP-SDK compatibility** | Test using the OFFICIAL `mcp.client.stdio.stdio_client`: initialize, tools/list, tools/call per tool (sim mode) — our stdio server must be consumable by a real MCP client, not just our test driver |
| **SKILL.md deployment** | 3 skill files (`skills/coverage-audit/SKILL.md`, `skills/reframe/SKILL.md`, `skills/state-diff/SKILL.md`): validated prompts + usage + evidence recap — for agents that consume skills instead of MCP |

## 2. ACs (TDD `iching-tools-packaging-tdd.md`)
- P1 `python -m build` succeeds; wheel installs in a clean venv; `itools --version` = 0.2.0
- P2 CI workflow file present and valid YAML with the required jobs
- P3 MCP-SDK test: official client lists 6 tools and calls each successfully (sim)
- P4 3 SKILL.md files present with validated prompts + evidence
- P5 zero regressions: prior suites stay green
