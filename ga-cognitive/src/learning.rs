use ga_semantics_core::prelude::*;

pub struct LearningPath {
    pub ordered_concepts: Vec<LearningStep>,
    pub cycle_completeness: f64,
}

pub struct LearningStep {
    pub name: String,
    pub phase: String,
    pub encoding: [f64; 8],
}

pub fn generate_learning_path(concepts: &[(String, [f64; 8])]) -> LearningPath {
    let phase_order = [
        ("Wood", 0), ("Fire", 1), ("Earth", 2), ("Metal", 3), ("Water", 4)
    ];

    let mut steps: Vec<(usize, LearningStep)> = vec![];
    for (name, enc) in concepts {
        let mv = Multivector::new(*enc);
        let dominant = mv.dominant_role();
        let phase_name = format!("{:?}", dominant.wuxing_phase());
        let phase_idx = phase_order.iter()
            .find(|(p, _)| *p == phase_name)
            .map(|(_, i)| *i)
            .unwrap_or(0);

        steps.push((phase_idx, LearningStep {
            name: name.clone(),
            phase: phase_name,
            encoding: *enc,
        }));
    }

    steps.sort_by_key(|(idx, _)| *idx);

    let ordered: Vec<LearningStep> = steps.into_iter().map(|(_, s)| s).collect();
    let phases_covered: std::collections::HashSet<&str> = ordered.iter().map(|s| s.phase.as_str()).collect();
    let completeness = phases_covered.len() as f64 / 5.0;

    LearningPath { ordered_concepts: ordered, cycle_completeness: completeness }
}

pub fn detect_prerequisites(concepts: &[(String, [f64; 8])]) -> Vec<(String, String)> {
    let mut prereqs = vec![];
    for i in 0..concepts.len() {
        for j in (i + 1)..concepts.len() {
            let mv_a = Multivector::new(concepts[i].1);
            let mv_b = Multivector::new(concepts[j].1);
            let (role, _) = RelationType::from_pair(&mv_a, &mv_b);
            if let RelationType::Constraining = role {
                prereqs.push((concepts[i].0.clone(), concepts[j].0.clone()));
            }
        }
    }
    prereqs
}

#[cfg(test)]
mod tests {
    use super::*;
    use ga_semantics_core::blade::Blade;

    #[test]
    fn generate_learning_path_orders_by_wuxing() {
        let e1 = Multivector::from_blade(Blade::E1, 1.0);
        let e3 = Multivector::from_blade(Blade::E3, 1.0);
        let scalar = Multivector::from_blade(Blade::Scalar, 1.0);
        let concepts = vec![
            ("EarthConcept".to_string(), *scalar.coefficients()),
            ("WoodConcept".to_string(), *e1.coefficients()),
            ("Earth2".to_string(), *e3.coefficients()),
        ];
        let path = generate_learning_path(&concepts);
        assert!(path.cycle_completeness > 0.0);
        assert_eq!(path.ordered_concepts.len(), 3);
        assert_eq!(path.ordered_concepts[0].phase, "Wood");
    }

    #[test]
    fn generate_learning_path_empty_returns_zero_completeness() {
        let concepts: Vec<(String, [f64; 8])> = vec![];
        let path = generate_learning_path(&concepts);
        assert_eq!(path.cycle_completeness, 0.0);
        assert!(path.ordered_concepts.is_empty());
    }

    #[test]
    fn generate_learning_path_single_concept() {
        let mv = Multivector::from_blade(Blade::E2, 1.0);
        let concepts = vec![("WaterConcept".to_string(), *mv.coefficients())];
        let path = generate_learning_path(&concepts);
        assert_eq!(path.ordered_concepts.len(), 1);
        assert!(path.cycle_completeness > 0.0);
    }

    #[test]
    fn detect_prerequisites_works() {
        let a = Multivector::from_blade(Blade::E3, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let c = Multivector::from_blade(Blade::Scalar, 1.0);
        let concepts = vec![
            ("GenConcept".to_string(), *a.coefficients()),
            ("KanConcept".to_string(), *b.coefficients()),
            ("KunConcept".to_string(), *c.coefficients()),
        ];
        let prereqs = detect_prerequisites(&concepts);
        assert!(!prereqs.is_empty());
    }
}
