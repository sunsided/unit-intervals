#![cfg(feature = "rand_distr")]

use rand::{RngExt, SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, StandardUniform};
use unit_interval::{SignedUnitInterval, UnitInterval};

#[test]
fn standard_uniform_samples_unit_interval_values() {
    let mut rng = StdRng::seed_from_u64(42);

    for _ in 0..1024 {
        let value: UnitInterval<f32> = StandardUniform.sample(&mut rng);

        assert!(UnitInterval::<f32>::contains(value.get()));
    }
}

#[test]
fn standard_uniform_samples_signed_unit_interval_values() {
    let mut rng = StdRng::seed_from_u64(42);

    for _ in 0..1024 {
        let value: SignedUnitInterval<f64> = StandardUniform.sample(&mut rng);

        assert!(SignedUnitInterval::<f64>::contains(value.get()));
    }
}

#[test]
fn rng_random_uses_standard_uniform_impls() {
    let mut rng = StdRng::seed_from_u64(42);

    let unit: UnitInterval<f64> = rng.random();
    let signed: SignedUnitInterval<f32> = rng.random();

    assert!(UnitInterval::<f64>::contains(unit.get()));
    assert!(SignedUnitInterval::<f32>::contains(signed.get()));
}
