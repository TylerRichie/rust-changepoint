use num::{One, Zero, Num, Float};
use std::cmp::{Ord, Ordering};
use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NonNaN<F: Float>(F);

impl<F: Float> NonNaN<F> {
    pub fn new(value: F) -> Option<Self> {
        if value.is_nan() || value.is_infinite() {
            None
        } else {
            Some(NonNaN(value))
        }
    }

    pub fn value(&self) -> F {
        let &NonNaN(value) = self;
        value
    }
}

impl<F: Float> Eq for NonNaN<F> {}

impl<F: Float> Ord for NonNaN<F> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect(
            "Impossible to create a NaN value for a NonNaN float, so this always is Some.",
        )
    }
}

fn clip_to_finite<F: Float>(raw_result: F) -> NonNaN<F> {
    NonNaN::new(if raw_result.is_infinite() {
        if raw_result.is_sign_positive() {
            F::max_value()
        } else {
            F::min_value()
        }
    } else {
        raw_result
    }).expect(
        "Clipping results to finite values ensures NaN and infinite values are impossible",
    )
}

impl<F: Float> Add for NonNaN<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let NonNaN(this) = self;
        let NonNaN(that) = other;
        let raw_result = this + that;
        clip_to_finite(raw_result)
    }
}

impl<F: Float> Sub for NonNaN<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let NonNaN(that) = other;
        let negated_other =
            NonNaN::new(-that).expect("If positive value was okay, negative value must be too.");
        self + negated_other
    }
}

impl<F: Float> Mul for NonNaN<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let NonNaN(this) = self;
        let NonNaN(that) = other;
        let raw_result = this * that;
        clip_to_finite(raw_result)
    }
}

fn as_divisor<F: Float>(candidate: F) -> F {
    if candidate == F::from(0.0).expect("0.0 is known to be a valid floating point value") {
        F::min_positive_value() *
            if candidate.is_sign_positive() {
                F::from(1.0).expect("1.0 is known to be a valid floating point value")
            } else {
                F::from(-1.0).expect("-1.0 is known to be a valid floating point value.")
            }
    } else {
        candidate
    }
}

impl<F: Float> Div for NonNaN<F> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let NonNaN(this) = self;
        let NonNaN(that) = other;
        let divisor = as_divisor(that);
        let raw_result = this / divisor;
        clip_to_finite(raw_result)
    }
}

impl<F: Float> Rem for NonNaN<F> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        let NonNaN(this) = self;
        let NonNaN(that) = other;
        let divisor = as_divisor(that);
        let raw_result = this.rem(divisor);
        clip_to_finite(raw_result)
    }
}

impl<F: Float> Zero for NonNaN<F> {
    fn zero() -> Self {
        NonNaN::new(F::zero()).expect("Zero is a legal NonNaN value.")
    }

    fn is_zero(&self) -> bool {
        let &NonNaN(value) = self;
        value.is_zero()
    }
}

impl<F: Float> One for NonNaN<F> {
    fn one() -> Self {
        NonNaN::new(F::one()).expect("One is a legal NonNaN value.")
    }
}

#[derive(Clone, Debug)]
pub enum ParseNonNaNError {
    ParseFloatError,
    NaNOrInfiniteError
}

impl<F: Float> Num for NonNaN<F> {
    type FromStrRadixErr = ParseNonNaNError;

