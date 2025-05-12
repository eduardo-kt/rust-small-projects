pub mod temperature;
use std::io;

pub enum Alternative {
    Celsius,
    Kelvin,
    Fahrenheit,
    T
}

pub fn processar_entrada() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada do terminal");
    let entrada = entrada.trim().to_string().to_lowercase();
    entrada
}