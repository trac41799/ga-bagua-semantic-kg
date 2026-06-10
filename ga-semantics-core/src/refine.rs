use crate::encoding::llm_encode;
use crate::RelationType;

/// Given a known concept's WuXing phase, determines the phase the OTHER
/// concept must have for the specified relationship to hold.
///
/// `known_phase`: the WuXing phase of the concept we already know
/// `need_first`:  true = we need the phase of A (first in A→B relation)
///                false = we need the phase of B (second in A→B relation)
fn phase_for_relation(
    relation: RelationType,
    known_phase: crate::bagua::WuXing,
    need_first: bool,
) -> Option<crate::bagua::WuXing> {
    use crate::bagua::WuXing;
    let all = [WuXing::Wood, WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water];

    match (relation, need_first) {
        // A generates B: A's phase generates B's phase
        (RelationType::Generative, true) => {
            // Known = B, need A: A generates B → A is what generates B
            all.iter().find(|&&w| w.generate() == known_phase).copied()
        }
        (RelationType::Generative, false) => {
            // Known = A, need B: A generates B → B = A.generate()
            Some(known_phase.generate())
        }

        // A controls B: A's phase controls B's phase
        (RelationType::Constraining, true) => {
            all.iter().find(|&&w| w.control() == known_phase).copied()
        }
        (RelationType::Constraining, false) => {
            Some(known_phase.control())
        }

        // B generates A (receptive): A = what B generates, or B = what generates A
        (RelationType::Receptive, true) => {
            // Known = B, need A: A = B.generate()
            Some(known_phase.generate())
        }
        (RelationType::Receptive, false) => {
            // Known = A, need B: B generates A → B is what generates A
            all.iter().find(|&&w| w.generate() == known_phase).copied()
        }

        // B controls A (influential): A = what B controls, or B = what controls A
        (RelationType::Influential, true) => {
            // Known = B, need A: A = B.control()
            Some(known_phase.control())
        }
        (RelationType::Influential, false) => {
            // Known = A, need B: B controls A → B is what controls A
            all.iter().find(|&&w| w.control() == known_phase).copied()
        }

        // Balancing: same phase, complementary trigram
        (RelationType::Balancing, _) => Some(known_phase),

        // Clarifying: same phase, different trigram
        (RelationType::Clarifying, _) => Some(known_phase),

        // Causal / Transmissive: weakly mapped
        _ => None,
    }
}

/// When two independently-encoded concepts don't align with an expected
/// WuXing relationship, adjust the weaker concept's encoding to shift its
/// dominant trigram into the correct WuXing phase.
///
/// Returns adjusted coefficient arrays (None = no change needed).
pub fn refine_encoding_pair(
    coeffs_a: &[f64; 8],
    coeffs_b: &[f64; 8],
    expected_relation: RelationType,
) -> (Option<[f64; 8]>, Option<[f64; 8]>, f64) {
    let a = llm_encode(coeffs_a);
    let b = llm_encode(coeffs_b);
    let (actual, _) = RelationType::from_pair(&a, &b);

    if actual == expected_relation {
        return (None, None, 1.0);
    }

    let sharp_a = a.encoding_sharpness();
    let sharp_b = b.encoding_sharpness();

    // Adjust the concept with lower encoding sharpness (easier to shift).
    // If adjusting A: we know B's phase, need A's → need_first = true.
    // If adjusting B: we know A's phase, need B's → need_first = false.
    let (adjust_a, target_phase) = if sharp_a < sharp_b {
        let b_phase = b.dominant_trigram().wuxing_phase();
        let tgt = phase_for_relation(expected_relation, b_phase, true);
        (true, tgt)
    } else {
        let a_phase = a.dominant_trigram().wuxing_phase();
        let tgt = phase_for_relation(expected_relation, a_phase, false);
        (false, tgt)
    };

    let target_phase = match target_phase {
        Some(p) => p,
        None => return (None, None, 0.0),
    };

    let target_trigrams = target_phase.trigrams();
    let target_blade_idx = target_trigrams[0].blade().index();

    let adjust_coeffs = if adjust_a { coeffs_a } else { coeffs_b };
    let mut new_coeffs = *adjust_coeffs;

    // Amplify the target blade's raw coefficient so that after normalization
    // it becomes dominant and places the concept in the target WuXing phase.
    let current_val = new_coeffs[target_blade_idx];
    let sign = if current_val >= 0.0 { 1.0 } else { -1.0 };
    new_coeffs[target_blade_idx] = (current_val.abs() + 0.5) * sign;

    let result = if adjust_a {
        (Some(new_coeffs), None)
    } else {
        (None, Some(new_coeffs))
    };

    // Verify
    let a_new = llm_encode(&result.0.unwrap_or(*coeffs_a));
    let b_new = llm_encode(&result.1.unwrap_or(*coeffs_b));
    let (actual_new, _) = RelationType::from_pair(&a_new, &b_new);

    if actual_new == expected_relation {
        (result.0, result.1, 0.7)
    } else {
        // Stronger push — set the target blade to 1.0
        let mut strong_coeffs = new_coeffs;
        strong_coeffs[target_blade_idx] = if current_val >= 0.0 { 1.5 } else { -1.5 };

        let a_s = llm_encode(&(if adjust_a { strong_coeffs } else { *coeffs_a }));
        let b_s = llm_encode(&(if adjust_a { *coeffs_b } else { strong_coeffs }));
        let (rel, _) = RelationType::from_pair(&a_s, &b_s);

        if rel == expected_relation {
            if adjust_a {
                (Some(strong_coeffs), None, 0.5)
            } else {
                (None, Some(strong_coeffs), 0.5)
            }
        } else {
            (None, None, 0.0)
        }
    }
}

