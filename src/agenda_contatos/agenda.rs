use crate::agenda_contatos::contato::Contato;

#[derive(Default)]
pub struct Agenda {
    contatos: Vec<Contato>,
}

impl Agenda {
    pub fn new() -> Self {
        Self { contatos: Vec::new() }
    }

    pub fn adicionar(&mut self, contato: Contato) {
        self.contatos.push(contato);
    }

    pub fn listar(&self) -> &[Contato] {
        &self.contatos
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Vec<&Contato> {
        self.contatos
            .iter()
            .filter(|c| c.nome.to_lowercase().contains(&nome.to_lowercase()))
            .collect()
    }

    pub fn remover(&mut self, index: usize) -> Option<Contato> {
        if index < self.contatos.len() {
            Some(self.contatos.remove(index))
        } else {
            None
        }
    }

    pub fn atualizar(&mut self, index: usize, contato: Contato) -> Result<(), ()> {
        if index < self.contatos.len() {
            self.contatos[index] = contato;
            Ok(())
        } else {
            Err(())
        }
    }
}
