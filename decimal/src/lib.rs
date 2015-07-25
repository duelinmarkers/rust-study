use std::ops;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Decimal {
    unscaled: i64,
    scale: i64
}

impl Decimal {
    fn new(unscaled: i64, scale: i64) -> Decimal {
        Decimal { unscaled: unscaled, scale: scale }
    }

    pub fn set_scale(self, scale: i64) -> Decimal {
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
               (self.unscaled as f64) / (10i64.pow(self.scale as u32) as f64))
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

fn downscale(n: i64, down_by: i64) -> i64 {
    let mut result = n;
    for _ in 0..down_by {
        result = result / 10;
    }
    result
}

fn upscale(n: i64, up_by: i64) -> i64 {
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
    }
}
