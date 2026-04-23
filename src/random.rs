//! Random sampling support through [`rand_distr`].
//!
//! This module is available with the `rand_distr` crate feature. It implements
//! [`Distribution`] for interval types where the sampled backing float is
//! guaranteed to satisfy the interval invariant, and provides adapter
//! distributions for arbitrary float distributions.
//!
//! [`StandardUniform`] samples [`UnitInterval`] values from `[0, 1)` and
//! [`SignedUnitInterval`] values from `[-1, 1)`.
//!
//! ```
//! use rand::{RngExt, SeedableRng, rngs::StdRng};
//! use rand_distr::{Distribution, StandardUniform};
//! use unit_interval::{SignedUnitInterval, UnitInterval};
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let unit: UnitInterval<f32> = StandardUniform.sample(&mut rng);
//! let signed: SignedUnitInterval<f64> = rng.random();
//!
//! assert!(UnitInterval::<f32>::contains(unit.get()));
//! assert!(SignedUnitInterval::<f64>::contains(signed.get()));
//! ```
//!
//! Bounded `rand_distr` float distributions whose support is inside `[0, 1]`
//! can also sample [`UnitInterval`] values directly.
//!
//! ```
//! use rand::{SeedableRng, rngs::StdRng};
//! use rand_distr::{Beta, Distribution, Open01, OpenClosed01};
//! use unit_interval::UnitInterval;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let beta = Beta::new(2.0_f64, 5.0).unwrap();
//!
//! let open: UnitInterval<f32> = Open01.sample(&mut rng);
//! let open_closed: UnitInterval<f64> = OpenClosed01.sample(&mut rng);
//! let beta_value: UnitInterval<f64> = beta.sample(&mut rng);
//!
//! assert!(UnitInterval::<f32>::contains(open.get()));
//! assert!(UnitInterval::<f64>::contains(open_closed.get()));
//! assert!(UnitInterval::<f64>::contains(beta_value.get()));
//! ```
//!
//! Arbitrary float distributions may produce values outside the target
//! interval. Use checked adapters when out-of-range samples should become
//! `None`, or saturating adapters when they should be clamped.
//!
//! ```
//! use rand::{SeedableRng, rngs::StdRng};
//! use rand_distr::{Distribution, Normal};
//! use unit_interval::{
//!     UnitInterval,
//!     random::{CheckedUnitIntervalDistribution, SaturatingUnitIntervalDistribution},
//! };
//!
//! let normal = Normal::new(0.5_f32, 2.0).unwrap();
//! let checked = CheckedUnitIntervalDistribution::new(normal);
//! let saturating = SaturatingUnitIntervalDistribution::new(normal);
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let checked_sample: Option<UnitInterval<f32>> = checked.sample(&mut rng);
//! let saturating_sample: UnitInterval<f32> = saturating.sample(&mut rng);
//!
//! assert!(checked_sample.is_none_or(|value| UnitInterval::<f32>::contains(value.get())));
//! assert!(UnitInterval::<f32>::contains(saturating_sample.get()));
//! ```
//!
//! The same adapter pattern is available for [`SignedUnitInterval`].
//!
//! ```
//! use rand::{SeedableRng, rngs::StdRng};
//! use rand_distr::{Distribution, Normal};
//! use unit_interval::{
//!     SignedUnitInterval,
//!     random::{
//!         CheckedSignedUnitIntervalDistribution, SaturatingSignedUnitIntervalDistribution,
//!     },
//! };
//!
//! let normal = Normal::new(0.0_f64, 2.0).unwrap();
//! let checked = CheckedSignedUnitIntervalDistribution::new(normal);
//! let saturating = SaturatingSignedUnitIntervalDistribution::new(normal);
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let checked_sample: Option<SignedUnitInterval<f64>> = checked.sample(&mut rng);
//! let saturating_sample: SignedUnitInterval<f64> = saturating.sample(&mut rng);
//!
//! assert!(checked_sample.is_none_or(|value| SignedUnitInterval::<f64>::contains(value.get())));
//! assert!(SignedUnitInterval::<f64>::contains(saturating_sample.get()));
//! ```

use crate::{SignedUnitInterval, UnitInterval, UnitIntervalFloat};
use ::rand::Rng;
use ::rand_distr::{Beta, Distribution, Open01, OpenClosed01, StandardUniform};

/// Adapts an arbitrary distribution over raw floats into checked
/// [`UnitInterval`] samples.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CheckedUnitIntervalDistribution<D> {
    distribution: D,
}

impl<D> CheckedUnitIntervalDistribution<D> {
    /// Creates a checked unit interval distribution adapter.
    #[inline]
    pub const fn new(distribution: D) -> Self {
        Self { distribution }
    }

