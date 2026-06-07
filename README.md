# GA-Bagua Semantic KG

**LLM semantic memory layer** — encode any concept into an 8-number vector,
then reason about relationships, similarity, analogy, and retrieval through
pure algebra. Zero training. Zero repeated LLM calls. Zero database server.

```
Concept → LLM (one-shot, ~200 tokens) → [0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]
                                              │
                    ┌─────────────────────────┼─────────────────────────┐
                    ▼                         ▼                         ▼
              similarity()              classify()                analogy()
              34ns–320µs                deterministic            deterministic
```

| Metric | Score |
|--------|:-----:|
| Relation Classification | **100%** |
| Dominant Role Detection | **100%** |
| Retrieval Precision@K | **73.3%** |
| Retrieval MRR | **0.878** |
| Analogy Accuracy | **80%** |
| Combined Score | **86.2%** |

---

## LLM Prompt: Self-Install

**Paste the prompt for your LLM client below.**

---

### Claude Desktop

```
I want you to install the GA-Bagua Semantic KG MCP server for Claude Desktop.
This is a local knowledge graph engine that gives you 29 semantic reasoning tools.

Step 1 — Detect my OS and download the binary:
- Windows: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract and place the binary:
- Windows: extract the .zip, move ga-semantics-mcp-x86_64-pc-windows-msvc.exe to %USERPROFILE%\.local\bin\ga-semantics-mcp.exe (create the folder if needed)
- macOS/Linux: tar -xzf the archive, move the binary to ~/.local/bin/ga-semantics-mcp, run chmod +x ~/.local/bin/ga-semantics-mcp

Step 3 — Configure Claude Desktop:
Read the existing file at:
- Windows: %APPDATA%\Claude\claude_desktop_config.json
- macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
- Linux: ~/.config/Claude/claude_desktop_config.json

If the file exists, merge this into the mcpServers object. If it doesn't exist, create it:
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "<full path to the binary from Step 2>"
    }
  }
}

Step 4 — Download the encoding skill:
Save https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
to %APPDATA%\Claude\skills\bagua-encoder\SKILL.md (Windows) or
~/.config/Claude/skills/bagua-encoder/SKILL.md (macOS/Linux).
Create directories as needed.

Step 5 — Verify:
Ask me to restart Claude Desktop, then test by saying:
"Encode 'Rate Limiter' using llm_encode with dominant role constraining"
Use the coefficients: [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]
The tool should return success with dominant_role: "constraining".

Full documentation: https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/docs/SYSTEM_GUIDE.md
```

---

### OpenCode

```
I want you to install the GA-Bagua Semantic KG MCP server for OpenCode CLI.
This is a local knowledge graph engine that gives you 29 semantic reasoning tools.

Step 1 — Detect my OS and download the binary:
- Windows: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract and place the binary:
- Windows: extract the .zip, move the .exe to %USERPROFILE%\.local\bin\ga-semantics-mcp.exe (create folder if needed)
- macOS/Linux: tar -xzf the archive, move binary to ~/.local/bin/ga-semantics-mcp, run chmod +x ~/.local/bin/ga-semantics-mcp

Step 3 — Configure OpenCode:
Read or create ~/.config/opencode/opencode.json (Linux/macOS) or %USERPROFILE%\.config\opencode\opencode.json (Windows).
Merge this into the file (add to existing mcpServers if present):
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "<full path to the binary from Step 2>"
    }
  }
}

Step 4 — Install the encoding skill:
Download https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
Save to ~/.config/opencode/skills/bagua-encoder/SKILL.md (Linux/macOS) or
%USERPROFILE%\.config\opencode\skills\bagua-encoder\SKILL.md (Windows).
Create directories as needed.

Step 5 — Verify:
Ask me to restart OpenCode, then test by saying:
"Use llm_encode for 'Rate Limiter' with coefficients [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]"
The result should show dominant_role: "constraining".
```

---

### Cursor

