<div align="center">

[**English**](README.md) · [**中文**](README.zh.md) · [**Tiếng Việt**](README.vi.md) · [**한국어**](README.ko.md) · [**日本語**](README.ja.md)

</div>

---

<p align="center">
  <img src="docs/img/logo-bw.png" alt="GA-Bagua Semantic KG" width="600">
</p>

<p align="center">
  <strong>LLM semantic memory — 8 bytes × 8 roles = 64 bytes per concept.<br>
  Deterministic, zero-training, interpretable.</strong><br>
  <a href="https://crates.io/crates/ga-semantics-core"><img src="https://img.shields.io/crates/v/ga-semantics-core?label=core" alt="Crates.io"></a>
  <a href="https://crates.io/crates/ga-semantics-mcp"><img src="https://img.shields.io/crates/v/ga-semantics-mcp?label=mcp" alt="Crates.io"></a>
  <a href="https://crates.io/crates/ga-semantics-cli"><img src="https://img.shields.io/crates/v/ga-semantics-cli?label=cli" alt="Crates.io"></a>
  <a href="https://www.npmjs.com/package/ga-semantics-mcp"><img src="https://img.shields.io/npm/v/ga-semantics-mcp?color=red" alt="npm"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License"></a>
   <a href="#tests"><img src="https://img.shields.io/badge/tests-228%20passing-brightgreen" alt="Tests"></a>
</p>

---

**GA-Bagua is a compact, interpretable semantic index for LLM agents.** It encodes
concepts into 64-byte vectors using Cl(3) Geometric Algebra mapped to the 8 Bagua
trigrams of the I-Ching. Once encoded, all semantic operations execute algebraically
in nanoseconds with zero tokens — forever.

**It does NOT replace LLM reasoning.** The LLM still reasons, verifies, and explains.
GA-Bagua eliminates redundant context reading by surfacing ranked concept candidates
for the LLM to evaluate at a fraction of the token cost.

```
Agent query: "Which concepts constrain throughput?"

  WITHOUT GA-Bagua                    WITH GA-Bagua
  ─────────────────                   ─────────────
  LLM reads ALL descriptions          GA-Bagua returns top-10 candidates (0 tokens, 500ns)
  → 100 concepts × 40 tok = 4K tok    LLM verifies ONLY those 10 (10 × 15 tok = 150 tok)
  → repeats per query                 → encoding cost amortized after 5 queries
  → 50 queries = 200K tokens          → 50 queries = 20K + 7.5K = 27.5K tokens
                                      → 7.3x token savings
```

## What It Can Do

| Capability | How | Performance |
|-----------|-----|:----------:|
| **Same-role retrieval** | Find concepts with the same Bagua role as the query | 42% P@1 (same domain), 100% R@10 |
| **Complementary discovery** | Find the antithesis of any concept (unique to GA-Bagua) | Exact trigram-level matching |
| **WuXing path traversal** | Multi-hop exploration along generate/control chains | 500ns per hop |
| **Multi-hop composition** | Compose 100 reasoning steps via rotor algebra | 200µs, zero drift |
| **Encoding stability** | Same concept → same label every time | 99.8% under ±5% noise |
| **Concept evolution** | Predict what a concept becomes when one aspect changes | Deterministic moving-line transform |
| **Relation classification** | Directional hints for LLM verification | 45–52% test accuracy |
| **Storage density** | 64 bytes per concept. 1M concepts = 64 MB | 48x denser than BERT |
| **Zero query cost** | All operations are pure algebra after encoding | 0 tokens, 500ns per op |
| **Sharpness gate** | Random noise gets 0.0 confidence | 93.5% of random pairs gated |
| **Document alignment** | Cross-document claim matching with relation classification | Precision@5 ≥ 70% |
| **Policy coherence** | Detect contradictory clauses within/across documents | F1 ≥ 0.67 |
| **Argument analysis** | Detect circular, non-sequitur, and contradictory arguments | F1 ≥ 0.89 |
| **Team compatibility** | WuXing-based personality matching and team formation | Compatible > identical pairs |
| **Learning paths** | Generate WuXing-ordered curriculum sequences | Correct phase ordering |
| **Creative ideation** | 64-hexagram perspective exploration via rotors | 3+ trigram coverage |

