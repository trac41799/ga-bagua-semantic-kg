# Phase 4: Benchmarks & Validation

**Date Range:** 2026-07-03 → 2026-07-11
**Status:** ⬜ Pending
**Epic:** Epic 6 — Benchmarking & Validation
**Depends On:** Phase 1-3 (Core + Bagua + Semantic Ops)

---

## Objective

Validate mathematical correctness and semantic value against established baselines. Execute the decision gate to determine production viability.

---

## Task Breakdown

### Day 1-2: Relation Classification Benchmark (July 3-4)

| Task | Est. | Status |
|------|------|--------|
| Select benchmark dataset (FB15k, WN18, or custom) | 2h | — |
| Implement data loader for benchmark triples | 3h | — |
| Implement Bagua tagging for benchmark relations | 3h | — |
| Run classification: assign trigram to each relation type | 2h | — |
| Compute accuracy vs. random baseline | 2h | — |
| Generate confusion matrix by trigram category | 2h | — |
| Write analysis report | 2h | — |

**Deliverable:** Relation classification benchmark results

### Day 3-4: Analogical Reasoning Benchmark (July 5-6)

| Task | Est. | Status |
|------|------|--------|
| Adapt Google word analogy test for GA | 3h | — |
| Map word vectors to multivectors (embedding step) | 4h | — |
| Run analogy computation on test set | 2h | — |
| Compute accuracy: exact match and top-k | 2h | — |
| Compare vs. standard vector arithmetic baseline | 2h | — |
| Write analysis report | 2h | — |

**Deliverable:** Analogical reasoning benchmark results

### Day 5-7: KG Link Prediction Benchmark (July 7-9)

| Task | Est. | Status |
|------|------|--------|
| Reproduce GeomE benchmark conditions (Xu et al., 2020) | 4h | — |
| Implement entity/relation multivector initialization | 3h | — |
| Implement training loop (margin-based loss) | 4h | — |
| Implement evaluation: MRR, Hits@1, Hits@3, Hits@10 | 3h | — |
| Run on FB15k-237 and WN18-RR | 4h | — |
| Compare Bagua-tagged vs. untagged multivectors | 3h | — |

**Deliverable:** KG link prediction benchmark results

### Day 8-9: Performance Benchmarks (July 10-11)

| Task | Est. | Status |
|------|------|--------|
| Run criterion benchmarks (product, rotor) | 2h | — |
| Profile memory usage per multivector | 2h | — |
| Benchmark batch operations (1K, 10K, 100K multivectors) | 3h | — |
| Measure parallel scaling (1, 2, 4, 8 cores) | 3h | — |
| Document performance characteristics | 2h | — |

**Deliverable:** Performance benchmark report

### Day 10: Decision Gate (July 11)

| Task | Est. | Status |
|------|------|--------|
| Compile all benchmark results | 2h | — |
| Evaluate against decision gate criteria | 2h | — |
| Write final assessment report | 3h | — |
| Determine next steps (production / research / shelve) | 1h | — |

**Deliverable:** Decision gate evaluation and next steps

---

## Benchmark Datasets

| Dataset | Type | Size | Use Case |
|---------|------|------|----------|
| FB15k-237 | Freebase subset | 14,541 entities, 237 relations, 272K triples | Link prediction |
| WN18-RR | WordNet subset | 40,943 entities, 11 relations, 86K triples | Link prediction |
| Google Analogy | Word pairs | 19,544 questions | Analogical reasoning |
| Custom Bagua | Trigram-tagged | TBD | Relation classification |

---

## Decision Gate Criteria

| Metric | Baseline | Threshold | Action |
|--------|----------|-----------|--------|
| Relation classification accuracy | Random (12.5%) + trigram majority-class | > baseline + 5% | ✅ Promote to production |
| Analogy accuracy (top-1) | Word2Vec vector arithmetic | > baseline + 5% | ✅ Promote to production |
| Link prediction MRR | GeomE reported MRR on same dataset | > GeomE MRR | ✅ Promote to production |
| Link prediction Hits@10 | GeomE reported Hits@10 on same dataset | > GeomE Hits@10 | ✅ Promote to production |
| Any 2 of above | — | > baseline + 5% | ✅ Promote to production |
| All within ±5% of baseline | — | — | 🟡 Maintain as research project |
| Any below baseline - 5% | — | — | 🔴 Document findings; shelve |

---

## Expected Outcomes

### Best Case (Promote)

- Bagua tagging improves or matches GeomE on link prediction
- Interpretability is demonstrably higher (human evaluation)
- Analogy computation works on standard benchmarks
- **Action:** Integrate into ACC knowledge graph layer

### Middle Case (Research)

- Bagua tagging matches baseline (no improvement, no regression)
- Interpretability is the primary value
- Analogy computation shows promise but needs refinement
- **Action:** Continue as research project; optional ACC integration

### Worst Case (Shelve)

- Bagua tagging degrades performance significantly
- Mathematical mapping doesn't translate to practical value
- **Action:** Document negative result; publish honest assessment; shelve

---

## Reporting

### Final Report Structure

1. **Executive Summary** — one paragraph verdict
2. **Benchmark Results** — tables with metrics
3. **Analysis** — what worked, what didn't, why
4. **Comparison** — vs. GeomE, vs. vector baselines
5. **Recommendation** — promote / research / shelve
6. **Appendix** — raw data, configuration, reproducibility instructions
