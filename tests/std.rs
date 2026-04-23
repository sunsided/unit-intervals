use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_std_float_methods_return_unit_interval_when_result_is_constrained() {
    let half = UnitInterval::<f64>::HALF;

    let floor: UnitInterval<f64> = half.floor();
    let ceil: UnitInterval<f64> = half.ceil();
    let round: UnitInterval<f64> = half.round();
    let trunc: UnitInterval<f64> = half.trunc();
    let fract: UnitInterval<f64> = half.fract();
    let sqrt: UnitInterval<f64> = half.sqrt();
    let cbrt: UnitInterval<f64> = half.cbrt();
    let atan: UnitInterval<f64> = half.atan();
    let tanh: UnitInterval<f64> = half.tanh();
    let asinh: UnitInterval<f64> = half.asinh();

    assert_eq!(floor.get(), 0.5_f64.floor());
    assert_eq!(ceil.get(), 0.5_f64.ceil());
    assert_eq!(round.get(), 0.5_f64.round());
    assert_eq!(trunc.get(), 0.5_f64.trunc());
    assert_eq!(fract.get(), 0.5_f64.fract());
    assert_eq!(sqrt.get(), 0.5_f64.sqrt());
    assert_eq!(cbrt.get(), 0.5_f64.cbrt());
    assert_eq!(atan.get(), 0.5_f64.atan());
    assert_eq!(tanh.get(), 0.5_f64.tanh());
    assert_eq!(asinh.get(), 0.5_f64.asinh());
}

#[test]
fn unit_interval_std_float_methods_return_backing_float_when_unconstrained() {
    let half = UnitInterval::<f64>::HALF;

    let pow: f64 = half.powi(2);
    let sine: f64 = half.sin();
    let hypot: f64 = half.hypot(0.5);
    let (sin, cos): (f64, f64) = half.sin_cos();

    assert_eq!(pow, 0.25);
    assert_eq!(sine, 0.5_f64.sin());
    assert_eq!(hypot, 0.5_f64.hypot(0.5));
    assert_eq!((sin, cos), 0.5_f64.sin_cos());
}

#[test]
fn signed_unit_interval_std_float_methods_return_constrained_types_when_possible() {
    let negative = SignedUnitInterval::<f32>::new(-0.5).unwrap();

    let cosine: UnitInterval = negative.cos();
    let (sine, sin_cos_cosine): (SignedUnitInterval, UnitInterval) = negative.sin_cos();

    assert_eq!(cosine.get(), (-0.5_f32).cos());
    assert_eq!(sine.get(), (-0.5_f32).sin());
    assert_eq!(sin_cos_cosine.get(), (-0.5_f32).cos());
}
