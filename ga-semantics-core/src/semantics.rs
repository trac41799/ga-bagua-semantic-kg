use crate::advanced::Trigram;
use crate::relation_type::RelationType;
use crate::rotor::Rotor;
use crate::Multivector;

/// Normalized semantic similarity between two multivectors.
/// Returns value in [-1, 1] where 1 = identical orientation, 0 = orthogonal, -1 = opposite.
pub fn semantic_similarity(a: &Multivector, b: &Multivector) -> f64 {
    let na = a.norm();
    let nb = b.norm();
    if na < f64::EPSILON || nb < f64::EPSILON {
        return 0.0;
    }
    let scalar = a.geo_product(&b.reverse()).scalar();
    (scalar / (na * nb)).clamp(-1.0, 1.0)
}

/// Dominant-role-weighted similarity: emphasizes alignment in dimensions where
/// BOTH concepts are strong. Uses sign-aware product-of-magnitudes normalized
/// by the product of individual norms. Returns [-1, 1].
pub fn dominant_similarity(a: &Multivector, b: &Multivector) -> f64 {
    let ca = a.coefficients();
    let cb = b.coefficients();
    let mut dot = 0.0;
    let mut na2 = 0.0;
    let mut nb2 = 0.0;
    for i in 0..8 {
        let wa = ca[i].abs();
        let wb = cb[i].abs();
        let sign = if ca[i] * cb[i] >= 0.0 { 1.0 } else { -1.0 };
        dot += wa * wb * sign;
        na2 += ca[i] * ca[i];
        nb2 += cb[i] * cb[i];
    }
    let norm = (na2 * nb2).sqrt();
    if norm < f64::EPSILON { return 0.0; }
    dot / norm
}

/// Normalized semantic difference (orthogonality measure).
/// Returns value in [0, 1] where 0 = identical, larger = more different.
pub fn semantic_difference(a: &Multivector, b: &Multivector) -> f64 {
    let na = a.norm();
    let nb = b.norm();
    if na < f64::EPSILON || nb < f64::EPSILON {
        return 1.0;
    }
    // Bivector magnitude of geometric product / (|a| * |b|)
    let gp = a.geo_product(b);
    let bivector_norm = gp.grade_projection(2).norm();
    (bivector_norm / (na * nb)).clamp(0.0, 1.0)
}

/// Classify the relationship between two multivectors as a semantic role label.
/// Returns the `RelationType` (e.g. causal, generative, constraining) corresponding
/// to the dominant blade of the geometric product.
pub fn semantic_relation(a: &Multivector, b: &Multivector) -> RelationType {
    let (role, _) = RelationType::from_pair(a, b);
    role
}

/// Strength of the relationship (magnitude of geometric product).
pub fn relation_strength(a: &Multivector, b: &Multivector) -> f64 {
    a.geo_product(b).norm()
}

/// Check if two multivectors are contradictory.
/// High bivector magnitude relative to total product magnitude indicates contradiction.
pub fn is_contradictory(a: &Multivector, b: &Multivector, threshold: f64) -> bool {
    let gp = a.geo_product(b);
    let total = gp.norm();
    if total < f64::EPSILON {
        return false;
    }
    let bivector_norm = gp.grade_projection(2).norm();
    (bivector_norm / total) > threshold
}

/// Analogy: "A is to B as C is to ?"
/// Uses WuXing cycle dynamics: finds the cycle relationship between A and B
/// (generate, control, reverse-generate, reverse-control), then applies it to C to predict D.
/// For phases with 2 trigrams, picks the first for generative/controlling, second for receptive/influential.
pub fn analogy(a: &Multivector, b: &Multivector, c: &Multivector) -> Option<Multivector> {
    let ta = a.dominant_trigram();
    let tb = b.dominant_trigram();
    let tc = c.dominant_trigram();
    let wa = ta.wuxing_phase();
    let wb = tb.wuxing_phase();
    let wc = tc.wuxing_phase();

    use crate::advanced::WuXing;
    let all = [WuXing::Wood, WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water];

    let (pred_w, use_first) = if wa == wb {
        return Some(*c);
    } else if wa.generate() == wb {
        (Some(wc.generate()), true)
    } else if wa.control() == wb {
        (Some(wc.control()), false)
    } else if wb.generate() == wa {
        (all.iter().find(|&&w| w.generate() == wc).copied(), false)
    } else if wb.control() == wa {
        (all.iter().find(|&&w| w.control() == wc).copied(), true)
    } else {
        (None, true)
    };

    let pred_w = pred_w?;
    let trigrams = pred_w.trigrams();
    let pred_trigram = if use_first { trigrams[0] } else { trigrams[trigrams.len() - 1] };
    let role = crate::relation_type::RelationType::from_trigram(pred_trigram);
    Some(Multivector::from_blade(role.blade(), 1.0))
}

