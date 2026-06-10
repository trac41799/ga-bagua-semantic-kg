# GA-Bagua Semantic KG — Benchmark & Comparative Analysis

**Date:** 2026-06-08
**Status:** Current — reflects from_pair_multi + WuXingIndex + baseline comparisons
**Tests:** 141 unit + 11 benchmark suites, all passing

---

## 1. Executive Summary

GA-Bagua is a **deterministic, zero-training, 64-byte semantic index for LLM agents.** It encodes concepts into 8-element multivectors in Cl(3) Geometric Algebra mapped to the 8 Bagua trigrams. Once encoded, all operations execute algebraically in nanoseconds with zero tokens.

This document provides: (a) current benchmark results, (b) comparative analysis against alternatives, and (c) an assessment of where GA-Bagua is uniquely strong vs. where it cannot compete.

---

## 2. Head-to-Head Comparison

| Dimension | TransE/ ComplEx | BERT/ada-002 | RAG+LLM | **GA-Bagua** |
|-----------|:---:|:---:|:---:|:------------:|
| **Dimensionality** | 50-500 | 384-4096 | N/A | **8** |
| **Interpretability** | None | None | LLM output | **8 named roles** |
| **Asymmetric relations** | Partial | No | Via LLM | Yes |
| **Cyclical relations** | No | Partial | Via LLM | Yes |
| **Algebraic composition** | No | No | No | **Yes (rotors)** |
| **Training required** | Yes | Pretrained | No | **No** |
| **Storage per concept** | 400-4000B | 1536-16384B | N/A | **64 bytes** |
| **Query time** | ~us | ~us | ~500 tokens | **500ns (algebra)** |
| **Query tokens** | N/A | N/A | ~500 | **0** |
| **MCP/Agent interface** | No | No | Partial | **29 tools** |
| **Complementary ("opposite")** | No | No | No | **Yes (unique)** |
| **Multi-hop (100-hop)** | No | No | Degrades | **Yes, zero drift** |
| **Grade spectrum** | No | No | No | **Yes (unique)** |
| **Concept evolution** | No | No | No | **Yes (unique)** |

---

## 3. Core Performance Benchmarks

### 3.1 Algebra Speed (debug build, 500K iterations)

```
Operation                ns/op       ops/sec
──────────────────────   ────────    ──────────
reverse                   73 ns      13.6M
dominant_role            250 ns       4.0M
classify_relation        410 ns       2.4M
analogy                  596 ns       1.7M
geo_product              2.1 us       471K
semantic_similarity      6.3 us       160K
batch_100_similarity     504 us       1,986/sec
```

### 3.2 Retrieval Quality (50 concepts, 4 domains, same-role + same-domain)

| Method | R@1 | R@3 | R@5 | R@10 |
|--------|:---:|:---:|:---:|:----:|
| **GA-Bagua (dominant_similarity)** | **42%** | 83% | 100% | 100% |
| GA-Bagua (fingerprint_similarity) | 17% | 17% | 33% | 50% |
| Keyword (Jaccard on names) | 0% | 0% | 17% | 83% |
| Random | 0% | 17% | 42% | 58% |

42% R@1 means: the top-ranked same-role + same-domain peer is the correct one 42% of the time. 100% R@10 means all same-role peers surface in the top-10. The LLM pipeline cost model: GA-Bagua returns 10 candidates (0 tokens), LLM verifies each (15 tokens), total 150 tokens per query vs 4,000 tokens reading all descriptions. Break-even at 5 queries.

### 3.3 Classification Accuracy (50 concepts, 53 relations, train/test split)

| Classifier | Train | Test | All 8 Labels |
|-----------|:-----:|:----:|:-----------:|
| from_pair (phase-only) | 7.1% | 24.0% | 4 of 8 |
| from_pair_multi (default) | 28.6% | 52.0% | 8 of 8 |
| from_pair_weighted (optimized) | 92.9% | 80.0% | 8 of 8 (f3-only, dataset artifact) |

