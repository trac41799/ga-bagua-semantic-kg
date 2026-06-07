# GA-Bagua Semantic KG — Strategy to Excellence

**Date:** 2026-06-07
**Status:** Living document
**Audience:** Engineers, product owners, AI integration architects

---

## 1. Where We Are

### 1.1 What Works

| Layer | Status | Detail |
|-------|--------|--------|
| Cl(3) Geometric Algebra | Complete | 8 basis blades, geometric product via Cayley table, inner/wedge product, grade projection, reverse, inverse, dualize |
| Rotor algebra | Complete | Construction from angle+plane, sandwich product application, composition, inverse, unit norm verification |
| Bagua taxonomy | Complete | 8 trigrams ↔ 8 blades bidirectional, 64 hexagrams with names, WuXing 5-phase generating/controlling cycles, trigram line-change transforms |
| RelationType taxonomy | Complete | 8 semantic role labels (generative, receptive, causal, transmissive, constraining, influential, clarifying, balancing) with descriptions and Bagua/WuXing mappings |
| Semantic operations | Complete | `semantic_similarity()`, `semantic_difference()`, `semantic_relation()`, `relation_strength()`, `is_contradictory()`, `analogy()`, `compose_relations()`, `compose_chain()`, `inverse_relation()`, `Context` transformations |
| Serialization | Complete | serde for Multivector, Blade; JSON roundtrip verified |
| Test suite | Complete | 98 tests (88 unit + 10 integration), all passing |
| MCP server | Operational | 14 tools with proper JSON Schemas; `semantic_explore` for one-shot concept exploration |
| CLI | Operational | 12 subcommands; JSON/CSV/human-readable output |

### 1.2 What's Broken or Absent

| Gap | Severity | Root Cause |
|-----|----------|-----------|
| Hash-based encoding collapses single words to pure scalar | High | `text_to_multivector` distributes word hashes by position index; single words all map to slot 0 |
| Same encoding for different concepts | High | Hash-based encoding reflects lexical identity, not semantic relatedness. "fire" and "water" both encode to `[1, 0, 0, 0, 0, 0, 0, 0]` |
| No validation against human-labeled relationships | Critical | Zero benchmarks against KG datasets (WN18RR, FB15k-237); no human-judgment correlation |
| CLI `--help` is broken (clap color feature disabled) | Low | `clap` defaults include `color`→`windows-sys`→`dlltool.exe`→build failure on GNU Windows. Workaround: disable `color` default feature |
| No Python bindings | Medium | PyO3 not yet hooked up |
| No retrieval/indexing layer | High | Can encode but can't retrieve; no top-k similarity search |
| Bagua dynamics unused in encoding | Critical | Trigram transforms, hexagram stacking, WuXing cycles, mutual resonance — all implemented but never called during concept encoding |
| No temporal/moving-line modeling | Medium | The Bagua's predictive capability (which line changes next) has no representation |
| No visualization | Low | Users can't see the 3D geometry or role-space radar charts |
| Benchmarks directory is a stub | High | `benches/` files exist but criterion not wired in; no benchmark can run |

### 1.3 The Core Tension

The project simultaneously stakes two claims:

1. **Claim A (mathematical):** Cl(3) algebra is a correct and complete algebraic encoding of the 8 Bagua trigrams and their composition rules. This claim is **proven** — the Cayley table, rotor algebra, trigram ↔ blade isomorphism, WuXing cycles are all verified.

2. **Claim B (semantic):** Mapping real-world concepts onto these 8 basis blades produces meaningful, interpretable relationship representations. This claim is **unverified** — the current hash encoder doesn't produce semantically meaningful encodings, so the algebraic operations operate on noise.

The gap is not in *what the tool does with data* (the algebra is correct). The gap is in *how data enters the tool* (the encoding is semantically vacuous).

---

## 2. The Encoding Problem: Why Hash Is Wrong and How to Fix It

### 2.1 The Diagnosis

The I-Ching worked for 3000 years not because trigrams are magic, but because there is a **consultation protocol** — a disciplined method for mapping a concrete situation onto the 8 fundamental dynamics. The protocol involves:

