use crate::bagua::{Trigram, WuXing};
use crate::RelationType;
use crate::Multivector;

// ── Types ──────────────────────────────────────────────────────────────────

/// Result of diagnosing a single (A→B, expected) relation pair.
#[derive(Debug, Clone)]
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
    pub alignment_scores: Vec<(RelationType, f64)>,
}

/// Aggregate summary across all diagnosed pairs.
#[derive(Debug, Clone)]
pub struct DiagnosticSummary {
    pub total_pairs: usize,
    pub correct_pairs: usize,
    pub pairs_with_fix_suggestions: usize,
    pub per_label_accuracy: Vec<(RelationType, f64, usize)>,
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Diagnose why a specific classification succeeded or failed.
/// For failing pairs, provides a human-readable reason and fix suggestion.
pub fn diagnose_pair(
    a: &Multivector,
    b: &Multivector,
    expected: RelationType,
) -> DiagnosticResult {
    let ta = a.dominant_trigram();
    let tb = b.dominant_trigram();
    let wa = ta.wuxing_phase();
    let wb = tb.wuxing_phase();
    let (actual, _conf) = RelationType::from_pair(a, b);
    let correct = actual == expected;
    let alignment_scores = encoding_alignment_scores(a, b);

    let (reason, fix) = if correct {
        (format!("Correct: {actual} — encodings align with expected relationship."), None)
    } else {
        build_diagnosis(ta, tb, wa, wb, actual, expected)
    };

    DiagnosticResult {
        actual_label: actual,
        expected_label: expected,
        correct,
        reason,
        a_trigram: ta,
        b_trigram: tb,
        a_phase: wa,
        b_phase: wb,
        fix_suggestion: fix,
        alignment_scores,
    }
}

/// Diagnose all relations in a batch.
pub fn diagnose_dataset(
    concepts: &[Multivector],
    relations: &[(usize, usize, RelationType)],
) -> Vec<DiagnosticResult> {
    relations
        .iter()
        .map(|&(a_idx, b_idx, expected)| {
            diagnose_pair(&concepts[a_idx], &concepts[b_idx], expected)
        })
        .collect()
}

/// Compute an aggregate summary from diagnostic results.
pub fn diagnostic_summary(results: &[DiagnosticResult]) -> DiagnosticSummary {
    let total_pairs = results.len();
    let correct_pairs = results.iter().filter(|r| r.correct).count();
    let pairs_with_fix_suggestions = results.iter().filter(|r| r.fix_suggestion.is_some()).count();

    let mut per_label: std::collections::HashMap<RelationType, (usize, usize)> =
        std::collections::HashMap::new();
    for r in results {
        let entry = per_label.entry(r.expected_label).or_insert((0, 0));
        entry.0 += 1; // total
        if r.correct {
            entry.1 += 1; // correct
        }
    }
    let per_label_accuracy: Vec<_> = RelationType::ALL
        .iter()
        .map(|label| {
            let &(total, correct) = per_label.get(label).unwrap_or(&(0, 0));
            let acc = if total > 0 {
                correct as f64 / total as f64
            } else {
                0.0
            };
            (*label, acc, total)
        })
        .collect();

    DiagnosticSummary {
        total_pairs,
        correct_pairs,
        pairs_with_fix_suggestions,
        per_label_accuracy,
    }
}

/// Compute alignment scores for all 8 relation types given a concept pair.
/// Each score indicates how well the encodings align with that relation type
/// according to WuXing cycle dynamics.
pub fn encoding_alignment_scores(
    a: &Multivector,
    b: &Multivector,
) -> Vec<(RelationType, f64)> {
    let ta = a.dominant_trigram();
    let tb = b.dominant_trigram();
    let wa = ta.wuxing_phase();
    let wb = tb.wuxing_phase();

    let mut scores = Vec::with_capacity(8);

    for &label in &RelationType::ALL {
        let score = compute_alignment_score(ta, tb, wa, wb, a, b, label);
        scores.push((label, score));
    }

    // Normalize scores to sum to 1.0 for interpretability
    let total: f64 = scores.iter().map(|(_, s)| s).sum();
    if total > f64::EPSILON {
        for (_, s) in &mut scores {
            *s /= total;
        }
    }

    scores
}

// ── Internal helpers ───────────────────────────────────────────────────────

fn build_diagnosis(
    ta: Trigram,
    tb: Trigram,
    wa: WuXing,
    wb: WuXing,
    actual: RelationType,
    expected: RelationType,
) -> (String, Option<String>) {
    let reason;
    let mut fix;

    if ta == tb {
        reason = format!(
            "Both concepts share the same dominant trigram {ta:?} ({wa:?} phase). \
             The same-trigram rule classifies this as {actual}. \
             For {expected}, the encodings must put A and B in different phases \
             or complementary trigrams within the same phase."
        );
        let target_b_phase = phase_needed_for_relation(wa, expected, false);
        if let Some(target) = target_b_phase {
            fix = Some(format!(
                "Re-encode concept B into {target:?} phase. \
                 A is {wa:?}, so for {expected}, B should be {target:?}."
            ));
        } else {
            fix = Some(format!(
                "Re-encode one concept into a different trigram so they differ. \
                 For {expected}, try making A={:?} and B in a different phase.",
                expected.bagua()
            ));
        }
    } else if wa == wb {
        let complementary = ta.complementary() == tb;
        reason = format!(
            "Both concepts are in the same WuXing phase ({wa:?}) \
             with trigrams {ta:?} and {tb:?} (complementary={complementary}). \
             Same-phase logic classifies as {actual}. \
             For {expected}, encodings need different phases aligned to the WuXing cycle."
        );
        let target_b_phase = phase_needed_for_relation(wa, expected, false);
        fix = target_b_phase.map(|target| {
            format!(
                "Re-encode concept B into {target:?} phase. \
                 A is {wa:?}, so for {expected}, B should be {target:?}."
            )
        });
        if fix.is_none() {
            fix = Some(format!(
                "Move one concept to a different phase. \
                 For {expected} with A={wa:?}, B should be in a phase \
                 reachable via WuXing generate/control cycle."
            ));
        }
    } else if wa.generate() == wb {
        reason = format!(
            "{wa:?} generates {wb:?} — WuXing says A generates B ({actual}). \
             Expected {expected}. The generating cycle maps to {actual} but the \
             semantic intent is {expected}."
        );
        fix = Some(format!(
            "For {expected} instead of {actual}: consider whether A truly generates B \
             or has a different relationship. If A triggers B (causal), ensure A is \
             Zhen-encoded. If A channels to B (transmissive), ensure A is Kan-encoded \
             and B is in the {wa:?}.generate()={:?} phase.",
            wa.generate()
        ));
    } else if wb.generate() == wa {
        reason = format!(
            "{wb:?} generates {wa:?} — WuXing says B generates A ({actual}). \
             Expected {expected}. The direction is reversed from intent."
        );
        let target_a_phase = phase_needed_for_relation(wb, expected, true);
        fix = target_a_phase.map(|target| {
            format!(
                "Re-encode concept A into {target:?} phase. \
                 B is {wb:?}, so for {expected}, A should be {target:?}."
            )
        });
        if fix.is_none() {
            fix = Some(format!(
                "The generation direction is reversed. Either swap A/B in the \
                 relationship definition, or re-encode A to be in a phase that \
                 generates {wb:?}."
            ));
        }
    } else if wa.control() == wb {
        reason = format!(
            "{wa:?} controls {wb:?} — WuXing says A constrains B ({actual}). \
             Expected {expected}. The control cycle overrides the intended label."
        );
        fix = Some(format!(
            "For {expected} instead of constraining: re-encode A into a different trigram \
             if the relationship is actually about triggering (causal), flowing (transmissive), \
             or shaping (influential) rather than limiting."
        ));
    } else if wb.control() == wa {
        reason = format!(
            "{wb:?} controls {wa:?} — WuXing says B controls A ({actual}). \
             Expected {expected}. The influence/control direction is reversed."
        );
        let target_a_phase = phase_needed_for_relation(wb, expected, true);
        fix = target_a_phase.map(|target| {
            format!(
                "Re-encode concept A into {target:?} phase. \
                 B is {wb:?}, so for {expected}, A should be {target:?}."
            )
        });
        if fix.is_none() {
            fix = Some(format!(
                "The control direction is reversed. Re-encode A or B to align \
                 the WuXing cycle with the intended {expected} relationship."
            ));
        }
    } else {
        reason = format!(
            "A={wa:?}/{ta:?}, B={wb:?}/{tb:?}. No WuXing cycle relationship \
             exists between these phases. Classified as {actual} (hexagram fallback). \
             Expected {expected}. The encodings are too far apart for any standard \
             relationship to be detected."
        );
        fix = Some(format!(
            "Re-encode one or both concepts to align with the WuXing cycle. \
             For {expected}, the target phases should be related via generating \
             or controlling cycle."
        ));
    }

    (reason, fix)
}

/// Determine what WuXing phase is needed for a concept to satisfy a relation.
/// `known_phase`: phase of the concept we already know
/// `need_a`: true = we need phase of A, false = we need phase of B
fn phase_needed_for_relation(
    known_phase: WuXing,
    relation: RelationType,
    need_a: bool,
) -> Option<WuXing> {
    let all = [WuXing::Wood, WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water];

    match (relation, need_a) {
        (RelationType::Generative, true) => {
            // Known = B, need A: A generates B → A is what generates B
            all.iter().find(|&&w| w.generate() == known_phase).copied()
        }
        (RelationType::Generative, false) => {
            // Known = A, need B: A generates B → B = A.generate()
            Some(known_phase.generate())
        }
        (RelationType::Constraining, true) => {
            all.iter().find(|&&w| w.control() == known_phase).copied()
        }
        (RelationType::Constraining, false) => {
            Some(known_phase.control())
        }
        (RelationType::Receptive, true) => {
            Some(known_phase.generate())
        }
        (RelationType::Receptive, false) => {
            all.iter().find(|&&w| w.generate() == known_phase).copied()
        }
        (RelationType::Influential, true) => {
            Some(known_phase.control())
        }
        (RelationType::Influential, false) => {
            all.iter().find(|&&w| w.control() == known_phase).copied()
        }
        (RelationType::Balancing, _) | (RelationType::Clarifying, _) => {
            Some(known_phase)
        }
        (RelationType::Causal, true) => {
            // A is causal (Zhen/Wood): A generates B
            Some(WuXing::Wood.generate())
        }
        (RelationType::Causal, false) => {
            all.iter().find(|&&w| w.generate() == known_phase).copied()
        }
        (RelationType::Transmissive, true) => {
            // A is transmissive (Kan/Water): A generates B
            Some(WuXing::Water.generate())
        }
        (RelationType::Transmissive, false) => {
            all.iter().find(|&&w| w.generate() == known_phase).copied()
        }
    }
}

/// Compute how well a concept pair's encodings align with a specific relation type.
fn compute_alignment_score(
    ta: Trigram,
    tb: Trigram,
    wa: WuXing,
    wb: WuXing,
    a: &Multivector,
    b: &Multivector,
    label: RelationType,
) -> f64 {
    let mut score = 0.0;

    // 1. WuXing cycle alignment (0–1)
    let cycle_score = match label {
        RelationType::Generative if wa.generate() == wb => 1.0,
        RelationType::Receptive if wb.generate() == wa => 1.0,
        RelationType::Constraining if wa.control() == wb => 1.0,
        RelationType::Influential if wb.control() == wa => 1.0,
        RelationType::Causal if ta == Trigram::Zhen && wa.generate() == wb => 1.0,
        RelationType::Transmissive if ta == Trigram::Kan && wa.generate() == wb => 1.0,
        RelationType::Clarifying if wa == wb && ta != tb => 0.7,
        RelationType::Balancing if wa == wb && ta.complementary() == tb => 0.9,
        _ => 0.0,
    };

    // Partial alignment: if the phases are 1 step from the needed relationship
    let partial_cycle = match (label, wa, wb) {
        (RelationType::Generative, a_phase, b_phase)
            if a_phase.generate().generate() == b_phase => 0.5, // 2-step generate
        (RelationType::Constraining, a_phase, b_phase)
            if a_phase.control().control() == b_phase => 0.3, // 2-step control
        _ => 0.0,
    };

    score += f64::max(cycle_score, partial_cycle) * 0.4;

    // 2. Trigram quality match (0–1): does A's encoding have weight
    //    in the blade corresponding to this relation type?
    let target_blade = label.bagua().blade().index();
    let a_weight = a.coefficient(target_blade).abs();
    let b_weight = b.coefficient(target_blade).abs();
    let max_weight = a_weight.max(b_weight);
    // Sigmoid: maps weight to [0,1] — 0.5 weight → ~0.62, 0.8 weight → ~0.83
    let quality_score = 1.0 / (1.0 + (-5.0 * (max_weight - 0.3)).exp());
    score += quality_score * 0.3;

    // 3. Geometric product pattern (0–1): does A*B have a strong component
    //    in the blade corresponding to this relation type?
    let product = a.geo_product(b);
    let prod_coeff = product.coefficient(target_blade).abs();
    let prod_norm = product.norm().max(f64::EPSILON);
    let prod_score = (prod_coeff / prod_norm).min(1.0);
    score += prod_score * 0.2;

    // 4. Encoding sharpness bonus: sharper encodings give clearer signals
    let sharpness = a.encoding_sharpness().min(b.encoding_sharpness());
    let sharpness_bonus = (sharpness * 0.3).min(0.1); // at most +0.1 bonus
    score += sharpness_bonus;

    score.clamp(0.0, 1.0)
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bagua::{Trigram, WuXing};
    use crate::encoding::llm_encode;
    use crate::RelationType;

    // ── FIXTURES: Real benchmark dataset encodings ──

    fn pipeline() -> Multivector {
        llm_encode(&[0.115, 0.1725, 0.8971, -0.0575, -0.115, 0.1725, 0.23, 0.23])
    }

    fn revenue_target() -> Multivector {
        llm_encode(&[0.1086, 0.3259, 0.1086, -0.1086, 0.1629, 0.1086, 0.1629, 0.8908])
    }

    fn message_queue() -> Multivector {
        llm_encode(&[0.1529, 0.2548, 0.8154, -0.2039, -0.2548, 0.1019, 0.3568, 0.051])
    }

    fn event_stream() -> Multivector {
        llm_encode(&[0.1118, 0.3354, 0.7267, -0.1118, 0.1677, 0.2795, 0.1677, 0.4472])
    }

    fn mutation() -> Multivector {
        llm_encode(&[0.1164, 0.9077, 0.1746, 0.0582, 0.1746, 0.1164, 0.1746, 0.2327])
    }

    fn natural_selection() -> Multivector {
        llm_encode(&[0.0566, 0.1132, 0.1132, 0.9058, 0.1698, 0.2831, 0.1698, 0.1132])
    }

    fn feature_flag() -> Multivector {
        llm_encode(&[0.1046, 0.2093, 0.1569, -0.1046, 0.1046, 0.8161, 0.3139, 0.3662])
    }

    fn deprecation_policy() -> Multivector {
        llm_encode(&[0.2267, 0.1133, 0.0567, 0.2834, 0.17, 0.8841, 0.17, 0.1133])
    }

    fn feedback_loop() -> Multivector {
        llm_encode(&[0.2794, 0.1117, 0.2794, 0.0559, 0.1676, 0.1676, 0.8716, 0.1117])
    }

    fn trend_analysis() -> Multivector {
        llm_encode(&[0.1729, 0.1729, 0.1153, 0.0576, 0.9222, 0.1729, 0.1153, 0.1729])
    }

    fn rate_limiter() -> Multivector {
        llm_encode(&[0.0428, -0.0856, -0.5136, 0.6847, 0.214, -0.2568, 0.1712, -0.3424])
    }

    fn api_gateway() -> Multivector {
        llm_encode(&[0.2247, 0.337, 0.8425, 0.0562, -0.1123, 0.1685, 0.2808, 0.0562])
    }

    // ── TESTS ──

    #[test]
    fn diagnose_generative_failure_pipeline_to_revenue() {
        let result = diagnose_pair(
            &pipeline(), &revenue_target(),
            RelationType::Generative,
        );

        assert!(!result.correct, "this pair is known to fail");
        assert_eq!(result.actual_label, RelationType::Receptive,
            "Water(pipe) ← Metal(revenue) means B generates A → receptive");
        assert_eq!(result.a_trigram, Trigram::Kan);
        assert_eq!(result.b_trigram, Trigram::Qian);
        assert_eq!(result.a_phase, WuXing::Water);
        assert_eq!(result.b_phase, WuXing::Metal);
        assert!(!result.reason.is_empty());
        assert!(result.fix_suggestion.is_some(), "diagnosis must suggest a fix");
        let fix = result.fix_suggestion.as_ref().unwrap();
        assert!(fix.to_lowercase().contains("phase") || fix.contains("Earth") || fix.contains("Wood"),
            "fix should mention phase changes, got: {fix}");
    }

    #[test]
    fn diagnose_same_trigram_pair_message_queue_to_event_stream() {
        let result = diagnose_pair(
            &message_queue(), &event_stream(),
            RelationType::Transmissive,
        );

        assert!(!result.correct, "known failure: same trigram → receptive");
        assert_eq!(result.actual_label, RelationType::Receptive);
        assert_eq!(result.a_trigram, Trigram::Kan);
        assert_eq!(result.b_trigram, Trigram::Kan);
        assert!(result.reason.to_lowercase().contains("same") || result.reason.contains("trigram"),
            "reason should mention same trigram: {}", result.reason);
        assert!(result.fix_suggestion.is_some());
    }

    #[test]
    fn diagnose_influential_failure_feature_flag_to_deprecation() {
        let result = diagnose_pair(
            &feature_flag(), &deprecation_policy(),
            RelationType::Influential,
        );

        assert!(!result.correct);
        assert_eq!(result.a_trigram, Trigram::Xun);
        assert_eq!(result.b_trigram, Trigram::Xun);
        assert_eq!(result.a_phase, WuXing::Wood);
        assert_eq!(result.b_phase, WuXing::Wood);
    }

    #[test]
    fn diagnose_causal_vs_constraining_mutation_to_selection() {
        let result = diagnose_pair(
            &mutation(), &natural_selection(),
            RelationType::Causal,
        );

        assert!(!result.correct);
        assert_eq!(result.actual_label, RelationType::Constraining,
            "Wood(Zhen) controls Earth(Gen) → constraining");
        assert_eq!(result.a_trigram, Trigram::Zhen);
        assert_eq!(result.b_trigram, Trigram::Gen);
        assert!(result.reason.to_lowercase().contains("control")
            || result.reason.to_lowercase().contains("constrain"),
            "reason should explain the control cycle: {}", result.reason);
    }

    #[test]
    fn diagnose_balancing_failure_feedback_to_trend() {
        let result = diagnose_pair(
            &feedback_loop(), &trend_analysis(),
            RelationType::Balancing,
        );

        assert!(!result.correct);
        assert_eq!(result.actual_label, RelationType::Influential,
            "Fire controls Metal → B controls A → influential");
        assert_eq!(result.a_trigram, Trigram::Dui);
        assert_eq!(result.b_trigram, Trigram::Li);
        assert!(!result.reason.is_empty(), "reason must not be empty");
        // Balancing requires same phase + complementary. These are in different
        // phases (Metal vs Fire) so the diagnosis explains the cycle mismatch.
        assert!(result.reason.to_lowercase().contains("different phases")
            || result.reason.to_lowercase().contains("balancing")
            || result.reason.to_lowercase().contains("control"),
            "should explain the failure: {}", result.reason);
    }

    #[test]
    fn diagnose_correct_pair_returns_correct_true() {
        let result = diagnose_pair(
            &rate_limiter(), &api_gateway(),
            RelationType::Constraining,
        );

        assert!(result.correct, "Rate Limiter constrains API Gateway should pass");
        assert_eq!(result.actual_label, RelationType::Constraining);
        assert!(result.fix_suggestion.is_none(),
            "correct pairs should not need fixes");
    }

    #[test]
    fn encoding_alignment_scores_all_eight_labels() {
        let scores = encoding_alignment_scores(&pipeline(), &revenue_target());
        assert_eq!(scores.len(), 8);
        for &(_, score) in &scores {
            assert!(score >= 0.0 && score <= 1.0,
                "alignment score must be in [0, 1], got {score}");
        }
    }

    #[test]
    fn encoding_alignment_scores_ranks_higher_for_aligned_pair() {
        let scores = encoding_alignment_scores(&rate_limiter(), &api_gateway());
        let (top_label, top_score) = scores.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        assert!(
            *top_label == RelationType::Constraining
            || scores.iter().any(|(l, s)| *l == RelationType::Constraining
                && (s - top_score).abs() < 0.01),
            "Constraining should be top or near-top for Earth→Water control pair. \
             Top label: {top_label:?} at {top_score:.3}"
        );
    }

    #[test]
    fn encoding_alignment_scores_distinct_for_different_pairs() {
        let scores1 = encoding_alignment_scores(&pipeline(), &revenue_target());
        let scores2 = encoding_alignment_scores(&rate_limiter(), &api_gateway());

        let sum_sq_diff: f64 = scores1.iter().zip(scores2.iter())
            .map(|((_, s1), (_, s2))| (s1 - s2).powi(2))
            .sum();
        assert!(sum_sq_diff > 0.01,
            "different pairs should produce different alignment scores, got diff={sum_sq_diff:.4}");
    }

    #[test]
    fn diagnose_dataset_batch_processes_all_pairs() {
        let concepts = vec![
            pipeline(), revenue_target(), message_queue(), event_stream(),
        ];
        let relations = vec![
            (0, 1, RelationType::Generative),
            (2, 3, RelationType::Transmissive),
        ];

        let results = diagnose_dataset(&concepts, &relations);
        assert_eq!(results.len(), 2);
        assert!(!results[0].correct);
        assert!(!results[1].correct);

        let summary = diagnostic_summary(&results);
        assert_eq!(summary.total_pairs, 2);
        assert_eq!(summary.correct_pairs, 0);
        assert_eq!(summary.pairs_with_fix_suggestions, 2);
    }

    #[test]
    fn diagnostic_summary_aggregates_correctly() {
        let results = vec![
            DiagnosticResult {
                actual_label: RelationType::Constraining,
                expected_label: RelationType::Constraining,
                correct: true,
                reason: "matched".into(),
                a_trigram: Trigram::Gen,
                b_trigram: Trigram::Kan,
                a_phase: WuXing::Earth,
                b_phase: WuXing::Water,
                fix_suggestion: None,
                alignment_scores: vec![(RelationType::Constraining, 1.0)],
            },
            DiagnosticResult {
                actual_label: RelationType::Receptive,
                expected_label: RelationType::Generative,
                correct: false,
                reason: "mismatch".into(),
                a_trigram: Trigram::Kan,
                b_trigram: Trigram::Qian,
                a_phase: WuXing::Water,
                b_phase: WuXing::Metal,
                fix_suggestion: Some("fix".into()),
                alignment_scores: vec![(RelationType::Generative, 0.5)],
            },
        ];

        let summary = diagnostic_summary(&results);
        assert_eq!(summary.total_pairs, 2);
        assert_eq!(summary.correct_pairs, 1);
        assert_eq!(summary.pairs_with_fix_suggestions, 1);
    }
}