    fn from_str_radix(
        str: &str,
        radix: u32
    ) -> Result<Self, Self::FromStrRadixErr> {
        let float_result = match F::from_str_radix(str, radix) {
            Ok(result) => result,
            Err(_) => return Err(ParseNonNaNError::ParseFloatError)
        };
        match NonNaN::new(float_result) {
            Some(result) => Ok(result),
            None => Err(ParseNonNaNError::NaNOrInfiniteError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;

    fn get_max_value<F: Float>(_: F) -> F {
        F::max_value()
    }

    #[test]
    fn negative_zero_equals_zero() {
        let small_neg: f32 = -0.0;
        let large_neg: f64 = -0.0;
        let small_pos: f32 = 0.0;
        let large_pos: f64 = 0.0;
        assert_eq!(small_neg, small_pos);
        assert_eq!(large_neg, large_pos);
    }

    #[test]
    fn finite_float_values_are_non_nan() {
        NonNaN::new(1.0).expect("NonNaN returned a None for a valid floating point number input.");
    }

    #[test]
    fn infinite_and_nan_floats_are_not_non_nan() {
        assert!(NonNaN::new(std::f32::NAN).is_none());
        assert!(NonNaN::new(std::f32::INFINITY).is_none());
        assert!(NonNaN::new(-std::f32::INFINITY).is_none());
    }

    #[test]
    fn clip_infinite_values() {
        let clipped_infinite = clip_to_finite(std::f32::INFINITY);
        let clipped_neg_infinite = clip_to_finite(-std::f32::INFINITY);
        let max_value = get_max_value(0.0);
        let min_value = -get_max_value(0.0);
        assert_eq!(clipped_infinite.value(), max_value);
        assert_eq!(clipped_neg_infinite.value(), min_value);
        let normal_value = 42.0;
        assert_eq!(clip_to_finite(normal_value).value(), normal_value);
    }

    #[test]
    fn prevent_zero_divisors() {
        let pos_zero_divisor = 0.0;
        let neg_zero_divisor = -0.0;
        assert!(as_divisor(pos_zero_divisor) > pos_zero_divisor);
        assert!(as_divisor(neg_zero_divisor) < neg_zero_divisor);
        let normal_divisor = 42.0;
        assert_eq!(as_divisor(normal_divisor), normal_divisor);
    }

    #[test]
    fn add_non_nan_floats() {
        let normal_left = NonNaN::new(3.0).unwrap();
        let normal_right = NonNaN::new(2.0).unwrap();
        let normal_result = normal_left + normal_right;
        assert_eq!(normal_result.value(), 5.0);
        let big_left = NonNaN::new(get_max_value(0.0)).unwrap();
        let big_right = NonNaN::new(get_max_value(0.0)).unwrap();
        let big_result = big_left + big_right;
        assert_eq!(big_result.value(), get_max_value(0.0));
    }

    #[test]
    fn sub_non_nan_floats() {
        let normal_left = NonNaN::new(3.0).unwrap();
        let normal_right = NonNaN::new(2.0).unwrap();
        let normal_result = normal_left - normal_right;
        assert_eq!(normal_result.value(), 1.0);
        let big_left = NonNaN::new(get_max_value(0.0)).unwrap();
        let small_right = NonNaN::new(-get_max_value(0.0)).unwrap();
        let big_result = big_left - small_right;
        assert_eq!(big_result.value(), get_max_value(0.0));
    }

    #[test]
    fn mul_non_nan_floats() {
        let normal_left = NonNaN::new(3.0).unwrap();
        let normal_right = NonNaN::new(2.0).unwrap();
        let normal_result = normal_left * normal_right;
        assert_eq!(normal_result.value(), 6.0);
        let big_left = NonNaN::new(get_max_value(0.0)).unwrap();
        let big_right = NonNaN::new(get_max_value(0.0)).unwrap();
        let big_result = big_left * big_right;
        assert_eq!(big_result.value(), get_max_value(0.0));
    }

    #[test]
    fn div_non_nan_floats() {
        let normal_left = NonNaN::new(3.0).unwrap();
        let normal_right = NonNaN::new(2.0).unwrap();
        let normal_result = normal_left / normal_right;
        assert_eq!(normal_result.value(), 1.5);
        let big_left = NonNaN::new(get_max_value(0.0)).unwrap();
        let zero_right = NonNaN::new(0.0).unwrap();
        let big_result = big_left / zero_right;
        assert_eq!(big_result.value(), get_max_value(0.0));
    }

    #[test]
    fn rem_non_nan_floats() {
        let normal_left = NonNaN::new(3.0).unwrap();
        let normal_right = NonNaN::new(2.0).unwrap();
        let normal_result = normal_left % normal_right;
        assert_eq!(normal_result.value(), 1.0);
        let big_left = NonNaN::new(get_max_value(0.0)).unwrap();
        let zero_right = NonNaN::new(0.0).unwrap();
        let zero_result = big_left % zero_right;
        assert_eq!(zero_result.value(), 0.0);
    }

}
