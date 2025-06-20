use std::io;

fn main() {
    println!("O que você quer comprar? ");
    let mut item = String::new();
    io::stdin().read_line(&mut item).unwrap();
    item = item.trim().to_string();
    
    println!("Qual o preço do item? ");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("Erro na leitura");
    let price: f32 = match price.trim().parse::<f32>() {
        Ok(numb) => numb,
        Err(_) => {
            println!("Entrada inválida");
            return;
        }
    };

    println!("Quantas unidades você quer comprar? ");
    let mut quantidade = String::new();
    io::stdin().read_line(&mut quantidade).expect("Erro na leitura");
    let quantidade: u32 = match quantidade.trim().parse::<u32>() {
        Ok(numb) => numb,
        Err(_) => {
            println!("Entrada inválida");
            return;
        }
    };

    let total = price * (quantidade as f32);

    println!("Você comprou {} {}/s por ${:.2} cada unidade.", quantidade, item, price);
    println!("O total pago pela compra foi ${:.2}.", total);
}