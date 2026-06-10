use crate::blade::Blade;
use crate::error::AlgebraicError;
use std::ops::{Add, Mul, Neg, Sub};

/// Cl(3) geometric product multiplication table.
/// `PROD_TABLE[i][j] = (result_index, sign_factor)`
/// where sign_factor is either 1.0 or -1.0.
const PROD_TABLE: [[(usize, f64); 8]; 8] = {
    let s: f64 = 1.0;
    let n: f64 = -1.0;
    [
        // Row 0: Scalar (1)
        [(0, s), (1, s), (2, s), (3, s), (4, s), (5, s), (6, s), (7, s)],
        // Row 1: e1
        [(1, s), (0, s), (4, s), (6, n), (2, s), (7, s), (3, n), (5, s)],
        // Row 2: e2
        [(2, s), (4, n), (0, s), (5, s), (1, n), (3, s), (7, s), (6, s)],
        // Row 3: e3
        [(3, s), (6, s), (5, n), (0, s), (7, s), (2, n), (1, s), (4, s)],
        // Row 4: e12
        [(4, s), (2, n), (1, s), (7, s), (0, n), (6, n), (5, s), (3, n)],
        // Row 5: e23
        [(5, s), (7, s), (3, n), (2, s), (6, s), (0, n), (4, n), (1, n)],
        // Row 6: e31
        [(6, s), (3, s), (7, s), (1, n), (5, n), (4, s), (0, n), (2, n)],
        // Row 7: e123
        [        (7, s), (5, s), (6, s), (4, s), (3, n), (1, n), (2, n), (0, n)],
    ]
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multivector {
    coefficients: [f64; 8],
}

impl Multivector {
    pub fn new(coefficients: [f64; 8]) -> Self {
        Multivector { coefficients }
    }

    pub fn zero() -> Self {
        Multivector {
            coefficients: [0.0; 8],
        }
    }

    pub fn one() -> Self {
        let mut v = [0.0; 8];
        v[0] = 1.0;
        Multivector::new(v)
    }

    pub fn from_blade(blade: Blade, coeff: f64) -> Self {
        blade.to_multivector(coeff)
    }

    pub fn coefficient(&self, blade: usize) -> f64 {
        self.coefficients[blade]
    }

    pub fn coefficients(&self) -> &[f64; 8] {
        &self.coefficients
    }

    pub fn scalar(&self) -> f64 {
        self.coefficients[0]
    }

    pub fn reverse(&self) -> Self {
        let mut c = self.coefficients;
        c[4] = -c[4];
        c[5] = -c[5];
        c[6] = -c[6];
        c[7] = -c[7];
        Multivector::new(c)
    }

    pub fn grade_involution(&self) -> Self {
        let mut c = self.coefficients;
        c[1] = -c[1];
        c[2] = -c[2];
        c[3] = -c[3];
        c[7] = -c[7];
        Multivector::new(c)
    }

    pub fn clifford_conjugate(&self) -> Self {
        let mut c = self.coefficients;
        c[1] = -c[1];
        c[2] = -c[2];
        c[3] = -c[3];
        c[4] = -c[4];
        c[5] = -c[5];
        c[6] = -c[6];
        Multivector::new(c)
    }

    pub fn norm_squared(&self) -> f64 {
        // norm squared = scalar part of A * reverse(A)
        let rev = self.reverse();
        let product = self.geo_product(&rev);
        product.coefficients[0]
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn inverse(&self) -> Result<Self, AlgebraicError> {
        let conj = self.clifford_conjugate();
        let product = self.geo_product(&conj);
        let ns = product.coefficients[0];
        if ns.abs() < f64::EPSILON {
            return Err(AlgebraicError::ZeroNorm);
        }
        Ok(conj * (1.0 / ns))
    }

    pub fn dualize(&self) -> Self {
        // Multiply by pseudoscalar e123
        let pseudoscalar = Multivector::from_blade(Blade::E123, 1.0);
        self.geo_product(&pseudoscalar)
    }

    pub fn geo_product(&self, other: &Self) -> Self {
        let mut result = [0.0; 8];
        let a = &self.coefficients;
        let b = &other.coefficients;

        for i in 0..8 {
            let ai = a[i];
            if ai == 0.0 {
                continue;
            }
            let row = &PROD_TABLE[i];
            for j in 0..8 {
                let bj = b[j];
                if bj == 0.0 {
                    continue;
                }
                let (k, sign) = row[j];
                result[k] += ai * bj * sign;
            }
        }

        Multivector::new(result)
    }

    pub fn inner_product(&self, other: &Self) -> f64 {
        self.geo_product(other).coefficients[0]
    }

    pub fn wedge_product(&self, other: &Self) -> Self {
        // The wedge product is the grade > 0 part of the geometric product
        let gp = self.geo_product(other);
        let mut result = gp.coefficients;
        result[0] = 0.0; // remove scalar part
        Multivector::new(result)
    }

    pub fn grade_projection(&self, grade: usize) -> Self {
        let mut c = [0.0; 8];
        for blade in Blade::GRADE_BLADES[grade] {
            c[blade.index()] = self.coefficients[blade.index()];
        }
        Multivector::new(c)
    }

    /// Returns the canonical Bagua trigram with the strongest coefficient (internal).
    pub fn dominant_trigram(&self) -> crate::bagua::Trigram {
        let mut max_idx = 0usize;
        let mut max_val = self.coefficients[0].abs();
        for i in 1..8 {
            let val = self.coefficients[i].abs();
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }
        crate::bagua::Trigram::from_index(max_idx).unwrap_or(crate::bagua::Trigram::Kun)
    }

    /// Returns the semantic role label with the strongest coefficient.
    pub fn dominant_role(&self) -> crate::relation_type::RelationType {
        let trigram = self.dominant_trigram();
        crate::relation_type::RelationType::from_trigram(trigram)
    }

    /// Encoding sharpness: how concentrated the encoding is on its dominant role.
    /// Returns value in [0, 1] where higher = more clearly defined concept.
    /// Random uniform vectors have sharpness ~0.13-0.18; well-encoded concepts
    /// have sharpness ~0.30-0.50. Used as a signal-quality gate in classification.
    pub fn encoding_sharpness(&self) -> f64 {
        let coeffs = &self.coefficients;
        let max_abs = coeffs.iter().map(|c| c.abs()).fold(0.0f64, f64::max);
        let sum_abs: f64 = coeffs.iter().map(|c| c.abs()).sum();
        if sum_abs < f64::EPSILON { return 0.0; }
        max_abs / sum_abs
    }

    pub fn role_weights(&self) -> [f64; 8] {
        self.coefficients
    }

    pub fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
        for i in 0..8 {
            if (self.coefficients[i] - other.coefficients[i]).abs() > epsilon {
                return false;
            }
        }
        true
    }
}

// Arithmetic trait implementations

impl Add for Multivector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut c = [0.0; 8];
        for (i, val) in c.iter_mut().enumerate() {
            *val = self.coefficients[i] + other.coefficients[i];
        }
        Multivector::new(c)
    }
}

