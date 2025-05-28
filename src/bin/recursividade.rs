use std::io::{self, Write};

fn fatorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

//TODO: implementar fns fibonacci como struct


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


fn fibo_number_at(position: usize) -> usize {
    let (mut a, mut b)=(0,1);
    for _ in 0..position {
        println!("{}", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}


fn fibo_between_xy(x:usize, y:usize) -> Vec<usize> {
    let (mut a, mut b) = (0,1);
    let mut fibo_vec: Vec<usize> = Vec::new();
    while a <= y {
        if a >= x {
            fibo_vec.push(a);
        }
        let temp = a;
        a = b;
        b += temp;
    }
    fibo_vec
}


fn fibo_closest_to(number:usize) -> usize {
    let (mut a, mut b) = (0,1);
    while b < number {
        let temp = a;
        a = b;
        b += temp;
    }
    if number.abs_diff(a) <= number.abs_diff(b) {
        println!("{} - {} - {}", a, number, b);
        a
    } else {
        b
    }
}

fn main() {
    let fib1 = fibo(20);
    println!("Número Fibonacci menor que 20 eh {}", fib1);
    
    let fib2 = fibo_number_at(10);
    println!("Numero Fibonacci na posicao 10 eh {}", fib2);

    let fib3 = fibo_closest_to(14);
    println!("Numero Fibonacci mais próximo de 14 eh {}",fib3);

    let fib4 = fibo_between_xy(4, 36);
    println!("Numeros Fibonacci entre 4 e 36");
    println!("{:?}", fib4);
    
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