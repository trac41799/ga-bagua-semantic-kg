<div align="center">

```
                    ☰                        Heaven / Generative
                ☱       ☴                  Lake / Balancing     Wind / Influential
                    ☲                        Fire / Clarifying
                ☳       ☶                  Thunder / Causal     Mountain / Constraining
                    ☵                        Water / Transmissive
                    ☷                        Earth / Receptive
```

# GA-Bagua Semantic KG

**LLM semantic memory — 8 dimensions, 64 hexagram states, zero training.**

[![Crates.io](https://img.shields.io/crates/v/ga-semantics-core?label=core)](https://crates.io/crates/ga-semantics-core)
[![Crates.io](https://img.shields.io/crates/v/ga-semantics-mcp?label=mcp)](https://crates.io/crates/ga-semantics-mcp)
[![Crates.io](https://img.shields.io/crates/v/ga-semantics-cli?label=cli)](https://crates.io/crates/ga-semantics-cli)
[![npm](https://img.shields.io/npm/v/ga-semantics-mcp?color=red)](https://www.npmjs.com/package/ga-semantics-mcp)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

</div>

---

Every concept becomes **8 numbers**. Every relationship is a **cycle step**. No training.
No vector database. No repeated LLM calls. Reasoning completes in **34 nanoseconds** to
**320 microseconds** with zero accumulated error.

```
 Concept name
      │
      ▼
 LLM (reads SKILL.md, ~200 tokens, one-shot)
      │
      ▼
 [☷ 0.04, ☳ -0.09, ☵ -0.51, ☶ 0.68, ☲ 0.21, ☴ -0.26, ☱ 0.17, ☰ -0.34]
      │
      ├── similar?   →  dominant_similarity()
      ├── related?   →  classify_hexagram()       (WuXing cycle lookup)
      └── analogy?   →  analogy()                  (cycle direction prediction)
```

| 木 Fire | 火 Earth | 土 Metal | 金 Water | 水 Wood |
|:---:|:---:|:---:|:---:|:---:|
| Relation 100% | Role 100% | P@K 73.3% | MRR 0.878 | Analogy 80% |

---

## Install

```bash
# Option 1 — npm (recommended)
npm install -g ga-semantics-mcp     # MCP server + CLI + skill

# Option 2 — Cargo
cargo install ga-semantics-mcp      # MCP server
cargo install ga-semantics-cli      # CLI tool

# Option 3 — download binary
# Pick your platform at: https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
```

Each archive contains `ga-semantics-mcp` (server), `ga-semantics` (CLI), and `bagua-encoder-skill.md`.

---

## LLM Self-Install

<details>
<summary><b>☰ Claude Desktop</b></summary>

```
Install the GA-Bagua Semantic KG MCP server + CLI for Claude Desktop.

Step 1 — Detect my OS, download:
  https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
  (pick the .zip for Windows or .tar.gz for macOS/Linux)

Step 2 — Extract to ~/.local/bin/ (macOS/Linux) or %USERPROFILE%\.local\bin\ (Windows).
  On macOS/Linux: chmod +x both extracted binaries.

Step 3 — Configure Claude Desktop MCP. Read or create:
  Windows: %APPDATA%\Claude\claude_desktop_config.json
  macOS:   ~/Library/Application Support/Claude/claude_desktop_config.json
  Linux:   ~/.config/Claude/claude_desktop_config.json
  Add: { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<path>/ga-semantics-mcp" } } }

Step 4 — Install skill: download
  https://raw.githubusercontent.com/trac41799/ga-bagua-semantic-kg/main/docs/skills/bagua-encoder/SKILL.md
  Save to %APPDATA%\Claude\skills\bagua-encoder\SKILL.md (or ~/.config/Claude/skills/...)

Step 5 — Restart Claude Desktop. Verify: "Encode Rate Limiter using llm_encode
  with coefficients [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]"
  Expected: dominant_role = "constraining".
```
</details>

<details>
<summary><b>☲ OpenCode</b></summary>

```
Install GA-Bagua Semantic KG for OpenCode CLI. Download from releases, extract both
binaries. Configure ~/.config/opencode/opencode.json:
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<path>/ga-semantics-mcp" } } }
Install skill to ~/.config/opencode/skills/bagua-encoder/SKILL.md.
Restart and verify with llm_encode.
```
</details>

<details>
<summary><b>☵ Cursor</b></summary>

```
Install GA-Bagua Semantic KG for Cursor. Download from releases, extract both binaries.
Configure ~/.cursor/mcp.json:
  { "mcpServers": { "ga-bagua-semantic-kg": { "command": "<path>/ga-semantics-mcp" } } }
Install skill to ~/.cursor/skills/bagua-encoder/SKILL.md.
In Composer agent, verify with llm_encode.
```
</details>

<details>
<summary><b>☳ Claude Code CLI</b></summary>

```
Install GA-Bagua Semantic KG for Claude Code. Download from releases, extract both
binaries. Configure ~/.claude/mcp.json. Save skill to ~/.claude/skills/bagua-encoder/.
Verify: echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | <path>/ga-semantics-mcp
```
</details>

<details>
<summary><b>☴ Continue.dev / ☱ Cline / ☰ Windsurf / other</b></summary>

```
Download from https://github.com/trac41799/ga-bagua-semantic-kg/releases/latest
Extract, place binaries on PATH, configure your client's MCP settings.
Install skill from docs/skills/bagua-encoder/SKILL.md
See docs/DELIVERY.md for detailed per-client instructions.
```
</details>

---

## CLI Usage

```bash
# Encode a concept
ga-semantics encode 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics encode -j 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20 --json

# Classify relationship
ga-semantics classify \
  "[0.04,-0.09,-0.51,0.68,0.21,-0.26,0.17,-0.34]" \
  "[0.25,0.15,-0.10,0.55,0.40,0.05,0.30,0.20]"

# Compute similarity
ga-semantics sim "[0.15,0.25,0.81,...]" "[0.30,0.10,0.60,...]"

# Solve analogy
ga-semantics analogy  "[A]" "[B]" "[C]"

# Explore Bagua
ga-semantics trigram qian --transforms
ga-semantics hexagram "[A]" "[B]"
ga-semantics wuxing water --cycle controlling

# Knowledge graph
ga-semantics store add "Auth System" 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20
ga-semantics store query "[0.05,-0.05,-0.45,0.70,0.15,-0.20,0.10,-0.30]"
ga-semantics store list
ga-semantics store export

# Benchmarks
ga-semantics bench timing
ga-semantics bench semantic
```

`--json` for machine output, `--csv` for spreadsheet, `--quiet` for values only.

---

## Encoding Crib Sheet

```
8 roles in order:
[receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]

Scale:  >0.5 strong  |  0.2–0.5 moderate  |  0.05–0.2 slight
       -0.05–0.05 irrelevant  |  <-0.05 counter-acts  |  <-0.5 strongly counter-acts

Normalize to unit length. Output ONLY a JSON array of 8 floats.
```

See **[SKILL.md](docs/skills/bagua-encoder/SKILL.md)** for the full encoding protocol.

---

## Rust API

```rust
use ga_semantics_core::prelude::*;

let mv = llm_encode(&[0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]);
let desc = multivector_describe(&mv);
let (rel, conf) = RelationType::from_pair(&a, &b);
let sim = dominant_similarity(&a, &b);
let d = analogy(&a, &b, &c);
```

```toml
[dependencies]
ga-semantics-core = { version = "0.1", features = ["store"] }
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Layer 4 ── MCP server (29 tools) + CLI + Python        │
│  Layer 3 ── semantics.rs — similarity, analogy, store   │
│  Layer 2 ── Cl(3) multivector engine — geometric product│
│  Layer 1 ── encoding.rs — llm_encode, role descriptions │
│  Layer 0 ── bagua.rs — trigrams, WuXing, 64 hexagrams   │
└─────────────────────────────────────────────────────────┘
```

**8 blades × 8 roles × 5 phases × 64 hexagrams** — a complete closed-form
semantic algebra with deterministic relationship classification via WuXing
generate/control cycles, not error-prone algebraic transformations.

---

## Documentation

| Document | Purpose |
|----------|---------|
| **[System Guide](docs/SYSTEM_GUIDE.md)** | Full reference: math, taxonomy, operations, API, benchmarks |
| **[Delivery Guide](docs/DELIVERY.md)** | Per-client configs, troubleshooting, distribution |
| **[Encoding Skill](docs/skills/bagua-encoder/SKILL.md)** | LLM protocol — 8 roles, rubric, examples |
| **[Strategy](docs/engineering/strategy-to-excellence.md)** | 7-layer improvement roadmap |
| **[Benchmarks](docs/engineering/semantic-accuracy-benchmark.md)** | Honest accuracy report |

## License

MIT OR Apache-2.0
