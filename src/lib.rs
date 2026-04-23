//! Small constrained float types for values in the closed intervals `[0, 1]`
//! and `[-1, 1]`.
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

#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

mod signed_unit_interval;
mod unit_interval;

pub use signed_unit_interval::SignedUnitInterval;
pub use signed_unit_interval::SignedUnitIntervalError;
pub use unit_interval::UnitInterval;
pub use unit_interval::UnitIntervalError;
pub use unit_interval::UnitIntervalFloat;
