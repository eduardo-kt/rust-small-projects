#[derive(Debug, Clone, Copy, PartialEq)]
enum Naipe {
    Clubs, Hearts, Spades, Diamonds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Valor {
    As, Um, Dois, Tres, Quatro, Cinco, Seis, 
    Sete, Oito, Nove, Dez, Valete, Rainha, Rei,    
}

#[derive(Debug, Clone, Copy)]
struct Carta {
    naipe: Naipe,
    valor: Valor,
}

// implementa um baralho
struct Baralho {
    cartas: Vec<Carta>,
}

impl Baralho {
    fn new() -> Self {
        let mut cartas = Vec::new();
        for naipe in [
            Naipe::Clubs, Naipe::Diamonds, 
            Naipe::Hearts, Naipe::Spades, ] {
                for valor in [
                    Valor::As, Valor::Um, Valor::Dois, 
                    Valor::Tres, Valor::Quatro, Valor::Cinco, 
                    Valor::Seis, Valor::Sete, Valor::Oito, 
                    Valor::Nove, Valor::Dez, Valor::Valete, 
                    Valor::Rainha, Valor::Rei,
                ] {
                cartas.push(Carta { naipe, valor });
            }
        }   
        Self {cartas}        
    }
    
    fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        self.cartas.shuffle(&mut rand::rng());
    }

    fn draw_card(&mut self, how_many: u8) -> Vec<Carta> {
        let mut hand = Vec::new();
        for _ in 0..how_many {
            hand.push(self.cartas.pop().unwrap());
        }
        hand
    }
}

fn main() {
    let mut baralho = Baralho::new();
    baralho.shuffle();
    let card = baralho.draw_card(5);
    println!("{:?}", card);

}

