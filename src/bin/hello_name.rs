//! Executa saudação personalizada.
//! Demonstra funções, organização de arquivos e testes.
//! A função e o teste estão em hello_worl_aux;
//! # Rodar exemplo
//! ```bash
//! cargo run --bin hello_name
//! ```
//! # Rodar teste
//! ```bash
//! cargo test --lib hello_name
//! ```
use rust_small_projs::hello_name;

fn main() {
    hello_name::hello("Charles");
}