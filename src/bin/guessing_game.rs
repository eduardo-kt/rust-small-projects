use std::io;
use std::cmp::Ordering;
use rand::Rng;

/// # Gessing Game 
/// From Klabnik, S., & Nichols, C. (2023). The Rust programming language (2nd edition).
/// with extra validation to guess number between allowed range (1..=100)

fn main() {
    println!("Guessing Game:");
    
    // random number generator
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Choose a number between 1-100:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read data from terminal.");

        let guess: u32 = match guess.trim().parse() {
            // additional range validation
            Ok(num) if (1..=100).contains(&num) => num,
            Ok(_) => {
                println!("Choose a valid number");
                continue;
            }
            Err(_) => {
                println!("Choose a valid number");
                continue;
            }
        };

        println!("Your guess is : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}