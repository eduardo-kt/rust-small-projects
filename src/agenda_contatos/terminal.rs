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
    println!("Digite o nome para buscar: ");
    let mut termo = String::new();
    io::stdin().read_line(&mut termo).unwrap();
    let termo = termo.trim().to_lowercase();

    let mut encontrados = vec![];

    for contato in &agenda.contatos {
        if contato.nome.to_lowercase().contains(&termo) {
            encontrados.push(contato);
        }
    }

    if encontrados.is_empty() {
        println!("Nenhum contato encontrado com o nome informado");
    } else {
        println!("Contatos encontrados:");
        for (i, contato) in encontrados.iter().enumerate() {
            println!(
                "{} - Nome: {}, Telefone: {}, Email: {}",
                i + 1,
                contato.nome,
                contato.telefone,
                contato.email,
            );
        }
    }
}

pub fn atualizar_contato(agenda: &mut Agenda){
    println!("Digite o nome do contato a ser atualizado: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();

    if let Some(contato) = agenda.buscar_por_nome(nome) {
        println!("Novo telefone: ");
        let mut telefone = String::new();
        io::stdin().read_line(&mut telefone).unwrap();
        let telefone = telefone.trim();
        if !telefone.is_empty() {
            contato.telefone = telefone.to_string();
        }

        println!("Novo email: ");
        let mut email = String::new();
        io::stdin().read_line(&mut email).unwrap();
        let email = email.trim();
        if !email.is_empty() {
            contato.email = email.to_string();
        }
        println!("Contato atualizado com sucesso");
    } else {
        println!("Contato não identificado");
    }
}

pub fn remover_contato(agenda: &mut Agenda){
    println!("Digite o nome do contato a ser removido: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();

    if agenda.remover_contato_por_nome(nome) {
        println!("Contato removido com sucesso");        
    } else {
        println!("Contato não encontrado");
    }
}