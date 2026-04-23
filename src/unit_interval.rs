use std::{
    error::Error,
    fmt,
    ops::{Add, Deref, Div, Mul, Sub},
};

/// A floating-point value constrained to the closed unit interval `[0, 1]`.
///
/// `UnitInterval` is useful for normalized values such as opacity, progress,
/// ratios, blend factors, and percentages represented as fractions.
///
/// The default backing type is `f32`; use `UnitInterval<f64>` when you need a
/// wider float.
///
/// # Examples
///
/// ```
/// use unit_interval::UnitInterval;
///
/// let progress = UnitInterval::new(0.75).unwrap();
///
/// assert_eq!(progress.get(), 0.75);
/// assert_eq!(UnitInterval::new(1.25), None);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct UnitInterval<T = f32>(T);

/// Error returned when converting an out-of-range value into a [`UnitInterval`].
///
/// # Examples
///
/// ```
/// use unit_interval::UnitInterval;
///
/// let err = UnitInterval::<f32>::try_from(2.0).unwrap_err();
///
/// assert_eq!(err.to_string(), "value is outside the unit interval");
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UnitIntervalError;

impl fmt::Display for UnitIntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("value is outside the unit interval")
    }
}

impl Error for UnitIntervalError {}

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

    /// The multiplicative identity, `1`.
    const ONE: Self;

    /// The midpoint value, `0.5`.
    const HALF: Self;

    /// Clamps a value into `[0, 1]`.
    ///
    /// Implementations treat `NaN` as zero.
    fn clamp_unit(self) -> Self;
}

impl<T: UnitIntervalFloat> UnitInterval<T> {
    /// The lower bound of the unit interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert_eq!(UnitInterval::<f32>::ZERO.get(), 0.0);
    /// ```
    pub const ZERO: Self = Self(T::ZERO);

    /// The upper bound of the unit interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert_eq!(UnitInterval::<f32>::ONE.get(), 1.0);
    /// ```
    pub const ONE: Self = Self(T::ONE);

    /// The midpoint of the unit interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert_eq!(UnitInterval::<f32>::HALF.get(), 0.5);
    /// ```
    pub const HALF: Self = Self(T::HALF);

    /// Creates a value if `v` is inside `[0, 1]`.
    ///
    /// Returns `None` for values outside the interval and for `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert_eq!(UnitInterval::<f32>::new(0.25).unwrap().get(), 0.25);
    /// assert_eq!(UnitInterval::<f32>::new(-0.25), None);
    /// assert_eq!(UnitInterval::<f32>::new(f32::NAN), None);
    /// ```
    pub fn new(v: T) -> Option<Self> {
        if Self::contains(v) {
            Some(Self(v))
        } else {
            None
        }
    }

    /// Returns whether `v` is inside `[0, 1]`.
    ///
    /// `NaN` is not contained in the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert!(UnitInterval::<f32>::contains(0.5));
    /// assert!(!UnitInterval::<f32>::contains(1.5));
    /// assert!(!UnitInterval::<f32>::contains(f32::NAN));
    /// ```
    pub fn contains(v: T) -> bool {
        v >= T::ZERO && v <= T::ONE
    }

    /// Creates a value by clamping `v` into `[0, 1]`.
    ///
    /// `NaN` is treated as zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert_eq!(UnitInterval::<f32>::saturating(-0.25).get(), 0.0);
    /// assert_eq!(UnitInterval::<f32>::saturating(1.25).get(), 1.0);
    /// assert_eq!(UnitInterval::<f32>::saturating(f32::NAN).get(), 0.0);
    /// ```
    pub fn saturating(v: T) -> Self {
        Self(v.clamp_unit())
    }

    /// Returns the inner floating-point value.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let value = UnitInterval::new(0.25).unwrap();
    ///
    /// assert_eq!(value.get(), 0.25);
    /// ```
    pub const fn get(self) -> T {
        self.0
    }

    /// Consumes the wrapper and returns the inner floating-point value.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let value = UnitInterval::new(0.25).unwrap();
    ///
    /// assert_eq!(value.into_inner(), 0.25);
    /// ```
    pub const fn into_inner(self) -> T {
        self.0
    }

    /// Returns whether this value is exactly zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert!(UnitInterval::<f32>::ZERO.is_zero());
    /// assert!(!UnitInterval::<f32>::HALF.is_zero());
    /// ```
    pub fn is_zero(self) -> bool {
        self.0 == T::ZERO
    }

