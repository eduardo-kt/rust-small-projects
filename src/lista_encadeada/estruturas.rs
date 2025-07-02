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
    
}