# ga-semantics — Roadmap

---

## Timeline Overview

```
Week 1-2:  Core Algebra Engine
Week 2-3:  Bagua Mapping Layer
Week 3:    Semantic Operations API
Week 4:    Benchmarks & Validation
Week 5:    Python Bindings & Publishing
Week 6-7:  MCP Server & CLI Delivery
Week 8-10: Application Expansion (Doc Intel, Cognitive, Ideation)
```

**Total Duration:** ~10 weeks (50 working days)
**Launch Target:** Mid August 2026

---

## Phase 1: Core Algebra Engine

**Duration:** Week 1-2 (June 4 – June 15, 2026)
**Status:** 🟡 In Planning

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M1.1 | June 6 | `Multivector` type with 8 f64 coefficients |
| M1.2 | June 9 | Geometric product via coefficient multiplication table |
| M1.3 | June 11 | Grade projection, inner/outer product, reverse, norm |
| M1.4 | June 12 | Inverse, dualization, rotor construction/application |
| M1.5 | June 13 | `Blade` enum and basis blade constants |
| M1.6 | June 14 | Algebra correctness tests (Cayley table, inverse, rotor) |
| M1.7 | June 15 | `serde` support + benchmarks |

### Exit Criteria

- [ ] All Cl(3) Cayley table entries verified
- [ ] Inverse property holds for all non-degenerate multivectors
- [ ] Rotor unitarity verified
- [ ] Criterion benchmarks report <50ns for geometric product
- [ ] `cargo clippy` passes with zero warnings

---

## Phase 2: Bagua Mapping Layer

**Duration:** Week 2-3 (June 16 – June 25, 2026)
**Status:** ⬜ Pending

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M2.1 | June 17 | `Trigrams` enum with blade mapping, binary encoding, Chinese names |
| M2.2 | June 19 | `Hexagram` type (upper/lower trigram, 64 named states) |
| M2.3 | June 20 | `WuXing` enum with generating/controlling cycles |
| M2.4 | June 22 | Trigram transformation rules (line-change operations) |
| M2.5 | June 23 | `dominant_role()` (public), `relationship_to()`, `wuxing_phase()` |
| M2.6 | June 25 | Bagua mapping tests (8 trigrams ↔ 8 basis blades) |

### Exit Criteria

- [ ] 8 trigrams correctly mapped to 8 basis blades (bidirectional)
- [ ] 64 hexagrams constructible from trigram pairs
- [ ] WuXing generating and controlling cycles verified
- [ ] All trigram metadata accessible (Chinese names, translations, binary)

---

## Phase 3: Semantic Operations API

**Duration:** Week 3 (June 26 – July 2, 2026)
**Status:** ⬜ Pending

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M3.1 | June 27 | `semantic_similarity()` + `semantic_difference()` |
| M3.2 | June 28 | `semantic_relation()` trigram classification |
| M3.3 | June 29 | Analogy computation: `analogy(a, b, c) = (a⁻¹b) * c` |
| M3.4 | June 30 | Relation composition: `compose(r1, r2) = r2 * r1` |
| M3.5 | July 1 | Context transformations (Context struct, apply, compose) |
| M3.6 | July 2 | Integration tests + examples |

### Exit Criteria

- [ ] Similarity metric returns values in [-1, 1]
- [ ] Difference metric returns values in [0, 1]
- [ ] Analogy computation produces correct results on test cases
- [ ] Relation composition is associative (mathematical property verified)
- [ ] Context transformations produce correct geometric results
- [ ] All 3 examples compile and run without errors

---

## Phase 4: Benchmarks & Validation

**Duration:** Week 4 (July 3 – July 11, 2026)
**Status:** ⬜ Pending

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M4.1 | July 4 | Relation classification benchmark setup |
| M4.2 | July 6 | Analogical reasoning benchmark (Google word analogy adaptation) |
| M4.3 | July 9 | KG link prediction benchmark (GeomE reproduction) |
| M4.4 | July 10 | Performance benchmarks (criterion, memory, batch, parallel) |
| M4.5 | July 11 | Decision gate evaluation |

### Exit Criteria

- [ ] Relation classification accuracy reported vs. random baseline
- [ ] Analogy benchmark runs on ≥100 test cases
- [ ] Link prediction benchmark produces MRR, Hits@1/3/10
- [ ] Performance benchmarks confirm O(1) for core operations
- [ ] Memory, batch throughput, and parallel scaling numbers documented
- [ ] Decision gate evaluation completed

