use std::io;

fn main() {
    println!("Digite uma frase: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada do terminal");
    
    let palavras  = contar_palavras(&entrada);
    println!("A frase possui {} palavras", palavras);

}

fn contar_palavras(frase: &str) -> usize {
    frase.split_whitespace().count()
}