use std::io;
use std::cmp::Ordering;
use rand::Rng;
// todo: docstring
// erro ao tentar numero fora do range
fn main() {
    println!("Guessing Game:");
    

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Choose a number between 1-100:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read data from terminal.");

        let guess: u32 = match guess.trim().parse() {
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