use crate::Multivector;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Blade {
    Scalar,
    E1,
    E2,
    E3,
    E12,
    E23,
    E31,
    E123,
}

impl Blade {
    pub const COUNT: usize = 8;

    pub fn index(self) -> usize {
        match self {
            Blade::Scalar => 0,
            Blade::E1 => 1,
            Blade::E2 => 2,
            Blade::E3 => 3,
            Blade::E12 => 4,
            Blade::E23 => 5,
            Blade::E31 => 6,
            Blade::E123 => 7,
        }
    }

    pub fn from_index(i: usize) -> Option<Self> {
        Some(match i {
            0 => Blade::Scalar,
            1 => Blade::E1,
            2 => Blade::E2,
            3 => Blade::E3,
            4 => Blade::E12,
            5 => Blade::E23,
            6 => Blade::E31,
            7 => Blade::E123,
            _ => return None,
        })
    }

    pub fn grade(self) -> usize {
        match self {
            Blade::Scalar => 0,
            Blade::E1 | Blade::E2 | Blade::E3 => 1,
            Blade::E12 | Blade::E23 | Blade::E31 => 2,
            Blade::E123 => 3,
        }
    }

    pub fn to_multivector(self, coeff: f64) -> Multivector {
        let mut v = [0.0; 8];
        v[self.index()] = coeff;
        Multivector::new(v)
    }

    pub const ALL: [Blade; 8] = [
        Blade::Scalar,
        Blade::E1,
        Blade::E2,
        Blade::E3,
        Blade::E12,
        Blade::E23,
        Blade::E31,
        Blade::E123,
    ];

    pub const GRADE_BLADES: [&[Blade]; 4] = [
        &[Blade::Scalar],
        &[Blade::E1, Blade::E2, Blade::E3],
        &[Blade::E12, Blade::E23, Blade::E31],
        &[Blade::E123],
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blade_count() {
        assert_eq!(Blade::COUNT, 8);
    }

    #[test]
    fn blade_index_roundtrip() {
        for i in 0..8 {
            let blade = Blade::from_index(i).unwrap();
            assert_eq!(blade.index(), i);
        }
    }

    #[test]
    fn blade_from_index_out_of_range() {
        assert_eq!(Blade::from_index(8), None);
        assert_eq!(Blade::from_index(255), None);
    }

    #[test]
    fn grade_of_blade() {
        assert_eq!(Blade::Scalar.grade(), 0);
        assert_eq!(Blade::E1.grade(), 1);
        assert_eq!(Blade::E2.grade(), 1);
        assert_eq!(Blade::E3.grade(), 1);
        assert_eq!(Blade::E12.grade(), 2);
        assert_eq!(Blade::E23.grade(), 2);
        assert_eq!(Blade::E31.grade(), 2);
        assert_eq!(Blade::E123.grade(), 3);
    }

    #[test]
    fn all_blades_exist() {
        assert_eq!(Blade::ALL.len(), 8);
        for blade in &Blade::ALL {
            assert!(Blade::from_index(blade.index()).is_some());
        }
    }

    #[test]
    fn grade_blades_counts() {
        assert_eq!(Blade::GRADE_BLADES[0].len(), 1);
        assert_eq!(Blade::GRADE_BLADES[1].len(), 3);
        assert_eq!(Blade::GRADE_BLADES[2].len(), 3);
        assert_eq!(Blade::GRADE_BLADES[3].len(), 1);
    }

    #[test]
    fn to_multivector_single_blade() {
        let mv = Blade::E1.to_multivector(0.5);
        assert_eq!(mv.coefficient(1), 0.5);
        for i in [0, 2, 3, 4, 5, 6, 7] {
            assert_eq!(mv.coefficient(i), 0.0);
        }
    }
}
