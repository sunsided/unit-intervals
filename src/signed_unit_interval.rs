use core::{
    cmp::Ordering,
    error::Error,
    fmt,
    ops::{Add, Deref, Div, Mul, Neg, Rem, Sub},
};

use crate::{UnitInterval, UnitIntervalError, UnitIntervalFloat};

/// A floating-point value constrained to the closed signed unit interval `[-1, 1]`.
///
/// `SignedUnitInterval` is useful for normalized signed values such as direction,
/// balance, centered offsets, joystick axes, and correlation-like coefficients.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SignedUnitInterval<T = f32>(T);

/// Error returned when converting an out-of-range value into a [`SignedUnitInterval`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SignedUnitIntervalError;

impl fmt::Display for SignedUnitIntervalError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("value is outside the signed unit interval")
    }
}

impl Error for SignedUnitIntervalError {}

impl<T: UnitIntervalFloat> SignedUnitInterval<T> {
    /// The lower bound of the signed unit interval.
    pub const NEG_ONE: Self = Self(T::NEG_ONE);

    /// The midpoint of the signed unit interval.
    pub const ZERO: Self = Self(T::ZERO);

    /// The upper bound of the signed unit interval.
    pub const ONE: Self = Self(T::ONE);

    /// The positive midpoint of the signed unit interval.
    pub const HALF: Self = Self(T::HALF);

    /// Creates a value if `v` is inside `[-1, 1]`.
    ///
    /// Returns `None` for values outside the interval and for `NaN`.
    #[inline]
    pub fn new(v: T) -> Option<Self> {
        if Self::contains(v) {
            Some(Self::from_inner(v))
        } else {
            None
        }
    }

    /// Returns whether `v` is inside `[-1, 1]`.
    ///
    /// `NaN` is not contained in the interval.
    #[inline]
    pub fn contains(v: T) -> bool {
        v >= T::NEG_ONE && v <= T::ONE
    }

    /// Creates a value by clamping `v` into `[-1, 1]`.
    ///
    /// `NaN` is treated as zero.
    #[inline]
    pub fn saturating(v: T) -> Self {
        Self::from_inner(v.clamp_signed_unit())
    }

    #[inline]
    pub(crate) fn from_inner(v: T) -> Self {
        Self::assert_contains(v);
        Self(v)
    }

    #[cfg(any(test, feature = "assertions"))]
    #[inline]
    fn assert_contains(v: T) {
        assert!(
            Self::contains(v),
            "SignedUnitInterval invariant violated: value is outside [-1, 1]"
        );
    }

    #[cfg(not(any(test, feature = "assertions")))]
    #[inline]
    fn assert_contains(_v: T) {}

    /// Returns the inner floating-point value.
    #[inline]
    pub const fn get(self) -> T {
        self.0
    }

    /// Consumes the wrapper and returns the inner floating-point value.
    #[inline]
    pub const fn into_inner(self) -> T {
        self.0
    }