### Decision Gate

| Benchmark Result | Action |
|-----------------|--------|
| Accuracy > baseline + 5% on 2+ benchmarks | Promote to production; integrate into ACC KG |
| Accuracy within ±5% of baseline | Maintain as research project; optional ACC integration |
| Accuracy < baseline - 5% | Document findings; publish negative result; shelve |

---

## Phase 5: Python Bindings & Publishing

**Duration:** Week 5 (July 14 – July 21, 2026)
**Status:** ⬜ Pending

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M5.1 | July 15 | PyO3 Python bindings (optional feature) |
| M5.2 | July 17 | API docs, examples, README, mathematical background |
| M5.3 | July 18 | Publishing preparation + verification (dry-run, CHANGELOG) |
| M5.4 | July 19 | Publish to crates.io |
| M5.5 | July 20 | Publish to PyPI (if Python bindings exist) |
| M5.6 | July 21 | Blog post / preprint on Cl(3)↔Bagua isomorphism |

### Exit Criteria

- [ ] `cargo publish --dry-run` passes
- [ ] `pip install ga-semantics` works (if Python bindings)
- [ ] ≥80% documentation coverage
- [ ] CHANGELOG.md updated and examples verified to compile
- [ ] Blog post or preprint drafted

---

## Phase 6: MCP Server & CLI Delivery

**Duration:** Week 6-7 (July 22 – August 4, 2026)
**Status:** ⬜ Pending

### Overview

Two standalone binaries built on top of the `ga-semantics` Rust library:

| Binary | Purpose | Primary Audience |
|--------|---------|-----------------|
| `ga-semantics-mcp` | MCP server exposing all operations as tools | AI agents (Claude, etc.) |
| `ga-semantics` | CLI for interactive use, scripting, and batch processing | KG engineers, researchers, DevOps |

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M6.1 | July 23 | `mcp-server/` crate with stdio transport, all tools registered |
| M6.2 | July 25 | MCP tools: `create_multivector`, `semantic_similarity`, `analogy` |
| M6.3 | July 28 | MCP tools: `classify_relation`, `compose_relations`, `contradiction` |
| M6.4 | July 30 | MCP tools: `trigram_info`, `wuxing_info`, `batch_process` |
| M6.5 | August 1 | `cli/` crate with clap, all subcommands + JSON/file/stdin I/O |
| M6.6 | August 2 | CLI human-readable output (tables, colors) + pipe-friendly JSON mode |
| M6.7 | August 3 | Integration tests: MCP tool calls roundtrip, CLI pipeline end-to-end |
| M6.8 | August 4 | README + examples for both binaries; CI publish to GitHub Releases |

### MCP Server — Tools

| Tool | Description |
|------|-------------|
| `create_multivector` | Build a multivector from coefficient array or blade+coefficient pairs |
| `semantic_similarity` | Compute similarity [-1, 1] between two multivectors |
| `semantic_difference` | Compute difference [0, 1] between two multivectors |
| `analogy` | Solve `a : b :: c : ?` via rotor application |
| `classify_relation` | Classify relation between two multivectors — returns role label (causal, generative, etc.) |
| `compose_relations` | Compose two rotors into a compound relation |
| `detect_contradiction` | Check if bivector magnitude exceeds threshold |
| `relation_type_info` | Look up a role label's definition, canonical Bagua trigram, WuXing phase, and examples |
| `wuxing_query` | Query generating/controlling cycle for a phase |
| `context_apply` | Apply a context rotor to transform a multivector |
| `batch_process` | Run multiple operations in a single request (JSON array) |

### CLI — Commands

| Command | Example |
|---------|---------|
| `ga-semantics mv create <coefficients>` | `ga-semantics mv create 1.0 0 0 0 0 0 0 0` |
| `ga-semantics sim <a> <b>` | `ga-semantics sim --json '[1,0,...]' '[0,1,...]'` |
| `ga-semantics analogy <a> <b> <c>` | `ga-semantics analogy --file a.json b.json c.json` |
| `ga-semantics classify <a> <b>` | `ga-semantics classify --stdin < pair.json` |
| `ga-semantics compose <r1> <r2>` | `ga-semantics compose r1.json r2.json` |
| `ga-semantics relation-type <role>` | `ga-semantics relation-type causal` |
| `ga-semantics wuxing <phase>` | `ga-semantics wuxing wood --cycle generating` |
| `ga-semantics batch <file>` | `ga-semantics batch operations.json` |
| `ga-semantics eval <file>` | `ga-semantics eval analogy_benchmark.json` |

