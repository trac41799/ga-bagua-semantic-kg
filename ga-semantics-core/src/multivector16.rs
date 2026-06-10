use crate::Blade;

/// A 16-dimensional multivector in Cl(4) geometric algebra.
/// Cl(4) has 16 basis blades, double the resolution of Cl(3)'s 8 blades.
/// Each of the 8 Bagua trigrams maps to 2 blade indices (primary + secondary).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multivector16 {
    coeffs: [f64; 16],
}

/// Product table entry: (result_blade_index, sign)
type ProductEntry = (usize, f64);

/// Pre-computed Cl(4) geometric product table.
/// product_table[a][b] = (result_index, sign) for blade a * blade b.
fn get_product_table() -> &'static [[ProductEntry; 16]; 16] {
    use std::sync::OnceLock;
    static TABLE: OnceLock<[[ProductEntry; 16]; 16]> = OnceLock::new();
    TABLE.get_or_init(|| compute_product_table())
}

fn compute_product_table() -> [[ProductEntry; 16]; 16] {
    let mut table = [[(0usize, 0.0f64); 16]; 16];
    for a_bits in 0u8..16u8 {
        for b_bits in 0u8..16u8 {
            let result_bits = a_bits ^ b_bits;
            let sign = blade_product_sign(a_bits, b_bits);
            table[a_bits as usize][b_bits as usize] = (result_bits as usize, sign);
        }
    }
    table
}

/// Compute the sign of the geometric product of two basis blades.
/// Blade bits: bit i (0-3) = 1 means e_{i+1} is present.
/// Sign: (-1)^(number of transpositions to reorder the combined factors
/// into canonical e1∧e2∧e3∧e4 order)
fn blade_product_sign(a_bits: u8, b_bits: u8) -> f64 {
    let mut sign = 1.0f64;

    // For each bit i set in a, for each bit j set in b:
    // If i > j: one swap needed (ei is out of order relative to ej)
    // If i = j: ei*ei = 1, handled by XOR already (result bit is 0)
    for i in 0u8..4u8 {
        if (a_bits >> i) & 1 == 0 { continue; }
        for j in 0u8..4u8 {
            if (b_bits >> j) & 1 == 0 { continue; }
            if i > j {
                sign = -sign;
            }
        }
    }

    sign
}

impl Multivector16 {
    pub fn new(coeffs: [f64; 16]) -> Self {
        Multivector16 { coeffs }
    }

    pub fn zero() -> Self {
        Multivector16 { coeffs: [0.0; 16] }
    }

    pub fn from_blade(index: usize, value: f64) -> Self {
        let mut coeffs = [0.0; 16];
        if index < 16 {
            coeffs[index] = value;
        }
        Multivector16 { coeffs }
    }

    pub fn coefficients(&self) -> &[f64; 16] { &self.coeffs }
    pub fn coefficient(&self, index: usize) -> f64 {
        self.coeffs.get(index).copied().unwrap_or(0.0)
    }

    /// Geometric product A * B.
    pub fn geometric_product(&self, other: &Multivector16) -> Multivector16 {
        let mut result = [0.0f64; 16];
        let table = get_product_table();

        for a in 0..16 {
            let ca = self.coeffs[a];
            if ca.abs() < 1e-15 { continue; }
            for b in 0..16 {
                let cb = other.coeffs[b];
                if cb.abs() < 1e-15 { continue; }
                let (res_idx, sign) = table[a][b];
                result[res_idx] += ca * cb * sign;
            }
        }
        Multivector16 { coeffs: result }
    }

    /// Scalar product: the grade-0 component of the geometric product.
    pub fn scalar_product(&self, other: &Multivector16) -> f64 {
        let mut sum = 0.0;
        for i in 0..16 {
            sum += self.coeffs[i] * other.coeffs[i];
        }
        sum
    }

    /// Euclidean norm of the multivector.
    pub fn norm(&self) -> f64 {
        let sum_sq: f64 = self.coeffs.iter().map(|c| c * c).sum();
        sum_sq.sqrt()
    }

