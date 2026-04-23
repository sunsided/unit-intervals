#![cfg(feature = "num-traits")]

use num_traits::{
    AsPrimitive, Bounded, ConstOne, FromPrimitive, NumCast, One, Pow, ToBytes, ToPrimitive,
    ops::{
        checked::{CheckedMul, CheckedNeg},
        saturating::SaturatingMul,
    },
};
use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_converts_to_primitives() {
    let value = UnitInterval::new(0.75_f32).unwrap();

    assert_eq!(value.to_f32(), Some(0.75));
    assert_eq!(value.to_f64(), Some(0.75));
    assert_eq!(value.to_i32(), Some(0));
    assert_eq!(value.to_u32(), Some(0));
}

#[test]
fn signed_unit_interval_converts_to_primitives() {
    let value = SignedUnitInterval::new(-0.75_f64).unwrap();

    assert_eq!(value.to_f32(), Some(-0.75));
    assert_eq!(value.to_f64(), Some(-0.75));
    assert_eq!(value.to_i32(), Some(0));
    assert_eq!(value.to_u32(), Some(0));
}

#[test]
fn primitive_conversions_validate_unit_interval() {
    assert_eq!(
        UnitInterval::<f32>::from_f32(0.25),
        Some(UnitInterval::new(0.25).unwrap())
    );
    assert_eq!(UnitInterval::<f32>::from_f32(1.25), None);
    assert_eq!(UnitInterval::<f32>::from_f32(f32::NAN), None);

    assert_eq!(
        <UnitInterval<f64> as NumCast>::from(1_u8),
        Some(UnitInterval::ONE)
    );
    assert_eq!(<UnitInterval<f64> as NumCast>::from(2_u8), None);
}

#[test]
fn primitive_conversions_validate_signed_unit_interval() {
    assert_eq!(
        SignedUnitInterval::<f32>::from_f32(-0.25),
        Some(SignedUnitInterval::new(-0.25).unwrap())
    );
    assert_eq!(SignedUnitInterval::<f32>::from_f32(-1.25), None);
    assert_eq!(SignedUnitInterval::<f32>::from_f32(f32::NAN), None);

    assert_eq!(
        <SignedUnitInterval<f64> as NumCast>::from(-1_i8),
        Some(SignedUnitInterval::NEG_ONE)
    );
    assert_eq!(<SignedUnitInterval<f64> as NumCast>::from(2_i8), None);
}

#[test]
fn bounds_match_constrained_intervals() {
    assert_eq!(UnitInterval::<f32>::min_value(), UnitInterval::ZERO);
    assert_eq!(UnitInterval::<f32>::max_value(), UnitInterval::ONE);

    assert_eq!(
        SignedUnitInterval::<f64>::min_value(),
        SignedUnitInterval::NEG_ONE
    );
    assert_eq!(
        SignedUnitInterval::<f64>::max_value(),
        SignedUnitInterval::ONE
    );
}

#[test]
fn to_bytes_delegates_to_backing_float() {
    let unit = UnitInterval::new(0.25_f32).unwrap();
    let signed = SignedUnitInterval::new(-0.25_f64).unwrap();

    assert_eq!(unit.to_be_bytes(), 0.25_f32.to_be_bytes());
    assert_eq!(unit.to_le_bytes(), 0.25_f32.to_le_bytes());
    assert_eq!(signed.to_be_bytes(), (-0.25_f64).to_be_bytes());
    assert_eq!(signed.to_le_bytes(), (-0.25_f64).to_le_bytes());
}

#[test]
fn as_primitive_converts_out_of_wrappers() {
    let unit = UnitInterval::new(0.25_f32).unwrap();
    let signed = SignedUnitInterval::new(-0.25_f64).unwrap();

    let unit_float: f64 = unit.as_();
    let unit_wide: UnitInterval<f64> = unit.as_();
    let signed_float: f32 = signed.as_();
    let signed_narrow: SignedUnitInterval<f32> = signed.as_();

    assert_eq!(unit_float, 0.25);
    assert_eq!(unit_wide, UnitInterval::new(0.25).unwrap());
    assert_eq!(signed_float, -0.25);
    assert_eq!(signed_narrow, SignedUnitInterval::new(-0.25).unwrap());
}

#[test]
fn one_traits_return_upper_bound() {
    assert_eq!(UnitInterval::<f32>::one(), UnitInterval::ONE);
    assert_eq!(
        <UnitInterval<f32> as ConstOne>::ONE,
        UnitInterval::<f32>::ONE
    );
    assert!(UnitInterval::<f32>::ONE.is_one());

    assert_eq!(SignedUnitInterval::<f64>::one(), SignedUnitInterval::ONE);
    assert_eq!(
        <SignedUnitInterval<f64> as ConstOne>::ONE,
        SignedUnitInterval::<f64>::ONE
    );
    assert!(SignedUnitInterval::<f64>::ONE.is_one());
}

#[test]
fn checked_mul_is_closed_over_intervals() {
    let unit = UnitInterval::new(0.5_f32).unwrap();
    let signed = SignedUnitInterval::new(-0.5_f64).unwrap();

    assert_eq!(CheckedMul::checked_mul(&unit, &unit), Some(unit * unit));
    assert_eq!(
        CheckedMul::checked_mul(&signed, &signed),
        Some(signed * signed)
    );
}

#[test]
fn saturating_mul_delegates_to_closed_multiplication() {
    let low = UnitInterval::new(0.25_f32).unwrap();
    let high = UnitInterval::new(0.75_f32).unwrap();

    assert_eq!(SaturatingMul::saturating_mul(&low, &high), low * high);

    let negative = SignedUnitInterval::new(-0.75_f64).unwrap();
    let positive = SignedUnitInterval::new(0.75_f64).unwrap();

    assert_eq!(
        SaturatingMul::saturating_mul(&negative, &positive),
        negative * positive
    );
}

#[test]
fn signed_unit_interval_supports_checked_negation() {
    let negative = SignedUnitInterval::new(-0.25_f32).unwrap();

    assert_eq!(
        CheckedNeg::checked_neg(&negative),
        Some(SignedUnitInterval::new(0.25).unwrap())
    );
}

#[test]
fn integer_powers_stay_in_interval() {
    let unit = UnitInterval::new(0.5_f32).unwrap();
    let signed = SignedUnitInterval::new(-0.5_f64).unwrap();

    assert_eq!(unit.pow(0_u8), UnitInterval::ONE);
    assert_eq!(unit.pow(3_u16), UnitInterval::new(0.125).unwrap());
    assert_eq!((&unit).pow(&2_u32), UnitInterval::new(0.25).unwrap());

    assert_eq!(signed.pow(0_usize), SignedUnitInterval::ONE);
    assert_eq!(signed.pow(3_u8), SignedUnitInterval::new(-0.125).unwrap());
    assert_eq!(
        (&signed).pow(&2_u16),
        SignedUnitInterval::new(0.25).unwrap()
    );
}