/// Confidence of analogy given expected result.
pub fn analogy_confidence(
    a: &Multivector, b: &Multivector,
    c: &Multivector, expected: &Multivector,
) -> f64 {
    analogy(a, b, c)
        .map(|result| semantic_similarity(&result, expected))
        .unwrap_or(0.0)
}

/// Batch analogy: average confidence across multiple test cases.
pub fn analogy_batch(pairs: &[(&Multivector, &Multivector, &Multivector, &Multivector)]) -> f64 {
    if pairs.is_empty() {
        return 0.0;
    }
    let total: f64 = pairs.iter().map(|(a, b, c, d)| analogy_confidence(a, b, c, d)).sum();
    total / pairs.len() as f64
}

/// Compose two relations (rotors). Apply r1 then r2.
pub fn compose_relations(r1: &Rotor, r2: &Rotor) -> Rotor {
    r2.compose(r1)
}

/// Compose a chain of relations (rotors). Apply in order.
pub fn compose_chain(relations: &[Rotor]) -> Rotor {
    relations.iter().cloned().fold(Rotor::identity(), |acc, r| r.compose(&acc))
}

/// Invert a relation (rotor).
pub fn inverse_relation(r: &Rotor) -> Rotor {
    r.inverse_rotor()
}

/// A context transformation — wraps a rotor that shifts the semantic frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Context(Rotor);

impl Context {
    pub fn new(rotor: Rotor) -> Self { Context(rotor) }

    pub fn identity() -> Self { Context(Rotor::identity()) }

    pub fn from_trigram_transform(from: Trigram, to: Trigram) -> Option<Self> {
        // Both trigrams must be bivectors (grade 2) to form a rotor
        if from.grade() != 2 || to.grade() != 2 {
            return None;
        }
        // Compute a rotor that rotates from 'from' to 'to' in the plane they span
        // The rotor R = to * from⁻¹ for unit bivectors
        let from_mv = Multivector::from_blade(from.blade(), 1.0);
        let to_mv = Multivector::from_blade(to.blade(), 1.0);
        let from_inv = from_mv.inverse().ok()?;
        let rotor_mv = to_mv.geo_product(&from_inv);
        // Normalize to get a pure rotor
        let n = rotor_mv.norm();
        if n < f64::EPSILON { return None; }
        let normalized = rotor_mv * (1.0 / n);
        // Build rotor from scalar + bivector parts
        let scalar_part = normalized.grade_projection(0);
        let bivector_part = normalized.grade_projection(2);
        let result_mv = scalar_part + bivector_part;
        let rotor = Rotor::from_multivector(result_mv)?;
        Some(Context(rotor))
    }

    pub fn apply(&self, mv: &Multivector) -> Multivector {
        self.0.apply(mv)
    }

    pub fn compose(&self, other: &Context) -> Context {
        Context(other.0.compose(&self.0))
    }

