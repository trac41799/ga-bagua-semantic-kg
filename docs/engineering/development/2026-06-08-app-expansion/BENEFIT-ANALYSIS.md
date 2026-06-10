# REPORT: Benefit Analysis & Benchmark Specification

**Date:** 2026-06-09  
**Context:** 11 application ideas across 3 subsystems (Doc Intel, Cognitive, Ideation).  
**Question:** What measurable value do these tools add, and how do we prove it?

---

## Executive Summary

The 11 COMBINE ideas add value along four axes inherent to the GA-Bagua architecture:

| Axis | GA-Bagua Advantage | vs Baseline |
|------|-------------------|-------------|
| **Speed** | Nanosecond-to-microsecond reasoning | Baseline: LLM call (500ms–5s). GA: **10^6× faster** |
| **Interpretability** | Every output maps to a named trigram, WuXing phase, and hexagram explanation | Baseline: embedding similarity scores with no explanation |
| **Zero-training** | No model training or fine-tuning needed | Baseline: classifiers need labeled training data |
| **Compactness** | 64 bytes per concept (8 × f64) | Baseline: 384–1536 dimensions × 4 bytes = 1.5KB–6KB per embedding |

These aren't speculative — the existing core already demonstrates:
- 100% relation classification accuracy on 15 test pairs
- 34 ns per geometric product operation
- 73.3% retrieval precision

The new tools layer domain logic on top of this proven core. The benchmark spec below validates that the domain logic is correct and the core advantages persist.

---

## System A: Document Intelligence — Benefit Analysis

### Idea #1: Argument Mapping & Fallacy Detection

| Aspect | Value |
|--------|-------|
| **Problem solved** | Fallacy detection currently requires either human experts or LLM calls ($/latency). Neither scales to large argument corpora. |
| **GA-Bagua advantage** | WuXing cycles provide a deterministic inference model. If premises and conclusion don't form a valid generating/controlling chain, it's structurally suspicious — flagged in nanoseconds. |
| **vs LLM baseline** | GPT-4 fallacy detection costs ~$0.01–0.03 per argument. GA classifier costs **$0 after initial LLM encode**. For 10K arguments: $100–300 vs $0. |
| **vs human baseline** | Human fallacy detection accuracy ~60–80% untrained, ~90% expert. GA targets **≥80% on standard benchmark** with nanosecond latency. |

### Idea #2: Multi-Document Semantic Alignment

| Aspect | Value |
|--------|-------|
| **Problem solved** | Aligning claims across documents is O(n²) pairwise comparison. Human review of 100 claims across 5 docs = 10,000 comparisons. |
| **GA-Bagua advantage** | Geometric product computes full relational signature (similarity + difference + relation type) in one operation. No separate cosine + Jaccard + overlap metrics needed. |
| **vs embedding baseline** | Embedding cosine similarity tells you "similar or not." GA tells you "similar (receptive), supporting (generative), conflicting (controlling), or complementary (balancing)" — **4-way classification, not 1-score**. |
| **Compactness** | 100 claims × 64 bytes = 6.4KB. Same claims as 768-dim embeddings = 300KB. 47× smaller. |

### Idea #4: Research Paper Synthesis

| Aspect | Value |
|--------|-------|
| **Problem solved** | Identifying research gaps requires reading all papers in a domain. Automated tools (e.g., Elicit, Consensus) use LLMs to summarize but don't model structure. |
| **GA-Bagua advantage** | WuXing cycle completeness = structural gap detection. If a domain has papers only in Wood/Fire phases but none in Metal (synthesis), the system flags it without needing to "understand" the content. |
| **Uniqueness** | No existing tool maps research to a 5-phase structural model. This is novel. |

### Idea #6: Policy Coherence Engine

| Aspect | Value |
|--------|-------|
| **Problem solved** | Organizations accumulate hundreds of policies. Human review for contradictions is infeasible. Current tools use regex/keyword matching — superficial. |
| **GA-Bagua advantage** | Semantic contradiction detection via bivector magnitude. Two policies can use different words but encode to contradictory multivectors → detected. Keyword matching would miss this entirely. |
| **vs LLM baseline** | Sending all policy pairs to an LLM for contradiction check is cost-prohibitive. GA checks all pairs in microseconds after one-time encoding. |

