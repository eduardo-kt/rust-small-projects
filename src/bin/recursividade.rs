use std::io::{self, Write};

fn fatorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

struct Fibonacci {
    a: u32,
    b: u32,
    i: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Self { a: 0, b: 1, i: 0 }
    }
    fn fibo_position_nro(mut self, n:u32) -> u32 {
        if n == self.i {
            return self.a
        } else {
            self.i += 1;
            let proximo = Fibonacci {
                a: self.b,
                b: self.a + self.b,
                i: self.i,
            };
            proximo.fibo_position_nro(n)            
        }
    }    
}

fn fibonacci(n:u32, a:u32, b:u32, i:u32) -> u32 {
    if n == i {
        return a
    } else {
        return fibonacci(n, b, b+a, i+1)
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
    let fibo_value = Fibonacci::new().fibo_position_nro(10);
    println!("{}", fibo_value);

    let somadigit = soma_digitos(127);
    println!("{}", somadigit);

    let valor_fibo = fibonacci(10, 0, 1, 0);
    println!("{}", valor_fibo);

     
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