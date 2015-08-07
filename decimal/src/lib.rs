use std::ops;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Decimal {
    unscaled: i64,
    scale: u32
}

impl Decimal {
    fn new(unscaled: i64, scale: u32) -> Decimal {
        Decimal { unscaled: unscaled, scale: scale }
    }

    pub fn set_scale(self, scale: u32) -> Decimal {
        match self.scale.cmp(&scale) {
            Ordering::Equal => self,
            Ordering::Greater => Decimal::new(downscale(self.unscaled, self.scale - scale), scale),
            Ordering::Less => Decimal::new(upscale(self.unscaled, scale - self.scale), scale)
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:.*}", self.scale as usize,
               (self.unscaled as f64) / (10i64.pow(self.scale) as f64))
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        self.unscaled == other.unscaled && self.scale == other.scale
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

fn downscale(n: i64, down_by: u32) -> i64 {
    let mut result = n;
    for _ in 0..down_by {
        result = result / 10;
    }
    result
}

fn upscale(n: i64, up_by: u32) -> i64 {
    let mut result = n;
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
        assert!(Decimal::new(1, 0) != Decimal::new(2, 0));
        assert!(Decimal::new(1, 0) != Decimal::new(10, 1));
    }
    #[test]
    fn add_decimals_with_same_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(51, 2) + Decimal::new(49, 2));
    }
    #[test]
    fn set_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(1000, 3).set_scale(2));
        assert_eq!(Decimal::new(100, 2), Decimal::new(10, 1).set_scale(2));
        assert_eq!(Decimal::new(12, 1), Decimal::new(125, 2).set_scale(1));
    }
    #[test]
    fn adding_decimals_with_different_scales_results_in_larger_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(9, 1) + Decimal::new(10, 2));
    }
    #[test]
    fn subtracting_decimals_with_different_scales_results_in_larger_scale() {
        assert_eq!(Decimal::new(100, 2), Decimal::new(11, 1) - Decimal::new(10, 2));
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
    fn displays_with_decimal_point() {
        assert_eq!("1.50", format!("{}", Decimal::new(150, 2)));
        assert_eq!("0.0010", format!("{}", Decimal::new(10, 4)));
    }
    #[test]
    fn supports_debug_format() {
        assert_eq!("Decimal { unscaled: 1, scale: 2 }",
                   format!("{:?}", Decimal::new(1, 2)));
    }
}
