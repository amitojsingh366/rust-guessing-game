use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");

    let number = rand::thread_rng().gen_range(1..101);

    let mut user_tries: u32 = 0;
    const TRIES: u32 = 10;

    loop {
        if &TRIES <= &user_tries {
            break;
        }

        println!("Please enter your guess (number between 1 and 100): ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading from console");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        user_tries = &user_tries + 1;
        let remaining_tries = &TRIES - &user_tries;

        match guess.cmp(&number) {
            Ordering::Less => println!("Too Low :(\n{} tries remaining", remaining_tries),
            Ordering::Greater => println!("Too high :(\n{} tries remaining", remaining_tries),
            Ordering::Equal => {
                println!("Correct guess :)");
                break;
            }
        }
    }
}
