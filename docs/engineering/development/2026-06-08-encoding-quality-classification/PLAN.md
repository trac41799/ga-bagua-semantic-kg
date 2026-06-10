# PLAN: Retrieval & Analogy Benchmarks

**Date:** 2026-06-09

---

## 1. Dense Retrieval Benchmark

### What
Compare GA-Bagua's `dominant_similarity()` against standard baselines for same-role concept retrieval across the full 50-concept dataset.

### Metrics
- Precision@K (K=1,3,5): among top K, how many share the same dominant role?
- Mean Reciprocal Rank: rank of the first same-role peer
- Query time: ns per comparison

### Baselines
| Method | Mechanism |
|--------|-----------|
| GA-Bagua `dominant_similarity()` | Sign-aware product-of-magnitudes in 8D |
| GA-Bagua `semantic_similarity()` | Geometric product scalar normalized |
| Cosine (8D raw coefs) | Standard dot product on coefficient vectors |
| Euclidean (8D raw coefs) | L2 distance |
| Random | Uniform shuffle |

### Dataset
All 50 concepts from benchmark_dataset.json, queried with their dominant roles as ground truth categories (4 categories: constraining 12, transmissive 7, clarifying 5, balancing/influential/generative/receptive 4-7 each).

### Test Structure
- For each concept as query, compute similarity to all 49 others
- Rank by similarity score descending
- Measure P@K and MRR for same-role peers
- Repeat for each baseline

### Acceptance
- GA-Bagua dominant_similarity P@5 > random
- At least one GA-Bagua method outperforms Euclidean/Cosine
- Honest reporting if GA-Bagua loses

---

## 2. Analogy Benchmark

### What
Compare GA-Bagua's `analogy()` against standard baselines on the existing 5 analogy quadruplets, then expand to 20+ new analogies across domains.

### Metrics
- Exact match: does `analogy(A,B,C)` produce the correct role for D?
- Confidence: how strong is the algebraic signal?

### Baselines
| Method | Mechanism |
|--------|-----------|
| GA-Bagua `analogy()` | WuXing cycle + trigram-position-aware selection |
| 3CosAdd (Word2Vec) | argmax(cos(D, B-A+C)) — algebraic composition |
| LLM analogy | "A is to B as C is to?" — prompted directly |
| Random | Uniform among 8 roles |

### Dataset
20 analogy quadruplets across 3 domains:
- Software concepts: RateLimiter:AuthProvider::FeatureFlag:DeprecationPolicy (constraining:constraining::influential:influential)
- Business concepts: Pipeline:Revenue::Onboarding:Productivity
- Biology concepts: Mutation:Variation::Mutation:NaturalSelection (causal:receptive::causal:constraining)

### Test Structure
- For each (A,B,C) triple, compute D = analogy(A,B,C)
- Compare D's dominant role to expected
- Report accuracy and confidence

### Acceptance
- GA-Bagua analogy accuracy > random (12.5%)
- Comparison to 3CosAdd baseline (if implementable in Rust)
- Honest reporting

---

## Files

| File | Purpose |
|------|---------|
| `tests/retrieval_benchmark.rs` | Dense retrieval comparison |
| `tests/analogy_benchmark.rs` | Analogy comparison |

## Implementation Order

1. Retrieval benchmark (simpler, builds on existing infrastructure)
2. Analogy benchmark (builds on existing analogy function)
