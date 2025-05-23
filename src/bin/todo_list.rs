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
            "2" =>{
                println!("\nTarefas:");
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.completed {"[X]"} else {"[ ]"};
                    println!("{} - {} {}", i, status, task.title);
                }
            },
            "3" =>{
                let index = read_input("Digite o número da tarefa para marcar como concluída: ");
                if let Ok(i) = index.trim().parse::<usize>() {
                    if let Some(task) = tasks.get_mut(i) {
                        task.completed = true;
                        println!("Tarefa Marcada como concluída!");
                    } else {
                        println!("Índice Inválido");
                    }
                } else {
                    println!("Entrada inválida.");
                }
            },
            "4" =>{
                let index = read_input("Digite o número da tarefa a remover: ");
                if let Ok(i) = index.trim().parse::<usize>() {
                    if i < tasks.len() {
                        tasks.remove(i);
                        println!("Tarefa removida");
                    } else {
                        println!("Índice Inválido");
                    }
                } else {
                    println!("Entrada Inválida");
                }
            },
            "5" => {
                println!("Saindo...");
                break;
            },
            _ => println!("Opção inválida"),
        }
    }
}
