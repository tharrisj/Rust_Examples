use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess.");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't convert to number! Please try again");
                continue
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
