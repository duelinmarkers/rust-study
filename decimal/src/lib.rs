use std::ops;
use std::cmp::Ordering;
use std::fmt;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal {
    unscaled: i64,
    scale: u32
}

impl Decimal {
    fn new(unscaled: i64, scale: u32) -> Decimal {
        Decimal { unscaled: unscaled, scale: scale }
    }

    pub fn set_scale(&self, scale: u32) -> Decimal {
        match self.scale.cmp(&scale) {
            Ordering::Equal => self.clone(),
            Ordering::Greater => Decimal::new(downscale(&self.unscaled, self.scale - scale), scale),
            Ordering::Less => Decimal::new(upscale(&self.unscaled, scale - self.scale), scale)
        }
    }
}

/// `Decimal` is only `PartialOrd`, not `Ord`, because its ordering is not antisymmetric,
/// i.e., two decimals may compare `Ordering::Equal` but not be `==` to one another.
/// However note that all `Decimal`s are comparable, so `partial_cmp` will never return
/// `None`.
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        Some(match self.scale.cmp(&other.scale) {
            Ordering::Equal => self.unscaled.cmp(&other.unscaled),
            Ordering::Greater => self.unscaled.cmp(&upscale(&other.unscaled, self.scale - other.scale)),
            Ordering::Less => upscale(&self.unscaled, other.scale - self.scale).cmp(&other.unscaled)
        })
    }
}

impl str::FromStr for Decimal {
    type Err = ParseDecimalError;
    fn from_str(s: &str) -> Result<Decimal, ParseDecimalError> {
        use DecimalErrorKind::*;
        let mut unscaled = 0i64;
        let mut scale = 0u32;
        let mut index = 0u32;
        let mut negative = false;
        let mut seen_decimal = false;
        for c in s.chars() {
            match c {
                '-' if index == 0 => negative = true,
                '.' => seen_decimal = true,
                c if c.is_digit(10) => {
                    unscaled = (unscaled * 10) + c.to_digit(10).unwrap() as i64;
                    if seen_decimal {
                        scale += 1;
                    }
                },
                _ => return Err(ParseDecimalError { kind: InvalidDigit })
            }
            index += 1;
        }
        if index == 0 {
            Err(ParseDecimalError { kind: Empty })
        } else {
            Ok(Decimal::new(if negative { -unscaled } else { unscaled }, scale))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseDecimalError { kind: DecimalErrorKind }

impl ParseDecimalError {
    fn __description(&self) -> &str {
        match self.kind {
            DecimalErrorKind::Empty => "cannot parse decimal from empty string",
            DecimalErrorKind::InvalidDigit => "invalid character found in string"
        }
    }
}

impl fmt::Display for ParseDecimalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl std::error::Error for ParseDecimalError {
    fn description(&self) -> &str {
        self.__description()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DecimalErrorKind {
    Empty,
    InvalidDigit,
}

impl fmt::Display for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:.*}", self.scale as usize,
               (self.unscaled as f64) / (10i64.pow(self.scale) as f64))
    }
}

impl ops::Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => Decimal::new(self.unscaled + other.unscaled, self.scale),
            Ordering::Less => self.set_scale(other.scale) + other,
            Ordering::Greater => self + other.set_scale(self.scale)
        }
    }
}

impl ops::Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => Decimal::new(self.unscaled - other.unscaled, self.scale),
            Ordering::Less => self.set_scale(other.scale) - other,
            Ordering::Greater => self - other.set_scale(self.scale)
        }
    }
}

impl ops::Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        Decimal::new(self.unscaled * other.unscaled, self.scale + other.scale)
    }
}

impl ops::Mul<i64> for Decimal {
    type Output = Decimal;
    fn mul(self, i: i64) -> Decimal {
        Decimal::new(self.unscaled * i, self.scale)
    }
}

impl ops::Mul<Decimal> for i64 {
    type Output = Decimal;
    fn mul(self, d: Decimal) -> Decimal {
        Decimal::new(self * d.unscaled, d.scale)
    }
}

impl ops::Div for Decimal {
    type Output = Decimal;
    fn div(self, other: Decimal) -> Decimal {
        let s = if other.scale > self.scale {
            self.set_scale(other.scale)
        } else {
            self
        };
        Decimal::new(s.unscaled / other.unscaled, s.scale - other.scale)
    }
}

