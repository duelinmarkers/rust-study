//! An implementation of the [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)
//! for validating credit card numbers and assorted other things.

/// Provides the check digit to be appended to `partial_num` to create a valid luhn-checkable value.
pub fn calculate_check_digit(partial_num: &str) -> Option<u32> {
    if partial_num.is_empty() {
        return None;
    }
    (0..).zip(partial_num.chars().rev()).fold(Some(0), |result, (index, c)| {
        result.and_then(|sum| {
            c.to_digit(10).map(|d| {
                sum + if (index % 2) == 0 {
                    sum_digits(d * 2)
                } else {
                    d
                }
            })
        })
    }).map(|sum| { 10 - (sum % 10) })
}

/// Performs the checksum on `s`.
pub fn valid_str(num: &str) -> bool {
    num.chars().last()
        .and_then(|last_char| { last_char.to_digit(10) })
        .map_or(false, |digit| {
            calculate_check_digit(butlast(1, num))
                .map_or(false, |real_digit| { digit == real_digit })
        })
}

fn sum_digits(i: u32) -> u32 {
    let mut i = i;
    let mut result = 0;
    while i > 0 {
        result += i % 10;
        i = i / 10;
    }
    result
}

fn butlast(n: usize, s: &str) -> &str {
    &s[..(s.len() - n)]
}

#[cfg(test)]
mod tests {
    mod calculate_check_digit {
        use super::super::calculate_check_digit;

        #[test]
        fn calculates_on_realistic_cc() {
            assert_eq!(1, calculate_check_digit("401288888888188").unwrap());
        }
        #[test]
        fn calculates_for_super_simple_examples() {
            assert_eq!(2, calculate_check_digit("4").unwrap());
            assert_eq!(1, calculate_check_digit("14").unwrap());
        }
        #[test]
        fn yields_none_on_non_numeric() {
            assert_eq!(None, calculate_check_digit("ffff"));
        }
        #[test]
        fn yields_none_on_empty() {
            assert_eq!(None, calculate_check_digit(""));
        }
    }

    mod valid_str {
        use super::super::valid_str;

        #[test]
        fn validates_good_cc() { assert!(valid_str("4012888888881881")); }
        #[test]
        fn invalidates_bad_cc() { assert!(!valid_str("4012888888881882")); }
        #[test]
        fn invalidates_non_numeric_check_digit() { assert!(!valid_str("401288888888188G")); }
        #[test]
        fn invalidates_non_numeric() { assert!(!valid_str("4012HI")); }
        #[test]
        fn invalidates_empty() { assert!(!valid_str("")); }
    }

    #[test]
    fn sum_digits_works() {
        use super::sum_digits;
        assert_eq!(5, sum_digits(23));
        assert_eq!(6, sum_digits(123));
        assert_eq!(18, sum_digits(54333));
    }
}
