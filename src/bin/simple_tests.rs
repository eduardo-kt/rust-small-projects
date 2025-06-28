//! Binário para realizar os exemplos do capítulo 11 (automated tests)
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2e)
//! O binário tem um módulo auxiliar de mesmo nome (simple_tests.rs)
//! Exemplos no binário, testes no módulo auxiliar
//! # Execução do binário
//! ```bash
//! cargo run --bin simple_tests
//! ```
//! # Execução dos testes
//! ```bash
//! cargo test --lib simple_tests
//! ```

use rust_small_projs::simple_tests;

fn main() {
    let result = simple_tests::simple_sum(2, 2);
    println!("{result}");
}
