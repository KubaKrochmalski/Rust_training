extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("[=== WELCOME TO GUESSING GAME ===]");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut attempt = 1;

    loop {
        println!("This is your {} attempt. Please enter your guess.", attempt);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Your number is too small. Try again."),
            Ordering::Greater => println!("Your number is too big. Try again."),
            Ordering::Equal   => {
                println!("You guessed correctly. Congratulations!");
                break;
            }
        }
        attempt = attempt + 1;
    }
}
