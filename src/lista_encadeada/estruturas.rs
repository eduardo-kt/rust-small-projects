use std::io::{self, Write};

pub enum Cor {
    Verde,
    Amarelo,
}
pub struct Card {
    cor: Cor,
    nro: u32,
}
pub struct Elemento {
    dado: Card,
    proximo: Option<Box<Elemento>>,
}

pub struct ListaSimples {
    head: Option<Box<Elemento>>,
    proximo_verde: u32,
    proximo_amarelo: u32,
}

impl ListaSimples {
    pub fn new() -> Self {
        Self { head: None, proximo_verde: 1, proximo_amarelo: 201 }
    }

    pub fn inserir(&mut self) {
        println!("
        Escolha a cor do cartão. Digite:
        1 - Cartão Amarelo(Prioridade)
        2 - Cartão Verde
        ");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();

        let entrada = entrada.trim().to_lowercase();

        let card = match entrada.as_str() {
            "1" => {
                let card = Card {
                    cor: Cor::Amarelo,
                    nro: self.proximo_amarelo,
                };
                self.proximo_amarelo += 1;
                card
            },
            "2" => {
                let card = Card {
                    cor: Cor::Verde,
                    nro: self.proximo_verde,
                };
                self.proximo_verde += 1;
                card
            },
            _ => {
                println!("Entrada inválida. Retornando ao menu principal...");
                return;
            },
            
        };
        let novo_elemento = Box::new(Elemento {
            dado: card,
            proximo: self.head.take()
        });

        self.head = Some(novo_elemento);
        println!("Cartão inserido com sucesso!");
    }
    
}