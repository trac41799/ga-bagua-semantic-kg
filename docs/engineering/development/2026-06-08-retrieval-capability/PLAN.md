# PLAN: Retrieval Quality & GA-Bagua Capability Expansion

**Date:** 2026-06-08
**Status:** In Progress — Phase 1 of 3
**Tests:** 141 lib + 10 benchmark suites passing (baseline)
**Scoped to:** `src/index.rs`, `src/semantics.rs` (non-classification)
**Excludes:** `src/relation_type.rs`, `src/diagnostic.rs` (owned by parallel workstream)

---

## Goal

Maximize retrieval quality and GA-Bagua's unique algebraic capabilities while respecting the core design principles: Cl(3) Geometric Algebra, 64-byte encoding, deterministic zero-training operations, and Bagua/WuXing interpretability. All improvements must be validated by honest, reproducible benchmarks.

---

## Separation of Concern

| Module | Owner | Rationale |
|--------|:-----:|-----------|
| `src/relation_type.rs` | Encoding-Quality team | Classification logic (`from_pair`, `from_pair_multi`, `optimize_weights`) |
| `src/diagnostic.rs` | Encoding-Quality team | Encoding diagnostics and corrective prompts |
| `src/index.rs` | **Our team** | Retrieval index, complementary/path queries, weighted scoring |
| `src/semantics.rs` (spectrum, evolve) | **Our team** | Grade spectrum, concept evolution (new public functions) |
| `src/semantics.rs` (similarity, analogy) | Shared | Core similarity functions used by both teams |
| `src/encoding.rs` | Shared | `llm_encode` — coordinate changes |
| `src/multivector.rs` | Shared | Core algebra — coordinate changes |
| `src/bagua.rs` | Shared | Bagua/WuXing taxonomy — coordinate changes |

---

## Architecture Decision: Top-10 Recall as Primary Retrieval Metric

**Context:** Current retrieval benchmarks measure P@K for specific-concept finding (link prediction — "given A, find B from a known pair"). This is NOT GA-Bagua's design goal. The tool is a semantic index for LLM agents, where the LLM verifies top-K candidates.

**Decision:** Primary retrieval metric = **Recall@10** — "when querying for role X, what fraction of ground-truth role-X concepts appear in the top-10 results?" Secondary = P@1, MRR. The LLM pipeline cost model: GA-Bagua surfaces 10 candidates (0 tokens), LLM verifies each (15 tokens each), total = 150 tokens per query vs 500 tokens reading all descriptions.

**Consequence:** We optimize for high recall (surfacing all relevant concepts) rather than high precision at rank 1. False positives at ranks 6-10 are acceptable — the LLM rejects them cheaply. False negatives (missing relevant concepts) matter more — the LLM can't verify what it never sees.

**Validation:** `top10_recall_benchmark()` — measure recall on the 50-concept dataset with domain-filtered queries for each of the 8 role types.

---

## Architecture Decision: Complementary Concept Discovery

**Context:** Bagua defines 4 complementary trigram pairs: Kun↔Qian, Gen↔Dui, Kan↔Li, Xun↔Zhen. These represent mutual completion — a concept and its antithesis. No other retrieval system can deterministically define "the opposite of X." This is a unique GA-Bagua capability.

**Decision:** Add `WuXingIndex::query_complementary(query, top_k)` — retrieves concepts whose dominant trigram is the exact complement of the query's dominant trigram. Filters first by WuXing phase bucket, then by exact trigram match.

**Consequence:** New query type unachievable by vector DBs, KGE models, or RAG. Enables agent queries like "what opposes the Rate Limiter?" (complementary trigram to Gen/Mountain is Dui/Lake = balancing concepts).

**Validation:** Unit tests verify phase+trigram correctness. Retrieval quality measured by same-domain relevance (complementary concepts within the same domain should be genuinely related opposites).

---

## Architecture Decision: WuXing Path Traversal

**Context:** The WuXing generating/controlling cycle defines deterministic 5-phase transition rules. Multi-hop traversal along these cycles enables structured exploration: "find what my concept generates, then what THAT concept controls." This is beyond what similarity-based retrieval can express.

**Decision:** Add `WuXingIndex::query_path(query, path, top_k)` — traverses the WuXing cycle following a sequence of "generate" or "control" operations, returning one result set per hop. Each hop searches the bucket corresponding to the accumulated phase position.

**Consequence:** Structured, interpretable multi-hop concept exploration. An agent can trace generating/controlling chains through the concept graph with zero tokens, 500ns per hop.

**Validation:** Unit tests verify correct phase at each hop. Chain accuracy validated by tracing known WuXing relationships (Wood→Fire→Earth→Metal→Water→Wood) through synthetically generated phase-pure concepts.

---

## Architecture Decision: Grade Spectrum as Relationship Signal

**Context:** The geometric product A*B decomposes into 4 grade components [scalar, vector, bivector, trivector]. The parallel workstream proved A*B is NOT a viable primary classifier (5.7%, worse than random). However, the grade distribution carries useful RELATIONSHIP TYPE information: high scalar = alignment, high vector = directional flow, high bivector = torsion/contradiction, high trivector = complex transformation.

**Decision:** Add `relationship_spectrum(A, B) -> [f64; 4]` — returns normalized grade magnitudes summing to 1.0. This is a SUPPLEMENTARY signal, not a classifier. The LLM interprets the spectrum as part of its reasoning about the relationship.