    /// Returns whether this value is exactly zero.
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == T::ZERO
    }

    /// Returns whether this value is exactly one.
    #[inline]
    pub fn is_one(self) -> bool {
        self.0 == T::ONE
    }

    /// Returns whether this value is exactly negative one.
    #[inline]
    pub fn is_neg_one(self) -> bool {
        self.0 == T::NEG_ONE
    }

    /// Returns `1 - self`.
    #[inline]
    pub fn complement(self) -> T {
        T::ONE - self.0
    }

    /// Returns the smaller of two signed unit interval values.
    #[inline]
    pub fn min<R: Into<Self>>(self, rhs: R) -> Self {
        let rhs = rhs.into();

        if self.0 <= rhs.0 { self } else { rhs }
    }

    /// Returns the larger of two signed unit interval values.
    #[inline]
    pub fn max<R: Into<Self>>(self, rhs: R) -> Self {
        let rhs = rhs.into();

        if self.0 >= rhs.0 { self } else { rhs }
    }

    /// Returns the midpoint between two signed unit interval values.
    #[inline]
    pub fn midpoint<R: Into<Self>>(self, rhs: R) -> Self {
        let rhs = rhs.into();

        Self::from_inner((self.0 + rhs.0) * T::HALF)
    }

    /// Returns the absolute distance between two signed unit interval values.
    #[inline]
    pub fn distance_to<R: Into<Self>>(self, rhs: R) -> T {
        let rhs = rhs.into();

        if self.0 >= rhs.0 {
            self.0 - rhs.0
        } else {
            rhs.0 - self.0
        }
    }

    /// Adds two values, returning `None` if the result is outside `[-1, 1]`.
    #[inline]
    pub fn checked_add<R: Into<Self>>(self, rhs: R) -> Option<Self> {
        Self::new(self.0 + rhs.into().0)
    }

    /// Adds two values and clamps the result into `[-1, 1]`.
    #[inline]
    pub fn saturating_add<R: Into<Self>>(self, rhs: R) -> Self {
        Self::saturating(self.0 + rhs.into().0)
    }

    /// Subtracts `rhs`, returning `None` if the result is outside `[-1, 1]`.
    #[inline]
    pub fn checked_sub<R: Into<Self>>(self, rhs: R) -> Option<Self> {
        Self::new(self.0 - rhs.into().0)
    }

    /// Subtracts `rhs` and clamps the result into `[-1, 1]`.
    #[inline]
    pub fn saturating_sub<R: Into<Self>>(self, rhs: R) -> Self {
        Self::saturating(self.0 - rhs.into().0)
    }

    /// Divides by `rhs`, returning `None` if the result is outside `[-1, 1]`.
    #[inline]
    pub fn checked_div<R: Into<Self>>(self, rhs: R) -> Option<Self> {
        Self::new(self.0 / rhs.into().0)
    }

    /// Divides by `rhs` and clamps the result into `[-1, 1]`.
    #[inline]
    pub fn saturating_div<R: Into<Self>>(self, rhs: R) -> Self {
        Self::saturating(self.0 / rhs.into().0)
    }

    /// Multiplies by an arbitrary float, returning `None` if the result is outside `[-1, 1]`.
    #[inline]
    pub fn checked_scale(self, factor: T) -> Option<Self> {
        Self::new(self.0 * factor)
    }

    /// Multiplies by an arbitrary float and clamps the result into `[-1, 1]`.
    #[inline]
    pub fn saturating_scale(self, factor: T) -> Self {
        Self::saturating(self.0 * factor)
    }

    /// Linearly interpolates between `start` and `end`.
    #[inline]
    pub fn lerp(self, start: T, end: T) -> T {
        start + (end - start) * self.0
    }
}

/// Returns [`SignedUnitInterval::ZERO`].
impl<T: UnitIntervalFloat> Default for SignedUnitInterval<T> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

/// Dereferences to the inner floating-point value.
impl<T> Deref for SignedUnitInterval<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Borrows the inner floating-point value.
impl<T> AsRef<T> for SignedUnitInterval<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: UnitIntervalFloat> From<UnitInterval<T>> for SignedUnitInterval<T> {
    #[inline]
    fn from(u: UnitInterval<T>) -> Self {
        Self::from_inner(u.get())
    }
}

impl<T: UnitIntervalFloat> TryFrom<SignedUnitInterval<T>> for UnitInterval<T> {
    type Error = UnitIntervalError;

    #[inline]
    fn try_from(value: SignedUnitInterval<T>) -> Result<Self, Self::Error> {
        Self::new(value.0).ok_or(UnitIntervalError)
    }
}

