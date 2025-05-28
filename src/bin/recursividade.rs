use std::io::{self, Write};

fn fatorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

//TODO: função para retornar número fibonacci menor que x

fn fibo(n:usize) -> usize {
    let (mut a, mut b) = (0,1);    
    loop {
        println!("{}",a);
        if a >= n {
            break;
        } else {
            let temp = a;
            a=b;
            b=temp+b;
        }
    }
    a
}

//TODO: função que retorna  número na posição x da seq fibonacci 
//TODO: função para retornar sequencia fibonacci entre x e y
//TODO: função para retornar número fibonacci mais próximo de x

fn main() {
    let fb = fibo(20);
    println!("{}", fb);
    
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