use std::io;
use std::rand;

fn main() {
    println!("Guess the number!");

    let secret_number = (rand::random::<int>() % 100i) + 1i;

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let input = io::stdin().read_line()
                           .ok()
                           .expect("Failed to read line");


    println!("You guessed: {}", input);
}