macro_rules! impl_signed_unit_interval_float {
    ($float:ty) => {
        impl From<SignedUnitInterval<$float>> for $float {
            #[inline]
            fn from(u: SignedUnitInterval<$float>) -> Self {
                u.0
            }
        }

        impl TryFrom<$float> for SignedUnitInterval<$float> {
            type Error = SignedUnitIntervalError;

            #[inline]
            fn try_from(value: $float) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(SignedUnitIntervalError)
            }
        }

        impl PartialEq<$float> for SignedUnitInterval<$float> {
            #[inline]
            fn eq(&self, other: &$float) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<SignedUnitInterval<$float>> for $float {
            #[inline]
            fn eq(&self, other: &SignedUnitInterval<$float>) -> bool {
                *self == other.0
            }
        }

        impl PartialOrd<$float> for SignedUnitInterval<$float> {
            #[inline]
            fn partial_cmp(&self, other: &$float) -> Option<Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl PartialOrd<SignedUnitInterval<$float>> for $float {
            #[inline]
            fn partial_cmp(&self, other: &SignedUnitInterval<$float>) -> Option<Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl Add for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                self.0 + rhs.0
            }
        }

        impl Add<UnitInterval<$float>> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn add(self, rhs: UnitInterval<$float>) -> Self::Output {
                self.0 + rhs.get()
            }
        }

        impl Add<SignedUnitInterval<$float>> for UnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn add(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self.get() + rhs.0
            }
        }

        impl Add<$float> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn add(self, rhs: $float) -> Self::Output {
                self.0 + rhs
            }
        }

        impl Add<SignedUnitInterval<$float>> for $float {
            type Output = $float;

            #[inline]
            fn add(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self + rhs.0
            }
        }

        impl Sub for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                self.0 - rhs.0
            }
        }

        impl Sub<UnitInterval<$float>> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn sub(self, rhs: UnitInterval<$float>) -> Self::Output {
                self.0 - rhs.get()
            }
        }

        impl Sub<SignedUnitInterval<$float>> for UnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn sub(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self.get() - rhs.0
            }
        }

        impl Sub<$float> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn sub(self, rhs: $float) -> Self::Output {
                self.0 - rhs
            }
        }

        impl Sub<SignedUnitInterval<$float>> for $float {
            type Output = $float;

            #[inline]
            fn sub(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self - rhs.0
            }
        }

        impl Mul<$float> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn mul(self, rhs: $float) -> Self::Output {
                self.0 * rhs
            }
        }

        impl Mul<SignedUnitInterval<$float>> for $float {
            type Output = $float;

            #[inline]
            fn mul(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self * rhs.0
            }
        }

        impl Div for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Div<UnitInterval<$float>> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn div(self, rhs: UnitInterval<$float>) -> Self::Output {
                self.0 / rhs.get()
            }
        }

        impl Div<SignedUnitInterval<$float>> for UnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn div(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self.get() / rhs.0
            }
        }

        impl Div<$float> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn div(self, rhs: $float) -> Self::Output {
                self.0 / rhs
            }
        }

        impl Div<SignedUnitInterval<$float>> for $float {
            type Output = $float;

            #[inline]
            fn div(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self / rhs.0
            }
        }

        impl Rem for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn rem(self, rhs: Self) -> Self::Output {
                self.0 % rhs.0
            }
        }

        impl Rem<UnitInterval<$float>> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn rem(self, rhs: UnitInterval<$float>) -> Self::Output {
                self.0 % rhs.get()
            }
        }

        impl Rem<SignedUnitInterval<$float>> for UnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn rem(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self.get() % rhs.0
            }
        }

        impl Rem<$float> for SignedUnitInterval<$float> {
            type Output = $float;

            #[inline]
            fn rem(self, rhs: $float) -> Self::Output {
                self.0 % rhs
            }
        }

        impl Rem<SignedUnitInterval<$float>> for $float {
            type Output = $float;

            #[inline]
            fn rem(self, rhs: SignedUnitInterval<$float>) -> Self::Output {
                self % rhs.0
            }
        }

        impl Neg for SignedUnitInterval<$float> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::from_inner(-self.0)
            }
        }

        #[cfg(any(test, feature = "std"))]
        impl SignedUnitInterval<$float> {
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
            pub fn abs(self) -> UnitInterval<$float> {
                UnitInterval::new(self.0.abs()).expect("absolute signed unit value is in [0, 1]")
            }

            /// Returns a number representing the sign of this value.
            #[inline]
            pub fn signum(self) -> Self {
                Self::from_inner(self.0.signum())
            }

            /// Returns this value with the sign of `sign`.
            #[inline]
            pub fn copysign(self, sign: $float) -> Self {
                Self::from_inner(self.0.copysign(sign))
            }

            /// Returns `true` if this value is positive zero or positive.
            #[inline]
            pub fn is_sign_positive(self) -> bool {
                self.0.is_sign_positive()
            }

            /// Returns `true` if this value is negative zero or negative.
            #[inline]
            pub fn is_sign_negative(self) -> bool {
                self.0.is_sign_negative()
            }

            /// Returns `true`; signed unit interval values are always finite.
            #[inline]
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }

            /// Returns `false`; signed unit interval values cannot be infinite.
            #[inline]
            pub fn is_infinite(self) -> bool {
                self.0.is_infinite()
            }

            /// Returns `false`; signed unit interval values cannot be `NaN`.
            #[inline]
            pub fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            /// Takes the reciprocal, `1 / self`.
            #[inline]
            pub fn recip(self) -> $float {
                self.0.recip()
            }

            /// Raises this value to an integer power.
            #[inline]
            pub fn powi(self, n: i32) -> $float {
                self.0.powi(n)
            }

            /// Raises this value to a floating-point power.
            #[inline]
            pub fn powf(self, n: $float) -> $float {
                self.0.powf(n)
            }

            /// Returns the square root.
            #[inline]
            pub fn sqrt(self) -> $float {
                self.0.sqrt()
            }

            /// Returns the cube root.
            #[inline]
            pub fn cbrt(self) -> Self {
                Self::from_inner(self.0.cbrt())
            }

            /// Computes `self * a + b` with one rounding error.
            #[inline]
            pub fn mul_add(self, a: $float, b: $float) -> $float {
                self.0.mul_add(a, b)
            }

            /// Returns the Euclidean division of this value by `rhs`.
            #[inline]
            pub fn div_euclid(self, rhs: $float) -> $float {
                self.0.div_euclid(rhs)
            }

            /// Returns the least non-negative remainder of this value divided by `rhs`.
            #[inline]
            pub fn rem_euclid(self, rhs: $float) -> $float {
                self.0.rem_euclid(rhs)
            }

            /// Returns `e^(self)`.
            #[inline]
            pub fn exp(self) -> $float {
                self.0.exp()
            }

            /// Returns `2^(self)`.
            #[inline]
            pub fn exp2(self) -> $float {
                self.0.exp2()
            }

            /// Returns the natural logarithm.
            #[inline]
            pub fn ln(self) -> $float {
                self.0.ln()
            }

            /// Returns the logarithm with respect to an arbitrary base.
            #[inline]
            pub fn log(self, base: $float) -> $float {
                self.0.log(base)
            }

            /// Returns the base 2 logarithm.
            #[inline]
            pub fn log2(self) -> $float {
                self.0.log2()
            }

            /// Returns the base 10 logarithm.
            #[inline]
            pub fn log10(self) -> $float {
                self.0.log10()
            }

            /// Returns the sine, in radians.
            #[inline]
            pub fn sin(self) -> Self {
                Self::from_inner(self.0.sin())
            }

            /// Returns the cosine, in radians.
            #[inline]
            pub fn cos(self) -> UnitInterval<$float> {
                UnitInterval::new(self.0.cos()).expect("cosine on [-1, 1] is in [0, 1]")
            }

            /// Returns the tangent, in radians.
            #[inline]
            pub fn tan(self) -> $float {
                self.0.tan()
            }

            /// Returns both sine and cosine, in radians.
            #[inline]
            pub fn sin_cos(self) -> (Self, UnitInterval<$float>) {
                let (sin, cos) = self.0.sin_cos();
                (
                    Self::from_inner(sin),
                    UnitInterval::new(cos).expect("cosine on [-1, 1] is in [0, 1]"),
                )
            }

            /// Returns the arcsine, in radians.
            #[inline]
            pub fn asin(self) -> $float {
                self.0.asin()
            }

            /// Returns the arccosine, in radians.
            #[inline]
            pub fn acos(self) -> $float {
                self.0.acos()
            }

            /// Returns the arctangent, in radians.
            #[inline]
            pub fn atan(self) -> Self {
                Self::from_inner(self.0.atan())
            }

            /// Returns the four-quadrant arctangent of `self` and `other`, in radians.
            #[inline]
            pub fn atan2(self, other: $float) -> $float {
                self.0.atan2(other)
            }

            /// Returns the hyperbolic sine.
            #[inline]
            pub fn sinh(self) -> $float {
                self.0.sinh()
            }

            /// Returns the hyperbolic cosine.
            #[inline]
            pub fn cosh(self) -> $float {
                self.0.cosh()
            }

            /// Returns the hyperbolic tangent.
            #[inline]
            pub fn tanh(self) -> Self {
                Self::from_inner(self.0.tanh())
            }

            /// Returns the inverse hyperbolic sine.
            #[inline]
            pub fn asinh(self) -> Self {
                Self::from_inner(self.0.asinh())
            }

            /// Returns the inverse hyperbolic cosine.
            #[inline]
            pub fn acosh(self) -> $float {
                self.0.acosh()
            }

            /// Returns the inverse hyperbolic tangent.
            #[inline]
            pub fn atanh(self) -> $float {
                self.0.atanh()
            }

            /// Calculates the length of the hypotenuse of a right-angle triangle.
            #[inline]
            pub fn hypot(self, other: $float) -> $float {
                self.0.hypot(other)
            }
        }
    };
}