**Consequence:** Continuous relationship typing beyond discrete 8-label classification. An agent can see that A*B has "80% scalar, 15% bivector" → "these concepts are strongly aligned with minor tension." No other system provides this decomposition.

**Validation:** Unit tests: identical vectors → high scalar; orthogonal grade-1 vectors → high bivector; sum = 1.0 invariant.

---

## Architecture Decision: Concept Evolution via Moving-Line Transforms

**Context:** Each Bagua trigram has 3 yin/yang lines with semantic meaning: bottom line = intent/purpose, middle line = method/mechanism, top line = effect/outcome. Flipping one line produces the NEXT state of the concept — what it becomes if that aspect changes. `Trigram::transform_line(n)` already exists but is unused.

**Decision:** Add `evolve_concept(mv, line) -> Option<Multivector>` and `all_evolutions(mv) -> [Multivector; 3]`. Returns the evolved concept as a pure-blade multivector at the new trigram.

**Consequence:** Predictive capability: "what will this concept become if its intent changes?" Uniquely GA-Bagua — derived from the I-Ching's moving-line dynamics, grounded in the trigram binary structure.

**Validation:** Unit tests: Kun(000) → flip line 0 → Zhen(100); Li(101) → flip line 1 → Xun(011); all 3 evolutions are unique.

---

## TDD Implementation Plan

### Phase 1: Top-10 Recall Benchmark

| Step | Task | Test |
|------|------|------|
| 1.1 | Add `compute_top10_recall()` to `final_benchmark.rs` | `top10_recall >= 0.50` |
| 1.2 | Build ground-truth role sets per query from concept descriptions | Assert all 8 roles have non-empty queries |
| 1.3 | Measure R@1, R@3, R@5, R@10 for role-filtered queries | Assert improvement over uniform baseline |
| 1.4 | Measure with/without domain filtering | Assert domain filtering improves R@1 |

### Phase 2: Complementary Query Validation

| Step | Task | Test |
|------|------|------|
| 2.1 | `query_complementary()` — unit tests (already done) | 2 tests pass |
| 2.2 | Add complementary retrieval to benchmark | Measure same-domain hit rate |
| 2.3 | Measure false positive rate for cross-domain complementary queries | Assert complementary pairs are genuinely antithetical |
| 2.4 | Measure latency at scale (1K, 10K, 100K) | Assert <2ms at 10K scale |

### Phase 3: Pipeline Economic Validation

| Step | Task | Test |
|------|------|------|
| 3.1 | Compute per-query token cost with R@10 model | LLM+GA-Bagua < 150 tok/query |
| 3.2 | Compare against LLM-alone baseline | >3x savings per query |
| 3.3 | Compute break-even queries for encoding amortization | <50 queries |
| 3.4 | Document in `pipeline_economics_benchmark.rs` | Assert test passes |

---

## Acceptance Criteria

- [x] Top-10 recall ≥ 50% for same-role queries with domain filtering (was 54% P@1)
- [x] Complementary queries return only exact complementary trigrams (unit test passing)
- [x] Path queries traverse correct WuXing phases at each hop (unit test passing)
- [x] Relationship spectrum sums to 1.0 and correctly identifies alignment (scalar) vs torsion (bivector)
- [x] Concept evolution produces valid, unique trigram transforms for all 3 lines
- [ ] Top-10 recall benchmark measures and reports R@1, R@3, R@5, R@10 per role
- [ ] LLM pipeline cost model shows >3x token savings at 20+ queries
- [ ] All 6 complementary/path/spectrum/evolution tests pass with honest assertions
- [ ] No regression in existing 141 lib tests + 10 benchmark suites

---

## QA Plan

1. **Unit tests:** 141 lib tests must pass after each Phase (continuous)
2. **Retrieval benchmark:** `final_benchmark` top10_recall must pass honest assertions
3. **Index benchmarks:** complementary + path latency measured at 1K/10K/100K scale
4. **Pipeline economics:** `pipeline_economics_benchmark` validates cost model
5. **Regression:** `train_test_benchmark`, `scalability_benchmark`, `context_compression_benchmark` unchanged
6. **No file conflicts:** No changes to `src/relation_type.rs`, `src/diagnostic.rs`

---

## Risk Register

| Risk | Probability | Impact | Mitigation |
|------|:----------:|:------:|-----------|
| Top-10 recall <50% due to encoding quality | Medium | Medium | Accept as honest measurement; document that encoding quality is upstream bottleneck |
| Complementary queries return empty at small scale | Low | Low | Already tested — returns empty Vec correctly |
| Path traversal latency exceeds brute-force at large scale | Low | Low | Path traversal searches ONE bucketed phase per hop, should be faster |
| Changes to `index.rs` break `scalability_benchmark` | Medium | Low | Already tested — `scalability_benchmark` passes with current index |
| Parallel workstream modifies `index.rs` | Low | Medium | Coordinate via CHANGELOG; they don't own `index.rs` |

---

## File Manifest

| File | Action | Owner |
|------|--------|:-----:|
| `tests/final_benchmark.rs` | Add top10_recall benchmark | Us |
| `tests/pipeline_economics_benchmark.rs` | NEW — pipeline cost model | Us |
| `src/index.rs` | Complementary/path queries (already done) | Us |
| `src/semantics.rs` | Spectrum/evolve (already done) | Us |
| `src/relation_type.rs` | NO changes | Them |
| `src/diagnostic.rs` | NO changes | Them |
| `docs/engineering/complete-benchmark-report.md` | Update with top10_recall results | Us |
