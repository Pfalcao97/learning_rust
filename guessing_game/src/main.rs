use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the secret number:");

    loop {

        println!("Guess the number!\nPlease input your guess.");

        let mut guess: String = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim()
                            .parse() {
                                Ok(num) => num,
                                Err(_) => {
                                    println!("Couldn't convert to i32, ignoring.");
                                    continue;
                                }
                            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {
                                println!("You win");
                                break;
                            },
            Ordering::Greater => println!("Too big."),
        }
    }
}