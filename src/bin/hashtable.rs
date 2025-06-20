use std::io;

fn hash_algo(entrada: &String) -> u32 {
    let mut hash_value = 0;
    for char in entrada.chars() {
        hash_value += char as u32;
    }
    println!("{}", hash_value);
    hash_value
}

fn main() {
    println!("Insira algum valor para calcular a hash: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let value = hash_algo(&entrada);
    println!("A posição hash é: {}.", value);
}