1. **Decomposition:** Breaking a situation into components
2. **Pattern recognition:** Identifying which trigram matches each component
3. **Composition:** Stacking trigrams into hexagrams (upper = context, lower = entity)
4. **Dynamics:** Identifying moving lines (which aspects are about to change)
5. **Interpretation:** Applying 3000 years of commentary to the resulting hexagram

GA-Bagua replaced all 5 steps with `DefaultHasher::hash(word)`. That's not an implementation detail — it's a category error. Hashing is for hash tables, not for semantic encoding.

### 2.2 The Fix: LLM as Oracle (Not as Neural Network)

The LLM should act as the *interpreter* applying the Bagua framework, not as an embedder. The key insight: an LLM already understands what "causal" means, what "generative" means, what "constraining" means. It can classify a concept description onto these 8 axes far better than any hash function — and it can do it now, with no training.

#### 2.2.1 The Encoding Skill/Prompt

This is designed as an installable **skill** for LLM harnesses (OpenCode, ClaudeCode, Cursor, etc.):

```markdown
# GA-Bagua Semantic Encoding Skill

## Purpose
Encode any concept, text, or code entity into an 8-element multivector
using geometric algebra semantic role taxonomy derived from the I-Ching Bagua.

## The 8 Semantic Roles
Each role maps to one basis blade of Cl(3) geometric algebra:

0. **receptive** (坤 Kūn — Earth, scalar blade): Accepts, follows, grounds;
   dependency adoption. Represents the passive, yielding aspect of a concept.

1. **causal** (震 Zhèn — Thunder, e1 blade): Triggers, initiates, starts
   chain reactions. Represents the event-driven, initiating aspect.

2. **transmissive** (坎 Kǎn — Water, e2 blade): Channels, flows, transmits.
   Represents the data-flow, pipeline, streaming aspect.

3. **constraining** (艮 Gèn — Mountain, e3 blade): Limits, bounds,
   restricts. Represents the permission/capacity/boundary aspect.

4. **influential** (巽 Xùn — Wind, e12 blade): Pervades, gradually affects.
   Represents the convention-spreading, osmotic aspect.

5. **clarifying** (離 Lí — Fire, e23 blade): Reveals, illuminates,
   makes visible. Represents the introspection, dependency-revelation aspect.

6. **balancing** (兌 Duì — Lake, e31 blade): Mirrors, equilibrates,
   reflects. Represents the feedback, mutual-dependency aspect.

7. **generative** (乾 Qián — Heaven, e123 blade): Introduces, creates,
   initiates new patterns. Represents the creative, innovative aspect.

## Encoding Rules
For a given concept, assign a weight in [-1.0, 1.0] to each role:
- Positive: the concept actively exhibits this quality
- Negative: the concept is the opposite/counter to this quality
- Near zero: this quality is irrelevant to the concept
- The 8 weights must form a unit-length vector (Euclidean norm ≈ 1.0)

## Output Format
Output ONLY a JSON array of 8 floats, one per role in order:
[receptive, causal, transmissive, constraining, influential, clarifying,
 balancing, generative]

## Example
Concept: "a database transaction that ensures atomicity of operations"
Output: [0.15, 0.05, 0.05, 0.85, 0.15, 0.30, 0.30, 0.15]
Reasoning: heavily constraining (transaction boundaries, rollback rules);
some clarifying (makes state visible); some balancing (ACID properties
reflect each other); lightly receptive (follows SQL conventions)

## Example
Concept: "a novel idea that disrupts an established industry"
Output: [0.10, 0.60, 0.30, -0.50, 0.45, 0.20, 0.15, 0.85]
Reasoning: strongly generative (creates new patterns); strongly causal
(triggers disruption); counter-constraining (breaks existing boundaries);
influential (spreads through industry); transmissive (flows through
networks)
```

#### 2.2.2 How the Skill Integrates

