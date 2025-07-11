//! Exercício do capítulo 12 (an I/O project) do livro
//! Klabnik, S., & Nichols, C. (2023). The Rust programming language (2e)
//! Execute com: cargo run --bin minigrep
//! Execute ex(p.245) com: cargo run --bin minigrep needle haystack
//! 'poem.txt' está em resources.
//! Execute pag 248 com: cargo run --bin minigrep the resources/poem.txt
//! # Exemplos
//! ```bash
//! # Rodar exemplo que retorna 1 linha
//! cargo run --bin minigrep frog resources/poem.txt
//! # Rodar exemplo que retorna multiplas linhas
//! cargo run --bin minigrep body resources/poem.txt
//! # Rodar exemplo que não retorna nada
//! cargo run --bin minigrep monomorphization resources/poem.txt
//! # Com ignore_case no powershel
//! $Env:IGNORE_CASE=1; cargo run --bin minigrep to resources/poem.txt
//! # para remover ignore case depois
//! Remove-Item Env:IGNORE_CASE
//! # para testar salvar conteúdo direto em arquivo .txt
//! cargo run --bin minigrep to resources/poem.txt > output.txt
//! ```

use rust_small_projs::minigrep::run_minigrep;

fn main() {
    run_minigrep();
    

}