### Exit Criteria

- [ ] MCP server starts, registers all tools, and responds to tool calls via stdio
- [ ] AI agent (Claude) can connect and successfully call all tools
- [ ] CLI installs via `cargo install ga-semantics` and `npm install -g ga-semantics` (via npx)
- [ ] All CLI subcommands accept `--json`, `--file`, and `--stdin` input modes
- [ ] `--help` output is complete and useful for each subcommand
- [ ] Integration tests cover: MCP roundtrip, CLI pipe, CLI JSON mode
- [ ] Both binaries publish to GitHub Releases via CI

---

## Phase 7: Application Expansion

**Duration:** Week 8-10 (June 8 – June 20, 2026)  
**Status:** 🟢 Complete

### Overview

Three application subsystems built on the core algebra engine:

| Subsystem | Crate | Ideas | Operations |
|-----------|-------|-------|------------|
| **Document Intelligence** | `ga-doc-intel` | Argument mapping, document alignment, research synthesis, policy coherence, cross-lingual alignment, smart contract audit | 10 |
| **Cognitive Systems** | `ga-cognitive` | Agent belief tracking, personality compatibility, learning path generation, goal decomposition | 15 |
| **Creative Ideation** | in-core (`bagua.rs`) | 64-hexagram perspective exploration, concept blending | 4 |

### Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| M7.1 | June 8 | Core changes: `document_id`/`agent_id` in store, `hexagram_step()`, `belief_revise()` |
| M7.2 | June 9 | `ga-doc-intel` crate: 6 modules + 24 tests |
| M7.3 | June 9 | `ga-cognitive` crate: 5 modules + 21 tests |
| M7.4 | June 9 | Ideation MCP tools (4) + CLI subcommands (4) |
| M7.5 | June 10 | 9 validation benchmarks (B1-B11) |
| M7.6 | June 10 | Documentation and final QA |

### Exit Criteria

- [x] 2 new workspace crates compile and pass tests (45 tests)
- [x] 29 new operations across MCP and CLI
- [x] 9 validation benchmarks all pass
- [x] 0 new clippy warnings
- [x] 179 existing core tests continue to pass
- [x] All project docs updated

---

**Duration:** TBD (after decision gate approval)
**Status:** ⬜ Pending

### Integration Tasks

| Task | Effort | Dependencies |
|------|--------|-------------|
| Add `relation_type` column to `knowledge_relations` | 1 day | ga-semantics v0.1 published |
| Add `relation_multivector` column (BLOB) | 0.5 day | ga-semantics v0.1 published |
| Knowledge Compounder role classification | 2 days | Schema migration complete |
| Contradiction detection via bivector magnitude | 1 day | Schema migration complete |
| KG visualization color-coding by role | 1 day | Schema migration complete |
| Multi-hop query composition via rotors | 2 days | Core integration complete |

---

## Version Plan

| Version | Milestone | Target |
|---------|-----------|--------|
| v0.1.0 | Core algebra + Bagua + semantic ops | July 2026 |
| v0.2.0 | Python bindings + benchmarks | August 2026 |
| v0.3.0 | SIMD optimization + batch operations | September 2026 |
| v0.4.0 | MCP server + CLI binaries | October 2026 |
| v0.5.0 | Application expansion (doc-intel, cognitive, ideation) | June 2026 |
| v1.0.0 | Production-ready + ACC integration | November 2026 |

---

## Risk-Adjusted Timeline

| Risk | Impact | Mitigation | Timeline Effect |
|------|--------|-----------|-----------------|
| Benchmarks show no improvement | High | Role label interpretability alone is valuable | No delay — proceed with v0.1 |
| PyO3 compilation issues | Medium | Use maturin for wheel building | +2 days if needed |
| Cl(3) implementation bugs | Low | Property-based testing catches issues | +1 day buffer built in |
| Academic review delays | Low | Submit early; iterate on feedback | No timeline dependency |
| MCP protocol spec changes | Low | Pin `mcp-rs` version; own transport layer | No delay |
| CLI binary size too large | Low | Static linking; strip symbols; optional | +1 day for CI tuning |