```
I want you to install the GA-Bagua Semantic KG MCP server for Cursor IDE.
This gives you 29 semantic reasoning tools including encoding, classification, and analogy solving.

Step 1 — Detect my OS and download the binary:
- Windows: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: download https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract and place:
- Windows: extract .zip, move .exe to %USERPROFILE%\.cursor\bin\ga-semantics-mcp.exe (create folders)
- macOS/Linux: tar -xzf, move to ~/.cursor/bin/ga-semantics-mcp, chmod +x ~/.cursor/bin/ga-semantics-mcp

Step 3 — Configure Cursor:
Read or create ~/.cursor/mcp.json (macOS/Linux) or %USERPROFILE%\.cursor\mcp.json (Windows).
Add:
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "<full path to the binary>"
    }
  }
}

Step 4 — Install the skill:
Download and save: https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
To ~/.cursor/skills/bagua-encoder/SKILL.md (macOS/Linux) or %USERPROFILE%\.cursor\skills\bagua-encoder\SKILL.md (Windows).

Step 5 — Verify:
In Cursor's Composer agent mode, say:
"Use llm_encode tool for concept 'Rate Limiter' with coefficients [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]"
Should return dominant_role: "constraining".
```

---

### Claude Code (CLI)

```
Install the GA-Bagua Semantic KG MCP server for Claude Code CLI.

Step 1 — Download the right binary for my OS:
- Windows: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract and place:
- Windows: extract .zip, move .exe to %USERPROFILE%\.claude\bin\ga-semantics-mcp.exe
- macOS/Linux: tar -xzf, move to ~/.claude/bin/ga-semantics-mcp, chmod +x ~/.claude/bin/ga-semantics-mcp

Step 3 — Configure Claude Code:
Read or create ~/.claude/mcp.json (macOS/Linux) or ~/.claude/claude_desktop_config.json.
Add:
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "<full path to binary>"
    }
  }
}

Step 4 — Install skill:
Save https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
to ~/.claude/skills/bagua-encoder/SKILL.md (macOS/Linux) or %USERPROFILE%\.claude\skills\bagua-encoder\SKILL.md (Windows).

Step 5 — Verify by running: echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | <path-to-binary>
Should output 29 tools. Then in Claude Code, use "llm_encode" with [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34].
```

---

### Continue.dev (VS Code / JetBrains)

```
Install the GA-Bagua Semantic KG MCP server for Continue.dev.

Step 1 — Download binary for my OS:
- Windows: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract and place:
- Windows: extract .zip, move .exe to %USERPROFILE%\.continue\bin\ga-semantics-mcp.exe
- macOS/Linux: tar -xzf, move to ~/.continue/bin/ga-semantics-mcp, chmod +x ~/.continue/bin/ga-semantics-mcp

Step 3 — Configure Continue:
Read or create ~/.continue/config.json. Add under "experimental":
{
  "experimental": {
    "modelContextProtocolServers": [
      {
        "name": "ga-bagua-semantic-kg",
        "command": "<full path to binary>"
      }
    ]
  }
}

Step 4 — Install skill:
Save https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
to ~/.continue/skills/bagua-encoder/SKILL.md (macOS/Linux) or %USERPROFILE%\.continue\skills\bagua-encoder\SKILL.md (Windows).

Step 5 — Verify in Continue chat: use @mcp to list tools, then call llm_encode.
```

---

### Cline (VS Code)

```
Install the GA-Bagua Semantic KG MCP server for Cline in VS Code.

Step 1 — Download binary:
- Windows: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract:
- Windows: extract .zip, move .exe to %USERPROFILE%\.cline\bin\ga-semantics-mcp.exe
- macOS/Linux: tar -xzf, move to ~/.cline/bin/ga-semantics-mcp, chmod +x

Step 3 — Open VS Code, press Ctrl+Shift+P, type "Cline: Open MCP Config".
If the file doesn't exist, create it with:
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "<full path to binary>"
    }
  }
}
If it exists, merge the ga-bagua-semantic-kg entry into the existing mcpServers object.

Step 4 — Install skill:
Save https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
to %USERPROFILE%\.cline\skills\bagua-encoder\SKILL.md (Windows) or ~/.cline/skills/bagua-encoder/SKILL.md (macOS/Linux).

Step 5 — In Cline chat, run: llm_encode(name="Rate Limiter", coefficients=[0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34])
```

---

### Windsurf

