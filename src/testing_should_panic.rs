#[derive(Debug)]
pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("O valor precisa estar entre 1 e 100");
        }
        Guess {value}
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // adiciona atributo se panic for comportamento esperado
    fn greater_than_100() {
        Guess::new(102);
    }
}