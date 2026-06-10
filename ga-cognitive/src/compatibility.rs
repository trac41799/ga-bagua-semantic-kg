use ga_semantics_core::prelude::*;

pub struct CompatibilityReport {
    pub person_a: String,
    pub person_b: String,
    pub compatibility_score: f64,
    pub relation_type: String,
    pub relation_description: String,
    pub interpretation: String,
}

pub fn personality_compatibility(a_encoding: &[f64; 8], b_encoding: &[f64; 8]) -> CompatibilityReport {
    use ga_semantics_core::semantics::semantic_similarity;

    let mv_a = Multivector::new(*a_encoding);
    let mv_b = Multivector::new(*b_encoding);
    let (role, confidence) = RelationType::from_pair(&mv_a, &mv_b);
    let sim = semantic_similarity(&mv_a, &mv_b);

    let comp_score = match role {
        RelationType::Generative | RelationType::Balancing => 0.5 + 0.5 * confidence,
        RelationType::Influential => 0.3 + 0.4 * confidence,
        RelationType::Constraining => 0.2 + 0.3 * confidence,
        RelationType::Receptive => 0.1 + 0.5 * sim,
        _ => 0.1 + 0.5 * confidence,
    };

    let interpretation = match role {
        RelationType::Generative => "Strong complementary pair — one enables the other",
        RelationType::Balancing => "Harmonious balance — mutual reflection and equilibrium",
        RelationType::Influential => "One influences the other — potential for growth",
        RelationType::Constraining => "One grounds the other — useful checks and balances",
        RelationType::Receptive => "Aligned in direction — works well together",
        _ => "Neutral compatibility",
    };

    CompatibilityReport {
        person_a: String::new(), person_b: String::new(),
        compatibility_score: comp_score.clamp(0.0, 1.0),
        relation_type: role.role_name().to_string(),
        relation_description: role.description().to_string(),
        interpretation: interpretation.to_string(),
    }
}

pub fn form_team(candidates: &[(String, [f64; 8])], team_size: usize) -> Vec<Vec<String>> {
    if candidates.is_empty() || team_size == 0 || candidates.len() < team_size {
        return vec![];
    }

    let mut best_score = -1.0;
    let mut best_indices: Vec<usize> = vec![];

    if team_size == 1 {
        return vec![vec![candidates[0].0.clone()]];
    }

    if team_size == 2 {
        for i in 0..candidates.len() {
            for j in (i + 1)..candidates.len() {
                let report = personality_compatibility(&candidates[i].1, &candidates[j].1);
                if report.compatibility_score > best_score {
                    best_score = report.compatibility_score;
                    best_indices = vec![i, j];
                }
            }
        }
    } else {
        let n = candidates.len();
        let k = team_size.min(n);
        let mut best_mean = -1.0;

        let mut combo: Vec<usize> = vec![0];
        while !combo.is_empty() {
            if combo.len() == k {
                let mut total = 0.0;
                let mut pairs = 0usize;
                for i in 0..k {
                    for j in (i + 1)..k {
                        let report = personality_compatibility(
                            &candidates[combo[i]].1,
                            &candidates[combo[j]].1,
                        );
                        total += report.compatibility_score;
                        pairs += 1;
                    }
                }
                let mean = if pairs > 0 { total / pairs as f64 } else { 0.0 };
                if mean > best_mean {
                    best_mean = mean;
                    best_indices = combo.clone();
                }
                let last_idx = *combo.last().unwrap();
                combo.pop();
                if last_idx + 1 < n {
                    combo.push(last_idx + 1);
                } else if let Some(last) = combo.last_mut() {
                    *last += 1;
                }
            } else {
                let next = *combo.last().unwrap() + 1;
                if next <= n - (k - combo.len()) {
                    combo.push(next);
                } else {
                    combo.pop();
                    if let Some(last) = combo.last_mut() {
                        *last += 1;
                    }
                }
            }
        }
    }

    if best_indices.is_empty() {
        best_indices = (0..team_size.min(candidates.len())).collect();
    }

    let team_names: Vec<String> = best_indices.iter().map(|&idx| candidates[idx].0.clone()).collect();
    vec![team_names]
}

pub fn form_best_team(candidates: &[(String, [f64; 8])], team_size: usize) -> Vec<String> {
    let teams = form_team(candidates, team_size);
    teams.first().cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::blade::Blade;

    #[test]
    fn personality_compatibility_self_is_balanced() {
        let enc = Multivector::from_blade(Blade::E1, 1.0);
        let report = personality_compatibility(enc.coefficients(), enc.coefficients());
        assert!(report.compatibility_score > 0.0);
        assert_eq!(report.relation_type, "receptive");
    }

    #[test]
    fn personality_compatibility_different_roles_works() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let report = personality_compatibility(a.coefficients(), b.coefficients());
        assert!(report.compatibility_score >= 0.0 && report.compatibility_score <= 1.0);
        assert!(!report.relation_type.is_empty());
    }

    #[test]
    fn form_team_pair_selects_best() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let c = Multivector::from_blade(Blade::Scalar, 1.0);
        let candidates = vec![
            ("Alice".to_string(), *a.coefficients()),
            ("Bob".to_string(), *b.coefficients()),
            ("Carol".to_string(), *c.coefficients()),
        ];
        let team = form_best_team(&candidates, 2);
        assert_eq!(team.len(), 2);
    }

    #[test]
    fn form_team_empty_returns_empty() {
        let candidates: Vec<(String, [f64; 8])> = vec![];
        let team = form_best_team(&candidates, 2);
        assert!(team.is_empty());
    }

    #[test]
    fn form_team_not_enough_candidates() {
        let enc = [0.1; 8];
        let candidates = vec![("Solo".to_string(), enc)];
        let team = form_best_team(&candidates, 3);
        assert!(team.is_empty());
    }
}
