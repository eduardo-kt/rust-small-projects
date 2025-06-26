use std::io;

// Conversor de peso
// converte entre Kg e Pounds
fn conversor_peso() {
    let mut entrada = String::new();
    println!(r#"
Conversor Pounds<=>Kgs
======================
Digite o peso: "#);
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    let peso: f32 = match entrada.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Digite um número que corresponda a um peso válido:");
            return;
        }
    };
    println!(r#"
Digite:
1 Para converter para Pounds
2 Para converter para Kgs
"#);
    let mut opcao = String::new();
    io::stdin().read_line(&mut opcao).expect("Erro ao ler entrada");
    match opcao.trim() {
        "1" => {
            let pound = peso * 2.20462;
            println!("O peso em pounds é {:.2}", pound);
        },
        "2" => {
            let kg = peso / 2.20462;
            println!("O peso em Kgs é {:.2}", kg);
        },
        _ => {
            println!("Escolha uma opção válida (1 ou 2).");
            return;
        },        
    }
}


fn main() {
    conversor_peso();
    
}