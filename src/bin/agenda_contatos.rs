//! Projeto: Agenda de Contatos
//!
//! Funcionalidades:
//! - Cadastro de contatos com as informações: nome, telefone e email.
//! - Listagem dos contatos cadastrados.
//! - Atualização dos dados de um contato existente.
//! - Remoção de contatos da agenda.
//! - Busca de contatos por nome, retornando resultados encontrados.
//! - Armazenamento local dos contatos, utilizando arquivo (JSON, CSV ou similar).
//!
//! Objetivo:
//! Construir uma aplicação básica para gerenciar uma lista de contatos pessoais,
//! exercitando operações CRUD e manipulação de dados persistentes.
//!
//! Escopo:
//! - Interface simples via terminal (linha de comando).
//! - Validação básica de dados (ex: formato de email e telefone).
//! - Persistência de dados para que as informações sejam mantidas entre execuções do programa.
//!
//! Uso:
//! Ideal para praticar estruturas de dados, manipulação de arquivos,
//! entrada e saída do usuário, e conceitos básicos de serialização.

use std::io::{self, Write};
use rust_small_projs::agenda_contatos::agenda::Agenda;
use rust_small_projs::agenda_contatos::persistencia::{carregar_agenda,salvar_agenda};
use rust_small_projs::agenda_contatos::terminal;

fn main() {
    let mut agenda = carregar_agenda("contatos.json").unwrap_or_else(|_| Agenda::new());

    loop {
        println!("\nMenu:");
        println!("1 - Listar contatos");
        println!("2 - Adicionar contato");
        println!("3 - Buscar contato por nome");
        println!("4 - Atualizar contato");
        println!("5 - Remover contato");
        println!("0 - Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();
        let escolha = escolha.trim();

        match escolha {
            "1" => terminal::listar_contatos(&agenda),
            "2" => terminal::adicionar_contato(&mut agenda),
            "3" => terminal::buscar_contato(&agenda),
            "4" => terminal::atualizar_contato(&mut agenda),
            "5" => terminal::remover_contato(&mut agenda),
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida, tente novamente."),
        }
    }

    if let Err(e) = salvar_agenda(&agenda, "contatos.json") {
        eprintln!("Erro ao salvar agenda: {}", e);
    }
}