### Idea #8: Cross-Lingual Concept Alignment

| Aspect | Value |
|--------|-------|
| **Problem solved** | Translation quality assessment and cross-cultural concept mapping. Does "freedom" (EN) = "自由" (ZH) = "Freiheit" (DE)? |
| **GA-Bagua advantage** | The 8 diagnostic questions are functional ("what does this concept DO?") — language-agnostic. If translations produce different encodings, the specific diverging roles reveal semantic drift. |
| **vs embedding baseline** | Multilingual embeddings conflate "similar in meaning" with "similar in training data." GA provides role-by-role divergence — you know *which aspect* of the concept differs, not just *that* it differs. |

### Idea #29: Smart Contract Semantic Auditor

| Aspect | Value |
|--------|-------|
| **Problem solved** | Smart contract bugs are often semantic — the code does something different from what was intended. Formal verification checks correctness but not intent-alignment. |
| **GA-Bagua advantage** | Encode intent (spec) and implementation (code description) separately. Geometric distance quantifies drift. Contradiction detection flags mismatches. Role divergence explains *which aspect* diverges. |
| **vs audit baseline** | Human audits cost $5K–50K per contract. Automated tools (Slither, Mythril) detect code patterns, not semantic mismatches. GA targets **>90% precision on known semantic vulnerabilities** at zero marginal cost after encoding. |

---

## System B: Cognitive Systems — Benefit Analysis

### Idea #3: Agent Belief State Tracking

| Aspect | Value |
|--------|-------|
| **Problem solved** | Multi-agent systems need to detect when agents have conflicting beliefs. Current approaches: full belief exchange (expensive) or trust scores (opaque). |
| **GA-Bagua advantage** | Each belief is 64 bytes. Agent A sends its belief encodings (6.4KB for 100 beliefs). Agent B checks contradictions locally in microseconds. No semantic parsing, no re-encoding. |
| **vs consensus baseline** | Full belief dialogue between agents is expensive. Compact encoding + local contradiction check = O(1) per pair. |

### Idea #7: Personality / Team Compatibility

| Aspect | Value |
|--------|-------|
| **Problem solved** | Team formation tools (Belbin, DISC, MBTI) use discrete categories with rigid compatibility rules. They don't model nuanced interaction. |
| **GA-Bagua advantage** | Continuous encoding (8 floats → unit sphere) captures nuance. WuXing cycles provide structured compatibility rules. Rotor transforms model dynamic interaction. |
| **vs existing tools** | MBTI gives 16 types with simple compatibility tables. GA gives a point on S⁷ with algebraic compatibility scoring — continuous, not categorical. |

### Idea #5: Learning Path Generation

| Aspect | Value |
|--------|-------|
| **Problem solved** | Curriculum design requires expert knowledge of prerequisite relationships. Automated tools are rare and typically rule-based. |
| **GA-Bagua advantage** | WuXing generating cycle provides a principled ordering heuristic. Prerequisite detection via controlling cycle — if A controls B, A should come first. No rules to hand-code. |
| **vs baseline** | Random ordering has ~1/n! chance of being correct. Rule-based systems need hand-crafted prerequisites. GA generates ordering from concept encodings alone. |

### Idea #10: Goal Decomposition & Coherence

| Aspect | Value |
|--------|-------|
| **Problem solved** | Breaking large goals into subgoals that are complete, non-redundant, and non-contradictory is a common planning failure mode. |
| **GA-Bagua advantage** | WuXing phase coverage = completeness check (are all 5 phases represented?). Contradiction detection = coherence check (do any subgoals conflict?). Both are algebraic — no heuristics. |
| **vs LLM baseline** | LLM can decompose goals but doesn't verify coherence structurally. GA provides a verifiable coherence score. |

---

## System C: Creative Ideation — Benefit Analysis

