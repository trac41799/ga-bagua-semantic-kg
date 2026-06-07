use ga_semantics_core::prelude::*;

#[test]
fn algebraic_identities() {
    let one = Multivector::one();
    let e1 = Multivector::from_blade(Blade::E1, 1.0);
    let e2 = Multivector::from_blade(Blade::E2, 1.0);
    let e3 = Multivector::from_blade(Blade::E3, 1.0);

    // e1 * e1 = 1
    assert!((e1.geo_product(&e1).scalar() - 1.0).abs() < 1e-10);

    // e1 * e2 = e12
    let e12 = e1.geo_product(&e2);
    assert!((e12.coefficient(Blade::E12.index()) - 1.0).abs() < 1e-10);

    // e1 * e2 * e3 = e123
    let e123 = e1.geo_product(&e2).geo_product(&e3);
    assert!((e123.coefficient(Blade::E123.index()) - 1.0).abs() < 1e-10);

    // e123^2 = -1
    let ps = Multivector::from_blade(Blade::E123, 1.0);
    let ps_sq = ps.geo_product(&ps);
    assert!((ps_sq.scalar() - (-1.0)).abs() < 1e-10);

    // 1 * anything = anything
    assert!(one.geo_product(&e1).approx_eq(&e1, 1e-10));
    assert!(e1.geo_product(&one).approx_eq(&e1, 1e-10));
}

#[test]
fn inverse_property() {
    // Inverse of pure vector
    let v = Multivector::from_blade(Blade::E1, 3.0);
    let inv_v = v.inverse().unwrap();
    let product_v = v.geo_product(&inv_v);
    assert!((product_v.scalar() - 1.0).abs() < 1e-10);
    for i in 1..8 {
        assert!(product_v.coefficient(i).abs() < 1e-10);
    }
    // Inverse of mixed-grade multivector (scalar + vector)
    let mv = Multivector::new([2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let inv = mv.inverse().unwrap();
    let product = mv.geo_product(&inv);
    assert!((product.scalar() - 1.0).abs() < 1e-10);
    for i in 1..8 {
        assert!(product.coefficient(i).abs() < 1e-10);
    }
}

#[test]
fn rotor_rotate_vector_in_plane() {
    let e1 = Multivector::from_blade(Blade::E1, 1.0);
    let _e2 = Multivector::from_blade(Blade::E2, 1.0);

    // Rotate by pi/2 in e12 plane: e1 -> e2
    let r = Rotor::new(std::f64::consts::PI / 2.0, Blade::E12).unwrap();
    let rotated = r.apply(&e1);

    assert!((rotated.coefficient(Blade::E2.index()) - 1.0).abs() < 1e-10);
    assert!(rotated.coefficient(Blade::E1.index()).abs() < 1e-10);
}

#[test]
fn multivector_add_sub_roundtrip() {
    let a = Multivector::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let b = Multivector::new([8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
    let c = (a + b) - b;
    assert!(c.approx_eq(&a, 1e-10));
}

#[test]
fn grade_projection_preserves_grade() {
    let mv = Multivector::new([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    for grade in 0..=3 {
        let proj = mv.grade_projection(grade);
        // Check that the projected multivector has only the expected grade components
        for g in 0..=3 {
            for blade in Blade::GRADE_BLADES[g] {
                let val = proj.coefficient(blade.index());
                if g == grade {
                    assert!((val - 1.0).abs() < 1e-10);
                } else {
                    assert_eq!(val, 0.0);
                }
            }
        }
    }
}

#[test]
fn dualize_pseudoscalar_property() {
    // Dualizing twice should multiply by e123^2 = -1
    let mv = Multivector::new([2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0, 19.0]);
    let double_dual = mv.dualize().dualize();
    let expected = mv * (-1.0);
    assert!(double_dual.approx_eq(&expected, 1e-10));
}

#[test]
fn rotor_compose_is_associative() {
    let r1 = Rotor::new(0.3, Blade::E12).unwrap();
    let r2 = Rotor::new(0.4, Blade::E23).unwrap();
    let r3 = Rotor::new(0.5, Blade::E31).unwrap();

    let left = r1.compose(&r2).compose(&r3);
    let right = r1.compose(&r2.compose(&r3));

    assert!(left.multivector().approx_eq(right.multivector(), 1e-10));
}

#[cfg(feature = "serde")]
#[test]
fn serde_roundtrip_multivector() {
    let mv = Multivector::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let json = serde_json::to_string(&mv).unwrap();
    let restored: Multivector = serde_json::from_str(&json).unwrap();
    assert!(mv.approx_eq(&restored, 1e-10));
}

#[cfg(feature = "serde")]
#[test]
fn serde_roundtrip_blade() {
    for blade in &Blade::ALL {
        let json = serde_json::to_string(blade).unwrap();
        let restored: Blade = serde_json::from_str(&json).unwrap();
        assert_eq!(*blade, restored);
    }
}

#[cfg(feature = "serde")]
#[test]
fn serde_roundtrip_blade_json_format() {
    let json = serde_json::to_string(&Blade::E12).unwrap();
    assert_eq!(json, "4");
}
