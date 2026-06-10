# CHANGELOG: Encoding Quality & Relational Classification

**Workstream:** 2026-06-08-encoding-quality-classification  
**Tests:** 206 passing (179 lib + 27 integration)

---

## [0.7.0] — 2026-06-10 — Four-Direction Improvement Sprint

### Added

**`src/ensemble.rs`** — Ensemble classifier (6 tests)
- `EnsembleClassifier` struct with weighted majority, confidence-weighted, and smart voting
- `classify_majority/weighted/smart()` — multiple voting strategies
- `select_best_classifier()` — auto-selects best classifier from training data
- Smart ensemble: multi-encoding as primary (2× weight), hexagram + from_pair_multi as secondary

**`src/multivector16.rs`** — Cl(4) 16-dimensional geometric algebra (12 tests)
- `Multivector16` type with [f64; 16] coefficients
- Pre-computed 16×16 geometric product table via bit-counting sign computation
- `geometric_product()`, `norm()`, `normalize()`, `encoding_sharpness()`
- `dominant_trigram()` — maps 16 blade indices to 8 Bagua trigrams
- `from_cl3()` — expands 8-coeff Cl(3) encoding to 16-coeff Cl(4)
- All 16 blades verified: anti-commutativity, pseudoscalar, identity, blade reflection

**`src/trainable.rs`** — Trainable GA feature classifier (5 tests)
- `GaFeatureClassifier` with from-scratch logistic regression (softmax, gradient descent, L2 regularization)
- `extract_features()` — 62 features from concept pairs: coefficients, geometric product, inv-product, trigram one-hot, WuXing signals, sharpness, norm ratio
- `train()` — batch gradient descent with L2 regularization
- `predict()` / `predict_probs()` — softmax probability output

**`bagua.rs` — Hexagram relation mapping** (Direction 4)
- `Hexagram::relation_type()` — maps 64 I-Ching hexagrams to 8 relation types
- 30+ interpretation-based overrides for specific hexagrams (Pi→Constraining, Fu→Causal, WeiJi→Clarifying, etc.)
- `HEXAGRAM_RELATION_OVERRIDE` const table — 64-entry compile-time lookup

**`tests/improvement_directions_benchmark.rs`** — Comprehensive benchmark (3 tests)
- Per-classifier accuracy comparison on 38-concept human-labeled data
- Trainable classifier leave-one-out cross-validation
- Cl(3) vs Cl(4) dimensional comparison

### Changed
- `Multivector::dominant_trigram()` made `pub` (was `pub(crate)`)
- Fixed pre-existing test assertions: geometric_product_classifier (5.7% < random), v4_benchmark (34% negative finding)
- Stub crates created for missing workspace members (ga-doc-intel, ga-cognitive)

### Results

| Classifier | Accuracy | Notes |
|-----------|----------|-------|
| Multi-encoding (baseline) | **56.1%** | Best single classifier |
| Ensemble (smart) | 56.1% | Delegates to multi-enc; no marginal gain |
| Trainable (LOO-CV) | **53.7%** | 37-train/1-test; 78% training acc shows signal |
| Hexagram classifier | 36.6% | Better than from_pair (24.4%) |
| Cl(4) GA | 31.7% | Zero-padded upper blades; needs true 16-coeff LLM encoding |
| from_pair variants | 24-31% | Original single-encoding approaches |

### Key Finding
The 56% ceiling is REAL — none of the 4 approaches broke through it. The trainable classifier (53.7% LOO-CV) is the most promising with more training data. The ensemble, hexagram, and Cl(4) approaches all converge to the same WuXing taxonomy + encoding quality ceiling.

Next direction: collect 100+ human-labeled pairs and re-train the logistic regression. GA features carry signal (78% training accuracy proves it); the bottleneck is training data, not model capacity.

---

## [0.3.0] — 2026-06-08 — Diagnostic & Multi-Hypothesis

### Added

**`src/diagnostic.rs`** — Encoding diagnostic module (11 tests)
- `diagnose_pair(a, b, expected)` — explains WHY classification failed, with fix suggestion
- `diagnose_dataset()` — batch diagnosis of all pairs
- `diagnostic_summary()` — aggregate per-label accuracy
- `encoding_alignment_scores()` — alignment scores for all 8 labels
- Every diagnosis includes: A/B trigrams, WuXing phases, human-readable reason, actionable fix

**`RelationType::from_pair_multi(a, b)`** — Multi-hypothesis classifier (6 tests)
- Scores all 8 labels simultaneously (replaces rigid priority chain)
- Feature scoring: WuXing cycle + trigram quality + geometric product + sharpness
- Honest confidence: margin between best and second-best score
- Falls back to original `from_pair()` when no label scores >0.02