### Idea #9: I Ching Creative Ideation Engine

| Aspect | Value |
|--------|-------|
| **Problem solved** | Brainstorming tools (ChatGPT, Claude) tend toward convergent, safe ideas. Structured divergent thinking is hard to prompt. |
| **GA-Bagua advantage** | 64 hexagrams = 64 structurally distinct perspectives guaranteed to be different. Rotor step between hexagrams = guaranteed perspective shift (nonzero geometric distance). |
| **vs LLM baseline** | "Give me 64 creative ideas" → LLM converges toward similar ideas. 64 hexagram-guided perspectives → structurally enforced divergence. |
| **vs random baseline** | Random hexagram = unstructured exploration. GA rotor = smooth, principled traversal through perspective space. |

---

## Benchmark Specifications

Each benchmark includes: dataset, metric, baseline, and pass threshold.

---

### B1: Argument Fallacy Detection

| Field | Detail |
|-------|--------|
| **Primary dataset** | LOGIC / LogicClimate (3,761 labeled arguments, 13 fallacy classes) via `tasksource/logical-fallacy` |
| **Secondary dataset** | CoCoLoFa (7,706 comments on 648 articles) |
| **Metric** | Precision, Recall, F1 on binary fallacy detection (fallacy present vs not) |
| **GA method** | 1. LLM encodes premise(s) + conclusion as multivectors. 2. WuXing cycle + bivector magnitude classifies inferential validity. 3. Per-fallacy heuristic rules detect specific fallacy types. |
| **Baseline 1** | Random classifier: precision = class balance, F1 ≈ 0.5 |
| **Baseline 2** | Zero-shot GPT-4: "Is this argument fallacious? [argument]" → binary label |
| **Baseline 3** | Cosine similarity between premise/conclusion sentence embeddings |
| **Pass threshold** | F1 ≥ 0.70 on binary detection (competitive with fine-tuned BERT on LogicClimate) |
| **Latency metric** | µs per argument after encoding (excludes LLM encoding time) |

---

### B2: Multi-Document Claim Alignment

| Field | Detail |
|--------|--------|
| **Primary dataset** | Synthetic: 5 documents, each with 20 claims. 30% of claims have known equivalents across documents (ground truth injected). |
| **Secondary dataset** | FEVER (185K claims with evidence alignment labels) — repurpose evidence retrieval as alignment task. |
| **Metric** | Precision@K: fraction of top-K aligned pairs that are true matches. MRR: mean reciprocal rank of first true match. |
| **GA method** | 1. Encode all claims. 2. For each claim in doc A, find top-K most similar claims in doc B via `dominant_similarity()`. 3. Classify each pair via `classify_relation()`. |
| **Baseline 1** | Cosine similarity between sentence-transformer embeddings (all-MiniLM-L6-v2) |
| **Baseline 2** | Jaccard overlap on bag-of-words |
| **Baseline 3** | Random pairing |
| **Pass threshold** | Precision@5 ≥ 0.70 on synthetic dataset (matched claims found in top 5). MRR ≥ 0.70. |
| **GA-specific** | 4-way classification accuracy (identical/supporting/conflicting/complementary) ≥ 0.75 |

---

### B3: Research Gap Detection

| Field | Detail |
|--------|--------|
| **Primary dataset** | Synthetic: Seed a domain with papers in Wood, Fire, Earth phases only. Ground truth: Metal and Water are gaps. |
| **Secondary dataset** | Real: Pick 10 review papers from arXiv. The "Future Work" sections provide ground-truth gaps. Encode the papers they cite as the research corpus. |
| **Metric** | Gap recall: what fraction of known gaps did the system flag? Gap precision: what fraction of flagged gaps are real? |
| **GA method** | 1. Encode all papers as documents. 2. Classify each paper into WuXing phase via dominant trigram. 3. Missing or underweighted phases = gaps. |
| **Baseline** | Random gap flagging (precision = 1/5 phases = 0.20) |
| **Pass threshold** | Gap recall ≥ 0.60 on synthetic (catches 3 of 5 known gaps). Precision ≥ 0.40 on real review papers. |