```
Install the GA-Bagua Semantic KG MCP server for Windsurf.

Step 1 — Download binary for my OS:
- Windows: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-pc-windows-msvc.zip
- macOS Intel: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-apple-darwin.tar.gz
- macOS Apple Silicon: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-apple-darwin.tar.gz
- Linux x64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-x86_64-unknown-linux-gnu.tar.gz
- Linux ARM64: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest/download/ga-semantics-mcp-aarch64-unknown-linux-gnu.tar.gz

Step 2 — Extract and place:
- Windows: extract .zip, move .exe to %USERPROFILE%\.windsurf\bin\ga-semantics-mcp.exe
- macOS/Linux: tar -xzf, move to ~/.windsurf/bin/ga-semantics-mcp, chmod +x

Step 3 — Configure Windsurf:
Read or create ~/.windsurf/mcp_config.json. Add:
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "<full path to binary>"
    }
  }
}

Step 4 — Install skill:
Save https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
to ~/.windsurf/skills/bagua-encoder/SKILL.md (macOS/Linux) or %USERPROFILE%\.windsurf\skills\bagua-encoder\SKILL.md (Windows).

Step 5 — Verify in Cascade: ask it to call llm_encode.
```

---

### Generic / Other MCP Clients

```
Install the GA-Bagua Semantic KG MCP server for my MCP-compatible LLM client.

Step 1 — Download the binary for my platform:
https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest

Step 2 — Extract:
- Windows .zip: extract, place .exe somewhere on PATH
- macOS/Linux .tar.gz: tar -xzf, place binary in ~/.local/bin/, chmod +x

Step 3 — Configure my MCP client to run:
{
  "command": "<full path to binary>",
  "args": []
}
With server name: "ga-bagua-semantic-kg"

Step 4 — Download the encoding skill:
https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
Save to your client's skills/instructions directory.

Step 5 — Verify with: echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | <binary-path>
```

---

## Manual Installation

### npm

```bash
npm install -g ga-semantics-mcp
# or
npx ga-semantics-mcp
```

### Cargo

```bash
cargo install ga-semantics-mcp
```

### Build from source

```bash
git clone https://github.com/trac41799/ga-bagua-semantic-kg.git
cd ga-bagua-semantic-kg
cargo build --release -p ga-semantics-mcp
```

Pure Rust — no C compiler needed.

---

## Encoding Crib Sheet

Save [SKILL.md](docs/skills/bagua-encoder/SKILL.md) to your client's skill directory,
or embed this directly in your LLM instructions:

```
Encode concepts with 8 roles in order:
[receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]

Scale: >0.5 strong | 0.2–0.5 moderate | 0.05–0.2 slight | -0.05–0.05 irrelevant
| <-0.05 counter-acts | <-0.5 strongly counter-acts

Normalize to unit length. Output ONLY a JSON array of 8 floats.
```

---

## Rust API

```rust
use ga_semantics_core::prelude::*;

let mv = llm_encode(&[0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]);
let desc = multivector_describe(&mv);
let (rel, conf) = RelationType::from_pair(&a, &b);
let sim = dominant_similarity(&a, &b);
let d = analogy(&a, &b, &c);

#[cfg(feature = "store")]
{
    let mut store = ConceptStore::open("knowledge.json")?;
    store.store_llm_concept("Auth System", "Verifies identity", &coeff)?;
    let results = store.query_similar(&query_mv, 5);
}
```

```toml
[dependencies]
ga-semantics-core = { version = "0.1", features = ["store"] }
```

---

## MCP Tools

The server exposes **29 tools**:

| Category | Tools |
|----------|-------|
| **Encoding** | `llm_encode`, `text_to_multivector`, `word_to_multivector`, `multivector_describe`, `validate_encoding`, `encoding_benchmark` |
| **Semantics** | `semantic_similarity`, `semantic_difference`, `classify_relation`, `detect_contradiction`, `analogy`, `compose_relations`, `context_apply`, `semantic_explore` |
| **Bagua** | `classify_hexagram`, `bagua_dynamics`, `relation_type_info`, `wuxing_query` |
| **Store** | `store_open`, `store_concept`, `store_llm_concept`, `store_query_similar`, `store_get_concept`, `store_list_concepts`, `store_add_relation`, `store_export`, `store_close` |
| **Util** | `create_multivector`, `batch_process` |

---

## Documentation

| Document | Contents |
|----------|----------|
| **[System Guide](docs/SYSTEM_GUIDE.md)** | Full technical reference — GA math, Bagua, WuXing, API, benchmarks |
| **[Delivery Guide](docs/DELIVERY.md)** | Per-client configs, troubleshooting, distribution architecture |
| **[Encoding Skill](docs/skills/bagua-encoder/SKILL.md)** | LLM encoding protocol — 8 roles, rubric, examples |

---

## License

MIT OR Apache-2.0
