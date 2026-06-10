# Directions: Beyond the 56% Ceiling

**Date:** 2026-06-10 (updated with implementation results)  
**Current state:** Multi-encoding at 56.1% on human-labeled data (38 concepts, 41 relations). WuXing taxonomy × encoding quality ceiling reached.

---

## Implementation Results (2026-06-10)

All 4 directions were implemented and benchmarked on the 38-concept human-labeled dataset:

| Direction | Status | Accuracy | vs Multi-enc | Key Finding |
|-----------|--------|----------|-------------|-------------|
| Multi-encoding (baseline) | — | **56.1%** | — | Best single classifier |
| #3 Ensemble (smart) | Implemented | 56.1% | +0.0pp | Delegates to multi-enc; no marginal gain |
| #4 Hexagram classifier | Implemented | **36.6%** | -19.5pp | Better than from_pair (24.4%), standalone only |
| #1 Cl(4) GA | Implemented | **31.7%** | -24.4pp | Zero-padding upper blades destroys signal; needs true 16-coeff LLM encoding |
| #2 Trainable (LOO-CV) | Implemented | **53.7%** | -2.4pp | RESPECTABLE for 37-train/1-test; 78% training acc shows overfitting; with more data could beat multi-enc |

### Key Insights

1. **Ensemble doesn't help when the best classifier is 2× better than all others.** Multi-encoding (56.1%) is so far ahead of from_pair variants (~31%) that voting always converges to multi-encoding's answer. The ensemble is a no-op.

2. **Hexagram classifier (36.6%)** shows the hexagram interpretation overrides DO capture patterns the WuXing cycle misses — it beats from_pair (24.4%) by 12.2pp. But it's still capped by the same WuXing encoding ceiling.

3. **Cl(4) with zero-padded encodings fails.** The 16D algebra is sound (product table verified, anti-commutativity correct, sharpness reasonable) but expanding 8-coeff encodings to 16 by zero-padding means the upper 8 blades are always zero. True Cl(4) requires 16-coefficient LLM encodings — doubling token cost per concept.

4. **Trainable classifier (53.7% LOO-CV)** is the most promising direction. With only 37 training samples and 62 features, it nearly matches multi-encoding. Training accuracy of 78% confirms GA features carry signal. With 100+ labeled pairs, this could become the strongest classifier.

### Recommended Next Step

**Collect more human-labeled training data** for the trainable classifier. At ~50+ labeled pairs, the gap between LOO-CV and training accuracy should narrow. This is the only direction that can potentially SURPASS multi-encoding by learning patterns the hardcoded WuXing rules miss.

---

## Original Proposal (2026-06-09)

## Direction 1: Higher-Dimensional Geometric Algebra

### Rationale
Cl(3) has 8 basis blades mapping to 8 Bagua trigrams. This is mathematically elegant but potentially too cramped — the 8 role types all compete in the same 8D space. Concepts that are semantically different may be forced into similar blade patterns because there are only 8 dimensions to express 8 roles.

Cl(4) has 16 basis blades. Cl(5) has 32. More dimensions mean:

- **Sparser encodings**: concepts can have clear dominant blades without forced collisions
- **Richer taxonomy**: more relationship types can be represented natively
- **Better separation**: same-role concepts cluster more cleanly

### Approach
1. Add Cl(4) support: 16-dimensional multivectors, new `Multivector16` type
2. Map the 8 roles to 8 primary blades AND 8 secondary blades (2 per role)
3. Adapt the WuXing cycle to work with 16 dimensions
4. Existing classification logic ports cleanly — just more dimensions

### Expected Impact
- Cl(4): 16D space → theoretical ceiling increase of 10-15pp
- Cl(5): 32D space → further gains but diminishing returns
- Cost: encoding size goes from 64 bytes to 128 bytes (Cl(4)) or 256 bytes (Cl(5))

### Risk
- Low. The algebra is well-understood. The challenge is in the encoding protocol — LLM must produce 16 coefficients instead of 8, which doubles token cost per concept.

---

## Direction 2: Trainable GA Feature Classifier