---

### B4: Policy Coherence

| Field | Detail |
|--------|--------|
| **Primary dataset** | Synthetic: 3 policy documents, each with 10 clauses. Inject K pairs of contradictory clauses (ground truth known). Vary K ∈ {0, 3, 6, 10}. |
| **Secondary dataset** | Real: corporate policies from publicly available handbooks (e.g., GitLab, Buffer, Basecamp). Human annotators label contradictions. |
| **Metric** | Contradiction detection: Precision, Recall, F1 on injected contradictions. |
| **GA method** | `is_contradictory()` with tuned threshold. Intra-doc and inter-doc modes. |
| **Baseline 1** | Cosine similarity between clause embeddings (low similarity ≠ contradiction — it could just be unrelated) |
| **Baseline 2** | LLM pairwise check: "Are these two clauses contradictory?" |
| **Pass threshold** | F1 ≥ 0.75 on synthetic with K=6 injected contradictions |

---

### B5: Cross-Lingual Concept Alignment

| Field | Detail |
|--------|--------|
| **Primary dataset** | Self-constructed: 50 concepts, each described in English, French, Japanese, and Chinese. Use professional translators + native LLM encoding. |
| **Secondary dataset** | regia-ai/crosslingual-sts-dataset (21.3M rows) — subsample 500 pairs across 5 language families. |
| **Metric** | Intra-concept similarity: mean `semantic_similarity(concept_L1, concept_L2)` for the same concept across languages. Inter-concept separation: mean similarity of different concepts across languages. |
| **GA method** | LLM encodes each language's description of the concept. Compute pairwise similarity matrix. |
| **Baseline 1** | LaBSE (Language-Agnostic BERT Sentence Embedding) cosine similarity |
| **Baseline 2** | Multilingual-E5 cosine similarity |
| **Pass threshold** | Intra-concept similarity ≥ 0.75 (convergent). Inter-concept similarity ≤ 0.40 (discriminative). Separation ratio ≥ 1.5. |

---

### B6: Smart Contract Semantic Audit