    /// Returns whether this value is exactly one.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert!(UnitInterval::<f32>::ONE.is_one());
    /// assert!(!UnitInterval::<f32>::HALF.is_one());
    /// ```
    pub fn is_one(self) -> bool {
        self.0 == T::ONE
    }

    /// Returns `1 - self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let value = UnitInterval::new(0.25).unwrap();
    ///
    /// assert_eq!(value.complement().get(), 0.75);
    /// ```
    pub fn complement(self) -> Self {
        Self(T::ONE - self.0)
    }

    /// Returns the smaller of two unit interval values.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(low.min(high), low);
    /// ```
    pub fn min(self, rhs: Self) -> Self {
        if self.0 <= rhs.0 { self } else { rhs }
    }

    /// Returns the larger of two unit interval values.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(low.max(high), high);
    /// ```
    pub fn max(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 { self } else { rhs }
    }

    /// Returns the midpoint between two unit interval values.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(low.midpoint(high).get(), 0.5);
    /// ```
    pub fn midpoint(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) * T::HALF)
    }

    /// Returns the absolute distance between two unit interval values.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(low.distance_to(high).get(), 0.5);
    /// assert_eq!(high.distance_to(low).get(), 0.5);
    /// ```
    pub fn distance_to(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else {
            Self(rhs.0 - self.0)
        }
    }

    /// Adds two values, returning `None` if the result is outside `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let a = UnitInterval::new(0.25).unwrap();
    /// let b = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(a.checked_add(a).unwrap().get(), 0.5);
    /// assert_eq!(b.checked_add(b), None);
    /// ```
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Self::new(self.0 + rhs.0)
    }

    /// Adds two values and clamps the result into `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let value = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(value.saturating_add(value).get(), 1.0);
    /// ```
    pub fn saturating_add(self, rhs: Self) -> Self {
        Self::saturating(self.0 + rhs.0)
    }

    /// Subtracts `rhs`, returning `None` if the result is outside `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(high.checked_sub(low).unwrap().get(), 0.5);
    /// assert_eq!(low.checked_sub(high), None);
    /// ```
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Self::new(self.0 - rhs.0)
    }

    /// Subtracts `rhs` and clamps the result into `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(low.saturating_sub(high).get(), 0.0);
    /// ```
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self::saturating(self.0 - rhs.0)
    }

    /// Divides by `rhs`, returning `None` if the result is outside `[0, 1]`.
    ///
    /// Division by zero follows the backing float semantics and produces
    /// `None`, because infinity is outside the unit interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(low.checked_div(high).unwrap().get(), 1.0 / 3.0);
    /// assert_eq!(high.checked_div(low), None);
    /// assert_eq!(high.checked_div(UnitInterval::ZERO), None);
    /// ```
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        Self::new(self.0 / rhs.0)
    }

    /// Divides by `rhs` and clamps the result into `[0, 1]`.
    ///
    /// Division by zero follows the backing float semantics and saturates to
    /// one for positive infinity, or zero for `0 / 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let low = UnitInterval::new(0.25).unwrap();
    /// let high = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(high.saturating_div(low).get(), 1.0);
    /// assert_eq!(low.saturating_div(UnitInterval::ZERO).get(), 1.0);
    /// ```
    pub fn saturating_div(self, rhs: Self) -> Self {
        Self::saturating(self.0 / rhs.0)
    }

    /// Multiplies by an arbitrary float, returning `None` if the result is
    /// outside `[0, 1]`.
    ///
    /// Use the `*` operator when multiplying by another [`UnitInterval`], since
    /// that operation always stays inside the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let value = UnitInterval::new(0.25).unwrap();
    ///
    /// assert_eq!(value.checked_scale(2.0).unwrap().get(), 0.5);
    /// assert_eq!(value.checked_scale(8.0), None);
    /// ```
    pub fn checked_scale(self, factor: T) -> Option<Self> {
        Self::new(self.0 * factor)
    }

    /// Multiplies by an arbitrary float and clamps the result into `[0, 1]`.
    ///
    /// Use the `*` operator when multiplying by another [`UnitInterval`], since
    /// that operation always stays inside the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// let value = UnitInterval::new(0.75).unwrap();
    ///
    /// assert_eq!(value.saturating_scale(2.0).get(), 1.0);
    /// assert_eq!(value.saturating_scale(-1.0).get(), 0.0);
    /// ```
    pub fn saturating_scale(self, factor: T) -> Self {
        Self::saturating(self.0 * factor)
    }

    /// Linearly interpolates between `start` and `end`.
    ///
    /// A value of zero returns `start`, one returns `end`, and values between
    /// zero and one return the corresponding point between them.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_interval::UnitInterval;
    ///
    /// assert_eq!(UnitInterval::<f32>::ZERO.lerp(10.0, 20.0), 10.0);
    /// assert_eq!(UnitInterval::<f32>::HALF.lerp(10.0, 20.0), 15.0);
    /// assert_eq!(UnitInterval::<f32>::ONE.lerp(10.0, 20.0), 20.0);
    /// ```
    pub fn lerp(self, start: T, end: T) -> T {
        start + (end - start) * self.0
    }
}