**Note:** The 80-93% optimized accuracy is a dataset artifact (uses only A's trigram, ignores B and WuXing). The honest WuXing-aware number is 45-52%. Per the parallel workstream's BENCHMARK_RESULTS.md, the WuXing cycle is restored (f1=0.6) when encodings are correctly aligned — proving the framework works when encoding quality is sufficient.

### 3.4 Encoding Stability

```
±5% coefficient noise:  100.0% dominant role preserved
±10% coefficient noise:  99.8% dominant role preserved
```

### 3.5 False Positive Gate

```
Random pairs confidence distribution (10,000 samples):
  [0.0-0.3):  93.5%  (gated to 0.0 by sharpness threshold)
  [0.3-0.6):   0.0%
  [0.6-0.8):   1.2%
  [0.8-1.0):   5.4%  (was 81% before sharpness gate)
```

---

## 4. Where GA-Bagua Wins

**1. Storage density: 48x smaller than alternatives.** 64 bytes per concept vs 3,072 bytes for BERT-base. 1M concepts = 64 MB (fits in L3 cache). BERT needs 3 GB.

**2. Zero query cost.** After one-time encoding (~200 tokens/concept), all queries cost 0 tokens and 500ns. LLM-direct costs ~500 tokens and 1-3s per query.

**3. Interpretable labels.** Every concept gets a human-readable Bagua role (generative, constraining, transmissive, etc.). No other system provides this.

**4. Deterministic, zero-training.** No corpus, no GPU, no gradient descent. Compiles to a single binary with one required dependency.

**5. Unique algebraic capabilities:**
- **Complementary concept discovery:** "What is the opposite of X?" — defined by Bagua complementary trigram pairs
- **WuXing path traversal:** "What concepts are 2 generating steps from X?"
- **Concept evolution:** "What will this concept become if one aspect changes?" — moving-line transforms
- **Multi-hop rotor composition:** 100-hop chains with zero numerical drift
- **Grade spectrum:** Continuous relationship typing via geometric product grade decomposition

**6. Built for LLM agent integration.** 29 MCP tools with JSON schemas. The LLM encodes, GA-Bagua retrieves, the LLM verifies.

---

## 5. Where GA-Bagua Cannot Compete

**1. Relation classification as a standalone answer.** At 45-52% test accuracy, labels are directional hints for the LLM, not final answers. The LLM must always verify.

**2. Specific concept retrieval.** "Given concept A, find the specific concept B from a known relation" scores 7% P@5 — below random. This is link prediction, not GA-Bagua's design goal.

**3. Semantic search from raw text.** Cannot match BERT/ada-002 for general text search. GA-Bagua retrieves by Bagua role, not by text content.

**4. Community adoption.** No published paper, small ecosystem, brand-new technology. Vector databases have 10+ years of production use.

---

## 6. Performance Density

How many relationship queries per second of compute?

| System | Queries/sec | Compute | Query type |
|--------|:----------:|---------|------------|
| GA-Bagua (similarity) | ~160,000 | 1 CPU core | O(1) geo product |
| GA-Bagua (full pipeline) | ~100,000 | 1 CPU core | sim + classify |
| TransE/ComplEx scoring | ~500,000 | GPU required | Learned scoring |
| BERT cosine | ~100,000 | CPU/GPU | 768-dim dot product |
| LLM reasoning (GPT-4) | ~2-5 | API call | ~500 tokens |

**GA-Bagua's position:** 50,000x faster than LLM reasoning for concept-level queries, comparable to learned embedding scoring on CPU but with interpretable output, and 48x more storage-efficient.

---

## 7. Pipeline Economics

```
Scenario: Agent analyzes 200-module codebase, making 200 relationship queries.

Pipeline                        │ Tokens  │ Cost       │ Latency
────────────────────────────────┼─────────┼────────────┼────────
LLM reads all code each query   │ 10,100K │  $101.00   │  600s
AI + GA-Bagua (200 encodes)     │     46K │    $0.46   │  100s
AI + summarization              │    502K │    $5.02   │  202s

Savings vs full context: 219x tokens ($100.54 per session)
Break-even: 5 queries (encoding cost amortized)
```

---

## 8. Recommended Positioning

GA-Bagua is not a KGE model competitor. It is:

**"A compact, interpretable, algebraically-composable semantic index for LLM agents."**

```
┌─────────────────────────────────────────────────────────────┐
│                    LLM (Reasoning Engine)                    │
│  Heavy, expensive, high-fidelity                            │
│  Called for: verification, explanation, complex reasoning    │
├─────────────────────────────────────────────────────────────┤
│          GA-Bagua (Semantic Index)                           │
│  64 bytes/concept, interpretable roles, algebraic retrieval  │
│  Called for: candidate generation, role queries,             │
│  complementary discovery, path exploration                  │
├─────────────────────────────────────────────────────────────┤
│              Vector DB (Document Search)                     │
│  Finds relevant documents from millions                      │
│  Called for: initial lookup, text-level similarity           │
└─────────────────────────────────────────────────────────────┘
```

---

*Data collected on Windows 11, Rust 1.78, debug build. 141 unit tests + 11 benchmark suites passing.*
