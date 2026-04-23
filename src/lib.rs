//! # unit-interval
//!
//! Small constrained float types for values in the closed intervals `[0, 1]`
//! and `[-1, 1]`.
//!
//! This crate provides [`UnitInterval`] and [`SignedUnitInterval`] wrappers for
//! normalized floating-point values. They are useful for values such as opacity,
//! progress, blend factors, percentages represented as fractions, centered
//! offsets, joystick axes, and other quantities where the valid range is part of
//! the type.
//!
//! Constructors reject out-of-range values and `NaN`; saturating constructors
//! clamp inputs into range. Operations that can leave the interval are available
//! in checked and saturating forms, while operations that are mathematically
//! closed over the interval return constrained values directly.
//!
//! # Examples
//!
//! ```
//! use unit_interval::UnitInterval;
//!
//! let opacity = UnitInterval::new(0.8).unwrap();
//! let clamped = UnitInterval::saturating(1.2);
//!
//! assert_eq!(opacity.get(), 0.8);
//! assert_eq!(clamped, UnitInterval::ONE);
//! ```
//!
//! ```
//! use unit_interval::{SignedUnitInterval, UnitInterval};
//!
//! let axis = SignedUnitInterval::new(-0.5).unwrap();
//! let scale = UnitInterval::new(0.25).unwrap();
//!
//! assert_eq!((axis * scale).get(), -0.125);
//! assert_eq!(axis.saturating_add(scale).get(), -0.25);
//! assert_eq!(
//!     axis.checked_add(SignedUnitInterval::<f32>::ONE),
//!     Some(SignedUnitInterval::<f32>::HALF),
//! );
//! ```
//!
//! # Crate features
//!
//! - `assertions`: enables internal invariant assertions in non-test builds.
//!   Tests always enable these assertions.
//! - `bytemuck`: enables [`bytemuck::Zeroable`], [`bytemuck::NoUninit`], and
//!   [`bytemuck::CheckedBitPattern`] support. These wrappers do not implement
//!   [`bytemuck::Pod`] because not every backing float bit pattern satisfies
//!   their interval invariants.
//! - `std`: enables APIs that require the Rust standard library. The crate is
//!   otherwise `no_std`.
//! - `rand_distr`: enables [`rand_distr`] distribution support for
//!   [`UnitInterval`] and [`SignedUnitInterval`].
//! - `rkyv`: enables zero-copy serialization and checked deserialization
//!   through the inner floating-point value.
//! - `serde`: enables transparent serialization and checked deserialization
//!   through the inner floating-point value.
//! - `unsafe`: allows unsafe code and enables unchecked constructors and
//!   operations such as [`UnitInterval::new_unchecked`] and
//!   [`SignedUnitInterval::new_unchecked`]. These APIs assume the caller has
//!   already proven that the produced value is inside the relevant interval and
//!   is not `NaN`.

#![no_std]
#![cfg_attr(
    not(any(feature = "unsafe", feature = "bytemuck")),
    forbid(unsafe_code)
)]
#![cfg_attr(any(feature = "unsafe", feature = "bytemuck"), allow(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "rand_distr")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_distr")))]
pub mod random;
mod signed_unit_interval;
mod unit_interval;

use core::ops::{Add, Div, Mul, Sub};
pub use signed_unit_interval::SignedUnitInterval;
pub use signed_unit_interval::SignedUnitIntervalError;
pub use unit_interval::UnitInterval;
pub use unit_interval::UnitIntervalError;

mod private {
    pub trait Sealed {}
}

/// Floating-point support required by [`UnitInterval`].
///
/// This trait is sealed and implemented only for `f32` and `f64`.
pub trait UnitIntervalFloat:
    private::Sealed
    + Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    /// The additive identity, `0`.
    const ZERO: Self;

    /// The lower bound of the signed unit interval, `-1`.
    const NEG_ONE: Self;

    /// The multiplicative identity, `1`.
    const ONE: Self;

    /// The midpoint value, `0.5`.
    const HALF: Self;

    /// Clamps a value into `[0, 1]`.
    ///
    /// Implementations treat `NaN` as zero.
    fn clamp_unit(self) -> Self;

    /// Clamps a value into `[-1, 1]`.
    ///
    /// Implementations treat `NaN` as zero.
    fn clamp_signed_unit(self) -> Self;
}
