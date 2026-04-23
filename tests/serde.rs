#![cfg(feature = "serde")]

use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_serializes_as_inner_value_and_deserializes_through_checked_constructor() {
    let value = UnitInterval::<f32>::new(0.25).unwrap();

    assert_eq!(serde_json::to_string(&value).unwrap(), "0.25");
    assert_eq!(
        serde_json::from_str::<UnitInterval<f32>>("0.25").unwrap(),
        value
    );
    assert!(serde_json::from_str::<UnitInterval<f32>>("-0.25").is_err());
    assert!(serde_json::from_str::<UnitInterval<f32>>("1.25").is_err());
}

#[test]
fn signed_unit_interval_serializes_as_inner_value_and_deserializes_through_checked_constructor() {
    let value = SignedUnitInterval::<f32>::new(-0.25).unwrap();

    assert_eq!(serde_json::to_string(&value).unwrap(), "-0.25");
    assert_eq!(
        serde_json::from_str::<SignedUnitInterval<f32>>("-0.25").unwrap(),
        value
    );
    assert!(serde_json::from_str::<SignedUnitInterval<f32>>("-1.25").is_err());
    assert!(serde_json::from_str::<SignedUnitInterval<f32>>("1.25").is_err());
}
