use std::string::ToString;

use unit_interval::UnitInterval;

#[test]
fn f32_constructor_accepts_unit_interval() {
    let default_width: UnitInterval = UnitInterval::new(0.5).unwrap();

    assert_eq!(default_width.get(), 0.5);
    assert_eq!(UnitInterval::<f32>::new(0.0).map(|u| u.get()), Some(0.0));
    assert_eq!(UnitInterval::<f32>::new(0.5).map(|u| u.get()), Some(0.5));
    assert_eq!(UnitInterval::<f32>::new(1.0).map(|u| u.get()), Some(1.0));
    assert_eq!(UnitInterval::<f32>::new(-0.1), None);
    assert_eq!(UnitInterval::<f32>::new(1.1), None);
}

#[test]
fn f64_constructor_accepts_unit_interval() {
    assert_eq!(UnitInterval::<f64>::new(0.0).map(|u| u.get()), Some(0.0));
    assert_eq!(UnitInterval::<f64>::new(0.5).map(|u| u.get()), Some(0.5));
    assert_eq!(UnitInterval::<f64>::new(1.0).map(|u| u.get()), Some(1.0));
    assert_eq!(UnitInterval::<f64>::new(-0.1), None);
    assert_eq!(UnitInterval::<f64>::new(1.1), None);
}

#[test]
fn constructors_reject_nan() {
    assert_eq!(UnitInterval::<f32>::new(f32::NAN), None);
    assert_eq!(UnitInterval::<f64>::new(f64::NAN), None);
}

#[test]
fn saturating_clamps_for_both_float_widths() {
    assert_eq!(UnitInterval::<f32>::saturating(-0.1).get(), 0.0);
    assert_eq!(UnitInterval::<f32>::saturating(1.1).get(), 1.0);
    assert_eq!(UnitInterval::<f32>::saturating(f32::NAN).get(), 0.0);
    assert_eq!(UnitInterval::<f64>::saturating(-0.1).get(), 0.0);
    assert_eq!(UnitInterval::<f64>::saturating(1.1).get(), 1.0);
    assert_eq!(UnitInterval::<f64>::saturating(f64::NAN).get(), 0.0);
}

#[test]
fn constants_and_basic_helpers_work() {
    assert_eq!(UnitInterval::<f32>::default(), UnitInterval::<f32>::ZERO);
    assert_eq!(UnitInterval::<f32>::ZERO.get(), 0.0);
    assert_eq!(UnitInterval::<f32>::HALF.get(), 0.5);
    assert_eq!(UnitInterval::<f32>::ONE.get(), 1.0);
    assert!(UnitInterval::<f32>::contains(0.5));
    assert!(!UnitInterval::<f32>::contains(1.5));
    assert!(UnitInterval::<f32>::ZERO.is_zero());
    assert!(UnitInterval::<f32>::ONE.is_one());
    assert_eq!(UnitInterval::new(0.25).unwrap().complement().get(), 0.75);
}

#[test]
fn standard_conversions_are_available() {
    let from_result = UnitInterval::<f32>::try_from(0.5).unwrap();
    let as_f64_interval = UnitInterval::<f64>::from(from_result);
    let as_f32_interval = UnitInterval::<f32>::from(as_f64_interval);

    assert_eq!(from_result.get(), 0.5);
    assert_eq!(
        UnitInterval::<f32>::try_from(1.5).unwrap_err().to_string(),
        "value is outside the unit interval"
    );
    assert_eq!(as_f64_interval.get(), 0.5);
    assert_eq!(as_f32_interval.get(), 0.5);
}

#[test]
fn comparison_helpers_preserve_unit_interval() {
    let low = UnitInterval::new(0.25).unwrap();
    let high = UnitInterval::new(0.75).unwrap();

    assert_eq!(low.min(high), low);
    assert_eq!(low.max(high), high);
    assert_eq!(low.midpoint(high).get(), 0.5);
    assert_eq!(low.distance_to(high).get(), 0.5);
}

#[test]
fn checked_arithmetic_rejects_out_of_range_results() {
    let low = UnitInterval::new(0.25).unwrap();
    let high = UnitInterval::new(0.75).unwrap();

    assert_eq!(low.checked_add(low).map(|u| u.get()), Some(0.5));
    assert_eq!(high.checked_add(high), None);
    assert_eq!(high.checked_sub(low).map(|u| u.get()), Some(0.5));
    assert_eq!(low.checked_sub(high), None);
    assert_eq!(low.checked_div(high).map(|u| u.get()), Some(1.0 / 3.0));
    assert_eq!(high.checked_div(low), None);
    assert_eq!(low.checked_scale(2.0).map(|u| u.get()), Some(0.5));
    assert_eq!(high.checked_scale(2.0), None);
}

#[test]
fn saturating_arithmetic_clamps_out_of_range_results() {
    let low = UnitInterval::new(0.25).unwrap();
    let high = UnitInterval::new(0.75).unwrap();

    assert_eq!(high.saturating_add(high).get(), 1.0);
    assert_eq!(low.saturating_sub(high).get(), 0.0);
    assert_eq!(high.saturating_div(low).get(), 1.0);
    assert_eq!(low.saturating_scale(-1.0).get(), 0.0);
    assert_eq!(high.saturating_scale(2.0).get(), 1.0);
}

#[test]
fn multiplication_and_lerp_are_convenient() {
    let low = UnitInterval::new(0.25).unwrap();
    let high = UnitInterval::new(0.75).unwrap();

    assert_eq!((low * high).get(), 0.1875);
    assert_eq!(UnitInterval::<f32>::HALF.lerp(10.0, 20.0), 15.0);
}

#[test]
fn unconstrained_arithmetic_returns_backing_float() {
    let low = UnitInterval::<f32>::new(0.25).unwrap();
    let high = UnitInterval::<f32>::new(0.75).unwrap();

    let sum: f32 = low + high;
    let difference: f32 = low - high;
    let product: UnitInterval<f32> = low * high;
    let scaled: f32 = low * 4.0;
    let reverse_scaled: f32 = 4.0 * low;
    let quotient: f32 = high / low;
    let remainder: f32 = high % low;
    let negated: f32 = -low;

    assert_eq!(sum, 1.0);
    assert_eq!(difference, -0.5);
    assert_eq!(product.get(), 0.1875);
    assert_eq!(scaled, 1.0);
    assert_eq!(reverse_scaled, 1.0);
    assert_eq!(quotient, 3.0);
    assert_eq!(remainder, 0.0);
    assert_eq!(negated, -0.25);
}

#[test]
fn comparisons_work_against_backing_float() {
    let half = UnitInterval::<f32>::HALF;

    assert_eq!(half, 0.5);
    assert_eq!(0.5, half);
    assert!(half < 0.75);
    assert!(0.25 < half);
    assert!(half >= 0.5);
    assert!(0.5 <= half);
}
