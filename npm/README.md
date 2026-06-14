# GA-Bagua Semantic KG — MCP Server

LLM semantic memory layer using Geometric Algebra and the I-Ching Bagua taxonomy.

## Quick Start — One Command Setup

### Option 1: One-liner (no npm needed)

**Unix / macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/npm/setup.js | node -
```

**Windows PowerShell:**
```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/npm/setup.js" -OutFile "$env:TEMP\ga-setup.js"; node "$env:TEMP\ga-setup.js"
```

The setup script will:
1. Download the correct binary for your platform
2. Auto-detect installed coding agent harnesses (Claude, OpenCode, Cursor, etc.)
3. Configure MCP server settings for each detected agent
4. Install the Bagua encoder skill into agent skill directories

### Option 2: Via npm

```bash
npm install -g ga-semantics-mcp
```

Then run the setup wizard:
```bash
npx ga-semantics-setup
```

Or just start the MCP server:
```bash
npx ga-semantics-mcp
```

### Option 3: Via Cargo (Rust toolchain)

```bash
cargo install ga-semantics-mcp
```

Then configure your LLM client manually (see Example Config below).

## Supported Agents

The setup auto-detects and configures:

| Agent | Auto-Config | Auto-Skill |
|-------|------------|------------|
| Claude Desktop | MCP config | — |
| Claude Code (CLI) | MCP config | CLAUDE.md + skill dir |
| OpenCode | MCP config | .opencode/skills/ |
| Cursor | MCP config | .cursor/skills/ |
| Continue.dev | MCP config | .continue/skills/ |
| Windsurf | MCP config | .windsurf/skills/ |
| Aider | Manual instructions | — |
| Cline (VS Code) | Manual instructions | .cline/skills/ |
| Codex (OpenAI) | CLI fallback | — |

## Example MCP Config

If configuring manually, add to your client's MCP config:

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

See [Delivery Guide](https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/docs/DELIVERY.md) for per-client config details.

## Setup Options

```
--help              Show help
--quiet             Minimal output
--yes               Auto-confirm all prompts
--path <dir>        Binary install directory (default: ~/.ga-semantics/bin)
--skip-binary       Skip binary download (use existing)
--skip-config       Skip MCP config installation
--skip-skills       Skip skill file installation
--agents <list>     Only configure specific agents (comma-separated)
--list-agents       List detected agents without installing
```

## What it does

- **Encode any concept** into an 8-number vector using the LLM's semantic understanding
- **Classify relationships** between concepts using WuXing cycle dynamics (generating, controlling, clarifying, etc.)
- **Compute similarity** with role-weighted metrics
- **Solve analogies** (A:B :: C:?) deterministically
- **Persist knowledge graphs** as JSON files with query and export capabilities
- **29 MCP tools** for LLM-driven semantic reasoning

## Documentation

- [System Guide](https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/docs/SYSTEM_GUIDE.md) — complete technical reference
- [Delivery Guide](https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/docs/DELIVERY.md) — configuration for every LLM client
- [Encoding Skill](https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/docs/skills/bagua-encoder/SKILL.md) — the LLM encoding protocol

## License

MIT OR Apache-2.0
