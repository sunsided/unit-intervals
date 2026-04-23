#![cfg(feature = "rkyv")]

use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_round_trips_through_archive() {
    let value = UnitInterval::<f32>::new(0.25).unwrap();

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
    let archived =
        rkyv::access::<rkyv::Archived<UnitInterval<f32>>, rkyv::rancor::Error>(&bytes).unwrap();
    let round_tripped =
        rkyv::deserialize::<UnitInterval<f32>, rkyv::rancor::Error>(archived).unwrap();

    assert_eq!(round_tripped, value);
}

#[test]
fn signed_unit_interval_round_trips_through_archive() {
    let value = SignedUnitInterval::<f32>::new(-0.25).unwrap();

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
    let archived =
        rkyv::access::<rkyv::Archived<SignedUnitInterval<f32>>, rkyv::rancor::Error>(&bytes)
            .unwrap();
    let round_tripped =
        rkyv::deserialize::<SignedUnitInterval<f32>, rkyv::rancor::Error>(archived).unwrap();

    assert_eq!(round_tripped, value);
}
