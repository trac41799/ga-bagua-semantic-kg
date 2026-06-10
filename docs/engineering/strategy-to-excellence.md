# GA-Bagua Semantic KG — Strategy to Excellence

**Date:** 2026-06-08
**Status:** Living document — reflects current state after parallel workstreams
**Tests:** 141 unit + 11 benchmark suites, all passing

---

## 1. Where We Are

### 1.1 What Works (Complete)

| Layer | Status | Detail |
|-------|:------:|--------|
| Cl(3) Geometric Algebra | Complete | 8 basis blades, geometric product, inner/wedge, reverse, inverse, dualize, grade projection |
| Rotor algebra | Complete | Construction, sandwich product, composition, inverse, multi-hop (100-hop, zero drift) |
| Bagua taxonomy | Complete | 8 trigrams ↔ 8 blades, 64 hexagrams with interpretations, WuXing 5-phase cycles |
| RelationType taxonomy | Complete | 8 semantic role labels with descriptions, Bagua/WuXing mappings |
| Multi-hypothesis classifier | Complete | `from_pair_multi` scores all 8 labels with 4 evidence sources + margin confidence |
| Sharpness gate | Complete | `encoding_sharpness()` gates random noise (93.5% → 0.0 confidence) |
| WuXingIndex | Complete | Phase-bucketed retrieval, domain support, weighted scoring, complementary queries, path traversal |
| Retrieval operations | Complete | `dominant_similarity`, `fingerprint_similarity`, `relationship_spectrum`, `evolve_concept` |
| MCP server | Operational | 29 tools with JSON schemas; diagnostic and corrective prompt tools |
| CLI | Operational | 12 subcommands; JSON/CSV/human-readable output |
| Test suite | 141 pass | 94 unit + 47 integration (diagnostic, classifier, spectrum, index) |
| Benchmarks | 11 pass | Train/test, retrieval quality, scalability, context compression, baselines |
| Diagnostic module | Complete | `diagnose_pair`, `diagnose_dataset`, encoding alignment scores, corrective prompts |

### 1.2 Current Bottlenecks

| Bottleneck | Severity | Root Cause | Owner |
|-----------|:--------:|-----------|:-----:|
| Encoding quality (alignment 15-19%) | Critical | SKILL.md v1/v2 encode intrinsic properties, not relational position | Encoding-quality team |
| Same-role R@1 ceiling (42%) | High | Secondary coefficients are noise; within-role ranking is random | Blocked by encoding |
| Relation classification limited to 45-52% | High | Encoding alignment ceiling; WuXing cycle signal requires correct phases | Blocked by encoding |
| Retrieval P@5 below random baseline | Medium | Measures link prediction (not design goal); use same-role R@10 instead | Documentation |
| Receptive label at 0% | Medium | Scorer prefers specific labels over generic; benchmark labeling artifact | Both teams |

### 1.3 What Was Tried and Rejected

| Attempt | Finding |
|---------|---------|
| Hash-based encoding (`text_to_multivector`) | 0% semantic accuracy. Maps word identity, not meaning. Deprecated. |
| Algebraic transformation (A⁻¹ * B) | Captures algebraic difference, not functional relationship. Replaced by WuXing cycle. |
| Single-path classifier (`from_pair`) | Only 4 of 8 labels reachable. Replaced by `from_pair_multi`. |
| Control-path trigram overrides | Control is Gen's domain; overriding reduced constraining accuracy. Reverted. |
| Pair-alignment confidence blending | Kills valid orthogonal-blade classifications. Replaced by sharpness gate. |
| A*B geometric product as primary classifier | 5.7% — worse than random. Rejected by parallel workstream. |
| Naive refinement on all data | 56% includes overfitting. Proper CV shows 17.5% calibrated. |

---

## 2. Architecture Decisions

### AD-1: Multi-Hypothesis over Priority Chain
`from_pair_multi` scores all 8 labels simultaneously with weighted evidence. Enables honest margin confidence and weight optimization. **Proven:** 45-52% test accuracy, all 8 labels predicted.

