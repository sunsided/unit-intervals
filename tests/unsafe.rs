#![cfg(feature = "unsafe")]

use unit_intervals::{SignedUnitInterval, UnitInterval};

#[test]
fn unit_interval_exposes_unchecked_construction_and_arithmetic() {
    let low = UnitInterval::new(0.25).unwrap();
    let high = UnitInterval::new(0.75).unwrap();

    // SAFETY: Every operation result below stays inside [0, 1].
    unsafe {
        assert_eq!(UnitInterval::new_unchecked(0.5).get(), 0.5);
        assert_eq!(low.add_unchecked(low).get(), 0.5);
        assert_eq!(high.sub_unchecked(low).get(), 0.5);
        assert_eq!(low.div_unchecked(high).get(), 1.0 / 3.0);
        assert_eq!(low.scale_unchecked(2.0).get(), 0.5);
    }
}

#[test]
fn signed_unit_interval_exposes_unchecked_construction_and_arithmetic() {
    let negative = SignedUnitInterval::new(-0.75).unwrap();
    let positive = SignedUnitInterval::new(0.75).unwrap();
    let unit = UnitInterval::new(0.5).unwrap();

    // SAFETY: Every operation result below stays inside [-1, 1].
    unsafe {
        assert_eq!(SignedUnitInterval::new_unchecked(-0.5).get(), -0.5);
        assert_eq!(negative.add_unchecked(unit).get(), -0.25);
        assert_eq!(positive.sub_unchecked(unit).get(), 0.25);
        assert_eq!(positive.div_unchecked(positive).get(), 1.0);
        assert_eq!(positive.scale_unchecked(0.5).get(), 0.375);
    }
}
