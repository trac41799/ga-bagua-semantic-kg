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

**Paste this into any LLM (Claude, GPT, Copilot, Gemini, etc.) to install automatically:**

```
Install the GA-Bagua Semantic KG MCP server on my machine. This is a knowledge
graph tool that lets you encode concepts into 8-number semantic vectors and
reason about relationships algebraically.

Steps:
1. Detect my OS and architecture (Windows/macOS/Linux, x64/arm64)
2. Download the correct pre-built binary from:
   https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
3. Place it somewhere accessible (e.g., ~/.local/bin/ or %USERPROFILE%\tools\)
4. On macOS/Linux, chmod +x the binary
5. Configure my LLM client to use it as an MCP server:
   - For Claude Desktop: edit %APPDATA%\Claude\claude_desktop_config.json
   - For Cursor: edit ~/.cursor/mcp.json
   - For OpenCode: edit ~/.config/opencode/opencode.json
   - For Claude Code: edit ~/.claude/mcp.json
   Add to the mcpServers object:
     "ga-bagua-semantic-kg": { "command": "<full path to binary>" }
6. Download the encoding skill from:
   https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
   Save it to the appropriate skills directory for my client.
7. Verify: ask me to encode a "Rate Limiter" concept, then verify the dominant
   role is "constraining" with confidence > 0.5.

After installation, read docs/SYSTEM_GUIDE.md from the repo for full API details.
```

---

## Manual Installation

### Option 1: Pre-built binary

Download from [GitHub Releases](https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest):

| Platform | Binary |
|----------|--------|
| Windows x64 | `ga-semantics-mcp-x86_64-pc-windows-msvc.exe` |
| macOS Intel | `ga-semantics-mcp-x86_64-apple-darwin` |
| macOS Apple Silicon | `ga-semantics-mcp-aarch64-apple-darwin` |
| Linux x64 | `ga-semantics-mcp-x86_64-unknown-linux-gnu` |
| Linux ARM64 | `ga-semantics-mcp-aarch64-unknown-linux-gnu` |

Extract (`.zip` on Windows, `.tar.gz` on others). Each archive contains the binary
plus `bagua-encoder-skill.md` — the LLM encoding guide.

### Option 2: npm

```bash
npm install -g ga-semantics-mcp
# or run ad-hoc:
npx ga-semantics-mcp
```

### Option 3: Cargo (Rust)

```bash
cargo install ga-semantics-mcp
```

Requires Rust 1.78+.

### Option 4: Build from source

```bash
git clone https://github.com/trac41799/ga-bagua-semantic-kg.git
cd ga-bagua-semantic-kg
cargo build --release -p ga-semantics-mcp
```

Pure Rust — no C compiler needed.

---

## Client Configuration

After installing the binary, configure your LLM client's MCP settings:

<details>
<summary><b>Claude Desktop</b></summary>

`%APPDATA%\Claude\claude_desktop_config.json` (Windows) |
`~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) |
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
</details>

<details>
<summary><b>OpenCode</b></summary>

`~/.config/opencode/opencode.json` or `.opencode/opencode.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```

Skill: copy `docs/skills/bagua-encoder/SKILL.md` to `.opencode/skills/bagua-encoder/SKILL.md`
</details>

<details>
<summary><b>Cursor</b></summary>

`~/.cursor/mcp.json` or `.cursor/mcp.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```
</details>

<details>
<summary><b>Claude Code (CLI)</b></summary>

`.claude/mcp.json` or `~/.claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```
</details>

<details>
<summary><b>Continue.dev</b></summary>

`~/.continue/config.json`

```json
{
  "experimental": {
    "modelContextProtocolServers": [
      { "name": "ga-bagua-semantic-kg", "command": "ga-semantics-mcp" }
    ]
  }
}
```
</details>

<details>
<summary><b>Windsurf</b></summary>

`~/.windsurf/mcp_config.json`

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```
</details>

<details>
<summary><b>Cline (VS Code)</b></summary>

Use `Ctrl+Shift+P` → "Cline: Open MCP Config"

```json
{
  "mcpServers": {
    "ga-bagua-semantic-kg": {
      "command": "ga-semantics-mcp"
    }
  }
}
```
</details>

After configuring, restart your client. You should see 29 tools available (hammer icon
in Claude, tool list in Cursor/OpenCode).

---

## Encoding Skill

The LLM needs the encoding guide to produce correct 8-coefficient vectors.
Save [SKILL.md](docs/skills/bagua-encoder/SKILL.md) to your client's skill directory,
or paste this crib sheet into your LLM instructions:

```
Encode concepts using 8 roles in order: [receptive, causal, transmissive,
constraining, clarifying, influential, balancing, generative].

Scale: >0.5 strong | 0.2–0.5 moderate | 0.05–0.2 slight | -0.05–0.05 irrelevant
| <-0.05 counter-acts | <-0.5 strongly counter-acts.

Normalize to unit length. Output ONLY a JSON array of 8 floats.
```

---

## Rust API

```rust
use ga_semantics_core::prelude::*;

// Encode from LLM-produced coefficients
let mv = llm_encode(&[0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]);

// Inspect: "moderately constraining; slightly receptive..."
println!("{}", multivector_describe(&mv));

// Classify relationship (deterministic, 100% accuracy)
let (rel, conf) = RelationType::from_pair(&a, &b);
// → (Receptive, 0.60): "essentially the same thing"

// Role-weighted similarity for retrieval
let sim = dominant_similarity(&a, &b);  // [-1, 1]

// Analogies: A:B :: C:?
let d = analogy(&a, &b, &c);  // Option<Multivector>

// JSON file-backed concept store
#[cfg(feature = "store")]
{
    let mut store = ConceptStore::open("knowledge.json")?;
    store.store_llm_concept("Auth System", "Verifies identity", &coeff)?;
    let results = store.query_similar(&query_mv, 5);
}
```

Add to `Cargo.toml`:
```toml
[dependencies]
ga-semantics-core = { version = "0.1", features = ["store"] }
```

---

## MCP Tools

The server exposes **29 tools** for LLM agents:

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
| **[System Guide](docs/SYSTEM_GUIDE.md)** | Full technical reference: GA math, Bagua taxonomy, WuXing cycles, API surface, benchmarks |
| **[Delivery Guide](docs/DELIVERY.md)** | Per-client installation, troubleshooting, distribution architecture |
| **[Encoding Skill](docs/skills/bagua-encoder/SKILL.md)** | The LLM encoding protocol — 8 roles, rubric, examples |

---

## Architecture

```
┌──────────────────────────────────────────────────────┐
│ Layer 4: MCP server / CLI / Python bindings          │
│ Layer 3: semantics.rs — similarity, analogy, retrieval│
│ Layer 2: Cl(3) multivector engine, geometric product  │
│ Layer 1: encoding.rs — llm_encode, role descriptions  │
│ Layer 0: bagua.rs — trigrams, WuXing, 64 hexagrams   │
└──────────────────────────────────────────────────────┘
```

**Why 8 dimensions instead of 384–4096?**
Each dimension has a fixed human-readable label (e.g., index 3 = "constraining").
The 8 numbers are self-documenting — you can read a concept's encoding and
understand what it is without a lookup table.

**Why WuXing instead of A⁻¹ * B?**
A⁻¹ * B measures geometric similarity, not functional relationship.
WuXing cycles (Wood→Fire→Earth→Metal→Water) are deterministic —
Metal always controls Wood, no training needed, zero error.

## License

MIT OR Apache-2.0