/// Refine all encodings iteratively until convergence.
pub fn refine_all_encodings(
    concepts: &mut [[f64; 8]],
    relations: &[(usize, usize, RelationType)],
    max_rounds: usize,
) -> (usize, usize) {
    let mut fixed = 0usize;
    let total = relations.len();

    for _round in 0..max_rounds {
        let mut changes = 0usize;
        for &(a_idx, b_idx, expected) in relations.iter() {
            let (adj_a, adj_b, _) = refine_encoding_pair(
                &concepts[a_idx], &concepts[b_idx], expected,
            );
            if let Some(new_a) = adj_a {
                concepts[a_idx] = new_a;
                changes += 1;
            }
            if let Some(new_b) = adj_b {
                concepts[b_idx] = new_b;
                changes += 1;
            }
        }
        fixed += changes;
        if changes == 0 { break; }
    }

    (fixed, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refine_same_role_stays_correct() {
        let ca = [0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34];
        let cb = [0.05, -0.30, -0.20, 0.75, 0.25, -0.10, 0.25, -0.20];
        let (adj_a, adj_b, conf) = refine_encoding_pair(&ca, &cb, RelationType::Receptive);
        assert!(adj_a.is_none());
        assert!(adj_b.is_none());
        assert!((conf - 1.0).abs() < 1e-10);
    }

    #[test]
    fn refine_misaligned_pair_gets_adjustment() {
        // Pipeline (transmissive) → Revenue (generative)
        let ca = [0.10, 0.15, 0.80, -0.05, -0.10, 0.15, 0.20, 0.20];
        let cb = [0.10, 0.30, 0.10, -0.10, 0.15, 0.10, 0.15, 0.85];
        let (adj_a, adj_b, _) = refine_encoding_pair(&ca, &cb, RelationType::Generative);
        assert!(adj_a.is_some() || adj_b.is_some(), "should suggest adjustment");

        let a_new = llm_encode(&adj_a.unwrap_or(ca));
        let b_new = llm_encode(&adj_b.unwrap_or(cb));
        let (rel, _) = RelationType::from_pair(&a_new, &b_new);
        assert_eq!(rel, RelationType::Generative,
            "adjusted pair should produce Generative, got {:?}", rel);
    }

    #[test]
    fn refine_all_converges() {
        let mut concepts = vec![
            [0.10, 0.15, 0.80, -0.05, -0.10, 0.15, 0.20, 0.20], // Pipeline
            [0.10, 0.30, 0.10, -0.10, 0.15, 0.10, 0.15, 0.85], // Revenue
            [0.15, 0.05, 0.05, 0.80, 0.30, 0.25, 0.10, 0.05], // Handbook
            [0.05, 0.10, 0.05, 0.85, -0.05, 0.25, 0.10, 0.10], // Budget
        ];
        let relations = vec![
            (0, 1, RelationType::Generative),
            (2, 3, RelationType::Receptive),
        ];
        let (fixed, _) = refine_all_encodings(&mut concepts, &relations, 10);
        assert!(fixed > 0, "should fix at least one misalignment");

        let (rel01, _) = RelationType::from_pair(
            &llm_encode(&concepts[0]), &llm_encode(&concepts[1]),
        );
        assert_eq!(rel01, RelationType::Generative,
            "Pipeline should generate Revenue after refinement, got {:?}", rel01);
    }
}