## How It Works

### Encoding (LLM, one-time)
```
Concept description → SKILL.md protocol → LLM outputs 8 coefficients → llm_encode() → 64-byte Multivector
Token cost: ~200 tokens per concept (one-time)
```

### Retrieval (algebraic, zero-token)
```
"Find constraining concepts" → WuXingIndex scans Earth-phase bucket → ranks by dominant_similarity → returns top-K
Latency: 500ns per query. Tokens: 0.
```

### Pipeline Pattern (LLM + GA-Bagua)
```
1. GA-Bagua surfaces top-K candidates (0 tokens)
2. LLM verifies each candidate against original descriptions (15 tokens each)
3. LLM reasons about results, presents findings (50 tokens)
Total per query: ~150 tokens vs ~4,000 tokens reading all descriptions
```

## Quick Start

```bash
npm install -g ga-semantics-mcp     # MCP server
cargo install ga-semantics-cli      # CLI tool
```

```bash
# Encode a concept
ga-semantics encode 0.25 0.15 -0.10 0.55 0.40 0.05 0.30 0.20

# Classify relationship (multi-hypothesis)
ga-semantics classify "[0.04,-0.09,-0.51,0.68,...]" "[0.25,0.15,-0.10,0.55,...]"

# Find similar concepts
ga-semantics store query "[0.05,-0.05,-0.45,0.70,...]"

# Explore WuXing cycles
ga-semantics wuxing water --cycle generating

# Concept evolution
ga-semantics bagua-dynamics "[0.15,0.05,0.10,0.30,0.85,...]"
```

## Rust API

```rust
use ga_semantics_core::prelude::*;

let mv = llm_encode(&[0.25, 0.15, -0.10, 0.55, 0.40, 0.05, 0.30, 0.20]);
let desc = multivector_describe(&mv);
let (rel, conf) = RelationType::from_pair_multi(&a, &b);
let sim = fingerprint_similarity(&a, &b);
let spectrum = relationship_spectrum(&a, &b);
let evolved = evolve_concept(&mv, 0);

// WuXingIndex with phase-bucketed retrieval
let index = WuXingIndex::new(concepts);
let peers = index.query_same_role(&query, 10, false);
let opposites = index.query_complementary(&query, 5);
let chain = index.query_path(&query, &["generate", "control"], 5);
```

```toml
[dependencies]
ga-semantics-core = { version = "0.1", features = ["store"] }
```

## The 8 Bagua Roles

| Role | Trigram | WuXing | Blade | Description |
|------|---------|--------|-------|-------------|
| generative | Qian ☰ | Metal | e123 | Creates, initiates new patterns |
| receptive | Kun ☷ | Earth | scalar | Accepts, follows, grounds |
| causal | Zhen ☳ | Wood | e1 | Triggers, initiates chain reactions |
| transmissive | Kan ☵ | Water | e2 | Channels, flows, transmits |
| constraining | Gen ☶ | Earth | e3 | Limits, bounds, restricts |
| influential | Xun ☴ | Wood | e23 | Pervades, gradually affects |
| clarifying | Li ☲ | Fire | e12 | Reveals, illuminates |
| balancing | Dui ☱ | Metal | e31 | Mirrors, equilibrates, reflects |

## Current Benchmarks

| Metric | Value | Notes |
|--------|:-----:|-------|
| Same-role P@1 (same domain) | 42% | dominant_similarity; bottleneck is encoding distinctiveness |
| Same-role R@10 (same domain) | 100% | All same-role peers surface in top-10 |
| Relation classification (test) | 45–52% | from_pair_multi on held-out pairs |
| Encoding stability | 99.8% | Dominant role preserved under ±5% noise |
| Multi-hop (100-hop) | 200µs, zero drift | Unique to GA-Bagua |
| Token savings (200 queries) | 219x | $101.00 → $0.46 per session |
| Random pairs gated | 93.5% → 0.0 conf | Sharpness gate at 0.25 |
| All 8 labels predicted | Yes | from_pair_multi scores all simultaneously |
| Storage | 64 bytes/concept | 1M concepts = 64 MB |
| Query latency | 500ns | Algebraic, no API, no GPU |

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│  LLM Agent (Reasoning Engine)                             │
│  Reads SKILL.md → encodes concepts → queries GA-Bagua    │
│  Verifies top-K results → reasons → explains             │
└────────────────────┬─────────────────────────────────────┘
                     │ MCP protocol