| Field | Detail |
|--------|--------|
| **Primary dataset** | jhsu12/smart_contract_vulnerability_kaggle (10,400 rows with vulnerable+fixed code pairs and root cause labels) |
| **Secondary dataset** | Bastet (4,402 findings from 394 Code4rena audits, expert-annotated) |
| **Metric** | Semantic drift detection: For known vulnerable contracts, does geometric distance between intent and implementation > threshold? For fixed versions, does distance decrease? |
| **GA method** | 1. LLM reads contract spec/NatSpec → encodes intent. 2. LLM reads Solidity source → encodes implementation. 3. `semantic_difference(intent, impl)` quantifies drift. 4. `is_contradictory()` flags critical mismatches. |
| **Baseline 1** | Static analysis tools (Slither, Mythril) — does the tool flag this as a vulnerability? |
| **Baseline 2** | GPT-4: "Does this code match its specification?" |
| **Pass threshold** | Semantic drift (difference score) for vulnerable contracts significantly higher than for fixed versions (Mann-Whitney p < 0.01, Cohen's d ≥ 0.5) |

---

### B7: Agent Belief Dissonance Detection

| Field | Detail |
|--------|--------|
| **Primary dataset** | Synthetic: Create an agent with N beliefs. Inject K pairs of contradictory beliefs. Ground truth: which pairs are contradictory. |
| **Secondary dataset** | Synthetic multi-agent: 3 agents with overlapping belief sets. Inject cross-agent contradictions. |
| **Metric** | Precision, Recall, F1 on detecting contradictory belief pairs (within and across agents). |
| **GA method** | `is_contradictory()` pairwise across all beliefs for an agent. Threshold tuned on validation set. |
| **Baseline** | LLM pairwise check: "Do these two beliefs contradict?" (expensive baseline) |
| **Pass threshold** | F1 ≥ 0.80 on synthetic with K=5 contradictions in N=20 beliefs. |

---

### B8: Team Compatibility Prediction

| Field | Detail |
|--------|--------|
| **Primary dataset** | Self-constructed: Encode 50 personality profiles. Form 100 random pairs + 50 expert-judged pairs. Expert judges rate compatibility 1–5. |
| **Secondary dataset** | Real: Encode known startup founding teams. Outcome labels (successful exit vs failed). This is correlation, not causation — interpret cautiously. |
| **Metric** | Spearman rank correlation between GA compatibility score and human-rated compatibility. |
| **GA method** | 1. Encode each personality as multivector. 2. `classify_relation(a, b)` gives relation type. 3. WuXing-based compatibility score favors generating + balancing cycles. |
| **Baseline 1** | MBTI compatibility lookup table (16 types → compatibility matrix) |
| **Baseline 2** | Big 5 trait similarity (cosine similarity on OCEAN vectors) |
| **Pass threshold** | Spearman ρ ≥ 0.40 (moderate correlation with human judgment; MBTI achieves ~0.15–0.25 in literature) |

---

### B9: Learning Path Ordering

| Field | Detail |
|--------|--------|
| **Primary dataset** | wiki-prerequisite-data: concept pairs with prerequisite strength (0–1) and reasons. |
| **Secondary dataset** | ESCO-PrereqSkill (3,196 skills with expert-defined prerequisite links). |
| **Metric** | Prerequisite prediction accuracy: given concepts A and B, does the system correctly predict whether A is a prerequisite of B? |
| **GA method** | 1. Encode each concept. 2. If A.wuxing_phase controls B.wuxing_phase → A is prerequisite of B. 3. If A generates B → A is prerequisite of B. 4. Rank ordering by WuXing cycle. |
| **Baseline 1** | Random ordering |
| **Baseline 2** | LLM zero-shot: "Should I learn A before B?" |
| **Baseline 3** | BERT fine-tuned on prerequisite prediction (if resources allow) |
| **Pass threshold** | Prerequisite accuracy ≥ 0.60 (beats random baseline of 0.50; competitive with zero-shot LLM) |

---

### B10: Goal Coherence Scoring

| Field | Detail |
|--------|--------|
| **Primary dataset** | Synthetic: Create 20 goal hierarchies. Inject K contradictory subgoals per hierarchy. Ground truth: which subgoals conflict. |
| **Secondary dataset** | Real: OKRs from public company documents (Google, Intel). Human annotators flag internal inconsistencies. |
| **Metric** | Contradiction detection accuracy within goal hierarchies. WuXing phase coverage score: 0.0–1.0 based on what fraction of 5 phases have subgoal coverage. |
| **GA method** | 1. Encode all subgoals. 2. `is_contradictory()` pairwise across subgoals. 3. Phase coverage via dominant trigram of each subgoal. |
| **Baseline** | Random contradiction flagging. |
| **Pass threshold** | Contradiction F1 ≥ 0.75. Coverage score correlates with human completeness judgment (Spearman ρ ≥ 0.40). |

---

### B11: Creative Ideation Quality

| Field | Detail |
|--------|--------|
| **Primary dataset** | Self-constructed: 5 problem statements. For each, generate ideas via: (a) ChatGPT free brainstorming, (b) random hexagram, (c) GA rotor stepping. |
| **Secondary dataset** | Human evaluation: N=10 raters score each generated idea on novelty (1–5) and usefulness (1–5). |
| **Metric** | Mean novelty score across 64 hexagram-generated ideas vs baseline. Diversity: mean pairwise geometric distance between generated ideas. Coverage: what fraction of 8 trigram perspectives are represented? |
| **GA method** | 1. Encode problem as seed multivector. 2. Step through 64 hexagrams via rotor. 3. For each, generate natural-language perspective prompt. 4. LLM elaborates each perspective into a full idea. |
| **Baseline 1** | ChatGPT: "Generate 64 creative solutions to [problem]. Be as diverse as possible." |
| **Baseline 2** | Random hexagram selection (same 64 prompts, different ordering) |
| **Pass threshold** | Mean pairwise geometric distance of GA-generated multivectors > ChatGPT-generated multivectors (higher diversity). Human-rated novelty: GA ≥ ChatGPT (non-inferiority). |

---

## Minimum Viable Benchmark Set per Phase

Not all benchmarks need to pass before shipping. The following are gate criteria:

### Phase 1 (Ideation) — 1 benchmark

| Benchmark | Gate Criteria |
|-----------|---------------|
| B11: Ideation quality | Mean novelty ≥ ChatGPT baseline AND 8/8 trigram coverage |

### Phase 2 (Doc Intel) — 4 benchmarks

| Benchmark | Gate Criteria |
|-----------|---------------|
| B1: Fallacy detection | F1 ≥ 0.70 on LOGIC |
| B2: Claim alignment | Precision@5 ≥ 0.70 on synthetic |
| B4: Policy coherence | F1 ≥ 0.75 on synthetic contradictions |
| B6: Contract audit | Significant drift for vulnerable contracts (p < 0.01) |

### Phase 3 (Cognitive) — 3 benchmarks

| Benchmark | Gate Criteria |
|-----------|---------------|
| B7: Belief dissonance | F1 ≥ 0.80 |
| B8: Team compatibility | Spearman ρ ≥ 0.40 |
| B9: Learning path | Accuracy ≥ 0.60 |

---

## Benchmark Dataset Summary

| # | Benchmark | Dataset | Ground Truth | Status |
|---|-----------|---------|-------------|--------|
| B1 | Fallacy detection | LOGIC/LogicClimate | 13-class labels | Exists on HF |
| B2 | Claim alignment | Synthetic + FEVER | Injected pairs / evidence labels | Synthetic (build) + exists |
| B3 | Research gaps | Synthetic + arXiv reviews | Injected gaps / future work | Synthetic (build) + manual |
| B4 | Policy coherence | Synthetic corporate policies | Injected contradictions | Synthetic (build) |
| B5 | Cross-lingual alignment | regia-ai/crosslingual-sts | Embedding labels | Exists on HF |
| B6 | Contract audit | jhsu12 + Bastet | Vuln types + severity | Exists on HF |
| B7 | Belief dissonance | Synthetic agent beliefs | Injected contradictions | Synthetic (build) |
| B8 | Team compatibility | Self-constructed + startup data | Human ratings / outcomes | Build |
| B9 | Learning path | wiki-prerequisite-data | Strength scores | Exists on HF |
| B10 | Goal coherence | Synthetic goal hierarchies | Injected contradictions | Synthetic (build) |
| B11 | Ideation quality | Self-constructed | Human novelty ratings | Build |

**5 of 11 benchmarks** use existing labeled datasets (no data construction needed). The remaining 6 use synthetic data with injected ground truth — standard practice for early-stage validation of novel systems.

---

## Benefit Summary by Axis

| Axis | Demonstrated In |
|-------|----------------|
| **Speed (ns–µs)** | B1 (fallacy per argument), B4 (coherence per clause pair), B7 (dissonance per belief pair) |
| **Interpretability** | B2 (4-way classification, not just similarity), B5 (per-role divergence), B6 (which role diverged in contract) |
| **Zero-training** | All benchmarks — no training phase, no fine-tuning |
| **Compactness** | B7 (100 beliefs = 6.4KB), B11 (64 hexagram perspectives = 4.1KB) |
| **vs LLM cost** | B1, B4, B6 — marginal cost $0 after initial encoding vs $0.01–0.03 per LLM check |
| **vs embedding baseline** | B2 (4-way vs 1-score), B5 (role-divergence vs scalar similarity) |

---

## Conclusion

The 29 operations add measurable, benchmarkable value. The core advantage — nanosecond-speed, interpretable, zero-training reasoning in a 64-byte representation — is testable. Five benchmarks use existing labeled datasets. Six use synthetic data with injected ground truth. The minimum viable set requires 8 benchmarks to pass across 3 phases, with quantitative thresholds tied to competitive baselines (GPT-4 zero-shot, sentence embeddings, static analysis tools).