```
┌──────────────────────────────────────────────────────────────┐
│  LLM Harness (OpenCode / ClaudeCode / Cursor)                │
│                                                              │
│  1. User asks: "Explore how module X relates to module Y"    │
│                                                              │
│  2. LLM invokes the Bagua Encoding skill for each concept:   │
│     → text_to_multivector("module X does auth...")           │
│     → text_to_multivector("module Y does database...")       │
│     Result: 2 × 8 f64 = 128 bytes                           │
│                                                              │
│  3. LLM calls MCP tools with the 8-element arrays:           │
│     → classify_relation(mv_x, mv_y)                          │
│     → semantic_similarity(mv_x, mv_y)                        │
│     → detect_contradiction(mv_x, mv_y)                       │
│                                                              │
│  4. LLM interprets results for user (1-2 sentences)          │
│                                                              │
│  5. LLM stores encoding for future queries (no re-encoding)  │
└──────────────────────────────────────────────────────────────┘
```

**Token economics:**
- Encode concept: ~200 prompt tokens (skill instructions) + ~20 concept tokens = ~220 tokens
- Re-use concept in 50 queries: 0 additional tokens for encoding
- Each relation query via MCP algebra: 0 LLM tokens
- Each query requiring LLM reasoning: ~50 tokens for interpretation
- **Cost per query with Bagua:** ~50 tokens (LLM interpretation only)
- **Cost per query without Bagua:** ~500 tokens (full-text concept comparison)
- **10x-20x token savings** for repeated concept exploration

#### 2.2.3 Why a Skill and Not a Prompt-in-the-Moment

| Approach | Pros | Cons |
|----------|------|------|
| Ad-hoc prompt each time | Simple; no setup | Inconsistent encoding; different LLM calls produce different coefficients for same concept |
| **Skill (this proposal)** | Consistent protocol; can be refined; portable across harnesses | One-time encoding cost per concept |
| Plugin/binary | Fastest; no prompt tokens per encoding | Requires model training or embedding model dependency |

The skill is the right first step because:
1. No training needed — uses LLM's existing semantic understanding
2. Consistent — the skill prompt stabilizes the encoding protocol
3. Iterable — refine the prompt based on encoding quality benchmarks
4. Composable — skill can emit JSON that flows directly into MCP tool calls

---

## 3. The Full Roadmap: 7 Layers to Excellence

```
Layer 7: Agent Integration (persistent memory)      ← Week 7-8
Layer 6: Visualization (WebGL 3D + radar charts)    ← Week 6
Layer 5: Multi-Hop Reasoning Engine                 ← Week 5-6
Layer 4: Retrieval + Persistence                    ← Week 4-5
Layer 3: Higher-Dimensional GA (Cl(4)/Cl(5))        ← Week 3-4
Layer 2: Benchmarks & Validation                    ← Week 2-3
Layer 1: LLM Encoding Protocol (THE SKILL)          ← Week 1-2  ← START HERE
──────────────────────────────────────────────────
Layer 0: Current state (algebra, taxonomy, MCP, CLI)
```

### Layer 1: LLM Encoding Protocol (Week 1-2)

This is the critical path. Nothing else matters until encoding quality is solved.

| Task | Effort | Deliverable |
|------|--------|-------------|
| Formalize the encoding skill prompt | 0.5 day | `docs/skills/bagua-encoder.md` with examples, role descriptions, encoding rules |
| Install as OpenCode skill | 0.5 day | `.opencode/skills/bagua-encoder/` with SKILL.md + test cases |
| Add MCP tool: `llm_encoded_semantic_explore` | 1 day | Accepts concept+related text arrays; tool returns null coefficients as placeholder; LLM fills in coefficients via the skill before calling algebra tools |
| Add MCP tool: `validate_encoding` | 1 day | Given a pair of encoded concepts and a human-labeled relationship, checks consistency and returns divergence score |
| Write encoding quality test suite | 2 days | 50 ground-truth concept pairs with known relationships; measure role-label alignment |
| Iterate on skill prompt | 2 days | Based on test suite results; tune role descriptions and encoding rules |

**Exit criteria:**
- [ ] LLM can encode a concept into 8 coefficients with 100% reproducibility (same concept, same coefficients)
- [ ] Encoded "cause → effect" relationship classified as "causal" with >70% confidence
- [ ] Encoded "boundary → restricted entity" classified as "constraining" with >70% confidence
- [ ] 50-pair encoding quality benchmark achieves >75% role-label alignment with human labels

