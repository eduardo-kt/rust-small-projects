use std::io;
use std::collections::HashMap;

/// Lê entrada do usuário e conta frequência de palavras.
/// 
fn listar_contagem_palavras() {
    println!("Digite uma frase: ");
    let mut frase = String::new();
    io::stdin().read_line(&mut frase).expect("Erro ao ler a frase");
    //frase = frase.to_string();
    let mut map: HashMap<String, i32> = HashMap::new();

    for word in frase.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    println!("Digite uma frase: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada do terminal");
    
    let palavras  = contar_palavras(&entrada);
    println!("A frase possui {} palavras", palavras);


    listar_contagem_palavras();

}

fn contar_palavras(frase: &str) -> usize {
    frase.split_whitespace().count()
}