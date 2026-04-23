#![cfg(feature = "arbitrary")]

use arbitrary::{Arbitrary, Unstructured};
use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_arbitrary_generates_values_inside_interval() {
    for seed in seeds() {
        let mut data = Unstructured::new(&seed);
        let value = UnitInterval::<f32>::arbitrary(&mut data).unwrap();

        assert!(UnitInterval::<f32>::contains(value.get()));
    }
}

#[test]
fn unit_interval_arbitrary_uses_full_unsigned_range() {
    let low_bytes = u32::MIN.to_le_bytes();
    let high_bytes = u32::MAX.to_le_bytes();
    let mut low = Unstructured::new(&low_bytes);
    let mut high = Unstructured::new(&high_bytes);

    assert_eq!(
        UnitInterval::<f32>::arbitrary(&mut low).unwrap(),
        UnitInterval::ZERO
    );
    assert_eq!(
        UnitInterval::<f32>::arbitrary(&mut high).unwrap(),
        UnitInterval::ONE
    );
}

#[test]
fn signed_unit_interval_arbitrary_generates_values_inside_interval() {
    for seed in seeds() {
        let mut data = Unstructured::new(&seed);
        let value = SignedUnitInterval::<f64>::arbitrary(&mut data).unwrap();

        assert!(SignedUnitInterval::<f64>::contains(value.get()));
    }
}

#[test]
fn signed_unit_interval_arbitrary_uses_full_unsigned_range() {
    let low_bytes = u64::MIN.to_le_bytes();
    let high_bytes = u64::MAX.to_le_bytes();
    let mut low = Unstructured::new(&low_bytes);
    let mut high = Unstructured::new(&high_bytes);

    assert_eq!(
        SignedUnitInterval::<f64>::arbitrary(&mut low).unwrap(),
        SignedUnitInterval::NEG_ONE
    );
    assert_eq!(
        SignedUnitInterval::<f64>::arbitrary(&mut high).unwrap(),
        SignedUnitInterval::ONE
    );
}

#[test]
fn arbitrary_size_hints_match_backing_unsigned_widths() {
    assert_eq!(UnitInterval::<f32>::size_hint(0), u32::size_hint(0));
    assert_eq!(UnitInterval::<f64>::size_hint(0), u64::size_hint(0));
    assert_eq!(SignedUnitInterval::<f32>::size_hint(0), u32::size_hint(0));
    assert_eq!(SignedUnitInterval::<f64>::size_hint(0), u64::size_hint(0));
}

fn seeds() -> [[u8; 8]; 5] {
    [[0; 8], [u8::MAX; 8], [0x80; 8], [0x55; 8], [0xAA; 8]]
}
