# GA-Bagua: A Deterministic 64-Byte Semantic Index for LLM Agents

**trac41799 · 2026-07-02**

---

## The Problem

LLM agents are bad at reasoning across many concepts. When an agent needs to
answer "which of these 100 concepts constrains throughput?", it has to read
all 100 concept descriptions. Each query costs ~1,000 tokens. Across 200
queries, that's 200,000 tokens — roughly $101 per session. You can't scale
that.

Embedding vectors (BERT, OpenAI) are the standard solution: precompute
vectors, then cosine-similarity search. But they have three problems:
1. **You need training data** to get good embeddings for your domain
2. **They're opaque** — you get a number, not an explanation of *why* two
   concepts are related
3. **They can't compose** — you can't answer "A relates to B, B relates to C,
   what's A to C?" algebraically

Vector databases solve the search problem but inherit the opacity.

---

## What GA-Bagua Does Differently

GA-Bagua encodes any concept into **8 numbers** (64 bytes total). Once
encoded, all semantic operations — similarity, classification, relation
composition, analogy — execute **purely algebraically in ~500ns with zero
API calls**.

The trick is the mathematics: **Cl(3) Geometric Algebra**, a 3D Clifford
algebra with 8 basis blades. Those 8 blades map perfectly to the 8 trigrams
of the I-Ching Bagua system:

| Blade | Trigram | Semantic Role | What it means |
|-------|---------|---------------|---------------|
| e₁₂₃ | ☰ Qián | Generative | Creative origin, innovation |
| 1     | ☷ Kūn  | Receptive | Ground state, convention |
| e₁    | ☳ Zhèn | Causal | Initiates, triggers |
| e₂    | ☵ Kǎn  | Transmissive | Flows, pipes, streams |
| e₃    | ☶ Gèn  | Constraining | Bounds, limits, guards |
| e₁₂   | ☴ Xùn  | Influential | Penetrates, spreads |
| e₂₃   | ☲ Lí   | Clarifying | Illuminates, reveals |
| e₃₁   | ☱ Duì  | Balancing | Reflects, mirrors |

These 8 roles are **interpretable** — you know *why* two concepts relate, not
just *that* they relate.

---

## How It Works (in 3 Steps)

### 1. Encode (LLM does this once)

The LLM reads the concept description and maps it to 8 coefficients using a
structured protocol (SKILL.md). Example:

```
"Rate Limiter — a gate that restricts throughput"
→ [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]
```

Cost: ~200 tokens. One time. The dominant role here is **Transmissive** (e₂,
coefficient 0.68) — the rate limiter is primarily a flow-control mechanism.

### 2. Store (64 bytes each)

1 million concepts = 64 MB. That's 48x denser than BERT embeddings.

### 3. Query (0 tokens, 500ns)

All semantic operations execute algebraically:
- `semantic_similarity(A, B)` → [-1, 1]
- `classify_relation(A, B)` → one of 8 role labels
- `analogy(a, b, c)` → "a is to b as c is to ?"
- `compose_relations(r1, r2)` → compound relationship

The LLM only verifies the top-K results surfaced by GA-Bagua (~15 tokens each).

---

## Architecture

```
┌─────────────────────────────────────┐
│  LLM Agent (Claude, Cursor, etc.)  │
│  Encodes concepts once (~200 tok)  │
│  Verifies top-K results (~15 tok)  │
└──────────────┬──────────────────────┘
               │ MCP Protocol (stdio / HTTP)
┌──────────────▼──────────────────────┐
│  GA-Bagua MCP Server (29 tools)    │
│  ┌─────────────────────────────┐   │
│  │  Cl(3) Geometric Algebra     │   │
│  │  8 basis blades, geo prod,  │   │
│  │  rotors, inverse, compose   │   │
│  ├─────────────────────────────┤   │
│  │  Bagua Taxonomy              │   │
│  │  8 trigrams → 8 roles       │   │
│  │  WuXing 5-phase cycle       │   │
│  ├─────────────────────────────┤   │
│  │  WuXingIndex                 │   │
│  │  Phase-bucketed retrieval   │   │
│  │  500ns per query            │   │
│  └─────────────────────────────┘   │
└────────────────────────────────────┘
```

