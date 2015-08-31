
pub fn calculate_check_digit(s: &str) -> Option<u32> {
    if let Some(sum) = (0..).zip(s.chars().rev()).fold(Some(0), |result, (index, c)| {
        match result {
            None => None,
            Some(sum) => match c.to_digit(10) {
                None => None,
                Some(d) => Some(sum + if (index % 2) == 0 {
                    sum_digits(d * 2)
                } else {
                    d
                })
            }
        }
    }) {
        Some((10 - (sum % 10)))
    } else {
        None
    }
}

pub fn valid_str(s: &str) -> bool {
    let check_digit = calculate_check_digit(&s[..(s.len() - 1)]);
    !s.is_empty()
        && s.chars().last().unwrap().to_digit(10) == check_digit
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

#[cfg(test)]
mod tests {
    mod calculate_check_digit {
        use super::super::calculate_check_digit;

        #[test]
        fn calculates() { assert_eq!(1, calculate_check_digit("401288888888188").unwrap()); }
        #[test]
        fn yields_none_on_bad_input() { assert_eq!(None, calculate_check_digit("ffff")); }
    }

    mod valid_str {
        use super::super::valid_str;

        #[test]
        fn validates_good_str() { assert!(valid_str("4012888888881881")); }
        #[test]
        fn invalidates_bad_str() { assert!(!valid_str("4012888888881882")); }
    }

    #[test]
    fn sum_digits_works() {
        use super::sum_digits;
        assert_eq!(5, sum_digits(23));
        assert_eq!(6, sum_digits(123));
        assert_eq!(18, sum_digits(54333));
    }
}
