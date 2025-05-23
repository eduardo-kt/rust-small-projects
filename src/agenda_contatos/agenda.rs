use crate::agenda_contatos::contato::Contato;

#[derive(Default)]
pub struct Agenda {
    pub contatos: Vec<Contato>,
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

    pub fn buscar_por_nome(&mut self, nome: &str) -> Option<&mut Contato> {
        self.contatos
            .iter_mut()
            .find(|c| c.nome == nome)
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
    pub fn remover_contato_por_nome(&mut self, nome: &str) -> bool {
        let original_len = self.contatos.len();
        self.contatos.retain(|c| c.nome != nome);
        original_len != self.contatos.len()
    }
}
