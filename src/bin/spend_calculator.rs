use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Despesa {
    pessoa: String,
    valor: f64,
    descricao: String,
}

fn main() {
    let mut pessoas: Vec<String> = Vec::new();
    let mut despesas: Vec<Despesa> = Vec::new();

    loop {
        println!(r#"
        -*- Calculadora de Despesas Compartilhadas -*-

        Digite 1 para adicionar pessoa
        Digite 2 para adicionar despesa
        Digite 3 para listar despesas
        Digite 4 para calcular divisão
        Digite 5 para sair
        "#);

        let mut escolha = String::new();
        // TODO: implementar com expect ao inves de unwrap
        io::stdin().read_line(&mut escolha).unwrap(); 
        let escolha = escolha.trim();

        match escolha {
            "1" => adicionar_pessoa(&mut pessoas),
            "2" => adicionar_despesa(&mut despesas, &pessoas),
            "3" => listar_despesas(&despesas),
            "4" => calcular_divisao(&despesas, &pessoas),
            "5" => {
                println!("Saindo do app");
                break;
            },
            _ => println!("Opção Inválida"),
        }
    }
}

fn adicionar_pessoa(pessoas: &mut Vec<String>) {
    println!("Digite o nome da pessoa: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim().to_string();

    if !pessoas.contains(&nome) {
        pessoas.push(nome);
        println!("pessoa adicionada");        
    } else {
        println!("A pessoa já está no grupo");
    }
}

fn adicionar_despesa(despesas: &mut Vec<Despesa>, pessoas: &Vec<String>) {
    println!("Quem realizou esta despesa? ");
    for (i, pessoa) in pessoas.iter().enumerate() {
        println!("{} {}", i, pessoa);
    }

    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();
    let index = index.trim().parse().unwrap_or(usize::MAX);

    if index >= pessoas.len() {
        println!("Índice inválido");
        return;
    }

    let pessoa = &pessoas[index];

    println!("Valor da despesa");

    let mut valor_despesa = String::new();
    io::stdin().read_line(&mut valor_despesa).unwrap();
    let valor: f64 = valor_despesa.trim().parse().unwrap_or(0.0);

    println!("Descrição da despesa:");

    let mut descricao = String::new();
    io::stdin().read_line(&mut descricao).unwrap();

    despesas.push(Despesa {
        pessoa: pessoa.to_string(),
        valor,
        descricao: descricao.trim().to_string(),
    });
    println!("Despesa adicionada");

}

fn listar_despesas() {}

fn calcular_divisao() {}