    pub fn rotor(&self) -> &Rotor { &self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Blade;

    fn make_v(c0: f64, c1: f64, c2: f64, c3: f64) -> Multivector {
        Multivector::new([c0, c1, c2, c3, 0.0, 0.0, 0.0, 0.0])
    }

    #[test]
    fn similarity_identical_is_one() {
        let a = make_v(1.0, 2.0, 0.0, 0.0);
        let sim = semantic_similarity(&a, &a);
        assert!((sim - 1.0).abs() < 1e-10);
    }

    #[test]
    fn similarity_symmetric() {
        let a = make_v(1.0, 2.0, 0.0, 0.0);
        let b = make_v(3.0, 4.0, 0.0, 0.0);
        let sim_ab = semantic_similarity(&a, &b);
        let sim_ba = semantic_similarity(&b, &a);
        assert!((sim_ab - sim_ba).abs() < 1e-10);
    }

    #[test]
    fn similarity_range() {
        let a = make_v(1.0, 0.0, 0.0, 0.0);
        let b = make_v(0.0, 1.0, 0.0, 0.0);
        let sim = semantic_similarity(&a, &b);
        assert!(sim >= -1.0 && sim <= 1.0);
    }

    #[test]
    fn similarity_zero_norm_returns_zero() {
        let a = Multivector::zero();
        let b = make_v(1.0, 0.0, 0.0, 0.0);
        assert_eq!(semantic_similarity(&a, &b), 0.0);
    }

    #[test]
    fn difference_identical_is_zero() {
        let a = make_v(1.0, 2.0, 1.0, 0.0);
        assert!((semantic_difference(&a, &a)).abs() < 1e-10);
    }

    #[test]
    fn difference_range() {
        let a = make_v(1.0, 0.0, 0.0, 0.0);
        let b = Multivector::from_blade(Blade::E12, 1.0);
        let diff = semantic_difference(&a, &b);
        assert!(diff >= 0.0 && diff <= 1.0);
    }

    #[test]
    fn relation_type_returns_role_label() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let rel = semantic_relation(&a, &b);
        assert_eq!(rel, RelationType::Receptive);
    }

    #[test]
    fn relation_strength_nonzero() {
        let a = make_v(1.0, 0.0, 0.0, 0.0);
        let b = make_v(2.0, 0.0, 0.0, 0.0);
        assert!(relation_strength(&a, &b) > 0.0);
    }

    #[test]
    fn is_contradictory_self_is_false() {
        let a = make_v(1.0, 2.0, 0.0, 0.0);
        assert!(!is_contradictory(&a, &a, 0.5));
    }

    #[test]
    fn analogy_self_to_self_is_identity() {
        let a = make_v(1.0, 2.0, 0.0, 0.0);
        let result = analogy(&a, &a, &a);
        assert!(result.is_some());
        assert!(result.unwrap().approx_eq(&a, 1e-10));
    }

    #[test]
    fn analogy_zero_norm_still_works() {
        let a = Multivector::zero();
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let result = analogy(&a, &b, &b);
        assert!(result.is_some());
    }

    #[test]
    fn compose_rotors_chain() {
        let r1 = Rotor::new(0.3, Blade::E12).unwrap();
        let r2 = Rotor::new(0.4, Blade::E23).unwrap();
        let composed = compose_relations(&r1, &r2);
        let v = Multivector::from_blade(Blade::E1, 1.0);
        // compose_relations(r1, r2) applies r1 then r2
        let stepwise = r2.apply(&r1.apply(&v));
        let combined = composed.apply(&v);
        assert!(stepwise.approx_eq(&combined, 1e-10));
    }

    #[test]
    fn compose_chain_empty_is_identity() {
        let r = compose_chain(&[]);
        let v = Multivector::from_blade(Blade::E1, 1.0);
        assert!(r.apply(&v).approx_eq(&v, 1e-10));
    }

    #[test]
    fn context_identity() {
        let ctx = Context::identity();
        let v = Multivector::from_blade(Blade::E1, 1.0);
        assert!(ctx.apply(&v).approx_eq(&v, 1e-10));
    }

    #[test]
    fn context_compose() {
        let r = Rotor::new(0.5, Blade::E12).unwrap();
        let ctx1 = Context::new(r);
        let ctx2 = Context::identity();
        let composed = ctx1.compose(&ctx2);
        let v = Multivector::from_blade(Blade::E1, 1.0);
        assert!(ctx1.apply(&v).approx_eq(&composed.apply(&v), 1e-10));
    }
}
