#![cfg(feature = "bytemuck")]

use bytemuck::{CheckedBitPattern, Zeroable};
use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_is_zeroable() {
    let value = UnitInterval::<f32>::zeroed();

    assert_eq!(value, UnitInterval::ZERO);
}

#[test]
fn signed_unit_interval_is_zeroable() {
    let value = SignedUnitInterval::<f32>::zeroed();

    assert_eq!(value, SignedUnitInterval::ZERO);
}

#[test]
fn unit_interval_checked_bit_pattern_accepts_only_valid_values() {
    assert!(UnitInterval::<f32>::is_valid_bit_pattern(&0.25));
    assert!(UnitInterval::<f64>::is_valid_bit_pattern(&1.0));
    assert!(!UnitInterval::<f32>::is_valid_bit_pattern(&-0.25));
    assert!(!UnitInterval::<f32>::is_valid_bit_pattern(&1.25));
    assert!(!UnitInterval::<f32>::is_valid_bit_pattern(&f32::NAN));
}

#[test]
fn signed_unit_interval_checked_bit_pattern_accepts_only_valid_values() {
    assert!(SignedUnitInterval::<f32>::is_valid_bit_pattern(&-0.25));
    assert!(SignedUnitInterval::<f64>::is_valid_bit_pattern(&1.0));
    assert!(!SignedUnitInterval::<f32>::is_valid_bit_pattern(&-1.25));
    assert!(!SignedUnitInterval::<f32>::is_valid_bit_pattern(&1.25));
    assert!(!SignedUnitInterval::<f32>::is_valid_bit_pattern(&f32::NAN));
}

#[test]
fn checked_casts_reject_invalid_bytes() {
    let valid = bytemuck::bytes_of(&0.25f32);
    let invalid = bytemuck::bytes_of(&1.25f32);

    assert_eq!(
        bytemuck::checked::try_from_bytes::<UnitInterval<f32>>(valid).copied(),
        Ok(UnitInterval::new(0.25).unwrap())
    );
    assert!(bytemuck::checked::try_from_bytes::<UnitInterval<f32>>(invalid).is_err());
}