impl_signed_unit_interval_float!(f32);
impl_signed_unit_interval_float!(f64);

/// Converts a `SignedUnitInterval<f32>` into its inner value widened to `f64`.
impl From<SignedUnitInterval<f32>> for f64 {
    #[inline]
    fn from(u: SignedUnitInterval) -> Self {
        u.0 as f64
    }
}

/// Converts a `SignedUnitInterval<f32>` into `SignedUnitInterval<f64>`.
impl From<SignedUnitInterval<f32>> for SignedUnitInterval<f64> {
    #[inline]
    fn from(u: SignedUnitInterval<f32>) -> Self {
        Self::from_inner(u.0 as f64)
    }
}

/// Converts a `SignedUnitInterval<f64>` into `SignedUnitInterval<f32>`.
impl From<SignedUnitInterval<f64>> for SignedUnitInterval<f32> {
    #[inline]
    fn from(u: SignedUnitInterval<f64>) -> Self {
        Self::from_inner(u.0 as f32)
    }
}

/// Multiplies two signed unit interval values.
impl<T: UnitIntervalFloat> Mul for SignedUnitInterval<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_inner(self.0 * rhs.0)
    }
}

/// Multiplies a signed unit interval by a unit interval.
impl<T: UnitIntervalFloat> Mul<UnitInterval<T>> for SignedUnitInterval<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: UnitInterval<T>) -> Self::Output {
        Self::from_inner(self.0 * rhs.get())
    }
}

