#![cfg(feature = "rand_distr")]

use rand::{RngExt, SeedableRng, rngs::StdRng};
use rand_distr::{Beta, Distribution, Open01, OpenClosed01, StandardUniform};
use unit_intervals::{
    SignedUnitInterval, UnitInterval,
    random::{
        CheckedSignedUnitIntervalDistribution, CheckedUnitIntervalDistribution,
        SaturatingSignedUnitIntervalDistribution, SaturatingUnitIntervalDistribution,
    },
};

struct Fixed<T>(T);

impl<T: Copy> Distribution<T> for Fixed<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, _rng: &mut R) -> T {
        self.0
    }
}

#[test]
fn standard_uniform_samples_unit_interval_values() {
    let mut rng = StdRng::seed_from_u64(42);

    for _ in 0..1024 {
        let value: UnitInterval<f32> = StandardUniform.sample(&mut rng);

        assert!(UnitInterval::<f32>::contains(value.get()));
    }
}

#[test]
fn bounded_float_distributions_sample_unit_interval_values() {
    let mut rng = StdRng::seed_from_u64(42);
    let beta = Beta::new(2.0_f64, 5.0).unwrap();

    for _ in 0..1024 {
        let open: UnitInterval<f32> = Open01.sample(&mut rng);
        let open_closed: UnitInterval<f64> = OpenClosed01.sample(&mut rng);
        let beta_value: UnitInterval<f64> = beta.sample(&mut rng);

        assert!(UnitInterval::<f32>::contains(open.get()));
        assert!(UnitInterval::<f64>::contains(open_closed.get()));
        assert!(UnitInterval::<f64>::contains(beta_value.get()));
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

#[test]
fn checked_adapters_defer_to_inner_distribution_and_reject_invalid_samples() {
    let mut rng = StdRng::seed_from_u64(42);
    let valid_unit = CheckedUnitIntervalDistribution::new(Fixed(0.25_f32));
    let invalid_unit = CheckedUnitIntervalDistribution::new(Fixed(1.25_f32));
    let valid_signed = CheckedSignedUnitIntervalDistribution::new(Fixed(-0.25_f64));
    let invalid_signed = CheckedSignedUnitIntervalDistribution::new(Fixed(-1.25_f64));

    assert_eq!(
        valid_unit.sample(&mut rng),
        Some(UnitInterval::new(0.25).unwrap())
    );
    assert_eq!(invalid_unit.sample(&mut rng), None);
    assert_eq!(
        valid_signed.sample(&mut rng),
        Some(SignedUnitInterval::new(-0.25).unwrap())
    );
    assert_eq!(invalid_signed.sample(&mut rng), None);
}

#[test]
fn saturating_adapters_defer_to_inner_distribution_and_clamp_invalid_samples() {
    let mut rng = StdRng::seed_from_u64(42);
    let low_unit = SaturatingUnitIntervalDistribution::new(Fixed(-0.25_f32));
    let high_unit = SaturatingUnitIntervalDistribution::new(Fixed(1.25_f32));
    let low_signed = SaturatingSignedUnitIntervalDistribution::new(Fixed(-1.25_f64));
    let high_signed = SaturatingSignedUnitIntervalDistribution::new(Fixed(1.25_f64));

    assert_eq!(low_unit.sample(&mut rng), UnitInterval::ZERO);
    assert_eq!(high_unit.sample(&mut rng), UnitInterval::ONE);
    assert_eq!(low_signed.sample(&mut rng), SignedUnitInterval::NEG_ONE);
    assert_eq!(high_signed.sample(&mut rng), SignedUnitInterval::ONE);
}
