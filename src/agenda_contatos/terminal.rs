use std::io;
use crate::agenda_contatos::{agenda::Agenda, contato::Contato};

pub fn listar_contatos(agenda: &Agenda) {
    for contato in agenda.listar() {
        println!("{:?}", contato); // TODO: melhorar impressao
    }

}

pub fn adicionar_contato(agenda: &mut Agenda){
    let mut nome = String::new();
    let mut telefone = String::new();
    let mut email = String::new();

    println!("Digite o nome: ");
    io::stdin().read_line(&mut nome).unwrap(); //TODO: tratamento de erro
    let nome = nome.trim().to_string();

    println!("Digite o telefone: ");
    io::stdin().read_line(&mut telefone).unwrap(); //TODO: tratamento de erro
    let telefone = telefone.trim().to_string();

    println!("Digite o email: ");
    io::stdin().read_line(&mut email).unwrap(); //TODO: tratamento de erro
    let email = email.trim().to_string();

    match Contato::new(nome, telefone, email) {
        Some(contato) => {
            agenda.adicionar(contato);
            println!("Contato adicionado com sucesso");
        }
        None => {
            println!("Erro ao adicionar contato. Revise email e/ou telefone");
        }
    }
}

pub fn buscar_contato(agenda: &Agenda){
    //TODO
}

pub fn atualizar_contato(agenda: &mut Agenda){
    //TODO
}

pub fn remover_contato(agenda: &mut Agenda){
    //TODO
}