/// Multiplies a unit interval by a signed unit interval.
impl<T: UnitIntervalFloat> Mul<SignedUnitInterval<T>> for UnitInterval<T> {
    type Output = SignedUnitInterval<T>;

    #[inline]
    fn mul(self, rhs: SignedUnitInterval<T>) -> Self::Output {
        SignedUnitInterval::from_inner(self.get() * rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::SignedUnitInterval;
    use crate::UnitInterval;

    #[test]
    fn constructors_accept_signed_unit_interval() {
        let default_width: SignedUnitInterval = SignedUnitInterval::new(-0.5).unwrap();

        assert_eq!(default_width.get(), -0.5);
        assert_eq!(
            SignedUnitInterval::<f32>::new(-1.0).map(|u| u.get()),
            Some(-1.0)
        );
        assert_eq!(
            SignedUnitInterval::<f32>::new(0.0).map(|u| u.get()),
            Some(0.0)
        );
        assert_eq!(
            SignedUnitInterval::<f32>::new(1.0).map(|u| u.get()),
            Some(1.0)
        );
        assert_eq!(SignedUnitInterval::<f32>::new(-1.1), None);
        assert_eq!(SignedUnitInterval::<f32>::new(1.1), None);
        assert_eq!(SignedUnitInterval::<f32>::new(f32::NAN), None);
    }

    #[test]
    #[should_panic(expected = "SignedUnitInterval invariant violated")]
    fn test_configuration_enables_internal_assertions() {
        SignedUnitInterval::<f32>::from_inner(1.1);
    }

    #[test]
    fn constants_conversions_and_helpers_work() {
        let unit = UnitInterval::new(0.25).unwrap();
        let signed = SignedUnitInterval::from(unit);
        let back_to_unit = UnitInterval::try_from(signed).unwrap();
        let negative = SignedUnitInterval::new(-0.25).unwrap();

        assert_eq!(
            SignedUnitInterval::<f32>::default(),
            SignedUnitInterval::ZERO
        );
        assert!(SignedUnitInterval::<f32>::NEG_ONE.is_neg_one());
        assert!(SignedUnitInterval::<f32>::ZERO.is_zero());
        assert!(SignedUnitInterval::<f32>::ONE.is_one());
        assert_eq!(SignedUnitInterval::<f32>::saturating(-1.25).get(), -1.0);
        assert_eq!(SignedUnitInterval::<f32>::saturating(1.25).get(), 1.0);
        assert_eq!(SignedUnitInterval::<f32>::saturating(f32::NAN).get(), 0.0);
        assert_eq!(signed.get(), 0.25);
        assert_eq!(back_to_unit, unit);
        assert!(UnitInterval::try_from(negative).is_err());
    }

    #[test]
    fn checked_and_saturating_arithmetic_accept_unit_interval() {
        let negative = SignedUnitInterval::new(-0.75).unwrap();
        let positive = SignedUnitInterval::new(0.75).unwrap();
        let unit = UnitInterval::new(0.5).unwrap();

        assert_eq!(negative.min(unit), negative);
        assert_eq!(negative.max(unit).get(), 0.5);
        assert_eq!(negative.midpoint(unit).get(), -0.125);
        assert_eq!(negative.distance_to(unit), 1.25);
        assert_eq!(negative.checked_add(unit).unwrap().get(), -0.25);
        assert_eq!(positive.checked_add(unit), None);
        assert_eq!(positive.saturating_add(unit).get(), 1.0);
        assert_eq!(negative.checked_sub(unit), None);
        assert_eq!(negative.saturating_sub(unit).get(), -1.0);
        assert_eq!(positive.checked_div(unit), None);
        assert_eq!(positive.saturating_div(unit).get(), 1.0);
    }

    #[test]
    fn constrained_results_return_constrained_types() {
        let negative = SignedUnitInterval::new(-0.5).unwrap();
        let positive = SignedUnitInterval::new(0.5).unwrap();
        let unit = UnitInterval::new(0.5).unwrap();

        let negated: SignedUnitInterval = -negative;
        let signed_product: SignedUnitInterval = negative * positive;
        let mixed_product: SignedUnitInterval = negative * unit;
        let reverse_mixed_product: SignedUnitInterval = unit * negative;
        let absolute: UnitInterval = negative.abs();
        let cosine: UnitInterval = negative.cos();
        let (sine, sin_cos_cosine): (SignedUnitInterval, UnitInterval) = negative.sin_cos();

        assert_eq!(negated.get(), 0.5);
        assert_eq!(signed_product.get(), -0.25);
        assert_eq!(mixed_product.get(), -0.25);
        assert_eq!(reverse_mixed_product.get(), -0.25);
        assert_eq!(absolute.get(), 0.5);
        assert_eq!(cosine.get(), (-0.5_f32).cos());
        assert_eq!(sine.get(), (-0.5_f32).sin());
        assert_eq!(sin_cos_cosine.get(), (-0.5_f32).cos());
    }

    #[test]
    fn unconstrained_operations_return_backing_float() {
        let negative = SignedUnitInterval::<f32>::new(-0.75).unwrap();
        let positive = SignedUnitInterval::<f32>::new(0.75).unwrap();
        let unit = UnitInterval::new(0.5).unwrap();

        let sum: f32 = positive + positive;
        let mixed_sum: f32 = positive + unit;
        let quotient: f32 = positive / negative;
        let distance: f32 = negative.distance_to(positive);

        assert_eq!(sum, 1.5);
        assert_eq!(mixed_sum, 1.25);
        assert_eq!(quotient, -1.0);
        assert_eq!(distance, 1.5);
    }
}
