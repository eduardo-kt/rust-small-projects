//! Mostra exemplos do capítulo 16 do livro 
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2nd edition).
use std:: thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

}