    /// Returns a shared reference to the wrapped distribution.
    #[inline]
    pub const fn as_inner(&self) -> &D {
        &self.distribution
    }

    /// Consumes the adapter and returns the wrapped distribution.
    #[inline]
    pub fn into_inner(self) -> D {
        self.distribution
    }
}

/// Adapts an arbitrary distribution over raw floats into saturating
/// [`UnitInterval`] samples.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SaturatingUnitIntervalDistribution<D> {
    distribution: D,
}

impl<D> SaturatingUnitIntervalDistribution<D> {
    /// Creates a saturating unit interval distribution adapter.
    #[inline]
    pub const fn new(distribution: D) -> Self {
        Self { distribution }
    }

    /// Returns a shared reference to the wrapped distribution.
    #[inline]
    pub const fn as_inner(&self) -> &D {
        &self.distribution
    }

    /// Consumes the adapter and returns the wrapped distribution.
    #[inline]
    pub fn into_inner(self) -> D {
        self.distribution
    }
}

/// Adapts an arbitrary distribution over raw floats into checked
/// [`SignedUnitInterval`] samples.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CheckedSignedUnitIntervalDistribution<D> {
    distribution: D,
}

impl<D> CheckedSignedUnitIntervalDistribution<D> {
    /// Creates a checked signed unit interval distribution adapter.
    #[inline]
    pub const fn new(distribution: D) -> Self {
        Self { distribution }
    }

    /// Returns a shared reference to the wrapped distribution.
    #[inline]
    pub const fn as_inner(&self) -> &D {
        &self.distribution
    }

    /// Consumes the adapter and returns the wrapped distribution.
    #[inline]
    pub fn into_inner(self) -> D {
        self.distribution
    }
}

/// Adapts an arbitrary distribution over raw floats into saturating
/// [`SignedUnitInterval`] samples.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SaturatingSignedUnitIntervalDistribution<D> {
    distribution: D,
}

impl<D> SaturatingSignedUnitIntervalDistribution<D> {
    /// Creates a saturating signed unit interval distribution adapter.
    #[inline]
    pub const fn new(distribution: D) -> Self {
        Self { distribution }
    }

    /// Returns a shared reference to the wrapped distribution.
    #[inline]
    pub const fn as_inner(&self) -> &D {
        &self.distribution
    }

    /// Consumes the adapter and returns the wrapped distribution.
    #[inline]
    pub fn into_inner(self) -> D {
        self.distribution
    }
}

impl<T, D> Distribution<Option<UnitInterval<T>>> for CheckedUnitIntervalDistribution<D>
where
    T: UnitIntervalFloat,
    D: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<UnitInterval<T>> {
        UnitInterval::new(self.distribution.sample(rng))
    }
}

impl<T, D> Distribution<UnitInterval<T>> for SaturatingUnitIntervalDistribution<D>
where
    T: UnitIntervalFloat,
    D: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnitInterval<T> {
        UnitInterval::saturating(self.distribution.sample(rng))
    }
}

impl<T, D> Distribution<Option<SignedUnitInterval<T>>> for CheckedSignedUnitIntervalDistribution<D>
where
    T: UnitIntervalFloat,
    D: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<SignedUnitInterval<T>> {
        SignedUnitInterval::new(self.distribution.sample(rng))
    }
}

impl<T, D> Distribution<SignedUnitInterval<T>> for SaturatingSignedUnitIntervalDistribution<D>
where
    T: UnitIntervalFloat,
    D: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SignedUnitInterval<T> {
        SignedUnitInterval::saturating(self.distribution.sample(rng))
    }
}

impl<T> Distribution<UnitInterval<T>> for StandardUniform
where
    T: UnitIntervalFloat,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnitInterval<T> {
        UnitInterval::from_inner(<Self as Distribution<T>>::sample(self, rng))
    }
}

impl<T> Distribution<SignedUnitInterval<T>> for StandardUniform
where
    T: UnitIntervalFloat,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SignedUnitInterval<T> {
        let value = <Self as Distribution<T>>::sample(self, rng) * (T::ONE + T::ONE) - T::ONE;

        SignedUnitInterval::from_inner(value)
    }
}

macro_rules! impl_unit_interval_distribution {
    ($distribution:ty, $float:ty) => {
        impl Distribution<UnitInterval<$float>> for $distribution {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnitInterval<$float> {
                UnitInterval::from_inner(<Self as Distribution<$float>>::sample(self, rng))
            }
        }
    };
}

impl_unit_interval_distribution!(Open01, f32);
impl_unit_interval_distribution!(Open01, f64);
impl_unit_interval_distribution!(OpenClosed01, f32);
impl_unit_interval_distribution!(OpenClosed01, f64);
impl_unit_interval_distribution!(Beta<f32>, f32);
impl_unit_interval_distribution!(Beta<f64>, f64);
