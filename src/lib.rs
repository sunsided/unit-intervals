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
//! - `std`: enables APIs that require the Rust standard library. The crate is
//!   otherwise `no_std`.
//! - `unsafe`: allows unsafe code and enables unchecked constructors and
//!   operations such as [`UnitInterval::new_unchecked`] and
//!   [`SignedUnitInterval::new_unchecked`]. These APIs assume the caller has
//!   already proven that the produced value is inside the relevant interval and
//!   is not `NaN`.

#![no_std]
#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]
#![cfg_attr(feature = "unsafe", allow(unsafe_code))]

#[cfg(any(test, feature = "std"))]
extern crate std;

mod signed_unit_interval;
mod unit_interval;

pub use signed_unit_interval::SignedUnitInterval;
pub use signed_unit_interval::SignedUnitIntervalError;
pub use unit_interval::UnitInterval;
pub use unit_interval::UnitIntervalError;
pub use unit_interval::UnitIntervalFloat;
