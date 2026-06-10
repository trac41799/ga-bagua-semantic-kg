# Plan: Robust Relational Classification System

**Created:** 2026-06-08
**Status:** In Progress
**Depends on:** trigram-specific classification rules (MERGED)
**Based on:** `handoff-encoding-quality.md`, current benchmark results

---

## Executive Summary

The current system achieves **24.0% test accuracy** (6/25) on the 50-concept benchmark. The WuXing cycle classification logic works — when encodings are correct, it achieves 100% on hand-tuned pairs. The bottleneck is **encoding quality**: concepts are encoded intrinsically ("what IS this thing?") rather than relationally ("what does A DO to B?").

This plan proposes a 6-phase approach using TDD to build a robust system.

---

## Current State (Honest Baseline)

```
Train accuracy:  17.9% (5/28)
Test accuracy:   24.0% (6/25)
Cross-domain:    37.5% (3/8)

Per-label accuracy:
  generative:    16.7% (1/6)
  receptive:     20.0% (1/5)
  causal:        20.0% (1/5)
  transmissive:   0.0% (0/7)   ← systematic failure
  constraining:  50.0% (6/12)  ← highest (easy: dominant trigram matches)
  influential:    0.0% (0/7)   ← systematic failure
  clarifying:    40.0% (2/5)
  balancing:      0.0% (0/6)   ← systematic failure
```

### Root Cause Analysis of Each Failed Label

#### Transmissive (0/7): Encoding Mismatch
The rule `Kan generates B → transmissive` requires B to be Wood-encoded. But ALL 7 transmissive-labeled pairs have B encoded as Metal, Earth, or Fire. The rule can never fire because the B concept is never Wood. Example: Pipeline(Kan/Water) → Revenue(Qian/Metal). Water generates Wood, not Metal.

#### Balancing (0/6): Phase Mismatch
The rule requires same phase + complementary trigrams. Of 6 balancing pairs, only 1 pair (Team Standup↔Feedback Loop) has both in the same phase, but even that one has non-complementary trigrams. The encodings consistently place balanced-pair concepts in different WuXing phases.

#### Influential (0/7): Control Direction
The rule requires B controls A (Xun controls → influential). But in most influential-labeled pairs, the trigram mapping gives A controlling B instead. Example: Feature Flag(Xun/Wood) → Deprecation Policy(Xun/Wood). Same trigram, falls through to "receptive".

#### Causal (20%): Trigger vs. Constrain
Mutation(Zhen/Wood) → Natural Selection(Gen/Earth). Zhen generates Fire, not Earth. But Wood controls Earth → "constraining". The LLM labeled this "causal" (Mutation triggers variation), but the WuXing says "constraining". The Causal role is fundamentally about triggering chain reactions — but the WuXing control relationship maps to "constraining".

---

## The Core Insight

The I-Ching consultation protocol works differently from how we're using it:

1. **A trigram is not a property — it's a situational role.** In I-Ching divination, the same situation can be assigned different trigrams depending on the question asked. "Fire" in the context of "how does it relate to Water?" is different from "Fire" in the context of "how does it relate to Wood?"

2. **The geometric product IS the relationship.** When we compute A*B, the resulting multivector captures the compound dynamics. Its dominant blade should indicate the relation type. The current code uses the geometric product only as a fallback, and even then only looks at the upper trigram.

3. **Confidence must be honest.** Currently, every WuXing cycle rule fires with 1.0 confidence regardless of encoding quality. Real confidence should reflect how well the full multivector geometry supports the classification.

---

## Phase 1: Encoding Diagnostic Module

### What
A new module `src/diagnostic.rs` that for any (A, B, expected_label) triple:
- Explains WHY the classification failed in human-readable terms
- Identifies which encoding is wrong and how to fix it
- Computes an "encoding alignment score" for each possible relation type

### TDD Tests (write first, then implement)
```
test diagnose_transmissive_failure:      "Pipeline→Revenue expected transmissive,
                                          got receptive. Pipeline enc=Water(Kan).
                                          Rule requires: A=Kan AND B=Wood.
                                          B=Revenue has phase=Metal(Qian).
                                          Fix: shift Revenue to Wood phase
                                          OR shift Pipeline to phase that generates Metal."

test diagnose_balancing_failure:         Both concepts must be same phase with
                                          complementary trigrams.

test encoding_alignment_score:           For a correctly encoded generative pair,
                                          the "generative" alignment score > all others.

test diagnose_all_labels:                Every label has at least one pair in the
                                          dataset that can be diagnosed.
test diagnose_suggests_actionable_fix:   Every diagnosis includes a concrete fix suggestion.
```

