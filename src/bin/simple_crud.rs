use std::io;

fn main() {
    let mut items: Vec<String> = Vec::new();

    loop {
        println!(r#"
        Escolha uma opção:
        =================
        1. Criar item
        2. Listar itens
        3. Atualizar item
        4. Deletar item
        5. Sair
        "#);
        
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler entrada");
        let opcao = opcao.trim();

        match opcao {
            "1" => {
                println!("Digite o nome do item: ");
                let mut novo_item = String::new();
                io::stdin().read_line(&mut novo_item).expect("Erro ao ler entrada de dados");
                items.push(novo_item.trim().to_string());
                println!("Item adicionado!");
            },
            "2" => {
                println!("Itens:");
                for (i, item) in items.iter().enumerate() {
                    println!("{}) {}", i, item);
                }
            },
            "3" => {
                println!("Digite o índice do item a atualizar: ");
                let mut idx = String::new();
                io::stdin().read_line(&mut idx).expect("Erro ao ter entrada de dados");
                let idx: usize = match idx.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Índice inválido");
                        continue;
                    }                    
                };
                if idx < items.len() {
                    println!("Digite o novo valor: ");
                    let mut novo_valor = String::new();
                    io::stdin().read_line(&mut novo_valor).expect("Erro ao ler entrada de dados");
                    items[idx] = novo_valor.trim().to_string();
                    println!("Item atualizado!");
                } else {
                    println!("Índice inválido!");
                }
            },
            "4" => {
                println!("Digite o valor do índice para deletar: ");
                let mut idx_del = String::new();
                io::stdin().read_line(&mut idx_del).expect("Erro ao ler entrada de dados");
                let idx_del: usize = match idx_del.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Índice inválido");
                        continue;
                    }
                };
                if idx_del < items.len() {
                    items.remove(idx_del);
                    println!("Item deletado");
                } else {
                    println!("Índice inválido")
                }
            },
            "5" => {
                println!("Saindo");
                break;
            },
            _ => println!("Opção Inválida"),
        }

    }
}