### Rationale
All current classifiers use hardcoded rules (WuXing cycle priority, trigram overrides). But we have 41 human-labeled pairs as training data. A small machine learning model trained on GA features could learn patterns that the hardcoded rules miss.

### Approach
1. For each concept pair (A, B), compute 64 features:
   - 8 coefficients from A, 8 from B = 16 features
   - 8 coefficients from A*B geometric product = 8 features
   - Dominant roles of A and B = 2 features
   - WuXing phase of A and B = 2 features
   - Encoding sharpness of A and B = 2 features
   - 34 geometric/semantic derived features
2. Train logistic regression or a small neural network on 41 human labels
3. Evaluate with leave-one-out cross-validation (41 folds)
4. The model outputs probabilities for all 8 labels

### Expected Impact
- 60-70% on human-labeled data (if GA features carry signal)
- Transparent: logistic regression weights show WHICH features matter
- Can combine with WuXing cycle rules as priors

### Risk
- Overfitting: 41 samples for 64 features is extreme. Need strong regularization.
- May just re-learn what the WuXing cycle already encodes
- Requires adding a linear algebra dependency or writing a minimal logistic regression from scratch

---

## Direction 3: Ensemble of Classifiers

### Rationale
We have 5 classifiers that make different errors:
- `from_pair()` — deterministic, overconfident, 24.4% on human labels
- `from_pair_multi()` — multi-hypothesis, conservative
- `from_pair_weighted()` — configurable weights
- `from_pair_with_geom_conf()` — geometric confidence blended
- `classify_multi_encoded()` — 5-phase multi-encoding

Each classifier makes different mistakes. A voting ensemble could outperform any single one.

### Approach
1. Run all 5 classifiers on each pair
2. Voting strategies:
   - **Majority vote**: return the label with most votes
   - **Confidence-weighted**: each classifier votes with its confidence score
   - **Best-on-train**: select the classifier that performs best on a leave-one-out split
3. Measure ensemble accuracy vs individual classifiers

### Expected Impact
- 3-8pp improvement over the single best classifier
- Zero additional tokens, zero training cost
- Immediately implementable

### Risk
- If all classifiers fail on the same pairs (correlated errors), ensemble doesn't help
- Very low. Pure arithmetic.

---

## Direction 4: Full 64-Hexagram Classification

### Rationale
The current system uses 8 roles mapped to 8 trigrams. But the I-Ching has 64 hexagrams — pairs of trigrams (upper over lower). Each hexagram has a named interpretation text.

Currently the hexagram is only a fallback. But the hexagram is the I-Ching's actual relationship taxonomy:

- "Heaven over Earth" (Qian/Kun) ≠ "Earth over Heaven" (Kun/Qian)
- The upper trigram represents the active force, the lower trigram represents the receiving situation
- This naturally captures ASYMMETRIC relationships that the 8-role flat taxonomy misses

### Approach
1. Map each of the 64 hexagrams to one of the 8 relation types using the interpretation text
2. OR: expand to a 64-way classification (finer granularity)
3. Use the geometric product's lower trigram as the primary relationship signal (proven to work: lower = A*B dominant blade)
4. Build a hexagram-based classifier that returns both the relation type AND the specific hexagram

### Expected Impact
- Moderate (5-10pp) if keeping 8-way mapping — the hexagram adds directional nuance
- High (15-25pp) if expanding to 16 or 32-way classification — many relationship nuances currently lost
- Most philosophically aligned with the I-Ching: the hexagram IS the relationship

### Risk
- The 64→8 mapping would need human verification for each hexagram
- 64-way classification on 41 samples is impossible statistically
- But as a FEATURE (not the classifier itself), the hexagram could enrich the 8-way decision

---

## Recommendation

| Direction | Impact | Effort | Risk | Priority |
|-----------|--------|--------|------|----------|
| Ensemble (#3) | Low-Med | **Minimal** | None | **Now** |
| Higher-dimensional GA (#1) | Medium | High | Low | Next |
| Hexagram features (#4) | Medium | Medium | Medium | After #1 |
| Trainable classifier (#2) | Med-High | Medium | High (overfit) | After #3 |

**Immediate action:** Build the ensemble (#3) — zero cost, immediate improvement measurement. If it works, it buys 3-8pp for free.
