use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contato {
    pub nome: String,
    pub telefone: String,
    pub email: String,
}

impl Contato {
    pub fn new(nome:String, telefone: String, email: String) -> Option<Self> {
        if !validar_email(&email) || !validar_telefone(&telefone) {
            return None;
        }

        Some(Self { nome, telefone, email})
    }
}

fn validar_email(email: &str) -> bool {
    Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap().is_match(email)    
}

fn validar_telefone(telefone: &str) -> bool {
    Regex::new(r"^\+?\d{8,15}$").unwrap().is_match(telefone)    
}