### Layer 2: Benchmarks & Validation (Week 2-3)

Prove the tool works — or discover its limits — before building more.

| Task | Effort | Deliverable |
|------|--------|-------------|
| Wire in criterion benchmarks | 1 day | `cargo bench` reports ns/op for geometric product, rotor application, analogy, batch |
| Analogy benchmark suite | 2 days | 100+ Google word analogy test cases adapted for Bagua encoding; accuracy@1, accuracy@5 |
| Relation classification benchmark | 2 days | Standard KG dataset (WN18RR subset or FB15k-237); 1000 relation triples; classify via Bagua + benchmark |
| Performance benchmarks | 1 day | Memory footprint, batch throughput, multi-hop composition latency |
| Encoding quality correlation study | 2 days | Compare LLM-assisted encoding vs hash-based vs random baseline on human-labeled triple dataset |

**Exit criteria:**
- [ ] `cargo bench` runs and produces HTML reports
- [ ] Analogy benchmark: accuracy@1 > 0.3 (100+ test cases)
- [ ] Relation classification: >50% accuracy on 8-class problem (random baseline = 12.5%)
- [ ] LLM-encoded results significantly outperform hash-encoded (p < 0.01)
- [ ] Geometric product latency < 50ns

### Layer 3: Higher-Dimensional GA (Week 3-4)

Add Cl(4) and Cl(5) as optional features to increase expressive power.

| Task | Effort | Deliverable |
|------|--------|-------------|
| `Cl4Multivector` type (16 coeffs) | 2 days | New multivector struct; 16×16 Cayley table; geo product, rotor in 6 bivector planes |
| `Cl5Multivector` type (32 coeffs) | 2 days | 32×32 Cayley table; 10 bivector planes |
| Sub-role taxonomy mapping | 1 day | 8 core roles × 2 (positive/negative emphasis) × 2 (strong/weak) = 32 sub-roles for Cl(5) |
| Backward compatibility layer | 1 day | `Multivector::project_to_cl3()` for seamless interop with existing Cl(3) operations |
| Dimension-agnostic trait | 1 day | `GeometricAlgebra<const D: usize>` trait for generic operations across Cl(3..5) |

**Exit criteria:**
- [ ] Cl(4) and Cl(5) geometric products match known Cayley tables
- [ ] All Cl(3) operations work identically in `project_to_cl3()`
- [ ] Encoding quality improves measurably with Cl(4)/Cl(5) vs Cl(3) baseline

### Layer 4: Retrieval + Persistence (Week 4-5)

Store concept encodings, retrieve by algebraic similarity.

| Task | Effort | Deliverable |
|------|--------|-------------|
| SQLite-backed concept store | 2 days | `ConceptStore` with CRUD; table: `concepts(id, name, text, mv_blob, created_at)` |
| Geometric product ANN retrieval | 2 days | `store.query_similar(query_mv, top_k)` → returns closest concepts ranked by similarity |
| Relation store | 1 day | `relations(id, from_id, to_id, relation_type, strength)` — stores classified relationships |
| Export/Import | 1 day | Dump entire concept graph as JSON; load from JSON; compatible with NetworkX/igraph |
| MCP tools for store | 1 day | `store_concept`, `query_similar`, `get_concept_graph`, `export_graph` |

**Exit criteria:**
- [ ] Store 10,000 encoded concepts and query top-10 by similarity in <1ms
- [ ] Export concept graph as JSON loadable by NetworkX
- [ ] MCP agent can store, retrieve, and explore concepts across sessions

### Layer 5: Multi-Hop Reasoning Engine (Week 5-6)

The unique value proposition: compose relationships without LLM calls.

| Task | Effort | Deliverable |
|------|--------|-------------|
| `ReasoningGraph` data structure | 2 days | Directed graph of concepts (nodes) with relation rotors (edges); adjacency list |
| Multi-hop path finder | 2 days | Dijkstra-like search for paths between two concepts; score by path length + confidence |
| Rotor chain composition | 1 day | Compose all rotors along a path → compound relation between distant concepts |
| MCP tool: `trace_relation` | 1 day | Given concept A and concept B, find all paths ≤ N hops, compose rotors, return classification |
| MCP tool: `explore_neighborhood` | 1 day | Given concept A, return all concepts within M hops, classified by relation type |

