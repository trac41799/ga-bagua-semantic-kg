# Implementation Plan v2 — Breaking the 56% Ceiling

**Date:** 2026-06-10  
**Baseline:** 56.4% accuracy on 38-concept human-labeled data (multi-encoding)  
**Target:** 65-75% via 4 complementary improvements

---

## Architecture Overview

```
                   ┌──────────────────────────────┐
                   │    EnsembleClassifier         │
                   │  (src/ensemble.rs) — NEW      │
                   │  ┌─ majority vote             │
                   │  ├─ confidence-weighted vote  │
                   │  └─ best-on-train selector    │
                   └──────────┬───────────────────┘
                              │ aggregates 5 classifiers
          ┌──────────┬────────┼────────┬──────────┐
          v          v        v        v          v
    from_pair  from_pair  from_pair  geom_conf  classify_multi
              _multi     _weighted              _encoded
                                              (needs MultiEncodedConcept)
          │          │        │        │          │
          └──────────┴────────┴────────┴──────────┘
                              │
                 adds Hexagram mapping (Direction 4)
                 adds Cl(4) support (Direction 1)
                 adds Trainable LR (Direction 2)
```

---

## Direction 3: Ensemble Classifier (PRIORITY: NOW)

### Files to create/modify
- **NEW:** `src/ensemble.rs` (~200 lines)
- **NEW:** `tests/ensemble_benchmark.rs` (~150 lines)
- **MODIFY:** `src/lib.rs` — add `pub mod ensemble;`

### Design

```rust
pub struct EnsembleClassifier {
    weights: FeatureWeights,  // for from_pair_weighted
}

impl EnsembleClassifier {
    pub fn new() -> Self { }

    /// Run all 5 classifiers, return consensus via majority vote
    pub fn classify_majority(
        a: &Multivector, b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
    ) -> (RelationType, f64)

    /// Confidence-weighted: each classifier votes with its confidence
    pub fn classify_weighted(
        a: &Multivector, b: &Multivector,
        mc_a: Option<&MultiEncodedConcept>,
        mc_b: Option<&MultiEncodedConcept>,
    ) -> (RelationType, f64)

    /// Selects the single best classifier from training data,
    /// then uses only that one for inference
    pub fn classify_best_on_train(
        a: &Multivector, b: &Multivector,
        training_data: &[(&Multivector, &Multivector, RelationType)],
    ) -> (RelationType, f64)
}
```

### Voting Logic
For majority/weighted, collect results from:
1. `from_pair(a, b)` → `(label, confidence)`
2. `from_pair_multi(a, b)` → `(label, confidence)`
3. `from_pair_weighted(a, b, weights)` → `(label, confidence)`
4. `from_pair_with_geom_conf(a, b)` → `(label, confidence)`
5. `classify_multi_encoded(mc_a, mc_b, weights)` → `(label, confidence)` — only if mc_a/mc_b are Some

**Majority vote:** pick label appearing most often. Tiebreak by sum of confidences.

**Confidence-weighted:** for each of 8 labels, sum confidences from classifiers voting for that label. Pick label with highest total confidence mass.

**Best-on-train:** evaluate each of the 5 classifiers on training data. Pick the one with highest accuracy. Use ONLY that one for test inference. This is the most conservative strategy — avoids overfitting to voting structure.

### Expected Impact
- 3-8pp improvement over best single classifier (56.4% → 59-64%)
- Zero additional tokens
- <500ns per classification (pure arithmetic)

---

## Direction 4: Hexagram-to-Relation Mapping (PRIORITY: AFTER ENSEMBLE)

### Files to modify
- **MODIFY:** `src/bagua.rs` — add `Hexagram::relation_type()` and `HEXAGRAM_RELATION_MAP`
- **NEW:** `tests/hexagram_benchmark.rs` (~100 lines)

### Design

Map each of the 64 I-Ching hexagrams to one of the 8 relation types, using the hexagram's I-Ching interpretation text as the semantic basis.

