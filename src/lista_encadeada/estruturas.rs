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
}

impl ListaSimples {
    pub fn new() -> Self {
        Self { head: None }
    }
    
}