    /// Unit-norm normalize
    pub fn normalize(&self) -> Option<Multivector16> {
        let n = self.norm();
        if n < f64::EPSILON {
            return None;
        }
        let mut coeffs = self.coeffs;
        for c in &mut coeffs { *c /= n; }
        Some(Multivector16 { coeffs })
    }

    /// Find the dominant trigram: maps the blade with the largest
    /// absolute coefficient to its corresponding Bagua trigram.
    pub fn dominant_trigram(&self) -> crate::bagua::Trigram {
        let mut best_idx = 0usize;
        let mut best_val = 0.0f64;
        for i in 0..16 {
            let abs = self.coeffs[i].abs();
            if abs > best_val {
                best_val = abs;
                best_idx = i;
            }
        }
        cl4_blade_to_trigram(best_idx)
    }

    /// Dominant role: the RelationType of the dominant trigram.
    pub fn dominant_role(&self) -> crate::relation_type::RelationType {
        crate::relation_type::RelationType::from_trigram(self.dominant_trigram())
    }

    /// Encoding sharpness: ratio of max(|coeff|) to sqrt(sum_sq).
    /// 0.0 = perfectly diffuse, 1.0 = single-blade encoding.
    pub fn encoding_sharpness(&self) -> f64 {
        let n = self.norm();
        if n < f64::EPSILON { return 0.0; }
        let max_abs = self.coeffs.iter().map(|c| c.abs()).fold(0.0f64, f64::max);
        (max_abs / n).clamp(0.0, 1.0)
    }

    /// Expand a Cl(3) 8-coefficient encoding to Cl(4) 16-coefficient.
    /// Primary blades (indices 0-7 in Cl(4)) map 1:1 to Cl(3) indices.
    /// Secondary blades (indices 8-15) are zero-initialized.
    /// Then normalize to unit norm.
    pub fn from_cl3(cl3_coeffs: &[f64; 8]) -> Self {
        let mut coeffs = [0.0f64; 16];
        coeffs[..8].copy_from_slice(cl3_coeffs);
        Multivector16 { coeffs }.normalize().unwrap_or_else(|| {
            let mut zero = Multivector16::zero();
            zero.coeffs[0] = 1.0;
            zero
        })
    }
}

/// Map Cl(4) blade index (0-15) to a Bagua trigram.
/// Each trigram gets 2 blade indices — its primary (same as Cl(3)) and
/// a secondary index in the upper half of the blade space.
pub fn cl4_blade_to_trigram(index: usize) -> crate::bagua::Trigram {
    match index {
        0 | 8 => crate::bagua::Trigram::Kun,    // scalar + e4
        1 | 9 => crate::bagua::Trigram::Zhen,   // e1 + e14
        2 | 10 => crate::bagua::Trigram::Kan,   // e2 + e24
        3 | 11 => crate::bagua::Trigram::Gen,   // e3 + e34
        4 | 12 => crate::bagua::Trigram::Li,    // e12 + e124
        5 | 13 => crate::bagua::Trigram::Xun,   // e13 + e134
        6 | 14 => crate::bagua::Trigram::Dui,   // e23 + e234
        7 | 15 => crate::bagua::Trigram::Qian,  // e123 + e1234
        _ => crate::bagua::Trigram::Kun,
    }
}

