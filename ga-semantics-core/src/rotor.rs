use crate::blade::Blade;
use crate::Multivector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotor {
    multivector: Multivector,
}

impl Rotor {
    pub fn new(theta: f64, plane: Blade) -> Option<Self> {
        if plane.grade() != 2 {
            return None;
        }
        let half = theta / 2.0;
        let cos = half.cos();
        let sin = half.sin();
        let mut coeffs = [0.0; 8];
        coeffs[0] = cos;
        coeffs[plane.index()] = -sin;
        Some(Rotor {
            multivector: Multivector::new(coeffs),
        })
    }

    pub fn multivector(&self) -> &Multivector {
        &self.multivector
    }

    /// Construct a Rotor from an even-grade multivector (scalar + bivector parts).
    pub fn from_multivector(mv: Multivector) -> Option<Self> {
        let grade1 = mv.grade_projection(1);
        let grade3 = mv.grade_projection(3);
        if grade1.norm() > 1e-10 || grade3.norm() > 1e-10 {
            return None;
        }
        let n = mv.norm();
        if n < f64::EPSILON {
            return None;
        }
        let normalized = mv * (1.0 / n);
        Some(Rotor { multivector: normalized })
    }

    /// For a unit rotor R, the inverse is the reverse R̃.
    pub fn inverse_rotor(&self) -> Self {
        Rotor {
            multivector: self.multivector.reverse(),
        }
    }

    pub fn apply(&self, mv: &Multivector) -> Multivector {
        let rev = self.multivector.reverse();
        let temp = self.multivector.geo_product(mv);
        temp.geo_product(&rev)
    }

    pub fn compose(&self, other: &Rotor) -> Rotor {
        let product = self.multivector.geo_product(other.multivector());
        Rotor {
            multivector: product,
        }
    }

    pub fn identity() -> Self {
        Rotor {
            multivector: Multivector::one(),
        }
    }

    pub fn is_unit(&self, epsilon: f64) -> bool {
        let norm_sq = self.multivector.norm_squared();
        (norm_sq - 1.0).abs() < epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blade::Blade;
    use std::f64::consts::PI;

    #[test]
    fn rotor_identity() {
        let r = Rotor::identity();
        assert!(r.is_unit(1e-10));
    }

    #[test]
    fn rotor_non_bivector_plane_returns_none() {
        assert!(Rotor::new(0.5, Blade::Scalar).is_none());
        assert!(Rotor::new(0.5, Blade::E1).is_none());
        assert!(Rotor::new(0.5, Blade::E123).is_none());
    }

    #[test]
    fn rotor_bivector_plane_succeeds() {
        assert!(Rotor::new(0.5, Blade::E12).is_some());
        assert!(Rotor::new(0.5, Blade::E23).is_some());
        assert!(Rotor::new(0.5, Blade::E31).is_some());
    }

    #[test]
    fn rotor_unit_norm() {
        let r = Rotor::new(PI / 4.0, Blade::E12).unwrap();
        assert!(r.is_unit(1e-10));
    }

    #[test]
    fn rotor_reverse_is_conjugate() {
        let r = Rotor::new(PI / 3.0, Blade::E12).unwrap();
        let mv = r.multivector();
        let product = mv.geo_product(&mv.reverse());
        assert!((product.scalar() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn rotor_apply_identity_preserves() {
        let r = Rotor::identity();
        let v = Multivector::from_blade(Blade::E1, 1.0);
        assert!(r.apply(&v).approx_eq(&v, 1e-10));
    }

    #[test]
    fn rotor_full_rotation_returns_negative() {
        let r = Rotor::new(PI, Blade::E12).unwrap();
        let v = Multivector::from_blade(Blade::E1, 1.0);
        let result = r.apply(&v);
        assert!((result.coefficient(Blade::E1.index()) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn rotor_compose_identity() {
        let r = Rotor::new(PI / 4.0, Blade::E12).unwrap();
        let id = Rotor::identity();
        let composed = r.compose(&id);
        assert!(composed.is_unit(1e-10));
        let v = Multivector::from_blade(Blade::E1, 1.0);
        assert!(r.apply(&v).approx_eq(&composed.apply(&v), 1e-10));
    }

    #[test]
    fn rotor_two_half_rotations() {
        let half = Rotor::new(PI / 4.0, Blade::E12).unwrap();
        let full = half.compose(&half);
        let v = Multivector::from_blade(Blade::E1, 1.0);
        let result = full.apply(&v);
        assert!((result.coefficient(Blade::E2.index()) - 1.0).abs() < 1e-10);
    }
}
