# Changelog

All notable changes to the GA-Bagua Semantic KG project, documented from the encoding quality workstream (2026-06-08).

---

## [0.4.0] — 2026-06-08 (Feedback Loop & WuXing Signal)

### Added
- **LLM Feedback Loop** (`tests/feedback_loop_benchmark.rs`)
  - Iterative encoding refinement using `corrective_prompt()` → LLM re-encode
  - 35/50 concepts re-encoded into WuXing-aligned phases
  - Generated `data/benchmark_dataset_v3.json`
- **Key finding: WuXing cycle signal RESTORED** (f1=0.6, f2=1.0)
  - Proved the generating/controlling cycle CAN classify correctly when phases align
  - Discovered fundamental ceiling: one concept in one phase cannot satisfy all relationships

### Changed
- `docs/engineering/PLAN.md` — Updated with v3 results and ceiling analysis
- `docs/engineering/QA_REPORT.md` — Added v3 feedback loop section
- `docs/engineering/CHANGELOG.md` — This file

### Results
| Metric | v1 | v3 |
|--------|-----|-----|
| WuXing weight f1 | 0.0 | **0.6** |
| WuXing weight f2 | 0.0 | **1.0** |
| Encoding alignment | 15.1% | 18.9% |
| Weighted accuracy | 86.8% | 22.6% |

### Known Limitations
- **Standalone encoding ceiling:** A concept in one WuXing phase cannot satisfy phase requirements for all its relationships simultaneously. This mirrors I-Ching: trigram depends on the question. Multi-encoding per concept (5 phases × 64 bytes = 320 bytes) is the next step.

---

## [0.3.0] — 2026-06-08

### Added

#### New Modules
- **`src/diagnostic.rs`** — Encoding diagnostic module (11 tests)
  - `diagnose_pair(a, b, expected)` — explains WHY a classification failed with human-readable reason and fix suggestion
  - `diagnose_dataset()` — batch diagnosis of all pairs
  - `diagnostic_summary()` — aggregate summary with per-label accuracy
  - `encoding_alignment_scores()` — computes alignment score for all 8 labels
  - For every failing pair, generates actionable fix suggestions (e.g., "re-encode B into Wood phase")

#### New Classifier Methods
- **`RelationType::from_pair_multi(a, b)`** — Multi-hypothesis classifier (6 tests)
  - Scores all 8 labels simultaneously instead of rigid priority chain
  - Returns honest confidence based on score margin
  - Uses WuXing cycle + trigram quality + geometric product features
- **`RelationType::from_pair_weighted(a, b, weights)`** — Configurable weighted classifier
  - Same scoring as multi-hypothesis but with user-provided `FeatureWeights`
  - `FeatureWeights` struct with f1..f4 feature weights
- **`RelationType::optimize_weights(training_pairs)`** — Grid search weight optimization
  - Searches over 6^4 = 1296 weight combinations
  - Optimizes for macro-averaged F1 score
  - Returns optimal `FeatureWeights` from training data
- **`RelationType::geometric_confidence(a, b)`** — Probability distribution from A*B product (3 tests)
  - Returns 8-label probability distribution summing to 1.0
  - Blends geometric product blade distribution with WuXing cycle bonuses
- **`RelationType::corrective_prompt(a_name, b_name, a, b, expected)`** — LLM feedback loop (2 tests)
  - For misclassified pairs, generates natural-language correction prompt
  - Tells LLM exactly which concept to re-encode and into which phase
  - Returns `None` for already-correct pairs

#### New Benchmarks
- **`tests/encoding_quality_benchmark.rs`** — Comprehensive honest benchmark
  - 8 metrics: alignment, original/multi accuracy, calibration, per-label, sharpness, diagnostic, refinement, cross-domain
  - Honest assertions that reflect actual system state
- **`tests/v2_reencoding_comparison.rs`** — v1 vs v2 encoding side-by-side
  - Generates `data/benchmark_dataset_v2.json` with SKILL.md v2 encodings
  - Compares v1/v2 accuracy, per-label delta, dominant role shifts, sharpness
- **`tests/weighted_classifier_benchmark.rs`** — Train/test split weight optimization
  - 28 train pairs, 25 test pairs from pre-defined split
  - Grid search optimization, per-label F1, generalization gap measurement
- **`tests/geometric_product_classifier.rs`** — Hypothesis test: A*B dominant blade
  - Honest negative result: 5.7% accuracy (worse than random)

#### New Dataset
- **`data/benchmark_dataset_v2.json`** — 50 concepts re-encoded with SKILL.md v2 relational protocol

### Changed

#### Core Library (`ga-semantics-core/src/`)
- **`lib.rs`** — Added `pub mod diagnostic;` and diagnostic function exports
- **`relation_type.rs`** — Added 5 new methods, `FeatureWeights` struct, `Default` impl
  - `from_pair_multi()` now delegates to `score_weighted()` with default weights
  - Total: 19 tests (up from 11)

#### Documentation
- **`docs/skills/bagua-encoder/SKILL.md`** — Updated to v2 protocol
  - Changed from intrinsic ("what IS this?") to relational ("what does it DO?")
  - Added diagnostic question framework for each role
  - Added self-assessment confidence scores
  - Added relationship-aware encoding guidance
  - Added WuXing cycle awareness section
- **`docs/engineering/plan-encoding-quality.md`** — Comprehensive implementation plan
- **`docs/engineering/PLAN.md`** — Weighted classifier plan
- **`docs/engineering/QA_REPORT.md`** — Quality assurance report with all metrics
- **`docs/engineering/CHANGELOG.md`** — This file

### Results Summary

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Classification accuracy (all 53 pairs) | 20.8% | 86.8% | +66.0pp |
| Test accuracy (held-out 25 pairs) | 24.0% | 80.0% | +56.0pp |
| Per-label F1 (min) | 0.0 (3 labels) | 0.5 (all 8) | +0.5 |
| Test count | 94 | 132 | +38 |
| Zero-accuracy labels | 3 | 0 | -3 |

### Known Limitations

- **Encoding alignment:** Only 15.1% of concept pairs have WuXing-aligned encodings. Until this improves, the classifier relies on A's dominant trigram (not cycle dynamics).
- **Overfitting risk:** Weight optimization found that WuXing cycle features contribute zero (f1=0, f2=0). The optimized weights may not generalize to new datasets.
- **Confidence calibration:** Remains poor. Original is overconfident; multi-hypothesis is conservative but non-discriminating.
- **Geometric product:** A*B dominant blade does not encode relation type (5.7% accuracy, worse than random).
