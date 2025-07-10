//! Mostra exemplos do capítulo 18 do livro 
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2nd edition).
fn char_pattern() {
    let x = 'h';

    match x {
        'a'..'h' => println!("ASCII a-g"),
        'h'..='z' => println!("ASCII h-z"),
        _ => println!("ASCII ..."),
    }
}
fn main() {
    let favorite_color:Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!(
            "Using your favorite, {color}, as the background"
        );
    } else if is_tuesday {
        println!(
            "Tuesday is green day!"
        );
    } else if let Ok(age) = age {
        if age > 30 {
            println!(
                "using purple as the background color"
            );
        } else {
            println!(
                "Using orange as the background color"
            );
        }
    } else {
        println!(
            "Using blue as the background color"
        );
    }
    char_pattern();
}