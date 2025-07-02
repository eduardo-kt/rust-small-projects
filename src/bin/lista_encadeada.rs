//! # Lista Hospitalar
//! 
//! Este projeto implementar uma lista encadeada simples que simula o registro
//! e atendimento de pacientes em um hospital. Os pacientes são registrados com
//! cartões Amarelos(prioridade) ou Verdes. Os cartões amarelos iniciam numeração 
//! em 201 e os cartões verdes iniciam numeração em 1. Para ambas as cores, 
//! numerações mais baixas tem prioridade no atendimento.
//! Ao inserir novo cartão o usuário é questionado sobre a cor do cartão. A numeração é
//! dada automaticamente em ordem crescente.
//! Ao fazer um atendimento o primeiro cartão da fila é retirado e o usuário recebe
//! uma comunicação de que o paciente do cartão deve se dirigir ao atendimento.//! 
//! 
//! 
//! ## Funcionalidades:
//! - Inserir cartão (paciente) com e sem prioridade
//! - Remover cartão (atender paciente) do início da lista
//! - Imprimir fila de espera
//! 
//! ## Exemplo de uso
//! ```bash
//! cargo run --bin lista_encadeada
//! ```

use rust_small_projs::lista_encadeada::menu_principal;

fn main() {
    menu_principal::menu();
}