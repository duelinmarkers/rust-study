extern crate rand;

use std::io;
use std::cmp::Ordering;

#[allow(dead_code)]
fn main() {
    println!("Guess the number!");

    let secret_number = (rand::random::<usize>() % 100) + 1;

    loop {
        println!("Please input your guess.");

        let mut s = String::new();
        let _ = io::stdin().read_line(&mut s).unwrap();
        let input_num = s.trim().parse::<usize>().ok();

        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", num);

        match cmp(num, secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                return;
            },
        }
    }
}

fn cmp(a: usize, b: usize) -> Ordering {
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