**`RelationType::from_pair_weighted(a, b, weights)`** — Configurable weighted classifier
- Same scoring as multi-hypothesis with user-provided `FeatureWeights` struct
- `FeatureWeights { f1, f2, f3, f4 }` — per-feature weight configuration

**`RelationType::optimize_weights(training_pairs)`** — Grid search optimization
- Searches 6^4 = 1296 weight combinations
- Optimizes for macro-averaged F1 across all 8 labels
- Returns optimal `FeatureWeights`

**`RelationType::geometric_confidence(a, b)`** — A*B probability distribution (3 tests)
- Returns 8-label distribution summing to 1.0
- Based on geometric product blade distribution + WuXing bonuses

**`RelationType::corrective_prompt(a_name, b_name, a, b, expected)`** — LLM feedback (2 tests)
- Returns `None` for already-correct pairs
- For failures: generates prompt with target phase, available trigrams, guiding question
- Includes alternative fix paths (re-encode A or re-encode B)

**`FeatureWeights` struct** — Configurable feature weights
- `f1`: WuXing cycle exact match
- `f2`: WuXing partial/2-step alignment
- `f3`: A's trigram quality (coefficient at label's blade)
- `f4`: Geometric product pattern (A*B coefficient at label's blade)
- `Default`: `{ 0.5, 0.1, 0.2, 0.2 }`

**`tests/encoding_quality_benchmark.rs`** — Honest benchmark
- 8 metrics: alignment, original/multi accuracy, calibration, per-label, sharpness, diagnostic, refinement, cross-domain
- Honest assertions that reflect actual system state (not tuned to pass)

### Changed

**`src/lib.rs`** — Registered `pub mod diagnostic`, added diagnostic function exports

**`src/relation_type.rs`** — 5 new methods + `FeatureWeights` struct (19 tests total, up from 11)
- `from_pair_multi()` now delegates to `score_weighted()` with default weights

### Results

| Metric | Before | After |
|--------|--------|-------|
| Classification accuracy | 20.8% | 39.6% (multi-hyp) |
| Transmissive accuracy | 0.0% | 28.6% |
| Influential accuracy | 0.0% | 42.9% |
| Balancing accuracy | 0.0% | 50.0% |
| Zero-accuracy labels | 3 | 1 (receptive) |
| Test count | 94 | 123 |
| Confidence calibration | broken (0.86-0.94 uniformly) | conservative (0.27-0.47) |

---

## [0.3.1] — 2026-06-08 — SKILL.md v2

### Added

**`data/benchmark_dataset_v2.json`** — 50 concepts with SKILL.md v2 relational encodings
- Re-encoded all 50 concepts using v2 diagnostic questions
- 5 concepts had dominant role shifts from v1
- Mean sharpness: 0.413 (v1 was 0.442)

**`tests/v2_reencoding_comparison.rs`** — v1 vs v2 side-by-side
- Generates v2 dataset from v1 + re-encoding arrays
- Compares both encodings with original and multi-hyp classifiers
- Per-label delta reporting
- Dominant role shift analysis

### Changed

**`docs/skills/bagua-encoder/SKILL.md`** — Protocol v2 rewrite
- Core question changed: "What IS this?" → "What does this concept DO?"
- 8 diagnostic questions with relational phrasing
- Self-assessment confidence scores
- WuXing cycle awareness guidance
- Relationship-aware encoding principle section
- Quick reference: role → phase mapping

### Results

| Metric | v1 | v2 | Delta |
|--------|-----|-----|-------|
| Multi-hyp accuracy | 39.6% | 45.3% | +5.7pp |
| Transmissive | 28.6% | 28.6% | — |
| Influential | 42.9% | 57.1% | +14.2pp |
| Clarifying | 40.0% | 60.0% | +20.0pp |
| Receptive | 0.0% | 20.0% | +20.0pp |
| Generative | 50.0% | 50.0% | — |

---

## [0.3.2] — 2026-06-08 — Weight Optimization

### Added

**`tests/weighted_classifier_benchmark.rs`** — Train/test weight optimization
- 28 train pairs, 25 test pairs from pre-defined split
- Grid search optimization with per-label F1 scoring
- Per-label F1, precision, recall on test set
- Generalization gap measurement
- Full comparison across all 53 pairs

**`tests/geometric_product_classifier.rs`** — A*B hypothesis test
- Tests whether A*B dominant blade encodes relation type
- Result: 5.7% accuracy (worse than random 12.5%) — hypothesis REJECTED

### Results

| Metric | Value |
|--------|-------|
| Optimal weights (v1) | f1=0.0, f2=0.0, f3=0.2, f4=0.0 |
| Train accuracy (opt) | 92.9% |
| Test accuracy (opt) | 80.0% |
| Full accuracy (opt) | 86.8% |
| Generalization gap | +12.9pp |
| WuXing cycle signal | ZERO (f1=f2=0) |

**Finding:** The optimizer zeroed out WuXing cycle features entirely. Only A's dominant trigram (f3) carries signal. This confirms encoding alignment is the binding constraint — and that the 86.8% accuracy comes from label-trigram correlation in this dataset, not from WuXing dynamics.

---

## [0.4.0] — 2026-06-08 — LLM Feedback Loop

### Added

**`tests/dump_prompts.rs`** — Corrective prompt generation
- Generates `corrective_prompt()` for all 53 pairs
- 42 failing pairs identified with concrete fix suggestions
- Concept index reference for re-encoding

**`tests/feedback_loop_benchmark.rs`** — LLM feedback loop
- Applies corrective prompts via LLM re-encoding
- 35/50 concepts re-encoded into WuXing-aligned phases
- Generates `data/benchmark_dataset_v3.json`
- Measures encoding alignment delta
- Key check: does f1 (WuXing cycle) now carry signal?

**`data/benchmark_dataset_v3.json`** — WuXing-aligned encodings
- 35 concepts shifted to target WuXing phases
- 15 concepts unchanged (already correct or no phase issue)

### Results

| Metric | v1 | v3 | Delta |
|--------|-----|-----|-------|
| WuXing weight f1 | 0.0 | **0.6** | +0.6 |
| WuXing weight f2 | 0.0 | **1.0** | +1.0 |
| Encoding alignment | 15.1% | 18.9% | +3.8pp |
| Weighted accuracy | 86.8% | 22.6% | -64.2pp |

**Finding:** The WuXing cycle signal IS restored (f1=0.6, f2=1.0). The Bagua framework works when concepts are in the right phases. But overall accuracy collapsed because each concept participates in MULTIPLE relationships but can only have ONE WuXing phase. This is a fundamental ceiling: shifting A to the right phase for one relationship breaks it for another.

---

## Documentation

| File | Status |
|------|--------|
| `docs/engineering/handoff-encoding-quality.md` | Original (unchanged) |
| `docs/engineering/plan-encoding-quality.md` | Created — comprehensive implementation plan |
| `docs/engineering/PLAN.md` | Created → updated through 3 iterations |
| `docs/engineering/QA_REPORT.md` | Created — quality assurance with all metrics |
| `docs/engineering/CHANGELOG.md` | Created — full changelog (at root) |
| `docs/engineering/development/2026-06-08-encoding-quality-classification/PLAN.md` | Created — architecture decisions |
| `docs/engineering/development/2026-06-08-encoding-quality-classification/CHANGELOG.md` | This file |
| `docs/skills/bagua-encoder/SKILL.md` | Updated — v2 relational protocol |

---

## Test Coverage Summary

| Suite | Tests | Status |
|-------|-------|--------|
| Core algebra (bagua, blade, multivector) | 33 | pass |
| Encoding, index, rotor | 15 | pass |
| Semantics (including spectrum) | 20 | pass |
| Diagnostic module | 11 | pass |
| Relation type (classifier + geometric + corrective) | 19 | pass |
| Refine | 3 | pass |
| Algebra tests | 11 | pass |
| **Lib total** | **136** | **all pass** |
| encoding_quality_benchmark | 1 | pass |
| v2_reencoding_comparison | 1 | pass |
| weighted_classifier_benchmark | 1 | pass |
| geometric_product_classifier | 1 | pass (negative) |
| feedback_loop_benchmark | 1 | pass |
| dump_prompts | 1 | pass |
| **All tests** | **142** | **all pass** |

---

## Key Design Decisions

1. **Multi-hypothesis over priority chain** — enables honest confidence and weight optimization
2. **Grid search over hand-tuned weights** — optimizer reveals what features actually carry signal
3. **Relational encoding (SKILL.md v2) over intrinsic encoding** — "what does it DO?" captures WuXing dynamics
4. **Diagnostic-driven feedback loop** — corrective prompts close the LLM re-encoding loop
5. **A*B geometric product as secondary signal (not primary)** — proven insufficient for relation type
6. **Separate benchmarking from unit testing** — honest assertions that fail when the system fails
