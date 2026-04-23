use unit_intervals::{SignedUnitInterval, UnitInterval};

#[test]
fn constructors_accept_signed_unit_interval() {
    let default_width: SignedUnitInterval = SignedUnitInterval::new(-0.5).unwrap();

    assert_eq!(default_width.get(), -0.5);
    assert_eq!(
        SignedUnitInterval::<f32>::new(-1.0).map(|u| u.get()),
        Some(-1.0)
    );
    assert_eq!(
        SignedUnitInterval::<f32>::new(0.0).map(|u| u.get()),
        Some(0.0)
    );
    assert_eq!(
        SignedUnitInterval::<f32>::new(1.0).map(|u| u.get()),
        Some(1.0)
    );
    assert_eq!(SignedUnitInterval::<f32>::new(-1.1), None);
    assert_eq!(SignedUnitInterval::<f32>::new(1.1), None);
    assert_eq!(SignedUnitInterval::<f32>::new(f32::NAN), None);
}

#[test]
fn constants_conversions_and_helpers_work() {
    let unit = UnitInterval::new(0.25).unwrap();
    let signed = SignedUnitInterval::from(unit);
    let back_to_unit = UnitInterval::try_from(signed).unwrap();
    let negative = SignedUnitInterval::new(-0.25).unwrap();

    assert_eq!(
        SignedUnitInterval::<f32>::default(),
        SignedUnitInterval::ZERO
    );
    assert!(SignedUnitInterval::<f32>::NEG_ONE.is_neg_one());
    assert!(SignedUnitInterval::<f32>::ZERO.is_zero());
    assert!(SignedUnitInterval::<f32>::ONE.is_one());
    assert_eq!(SignedUnitInterval::<f32>::saturating(-1.25).get(), -1.0);
    assert_eq!(SignedUnitInterval::<f32>::saturating(1.25).get(), 1.0);
    assert_eq!(SignedUnitInterval::<f32>::saturating(f32::NAN).get(), 0.0);
    assert_eq!(signed.get(), 0.25);
    assert_eq!(back_to_unit, unit);
    assert!(UnitInterval::try_from(negative).is_err());
}

#[test]
fn checked_and_saturating_arithmetic_accept_unit_interval() {
    let negative = SignedUnitInterval::new(-0.75).unwrap();
    let positive = SignedUnitInterval::new(0.75).unwrap();
    let unit = UnitInterval::new(0.5).unwrap();

    assert_eq!(negative.min(unit), negative);
    assert_eq!(negative.max(unit).get(), 0.5);
    assert_eq!(negative.midpoint(unit).get(), -0.125);
    assert_eq!(negative.distance_to(unit), 1.25);
    assert_eq!(negative.checked_add(unit).unwrap().get(), -0.25);
    assert_eq!(positive.checked_add(unit), None);
    assert_eq!(positive.saturating_add(unit).get(), 1.0);
    assert_eq!(negative.checked_sub(unit), None);
    assert_eq!(negative.saturating_sub(unit).get(), -1.0);
    assert_eq!(positive.checked_div(unit), None);
    assert_eq!(positive.saturating_div(unit).get(), 1.0);
}

#[test]
fn constrained_results_return_constrained_types() {
    let negative = SignedUnitInterval::new(-0.5).unwrap();
    let positive = SignedUnitInterval::new(0.5).unwrap();
    let unit = UnitInterval::new(0.5).unwrap();

    let negated: SignedUnitInterval = -negative;
    let signed_product: SignedUnitInterval = negative * positive;
    let mixed_product: SignedUnitInterval = negative * unit;
    let reverse_mixed_product: SignedUnitInterval = unit * negative;

    assert_eq!(negated.get(), 0.5);
    assert_eq!(signed_product.get(), -0.25);
    assert_eq!(mixed_product.get(), -0.25);
    assert_eq!(reverse_mixed_product.get(), -0.25);
}

#[test]
fn unconstrained_operations_return_backing_float() {
    let negative = SignedUnitInterval::<f32>::new(-0.75).unwrap();
    let positive = SignedUnitInterval::<f32>::new(0.75).unwrap();
    let unit = UnitInterval::new(0.5).unwrap();

    let sum: f32 = positive + positive;
    let mixed_sum: f32 = positive + unit;
    let quotient: f32 = positive / negative;
    let distance: f32 = negative.distance_to(positive);

    assert_eq!(sum, 1.5);
    assert_eq!(mixed_sum, 1.25);
    assert_eq!(quotient, -1.0);
    assert_eq!(distance, 1.5);
}
