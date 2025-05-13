use rust_small_projs::todolist::{read_input, Task};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("--- Lista de Tarefas ---");
        println!(r#"
1. Adicionar tarefa
2. Listar tarefas
3. Marcar tarefa como concluída
4. Remover tarefa
5. Sair
        "#);

        let choice = read_input("Escolha uma opção: ");

        match choice.trim() {
            "1" => {
                let desc = read_input("Digite a tarefa: ");
                tasks.push(Task::new(desc));
                println!("Tarefa adicionada");
            },
            "2" =>{},
            "3" =>{},
            "4" =>{},
            "5" => {
                println!("Saindo...");
                break;
            },
            _ => println!("Opção inválida"),
        }
    }
}