/// Map Cl(3) Blade to Cl(4) primary blade index (same as Cl(3) index).
pub fn cl3_to_cl4_index(blade: Blade) -> usize {
    blade.index()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cl4_identity_times_blade() {
        let id = Multivector16::from_blade(0, 1.0); // scalar=1
        let blade = Multivector16::from_blade(1, 3.0); // e1=3
        let result = id.geometric_product(&blade);
        assert!((result.coeffs[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn cl4_e1_times_e1_is_scalar() {
        let e1 = Multivector16::from_blade(1, 1.0);
        let result = e1.geometric_product(&e1);
        assert!((result.coeffs[0] - 1.0).abs() < 1e-10, "e1*e1 should be 1");
        // All other coefficients should be 0
        for i in 1..16 {
            assert!(result.coeffs[i].abs() < 1e-10, "coefficient {i} should be 0");
        }
    }

    #[test]
    fn cl4_e1_times_e2_is_e12() {
        let e1 = Multivector16::from_blade(1, 1.0);
        let e2 = Multivector16::from_blade(2, 1.0);
        let result = e1.geometric_product(&e2);
        // e1^e2 = e12, index 3 in our bit-ordering
        assert!((result.coeffs[3] - 1.0).abs() < 1e-10, "e1*e2 should be e12");
    }

    #[test]
    fn cl4_e2_times_e1_is_neg_e12() {
        let e2 = Multivector16::from_blade(2, 1.0);
        let e1 = Multivector16::from_blade(1, 1.0);
        let result = e2.geometric_product(&e1);
        assert!((result.coeffs[3] + 1.0).abs() < 1e-10, "e2*e1 should be -e12");
    }

    #[test]
    fn cl4_norm_is_sqrt_of_sum_sq() {
        let mv = Multivector16::new([0.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                      0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((mv.norm() - 5.0).abs() < 1e-10, "||3e1+4e2|| should be 5");
    }

    #[test]
    fn cl4_sharpness_on_single_blade() {
        let mv = Multivector16::from_blade(3, 1.0);
        assert!((mv.encoding_sharpness() - 1.0).abs() < 1e-10,
            "single-blade encoding should have sharpness=1.0");
    }

    #[test]
    fn cl4_sharpness_on_uniform() {
        let mv = Multivector16::new([1.0; 16]);
        let s = mv.encoding_sharpness();
        assert!(s < 0.5, "uniform encoding should be diffuse, got {s}");
    }

    #[test]
    fn cl4_from_cl3_preserves_dominant_trigram() {
        // Cl(3) encoding with dominant at index 3 (Gen/Earth/constraining)
        let cl3: [f64; 8] = [0.05, 0.10, 0.10, 0.85, 0.15, 0.25, 0.15, 0.10];
        let mv16 = Multivector16::from_cl3(&cl3);
        let trigram = mv16.dominant_trigram();
        assert_eq!(trigram, crate::bagua::Trigram::Gen,
            "Cl(4) expansion should preserve dominant trigram, got {:?}", trigram);
    }

    #[test]
    fn cl4_geometric_product_with_pseudoscalar() {
        let e1234 = Multivector16::from_blade(15, 1.0); // pseudoscalar
        let e1 = Multivector16::from_blade(1, 1.0);
        let result = e1.geometric_product(&e1234);
        // e1 * e1234 = e234 (index 14)
        assert!(result.coeffs[14].abs() > 0.9, "e1*e1234 should be ±e234");
    }

    #[test]
    fn cl4_blade_reflection_grade_test() {
        // Verify that all 16 blades map to valid trigrams
        for i in 0..16 {
            let _ = cl4_blade_to_trigram(i);
            // Just ensure no panic
        }

        // Verify the 8 trigram types all appear among the 16 blades
        use std::collections::HashSet;
        let trigrams: HashSet<_> = (0..16).map(|i| cl4_blade_to_trigram(i)).collect();
        assert_eq!(trigrams.len(), 8, "should map to all 8 trigrams exactly once each");
    }

    #[test]
    fn cl4_product_table_is_consistent() {
        // Test anti-commutativity: ei*ej = -ej*ei for i≠j
        for i in 1..4 {
            for j in (i+1)..=4 {
                let blade_i = 1 << (i - 1);  // e_i is bit (i-1)
                let blade_j = 1 << (j - 1);  // e_j is bit (j-1)
                let a = Multivector16::from_blade(blade_i, 1.0);
                let b = Multivector16::from_blade(blade_j, 1.0);
                let ab = a.geometric_product(&b);
                let ba = b.geometric_product(&a);

                // ab + ba should be 0 (anti-commute)
                let sum: f64 = ab.coeffs.iter().zip(ba.coeffs.iter())
                    .map(|(x, y)| (x + y).abs()).sum();
                assert!(sum < 1e-10,
                    "e{i}*e{j} should anti-commute with e{j}*e{i}, sum={sum}");
            }
        }
    }
}
