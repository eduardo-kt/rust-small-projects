use std::io;

pub fn menu() {
    
    loop {
        println!("
     ________________________
    | Atendimento Hospitalar |
     ------------------------ 
        Digite
        1 - Adicionar paciente a fila
        2 - Mostrar pacientes na fila
        3 - Chamar paciente
        4 - Sair");
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada.");

        match entrada.trim() {
            "1" => println!("Adiciona pacientes"),
            "2" => println!("Mostra pacientes"),
            "3" => println!("Chama Paciente"),
            "4" => break println!("Saindo do Aplicativo..."),
            _ => println!("Escolha inválida. Retornando ao menu principal."),
            };            
        }
}
