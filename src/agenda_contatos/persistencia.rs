use crate::agenda_contatos::agenda::Agenda;
use serde_json;
use std::fs;

use super::contato::Contato;

pub fn salvar_agenda(agenda: &Agenda, caminho: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&agenda.listar())?;
    fs::write(caminho, json)?;
    Ok(())
}

pub fn carregar_agenda(caminho: &str) -> Result<Agenda, Box<dyn std::error::Error>> {
    let dados = fs::read_to_string(caminho)?;
    let contatos: Vec<Contato> = serde_json::from_str(&dados)?;
    let mut agenda = Agenda::new();
    for contato in contatos {
        agenda.adicionar(contato);
    }
    Ok(agenda)
}