### AD-2: Sharpness Gate over Confidence Blending
Gate by encoding concentration (`encoding_sharpness > 0.25`) rather than blending with geometric alignment. **Proven:** 93.5% of random pairs → 0.0 confidence.

### AD-3: WuXingIndex over ANN
Phase-bucketed retrieval with domain filtering. Zero accuracy loss, 1.4–5.8x speedup. ANN not needed at practical scales (10K-50K concepts). **Proven:** 500ns per query, 64 bytes/concept.

### AD-4: Generate-Only Trigram Overrides
Trigram-specific rules for generate relationships (Zhen→causal, Kan→transmissive, Li→clarifying, Xun→influential). Control relationships remain generic (Gen's domain). **Proven:** Recovered constraining accuracy while adding fine-grained labels.

### AD-5: Separation of Concern
Classification (`relation_type.rs`, `diagnostic.rs`) owned by encoding-quality team. Retrieval (`index.rs`, `semantics.rs` spectrum/evolve) owned by retrieval team. **Proven:** 141 tests pass, no merge conflicts.

---

## 3. Remaining Roadmap (Priority-Ordered)

### Immediate: Documentation & Distribution

| Task | Impact |
|------|--------|
| Publish crates.io updates (core, mcp, cli) | Makes current version accessible |
| Write preprint / technical report | Academic credibility |
| Build demo: explore a real open-source codebase | User adoption |
| Python wheels via maturin + PyPI | Python ecosystem access |

### Blocked by Encoding Quality

| Task | Blocked By |
|------|-----------|
| Improve same-role R@1 beyond 42% | Need distinctive secondary coefficient patterns |
| Improve relation classification beyond 52% | Need WuXing cycle alignment (f1 > 0) |
| Enable fingerprint_similarity to work | Need structured secondary coefficients |
| Semantic cross-role retrieval | Need relational encoding protocol |

### Future (Post-Encoding)

| Task | Prerequisite |
|------|-------------|
| Cl(4)/Cl(5) higher-dimensional GA | Encoding quality must improve first |
| ANN retrieval (million-scale) | Not needed yet; WuXingIndex sufficient |
| Real-world benchmark dataset with LLM-cross-validated labels | Independent workstream |
| Streaming/incremental encoding | Encoding protocol v3 |

---

## 4. Success Metrics

| Metric | Current | Target | Prerequisite |
|--------|:------:|:------:|-------------|
| Same-role R@1 (same domain) | 42% | 70% | Encoding distinctiveness |
| Same-role R@10 (same domain) | 100% | — | Already met |
| Relation classification test | 45-52% | 65% | WuXing cycle alignment |
| WuXing cycle signal (f1) | 0.0 (v1), 0.6 (v3) | >0.5 sustained | Multiple encoding per concept |
| Token savings (200 queries) | 219x | — | Already met |
| Multi-hop stability | 100-hop, zero drift | — | Already met |
| Encoding stability | 99.8% | — | Already met |
| Random noise gated | 93.5% → 0.0 | — | Already met |
| Test coverage | 141 unit + 11 bench | 160+ | Ongoing |

---

## 5. Risk Register

| Risk | Probability | Impact | Mitigation |
|------|:----------:|:------:|-----------|
| Encoding quality ceiling cannot be raised | Medium | High | Accept as honest limitation; reposition as role-index, not classifier |
| Market doesn't adopt Bagua/WuXing framework | Medium | Medium | Lead with Cl(3) math; Bagua is naming convention, not foundation |
| Parallel workstreams conflict | Low | Medium | Separation of concern documented; module ownership clear |
| LLM encoding quality varies across models | High | Medium | SKILL.md v2 protocol is model-agnostic; test across Claude, GPT-4 |
| 8 dimensions inherently insufficient | Medium | High | Cl(4)/Cl(5) planned; document current 8D ceiling honestly |

---

*This document supersedes previous roadmap.md and epics.md for strategic direction.*
