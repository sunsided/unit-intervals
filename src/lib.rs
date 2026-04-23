//! A small constrained float type for values in the closed unit interval `[0, 1]`.
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

mod unit_interval;

pub use unit_interval::UnitInterval;
pub use unit_interval::UnitIntervalError;
pub use unit_interval::UnitIntervalFloat;
