//! Exercício do capítulo 12 (an I/O project) do livro
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2e)
//! Execute com: cargo run --bin minigrep
//! Execute ex(p.245) com: cargo run --bin minigrep needle haystack

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

}