impl ops::Rem for Decimal {
    type Output = Decimal;
    fn rem(self, other: Decimal) -> Decimal {
        let s = if other.scale > self.scale {
            self.set_scale(other.scale)
        } else {
            self
        };
        Decimal::new(s.unscaled % other.unscaled, s.scale - other.scale)
    }
}

fn downscale(n: &i64, down_by: u32) -> i64 {
    let mut result = n.clone();
    for _ in 0..down_by {
        result = result / 10;
    }
    result
}

fn upscale(n: &i64, up_by: u32) -> i64 {
    let mut result = n.clone();
    for _ in 0..up_by {
        result = result * 10;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::Decimal;

    #[test]
    fn equality() {
        assert!(Decimal::new(1, 0) == Decimal::new(1, 0));
        assert!(Decimal::new(1, 0) != Decimal::new(1, 1));
        assert!(Decimal::new(1, 0) != Decimal::new(10, 1));
        assert!(Decimal::new(1, 0) != Decimal::new(2, 0));
    }
    #[test]
    fn numeric_comparison() {
        assert!(Decimal::new(1, 0) < Decimal::new(2, 0));
        assert!(Decimal::new(1, 0) > Decimal::new(2, 1));
        assert_eq!(::std::cmp::Ordering::Equal,
                   Decimal::new(1, 0).partial_cmp(&Decimal::new(10, 1)).unwrap());
    }
    #[test]
    fn set_scale_upwards_pads_with_zeros() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(1, 0).set_scale(2));
        assert_eq!(Decimal::new(100, 2), Decimal::new(10, 1).set_scale(2));
    }
    #[test]
    fn set_scale_downwards_truncates() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(1000, 3).set_scale(2));
        assert_eq!(Decimal::new(12, 1), Decimal::new(125, 2).set_scale(1));
    }
    #[test]
    #[should_panic(expected = "arithmetic operation overflowed")]
    fn set_scale_to_overflow_unscaled_value_panics() {
        Decimal::new(::std::i64::MAX, 3).set_scale(4);
    }
    #[test]
    fn parse_from_str() {
        assert_eq!(Ok(Decimal::new(1, 0)), ::std::str::FromStr::from_str("1"));
        assert_eq!(Ok(Decimal::new(1, 0)), "1".parse());
        assert_eq!(Ok(Decimal::new(1, 0)), "1.".parse());
        assert_eq!(Ok(Decimal::new(1, 2)), "0.01".parse());
        assert_eq!(Ok(Decimal::new(1, 2)), ".01".parse());
        assert_eq!(Ok(Decimal::new(100, 2)), "1.00".parse());
        assert_eq!(Ok(Decimal::new(23, 3)), "0.023".parse());
        assert_eq!(Ok(Decimal::new(-125, 2)), "-1.25".parse());
        assert_eq!(Ok(Decimal::new(0, 0)), "000".parse());
        assert_eq!(Ok(Decimal::new(0, 0)), "0.".parse());
        assert_eq!(Ok(Decimal::new(0, 0)), "-".parse()); // error?
        assert_eq!(Ok(Decimal::new(0, 0)), ".".parse()); // error?
        assert_eq!(Ok(Decimal::new(0, 0)), "-.".parse()); // error?
    }
    #[test]
    fn parse_failures() {
        use std::error::Error;
        assert_eq!("invalid character found in string",
                   "2g".parse::<Decimal>().err().unwrap().description());
        assert_eq!("invalid character found in string",
                   "2-2".parse::<Decimal>().err().unwrap().description());
        assert_eq!("cannot parse decimal from empty string",
                   "".parse::<Decimal>().err().unwrap().description());
    }
    #[test]
    fn adding_decimals_with_same_scale_maintains_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(51, 2) + Decimal::new(49, 2));
    }
    #[test]
    fn adding_decimals_with_different_scales_results_in_larger_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(9, 1) + Decimal::new(10, 2));
    }
    #[test]
    #[should_panic(expected = "arithmetic operation overflowed")]
    fn adding_to_overflow_unscaled_value_panics() {
        Decimal::new(::std::i64::MAX, 0) + Decimal::new(1, 0);
    }
    #[test]
    fn subtracting_decimals_with_different_scales_results_in_larger_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(11, 1) - Decimal::new(10, 2));
    }
    #[test]
    #[should_panic(expected = "arithmetic operation overflowed")]
    fn subtracting_to_overflow_unscaled_value_panics() {
        Decimal::new(::std::i64::MIN, 0) - Decimal::new(1, 0);
    }
    #[test]
    fn multiplying_decimals_results_in_summed_scales() {
        assert_eq!(Decimal::new(1500, 1), Decimal::new(100, 0) * Decimal::new(15, 1));
        assert_eq!(Decimal::new(2500, 2), Decimal::new(100, 0) * Decimal::new(25, 2));
        assert_eq!(Decimal::new(49995, 5), Decimal::new(15, 1) * Decimal::new(3333, 4));
    }
    #[test]
    fn multiplying_decimal_by_int_is_commutative() {
        assert_eq!(Decimal::new(246, 2), Decimal::new(123, 2) * 2);
        assert_eq!(Decimal::new(246, 2), 2 * Decimal::new(123, 2));
    }
    #[test]
    #[should_panic(expected = "arithmetic operation overflowed")]
    fn multiplying_to_overflow_unscaled_value_panics() {
        Decimal::new(::std::i64::MAX, 2) * Decimal::new(10001, 4);
    }
    #[test]
    #[should_panic(expected = "arithmetic operation overflowed")]
    fn multiplying_to_overflow_scale_panics() {
        Decimal::new(1, ::std::u32::MAX) * Decimal::new(1, 1);
    }
    #[test]
    fn dividing_decimal_by_decimal() {
        assert_eq!(Decimal::new(137, 1), Decimal::new(685, 2) / Decimal::new(5, 1));
        assert_eq!(Decimal::new(30, 0), Decimal::new(75, 1) / Decimal::new(25, 2));
    }
    #[test]
    fn dividing_decimals_truncates_remainder() {
        assert_eq!(Decimal::new(2, 0), Decimal::new(5, 0) / Decimal::new(2, 0));
        assert_eq!(Decimal::new(212, 2), Decimal::new(425, 2) / Decimal::new(2, 0));
    }
    #[test]
    fn get_remainder() {
        assert_eq!(Decimal::new(1, 0), Decimal::new(5, 0) % Decimal::new(2, 0));
        assert_eq!(Decimal::new(1, 2), Decimal::new(425, 2) % Decimal::new(2, 0));
    }
    #[test]
    fn ops_on_negative_decimals() {
        assert_eq!(Decimal::new(10, 1), Decimal::new(12, 1) + Decimal::new(-2, 1));
        assert_eq!(Decimal::new(-1, 3), Decimal::new(0, 0) - Decimal::new(1, 3));
        assert_eq!(Decimal::new(1, 0), Decimal::new(-1, 0) * Decimal::new(-1, 0));
        assert_eq!(Decimal::new(-3, 0), Decimal::new(3, 0) / Decimal::new(-1, 0));
    }
    #[test]
    fn performing_ops_on_decimals_does_not_preclude_further_use() {
        let fifty_cents = Decimal::new(5, 1).set_scale(2);
        let buck_fifty = Decimal::new(150, 2);
        let two_dollars = Decimal::new(200, 2);
        assert_eq!(buck_fifty + fifty_cents,
                   two_dollars);
        assert_eq!(fifty_cents + fifty_cents + fifty_cents,
                   buck_fifty);
        assert_eq!(two_dollars - fifty_cents,
                   buck_fifty);
    }
    #[test]
    fn displays_with_decimal_point_and_optional_negative_sign() {
        assert_eq!("1.50", format!("{}", Decimal::new(150, 2)));
        assert_eq!("0.0010", format!("{}", Decimal::new(10, 4)));
        assert_eq!("-0.1", format!("{}", Decimal::new(-1, 1)));
    }
    #[test]
    fn supports_debug_format() {
        assert_eq!("Decimal { unscaled: 1, scale: 2 }",
                   format!("{:?}", Decimal::new(1, 2)));
    }
}
