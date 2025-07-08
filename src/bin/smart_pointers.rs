//! Mostra exemplos do capítulo 15 do livro 
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2nd edition). 

fn main() {
    let x = 5; // isto é um inteiro
    let y = &x; // isto é um ponteiro para um inteiro

    assert_eq!(x,5);
    assert_eq!(*y,5); // * dereference. &i32 vira i32
}
