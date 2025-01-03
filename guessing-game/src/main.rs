use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=9);

    loop {
        println!("\nPlease input your guess (1-9):");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) if num >= 1 && num <= 9 => num,
            Ok(_) => {
                println!("{}", "Please enter a number between 1 and 9.".red());
                continue;
            }
            Err(_) => {
                println!("{}", "Invalid input. Please enter a valid number.".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "Correct!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big!".yellow()),
            Ordering::Less => println!("{}", "Too small!".yellow()),
        }
    }
}
