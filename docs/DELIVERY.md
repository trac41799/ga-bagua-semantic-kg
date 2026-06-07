# GA-Bagua Semantic KG — Delivery & Installation Guide

## Quick Start (3 steps)

```
1. Install the binary
2. Wire it into your LLM client config  
3. Install the encoding skill
```

---

## Step 1: Install the Server Binary

### Option A: Pre-built binary (recommended)

Download from [GitHub Releases](https://github.com/YOUR_ORG/ga-bagua-semantic-kg/releases):

| Platform | Binary |
|----------|--------|
| Windows x64 | `ga-semantics-mcp-windows-x86_64.exe` |
| macOS x64 | `ga-semantics-mcp-darwin-x86_64` |
| macOS ARM | `ga-semantics-mcp-darwin-aarch64` |
| Linux x64 | `ga-semantics-mcp-linux-x86_64` |
| Linux ARM | `ga-semantics-mcp-linux-aarch64` |

Place it somewhere on your PATH, or note the full path for the config step.

### Option B: Via npm (Node.js wrapper)

```bash
npm install -g ga-semantics-mcp
```

This downloads the right binary for your platform and makes `ga-semantics-mcp` available
globally. Also works with `npx`:

```bash
npx ga-semantics-mcp
```

### Option C: Via Cargo (Rust toolchain required)

```bash
cargo install ga-semantics-mcp
```

Requires Rust 1.78+. No C compiler needed (pure Rust, no C dependencies).

### Option D: Build from source

```bash
git clone https://github.com/YOUR_ORG/ga-bagua-semantic-kg.git
cd ga-bagua-semantic-kg
cargo build --release -p ga-semantics-mcp
# Binary at: target/release/ga-semantics-mcp[.exe]
```

### Verify installation

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ga-semantics-mcp
```

Should return a JSON-RPC response with server info.

---

## Step 2: Configure Your LLM Client

### Claude Desktop

**Config file:** `%APPDATA%\Claude\claude_desktop_config.json` (Windows) or
`~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or
`~/.config/Claude/claude_desktop_config.json` (Linux)

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

If using a custom path:
```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "C:\\tools\\ga-semantics-mcp.exe"
    }
  }
}
```

Restart Claude Desktop. You should see a hammer icon indicating tools are available.

---

### OpenCode (OpenCode CLI)

**Config file:** `~/.config/opencode/opencode.json` or project-local `.opencode/opencode.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

Also install the encoding skill (see Step 3) to `.opencode/skills/bagua-encoder/SKILL.md`.

---

### Cursor

**Config file:** `~/.cursor/mcp.json` (global) or `.cursor/mcp.json` (project)

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

Cursor supports MCP natively in Composer agent mode. The encoding skill should be placed in
`.cursor/skills/bagua-encoder/SKILL.md` for the LLM to reference it.

---

### Claude Code (CLI)

**Config file:** `~/.claude/claude_desktop_config.json` or `.claude/mcp.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

For Claude Code's custom instructions, add this to `CLAUDE.md`:
```markdown
## GA-Bagua Semantic Encoding

When I ask you to encode a concept, use the `llm_encode` tool with 8 coefficients
in this order: [receptive, causal, transmissive, constraining, clarifying,
influential, balancing, generative]. Read docs/skills/bagua-encoder/SKILL.md
for the encoding guide.
```

---

### Continue.dev (VS Code / JetBrains)

**Config file:** `~/.continue/config.json`

```json
{
  "experimental": {
    "modelContextProtocolServers": [
      {
        "name": "ga-bagua-semantic-kg",
        "command": "ga-semantics-mcp"
      }
    ]
  }
}
```

Copy `docs/skills/bagua-encoder/SKILL.md` to a location the Continue LLM can read
(e.g., include it in your `@docs` context).

---

### Cline (VS Code)

**Config:** Use the MCP Servers view in VS Code (`Ctrl+Shift+P` → "Cline: Open MCP Config")

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

---

### Aider

Aider connects to MCP servers via a helper. Install `aider-mcp-bridge`:

```bash
pip install aider-mcp-bridge
```

Then in `.aider.conf.yml`:
```yaml
mcp_servers:
  - name: ga-bagua-semantic-kg
    command: ga-semantics-mcp
```

---

### Windsurf

**Config file:** `~/.windsurf/mcp_config.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

---

### GitHub Copilot / Copilot Chat

Copilot does not natively support MCP. Workaround: use the CLI directly as a subprocess or
through a plugin. The `ga-semantics-cli` crate provides a command-line interface:

```bash
# Encode a concept
ga-semantics-cli encode 0.04 -0.09 -0.51 0.68 0.21 -0.26 0.17 -0.34

# Show WuXing cycles
ga-semantics-cli wuxing
```

---

## Step 3: Install the Encoding Skill

The LLM needs the encoding guide to produce correct 8-coefficient vectors.
Copy `docs/skills/bagua-encoder/SKILL.md` to your client's skill directory:

| Client | Skill directory |
|--------|----------------|
| OpenCode CLI | `.opencode/skills/bagua-encoder/SKILL.md` (project) or `~/.config/opencode/skills/bagua-encoder/SKILL.md` (global) |
| Claude Code | `.claude/skills/bagua-encoder/SKILL.md` |
| Cursor | `.cursor/skills/bagua-encoder/SKILL.md` |
| General | Any directory the LLM's `@docs` or context system can read |

Alternatively, embed the crib sheet directly in your instructions:

```markdown
When encoding concepts for GA-Bagua, use these 8 roles in order:
[receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]

Encoding scale:
  >0.5 strong  |  0.2–0.5 moderate  |  0.05–0.2 slight
  -0.05–0.05 irrelevant  |  <-0.05 counter-acts  |  <-0.5 strongly counter-acts

Output as JSON array. Normalize to unit length.
```

---

## Verification: Test the Full Pipeline

Once installed, test with any MCP-compatible LLM:

```
User: "Encode a Rate Limiter concept for me"

LLM (reads SKILL.md, assigns coefficients):
  → calls llm_encode("Rate Limiter", [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34])

MCP: Returns dominant role: constraining (Earth/Mountain), all 8 role breakdowns.

User: "Store that as concept #1. Now encode an Auth System and store it as #2."

LLM: → calls llm_encode; then store_llm_concept for each.

User: "What's the relationship between Rate Limiter and Auth System?"

LLM: → calls classify_hexagram with both encodings.

MCP: Returns hexagram Kam over Li (未濟 Wei Ji), relationship: receptive (0.60).

User: "Find the top 3 concepts most similar to a Message Queue encoding."

LLM: → calls store_query_similar with query encoding.

MCP: Returns ranked list with similarity scores.
```

---

## Distribution Checklist

- [ ] Publish `ga-semantics-core` to crates.io
- [ ] Publish `ga-semantics-mcp` to crates.io
- [ ] Set up GitHub Actions to build release binaries
- [ ] Publish npm package `ga-semantics-mcp` to npmjs.com
- [ ] Tag a GitHub release with attached binaries
- [ ] Include SKILL.md in release archive
- [ ] Include SYSTEM_GUIDE.md in release archive
- [ ] Add this DELIVERY.md to the repository

---

## Troubleshooting

### "command not found" or spawn error
- Verify the binary path is correct in the MCP config
- On Windows, try the full path: `C:\\Users\\You\\tools\\ga-semantics-mcp.exe`
- On macOS, you may need to `chmod +x` the binary

### "store_open" returns error
- Ensure the directory for the JSON file is writable
- If the file already exists, verify it's valid JSON

### LLM produces garbage coefficients (e.g., all zeros, wrong sign patterns)
- The LLM needs to read the encoding skill first. Ask it to read `docs/skills/bagua-encoder/SKILL.md`
- Use `validate_encoding` to check coefficient quality

### "No C compiler" error when building from source
- The crate has no C dependencies. This error usually means you're building the wrong
  package or a dependency got pulled in. Make sure you're running `cargo build -p ga-semantics-mcp`
  (not the workspace root which might include optional features you don't need).
- The `store` feature (JSON file store) is pure Rust. The `python` feature is off by default.

### MCP server starts but tools don't appear
- The client might need a restart after config changes
- Check the client's MCP logs for errors
- Verify the JSON-RPC initialize response is correct: `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ga-semantics-mcp`

---

## Architecture of Distribution

```
┌─────────────────────────────────────────────────────────┐
│                     GitHub Release                       │
│  ┌──────────────────────────────────────────────────┐   │
│  │ ga-semantics-mcp-windows-x86_64.exe              │   │
│  │ ga-semantics-mcp-darwin-x86_64                   │   │
│  │ ga-semantics-mcp-darwin-aarch64                  │   │
│  │ ga-semantics-mcp-linux-x86_64                    │   │
│  │ ga-semantics-mcp-linux-aarch64                   │   │
│  │ docs/skills/bagua-encoder/SKILL.md               │   │
│  │ docs/SYSTEM_GUIDE.md                             │   │
│  │ docs/DELIVERY.md                                 │   │
│  └──────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
         │                              │
         ▼                              ▼
   ┌──────────┐                  ┌──────────┐
   │ crates.io │                  │  npmjs   │
   │ (source)  │                  │ (binary  │
   │           │                  │  wrapper)│
   └──────────┘                  └──────────┘
         │                              │
         └──────────┬───────────────────┘
                    ▼
         ┌──────────────────┐
         │   User's Machine  │
         │  ┌──────────────┐ │
         │  │ MCP Config   │ │──▶ Claude / Cursor / OpenCode / ...
         │  │ + SKILL.md   │ │
         │  │ + binary     │ │
         │  └──────────────┘ │
         └──────────────────┘
```

The server is a single static binary with zero runtime dependencies. The skill is a
single markdown file that any LLM can read. Together they form a self-contained
knowledge graph engine that runs locally with no cloud services, no API keys, and
no database server.