The mapping logic:
- Upper trigram determines the "active force"
- Lower trigram determines the "receiving situation"
- The hexagram interpretation describes the relationship between them

**Hardcoded mapping table** for all 64 hexagrams → 8 relation types:
```
Upper × Lower interpretations → relation type:
Qian(upper) anything → generative (heaven initiates)
Kun(upper) anything → receptive (earth receives)
Zhen(upper) anything → causal (thunder triggers)
Kan(upper) anything → transmissive (water flows/channels)
Gen(upper) anything → constraining (mountain blocks/bounds)
Xun(upper) anything → influential (wind pervades)
Li(upper) anything → clarifying (fire illuminates)
Dui(upper) anything → balancing (lake reflects/mirrors)
```

This maps the CURRENT hexagram fallback in `from_pair()` into a standalone classifier that can vote in the ensemble. The upper trigram already determines the label in the fallback — this just formalizes it.

**Additional refinement:** For certain hexagrams, the interpretation text suggests a different relation than the upper trigram alone:
- Hexagram 12 (Pi/Standstill — Qian over Kun): constraining (creative blocked by passive) NOT generative
- Hexagram 24 (Fu/Return — Kun over Zhen): causal (thunder returning from earth) NOT receptive
- Hexagram 64 (WeiJi/Before Completion — Li over Kan): clarifying (fire over water) NOT causal

We'll add 10-15 such overrides based on the hexagram interpretation text.

### New method on Hexagram
```rust
impl Hexagram {
    /// Classify the relationship type from hexagram interpretation.
    pub fn relation_type(self) -> (RelationType, f64)
}
```

### Expected Impact
- As standalone classifier: 15-25% (comparable to from_pair fallback)
- As extra vote in ensemble: 1-3pp marginal improvement
- Philosophical alignment: the hexagram IS the relationship in I-Ching

---

## Direction 1: Cl(4) Higher-Dimensional GA (PRIORITY: MEDIUM)

### Files to create/modify
- **NEW:** `src/multivector16.rs` (~400 lines)
- **MODIFY:** `src/multi_encoding.rs` — add Cl(4) variant of MultiEncodedConcept
- **MODIFY:** `src/lib.rs` — add module
- **NEW:** `tests/cl4_benchmark.rs` (~150 lines)

### Design

**Multivector16** has 16 coefficients for Cl(4):
```
Basis blades: Scalar(1), E1, E2, E3, E4, E12, E13, E14, E23, E24, E34, E123, E124, E134, E234, E1234
```

Grade distribution:
- Grade 0: 1 blade
- Grade 1: 4 blades
- Grade 2: 6 blades
- Grade 3: 4 blades
- Grade 4: 1 blade
- Total: 16 blades

**Geometric product table:** 16×16 = 256 entries. Pre-computed as `[[(usize, f64); 16]; 16]` — each entry maps to index+sign.