---

## Honest Benchmarks (Not Cherry-Picked)

I'm going to be upfront about what works and what doesn't:

### What Works

| Metric | Result |
|--------|--------|
| Multi-hop stability (100 hops) | Zero drift |
| Token savings (200 queries) | 219x ($101 → $0.46) |
| Storage density | 64 bytes/concept (48x denser than BERT) |
| Query latency | ~500ns per operation |
| Encoding stability | 99.8% (dominant role preserved ±5% noise) |
| Noise rejection | 93.5% of random pairs gated to 0.0 confidence |
| Same-role Recall@10 (same domain) | 100% |

### What Needs Work

| Metric | Current | Target |
|--------|:------:|:------:|
| Same-role Precision@1 | 42% | 70% |
| Relation classification test accuracy | 45-52% | 65% |
| WuXing encoding alignment | 15-19% | >50% |
| Document alignment Precision@5 | ~70% | 85% |

The fundamental bottleneck is **LLM encoding quality**. The mathematical
framework is sound, but the LLM's ability to produce coefficients that align
with the WuXing 5-phase taxonomy is limited. When the protocol works,
classification reaches 80-87% (trained weights). The gap is in the
encoding step, not the algebra.

### What Was Tried and Rejected

- **A*B geometric product as classifier**: 5.7% accuracy — worse than random (12.5%)
- **Hash-based encoding**: 0% semantic accuracy — maps word identity, not meaning
- **Single-path classifier**: Only 4 of 8 labels reachable
- **Naive refinement on all data**: 56% includes overfitting; proper CV → 17.5%

---

## Getting Started

```bash
# Install the MCP server
npm install -g ga-semantics-mcp

# Or via cargo
cargo install ga-semantics-cli

# Encode a concept
ga-semantics-cli encode 0.04 -0.09 -0.51 0.68 0.21 -0.26 0.17 -0.34

# Start the HTTP server
npx ga-semantics-mcp --http --port 3100
```

**Try the interactive demo:**
[https://trac41799.github.io/ga-bagua-semantic-kg/demo/](https://trac41799.github.io/ga-bagua-semantic-kg/demo/)

MCP clients that work out of the box: Claude Desktop, Cursor, OpenCode,
Claude Code CLI, Continue.dev, Cline, Windsurf.

---

## Is This Ready for Production?

**It depends on your use case.**

- **If you need a fast, interpretable way to classify concepts into 8
  semantic roles and browse relationships**: Yes.
- **If you need >90% relation classification accuracy on arbitrary domains**:
  Not yet — the encoding bottleneck is real.
- **If you need a drop-in replacement for vector DB retrieval**: No — P@1 is
  42%, not competitive with cosine similarity on learned embeddings.
- **If you need composable, reversible, interpretable algebraic relationships**:
  Yes — nothing else does what GA-Bagua does.

The project is honest about its limitations. The math is novel. The
engineering is solid (228 tests, 31 benchmark suites, CI/CD, 5-platform
distribution). The encoding quality problem is documented and measurable.
If that's interesting to you, I'd love your help.

---

## How to Contribute

The #1 thing the project needs is **better LLM encoding**. If you have ideas
about prompting protocols that produce consistent, WuXing-aligned coefficients
across models, that's the highest-leverage contribution possible.

Other areas:
- Real-world benchmark datasets with independently-labeled pairs
- Cross-model encoding consistency testing
- Python/WASM bindings
- Tutorials and integration examples

[GitHub](https://github.com/trac41799/ga-bagua-semantic-kg) ·
[System Guide](https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/docs/SYSTEM_GUIDE.md) ·
[Contributing](https://github.com/trac41799/ga-bagua-semantic-kg/blob/main/CONTRIBUTING.md) ·
[Demo](https://trac41799.github.io/ga-bagua-semantic-kg/demo/)

---

*Published under MIT OR Apache-2.0. Built with Rust, love, and the I-Ching.*