┌────────────────────▼─────────────────────────────────────┐
│  GA-Bagua MCP Server (33 tools)                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ ga-semantics-core                                 │    │
│  │ ┌─────────┐ ┌─────────┐ ┌───────────────┐       │    │
│  │ │Cl(3) GA │ │ Bagua   │ │ relation_type  │       │    │
│  │ │8 blades │ │ 8 tri.  │ │ from_pair      │       │    │
│  │ │rotors   │ │ 5 WuXing│ │ sharpness gate │       │    │
│  │ │geo prod │ │ 64 hexa │ │ FeatureWeights │       │    │
│  │ └─────────┘ └─────────┘ └───────────────┘       │    │
│  │ ┌──────────┐ ┌──────────┐ ┌────────────────┐   │    │
│  │ │index     │ │semantics │ │diagnostic       │   │    │
│  │ │WuXingIdx │ │spectrum  │ │encoding diag    │   │    │
│  │ │complement│ │evolve    │ │corrective prompt│   │    │
│  │ │path      │ │ideation  │ │                  │   │    │
│  │ └──────────┘ └──────────┘ └────────────────┘   │    │
│  └──────────────────────────────────────────────────┘    │
│  ┌──────────────┐ ┌──────────────┐                      │
│  │ ga-doc-intel  │ │ ga-cognitive │                      │
│  │ alignment     │ │ agent store  │                      │
│  │ synthesis     │ │ compatibility│                      │
│  │ coherence     │ │ learning path│                      │
│  │ fallacy       │ │ goal tree    │                      │
│  │ contract audit│ │ belief track │                      │
│  └──────────────┘ └──────────────┘                      │
└──────────────────────────────────────────────────────────┘
```

## Documentation

| Document | Purpose |
|----------|---------|
| **[System Guide](docs/SYSTEM_GUIDE.md)** | Full reference: math, taxonomy, operations, API, benchmarks |
| **[LLM Pipeline Pattern](docs/engineering/llm-pipeline-pattern.md)** | How GA-Bagua integrates with LLM agents |
| **[Complete Benchmark Report](docs/engineering/complete-benchmark-report.md)** | Honest, comprehensive benchmark results |
| **[Benchmark Results](docs/engineering/development/2026-06-08-encoding-quality-classification/BENCHMARK_RESULTS.md)** | Detailed classification accuracy report |
| **[Encoding Quality Handoff](docs/engineering/handoff-encoding-quality.md)** | Encoding quality workstream plan |
| **[Encoding Skill](docs/skills/bagua-encoder/SKILL.md)** | LLM protocol — 8 roles, rubric, examples |
| **[Delivery Guide](docs/DELIVERY.md)** | Per-client configs, troubleshooting, distribution |
| **[Application Benchmarks](docs/engineering/benchmarks.md)** | All 11 expansion benchmarks (B1-B11) |
| **[Benchmark Realism Assessment](docs/engineering/development/2026-06-08-app-expansion/BENCHMARK-REALISM-ASSESSMENT.md)** | Critical review of benchmark validity |
| **[Expansion Plan](docs/engineering/development/2026-06-08-app-expansion/PLAN.md)** | Full expansion implementation plan |
| **[Benefit Analysis](docs/engineering/development/2026-06-08-app-expansion/BENEFIT-ANALYSIS.md)** | Benefit analysis and benchmark specs |

## License

MIT OR Apache-2.0

---

*228 unit tests + 31 benchmark suites passing. Run: `cargo test`*