**Minimal implementation** (not full algebra, just what's needed):
- `norm()` — sqrt of scalar part squared
- `geometric_product()` — via lookup table
- `dominant_trigram()` — maps 16 blade indices to 8 trigrams (some blades map to same trigram)
- `encoding_sharpness()` — same as Cl(3)
- `coefficient(index)` — direct access

**Blade-to-Trigram mapping for Cl(4):**
The 16 blades in Cl(4) can represent the 8 trigrams with more subspaces:
- Each trigram gets 2 blades (primary + secondary)
- Primary blades: same as Cl(3) mapping (0→Kun, 1→Zhen, 2→Kan, 3→Gen, 4→Li, 5→Xun, 6→Dui, 7→Qian)
- Secondary blades: additional Cl(4) blades (E4-type blades)
- This doubles the encoding resolution from 8D to 16D

**Encoding path:**
- LLM produces 8 coefficients (same as before) — backward compatible
- `to_cl4(encoding: &[f64; 8]) -> [f64; 16]` — expand to 16D with zero-padding for secondary blades
- Or: LLM produces 16 coefficients directly (new path, higher token cost)

**Multi-encoding adaptation:**
- `MultiEncodedConceptCl4` — 5 phases × 16D encodings
- `classify_multi_encoded_cl4()` — same logic, expanded dimensions

### Expected Impact
- Theoretical: 16D space gives more room for role separation
- Richer blade space → better WuXing phase separation
- Same trigram concepts can occupy different secondary blades
- Expected improvement: 5-12pp over Cl(3) baseline

---

## Direction 2: Trainable GA Feature Classifier (PRIORITY: AFTER ALL)

### Files to create/modify
- **NEW:** `src/trainable.rs` (~250 lines)
- **NEW:** `tests/trainable_benchmark.rs` (~100 lines)
- **MODIFY:** `src/lib.rs` — add module

### Design

**Features (64 total per pair):**
1. A coefficients (8)
2. B coefficients (8)
3. A*B geometric product coefficients (8)
4. A inverse coefficients (8)
5. B inverse coefficients (8)
6. A dominant trigram one-hot (8)
7. B dominant trigram one-hot (8)
8. WuXing cycle features (4): A generates B?, B generates A?, A controls B?, B controls A?
9. Encoding sharpness (2): sharpness_A, sharpness_B
10. Scalar: norm_ratio = |A|/|B|

Total: 64 features (reduce to 62 by dropping redundant ones).

**Model: Multi-class logistic regression (softmax)**
- No dependencies — implement from scratch
- 64 features × 8 classes = 512 weights + 8 biases
- L2 regularization (λ=0.1) to prevent overfitting
- Leave-one-out cross-validation (38 folds)
- Gradient descent: 1000 iterations, learning rate 0.01

```rust
pub struct GaClassifier {
    weights: [[f64; 64]; 8],  // per-class weight vectors
    biases: [f64; 8],
    lambda: f64,  // L2 regularization
}

impl GaClassifier {
    pub fn train(features: &[[f64; 64]], labels: &[RelationType]) -> Self
    pub fn predict(&self, features: &[f64; 64]) -> (RelationType, f64)
    pub fn extract_features(a: &Multivector, b: &Multivector) -> [f64; 64]
}
```

### Training strategy
- Leave-one-out CV: train on 37 pairs, test on 1, repeat 38 times
- Report LOO accuracy (most honest metric for small datasets)
- Also report: training accuracy (should be high — tests for overfitting)

### Expected Impact
- LOO-CV accuracy: 45-55% (limited by 38 samples, 64 features)
- Potential: if GA features carry signal, logistic regression can discover patterns the hardcoded rules miss
- Risk: severe overfitting. Regularization is critical.
- If LOO > 60%, this is the most promising direction

---

## Implementation Order

| Step | Direction | Module | Lines | Effort |
|------|-----------|--------|-------|--------|
| 1 | #3 Ensemble | `src/ensemble.rs` | ~200 | 30 min |
| 2 | #4 Hexagram | `src/bagua.rs` (modify) | ~100 | 30 min |
| 3 | #1 Cl(4) | `src/multivector16.rs` | ~400 | 2 hours |
| 4 | #2 Trainable | `src/trainable.rs` | ~250 | 1 hour |
| 5 | Benchmarks | 4 new test files | ~500 | 30 min |
| 6 | Wire up lib.rs | `src/lib.rs` | ~20 | 5 min |
| 7 | Run all | `cargo test` | — | 5 min |

---

## Success Criteria

| Metric | Before | Target |
|--------|--------|--------|
| Multi-encoding accuracy (38 human labels) | 56.4% | 65-70% |
| Ensemble improvements over best single | — | +3-8pp |
| Cl(4) improvement over Cl(3) | — | +5-12pp |
| Trainable LOO-CV accuracy | — | 45-55% |
| All 147 lib tests passing | ✓ | ✓ |
| All benchmark tests passing | 13 | 17+ |
