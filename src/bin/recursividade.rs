use std::io;

fn fatorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

fn main() {
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
    println!("{}", resultado);
}