### API
```rust
pub struct DiagnosticResult {
    pub actual_label: RelationType,
    pub expected_label: RelationType,
    pub correct: bool,
    pub reason: String,
    pub a_trigram: Trigram,
    pub b_trigram: Trigram,
    pub a_phase: WuXing,
    pub b_phase: WuXing,
    pub fix_suggestion: Option<String>,
    pub alignment_scores: [(RelationType, f64); 8],
}

pub fn diagnose_pair(a: &Multivector, b: &Multivector,
    expected: RelationType) -> DiagnosticResult;
pub fn diagnose_dataset(concepts: &[Multivector], relations: &[(usize, usize, RelationType)])
    -> Vec<DiagnosticResult>;
pub fn encoding_alignment_scores(a: &Multivector, b: &Multivector)
    -> [(RelationType, f64); 8];
```

---

## Phase 2: Multi-Hypothesis Classifier

### What
Replace the single-path rigid priority chain in `from_pair()` with a multi-hypothesis scorer that:
1. Evaluates all 8 relation types simultaneously
2. Scores each based on: WuXing alignment, trigram quality match, geometric product pattern, hexagram interpretation
3. Returns the best label with a geometrically-honest confidence

### TDD Tests
```
test multi_hypothesis_ranks_correctly:   For hand-tuned pairs where we know the
                                          correct label, the multi-hypothesis
                                          classifier ranks it #1.

test multi_hypothesis_on_failing_pairs:  On cases where current classifier is wrong,
                                          multi-hypothesis may be wrong too (encoding
                                          is the root cause), but the confidence
                                          should be LOWER, reflecting uncertainty.

test confidence_is_honest:              On ambiguous pairs (same trigram both sides),
                                          confidence < 0.5 even if a label is returned.

test geometric_product_based_relation:  A*B dominant blade correlates with the
                                          actual semantic relationship.

test all_8_labels_are_ranked:           Every label gets a numeric score in [0,1].
test scores_summarize_geometric_evidence: For blade-encoded pairs (single coefficient
                                           = 1.0), only one label should get high score.
```

### Scoring Formula
For each relation type R:
```
score(R) = w1 * wuxing_cycle_score(A, B, R)
         + w2 * trigram_quality_score(A, B, R)
         + w3 * geometric_product_score(A, B, R)
         + w4 * hexagram_interpretation_score(A, B, R)
```
where w1..w4 are tuning weights (initially equal, empirically adjusted).

### Geometric Product Scoring
The geometric product A*B captures the compound dynamics. If A represents a generative force and B participates, the product should have a strong generative blade. Score based on:
- `(A*B).coefficient(R.blade().index()).abs()` normalized
- `(A*B).grade_projection(R.blade().grade()).norm()` for grade-aware scoring

---

## Phase 3: Contextual Encoding Refinement

### What
Extend `refine.rs` to support contextual encoding: given a relationship (A→B, expected_label), suggest how to contextually re-encode A (or B) for that specific relationship.

### TDD Tests
```
test contextual_refine_changes_label:    Feed a misclassified pair through contextual
                                          refine; after refinement, the label matches.

test contextual_refine_preserves_dominant_role:  After refinement, the dominant role
                                                   of the original concept changes only
                                                   if necessary for the relationship.
test contextual_refinement_batch:         Batch refinement on all failing pairs.
test contextual_vs_independent_encoding:  Compare accuracy with and without contextual
                                          refinement. Contextual should be higher.
```

### Key Difference from Current refine.rs
Current refine.rs adjusts raw coefficients to force the dominant trigram to match. Contextual refinement instead:
1. Preserves the original intrinsic encoding
2. Creates a "contextual encoding" that is a rotor-transformed version of the intrinsic encoding
3. The contextual encoding is specific to the (A, B) pair; the intrinsic encoding is unchanged

This models the I-Ching concept that the trigram you draw depends on the question.

---

## Phase 4: SKILL.md v2

### What
Document the contextual encoding protocol that guides LLMs to produce better standalone encodings by thinking relationally.

### Key Changes from v1
| Aspect | v1 | v2 |
|--------|----|----|
| Question | "What is this concept?" | "What does this concept DO to other concepts?" |
| Output | 8 coefficients | 8 coefficients + confidence annotations |
| Guidance | Assign weights based on properties | Assign weights based on relational dynamics |
| Confidence | Not specified | Self-assess: "how certain are you about each weight?" |

### Encoding Questions (v2)
```
For concept X:
1. GENERATIVE: What does X create, enable, or bring into existence? → [0]
2. CAUSAL: What does X trigger, initiate, or set in motion? → [1]
3. TRANSMISSIVE: What flows through X, what does X channel or transmit? → [2]
4. CONSTRAINING: What does X limit, bound, or restrict? → [3]
5. CLARIFYING: What does X reveal, illuminate, or make visible? → [4]
6. INFLUENTIAL: What does X gradually shape, pervade, or spread into? → [5]
7. BALANCING: What does X mirror, equilibrate, or reflect? → [6]
8. RECEPTIVE: What does X accept, follow, or ground itself in? → [7]
```

---

## Phase 5: Comprehensive Encoding Quality Benchmark

