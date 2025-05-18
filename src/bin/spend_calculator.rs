use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Despesa {
    pessoa: String,
    valor: f64,
    descricao: String,
}
// TODO: refatorar main. colocar f-auxiliares no modulo spend_calculator
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
        // TODO: implementar com expect ao inves de unwrap. ver todos
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
// TODO: adicionar persistencia de dados
// TODO: adicionar testes unitários
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

fn listar_despesas(despesas: &Vec<Despesa>) {
    println!("== Lista de Despesas ==");
    for d in despesas {
        println!(
            "{} gastou R${:.2} em \"{}\"",d.pessoa, d.valor, d.descricao
        );
    }
}
// TODO: tratar divisão por zero em valor individual
fn calcular_divisao(despesas: &Vec<Despesa>, pessoas: &Vec<String>) {
    let total: f64 = despesas.iter().map(|d| d.valor).sum();
    let valor_individual = total / pessoas.len() as f64;

    let mut pagos: HashMap<String, f64> = HashMap::new();

    for d in despesas {
        *pagos.entry(d.pessoa.clone()).or_insert(0.0) += d.valor;
    }

    println!(" # Saldo por Pessoa #");
    for pessoa in pessoas {
        let pagou = pagos.get(pessoa).unwrap_or(&0.0);
        let saldo = pagou - valor_individual;

        if saldo > 0.0 {
            println!("{} tem R${:.2} a receber", pessoa, saldo);
        } else if saldo < 0.0 {
            println!("{} tem saldo de R${:.2}", pessoa, saldo);
        } else {
            println!("{} está quite", pessoa);
        }

    }
    
}
