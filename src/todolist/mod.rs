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