**Exit criteria:**
- [ ] Trace A→B relation across 5+ hops in <10ms for a 10,000 node graph
- [ ] Compound rotor correctly composes individual rotors (verified by manual application)
- [ ] `trace_relation` returns interpretable results ("A ⊗ B: moderately generative [0.65], strongly counter-constraining [0.82] via 3-hop path")

### Layer 6: Visualization (Week 6)

Make the geometry visible. If people can see it, they believe it.

| Task | Effort | Deliverable |
|------|--------|-------------|
| 3D Cl(3) space renderer | 2 days | Three.js visualization: concepts as colored points in 3D vector space; bivector planes as translucent disks |
| Role radar chart | 1 day | Per-concept radar/spider chart showing 8 role weights |
| Relation arrows | 1 day | Arrows between concepts colored by relation type; animated rotor transformations |
| MCP tool: `visualize_subgraph` | 1 day | Returns HTML/CSS/JS that renders a self-contained 3D visualization of a concept subgraph |
| Web dashboard | 2 days | Serve from CLI: `ga-semantics serve --port 8080` → interactive exploration UI |

**Exit criteria:**
- [ ] `visualize_subgraph(concept_ids, depth=2)` produces a self-contained HTML file
- [ ] 3D view shows concepts positioned by dominant role; arrows colored by relation
- [ ] Clicking a concept shows its 8-role radar chart

### Layer 7: Agent Integration (Week 7-8)

Package everything so an LLM agent uses GA-Bagua as persistent semantic memory.

| Task | Effort | Deliverable |
|------|--------|-------------|
| Python bindings via PyO3 | 2 days | `pip install ga-semantics` exposes all core operations |
| Codebase scanner skill | 2 days | OpenCode skill that crawls a codebase, encodes each symbol (function/class/module) into Bagua, stores relations |
| Session persistence | 1 day | Agent can save/load the entire concept graph across sessions |
| GitHub Release CI | 1 day | Pre-built binaries for Linux (x64), macOS (ARM64), Windows (x64) |
| crates.io publish | 1 day | `ga-semantics-core`, `ga-semantics-mcp`, `ga-semantics-cli` on crates.io |

**Exit criteria:**
- [ ] `pip install ga-semantics` works on all platforms
- [ ] Agent can encode 1000 code symbols, classify relations, and answer "how does X relate to Y?" without re-encoding
- [ ] Pre-built binaries available on GitHub Releases
- [ ] At least one public demo: exploring a real open-source codebase (e.g., ripgrep, serde, or tokio)

---

## 4. Architecture of the Complete System

