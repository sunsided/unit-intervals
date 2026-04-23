use core::{
    cmp::Ordering,
    error::Error,
    fmt,
    ops::{Add, Deref, Div, Mul, Neg, Rem, Sub},
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
    #[inline]
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
    #[inline(always)]
    pub fn new(v: T) -> Option<Self> {
        if Self::contains(v) {
            Some(Self::from_inner(v))
        } else {
            None
        }
    }

    /// Creates a value without checking that `v` is inside `[0, 1]`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `v` is greater than or equal to zero,
    /// less than or equal to one, and not `NaN`.
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub const unsafe fn new_unchecked(v: T) -> Self {
        Self(v)
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
    #[inline]
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
    #[inline]
    pub fn saturating(v: T) -> Self {
        Self::from_inner(v.clamp_unit())
    }

    #[inline(always)]
    pub(crate) fn from_inner(v: T) -> Self {
        Self::assert_contains(v);
        Self(v)
    }

    #[cfg(any(test, feature = "assertions"))]
    #[inline(always)]
    fn assert_contains(v: T) {
        assert!(
            Self::contains(v),
            "UnitInterval invariant violated: value is outside [0, 1]"
        );
    }

    #[cfg(not(any(test, feature = "assertions")))]
    #[inline(always)]
    fn assert_contains(_v: T) {}

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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn complement(self) -> Self {
        Self::from_inner(T::ONE - self.0)
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn midpoint(self, rhs: Self) -> Self {
        Self::from_inner((self.0 + rhs.0) * T::HALF)
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
    #[inline]
    pub fn distance_to(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 {
            Self::from_inner(self.0 - rhs.0)
        } else {
            Self::from_inner(rhs.0 - self.0)
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
    #[inline(always)]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Self::new(self.0 + rhs.0)
    }

    /// Adds two values without checking that the result is inside `[0, 1]`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `self + rhs` is inside `[0, 1]` and not
    /// `NaN`.
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub unsafe fn add_unchecked(self, rhs: Self) -> Self {
        // SAFETY: Guaranteed by the caller.
        unsafe { Self::new_unchecked(self.0 + rhs.0) }
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Self::new(self.0 - rhs.0)
    }

    /// Subtracts `rhs` without checking that the result is inside `[0, 1]`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `self - rhs` is inside `[0, 1]` and not
    /// `NaN`.
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub unsafe fn sub_unchecked(self, rhs: Self) -> Self {
        // SAFETY: Guaranteed by the caller.
        unsafe { Self::new_unchecked(self.0 - rhs.0) }
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        Self::new(self.0 / rhs.0)
    }

    /// Divides by `rhs` without checking that the result is inside `[0, 1]`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `self / rhs` is inside `[0, 1]` and not
    /// `NaN`.
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub unsafe fn div_unchecked(self, rhs: Self) -> Self {
        // SAFETY: Guaranteed by the caller.
        unsafe { Self::new_unchecked(self.0 / rhs.0) }
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn checked_scale(self, factor: T) -> Option<Self> {
        Self::new(self.0 * factor)
    }

    /// Multiplies by an arbitrary float without checking that the result is
    /// inside `[0, 1]`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `self * factor` is inside `[0, 1]` and
    /// not `NaN`.
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub unsafe fn scale_unchecked(self, factor: T) -> Self {
        // SAFETY: Guaranteed by the caller.
        unsafe { Self::new_unchecked(self.0 * factor) }
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
    #[inline(always)]
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
    #[inline]
    pub fn lerp(self, start: T, end: T) -> T {
        start + (end - start) * self.0
    }
}

/// Returns [`UnitInterval::ZERO`].
impl<T: UnitIntervalFloat> Default for UnitInterval<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

/// Dereferences to the inner floating-point value.
impl<T> Deref for UnitInterval<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Borrows the inner floating-point value.
impl<T> AsRef<T> for UnitInterval<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

macro_rules! impl_unit_interval_float {
    ($float:ty) => {
        impl private::Sealed for $float {}

        impl UnitIntervalFloat for $float {
            const ZERO: Self = 0.0;
            const NEG_ONE: Self = -1.0;
            const ONE: Self = 1.0;
            const HALF: Self = 0.5;

            #[inline]
            fn clamp_unit(self) -> Self {
                if self.is_nan() {
                    return Self::ZERO;
                }

                self.clamp(Self::ZERO, Self::ONE)
            }

            #[inline]
            fn clamp_signed_unit(self) -> Self {
                if self.is_nan() {
                    return Self::ZERO;
                }

                self.clamp(Self::NEG_ONE, Self::ONE)
            }
        }

        impl From<UnitInterval<$float>> for $float {
            #[inline(always)]
            fn from(u: UnitInterval<$float>) -> Self {
                u.0
            }
        }

        impl TryFrom<$float> for UnitInterval<$float> {
            type Error = UnitIntervalError;

            #[inline]
            fn try_from(value: $float) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(UnitIntervalError)
            }
        }

        impl PartialEq<$float> for UnitInterval<$float> {
            #[inline(always)]
            fn eq(&self, other: &$float) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<UnitInterval<$float>> for $float {
            #[inline(always)]
            fn eq(&self, other: &UnitInterval<$float>) -> bool {
                *self == other.0
            }
        }

        impl PartialOrd<$float> for UnitInterval<$float> {
            #[inline(always)]
            fn partial_cmp(&self, other: &$float) -> Option<Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl PartialOrd<UnitInterval<$float>> for $float {
            #[inline(always)]
            fn partial_cmp(&self, other: &UnitInterval<$float>) -> Option<Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl Add for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                self.0 + rhs.0
            }
        }

        impl Add<$float> for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn add(self, rhs: $float) -> Self::Output {
                self.0 + rhs
            }
        }

        impl Add<UnitInterval<$float>> for $float {
            type Output = $float;

            #[inline(always)]
            fn add(self, rhs: UnitInterval<$float>) -> Self::Output {
                self + rhs.0
            }
        }

        impl Sub for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                self.0 - rhs.0
            }
        }

        impl Sub<$float> for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn sub(self, rhs: $float) -> Self::Output {
                self.0 - rhs
            }
        }

        impl Sub<UnitInterval<$float>> for $float {
            type Output = $float;

            #[inline(always)]
            fn sub(self, rhs: UnitInterval<$float>) -> Self::Output {
                self - rhs.0
            }
        }

        impl Mul<$float> for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn mul(self, rhs: $float) -> Self::Output {
                self.0 * rhs
            }
        }

        impl Mul<UnitInterval<$float>> for $float {
            type Output = $float;

            #[inline(always)]
            fn mul(self, rhs: UnitInterval<$float>) -> Self::Output {
                self * rhs.0
            }
        }

        impl Div for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Div<$float> for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn div(self, rhs: $float) -> Self::Output {
                self.0 / rhs
            }
        }

        impl Div<UnitInterval<$float>> for $float {
            type Output = $float;

            #[inline(always)]
            fn div(self, rhs: UnitInterval<$float>) -> Self::Output {
                self / rhs.0
            }
        }

        impl Rem for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self::Output {
                self.0 % rhs.0
            }
        }

        impl Rem<$float> for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn rem(self, rhs: $float) -> Self::Output {
                self.0 % rhs
            }
        }

        impl Rem<UnitInterval<$float>> for $float {
            type Output = $float;

            #[inline(always)]
            fn rem(self, rhs: UnitInterval<$float>) -> Self::Output {
                self % rhs.0
            }
        }

        impl Neg for UnitInterval<$float> {
            type Output = $float;

            #[inline(always)]
            fn neg(self) -> Self::Output {
                -self.0
            }
        }

        #[cfg(any(test, feature = "std"))]
        impl UnitInterval<$float> {
            /// Returns the largest integer less than or equal to this value.
            #[inline]
            pub fn floor(self) -> Self {
                Self::from_inner(self.0.floor())
            }

            /// Returns the smallest integer greater than or equal to this value.
            #[inline]
            pub fn ceil(self) -> Self {
                Self::from_inner(self.0.ceil())
            }

            /// Returns the nearest integer to this value, rounding halfway cases away from zero.
            #[inline]
            pub fn round(self) -> Self {
                Self::from_inner(self.0.round())
            }

            /// Returns the integer part of this value.
            #[inline]
            pub fn trunc(self) -> Self {
                Self::from_inner(self.0.trunc())
            }

            /// Returns the fractional part of this value.
            #[inline]
            pub fn fract(self) -> Self {
                Self::from_inner(self.0.fract())
            }

            /// Returns the absolute value.
            #[inline]
            pub fn abs(self) -> Self {
                Self::from_inner(self.0.abs())
            }

            /// Returns a number representing the sign of this value.
            #[inline(always)]
            pub fn signum(self) -> $float {
                self.0.signum()
            }

            /// Returns this value with the sign of `sign`.
            #[inline(always)]
            pub fn copysign(self, sign: $float) -> $float {
                self.0.copysign(sign)
            }

            /// Returns `true` if this value is positive zero.
            #[inline(always)]
            pub fn is_sign_positive(self) -> bool {
                self.0.is_sign_positive()
            }

            /// Returns `true` if this value is negative zero.
            #[inline(always)]
            pub fn is_sign_negative(self) -> bool {
                self.0.is_sign_negative()
            }

            /// Returns `true`; unit interval values are always finite.
            #[inline(always)]
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }

            /// Returns `false`; unit interval values cannot be infinite.
            #[inline(always)]
            pub fn is_infinite(self) -> bool {
                self.0.is_infinite()
            }

            /// Returns `false`; unit interval values cannot be `NaN`.
            #[inline(always)]
            pub fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            /// Takes the reciprocal, `1 / self`.
            #[inline(always)]
            pub fn recip(self) -> $float {
                self.0.recip()
            }

            /// Raises this value to an integer power.
            #[inline(always)]
            pub fn powi(self, n: i32) -> $float {
                self.0.powi(n)
            }

            /// Raises this value to a floating-point power.
            #[inline(always)]
            pub fn powf(self, n: $float) -> $float {
                self.0.powf(n)
            }

            /// Returns the square root.
            #[inline(always)]
            pub fn sqrt(self) -> Self {
                Self::from_inner(self.0.sqrt())
            }

            /// Returns the cube root.
            #[inline(always)]
            pub fn cbrt(self) -> Self {
                Self::from_inner(self.0.cbrt())
            }

            /// Computes `self * a + b` with one rounding error.
            #[inline(always)]
            pub fn mul_add(self, a: $float, b: $float) -> $float {
                self.0.mul_add(a, b)
            }

            /// Returns the Euclidean division of this value by `rhs`.
            #[inline(always)]
            pub fn div_euclid(self, rhs: $float) -> $float {
                self.0.div_euclid(rhs)
            }

            /// Returns the least non-negative remainder of this value divided by `rhs`.
            #[inline(always)]
            pub fn rem_euclid(self, rhs: $float) -> $float {
                self.0.rem_euclid(rhs)
            }

            /// Returns `e^(self)`.
            #[inline(always)]
            pub fn exp(self) -> $float {
                self.0.exp()
            }

            /// Returns `2^(self)`.
            #[inline(always)]
            pub fn exp2(self) -> $float {
                self.0.exp2()
            }

            /// Returns the natural logarithm.
            #[inline(always)]
            pub fn ln(self) -> $float {
                self.0.ln()
            }

            /// Returns the logarithm with respect to an arbitrary base.
            #[inline(always)]
            pub fn log(self, base: $float) -> $float {
                self.0.log(base)
            }

            /// Returns the base 2 logarithm.
            #[inline(always)]
            pub fn log2(self) -> $float {
                self.0.log2()
            }

            /// Returns the base 10 logarithm.
            #[inline(always)]
            pub fn log10(self) -> $float {
                self.0.log10()
            }

            /// Returns the sine, in radians.
            #[inline(always)]
            pub fn sin(self) -> $float {
                self.0.sin()
            }

            /// Returns the cosine, in radians.
            #[inline(always)]
            pub fn cos(self) -> $float {
                self.0.cos()
            }

            /// Returns the tangent, in radians.
            #[inline(always)]
            pub fn tan(self) -> $float {
                self.0.tan()
            }

            /// Returns both sine and cosine, in radians.
            #[inline(always)]
            pub fn sin_cos(self) -> ($float, $float) {
                self.0.sin_cos()
            }

            /// Returns the arcsine, in radians.
            #[inline(always)]
            pub fn asin(self) -> $float {
                self.0.asin()
            }

            /// Returns the arccosine, in radians.
            #[inline(always)]
            pub fn acos(self) -> $float {
                self.0.acos()
            }

            /// Returns the arctangent, in radians.
            #[inline(always)]
            pub fn atan(self) -> Self {
                Self::from_inner(self.0.atan())
            }

            /// Returns the four-quadrant arctangent of `self` and `other`, in radians.
            #[inline(always)]
            pub fn atan2(self, other: $float) -> $float {
                self.0.atan2(other)
            }

            /// Returns the hyperbolic sine.
            #[inline(always)]
            pub fn sinh(self) -> $float {
                self.0.sinh()
            }

            /// Returns the hyperbolic cosine.
            #[inline(always)]
            pub fn cosh(self) -> $float {
                self.0.cosh()
            }

            /// Returns the hyperbolic tangent.
            #[inline(always)]
            pub fn tanh(self) -> Self {
                Self::from_inner(self.0.tanh())
            }

            /// Returns the inverse hyperbolic sine.
            #[inline(always)]
            pub fn asinh(self) -> Self {
                Self::from_inner(self.0.asinh())
            }

            /// Returns the inverse hyperbolic cosine.
            #[inline(always)]
            pub fn acosh(self) -> $float {
                self.0.acosh()
            }

            /// Returns the inverse hyperbolic tangent.
            #[inline(always)]
            pub fn atanh(self) -> $float {
                self.0.atanh()
            }

            /// Calculates the length of the hypotenuse of a right-angle triangle.
            #[inline(always)]
            pub fn hypot(self, other: $float) -> $float {
                self.0.hypot(other)
            }
        }
    };
}

