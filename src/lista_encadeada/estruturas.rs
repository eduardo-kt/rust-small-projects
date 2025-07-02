use std::io::{self, Write};

#[derive(Clone)]
pub enum Cor {
    Verde,
    Amarelo,
}

impl Cor {
    fn id(&self) -> &str {
        match self {           
            Cor::Amarelo => "A",
            Cor::Verde => "V",
        }
    }    
}

#[derive(Clone)]
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

        match entrada.as_str() {
            "1" => {
                self.inserir_com_prioridade();
                println!("Cartão inserido com sucesso!");
            },
            "2" => {
                self.inserir_sem_prioridade();
                println!("Cartão inserido com sucesso!");
            },
            _ => {
                println!("Entrada inválida. Retornando ao menu principal...");
                return;
            },
            
        };

    }

    pub fn imprimir(&self) {
        let mut atual = self.head.as_ref();

        while let Some(nodo) = atual {
            let card = &nodo.dado;
            print!("{}{} -> ", card.cor.id(), card.nro);
            atual = nodo.proximo.as_ref();
        }
        print!("None");
    }

    pub fn chamar_paciente(&mut self) {
        match self.head.take() {
            Some(nodo) => {
                let card = nodo.dado;
                println!("
                Atenção paciente cartão {}{}!
                Dirija-se ao atendimento.",
            card.cor.id(), card.nro);
            self.head = nodo.proximo;
            }
            None => println!("Lista de Espera vazia.")            
        }
    }
    pub fn inserir_sem_prioridade(&mut self) -> Card {
        let card = Card {
            cor: Cor::Verde,
            nro: self.proximo_verde,
        };
        self.proximo_verde += 1;

        let novo = Box::new(Elemento {
            dado: card.clone(),
            proximo: None,
        });

        match self.head.as_mut() {
            None => {
                self.head = Some(novo);                
            }
            Some(mut atual) => {
                while let Some(ref mut proximo) = atual.proximo  {
                    atual = proximo;                    
                }
                atual.proximo = Some(novo);
            }            
        }
        card
    }

    pub fn inserir_com_prioridade(&mut self) {
        let card = Card {
            cor: Cor::Amarelo,
            nro: self.proximo_amarelo,
        };
        self.proximo_amarelo += 1;

        let mut novo = Box::new(Elemento {
            dado: card.clone(),
            proximo: None,
        });

        match self.head.as_mut() {
            None => {
                self.head = Some(novo);
            },
            Some(head) => {
                if matches!(head.dado.cor, Cor::Verde) {
                    let novo_head = Box::new(Elemento {
                        dado: card,
                        proximo: self.head.take(),
                    });
                    self.head = Some(novo_head);
                    println!("Cartão inserido com sucesso");
                    return;
                }
                let mut atual = &mut self.head;
                while let Some(ref nodo) = atual.as_ref() {
                    if matches!(nodo.dado.cor, Cor::Verde) {
                        break;
                    }
                    atual = &mut atual.as_mut().unwrap().proximo;
                }
                novo.proximo = atual.take();
                *atual = Some(novo);
            }
        }
        println!("Cartao inserido com sucesso")

    }
    
}