```
┌─────────────────────────────────────────────────────────────────┐
│                        LLM AGENT                                 │
│  ┌──────────────────────┐     ┌──────────────────────────────┐  │
│  │ Bagua Encoder Skill  │     │  General Reasoning            │  │
│  │ (text → 8 coeffs)    │     │  (narrative, explanation)     │  │
│  └─────────┬────────────┘     └──────────┬───────────────────┘  │
└────────────┼─────────────────────────────┼──────────────────────┘
             │                             │
             │ encode                      │ interpret results
             ▼                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                  ga-semantics-mcp (MCP Server)                    │
│                                                                  │
│  text_to_multivector  semantic_explore  trace_relation           │
│  classify_relation    analogy           visualize_subgraph       │
│  compose_relations    detect_contradiction                       │
│  store_concept        query_similar      wuxing_query            │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                  ga-semantics-core                          │  │
│  │                                                            │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │  │
│  │  │ Cl(3)    │  │ Cl(4)    │  │ Cl(5)    │  │ Bagua    │  │  │
│  │  │ Algebra  │  │ Algebra  │  │ Algebra  │  │ Taxonomy │  │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────────┐    │  │
│  │  │ Semantics│  │ Encoding │  │ Concept Store (SQLite)│    │  │
│  │  │ (sim,    │  │ (hash,   │  │                       │    │  │
│  │  │  diff,   │  │  llm,    │  │ concepts, relations,  │    │  │
│  │  │  analogy)│  │  word)   │  │ graph, export/import  │    │  │
│  │  └──────────┘  └──────────┘  └──────────────────────┘    │  │
│  └───────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 5. The Bagua Dynamics That Must Drive Encoding

These features are all implemented in the codebase but are never used during concept encoding. Each one represents a Bagua principle that should inform how concepts get encoded:

### 5.1 Trigram Binary Structure (yin/yang lines)

```rust
Trigram::binary() → [bool; 3]
```

A trigram's three lines correspond to three aspects of a concept:
- **Bottom line (heaven):** Intent/purpose — what the concept tries to achieve
- **Middle line (human):** Method/mechanism — how it operates
- **Top line (earth):** Effect/outcome — what it actually produces

Encoding protocol: decompose a concept into these three aspects, assign yang (active, strong) or yin (passive, weak) to each, and the resulting binary pattern determines the dominant trigram. This gives a principled way to map a concept onto a trigram without statistical encoding.

### 5.2 Line-Change Transforms

```rust
trigram.transform_line(n) → Trigram
```

Each line change represents which aspect of a concept is shifting. A concept that is "about to become more constrained" has a moving bottom line. The transform gives the *next state*.

Encoding protocol: identify which aspect of a concept is most dynamic or unstable — encode that as a moving line to predict how the concept evolves.

### 5.3 Hexagram Stacking

```rust
Hexagram::new(upper, lower) → Hexagram
```

Upper trigram = context/environment; lower trigram = entity/self. The hexagram's meaning is not the sum of the two trigrams — it's an emergent interpretation.

Encoding protocol: when encoding a pair of related concepts, treat the pair AS a hexagram. The classification between them should match the hexagram's traditional interpretation.

### 5.4 WuXing Cycles

```rust
WuXing::Wood.generate() → Fire
WuXing::Wood.control() → Earth
```

The generating cycle (Wood→Fire→Earth→Metal→Water→Wood) describes supportive relationships. The controlling cycle (Wood→Earth→Water→Fire→Metal→Wood) describes restrictive relationships.

Encoding protocol: when encoding two concepts in a known relationship, the trigram→WuXing mapping should align with the generating or controlling cycle. If concept A is the "parent" of concept B, A's WuXing element should generate B's WuXing element.

### 5.5 Complementary Trigrams

```rust
trigram.complementary() → Trigram
```

Complementary trigrams (Kun↔Qian, Gen↔Dui, Kan↔Li, Xun↔Zhen) represent mutual completion. A concept and its antithesis should have complementary trigrams.

Encoding protocol: if two concepts are opposites, their dominant trigrams should be complementary pairs.

---

## 6. Success Metrics

| Metric | Current | Target (After Layer 2) | Target (After Layer 5) |
|--------|---------|------------------------|------------------------|
| Encoding reproducibility | Hash: 100% (deterministic), LLM: untested | LLM: >95% (same prompt, same concept, same coefficients) | Same |
| Relation classification accuracy (human-labeled) | Untested | >70% accuracy on 50-pair benchmark | >85% on 500-pair benchmark |
| Analogy accuracy (Google test set) | Untested | >30% accuracy@1 | >50% accuracy@1 |
| Encoding uniqueness (distinct concepts → distinct encodings) | Poor (single words collapse) | >99% of distinct concepts have non-identical encodings | Same |
| Query latency (10,000 concept store) | N/A (no store) | <1ms for top-10 similarity | <0.5ms |
| Multi-hop composition latency (5 hops) | N/A | <10ms | <5ms |
| Token savings vs raw text exploration | N/A | 5x fewer tokens (encoding = ~220 tokens one-time; queries = 0) | 20x fewer tokens (persistent store + cached encodings) |
| Test coverage (core + MCP) | 98 tests passing | 150+ tests; encoding quality tests added | 250+ tests; integration tests for full pipeline |

---

## 7. Risk Register

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| LLM encoding inconsistent across different models | High | Medium | Lock skill prompt; test across Claude, GPT-4, Gemini; if too variable, fall back to embedding-based encoder |
| 8 dimensions inherently insufficient for complex concepts | Medium | High | Cl(4) and Cl(5) already planned; if still insufficient, the tool's niche shrinks to simple relationship classification only |
| No benchmark improvement over random baseline | Medium | High | Pivot messaging: the role label taxonomy IS the value even if numbers aren't better; publish negative result honestly |
| Academic community dismisses Bagua grounding as mysticism | Medium | Low | Always lead with Cl(3) math; Bagua is the *naming convention*, not the foundation; the math stands alone |
| LLM encoding costs too many tokens for large-scale use | Medium | Medium | One-time encoding per concept is amortized across many queries; 220 tokens to encode a concept is cheap if queried 20+ times |
| Community doesn't adopt MCP interface | Low | Medium | Provide Python bindings as fallback; pip install is lower barrier than MCP for Python ML users |
| Bagua dynamics (WuXing, moving lines) produce arbitrary-feeling rules | High | Medium | Validate each rule against benchmark data; if a rule doesn't improve classification accuracy, deprecate it |

---

## 8. Immediate Next Actions (This Week)

1. **Create the encoding skill file** at `docs/skills/bagua-encoder/SKILL.md` following OpenCode skill conventions
2. **Write the 50-pair encoding quality benchmark** — curate 50 concept pairs with unambiguous human-labeled relationships (e.g., "earthquake → building collapse" = causal; "privacy policy → data collection" = constraining)
3. **Run the benchmark with LLM-assisted encoding** to establish baseline encoding quality
4. **Remove hash-based encoding as the default** — make it an explicit fallback labeled "lexical encoding (low quality)"
5. **Fix the CLI `--help`** by adding back clap `help` and `usage` features without `color`

---

## Appendix A: The 64 Hexagram Naming — How It Could Inform Classification

The 64 hexagrams are built from stacking two trigrams. If we treat a pair of encoded concepts as upper and lower trigrams, the hexagram number/name would classify the PAIR at a higher semantic resolution than just the 8 role labels. This is already implemented in `bagua.rs::HEXAGRAM_NAMES` but never exposed through the MCP or CLI as a classification tool.

Example: If concept A encodes to Qian (generative) and B to Kun (receptive), the pair is hexagram 12 (否 Pǐ — "Standstill/Stagnation"). The I-Ching commentary describes this as a situation where creative force meets receptive passivity but cannot move — a blockage. This is semantically richer than just saying "A is generative, B is receptive" — it tells you about the *compound dynamic*.

Adding a `classify_hexagram(a, b)` tool that returns:
```json
{
  "hexagram_number": 12,
  "hexagram_name": "否 Pǐ — Standstill/Stagnation",
  "upper_trigram": "generative",
  "lower_trigram": "receptive",
  "interpretation": "Creative force blocked by passive acceptance; innovation meets inertia"
}
```

...would open up the full 64-state relationship taxonomy.

---

## Appendix B: Skill File Template (OpenCode)

```
.opencode/skills/bagua-encoder/
├── SKILL.md              # The encoding protocol prompt (Section 2.2.1)
├── examples.md           # Annotated encoding examples
├── test_cases.json       # Ground-truth pairs for validation
└── README.md             # Install instructions
```

The skill's `SKILL.md` would be exactly the encoding prompt from Section 2.2.1 above. OpenCode would load it as a skill, and the LLM would apply it whenever the user asks to encode, compare, or explore concepts.

---

## Appendix C: CLI Fix Procedure

The CLI's `Cargo.toml` currently has:
```toml
clap = { version = "4", features = ["derive", "std", "help", "usage", "error-context"], default-features = false }
```

This disables the `color` feature (which pulls `windows-sys` → `dlltool.exe`), keeping the CLI buildable on GNU Windows. The `--help` works with these explicit features. No further fix needed — the current `Cargo.toml` is correct.

If terminal colors are desired on MSVC platforms, add:
```toml
[target.'cfg(target_env = "msvc")'.dependencies]
clap = { version = "4", features = ["derive", "color"] }
```

---

*This document supersedes the previous roadmap.md and epics.md for strategic direction. Those documents remain authoritative for existing feature specifications.*