### What
A new benchmark file `tests/encoding_quality_benchmark.rs` that honestly measures:
1. **Encoding alignment**: For each relation, does the encoding structure (dominant trigram, phase) align with the expected relation type?
2. **Encoding sharpness distribution**: What's the distribution of sharpness scores across concepts? (Higher = better defined)
3. **Encoding stability**: How much do encodings drift under ±5% noise?
4. **Multi-hypothesis accuracy**: With the improved classifier, what's per-label accuracy?
5. **Confidence calibration**: Does confidence correlate with correctness?
6. **Refinement delta**: How much does contextual refinement improve accuracy?
7. **Geometric product signal**: Does the geometric product's dominant blade match the expected relation type?

### Honest Reporting
- Report both the **raw accuracy** (standalone encodings) and **refined accuracy** (after contextual refinement)
- Label the gap honestly: "contextual refinement improves accuracy by X%, but requires per-pair encoding which costs Y tokens"
- Report per-label F1, precision, recall — not just accuracy
- Include baselines: random (12.5%), majority class, cosine similarity

---

## Phase 6: Validation Pipeline

### What
After all phases are implemented:
1. Run all existing benchmarks — ensure no regression
2. Run the new encoding quality benchmark
3. Report numbers honestly against the acceptance criteria from handoff

### Acceptance Criteria (from handoff)
- [ ] At least 2 of 4 zero-accuracy labels reach > 30% test accuracy
- [ ] Overall test accuracy > 35%
- [ ] Dominant role identification remains at 100%
- [ ] No regression in retrieval benchmarks (P@5, MRR)
- [ ] No regression in scalability, context compression

---

## Implementation Order (TDD)

### Step 1: Diagnostic Module (Test First)
1. Write failing tests for `diagnostic.rs`
2. Implement `diagnose_pair()`, `diagnose_dataset()`, `encoding_alignment_scores()`
3. Verify tests pass
4. Run on current dataset — verify diagnoses are actionable

### Step 2: Multi-Hypothesis Classifier (Test First)
1. Write failing tests for the new classifier
2. Implement `from_pair_multi_hypothesis()` alongside existing `from_pair()`
3. Verify the multi-hypothesis classifier matches or exceeds existing on hand-tuned pairs
4. Measure improvement on the benchmark dataset
5. Integrate confidence scoring from geometric evidence

### Step 3: Contextual Refinement (Test First)
1. Write failing tests for contextual refinement
2. Implement `contextual_refine_pair()`
3. Measure accuracy delta from refinement

### Step 4: Encoding Quality Benchmark (Test First)
1. Write the benchmark as a new test file
2. It should FAIL initially (accuracy below targets)
3. Use it to measure progress through phases

### Step 5: SKILL.md v2
1. Draft v2 protocol
2. Re-encode a subset of concepts using v2
3. Measure delta between v1 and v2 accuracy

### Step 6: Full Validation
1. Run all tests
2. Run all benchmarks
3. Report honestly

---

## Metrics That Matter

| Metric | Current | Phase 1 Target | Phase 2 Target | Phase 3 Target |
|--------|---------|---------------|---------------|---------------|
| Test accuracy | 24.0% | 24.0% (no change) | 35%+ | 45%+ |
| Transmissive | 0.0% | 0.0% | 20%+ | 30%+ |
| Balancing | 0.0% | 0.0% | 20%+ | 30%+ |
| Influential | 0.0% | 0.0% | 20%+ | 30%+ |
| Constraining | 50.0% | 50.0% | 55%+ | 60%+ |
| Confidence calib. | broken | broken | honest | honest |
| Diagnostic coverage | N/A | 100% pairs | 100% pairs | 100% pairs |

---

## Risks

1. **Encoding is the fundamental bottleneck** — the multi-hypothesis classifier may not help much if standalone encodings are fundamentally misaligned. The real gain may come only from contextual refinement.

2. **Contextual encoding defeats the purpose** — If we need to contextually re-encode for every pair, we lose the "encode once, classify many" efficiency. The goal is better STANDALONE encodings that work across multiple relationships.

3. **Binary blade encodings are the ceiling** — If we force concepts to have a single dominant role (single trigram), we can't represent multi-faceted concepts. But the WuXing taxonomy only has 5 phases × 8 trigrams = limited expressivity.

---

## Why This Approach Is Honest

1. **TDD ensures tests aren't tuned to code**: Tests are written before implementation, with expected outcomes derived from domain knowledge, not from the current code's behavior.

2. **Confidence calibration is measured**: We don't just report accuracy — we report whether the system KNOWS when it's wrong.

3. **Per-label breakdown**: We don't hide behind a single accuracy number. Each of the 8 labels is reported separately.

4. **Baselines included**: Every metric has a random baseline and a majority-class baseline for comparison.

5. **The encoding gap is acknowledged**: We report both raw accuracy (standalone encodings) and refined accuracy (contextual), honestly labeling the difference.
