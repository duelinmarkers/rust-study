// Copyright (c) 2015 John D. Hume
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

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
