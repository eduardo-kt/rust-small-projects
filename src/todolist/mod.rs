#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title:String) -> Self {
        Self {
            title,
            completed: false,
        }
    }

    pub fn completed(&mut self) {
        self.completed = true;
    }
    
}

pub fn read_input(prompt: &str) -> String {
    use std::io;
    use std::io::Write;

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}