//! Mostra exemplos do capítulo 15 do livro 
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2nd edition). 
use rust_small_projs::smart_pointers::{MyBox,hello};

fn main() {
    let x = 5; // isto é um inteiro
    // let y = &x; // ponteiro -> inteiro
    // let y = Box::new(x); // um box -> cópia de x 
    let y = MyBox::new(x);

    assert_eq!(x,5);
    assert_eq!(*y,5); // * dereference. &i32 vira i32

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