/// Returns [`UnitInterval::ZERO`].
impl<T: UnitIntervalFloat> Default for UnitInterval<T> {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Dereferences to the inner floating-point value.
impl<T> Deref for UnitInterval<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Borrows the inner floating-point value.
impl<T> AsRef<T> for UnitInterval<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

macro_rules! impl_unit_interval_float {
    ($float:ty) => {
        impl private::Sealed for $float {}

        impl UnitIntervalFloat for $float {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            const HALF: Self = 0.5;

            fn clamp_unit(self) -> Self {
                if self.is_nan() {
                    return Self::ZERO;
                }

                self.clamp(Self::ZERO, Self::ONE)
            }
        }

        impl From<UnitInterval<$float>> for $float {
            fn from(u: UnitInterval<$float>) -> Self {
                u.0
            }
        }

        impl TryFrom<$float> for UnitInterval<$float> {
            type Error = UnitIntervalError;

            fn try_from(value: $float) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(UnitIntervalError)
            }
        }
    };
}

impl_unit_interval_float!(f32);
impl_unit_interval_float!(f64);

/// Converts a `UnitInterval<f32>` into its inner value widened to `f64`.
impl From<UnitInterval<f32>> for f64 {
    fn from(u: UnitInterval) -> Self {
        u.0 as f64
    }
}

/// Converts a `UnitInterval<f32>` into `UnitInterval<f64>`.
impl From<UnitInterval<f32>> for UnitInterval<f64> {
    fn from(u: UnitInterval<f32>) -> Self {
        Self(u.0 as f64)
    }
}

/// Converts a `UnitInterval<f64>` into `UnitInterval<f32>`.
impl From<UnitInterval<f64>> for UnitInterval<f32> {
    fn from(u: UnitInterval<f64>) -> Self {
        Self(u.0 as f32)
    }
}

/// Multiplies two unit interval values.
///
/// Multiplication is implemented as an operator because the product of two
/// values in `[0, 1]` is also in `[0, 1]`.
///
/// # Examples
///
/// ```
/// use unit_interval::UnitInterval;
///
/// let a = UnitInterval::new(0.25).unwrap();
/// let b = UnitInterval::new(0.75).unwrap();
///
/// assert_eq!((a * b).get(), 0.1875);
/// ```
impl<T: UnitIntervalFloat> Mul for UnitInterval<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

mod private {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::UnitInterval;

    #[test]
    fn f32_constructor_accepts_unit_interval() {
        let default_width: UnitInterval = UnitInterval::new(0.5).unwrap();

        assert_eq!(default_width.get(), 0.5);
        assert_eq!(UnitInterval::<f32>::new(0.0).map(|u| u.get()), Some(0.0));
        assert_eq!(UnitInterval::<f32>::new(0.5).map(|u| u.get()), Some(0.5));
        assert_eq!(UnitInterval::<f32>::new(1.0).map(|u| u.get()), Some(1.0));
        assert_eq!(UnitInterval::<f32>::new(-0.1), None);
        assert_eq!(UnitInterval::<f32>::new(1.1), None);
    }

    #[test]
    fn f64_constructor_accepts_unit_interval() {
        assert_eq!(UnitInterval::<f64>::new(0.0).map(|u| u.get()), Some(0.0));
        assert_eq!(UnitInterval::<f64>::new(0.5).map(|u| u.get()), Some(0.5));
        assert_eq!(UnitInterval::<f64>::new(1.0).map(|u| u.get()), Some(1.0));
        assert_eq!(UnitInterval::<f64>::new(-0.1), None);
        assert_eq!(UnitInterval::<f64>::new(1.1), None);
    }

