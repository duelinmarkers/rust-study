extern crate rand;

use std::io;
use std::cmp::Ordering;

const MAX_NUM : u8 = 100;

#[allow(dead_code)]
fn main() {
    println!("Guess the number between 1 and {}!", MAX_NUM);

    let secret_number = (rand::random::<u8>() % MAX_NUM) + 1;

    loop {
        println!("Please input your guess.");

        let mut s = String::new();
        let _ = io::stdin().read_line(&mut s).unwrap();

        if let Some(num) = s.trim().parse::<u8>().ok() {

            match cmp(num, secret_number) {
                Ordering::Less    => println!("{} is too small!", num),
                Ordering::Greater => println!("{} is too big!", num),
                Ordering::Equal   => {
                    println!("{} is the number -- You win!", num);
                    return;
                },
            }
        } else {
            println!("Please input a number between 1 and {}!", MAX_NUM);
            continue;
        }

    }
}

fn cmp(a: u8, b: u8) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

#[cfg(test)]
mod cmp {

    use std::cmp::Ordering;

    #[test]
    fn when_first_is_less_than_second() {
        assert_eq!(Ordering::Less, super::cmp(0, 1));
    }

    #[test]
    fn when_args_are_equal() {
        assert_eq!(Ordering::Equal, super::cmp(1, 1));
    }

    #[test]
    fn when_first_is_greater_than_second() {
        assert_eq!(Ordering::Greater, super::cmp(1, 0));
    }
}
