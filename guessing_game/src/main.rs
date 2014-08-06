use std::io;
use std::rand;

#[allow(dead_code)]
fn main() {
    println!("Guess the number!");

    let secret_number = (rand::random::<uint>() % 100) + 1;

    loop {
        println!("Please input your guess.");

        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
        let input_num: Option<uint> = from_str(input.as_slice().trim());

        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", num);

        match cmp(num, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => {
                println!("You win!");
                return;
            },
        }
    }
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

#[cfg(test)]
mod cmp {

    #[test]
    fn when_first_is_less_than_second() {
        assert_eq!(Less, super::cmp(0, 1));
    }

    #[test]
    fn when_args_are_equal() {
        assert_eq!(Equal, super::cmp(1, 1));
    }

    #[test]
    fn when_first_is_greater_than_second() {
        assert_eq!(Greater, super::cmp(1, 0));
    }
}