impl Sub for Multivector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut c = [0.0; 8];
        for (i, val) in c.iter_mut().enumerate() {
            *val = self.coefficients[i] - other.coefficients[i];
        }
        Multivector::new(c)
    }
}

impl Neg for Multivector {
    type Output = Self;
    fn neg(self) -> Self {
        let mut c = [0.0; 8];
        for (i, val) in c.iter_mut().enumerate() {
            *val = -self.coefficients[i];
        }
        Multivector::new(c)
    }
}

impl Mul<f64> for Multivector {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        let mut c = [0.0; 8];
        for (i, val) in c.iter_mut().enumerate() {
            *val = self.coefficients[i] * scalar;
        }
        Multivector::new(c)
    }
}

impl Mul<Multivector> for f64 {
    type Output = Multivector;
    fn mul(self, mv: Multivector) -> Multivector {
        mv * self
    }
}

impl Mul for Multivector {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.geo_product(&other)
    }
}

impl std::fmt::Display for Multivector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blade_names = ["1", "e1", "e2", "e3", "e12", "e23", "e31", "e123"];
        let mut terms = Vec::new();
        for (i, name) in blade_names.iter().enumerate() {
            let c = self.coefficients[i];
            if c != 0.0 {
                terms.push(format!("{:.4}*{}", c, name));
            }
        }
        if terms.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", terms.join(" + "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blade::Blade;

    #[test]
    fn multivector_new_and_coefficient() {
        let mv = Multivector::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(mv.coefficient(0), 1.0);
        assert_eq!(mv.coefficient(1), 2.0);
        assert_eq!(mv.coefficient(7), 8.0);
    }

    #[test]
    fn zero_multivector() {
        let mv = Multivector::zero();
        for i in 0..8 {
            assert_eq!(mv.coefficient(i), 0.0);
        }
    }

    #[test]
    fn one_multivector() {
        let mv = Multivector::one();
        assert_eq!(mv.coefficient(0), 1.0);
        for i in 1..8 {
            assert_eq!(mv.coefficient(i), 0.0);
        }
    }

    #[test]
    fn from_blade_construction() {
        let mv = Multivector::from_blade(Blade::E12, 3.0);
        assert_eq!(mv.coefficient(Blade::E12.index()), 3.0);
        for i in [0, 1, 2, 3, 5, 6, 7] {
            assert_eq!(mv.coefficient(i), 0.0);
        }
    }

    #[test]
    fn add_multivectors() {
        let a = Multivector::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = Multivector::new([2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let c = a + b;
        assert_eq!(c.coefficient(0), 3.0);
    }

    #[test]
    fn sub_multivectors() {
        let a = Multivector::new([5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = Multivector::new([3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let c = a - b;
        assert_eq!(c.coefficient(0), 2.0);
    }

    #[test]
    fn neg_multivector() {
        let a = Multivector::new([1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = -a;
        assert_eq!(b.coefficient(0), -1.0);
        assert_eq!(b.coefficient(1), 2.0);
    }

    #[test]
    fn scalar_multiplication() {
        let a = Multivector::from_blade(Blade::E1, 2.0);
        let b = a * 3.0;
        assert_eq!(b.coefficient(1), 6.0);
    }

    #[test]
    fn scalar_multiplication_left() {
        let a = Multivector::from_blade(Blade::E1, 2.0);
        let b = 3.0 * a;
        assert_eq!(b.coefficient(1), 6.0);
    }

    #[test]
    fn reverse_grade_0() {
        let mv = Multivector::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(mv.reverse(), mv);
    }

    #[test]
    fn reverse_grade_1() {
        let mv = Multivector::from_blade(Blade::E1, 5.0);
        assert_eq!(mv.reverse(), mv);
    }

    #[test]
    fn reverse_grade_2() {
        let mv = Multivector::from_blade(Blade::E12, 3.0);
        let rev = mv.reverse();
        assert_eq!(rev.coefficient(Blade::E12.index()), -3.0);
    }

    #[test]
    fn reverse_grade_3() {
        let mv = Multivector::from_blade(Blade::E123, 2.0);
        let rev = mv.reverse();
        assert_eq!(rev.coefficient(Blade::E123.index()), -2.0);
    }

    #[test]
    fn norm_squared_of_scalar() {
        let mv = Multivector::new([3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((mv.norm_squared() - 9.0).abs() < 1e-10);
    }

    #[test]
    fn norm_of_scalar() {
        let mv = Multivector::new([-4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((mv.norm() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn inverse_of_scalar() {
        let mv = Multivector::new([2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let inv = mv.inverse().unwrap();
        let product = mv.geo_product(&inv);
        assert!((product.scalar() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn zero_norm_returns_error() {
        let mv = Multivector::zero();
        assert!(mv.inverse().is_err());
    }

    #[test]
    fn dualize_twice_returns_negative() {
        let mv = Multivector::from_blade(Blade::E1, 1.0);
        let dual = mv.dualize();
        let double_dual = dual.dualize();
        assert!((double_dual.coefficient(1) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn geo_product_identity() {
        let a = Multivector::one();
        let b = Multivector::from_blade(Blade::E12, 2.5);
        let prod = a.geo_product(&b);
        assert!((prod.coefficient(Blade::E12.index()) - 2.5).abs() < 1e-10);
    }

    #[test]
    fn geo_product_e1_e2_is_e12() {
        let e1 = Multivector::from_blade(Blade::E1, 1.0);
        let e2 = Multivector::from_blade(Blade::E2, 1.0);
        let prod = e1.geo_product(&e2);
        assert!((prod.coefficient(Blade::E12.index()) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn geo_product_e2_e1_is_negative_e12() {
        let e1 = Multivector::from_blade(Blade::E1, 1.0);
        let e2 = Multivector::from_blade(Blade::E2, 1.0);
        let prod = e2.geo_product(&e1);
        assert!((prod.coefficient(Blade::E12.index()) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn geo_product_e1_e1_is_one() {
        let e1 = Multivector::from_blade(Blade::E1, 1.0);
        let prod = e1.geo_product(&e1);
        assert!((prod.scalar() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn geo_product_e123_e123_is_negative_one() {
        let ps = Multivector::from_blade(Blade::E123, 1.0);
        let prod = ps.geo_product(&ps);
        assert!((prod.scalar() - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn grade_projection_scalar() {
        let mv = Multivector::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let g0 = mv.grade_projection(0);
        assert_eq!(g0.coefficient(0), 1.0);
        for i in 1..8 {
            assert_eq!(g0.coefficient(i), 0.0);
        }
    }

    #[test]
    fn grade_projection_vector() {
        let mv = Multivector::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let g1 = mv.grade_projection(1);
        assert_eq!(g1.coefficient(1), 2.0);
        assert_eq!(g1.coefficient(2), 3.0);
        assert_eq!(g1.coefficient(3), 4.0);
        assert_eq!(g1.coefficient(0), 0.0);
    }

    #[test]
    fn inner_product_symmetric() {
        let a = Multivector::new([1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = Multivector::new([3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let ip = a.inner_product(&b);
        // geometric product scalar part: 1*3 + e1*e1 term: 2*4*1 = 8, total = 11
        assert!((ip - 11.0).abs() < 1e-10);
    }

    #[test]
    fn approx_eq_same() {
        let a = Multivector::new([1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!(a.approx_eq(&a, 1e-10));
    }

    #[test]
    fn approx_eq_different() {
        let a = Multivector::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = Multivector::new([1.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!(!a.approx_eq(&b, 1e-4));
        assert!(a.approx_eq(&b, 0.01));
    }

    #[test]
    fn encoding_sharpness_one_role_is_one() {
        let a = Multivector::from_blade(Blade::E1, 1.0);
        assert!((a.encoding_sharpness() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn encoding_sharpness_uniform_is_low() {
        let a = Multivector::new([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        assert!((a.encoding_sharpness() - 0.125).abs() < 1e-10);
    }

    #[test]
    fn encoding_sharpness_zero_is_zero() {
        let a = Multivector::zero();
        assert_eq!(a.encoding_sharpness(), 0.0);
    }

    #[test]
    fn encoding_sharpness_hand_tuned_is_moderate() {
        // LLM-encoded concept (Rate Limiter)
        let a = crate::encoding::llm_encode(&[0.04, -0.09, -0.51, 0.68, 0.21, -0.26, 0.17, -0.34]);
        let sharp = a.encoding_sharpness();
        assert!(sharp > 0.25, "hand-tuned encoding should have sharpness > 0.25, got {}", sharp);
        assert!(sharp < 0.65, "encoding is not single-role, got {}", sharp);
    }

    #[test]
    fn encoding_sharpness_random_is_below_threshold() {
        use crate::encoding::llm_encode;
        let mut below = 0usize;
        let mut seed: u64 = 0xCAFE;
        for _ in 0..100 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let s = seed;
            let raw = [
                ((s as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(3) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(7) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(11) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(13) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(17) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(19) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
                ((s.wrapping_mul(23) as f64) / (u64::MAX as f64)) * 2.0 - 1.0,
            ];
            let mv = llm_encode(&raw);
            if mv.encoding_sharpness() < 0.25 { below += 1; }
        }
        assert!(below >= 75, ">75% of random encodings should fall below 0.25 sharpness (got {}/100)", below);
    }
}
