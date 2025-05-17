use std::io;
use rand::Rng;
use rust_small_projs::random_walk::desenhar_grafico;

fn main() {
    println!("Digite valor inicial: ");
    let valor_inicial = loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Escolha um valor válido");
        match entrada.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Escolha um valor válido.\n");
                continue;
            }
        }
    };
    println!("Digite o número de períodos: ");
    let periodos = loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Escolha um valor válido");
        match entrada.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Escolha um valor válido.\n");
                continue;
            }
        }
    };

    let mut valores:Vec<f64> = Vec::new();
    valores.push(valor_inicial);

    let mut rng = rand::rng();


    for _ in 1..=periodos {
        let ultimo = *valores.last().unwrap();
        let sorteio = rng.random_range(0..=1);
        
        let novo = match sorteio {
            1 => ultimo * 1.1,
            0 => ultimo * 0.9,
            _ => unreachable!(),
        };
        valores.push(novo);
    }

    println!("Valores obtidos:");
    for (i,v) in valores.iter().enumerate() {
        println!("Periodo {}: {:.2}",i,v);
    }

    desenhar_grafico(&valores).unwrap();


}