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

/// Retrieval-optimized similarity: when both concepts share the same dominant
/// role, suppresses the dominant blade and ranks by secondary coefficient patterns.
/// This breaks the tie between same-role concepts that dominant_similarity
/// scores identically, producing meaningful within-role ranking.
///
/// When concepts have DIFFERENT dominant roles, falls back to dominant_similarity.
pub fn fingerprint_similarity(a: &Multivector, b: &Multivector) -> f64 {
    let dom_a = a.dominant_trigram().blade().index();
    let dom_b = b.dominant_trigram().blade().index();

    if dom_a != dom_b {
        return dominant_similarity(a, b);
    }

    // Same dominant role — suppress the dominant blade, emphasize secondary patterns
    let ca = a.coefficients();
    let cb = b.coefficients();
    let mut dot = 0.0;
    let mut na2 = 0.0;
    let mut nb2 = 0.0;
    for i in 0..8 {
        let w = if i == dom_a { 0.15 } else { 1.0 }; // Suppress dominant, boost secondary
        let wa = ca[i].abs();
        let wb = cb[i].abs();
        let sign = if ca[i] * cb[i] >= 0.0 { 1.0 } else { -1.0 };
        dot += wa * wb * sign * w;
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
///
/// For phases with 2 trigrams, selects the predicted trigram based on A's position
/// within its own WuXing phase:
///   - Generate: if A is the first trigram of its phase, pick the second trigram of
///     the predicted phase (yielding receives from active). If A is the second trigram,
///     pick the first (active receives from yielding).
///   - Control: if A is the first trigram of its phase, pick the first trigram of the
///     predicted phase (active controls active). If A is the second, pick the second
///     (yielding controls yielding).
pub fn analogy(a: &Multivector, b: &Multivector, c: &Multivector) -> Option<Multivector> {
    let ta = a.dominant_trigram();
    let tb = b.dominant_trigram();
    let tc = c.dominant_trigram();
    let wa = ta.wuxing_phase();
    let wb = tb.wuxing_phase();

    use crate::advanced::WuXing;
    let all = [WuXing::Wood, WuXing::Fire, WuXing::Earth, WuXing::Metal, WuXing::Water];

    let (pred_w, use_first) = if wa == wb {
        return Some(*c);
    } else if wa.generate() == wb {
        let use_first = ta != wa.trigrams()[0];
        (Some(tc.wuxing_phase().generate()), use_first)
    } else if wa.control() == wb {
        let use_first = ta == wa.trigrams()[0];
        (Some(tc.wuxing_phase().control()), use_first)
    } else if wb.generate() == wa {
        let use_first = ta != wa.trigrams()[0];
        (all.iter().find(|&&w| w.generate() == tc.wuxing_phase()).copied(), use_first)
    } else if wb.control() == wa {
        let use_first = ta == wa.trigrams()[0];
        (all.iter().find(|&&w| w.control() == tc.wuxing_phase()).copied(), use_first)
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

/// Belief revision: find the rotor R such that R * old_mv * R̃ ≈ new_mv.
/// Constructs a Rotor from the bivector of the geometric product of old and new,
/// representing the rotation plane that transforms old_mv toward new_mv.
/// Returns None if either input is degenerate.
pub fn belief_revise(old_mv: &Multivector, new_mv: &Multivector) -> Option<Rotor> {
    let na = old_mv.norm();
    let nb = new_mv.norm();
    if na < f64::EPSILON || nb < f64::EPSILON {
        return None;
    }

    // R = normalize(1 + B*A⁻¹) ensures the rotor has scalar + bivector parts
    let a_inv = old_mv.inverse().ok()?;
    let gp = new_mv.geo_product(&a_inv);

    let one = Multivector::one();
    let sum = one + gp;

    let n = sum.norm();
    if n < f64::EPSILON {
        return None;
    }
    let normalized = sum * (1.0 / n);

    // Extract scalar + bivector parts for a pure rotor
    let scalar_part = normalized.grade_projection(0);
    let bivector_part = normalized.grade_projection(2);
    let result_mv = scalar_part + bivector_part;

    Rotor::from_multivector(result_mv)
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
    fn belief_revise_identity() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let r = belief_revise(&a, &a).unwrap();
        let rotated = r.apply(&a);
        assert!(rotated.approx_eq(&a, 1e-10));
    }

    #[test]
    fn belief_revise_rotates_vector() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let r = belief_revise(&a, &b).unwrap();
        let rotated = r.apply(&a);
        assert!(rotated.approx_eq(&b, 1e-10), "R should rotator E1 to E2, got {:?}", rotated);
    }

    #[test]
    fn belief_revise_degenerate_returns_none() {
        let zero = Multivector::zero();
        let a = Multivector::from_blade(Blade::E1, 1.0);
        assert!(belief_revise(&zero, &a).is_none());
        assert!(belief_revise(&a, &zero).is_none());
    }

    #[test]
    fn belief_revise_bivectors() {
        let a = Multivector::from_blade(Blade::E12, 1.0);
        let b = Multivector::from_blade(Blade::E23, 1.0);
        let r = belief_revise(&a, &b).unwrap();
        let rotated = r.apply(&a);
        assert!(rotated.approx_eq(&b, 1e-10), "E12 rotated by R should approx equal E23");
    }
}

/// Decompose the geometric product A*B into its grade components
/// and return the normalized magnitude of each grade as a 4-element array
/// [scalar, vector, bivector, trivector].
///
/// This "relationship spectrum" captures the TYPE of interaction:
///   - High scalar → alignment/similarity (same role, compatible)
///   - High vector → directional difference (asymmetric flow)
///   - High bivector → rotational tension (contradiction, torque)
///   - High trivector → higher-order transformation (complex dynamics)
///
/// The spectrum is normalized to sum to 1.0 (a distribution over grades).
pub fn relationship_spectrum(a: &Multivector, b: &Multivector) -> [f64; 4] {
    let gp = a.geo_product(b);
    let total = gp.norm();
    if total < f64::EPSILON {
        return [0.0; 4];
    }
    let g0 = gp.grade_projection(0).norm();
    let g1 = gp.grade_projection(1).norm();
    let g2 = gp.grade_projection(2).norm();
    let g3 = gp.grade_projection(3).norm();
    let sum = g0 + g1 + g2 + g3;
    if sum < f64::EPSILON {
        return [0.0; 4];
    }
    [g0 / sum, g1 / sum, g2 / sum, g3 / sum]
}

/// Predict how a concept EVOLVES when one of its three aspects changes.
/// Each Bagua trigram has 3 lines (bottom=intent/purpose, middle=method/
/// mechanism, top=effect/outcome). Flipping one line produces the NEXT state
/// of the concept — what it becomes if that aspect transforms.
///
/// Returns the Multivector of the evolved concept, encoding the new trigram
/// at full strength (coefficient 1.0 on the new blade).
pub fn evolve_concept(mv: &Multivector, line: usize) -> Option<Multivector> {
    let trigram = mv.dominant_trigram();
    let next_trigram = trigram.transform_line(line)?;
    let role = crate::relation_type::RelationType::from_trigram(next_trigram);
    Some(Multivector::from_blade(role.blade(), 1.0))
}

/// All three possible evolutions of a concept (one per line flip).
pub fn all_evolutions(mv: &Multivector) -> [Multivector; 3] {
    [
        evolve_concept(mv, 0).unwrap_or(*mv),
        evolve_concept(mv, 1).unwrap_or(*mv),
        evolve_concept(mv, 2).unwrap_or(*mv),
    ]
}

#[cfg(test)]
mod spectrum_tests {
    use super::*;
    use crate::Blade;

    #[test]
    fn spectrum_same_role_high_scalar() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E1, 1.0);
        let s = relationship_spectrum(&a, &b);
        assert!(s[0] > 0.9, "identical vectors should have high scalar, got {:?}", s);
    }

    #[test]
    fn spectrum_orthogonal_high_bivector() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        let b = Multivector::from_blade(Blade::E2, 1.0);
        let s = relationship_spectrum(&a, &b);
        assert!(s[2] > 0.5, "orthogonal grade-1 vectors should have high bivector, got {:?}", s);
    }

    #[test]
    fn evolve_changes_trigram() {
        let kun = Multivector::from_blade(Blade::Scalar, 1.0); // Kun: [0,0,0]
        let evolved = evolve_concept(&kun, 0).unwrap(); // Flip bottom → [1,0,0] = Zhen
        assert_eq!(evolved.dominant_trigram(), crate::bagua::Trigram::Zhen);
    }

    #[test]
    fn evolve_all_produces_three_unique() {
        let mv = Multivector::from_blade(Blade::E12, 1.0); // Li: [1,0,1]
        let evolutions = all_evolutions(&mv);
        let t0 = evolutions[0].dominant_trigram();
        let t1 = evolutions[1].dominant_trigram();
        let t2 = evolutions[2].dominant_trigram();
        assert_ne!(t0, t1);
        assert_ne!(t1, t2);
        assert_ne!(t0, t2);
    }

    #[test]
    fn spectrum_sums_to_one() {
        let a = Multivector::new([0.5, 0.3, 0.1, 0.0, 0.2, 0.1, 0.15, 0.05]);
        let b = Multivector::new([0.2, 0.1, 0.6, 0.1, 0.0, 0.05, 0.3, 0.1]);
        let s = relationship_spectrum(&a, &b);
        let sum: f64 = s.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10, "spectrum should sum to 1.0, got {}", sum);
    }
}
