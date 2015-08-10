#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate decimal;
extern crate quickcheck;

use decimal::Decimal;
use quickcheck::TestResult;

#[quickcheck]
fn displayed_strings_reparse_as_same_value(unscaled: i64, scale: u32) -> bool {
    let original = Decimal::new(unscaled, scale);
    let reparsed = format!("{}", original).parse::<Decimal>().unwrap();
    original == reparsed
}

#[quickcheck]
fn divide_then_multiply_then_add_remainder_restores_original_value(
    dividend_unscaled: i64, divisor_unscaled: i64, scale: u32) -> TestResult {
    let dividend = Decimal::new(dividend_unscaled, scale);
    let divisor = Decimal::new(divisor_unscaled, scale);
    if divisor_unscaled == 0 {
        TestResult::discard()
    } else {
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        let rebuilt_dividend = ((quotient * divisor) + remainder).adjust_scale(scale);
        if dividend == rebuilt_dividend {
            TestResult::passed()
        } else {
            TestResult::error(
                format!("dividend:{} divisor:{} quotient:{} remainder:{} rebuilt:{}",
                        dividend, divisor, quotient, remainder, rebuilt_dividend))
        }
    }
}
