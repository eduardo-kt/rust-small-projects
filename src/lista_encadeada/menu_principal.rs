use crate::lista_encadeada::estruturas;
use std::io::{self, Write};


pub fn menu() {
    let mut lista_hospitalar = estruturas::ListaSimples::new();
    
    loop {
        println!("
     ________________________
    | Atendimento Hospitalar |
     ------------------------ 
        Digite:
        1 - Adicionar paciente a fila
        2 - Mostrar pacientes na fila
        3 - Chamar paciente
        4 - Sair");
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada.");

        match entrada.trim() {
            "1" => lista_hospitalar.inserir(),
            "2" => lista_hospitalar.imprimir(),
            "3" => lista_hospitalar.chamar_paciente(),
            "4" => break println!("Saindo do Aplicativo..."),
            _ => println!("Escolha inválida. Retornando ao menu principal..."),
            };            
        }
}