    #[test]
    fn constructors_reject_nan() {
        assert_eq!(UnitInterval::<f32>::new(f32::NAN), None);
        assert_eq!(UnitInterval::<f64>::new(f64::NAN), None);
    }

    #[test]
    fn saturating_clamps_for_both_float_widths() {
        assert_eq!(UnitInterval::<f32>::saturating(-0.1).get(), 0.0);
        assert_eq!(UnitInterval::<f32>::saturating(1.1).get(), 1.0);
        assert_eq!(UnitInterval::<f32>::saturating(f32::NAN).get(), 0.0);
        assert_eq!(UnitInterval::<f64>::saturating(-0.1).get(), 0.0);
        assert_eq!(UnitInterval::<f64>::saturating(1.1).get(), 1.0);
        assert_eq!(UnitInterval::<f64>::saturating(f64::NAN).get(), 0.0);
    }

    #[test]
    fn constants_and_basic_helpers_work() {
        assert_eq!(UnitInterval::<f32>::default(), UnitInterval::<f32>::ZERO);
        assert_eq!(UnitInterval::<f32>::ZERO.get(), 0.0);
        assert_eq!(UnitInterval::<f32>::HALF.get(), 0.5);
        assert_eq!(UnitInterval::<f32>::ONE.get(), 1.0);
        assert!(UnitInterval::<f32>::contains(0.5));
        assert!(!UnitInterval::<f32>::contains(1.5));
        assert!(UnitInterval::<f32>::ZERO.is_zero());
        assert!(UnitInterval::<f32>::ONE.is_one());
        assert_eq!(UnitInterval::new(0.25).unwrap().complement().get(), 0.75);
    }

    #[test]
    fn standard_conversions_are_available() {
        let from_result = UnitInterval::<f32>::try_from(0.5).unwrap();
        let as_f64_interval = UnitInterval::<f64>::from(from_result);
        let as_f32_interval = UnitInterval::<f32>::from(as_f64_interval);

        assert_eq!(from_result.get(), 0.5);
        assert_eq!(
            UnitInterval::<f32>::try_from(1.5).unwrap_err().to_string(),
            "value is outside the unit interval"
        );
        assert_eq!(as_f64_interval.get(), 0.5);
        assert_eq!(as_f32_interval.get(), 0.5);
    }

    #[test]
    fn comparison_helpers_preserve_unit_interval() {
        let low = UnitInterval::new(0.25).unwrap();
        let high = UnitInterval::new(0.75).unwrap();

        assert_eq!(low.min(high), low);
        assert_eq!(low.max(high), high);
        assert_eq!(low.midpoint(high).get(), 0.5);
        assert_eq!(low.distance_to(high).get(), 0.5);
    }

    #[test]
    fn checked_arithmetic_rejects_out_of_range_results() {
        let low = UnitInterval::new(0.25).unwrap();
        let high = UnitInterval::new(0.75).unwrap();

        assert_eq!(low.checked_add(low).map(|u| u.get()), Some(0.5));
        assert_eq!(high.checked_add(high), None);
        assert_eq!(high.checked_sub(low).map(|u| u.get()), Some(0.5));
        assert_eq!(low.checked_sub(high), None);
        assert_eq!(low.checked_div(high).map(|u| u.get()), Some(1.0 / 3.0));
        assert_eq!(high.checked_div(low), None);
        assert_eq!(low.checked_scale(2.0).map(|u| u.get()), Some(0.5));
        assert_eq!(high.checked_scale(2.0), None);
    }

    #[test]
    fn saturating_arithmetic_clamps_out_of_range_results() {
        let low = UnitInterval::new(0.25).unwrap();
        let high = UnitInterval::new(0.75).unwrap();

        assert_eq!(high.saturating_add(high).get(), 1.0);
        assert_eq!(low.saturating_sub(high).get(), 0.0);
        assert_eq!(high.saturating_div(low).get(), 1.0);
        assert_eq!(low.saturating_scale(-1.0).get(), 0.0);
        assert_eq!(high.saturating_scale(2.0).get(), 1.0);
    }

    #[test]
    fn multiplication_and_lerp_are_convenient() {
        let low = UnitInterval::new(0.25).unwrap();
        let high = UnitInterval::new(0.75).unwrap();

        assert_eq!((low * high).get(), 0.1875);
        assert_eq!(UnitInterval::<f32>::HALF.lerp(10.0, 20.0), 15.0);
    }
}
