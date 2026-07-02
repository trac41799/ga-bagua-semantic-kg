# Contributing to GA-Bagua Semantic KG

Thanks for your interest. This project explores deterministic semantic
representation via Cl(3) Geometric Algebra mapped to the 8 Bagua trigrams.
Contributions in any form are welcome.

## Quick Setup

```bash
git clone https://github.com/trac41799/ga-bagua-semantic-kg
cd ga-bagua-semantic-kg
cargo build
cargo test
```

You need Rust 1.78+ (`rustup update`).

## What to Work On

### Good First Issues

Look for issues tagged `good first issue` — they are self-contained and come
with clear acceptance criteria:

- Add a new MCP tool
- Improve CLI output formatting
- Add tests for an uncovered edge case
- Write documentation for a module
- Add a benchmark for a new operation

### Areas That Need Help

| Area | Why |
|------|-----|
| **Encoding quality (SKILL.md v3)** | The #1 bottleneck. LLM encodings align to WuXing phases at only 15-19%. Better prompting protocols or multi-encoding strategies needed. |
| **Relation classification** | Current 45-52% accuracy ceiling is tied to encoding alignment. Any improvement to encoding ripples through the entire system. |
| **Real-world benchmark datasets** | We need independently-labeled concept pairs across diverse domains. |
| **Language bindings** | Python (PyO3), WASM, or other platform bridges. |
| **Documentation & tutorials** | Walkthroughs for specific use cases (agent memory, document alignment, argument analysis). |
| **Cross-model encoding consistency** | Test the SKILL.md protocol across Claude, GPT, Gemini, etc. and document variance. |

## Project Architecture

```
ga-bagua-semantic-kg/
├── ga-semantics-core/     # Core algebra, Bagua taxonomy, encoding, retrieval, storage
├── ga-semantics-mcp/      # MCP server (stdio + HTTP) — 29+ tools for AI agents
├── ga-semantics-cli/      # CLI tool with 12+ subcommands
├── ga-doc-intel/          # Document intelligence (alignment, coherence, fallacy, contract)
├── ga-cognitive/          # Cognitive systems (agent store, belief, compatibility, learning, goal)
├── docs/                  # System guide, delivery guide, benchmark reports, proposals
├── npm/                   # npm package wrapper (ga-semantics-mcp)
└── data/                  # Benchmark datasets, training pairs, FB15k subset
```

Key files:
- `ga-semantics-core/src/lib.rs` — public API surface
- `ga-semantics-core/src/multivector.rs` — Cl(3) Multivector type, geometric product
- `ga-semantics-core/src/bagua.rs` — Trigram, Hexagram, WuXing, hexagram_step
- `ga-semantics-core/src/relation_type.rs` — RelationType enum, from_pair_multi classifier
- `ga-semantics-core/src/index.rs` — WuXingIndex retrieval
- `ga-semantics-core/src/encoding.rs` — llm_encode, multivector_describe
- `docs/SYSTEM_GUIDE.md` — Complete reference (math, taxonomy, operations)

## Development Workflow

### Adding a New Feature

1. Open an issue describing the feature and the design approach
2. Write tests first (see `ga-semantics-core/tests/` for patterns)
3. Implement in the appropriate crate
4. Run `cargo test --all` and `cargo clippy --all -- -D warnings`
5. If it's an MCP tool, add it to `ga-semantics-mcp/src/main.rs`
6. If it's a CLI command, add it to `ga-semantics-cli/src/main.rs`
7. Open a PR with a clear description

### Commit Style

- `feat:` new feature
- `fix:` bug fix
- `docs:` documentation
- `test:` tests only
- `bench:` benchmarks
- `refactor:` no behavior change
- `chore:` CI, build, dependencies

### Before Submitting a PR

```bash
cargo test --all
cargo clippy --all -- -D warnings
cargo fmt --all -- --check
```

## Understanding the System

### The Core Idea

The system maps 8 Cl(3) basis blades to 8 Bagua trigrams, creating a
deterministic, interpretable 8-dimensional semantic representation. The LLM
encodes a concept into 8 coefficients once (~200 tokens). After that, all
semantic operations execute algebraically in ~500ns with zero API calls.

Read `docs/SYSTEM_GUIDE.md` for the full mathematical background.

### The Encoding Bottleneck

The critical path is the LLM encoding step. If the LLM produces coefficients
that don't align with the WuXing taxonomy, downstream accuracy suffers.
The current SKILL.md protocol achieves ~15-19% WuXing phase alignment.
This is the single most impactful area to improve.

Read `docs/engineering/strategy-to-excellence.md` for the full analysis.

## Getting Help

- Open a GitHub Discussion for questions
- Mention `@trac41799` in issues for maintainer attention
- Read `docs/SYSTEM_GUIDE.md` and `docs/DELIVERY.md` first

## License

By contributing, you agree that your contributions will be dual-licensed
under the MIT OR Apache-2.0 license, matching the project.

