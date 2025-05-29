use std::io::{self, Write};

fn fatorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

fn soma_digitos(n: u32) -> u32 {
    if n < 10 {
        n
    } else {
        (n % 10) + soma_digitos(n / 10)
    }
}

fn main() {

    let somadigit = soma_digitos(127);
    println!("{}", somadigit);

     
    println!(r#"
    Calculo fatorial:
    ================
    "#);
    print!("Digite um número para cálculo fatorial: ");
    io::stdout().flush().unwrap();
    let valor: u32 = loop {
        let mut entrada = String::new();  
        io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada do terminal");
        match entrada.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("O valor de entrada precisa ser um inteiro positivo");
                continue;
            }
        }
    };
    let resultado = fatorial(valor);    
    println!("O fatorial de {} é {}", valor, resultado);     
}