impl_unit_interval_float!(f32);
impl_unit_interval_float!(f64);

/// Converts a `UnitInterval<f32>` into its inner value widened to `f64`.
impl From<UnitInterval<f32>> for f64 {
    #[inline]
    fn from(u: UnitInterval) -> Self {
        u.0 as f64
    }
}

/// Converts a `UnitInterval<f32>` into `UnitInterval<f64>`.
impl From<UnitInterval<f32>> for UnitInterval<f64> {
    #[inline]
    fn from(u: UnitInterval<f32>) -> Self {
        Self::from_inner(u.0 as f64)
    }
}

/// Converts a `UnitInterval<f64>` into `UnitInterval<f32>`.
impl From<UnitInterval<f64>> for UnitInterval<f32> {
    #[inline]
    fn from(u: UnitInterval<f64>) -> Self {
        Self::from_inner(u.0 as f32)
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

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_inner(self.0 * rhs.0)
    }
}

mod private {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::UnitInterval;
    use std::string::ToString;

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
    #[should_panic(expected = "UnitInterval invariant violated")]
    fn test_configuration_enables_internal_assertions() {
        UnitInterval::<f32>::from_inner(1.1);
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

    #[cfg(feature = "unsafe")]
    #[test]
    fn unsafe_feature_exposes_unchecked_construction_and_arithmetic() {
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

    #[test]
    fn unconstrained_arithmetic_returns_backing_float() {
        let low = UnitInterval::<f32>::new(0.25).unwrap();
        let high = UnitInterval::<f32>::new(0.75).unwrap();

        let sum: f32 = low + high;
        let difference: f32 = low - high;
        let product: UnitInterval<f32> = low * high;
        let scaled: f32 = low * 4.0;
        let reverse_scaled: f32 = 4.0 * low;
        let quotient: f32 = high / low;
        let remainder: f32 = high % low;
        let negated: f32 = -low;

        assert_eq!(sum, 1.0);
        assert_eq!(difference, -0.5);
        assert_eq!(product.get(), 0.1875);
        assert_eq!(scaled, 1.0);
        assert_eq!(reverse_scaled, 1.0);
        assert_eq!(quotient, 3.0);
        assert_eq!(remainder, 0.0);
        assert_eq!(negated, -0.25);
    }

    #[test]
    fn comparisons_work_against_backing_float() {
        let half = UnitInterval::<f32>::HALF;

        assert_eq!(half, 0.5);
        assert_eq!(0.5, half);
        assert!(half < 0.75);
        assert!(0.25 < half);
        assert!(half >= 0.5);
        assert!(0.5 <= half);
    }

    #[test]
    fn math_methods_return_unit_interval_when_result_is_constrained() {
        let half = UnitInterval::<f64>::HALF;

        let floor: UnitInterval<f64> = half.floor();
        let ceil: UnitInterval<f64> = half.ceil();
        let round: UnitInterval<f64> = half.round();
        let trunc: UnitInterval<f64> = half.trunc();
        let fract: UnitInterval<f64> = half.fract();
        let abs: UnitInterval<f64> = half.abs();
        let sqrt: UnitInterval<f64> = half.sqrt();
        let cbrt: UnitInterval<f64> = half.cbrt();
        let atan: UnitInterval<f64> = half.atan();
        let tanh: UnitInterval<f64> = half.tanh();
        let asinh: UnitInterval<f64> = half.asinh();

        assert_eq!(floor.get(), 0.5_f64.floor());
        assert_eq!(ceil.get(), 0.5_f64.ceil());
        assert_eq!(round.get(), 0.5_f64.round());
        assert_eq!(trunc.get(), 0.5_f64.trunc());
        assert_eq!(fract.get(), 0.5_f64.fract());
        assert_eq!(abs.get(), 0.5_f64.abs());
        assert_eq!(sqrt.get(), 0.5_f64.sqrt());
        assert_eq!(cbrt.get(), 0.5_f64.cbrt());
        assert_eq!(atan.get(), 0.5_f64.atan());
        assert_eq!(tanh.get(), 0.5_f64.tanh());
        assert_eq!(asinh.get(), 0.5_f64.asinh());
    }

    #[test]
    fn unconstrained_math_methods_return_backing_float() {
        let half = UnitInterval::<f64>::HALF;

        let pow: f64 = half.powi(2);
        let sine: f64 = half.sin();
        let hypot: f64 = half.hypot(0.5);
        let (sin, cos): (f64, f64) = half.sin_cos();

        assert_eq!(pow, 0.25);
        assert_eq!(sine, 0.5_f64.sin());
        assert_eq!(hypot, 0.5_f64.hypot(0.5));
        assert_eq!((sin, cos), 0.5_f64.sin_cos());
    }
}
