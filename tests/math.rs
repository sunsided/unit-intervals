use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_core_float_methods_are_available_without_std() {
    let half = UnitInterval::<f64>::HALF;

    let abs: UnitInterval<f64> = half.abs();
    let signum: f64 = half.signum();
    let copysign: f64 = half.copysign(-1.0);
    let recip: f64 = half.recip();

    assert_eq!(abs.get(), 0.5_f64.abs());
    assert_eq!(signum, 1.0);
    assert_eq!(copysign, -0.5);
    assert_eq!(recip, 2.0);
    assert!(half.is_sign_positive());
    assert!(!half.is_sign_negative());
    assert!(half.is_finite());
    assert!(!half.is_infinite());
    assert!(!half.is_nan());
}

#[test]
fn signed_unit_interval_core_float_methods_are_available_without_std() {
    let negative = SignedUnitInterval::<f32>::new(-0.5).unwrap();

    let absolute: UnitInterval = negative.abs();
    let signum: SignedUnitInterval = negative.signum();
    let copysign: SignedUnitInterval = negative.copysign(1.0);
    let recip: f32 = negative.recip();

    assert_eq!(absolute.get(), 0.5);
    assert_eq!(signum.get(), -1.0);
    assert_eq!(copysign.get(), 0.5);
    assert_eq!(recip, -2.0);
    assert!(!negative.is_sign_positive());
    assert!(negative.is_sign_negative());
    assert!(negative.is_finite());
    assert!(!negative.is_infinite());
    assert!(!negative.is_nan());
}
