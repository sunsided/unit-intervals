#![cfg(feature = "num-traits")]

use num_traits::{AsPrimitive, Bounded, FromPrimitive, NumCast, ToBytes, ToPrimitive};
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
